//! Fixed external schedules; actors choose inquiry and execute resource actions.
use crate::{
    assessment as a, attention as t,
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
    simulation::Simulation,
};
use serde::{Deserialize, Serialize};
pub const CASES: [&str; 12] = [
    "protected",
    "minor_fifo",
    "importance",
    "costly_salience",
    "repeated",
    "conflict",
    "hidden",
    "late_attribution",
    "redelivery",
    "equivalent",
    "none_survives",
    "quality",
];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub goal: i32,
    pub weak: i32,
    pub traffic: i32,
    pub response: i32,
    pub effort: u32,
    pub count: usize,
    pub only_second: bool,
    pub reverse: bool,
    pub partner_second: bool,
    pub amount: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            goal: 60,
            weak: 40,
            traffic: 70,
            response: 90,
            effort: 0,
            count: 40,
            only_second: false,
            reverse: false,
            partner_second: false,
            amount: 1,
        }
    }
}
pub fn held_out() -> Vec<Config> {
    (0..4)
        .map(|i| Config {
            goal: [40, 60, 80, 60][i],
            weak: [20, 40, 55, 40][i],
            traffic: [90, 20, 70, 90][i],
            response: [60, 80, 90, 80][i],
            effort: [30, 10, 0, 10][i],
            count: [32, 40, 48, 40][i],
            only_second: i == 0,
            reverse: i % 2 == 1,
            partner_second: i % 2 == 0,
            amount: if i == 3 { 2 } else { 1 },
        })
        .collect()
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub base: old::Snapshot,
    pub attention: t::Attention,
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
        for (index, record) in self.attention.records.iter().enumerate() {
            let query = compact.query(QueryId(record.query))?;
            let index = index + self.attention.first_query;
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
            let expected = if self.attention.mode == t::Mode::CurrentNeed {
                t::policy(base, &record.basis)
            } else {
                base
            };
            if query.decision != expected {
                return Err("inquiry scores/selection disagree".into());
            }
        }
        if self.attention.first_query > compact.provenance.queries.len()
            || self.attention.records.len()
                != compact.provenance.queries.len() - self.attention.first_query
        {
            return Err("missing attention record".into());
        }
        Ok(())
    }
}
fn snapshot(sim: &Simulation) -> Snapshot {
    Snapshot {
        base: old::snapshot(sim),
        attention: sim.attention().unwrap(),
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
pub struct Trial {
    pub seed: u64,
    pub case: String,
    pub config: Config,
    pub method: Method,
    pub mode: t::Mode,
    pub targets: [u64; 2],
    pub resource_partner: u32,
    pub points: Vec<Point>,
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
fn native(
    sim: &mut Simulation,
    event: u64,
    speaker: u32,
    quality: i32,
    positive: bool,
    channel: u8,
) {
    sim.communicate(
        speaker,
        0,
        event,
        EvidenceKind::Fallible {
            scarce: positive,
            reliability: quality,
            source: channel,
        },
    )
    .unwrap();
}
fn acquired(sim: &mut Simulation, event: u64, who: u32, quality: i32) {
    sim.inspect_refusal(event, &[(who, sensor(quality, true, 0))])
        .unwrap();
    sim.provenance_exchange(who, 0, event, true).unwrap();
}
pub fn trial(seed: u64, method: Method, mode: t::Mode, case: &str, config: Config) -> Trial {
    assert!(CASES.contains(&case));
    let mut generator = Generator::new(seed);
    let mut sim = Simulation::new((0..11).map(|id| generator.agent(id)).collect()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(seed, sensor(40, true, 0)).unwrap();
    sim.enable_concerns().unwrap();
    let mut events = vec![];
    for partner in 1..9 {
        sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
        sim.set_circumstances(partner, 1, 90, 0, 90).unwrap();
        sim.set_communication_profile(
            0,
            Profile {
                relationship_goal: if partner == 1 {
                    if matches!(case, "minor_fifo" | "none_survives") {
                        40
                    } else {
                        config.goal
                    }
                } else if partner == 2 {
                    40
                } else {
                    0
                },
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
        events.push(sim.events().last().unwrap().id);
    }
    sim.enable_inquiry().unwrap();
    sim.enable_provenance().unwrap();
    for id in 0..11 {
        sim.set_circumstances(id, 4, 30, 60, 80).unwrap();
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
    sim.enable_assessment(a::Method::Grouped).unwrap();
    sim.enable_retention(method).unwrap();
    sim.enable_attention(mode).unwrap();
    let targets = [events[0], events[1]];
    let mut points = vec![point(&sim, "concerns before pressure")];
    if matches!(case, "hidden" | "late_attribution") {
        sim.inspect_refusal(targets[0], &[(9, sensor(50, false, 0))])
            .unwrap();
        sim.provenance_exchange(9, 10, targets[0], true).unwrap();
        sim.provenance_exchange(9, 0, targets[0], false).unwrap();
        sim.provenance_exchange(10, 0, targets[0], false).unwrap();
        native(&mut sim, targets[0], 1, 40, true, 0);
    } else if case != "minor_fifo" {
        native(
            &mut sim,
            targets[0],
            1,
            if case == "conflict" {
                80
            } else if case == "quality" {
                55
            } else if case == "none_survives" {
                20
            } else {
                config.weak
            },
            true,
            0,
        );
        if case == "conflict" {
            native(&mut sim, targets[0], 1, 80, false, 1);
        }
    }
    if case == "conflict" {
        native(&mut sim, targets[1], 2, 20, true, 0);
    }
    points.push(point(&sim, "before pressure"));
    let mut deliveries = vec![];
    for &event in &events[2..] {
        for who in 1..11 {
            if who != sim_actor(event, &events) {
                deliveries.push((event, who));
            }
        }
    }
    if config.reverse {
        deliveries.reverse();
    }
    if case == "repeated" {
        for _ in 0..config.count {
            native(&mut sim, events[2], 3, 20, true, 0);
        }
    } else if case != "equivalent" {
        for &(event, who) in deliveries.iter().take(config.count) {
            acquired(
                &mut sim,
                event,
                who,
                if case == "quality" {
                    20
                } else if matches!(case, "minor_fifo" | "none_survives") {
                    90
                } else {
                    config.traffic
                },
            );
        }
    } else {
        for _ in 0..4 {
            native(&mut sim, events[2], 3, 20, true, 0);
        }
    }
    if case == "minor_fifo" {
        native(&mut sim, targets[0], 1, 20, true, 0);
    }
    if case == "none_survives" {
        for _ in 0..40 {
            native(&mut sim, events[7], 8, 90, true, 0);
        }
    }
    points.push(point(&sim, "after pressure"));
    if case == "redelivery" {
        acquired(&mut sim, targets[0], 9, 55);
    }
    for (i, &event) in targets.iter().enumerate() {
        if !(case == "conflict" && i == 0) {
            sim.set_inquiry_capability(
                i as u32 + 1,
                event,
                1,
                sensor(config.response, case != "costly_salience", config.effort),
            )
            .unwrap();
        }
    }
    points.push(point(&sim, "before limited inquiry"));
    let meeting = if config.only_second {
        vec![0, 2]
    } else if config.reverse {
        vec![2, 1, 0]
    } else {
        vec![0, 1, 2]
    };
    sim.inquiry_meeting(&meeting).unwrap();
    points.push(point(&sim, "after limited inquiry"));
    if case == "late_attribution" {
        sim.provenance_exchange(9, 0, targets[0], true).unwrap();
        sim.provenance_exchange(10, 0, targets[0], true).unwrap();
    }
    if case == "hidden" {
        native(&mut sim, targets[0], 1, config.response, true, 1);
    }
    points.push(point(&sim, "after later delivery"));
    let partner = if config.partner_second
        ^ matches!(case, "importance" | "costly_salience" | "minor_fifo")
    {
        2
    } else {
        1
    };
    sim.add_scene([0, partner], config.amount, 6).unwrap();
    sim.run();
    points.push(point(&sim, "after executed resource scene"));
    let consumption = sim.consume(0).unwrap();
    points.push(point(&sim, "after subsequent consumption"));
    sim.validate_history_indexes().unwrap();
    let final_state = snapshot(&sim);
    final_state.validate().unwrap();
    Trial {
        seed,
        case: case.into(),
        config,
        method,
        mode,
        targets,
        resource_partner: partner,
        points,
        consumption,
        final_state,
    }
}
fn sim_actor(event: u64, events: &[u64]) -> u32 {
    events.iter().position(|&e| e == event).unwrap() as u32 + 1
}
pub fn suite(seed: u64, held: bool) -> Vec<Trial> {
    let mut trials = vec![];
    for config in if held {
        held_out()
    } else {
        vec![Config::default()]
    } {
        for case in CASES {
            for mode in [t::Mode::Unchanged, t::Mode::CurrentNeed] {
                for method in [Method::Fifo, Method::Quality, Method::Salient] {
                    trials.push(trial(seed, method, mode, case, config.clone()));
                }
            }
        }
    }
    trials
}
pub fn selected(trial: &Trial) -> q::Action {
    let point = &trial.points[3];
    trial.final_state.base.base.base.base.inquiry.records[point.inquiry_end..]
        .iter()
        .find(|r| r.decision.input.owner == 0)
        .unwrap()
        .decision
        .selected
}
