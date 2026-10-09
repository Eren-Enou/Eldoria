//! Fixed external opportunities; no case labels or future partners reach policy.
use crate::{
    assessment as a,
    audit::QueryId,
    behavior::scarcity_policy,
    cognition::{Consumption, EvidenceKind},
    concerns::Concern,
    experiment009 as old,
    foresight::{Channel, Sensor},
    inquiry as q,
    intentional::Profile,
    model::{Agent, Generator},
    retention::Method,
    self_evaluation as s,
    simulation::Simulation,
};
use serde::{Deserialize, Serialize};
pub const CASES: [&str; 15] = [
    "novel_no_progress",
    "context_specific",
    "repetition",
    "partial",
    "conflict",
    "cost",
    "changing_source",
    "later_informed",
    "ignorance",
    "withholding",
    "misleading",
    "changed_circumstances",
    "forgetting",
    "no_information",
    "mirrored_future",
];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub support: i32,
    pub quality: i32,
    pub shift: i32,
    pub effort: u32,
    pub training: usize,
    pub slots: usize,
    pub reverse: bool,
    pub partner: u32,
    pub amount: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            support: 20,
            quality: 50,
            shift: 0,
            effort: 0,
            training: 2,
            slots: 2,
            reverse: false,
            partner: 1,
            amount: 1,
        }
    }
}
pub fn held_out() -> Vec<Config> {
    vec![
        Config {
            support: 10,
            quality: 20,
            shift: -10,
            effort: 20,
            training: 2,
            slots: 2,
            reverse: true,
            partner: 2,
            amount: 1,
        },
        Config {
            support: 40,
            quality: 80,
            shift: 10,
            effort: 0,
            training: 4,
            slots: 3,
            reverse: false,
            partner: 1,
            amount: 2,
        },
        Config {
            support: 55,
            quality: 50,
            shift: 0,
            effort: 20,
            training: 4,
            slots: 2,
            reverse: true,
            partner: 2,
            amount: 2,
        },
    ]
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub base: old::Snapshot,
    pub learning: s::Learning,
}
impl Snapshot {
    pub fn validate(&self) -> Result<(), String> {
        self.base.validate()?;
        let compact = &self.base.base.base;
        if self.learning.first_inquiry > compact.base.inquiry.records.len() {
            return Err("invalid prospective boundary".into());
        }
        let assessment = self
            .base
            .base
            .assessment
            .as_ref()
            .ok_or("assessment missing")?;
        let prefix = |end: usize| -> Result<a::Assessment, String> {
            if end > assessment.records.len() {
                return Err("assessment boundary missing".into());
            }
            let mut state = a::Assessment::new(a::Method::Grouped);
            for r in &assessment.records[..end] {
                let evict = self
                    .base
                    .retention
                    .records
                    .iter()
                    .find(|t| t.assessment == r.id)
                    .map_or(Some(0), |t| t.decision.evict);
                state.receive_selected(
                    r.owner,
                    r.incoming.clone(),
                    r.prior_belief,
                    a::grouped,
                    evict,
                );
            }
            Ok(state)
        };
        let mut rebuilt = s::Learning::new(self.learning.mode, self.learning.first_inquiry);
        if self.learning.records.len()
            != compact.base.inquiry.records.len() - self.learning.first_inquiry
        {
            return Err("missing learning capture".into());
        }
        for (offset, record) in self.learning.records.iter().enumerate() {
            if record.inquiry as usize != offset + self.learning.first_inquiry {
                return Err("inquiry order".into());
            }
            let reference = compact
                .provenance
                .queries
                .iter()
                .find(|r| r.inquiry.0 == record.inquiry)
                .ok_or("missing query")?;
            let query = compact.query(QueryId(reference.id.0))?;
            let q = &compact.base.inquiry.records[record.inquiry as usize];
            let own = prefix(record.assessment_before)?;
            let items: Vec<_> = own
                .items
                .get(&record.owner)
                .into_iter()
                .flatten()
                .cloned()
                .collect();
            let basis = s::project(&q.decision.input.concerns, &items);
            let tick = |r: &a::Record| match r.incoming.receipt {
                a::Receipt::Native(id) => compact.base.base.cognition.information[id as usize].tick,
                a::Receipt::Provenance(id) => compact.provenance.receipts[id as usize].tick,
            };
            if assessment.records[..record.assessment_before]
                .iter()
                .any(|r| tick(r) > reference.observed_at)
                || assessment
                    .records
                    .get(record.assessment_before)
                    .is_some_and(|r| tick(r) <= reference.observed_at)
                || record.assessment_after < record.assessment_before
                || record.assessment_after > assessment.records.len()
                || assessment.records[record.assessment_before..record.assessment_after]
                    .iter()
                    .any(|r| tick(r) > reference.completed_at)
                || assessment
                    .records
                    .get(record.assessment_after)
                    .is_some_and(|r| tick(r) <= reference.completed_at)
                || record.owner != q.decision.input.owner
                || record.basis != basis
                || record.selected != q.decision.selected
            {
                return Err("nonlocal learning boundary".into());
            }
            let cells: Vec<_> = rebuilt
                .cells
                .get(&record.owner)
                .into_iter()
                .flatten()
                .cloned()
                .collect();
            if query.decision
                != s::policy(
                    crate::provenance::policy(&query.decision.input),
                    self.learning.mode,
                    &basis,
                    &cells,
                )
            {
                return Err("learning decision mismatch".into());
            }
            let mut expected = record.clone();
            expected.outcome = None;
            expected.before = None;
            expected.after = None;
            expected.evicted = None;
            if q.valid
                && q.cell_after.is_some()
                && let q::Action::Ask {
                    concern,
                    source,
                    strategy,
                } = q.decision.selected
            {
                let b = basis.iter().find(|b| b.concern == concern).unwrap();
                let after = prefix(record.assessment_after)?;
                let items: Vec<_> = after
                    .items
                    .get(&record.owner)
                    .into_iter()
                    .flatten()
                    .cloned()
                    .collect();
                let outcome = s::Outcome {
                    support_before: b.support,
                    support_after: a::grouped(&items, b.event),
                    status_before: q.concern_before.as_ref().unwrap().status,
                    status_after: q.concern_after.as_ref().unwrap().status,
                    receipt_acquired: assessment.records
                        [record.assessment_before..record.assessment_after]
                        .iter()
                        .any(|r| r.owner == record.owner && r.incoming.event == b.event),
                    novelty: q.novelty,
                    novelty_value: q.realized_value,
                    time: q.time_spent,
                };
                let (before, after, evicted) = rebuilt.learn(
                    record.owner,
                    (source, strategy, b.context),
                    outcome.target(),
                    record.inquiry,
                );
                expected.outcome = Some(outcome);
                expected.before = before;
                expected.after = Some(after);
                expected.evicted = evicted;
            }
            if expected != *record {
                return Err("learning update mismatch".into());
            }
        }
        if rebuilt.cells != self.learning.cells {
            return Err("live learning mismatch".into());
        }
        Ok(())
    }
}
pub fn snapshot(sim: &Simulation) -> Snapshot {
    Snapshot {
        base: old::snapshot(sim),
        learning: sim.self_evaluation().unwrap(),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub label: String,
    pub assessment_end: usize,
    pub inquiry_end: usize,
    pub event_end: usize,
    pub basis: Vec<s::Basis>,
    pub concerns: Vec<Concern>,
    pub agents: Vec<Agent>,
    pub cells: Vec<s::Cell>,
}
fn point(sim: &Simulation, label: &str) -> Point {
    let a = sim.assessment().unwrap();
    let concerns = sim.concerns().unwrap().items[&0].clone();
    let items: Vec<_> = a.items.get(&0).into_iter().flatten().cloned().collect();
    Point {
        label: label.into(),
        assessment_end: a.records.len(),
        inquiry_end: sim.inquiry().unwrap().records.len(),
        event_end: sim.events().len(),
        basis: s::project(&concerns, &items),
        concerns,
        agents: sim.agents(),
        cells: sim
            .self_evaluation()
            .unwrap()
            .cells
            .get(&0)
            .into_iter()
            .flatten()
            .cloned()
            .collect(),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Opportunity {
    pub phase: String,
    pub participants: Vec<u32>,
    pub before: Point,
    pub after: Point,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub case: String,
    pub config: Config,
    pub mode: s::Mode,
    pub targets: [u64; 3],
    pub resource_partner: u32,
    pub opportunities: Vec<Opportunity>,
    pub after_action: Point,
    pub persistent: Point,
    pub consumption: Consumption,
    pub final_state: Snapshot,
}
fn sensor(quality: i32, inverted: bool, effort: u32) -> Sensor {
    Sensor {
        reliability: quality,
        channel: if inverted {
            Channel::InvertedFixture
        } else {
            Channel::AccurateFixture
        },
        effort,
        second: None,
    }
}
fn profile(sim: &mut Simulation, id: u32, willing: bool) {
    sim.set_communication_profile(
        id,
        Profile {
            relationship_goal: if willing { 100 } else { 0 },
            privacy: if willing { 0 } else { 100 },
            ..Default::default()
        },
    )
    .unwrap();
}
fn meeting(
    sim: &mut Simulation,
    phase: &str,
    mut participants: Vec<u32>,
    reverse: bool,
    opportunities: &mut Vec<Opportunity>,
) {
    if reverse {
        participants.reverse();
    }
    let before = point(sim, "before opportunity");
    sim.inquiry_meeting(&participants).unwrap();
    let after = point(sim, "after opportunity");
    opportunities.push(Opportunity {
        phase: phase.into(),
        participants,
        before,
        after,
    });
}
pub fn trial(seed: u64, mode: s::Mode, case: &str, config: Config) -> Trial {
    assert!(CASES.contains(&case));
    let mut g = Generator::new(seed);
    let mut sim = Simulation::new((0..5).map(|id| g.agent(id)).collect()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(
        seed,
        sensor(
            if case == "partial" {
                10
            } else {
                config.support
            },
            false,
            0,
        ),
    )
    .unwrap();
    sim.enable_concerns().unwrap();
    let mut targets = [0; 3];
    for (index, goal) in [80, 65, 45].into_iter().enumerate() {
        let partner = index as u32 + 1;
        sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
        sim.set_circumstances(partner, 1, 90, 0, 90).unwrap();
        sim.set_communication_profile(
            0,
            Profile {
                relationship_goal: (goal + config.shift).clamp(30, 82),
                privacy: 100,
                ..Default::default()
            },
        )
        .unwrap();
        profile(&mut sim, partner, false);
        sim.add_scene([0, partner], 1, 6).unwrap();
        sim.run();
        targets[index] = sim.events().last().unwrap().id;
    }
    sim.enable_inquiry().unwrap();
    sim.enable_provenance().unwrap();
    sim.enable_assessment(a::Method::Grouped).unwrap();
    sim.enable_retention(Method::Fifo).unwrap();
    for id in 0..5 {
        sim.set_circumstances(id, 4, 30, 60, 80).unwrap();
        profile(&mut sim, id, true);
    }
    let initial = if case == "partial" {
        10
    } else {
        config.support
    };
    for i in [0, 2] {
        sim.communicate(
            i as u32 + 1,
            0,
            targets[i],
            EvidenceKind::Fallible {
                scarce: true,
                reliability: initial,
                source: 0,
            },
        )
        .unwrap();
    }
    if case == "conflict" {
        sim.communicate(
            1,
            0,
            targets[0],
            EvidenceKind::Fallible {
                scarce: false,
                reliability: initial,
                source: 1,
            },
        )
        .unwrap();
    }
    // Voluntary retrospective acquisitions; no cognitive state is inserted.
    if matches!(case, "context_specific" | "misleading" | "mirrored_future") {
        sim.inspect_refusal(
            targets[1],
            &[(1, sensor(config.quality, case == "misleading", 0))],
        )
        .unwrap();
    }
    if matches!(case, "changed_circumstances" | "repetition" | "partial") {
        sim.set_inquiry_capability(
            1,
            targets[0],
            1,
            sensor(config.quality, false, config.effort),
        )
        .unwrap();
    }
    if case == "changing_source" {
        sim.inspect_refusal(targets[0], &[(4, sensor(config.quality, false, 0))])
            .unwrap();
        sim.inspect_refusal(targets[2], &[(1, sensor(config.quality, false, 0))])
            .unwrap();
    }
    if case == "no_information" || case == "withholding" {
        if case == "withholding" {
            sim.inspect_refusal(targets[0], &[(4, sensor(config.quality, false, 0))])
                .unwrap();
        }
        profile(&mut sim, 4, false);
    }
    sim.enable_self_evaluation(mode).unwrap();
    let mut opportunities = vec![];
    for _ in 0..config.training {
        let source = if matches!(
            case,
            "ignorance" | "withholding" | "no_information" | "changing_source" | "later_informed"
        ) {
            4
        } else {
            1
        };
        meeting(
            &mut sim,
            "training",
            vec![0, source],
            config.reverse,
            &mut opportunities,
        );
    }
    // Circumstance changes are fixed by family/time, never by a selected action or mode.
    if case == "changed_circumstances" {
        profile(&mut sim, 1, false);
    }
    if case == "changing_source" {
        for _ in 0..36 {
            sim.provenance_exchange(1, 4, targets[2], true).unwrap();
        }
    }
    if case == "later_informed" {
        sim.inspect_refusal(targets[0], &[(4, sensor(config.quality, false, 0))])
            .unwrap();
    }
    if case == "forgetting" {
        for _ in 0..36 {
            sim.communicate(
                3,
                0,
                targets[2],
                EvidenceKind::Fallible {
                    scarce: true,
                    reliability: initial,
                    source: 0,
                },
            )
            .unwrap();
        }
    }
    if matches!(
        case,
        "novel_no_progress" | "cost" | "conflict" | "changed_circumstances"
    ) {
        for (i, &event) in targets.iter().enumerate() {
            if case == "conflict" && i == 0 {
                continue;
            }
            sim.set_inquiry_capability(
                i as u32 + 1,
                event,
                1,
                sensor(
                    config.quality,
                    false,
                    config.effort + if case == "cost" { 40 } else { 0 },
                ),
            )
            .unwrap();
        }
    }
    for slot in 0..config.slots {
        let ids = match case {
            "ignorance" | "withholding" | "no_information" | "changing_source"
            | "later_informed" => vec![0, 4],
            "context_specific" | "misleading" | "mirrored_future" => vec![0, 1],
            "changed_circumstances" if slot > 0 => vec![0, 2],
            "changed_circumstances" | "repetition" | "partial" | "forgetting" => vec![0, 1],
            _ => vec![0, 1, 2, 3],
        };
        meeting(&mut sim, "limited", ids, config.reverse, &mut opportunities);
    }
    let partner = if case == "mirrored_future" {
        config.partner % 2 + 1
    } else {
        config.partner
    };
    sim.add_scene([0, partner], config.amount, 6).unwrap();
    sim.run();
    let after_action = point(&sim, "executed resource scene");
    let consumption = sim.consume(0).unwrap();
    let persistent = point(&sim, "subsequent consumption");
    sim.validate_history_indexes().unwrap();
    let final_state = snapshot(&sim);
    final_state.validate().unwrap();
    Trial {
        seed,
        case: case.into(),
        config,
        mode,
        targets,
        resource_partner: partner,
        opportunities,
        after_action,
        persistent,
        consumption,
        final_state,
    }
}
pub fn suite(seed: u64, held: bool) -> Vec<Trial> {
    let configs = if held {
        held_out()
    } else {
        vec![Config::default()]
    };
    configs
        .into_iter()
        .flat_map(|config| {
            CASES.into_iter().flat_map(move |case| {
                s::MODES.into_iter().map({
                    let config = config.clone();
                    move |mode| trial(seed, mode, case, config.clone())
                })
            })
        })
        .collect()
}

pub fn summary(t: &Trial, held: bool) -> serde_json::Value {
    let compact = &t.final_state.base.base.base;
    let steps:Vec<_>=t.opportunities.iter().map(|o| {
        let q=compact.base.inquiry.records[o.before.inquiry_end..o.after.inquiry_end].iter().find(|r|r.decision.input.owner==0).unwrap();
        let learn=t.final_state.learning.records.iter().find(|r|r.inquiry==q.id).unwrap();
        serde_json::json!({"phase":o.phase,"inquiry":q.id,"selected":q.decision.selected,"outcome":learn.outcome,"before":learn.before,"after":learn.after,"time":q.time_spent,"baseline_cell":q.cell_after})
    }).collect();
    let start = t.opportunities.last().unwrap().after.event_end;
    let events = &compact.base.base.events[start..];
    serde_json::json!({"seed":t.seed,"held":held,"case":t.case,"config":t.config,"mode":t.mode,"steps":steps,
        "actions":events.iter().map(|e|e.decision.selected).collect::<Vec<_>>(),"event_ids":events.iter().map(|e|e.id).collect::<Vec<_>>(),
        "food_after_action":t.after_action.agents.iter().map(|a|a.food).collect::<Vec<_>>(),"persistent_food":t.persistent.agents.iter().map(|a|a.food).collect::<Vec<_>>(),
        "resource_partner":t.resource_partner,"actor_cells":t.persistent.cells.len(),"serialized_bytes":serde_json::to_vec(t).unwrap().len(),
        "live_cell_bytes":serde_json::to_vec(&t.final_state.learning.cells).unwrap().len(),"audit_bytes":serde_json::to_vec(&t.final_state.learning.records).unwrap().len()})
}
