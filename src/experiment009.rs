//! Matched retention fixtures: actual refusals and validated local receipts.
use crate::{
    assessment::{self as a, Method as AssessmentMethod},
    behavior::scarcity_policy,
    cognition::EvidenceKind,
    concerns::Status,
    experiment008,
    foresight::{Channel, Sensor},
    intentional::Profile,
    model::{Action, Generator},
    retention::{self as r, Method},
    simulation::Simulation,
};
use serde::{Deserialize, Serialize};

pub const SCENARIOS: [&str; 12] = [
    "consequential_flood",
    "unresolved_concern",
    "relevance_expires",
    "conflict",
    "known_repetition",
    "hidden_origin",
    "later_attribution",
    "equal_salience",
    "salient_wrong",
    "minor_later_useful",
    "event_traffic",
    "redelivery",
];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub quality: i32,
    pub traffic_quality: i32,
    pub weak_quality: i32,
    pub distinct_quality: i32,
    pub attributed: bool,
    pub minor_early: bool,
    pub goal: i32,
    pub reverse: bool,
    pub interleave: bool,
    pub shift: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            quality: 80,
            traffic_quality: 40,
            weak_quality: 40,
            distinct_quality: 70,
            attributed: false,
            minor_early: false,
            goal: 80,
            reverse: false,
            interleave: false,
            shift: false,
        }
    }
}
/// Combinations from pre-registration and the disclosed harness coverage review.
pub fn held_out() -> Vec<Config> {
    [20, 45, 70, 90]
        .into_iter()
        .enumerate()
        .map(|(i, quality)| Config {
            quality,
            traffic_quality: [70, 20, 90, 45][i],
            weak_quality: quality,
            distinct_quality: [90, 60, 80, 70][i],
            attributed: i % 2 == 1,
            minor_early: i % 2 == 0,
            goal: [0, 40, 60, 100][i],
            reverse: i % 2 == 0,
            interleave: i % 2 == 1,
            shift: true,
        })
        .collect()
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub base: experiment008::Snapshot,
    pub retention: r::Retention,
}
impl Snapshot {
    pub fn validate(&self) -> Result<(), String> {
        self.retention
            .validate(self.base.assessment.as_ref().ok_or("missing assessment")?)?;
        self.base.validate_receipts()
    }
}
pub fn snapshot(sim: &Simulation) -> Snapshot {
    Snapshot {
        base: experiment008::snapshot(sim),
        retention: sim.retention().unwrap(),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub label: String,
    pub basis_support: i32,
    pub target_items: usize,
    pub target_origins: usize,
    pub positive: bool,
    pub negative: bool,
    pub belief: Option<i32>,
    pub concern: Option<Status>,
    pub trust: i32,
    pub action: Action,
    pub live_items: usize,
}
fn point(sim: &Simulation, event: u64, label: &str) -> Point {
    let basis = sim.assessment().unwrap();
    let items: Vec<_> = basis
        .items
        .get(&0)
        .map(|v| v.iter().cloned().collect())
        .unwrap_or_default();
    let local = experiment008::point(sim, event, label);
    Point {
        label: label.into(),
        basis_support: a::grouped(&items, event),
        target_items: items.iter().filter(|i| i.event == event).count(),
        target_origins: items
            .iter()
            .filter(|i| i.event == event && i.quality > 0)
            .map(|i| i.origin)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        positive: items
            .iter()
            .any(|i| i.event == event && i.claim && i.quality > 0),
        negative: items
            .iter()
            .any(|i| i.event == event && !i.claim && i.quality > 0),
        belief: local.support,
        concern: local.concern,
        trust: local.trust,
        action: local.probe.selected,
        live_items: items.len(),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub scenario: String,
    pub config: Config,
    pub method: Method,
    pub target: u64,
    pub interventions: Vec<String>,
    pub initial: Snapshot,
    pub points: Vec<Point>,
    pub final_state: Snapshot,
}
fn sensor(quality: i32, positive: bool) -> Sensor {
    Sensor {
        reliability: quality,
        channel: if positive {
            Channel::AccurateFixture
        } else {
            Channel::InvertedFixture
        },
        effort: 0,
        second: None,
    }
}
pub fn setup(seed: u64, method: Method, goal: i32, shift: bool) -> (Simulation, Vec<u64>) {
    let mut g = Generator::new(seed);
    let mut sim = Simulation::new((0..9).map(|id| g.agent(id)).collect()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(seed, sensor(40, true)).unwrap();
    sim.enable_concerns().unwrap();
    sim.enable_inquiry().unwrap();
    sim.enable_provenance().unwrap();
    let mut events = vec![];
    for round in 0..(6 + usize::from(shift)) {
        sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
        sim.set_circumstances(1, 1, 90, 0, 90).unwrap();
        sim.set_communication_profile(
            0,
            Profile {
                relationship_goal: if round == usize::from(shift) { goal } else { 0 },
                privacy: 100,
                ..Default::default()
            },
        )
        .unwrap();
        sim.set_communication_profile(
            1,
            Profile {
                relationship_goal: 0,
                privacy: 100,
                ..Default::default()
            },
        )
        .unwrap();
        sim.add_scene([0, 1], 1, 6).unwrap();
        sim.run();
        let event = sim.events().last().unwrap().id;
        if !shift || round > 0 {
            events.push(event);
        }
    }
    // Public opportunities permit acquisition/sharing but do not select messages.
    for id in 0..9 {
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
    sim.enable_assessment(AssessmentMethod::Grouped).unwrap();
    sim.enable_retention(method).unwrap();
    (sim, events)
}
fn native(sim: &mut Simulation, event: u64, positive: bool, quality: i32, source: u8) {
    sim.communicate(
        1,
        0,
        event,
        EvidenceKind::Fallible {
            scarce: positive,
            reliability: quality,
            source,
        },
    )
    .unwrap();
}
fn acquired(sim: &mut Simulation, event: u64, who: u32, quality: i32, positive: bool) {
    sim.inspect_refusal(event, &[(who, sensor(quality, positive))])
        .unwrap();
    assert!(
        sim.provenance_exchange(who, 0, event, true)
            .unwrap()
            .is_some()
    );
}
fn distinct(sim: &mut Simulation, events: &[u64], quality: i32, reverse: bool, count: usize) {
    let mut sequence: Vec<_> = events
        .iter()
        .flat_map(|&event| (2..9).map(move |who| (event, who)))
        .collect();
    if reverse {
        sequence.reverse();
    }
    for (event, who) in sequence.into_iter().take(count) {
        acquired(sim, event, who, quality, true);
    }
}
pub fn trial(seed: u64, method: Method, scenario: &str, config: Config) -> Trial {
    assert!(SCENARIOS.contains(&scenario));
    let goal = if matches!(
        scenario,
        "equal_salience" | "minor_later_useful" | "redelivery"
    ) {
        0
    } else {
        config.goal
    };
    let (mut sim, events) = setup(seed, method, goal, config.shift);
    let target = events[0];
    let noise = &events[1..];
    let initial = snapshot(&sim);
    let mut points = vec![point(&sim, target, "initial")];
    // Zero-quality acquired payload permits later reappraisal without new content.
    sim.inspect_refusal(target, &[(8, sensor(0, true))])
        .unwrap();
    match scenario {
        "unresolved_concern" => {
            native(&mut sim, target, true, config.weak_quality, 0);
            points.push(point(&sim, target, "weak active basis"));
            distinct(&mut sim, noise, config.distinct_quality, config.reverse, 35);
            // Repeated known acquisition: no new inspection, root or information.
            let repeated_event = if config.reverse {
                noise[0]
            } else {
                *noise.last().unwrap()
            };
            let repeated_speaker = if config.reverse { 2 } else { 8 };
            for _ in 0..12 {
                sim.provenance_exchange(repeated_speaker, 0, repeated_event, true)
                    .unwrap();
            }
        }
        "relevance_expires" => {
            native(&mut sim, target, true, config.weak_quality, 0);
            points.push(point(&sim, target, "active"));
            acquired(&mut sim, target, 2, 50, true);
            points.push(point(&sim, target, "legitimate resolution"));
            distinct(&mut sim, noise, config.distinct_quality, config.reverse, 35);
        }
        "equal_salience" => {
            native(&mut sim, target, true, 50, 0);
            distinct(&mut sim, noise, 50, config.reverse, 35);
        }
        "minor_later_useful" => {
            if config.minor_early {
                native(&mut sim, target, true, 20, 0);
            }
            distinct(&mut sim, noise, config.distinct_quality, config.reverse, 32);
            if !config.minor_early {
                native(&mut sim, target, true, 20, 0);
            }
            points.push(point(&sim, target, "minor item after pressure"));
            // Future relevance is this later genuine acquisition, never a retention input.
            acquired(&mut sim, target, 2, 50, true);
        }
        "redelivery" => {
            native(&mut sim, target, true, config.weak_quality, 0);
            distinct(&mut sim, noise, config.distinct_quality, config.reverse, 35);
            points.push(point(&sim, target, "forgotten"));
            acquired(&mut sim, target, 2, 90, true);
        }
        _ => {
            if matches!(
                scenario,
                "known_repetition" | "hidden_origin" | "later_attribution"
            ) {
                sim.inspect_refusal(target, &[(2, sensor(50, true))])
                    .unwrap();
                sim.provenance_exchange(2, 3, target, true).unwrap();
                sim.provenance_exchange(
                    2,
                    0,
                    target,
                    scenario != "hidden_origin" || config.attributed,
                )
                .unwrap();
                sim.provenance_exchange(
                    3,
                    0,
                    target,
                    scenario == "known_repetition" || config.attributed,
                )
                .unwrap();
            } else if scenario == "conflict" {
                for (positive, source) in if config.reverse {
                    [(false, 1), (true, 0)]
                } else {
                    [(true, 0), (false, 1)]
                } {
                    native(&mut sim, target, positive, config.quality, source);
                }
            } else {
                native(
                    &mut sim,
                    target,
                    scenario != "salient_wrong",
                    config.quality,
                    0,
                );
            }
            points.push(point(&sim, target, "before pressure"));
            for index in 0..34 {
                if scenario == "known_repetition" && index % 2 == 0 {
                    sim.provenance_exchange(3, 0, target, true).unwrap();
                } else {
                    let event = if config.interleave || scenario == "event_traffic" {
                        noise[index % noise.len()]
                    } else {
                        noise[0]
                    };
                    native(&mut sim, event, true, config.traffic_quality, 0);
                }
            }
            if scenario == "later_attribution" {
                points.push(point(&sim, target, "before revelation"));
                sim.provenance_exchange(3, 0, target, true).unwrap();
            }
        }
    }
    points.push(point(&sim, target, "after pressure"));
    if scenario == "salient_wrong" {
        native(&mut sim, target, true, 90, 1);
        points.push(point(&sim, target, "later legitimate counterevidence"));
    }
    assert!(
        sim.native_acquired_exchange(8, 0, target, true)
            .unwrap()
            .is_some()
    );
    points.push(point(&sim, target, "legitimate zero-quality reappraisal"));
    sim.validate_history_indexes().unwrap();
    let final_state = snapshot(&sim);
    final_state.validate().unwrap();
    Trial { seed,scenario:scenario.into(),config,method,target,
        interventions:vec!["Actual scarcity refusals; own goal varies only at creation; privacy100 permits voluntary silence; then matched food4/hunger30/generosity60/caution80, public acquisition opportunities".into(),
            "Accurate/Inverted fixture channels set received direction; controlled native deliveries isolate assimilation; no observer truth in retention input".into()],
        initial,points,final_state }
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
            SCENARIOS.into_iter().flat_map(move |scenario| {
                [Method::Fifo, Method::Quality, Method::Salient]
                    .into_iter()
                    .map({
                        let config = config.clone();
                        move |method| trial(seed, method, scenario, config.clone())
                    })
            })
        })
        .collect()
}
/// Saturated, matched population fixture; receipt timings are measured separately.
pub fn population_sim(seed: u64, count: u32, method: Option<Method>) -> Simulation {
    assert!(count.is_multiple_of(2));
    let mut g = Generator::new(seed);
    let mut sim = Simulation::new((0..count).map(|id| g.agent(id)).collect()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(seed, sensor(40, true)).unwrap();
    sim.enable_concerns().unwrap();
    sim.enable_inquiry().unwrap();
    sim.enable_provenance().unwrap();
    for id in 0..count {
        let donor = id % 2 == 1;
        sim.set_circumstances(
            id,
            u32::from(donor),
            90,
            if donor { 0 } else { 20 },
            if donor { 90 } else { 20 },
        )
        .unwrap();
        sim.set_communication_profile(
            id,
            Profile {
                relationship_goal: 80,
                privacy: 100,
                ..Default::default()
            },
        )
        .unwrap();
    }
    for id in (0..count).step_by(2) {
        sim.add_scene([id, id + 1], 1, 6).unwrap();
    }
    sim.run();
    sim.enable_assessment(AssessmentMethod::Grouped).unwrap();
    if let Some(method) = method {
        sim.enable_retention(method).unwrap();
    }
    for event in sim
        .events()
        .into_iter()
        .filter(|e| e.outcome == Some(crate::model::Outcome::Refusal))
    {
        let speaker = event.decision.actor;
        let listener = *event
            .participants
            .iter()
            .find(|&&id| id != speaker)
            .unwrap();
        for index in 0..36 {
            sim.communicate(
                speaker,
                listener,
                event.id,
                EvidenceKind::Fallible {
                    scarce: true,
                    reliability: if index == 0 { 80 } else { 40 },
                    source: u8::from(index == 0),
                },
            )
            .unwrap();
        }
    }
    sim.validate_history_indexes().unwrap();
    sim
}

pub fn population(seed: u64, count: u32, method: Method) -> Snapshot {
    let state = snapshot(&population_sim(seed, count, Some(method)));
    state.validate().unwrap();
    state
}
