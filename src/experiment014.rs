//! Fixed014 opportunities, opt-in selection and uninterrupted consequences.
use crate::{
    activity as a, assessment,
    behavior::scarcity_policy,
    cognition::{Consumption, EvidenceKind},
    experiment009, experiment013,
    foresight::{Channel, Sensor},
    intentional::Profile,
    model::{Agent, Generator},
    retention, self_evaluation,
    simulation::Simulation,
};
use serde::{Deserialize, Serialize};
pub const CASES: [&str; 14] = [
    "information_first",
    "material_first",
    "misleading",
    "no_information",
    "no_material",
    "competition",
    "improvement",
    "willingness",
    "tight",
    "loose",
    "mirrored",
    "delayed",
    "longitudinal",
    "reservation",
];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub budget: u64,
    pub support: i32,
    pub quality: i32,
    pub periods: usize,
    pub long_periods: usize,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            budget: 40,
            support: 20,
            quality: 50,
            periods: 4,
            long_periods: 24,
        }
    }
}
pub fn sensitivity() -> [Config; 2] {
    [
        Config {
            budget: 24,
            support: 10,
            quality: 20,
            periods: 6,
            long_periods: 32,
        },
        Config {
            budget: 64,
            support: 40,
            quality: 80,
            periods: 8,
            long_periods: 40,
        },
    ]
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub tick: u64,
    pub events: usize,
    pub queries: usize,
    pub assessment: usize,
    pub agents: Vec<Agent>,
    pub basis: Vec<self_evaluation::Basis>,
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
        tick: sim.activity_tick(),
        events: sim.events().len(),
        queries: sim.inquiry().unwrap().records.len(),
        assessment: assessment.records.len(),
        agents: sim.agents(),
        basis: self_evaluation::project(&sim.concerns().unwrap().items[&0], &items),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub decision: a::Decision,
    pub before: Point,
    pub after: Point,
    pub expired: Vec<a::Action>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Period {
    pub start: u64,
    pub end: u64,
    pub qclose: u64,
    pub rclose: u64,
    pub source: bool,
    pub partner: bool,
    pub records: Vec<Record>,
    pub unused: Vec<a::Action>,
    pub consumption: Vec<Consumption>,
    pub after: Point,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Intervention {
    pub period: usize,
    pub kind: String,
    pub before: Point,
    pub after: Point,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub case: String,
    pub config: Config,
    pub mode: a::Mode,
    pub targets: [u64; 2],
    pub initial: Point,
    pub periods: Vec<Period>,
    pub interventions: Vec<Intervention>,
    pub final_state: experiment013::Snapshot,
}
fn sensor(quality: i32, inverted: bool) -> Sensor {
    Sensor {
        reliability: quality,
        channel: if inverted {
            Channel::InvertedFixture
        } else {
            Channel::AccurateFixture
        },
        effort: 0,
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
pub fn setup(seed: u64, mode: a::Mode, case: &str, c: &Config) -> (Simulation, [u64; 2]) {
    let mut generator = Generator::new(seed);
    let mut sim = Simulation::new((0..3).map(|id| generator.agent(id)).collect()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(seed, sensor(c.support, false))
        .unwrap();
    sim.enable_concerns().unwrap();
    let mut targets = [0; 2];
    for (i, target) in targets.iter_mut().enumerate() {
        sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
        sim.set_circumstances(1, 1, 90, 0, 90).unwrap();
        sim.set_communication_profile(
            0,
            Profile {
                relationship_goal: if i == 0 { 80 } else { 65 },
                privacy: 100,
                ..Default::default()
            },
        )
        .unwrap();
        profile(&mut sim, 1, false);
        sim.add_scene([0, 1], 1, 6).unwrap();
        sim.run();
        *target = sim.events().last().unwrap().id;
    }
    sim.enable_inquiry().unwrap();
    sim.enable_provenance().unwrap();
    sim.enable_assessment(assessment::Method::Grouped).unwrap();
    sim.enable_retention(retention::Method::Fifo).unwrap();
    for id in 0..3 {
        sim.set_circumstances(
            id,
            4,
            if id == 0 { 30 } else { 0 },
            if id <= 1 { 100 } else { 60 },
            if id == 1 {
                0
            } else if id == 0 {
                40
            } else {
                80
            },
        )
        .unwrap();
        profile(&mut sim, id, true);
    }
    if matches!(case, "material_first" | "misleading" | "mirrored") {
        sim.set_circumstances(0, 0, 80, 60, 80).unwrap();
    }
    if case == "competition" {
        sim.set_circumstances(0, 4, 30, 100, 0).unwrap();
    }
    if case == "mirrored" {
        sim.set_circumstances(1, 4, 90, 0, 100).unwrap();
    }
    sim.communicate(
        1,
        0,
        targets[0],
        EvidenceKind::Fallible {
            scarce: true,
            reliability: c.support,
            source: 0,
        },
    )
    .unwrap();
    if !matches!(
        case,
        "no_information" | "improvement" | "willingness" | "longitudinal"
    ) {
        sim.inspect_refusal(targets[0], &[(2, sensor(c.quality, case == "misleading"))])
            .unwrap();
    }
    sim.enable_self_evaluation(self_evaluation::Mode::Contextual)
        .unwrap();
    if mode == a::Mode::LocalPulse {
        sim.set_self_evaluation_policy(experiment013::pulse)
            .unwrap();
    }
    (sim, targets)
}
pub fn trial(seed: u64, mode: a::Mode, case: &str, c: Config) -> Trial {
    assert!(CASES.contains(&case));
    let (mut sim, targets) = setup(seed, mode, case, &c);
    let initial = point(&sim);
    let n = if case == "longitudinal" {
        c.long_periods
    } else {
        c.periods
    };
    let mut periods = vec![];
    let mut interventions = vec![];
    for day in 0..n {
        if day > 0 {
            let before = point(&sim);
            for agent in sim.agents() {
                sim.set_circumstances(
                    agent.id,
                    agent.food,
                    (agent.hunger + 20).min(100),
                    agent.generosity,
                    agent.caution,
                )
                .unwrap();
            }
            interventions.push(Intervention {
                period: day,
                kind: "external relative hunger+20".into(),
                before,
                after: point(&sim),
            });
        }
        if (matches!(case, "improvement" | "willingness") && day == n / 2)
            || (case == "longitudinal" && day == 6)
        {
            let before = point(&sim);
            sim.inspect_refusal(targets[0], &[(2, sensor(c.quality, false))])
                .unwrap();
            interventions.push(Intervention {
                period: day,
                kind: "selected source acquisition".into(),
                before,
                after: point(&sim),
            });
        }
        if case == "willingness" && day == n / 2 {
            let before = point(&sim);
            profile(&mut sim, 2, false);
            sim.set_circumstances(1, sim.agents()[1].food, 90, 0, 100)
                .unwrap();
            interventions.push(Intervention {
                period: day,
                kind: "private willingness change".into(),
                before,
                after: point(&sim),
            });
        }
        if case == "longitudinal" && day == 10 {
            let before = point(&sim);
            sim.inspect_refusal(targets[1], &[(0, sensor(c.support, false))])
                .unwrap();
            for _ in 0..36 {
                sim.provenance_exchange(0, 2, targets[1], true).unwrap();
            }
            interventions.push(Intervention {
                period: day,
                kind: "provider FIFO pressure".into(),
                before,
                after: point(&sim),
            });
        }
        if case == "longitudinal" && day == 12 {
            let before = point(&sim);
            for _ in 0..36 {
                sim.communicate(
                    1,
                    0,
                    targets[1],
                    EvidenceKind::Fallible {
                        scarce: true,
                        reliability: c.support,
                        source: 0,
                    },
                )
                .unwrap();
            }
            interventions.push(Intervention {
                period: day,
                kind: "actor FIFO pressure".into(),
                before,
                after: point(&sim),
            });
        }
        if case == "longitudinal" && day == 14 {
            let before = point(&sim);
            sim.inspect_refusal(targets[0], &[(0, sensor(c.quality, true))])
                .unwrap();
            sim.provenance_exchange(0, 2, targets[0], true).unwrap();
            interventions.push(Intervention {
                period: day,
                kind: "focal first inverted inspection and relay".into(),
                before,
                after: point(&sim),
            });
        }
        let budget = match case {
            "tight" => 16,
            "loose" => 70,
            "reservation" => 15,
            _ => c.budget,
        };
        let rw = if matches!(
            case,
            "material_first" | "misleading" | "competition" | "mirrored"
        ) {
            8.min(budget)
        } else {
            25.min(budget)
        };
        let qw = if case == "delayed" {
            13.min(budget)
        } else {
            budget
        };
        let start = sim.activity_tick();
        sim.activity_open(
            0,
            Some(2),
            if case == "no_material" { None } else { Some(1) },
            budget,
            qw,
            rw,
        )
        .unwrap();
        let mut records = vec![];
        let mut used = vec![];
        let mut expired = vec![];
        for _ in 0..3 {
            if sim.activity_tick() >= start + budget {
                break;
            }
            let input = sim.activity_input().unwrap();
            let decision = a::policy(&input, mode);
            let before = point(&sim);
            let action = decision.selected;
            sim.activity_execute(action).unwrap();
            let after = point(&sim);
            if action != a::Action::Wait {
                used.push(action);
            }
            let lost = [
                (a::Action::Inquiry, start + qw, true),
                (a::Action::Resource, start + rw, case != "no_material"),
            ]
            .into_iter()
            .filter(|(a, close, present)| {
                *present && !used.contains(a) && !expired.contains(a) && after.tick >= *close
            })
            .map(|(a, _, _)| a)
            .collect::<Vec<_>>();
            expired.extend(&lost);
            records.push(Record {
                decision,
                before,
                after,
                expired: lost,
            });
            if action == a::Action::Wait {
                break;
            }
        }
        let unused = [a::Action::Inquiry, a::Action::Resource]
            .into_iter()
            .filter(|a| !used.contains(a) && (*a != a::Action::Resource || case != "no_material"))
            .collect();
        let consumption = sim.activity_finish().unwrap();
        periods.push(Period {
            start,
            end: start + budget,
            qclose: start + qw,
            rclose: start + rw,
            source: true,
            partner: case != "no_material",
            records,
            unused,
            consumption,
            after: point(&sim),
        });
    }
    sim.validate_history_indexes().unwrap();
    let final_state = experiment013::Snapshot {
        base: experiment009::snapshot(&sim),
        learning: sim.self_evaluation().unwrap(),
        mode: if mode == a::Mode::LocalPulse {
            experiment013::Mode::Pulse
        } else {
            experiment013::Mode::Contextual
        },
    };
    final_state.validate().unwrap();
    Trial {
        seed,
        case: case.into(),
        config: c,
        mode,
        targets,
        initial,
        periods,
        interventions,
        final_state,
    }
}
pub fn suite(seed: u64, held: bool) -> Vec<Trial> {
    let configs = if held {
        sensitivity().to_vec()
    } else {
        vec![Config::default()]
    };
    configs
        .into_iter()
        .flat_map(|c| {
            CASES.into_iter().flat_map(move |case| {
                a::MODES.into_iter().map({
                    let c = c.clone();
                    move |mode| trial(seed, mode, case, c.clone())
                })
            })
        })
        .collect()
}
pub fn summary(t: &Trial, held: bool) -> serde_json::Value {
    let records = t
        .periods
        .iter()
        .flat_map(|p| &p.records)
        .collect::<Vec<_>>();
    let compact = &t.final_state.base.base.base;
    let events = &compact.base.base.events;
    serde_json::json!({"seed":t.seed,"held":held,"case":t.case,"config":t.config,"mode":t.mode,
        "activities":records.iter().map(|r|r.decision.selected).collect::<Vec<_>>(),
        "inquiry_ticks":records.iter().filter(|r|r.decision.selected==a::Action::Inquiry).map(|r|r.after.tick-r.before.tick).sum::<u64>(),
        "resource_ticks":records.iter().filter(|r|r.decision.selected==a::Action::Resource).map(|r|r.after.tick-r.before.tick).sum::<u64>(),
        "wait_ticks":t.periods.iter().map(|p|p.end-p.start-p.records.iter().filter(|r|r.decision.selected!=a::Action::Wait).map(|r|r.after.tick-r.before.tick).sum::<u64>()).sum::<u64>(),
        "unused_inquiry":t.periods.iter().filter(|p|p.unused.contains(&a::Action::Inquiry)).count(),"unused_resource":t.periods.iter().filter(|p|p.unused.contains(&a::Action::Resource)).count(),
        "transfers":events[t.initial.events..].iter().map(|e|e.transferred).sum::<u32>(),"consumed":t.periods.iter().flat_map(|p|&p.consumption).map(|c|c.consumed).sum::<u32>(),
        "food":t.periods.last().unwrap().after.agents.iter().map(|a|a.food).collect::<Vec<_>>(),"hunger":t.periods.last().unwrap().after.agents.iter().map(|a|a.hunger).collect::<Vec<_>>(),
        "trust":t.periods.last().unwrap().after.agents[0].trust,"bytes":serde_json::to_vec(t).unwrap().len(),"learning_bytes":serde_json::to_vec(&t.final_state.learning.cells).unwrap().len()})
}
