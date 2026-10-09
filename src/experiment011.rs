//! Controlled sequential opportunities; policy never receives fixture configuration.
use crate::{
    assessment as a, attention as t,
    audit::QueryId,
    behavior::scarcity_policy,
    cognition::{Consumption, EvidenceKind},
    concerns::Concern,
    experiment009 as old,
    foresight::{Channel, Sensor},
    inquiry as q, inquiry_value as v,
    intentional::Profile,
    model::{Agent, Generator},
    retention::Method,
    simulation::Simulation,
};
use serde::{Deserialize, Serialize};
pub const CASES: [&str; 16] = [
    "uncertain_minor",
    "poor_answerability",
    "near_resolution",
    "far_resolution",
    "expensive",
    "learned_poor",
    "new_opportunity",
    "resolved",
    "wrong_confidence",
    "minor_later",
    "mirrored_future",
    "equal_tie",
    "partial_answers",
    "silent",
    "same_uncertainty",
    "same_importance",
];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub goal_shift: i32,
    pub support_shift: i32,
    pub quality: i32,
    pub effort: u32,
    pub source_subset: bool,
    pub slots: usize,
    pub reverse: bool,
    pub partner: u32,
    pub amount: u32,
    pub unmet_food: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            goal_shift: 0,
            support_shift: 0,
            quality: 90,
            effort: 0,
            source_subset: false,
            slots: 2,
            reverse: false,
            partner: 1,
            amount: 1,
            unmet_food: 4,
        }
    }
}
pub fn held_out() -> Vec<Config> {
    (0..4)
        .map(|i| Config {
            goal_shift: [-10, 10, 0, 10][i],
            support_shift: [-15, 10, 15, 0][i],
            quality: [35, 65, 95, 65][i],
            effort: [35, 12, 0, 12][i],
            source_subset: i == 0,
            slots: if i == 2 { 3 } else { 2 },
            reverse: i % 2 == 1,
            partner: [2, 3, 1, 3][i],
            amount: if i == 3 { 2 } else { 1 },
            unmet_food: 4,
        })
        .collect()
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub base: old::Snapshot,
    pub value: v::ValueAudit,
}
impl Snapshot {
    pub fn validate(&self) -> Result<(), String> {
        self.base.validate()?;
        let assessment = self
            .base
            .base
            .assessment
            .as_ref()
            .ok_or("missing assessment")?;
        let compact = &self.base.base.base;
        let mut rebuilt = a::Assessment::new(a::Method::Grouped);
        let mut cursor = 0;
        let receipt_tick = |r: &a::Record| match r.incoming.receipt {
            a::Receipt::Native(id) => compact.base.base.cognition.information[id as usize].tick,
            a::Receipt::Provenance(id) => compact.provenance.receipts[id as usize].tick,
        };
        for (index, record) in self.value.records.iter().enumerate() {
            let query = compact.query(QueryId(record.query))?;
            let index = index + self.value.first_query;
            let observed = compact
                .provenance
                .queries
                .get(index)
                .ok_or("missing query")?
                .observed_at;
            if record.query != index as u64
                || record.inquiry != query.inquiry_record
                || record.assessment_end < cursor
                || record.assessment_end > assessment.records.len()
            {
                return Err("invalid attention reference".into());
            }
            if assessment.records[..record.assessment_end]
                .iter()
                .any(|r| receipt_tick(r) > observed)
                || assessment
                    .records
                    .get(record.assessment_end)
                    .is_some_and(|r| receipt_tick(r) <= observed)
            {
                return Err("assessment boundary is not available at inquiry time".into());
            }
            for r in &assessment.records[cursor..record.assessment_end] {
                let evict = self
                    .base
                    .retention
                    .records
                    .iter()
                    .find(|c| c.assessment == r.id)
                    .map_or(Some(0), |c| c.decision.evict);
                rebuilt.receive_selected(
                    r.owner,
                    r.incoming.clone(),
                    r.prior_belief,
                    a::grouped,
                    evict,
                );
            }
            cursor = record.assessment_end;
            let input = &query.decision.input;
            let items: Vec<_> = rebuilt
                .items
                .get(&input.base.owner)
                .into_iter()
                .flatten()
                .cloned()
                .collect();
            if record.basis != t::project(&input.base.concerns, &items) {
                return Err("projection disagrees with retained prefix".into());
            }
            let base = crate::provenance::policy(input);
            let expected = v::compare(self.value.mode, base, &record.basis);
            if query.decision != expected {
                return Err("inquiry scores/selection disagree".into());
            }
        }
        if self.value.first_query > compact.provenance.queries.len()
            || self.value.records.len() != compact.provenance.queries.len() - self.value.first_query
        {
            return Err("missing attention record".into());
        }
        Ok(())
    }
}
fn snapshot(sim: &Simulation) -> Snapshot {
    Snapshot {
        base: old::snapshot(sim),
        value: sim.inquiry_value().unwrap(),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub label: String,
    pub assessment_end: usize,
    pub inquiry_end: usize,
    pub event_end: usize,
    pub basis: Vec<t::Basis>,
    pub concerns: Vec<Concern>,
    pub agents: Vec<Agent>,
}
fn point(sim: &Simulation, label: &str) -> Point {
    let assessment = sim.assessment().unwrap();
    let concerns = sim.concerns().unwrap().items[&0].clone();
    let items: Vec<_> = assessment
        .items
        .get(&0)
        .into_iter()
        .flatten()
        .cloned()
        .collect();
    Point {
        label: label.into(),
        assessment_end: assessment.records.len(),
        inquiry_end: sim.inquiry().unwrap().records.len(),
        event_end: sim.events().len(),
        basis: t::project(&concerns, &items),
        concerns,
        agents: sim.agents(),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Opportunity {
    pub before: Point,
    pub after: Point,
    pub remaining: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub case: String,
    pub config: Config,
    pub mode: v::Mode,
    pub targets: [u64; 3],
    pub resource_partner: u32,
    pub before_prelude: Point,
    pub opportunities: Vec<Opportunity>,
    pub after_action: Point,
    pub persistent: Point,
    pub consumption: Consumption,
    pub final_state: Snapshot,
}
fn sensor(quality: i32, positive: bool, effort: u32) -> Sensor {
    Sensor {
        reliability: quality,
        channel: if positive {
            Channel::AccurateFixture
        } else {
            Channel::InvertedFixture
        },
        effort,
        second: None,
    }
}
pub fn trial(seed: u64, mode: v::Mode, case: &str, config: Config) -> Trial {
    assert!(CASES.contains(&case));
    assert!((2..=3).contains(&config.slots));
    let mut generator = Generator::new(seed);
    let mut sim = Simulation::new((0..5).map(|id| generator.agent(id)).collect()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(seed, sensor(40, true, 0)).unwrap();
    sim.enable_concerns().unwrap();
    let goals = match case {
        "uncertain_minor" | "mirrored_future" | "minor_later" => [80, 22, 45],
        "near_resolution" | "far_resolution" => [45, 55, 30],
        "same_importance" | "equal_tie" | "partial_answers" => [50, 50, 50],
        _ => [80, 65, 45],
    };
    let mut targets = [0; 3];
    for (index, goal) in goals.into_iter().enumerate() {
        let partner = index as u32 + 1;
        sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
        sim.set_circumstances(partner, 1, 90, 0, 90).unwrap();
        sim.set_communication_profile(
            0,
            Profile {
                relationship_goal: (goal + config.goal_shift).clamp(22, 82),
                privacy: 100,
                ..Default::default()
            },
        )
        .unwrap();
        sim.set_communication_profile(
            partner,
            Profile {
                relationship_goal: 0,
                privacy: 100,
                ..Default::default()
            },
        )
        .unwrap();
        sim.add_scene([0, partner], 1, 6).unwrap();
        sim.run();
        targets[index] = sim.events().last().unwrap().id;
    }
    sim.enable_inquiry().unwrap();
    sim.enable_provenance().unwrap();
    sim.enable_assessment(a::Method::Grouped).unwrap();
    sim.enable_retention(Method::Fifo).unwrap();
    for id in 0..5 {
        sim.set_circumstances(id, if id == 4 { config.unmet_food } else { 4 }, 30, 60, 80)
            .unwrap();
        sim.set_communication_profile(
            id,
            Profile {
                relationship_goal: 100,
                privacy: 0,
                ..Default::default()
            },
        )
        .unwrap();
    }
    let support: [i32; 3] = match case {
        "uncertain_minor" | "mirrored_future" | "minor_later" => [40, 0, 20],
        "near_resolution" => [55, 20, 0],
        "far_resolution" => [5, 20, 0],
        "poor_answerability" | "learned_poor" => [40, 20, 0],
        "new_opportunity" => [0, 20, 0],
        "wrong_confidence" => [-85, 20, 0],
        "equal_tie" | "same_uncertainty" => [20, 20, 20],
        "partial_answers" => [10, 20, 30],
        "same_importance" => [0, 0, 0],
        _ => [45, 20, 0],
    };
    let order: Vec<_> = if config.reverse {
        (0..3).rev().collect()
    } else {
        (0..3).collect()
    };
    for i in order {
        if support[i] != 0 {
            let quality = (support[i].abs() + config.support_shift).clamp(1, 90);
            sim.communicate(
                i as u32 + 1,
                0,
                targets[i],
                EvidenceKind::Fallible {
                    scarce: support[i] > 0,
                    reliability: quality,
                    source: 0,
                },
            )
            .unwrap();
        }
    }
    let before_prelude = point(
        &sim,
        "matched local state before shared history preparation",
    );
    let prelude = match case {
        "learned_poor" | "new_opportunity" => 1,
        "partial_answers" => 9,
        "near_resolution" | "far_resolution" => 7,
        _ => 0,
    };
    if prelude > 0 {
        for id in 1..4 {
            sim.set_communication_profile(
                id,
                Profile {
                    relationship_goal: 0,
                    privacy: 100,
                    ..Default::default()
                },
            )
            .unwrap();
        }
        for _ in 0..prelude {
            sim.inquiry_meeting(if case == "partial_answers" {
                &[0, 1, 2, 3]
            } else {
                &[0, 1]
            })
            .unwrap();
        }
        for id in 1..4 {
            sim.set_communication_profile(
                id,
                Profile {
                    relationship_goal: 100,
                    privacy: 0,
                    ..Default::default()
                },
            )
            .unwrap();
        }
    }
    sim.enable_inquiry_value(mode).unwrap();
    let mut opportunities = vec![];
    for slot in 0..config.slots {
        for (i, &event) in targets.iter().enumerate() {
            let quality = match case {
                "near_resolution" | "far_resolution" | "partial_answers" => 20,
                "poor_answerability" | "same_importance" if i == 0 => 10,
                "new_opportunity" if slot == 0 && i == 0 => 10,
                "equal_tie" => 70,
                _ => config.quality.min(90),
            };
            let effort = config.effort + if case == "expensive" && i == 0 { 45 } else { 0 };
            sim.set_inquiry_capability(
                i as u32 + 1,
                event,
                if case == "new_opportunity" && slot > 0 && i == 0 {
                    0
                } else {
                    1
                },
                sensor(quality, true, effort),
            )
            .unwrap();
        }
        if case == "silent" {
            for id in 1..4 {
                sim.set_communication_profile(
                    id,
                    Profile {
                        relationship_goal: 0,
                        privacy: 100,
                        ..Default::default()
                    },
                )
                .unwrap();
            }
        }
        let before = point(&sim, &format!("before limited opportunity {slot}"));
        let mut meeting = if case == "partial_answers" {
            // Fixed successive public sources, independent of treatment/selection.
            vec![
                0,
                ((slot + usize::from(config.source_subset)) % 3 + 1) as u32,
            ]
        } else if matches!(case, "near_resolution" | "far_resolution") {
            vec![0, 1]
        } else if config.source_subset {
            vec![0, 2, 3]
        } else {
            vec![0, 1, 2, 3]
        };
        if config.reverse {
            meeting.reverse();
        }
        sim.inquiry_meeting(&meeting).unwrap();
        let after = point(&sim, &format!("after limited opportunity {slot}"));
        opportunities.push(Opportunity {
            before,
            after,
            remaining: config.slots - slot - 1,
        });
    }
    let partner = if case == "mirrored_future" {
        config.partner % 3 + 1
    } else if case == "minor_later" {
        2
    } else {
        config.partner
    };
    sim.add_scene([0, partner], config.amount, 6).unwrap();
    sim.run();
    let after_action = point(&sim, "after executed resource scene");
    let consumption = sim.consume(0).unwrap();
    let persistent = point(&sim, "after subsequent consumption");
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
        before_prelude,
        opportunities,
        after_action,
        persistent,
        consumption,
        final_state,
    }
}
pub fn suite(seed: u64, held: bool) -> Vec<Trial> {
    let mut trials = vec![];
    for config in if held {
        held_out()
    } else {
        vec![Config::default()]
    } {
        for case in CASES {
            for mode in v::MODES {
                trials.push(trial(seed, mode, case, config.clone()));
            }
        }
    }
    trials
}
pub fn queries(trial: &Trial) -> Vec<&q::Record> {
    let records = &trial.final_state.base.base.base.base.inquiry.records;
    trial
        .opportunities
        .iter()
        .map(|o| {
            records[o.before.inquiry_end..o.after.inquiry_end]
                .iter()
                .find(|r| r.decision.input.owner == 0)
                .unwrap()
        })
        .collect()
}
