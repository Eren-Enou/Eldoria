//! 013-only transient reconsideration and fixed changing-world fixtures.
use crate::{
    assessment as a,
    audit::QueryId,
    behavior::scarcity_policy,
    cognition::{Consumption, EvidenceKind},
    experiment009 as old,
    foresight::{Channel, Sensor},
    inquiry as q,
    intentional::Profile,
    model::{Agent, Generator},
    provenance as p,
    retention::Method,
    self_evaluation as s,
    simulation::Simulation,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Unchanged,
    GlobalProgress,
    Contextual,
    Pulse,
    Ungated,
}
pub const MODES: [Mode; 5] = [
    Mode::Unchanged,
    Mode::GlobalProgress,
    Mode::Contextual,
    Mode::Pulse,
    Mode::Ungated,
];
impl Mode {
    pub fn learning(self) -> s::Mode {
        match self {
            Self::Unchanged => s::Mode::Unchanged,
            Self::GlobalProgress => s::Mode::GlobalProgress,
            _ => s::Mode::Contextual,
        }
    }
}
pub fn evaluate(
    base: p::Decision,
    mode: Mode,
    basis: &[s::Basis],
    cells: &[s::Cell],
) -> p::Decision {
    let mut d = s::policy(base, mode.learning(), basis, cells);
    if matches!(mode, Mode::Pulse | Mode::Ungated) {
        for c in &mut d.decision.candidates {
            if let q::Action::Ask {
                concern,
                source,
                strategy,
            } = c.action
            {
                let own = d
                    .input
                    .base
                    .concerns
                    .iter()
                    .find(|x| x.id == concern)
                    .unwrap();
                if c.attempts < 2
                    || c.prior_or_learned > strategy.prior() / 2
                    || (mode == Mode::Pulse && own.age % 4 != 0)
                {
                    continue;
                }
                let credibility = d
                    .input
                    .base
                    .sources
                    .iter()
                    .find(|x| x.id == source)
                    .unwrap()
                    .credibility;
                let independence = d
                    .factors
                    .iter()
                    .find(|x| x.action == c.action)
                    .unwrap()
                    .independence;
                let gain = (strategy.prior() + credibility / 10).clamp(0, 100) * independence / 100;
                c.benefit = c.benefit.max(c.importance * gain / 100);
                c.score = c.benefit - c.cost as i32;
            }
        }
        d.decision.selected = d
            .decision
            .candidates
            .iter()
            .fold((q::Action::Pause, 0), |best, c| {
                if c.score > best.1 {
                    (c.action, c.score)
                } else {
                    best
                }
            })
            .0;
    }
    d
}
pub fn pulse(base: p::Decision, _: s::Mode, b: &[s::Basis], c: &[s::Cell]) -> p::Decision {
    evaluate(base, Mode::Pulse, b, c)
}
pub fn ungated(base: p::Decision, _: s::Mode, b: &[s::Basis], c: &[s::Cell]) -> p::Decision {
    evaluate(base, Mode::Ungated, b, c)
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub base: old::Snapshot,
    pub learning: s::Learning,
    pub mode: Mode,
}
impl Snapshot {
    pub fn validate(&self) -> Result<(), String> {
        self.base.validate()?;
        if self.learning.mode != self.mode.learning() {
            return Err("mode mismatch".into());
        }
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
                != evaluate(
                    crate::provenance::policy(&query.decision.input),
                    self.mode,
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

pub const CASES: [&str; 13] = [
    "later_useful",
    "unchanged",
    "hidden_twin",
    "lost_access",
    "unwilling",
    "useful_offer",
    "misleading_offer",
    "competition",
    "repeated_failure",
    "misleading_success",
    "forgetting",
    "longitudinal",
    "mirrored_future",
];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub support: i32,
    pub quality: i32,
    pub effort: u32,
    pub change: usize,
    pub rounds: usize,
    pub long_rounds: usize,
    pub amount: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            support: 20,
            quality: 50,
            effort: 0,
            change: 8,
            rounds: 24,
            long_rounds: 48,
            amount: 1,
        }
    }
}
pub fn held_out() -> Vec<Config> {
    vec![
        Config {
            support: 10,
            quality: 20,
            effort: 20,
            change: 12,
            rounds: 32,
            long_rounds: 56,
            amount: 1,
        },
        Config {
            support: 40,
            quality: 80,
            effort: 0,
            change: 16,
            rounds: 40,
            long_rounds: 72,
            amount: 2,
        },
    ]
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub inquiry_end: usize,
    pub event_end: usize,
    pub assessment_end: usize,
    pub basis: Vec<s::Basis>,
    pub cells: Vec<s::Cell>,
    pub food: Vec<u32>,
}
fn point(sim: &Simulation) -> Point {
    let assessment = sim.assessment().unwrap();
    let items = assessment
        .items
        .get(&0)
        .into_iter()
        .flatten()
        .cloned()
        .collect::<Vec<_>>();
    Point {
        inquiry_end: sim.inquiry().unwrap().records.len(),
        event_end: sim.events().len(),
        assessment_end: assessment.records.len(),
        basis: s::project(&sim.concerns().unwrap().items[&0], &items),
        cells: sim
            .self_evaluation()
            .unwrap()
            .cells
            .get(&0)
            .into_iter()
            .flatten()
            .cloned()
            .collect(),
        food: sim.agents().iter().map(|a| a.food).collect(),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Opportunity {
    pub round: usize,
    pub phase: String,
    pub participants: Vec<u32>,
    pub before: Point,
    pub after: Point,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Change {
    pub round: usize,
    pub kind: String,
    pub source: u32,
    pub event: u64,
    pub inspections: usize,
    pub receipts: usize,
    pub knowledge: usize,
}
fn changed(sim: &Simulation, round: usize, kind: &str, source: u32, event: u64) -> Change {
    let state = sim.compact_snapshot("transition").unwrap();
    Change {
        round,
        kind: kind.into(),
        source,
        event,
        inspections: state.provenance.windows.len(),
        receipts: state.provenance.receipts.len(),
        knowledge: state
            .provenance
            .knowledge
            .get(&source)
            .map_or(0, |x| x.len()),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resource {
    pub round: usize,
    pub partner: u32,
    pub start: usize,
    pub end: usize,
    pub actors_before: Vec<Agent>,
    pub actors_after: Vec<Agent>,
    pub consumption: Consumption,
    pub persistent_food: Vec<u32>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub case: String,
    pub config: Config,
    pub mode: Mode,
    pub targets: [u64; 3],
    pub opportunities: Vec<Opportunity>,
    pub changes: Vec<Change>,
    pub resources: Vec<Resource>,
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
pub fn trial(seed: u64, mode: Mode, case: &str, config: Config) -> Trial {
    assert!(CASES.contains(&case));
    let mut g = Generator::new(seed);
    let mut sim = Simulation::new((0..7).map(|id| g.agent(id)).collect()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(seed, sensor(config.support, false, 0))
        .unwrap();
    sim.enable_concerns().unwrap();
    let mut targets = [0; 3];
    for (i, goal) in [80, 65, 45].into_iter().enumerate() {
        let partner = i as u32 + 1;
        sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
        sim.set_circumstances(partner, 1, 90, 0, 90).unwrap();
        sim.set_communication_profile(
            0,
            Profile {
                relationship_goal: goal,
                privacy: 100,
                ..Default::default()
            },
        )
        .unwrap();
        profile(&mut sim, partner, false);
        sim.add_scene([0, partner], 1, 6).unwrap();
        sim.run();
        targets[i] = sim.events().last().unwrap().id;
    }
    sim.enable_inquiry().unwrap();
    sim.enable_provenance().unwrap();
    sim.enable_assessment(a::Method::Grouped).unwrap();
    sim.enable_retention(Method::Fifo).unwrap();
    for id in 0..7 {
        sim.set_circumstances(id, 4, 30, 60, 80).unwrap();
        profile(&mut sim, id, true);
    }
    for i in [0, 2] {
        sim.communicate(
            i as u32 + 1,
            0,
            targets[i],
            EvidenceKind::Fallible {
                scarce: true,
                reliability: config.support,
                source: 0,
            },
        )
        .unwrap();
    }
    sim.inspect_refusal(targets[2], &[(6, sensor(config.support, false, 0))])
        .unwrap();
    if matches!(case, "lost_access" | "unwilling") {
        sim.inspect_refusal(targets[0], &[(4, sensor(config.support, false, 0))])
            .unwrap();
    }
    if case == "competition" {
        sim.inspect_refusal(targets[1], &[(5, sensor(config.quality, false, 0))])
            .unwrap();
    }
    sim.enable_self_evaluation(mode.learning()).unwrap();
    match mode {
        Mode::Pulse => sim.set_self_evaluation_policy(pulse).unwrap(),
        Mode::Ungated => sim.set_self_evaluation_policy(ungated).unwrap(),
        _ => {}
    }
    let rounds = if case == "longitudinal" {
        config.long_rounds
    } else {
        config.rounds
    };
    let mut opportunities = vec![];
    let mut changes = vec![];
    let mut resources = vec![];
    for round in 0..rounds {
        if round == config.change {
            match case {
                "later_useful" | "hidden_twin" | "misleading_success" | "mirrored_future"
                | "competition" | "longitudinal" => {
                    let quality = if case == "longitudinal" {
                        20
                    } else {
                        config.quality
                    };
                    let acquired_source = if case == "hidden_twin" { 5 } else { 4 };
                    sim.inspect_refusal(
                        targets[0],
                        &[(
                            acquired_source,
                            sensor(quality, case == "misleading_success", config.effort),
                        )],
                    )
                    .unwrap();
                    changes.push(changed(&sim, round, "acquire", acquired_source, targets[0]));
                }
                "lost_access" => {
                    for _ in 0..36 {
                        sim.provenance_exchange(6, 4, targets[2], true).unwrap();
                    }
                    changes.push(changed(
                        &sim,
                        round,
                        "provider eviction pressure",
                        4,
                        targets[0],
                    ));
                }
                "unwilling" => {
                    profile(&mut sim, 4, false);
                    changes.push(changed(&sim, round, "privacy intervention", 4, targets[0]));
                }
                "useful_offer" => {
                    sim.set_inquiry_capability(
                        1,
                        targets[0],
                        1,
                        sensor(config.quality, false, config.effort),
                    )
                    .unwrap();
                    changes.push(changed(&sim, round, "new native capability", 1, targets[0]));
                }
                "forgetting" => {
                    for _ in 0..36 {
                        sim.communicate(
                            3,
                            0,
                            targets[2],
                            EvidenceKind::Fallible {
                                scarce: true,
                                reliability: config.support,
                                source: 0,
                            },
                        )
                        .unwrap();
                    }
                    changes.push(changed(
                        &sim,
                        round,
                        "actor eviction pressure",
                        0,
                        targets[0],
                    ));
                }
                "misleading_offer" => {
                    sim.inspect_refusal(
                        targets[0],
                        &[(4, sensor(config.quality, false, config.effort))],
                    )
                    .unwrap();
                    sim.provenance_offer(4, 0, targets[0], true).unwrap();
                    profile(&mut sim, 4, false);
                    changes.push(changed(
                        &sim,
                        round,
                        "voluntary metadata offer then private unwillingness",
                        4,
                        targets[0],
                    ));
                }
                _ => {}
            }
        }
        if case == "longitudinal" {
            if round == config.change + 4 {
                sim.inspect_refusal(targets[1], &[(5, sensor(20, false, config.effort))])
                    .unwrap();
                changes.push(changed(&sim, round, "acquire", 5, targets[1]));
            }
            if round == config.change + 8 {
                for _ in 0..36 {
                    sim.provenance_exchange(6, 4, targets[2], true).unwrap();
                }
                changes.push(changed(
                    &sim,
                    round,
                    "provider eviction pressure",
                    4,
                    targets[0],
                ));
            }
            if round == config.change + 12 {
                profile(&mut sim, 5, false);
                changes.push(changed(&sim, round, "privacy intervention", 5, targets[1]));
            }
            if round == config.change + 16 {
                sim.inspect_refusal(targets[0], &[(6, sensor(20, true, config.effort))])
                    .unwrap();
                sim.provenance_exchange(6, 4, targets[0], true).unwrap();
                sim.provenance_offer(4, 0, targets[0], true).unwrap();
                profile(&mut sim, 5, true);
                changes.push(changed(
                    &sim,
                    round,
                    "new inverted acquisition/relay; willingness restored",
                    4,
                    targets[0],
                ));
            }
        }
        let participants = if case == "useful_offer" {
            vec![0, 1]
        } else if matches!(case, "competition" | "longitudinal") {
            vec![0, 4, 5]
        } else {
            vec![0, 4]
        };
        let before = point(&sim);
        sim.inquiry_meeting(&participants).unwrap();
        let after = point(&sim);
        opportunities.push(Opportunity {
            round,
            phase: if round < config.change {
                "training"
            } else {
                "changing"
            }
            .into(),
            participants,
            before,
            after,
        });
        if (round + 1) % 16 == 0 || round + 1 == rounds {
            let partner = if case == "mirrored_future" { 2 } else { 1 };
            let start = sim.events().len();
            let actors_before = sim.agents();
            sim.add_scene([0, partner], config.amount, 6).unwrap();
            sim.run();
            let end = sim.events().len();
            let actors_after = sim.agents();
            let consumption = sim.consume(0).unwrap();
            resources.push(Resource {
                round,
                partner,
                start,
                end,
                actors_before,
                actors_after,
                consumption,
                persistent_food: sim.agents().iter().map(|a| a.food).collect(),
            });
        }
    }
    sim.validate_history_indexes().unwrap();
    let final_state = Snapshot {
        base: old::snapshot(&sim),
        learning: sim.self_evaluation().unwrap(),
        mode,
    };
    final_state.validate().unwrap();
    Trial {
        seed,
        case: case.into(),
        config,
        mode,
        targets,
        opportunities,
        changes,
        resources,
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
        .flat_map(|c| {
            CASES.into_iter().flat_map(move |case| {
                MODES.into_iter().map({
                    let c = c.clone();
                    move |mode| trial(seed, mode, case, c.clone())
                })
            })
        })
        .collect()
}
pub fn summary(t: &Trial, held: bool) -> serde_json::Value {
    let compact = &t.final_state.base.base.base;
    let inquiry = &compact.base.inquiry.records;
    let steps:Vec<_>=t.opportunities.iter().map(|o|{let r=inquiry[o.before.inquiry_end..o.after.inquiry_end].iter().find(|r|r.decision.input.owner==0).unwrap();let l=t.final_state.learning.records.iter().find(|l|l.inquiry==r.id).unwrap();serde_json::json!({"round":o.round,"inquiry":r.id,"selected":r.decision.selected,"time":r.time_spent,"outcome":l.outcome,"before":l.before,"after":l.after,"age":r.concern_before.as_ref().map(|c|c.age),"baseline_cell":r.cell_after})}).collect();
    let actions: Vec<_> = t
        .resources
        .iter()
        .map(|r| {
            compact.base.base.events[r.start..r.end]
                .iter()
                .map(|e| e.decision.selected)
                .collect::<Vec<_>>()
        })
        .collect();
    serde_json::json!({"seed":t.seed,"held":held,"case":t.case,"config":t.config,"mode":t.mode,"steps":steps,"changes":t.changes,"actions":actions,"persistent_food":t.resources.last().unwrap().persistent_food,"serialized_bytes":serde_json::to_vec(t).unwrap().len(),"live_cell_bytes":serde_json::to_vec(&t.final_state.learning.cells).unwrap().len(),"audit_bytes":serde_json::to_vec(&t.final_state.learning.records).unwrap().len()})
}
