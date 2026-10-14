//! Preregistered015 stationary cohort and opt-in independent invitation histories.
use crate::{
    assessment,
    cognition::Consumption,
    encounters as e, experiment009, experiment013, experiment014,
    foresight::{Channel, Sensor},
    intentional::Profile,
    model::{Agent, Generator},
    retention, self_evaluation,
    simulation::Simulation,
};
use serde::{Deserialize, Serialize};
pub const CASES: [&str; 13] = [
    "mutual",
    "unilateral",
    "contended",
    "unavailable",
    "missed",
    "history",
    "information",
    "misleading",
    "private_mirror",
    "formerly_useful",
    "refusal",
    "separate",
    "longitudinal",
];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub budget: u64,
    pub periods: usize,
    pub long_periods: usize,
    pub order: e::Order,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            budget: 24,
            periods: 6,
            long_periods: 24,
            order: e::Order::Ascending,
        }
    }
}
pub fn sensitivity() -> [Config; 4] {
    [
        Config {
            order: e::Order::Descending,
            ..Default::default()
        },
        Config {
            order: e::Order::Rotating,
            ..Default::default()
        },
        Config {
            budget: 16,
            periods: 8,
            long_periods: 32,
            ..Default::default()
        },
        Config {
            budget: 48,
            periods: 8,
            long_periods: 32,
            ..Default::default()
        },
    ]
}
fn sensor(quality: i32) -> Sensor {
    Sensor {
        reliability: quality,
        channel: Channel::AccurateFixture,
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
pub fn setup(seed: u64, case: &str) -> Simulation {
    let history = matches!(
        case,
        "history"
            | "information"
            | "misleading"
            | "private_mirror"
            | "formerly_useful"
            | "longitudinal"
    );
    let mut sim = if history {
        let config = experiment014::Config {
            support: if case == "formerly_useful" { 10 } else { 20 },
            quality: if case == "formerly_useful" { 20 } else { 50 },
            ..Default::default()
        };
        experiment014::setup(
            seed,
            crate::activity::Mode::Local,
            if case == "misleading" {
                "misleading"
            } else {
                "information_first"
            },
            &config,
        )
        .0
    } else {
        let mut generator = Generator::new(seed);
        let mut s = Simulation::new((0..3).map(|id| generator.agent(id)).collect()).unwrap();
        s.set_policy(crate::behavior::scarcity_policy);
        s.enable_foresight(seed, sensor(20)).unwrap();
        s.enable_concerns().unwrap();
        s.enable_inquiry().unwrap();
        s.enable_provenance().unwrap();
        s.enable_assessment(assessment::Method::Grouped).unwrap();
        s.enable_retention(retention::Method::Fifo).unwrap();
        s.enable_self_evaluation(self_evaluation::Mode::Contextual)
            .unwrap();
        for id in 0..3 {
            s.set_circumstances(id, 2, 20, 40, 50).unwrap();
            profile(&mut s, id, true);
        }
        s
    };
    if matches!(
        case,
        "information" | "misleading" | "private_mirror" | "formerly_useful" | "longitudinal"
    ) {
        sim.set_circumstances(0, 4, 30, 0, 100).unwrap();
    }
    if matches!(
        case,
        "mutual" | "unilateral" | "contended" | "unavailable" | "missed"
    ) {
        sim.set_circumstances(0, 0, 90, 0, 10).unwrap();
        sim.set_circumstances(1, 4, 0, 100, 0).unwrap();
        sim.set_circumstances(2, 0, 0, 0, 100).unwrap();
    }
    if case == "unilateral" {
        sim.set_circumstances(1, 4, 0, 0, 100).unwrap();
    }
    if case == "contended" {
        sim.set_circumstances(2, 0, 90, 0, 10).unwrap();
    }
    if case == "refusal" {
        for id in 0..3 {
            sim.set_circumstances(id, if id == 0 { 0 } else { 4 }, 80, 0, 100)
                .unwrap();
        }
    }
    if case == "separate" {
        for id in 0..3 {
            sim.set_circumstances(id, 0, 0, 0, 100).unwrap();
            profile(&mut sim, id, false);
        }
    }
    if case == "private_mirror" {
        profile(&mut sim, 2, false);
    }
    if case == "formerly_useful" {
        sim.inquiry_meeting(&[0, 2]).unwrap();
        let targets = sim.concerns().unwrap().items[&0]
            .iter()
            .map(|c| c.event)
            .collect::<Vec<_>>();
        let b = targets[1];
        sim.inspect_refusal(b, &[(0, sensor(10))]).unwrap();
        for _ in 0..36 {
            sim.provenance_exchange(0, 2, b, true).unwrap();
        }
    }
    sim.enable_encounters().unwrap();
    sim
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Period {
    pub start: u64,
    pub end: u64,
    pub busy_until: [u64; 3],
    pub before: e::State,
    pub waves: Vec<e::Wave>,
    pub consumption: Vec<Consumption>,
    pub after: e::State,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Intervention {
    pub period: usize,
    pub before: Vec<Agent>,
    pub after: Vec<Agent>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub case: String,
    pub mode: e::Mode,
    pub config: Config,
    pub initial: e::State,
    pub periods: Vec<Period>,
    pub interventions: Vec<Intervention>,
    pub final_state: experiment013::Snapshot,
}
pub fn trial(seed: u64, mode: e::Mode, case: &str, config: Config) -> Trial {
    assert!(CASES.contains(&case));
    let mut sim = setup(seed, case);
    let initial = sim.encounter_state();
    let mut periods = vec![];
    let mut interventions = vec![];
    let mut wave = 0;
    let n = if case == "longitudinal" {
        config.long_periods
    } else {
        config.periods
    };
    for period in 0..n {
        if period > 0 {
            let before = sim.agents();
            for a in &before {
                sim.set_circumstances(
                    a.id,
                    a.food,
                    (a.hunger + 20).min(100),
                    a.generosity,
                    a.caution,
                )
                .unwrap();
            }
            interventions.push(Intervention {
                period,
                before,
                after: sim.agents(),
            });
        }
        let before = sim.encounter_state();
        let start = before.tick;
        let budget = if case == "missed" { 2 } else { config.budget };
        let end = start + budget;
        let busy_until = if case == "unavailable" && period == 0 {
            [start, end, start]
        } else {
            [start; 3]
        };
        sim.encounter_open(budget, busy_until).unwrap();
        let mut waves = vec![];
        while sim.activity_tick() < end {
            let record = sim.encounter_wave(mode, config.order, wave).unwrap();
            wave += 1;
            waves.push(record);
        }
        let consumption = sim.encounter_finish().unwrap();
        let after = sim.encounter_state();
        periods.push(Period {
            start,
            end,
            busy_until,
            before,
            waves,
            consumption,
            after,
        });
    }
    let final_state = experiment013::Snapshot {
        base: experiment009::snapshot(&sim),
        learning: sim.self_evaluation().unwrap(),
        mode: experiment013::Mode::Contextual,
    };
    final_state.validate().unwrap();
    Trial {
        seed,
        case: case.into(),
        mode,
        config,
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
                e::MODES.into_iter().map({
                    let c = c.clone();
                    move |mode| trial(seed, mode, case, c.clone())
                })
            })
        })
        .collect()
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Summary {
    pub seed: u64,
    pub case: String,
    pub mode: e::Mode,
    pub config: Config,
    pub invitations: usize,
    pub accepted: usize,
    pub declined: usize,
    pub contention: usize,
    pub reciprocal: usize,
    pub encounters: usize,
    pub inquiries: usize,
    pub transfers: u32,
    pub consumed: u32,
    pub final_food: Vec<u32>,
    pub final_hunger: Vec<i32>,
    pub bytes: usize,
}
pub fn summary(t: &Trial) -> Summary {
    let waves = t.periods.iter().flat_map(|p| &p.waves).collect::<Vec<_>>();
    let allocations = waves
        .iter()
        .flat_map(|w| &w.allocations)
        .collect::<Vec<_>>();
    let events = &t.final_state.base.base.base.base.base.events;
    Summary {
        seed: t.seed,
        case: t.case.clone(),
        mode: t.mode,
        config: t.config.clone(),
        invitations: allocations.len(),
        accepted: allocations
            .iter()
            .filter(|a| {
                matches!(
                    a.disposition,
                    e::Disposition::Execute
                        | e::Disposition::Contention
                        | e::Disposition::Reciprocal
                )
            })
            .count(),
        declined: allocations
            .iter()
            .filter(|a| a.disposition == e::Disposition::Declined)
            .count(),
        contention: allocations
            .iter()
            .filter(|a| a.disposition == e::Disposition::Contention)
            .count(),
        reciprocal: allocations
            .iter()
            .filter(|a| a.disposition == e::Disposition::Reciprocal)
            .count(),
        encounters: waves.iter().filter(|w| w.execution.is_some()).count(),
        inquiries: waves
            .iter()
            .filter(|w| {
                w.execution
                    .as_ref()
                    .is_some_and(|e| e.invitation.kind == e::Kind::Inquiry)
            })
            .count(),
        transfers: events[t.initial.events..]
            .iter()
            .map(|e| e.transferred)
            .sum(),
        consumed: t
            .periods
            .iter()
            .flat_map(|p| &p.consumption)
            .map(|c| c.consumed)
            .sum(),
        final_food: t
            .periods
            .last()
            .unwrap()
            .after
            .agents
            .iter()
            .map(|a| a.food)
            .collect(),
        final_hunger: t
            .periods
            .last()
            .unwrap()
            .after
            .agents
            .iter()
            .map(|a| a.hunger)
            .collect(),
        bytes: serde_json::to_vec(t).unwrap().len(),
    }
}
