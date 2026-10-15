//! Preregistered016 fixtures; no policy sees case/config/future changes.
use crate::{
    cognition::Consumption, contact_learning as c, encounters as e, experiment009, experiment013,
    experiment015 as old, intentional::Profile, simulation::Simulation,
};
use serde::{Deserialize, Serialize};
pub const CASES: [&str; 14] = [
    "decline",
    "late_willing",
    "brief_willing",
    "alternative",
    "unavailable",
    "contended",
    "refusal",
    "information",
    "misleading",
    "redundant",
    "separate",
    "mutual",
    "longitudinal",
    "missed",
];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub budget: u64,
    pub periods: usize,
    pub long_periods: usize,
    pub shift: u64,
    pub order: e::Order,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            budget: 24,
            periods: 6,
            long_periods: 24,
            shift: 7,
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
            shift: 3,
            ..Default::default()
        },
        Config {
            budget: 48,
            periods: 8,
            long_periods: 32,
            shift: 11,
            ..Default::default()
        },
    ]
}
pub fn setup(seed: u64, case: &str) -> Simulation {
    let original = match case {
        "decline" | "late_willing" | "brief_willing" => "private_mirror",
        "alternative" => "unilateral",
        "redundant" | "longitudinal" => "formerly_useful",
        _ => case,
    };
    let mut s = old::setup(seed, original);
    if case == "alternative" {
        s.set_circumstances(2, 4, 0, 100, 0).unwrap();
    }
    s
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileChange {
    pub tick: u64,
    pub actor: u32,
    pub before: Profile,
    pub after: Profile,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Period {
    pub start: u64,
    pub end: u64,
    pub busy_until: [u64; 3],
    pub before: e::State,
    pub waves: Vec<c::Wave>,
    pub consumption: Vec<Consumption>,
    pub after: e::State,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub case: String,
    pub mode: c::Mode,
    pub config: Config,
    pub initial: e::State,
    pub initial_profiles: Vec<Profile>,
    pub profile_changes: Vec<ProfileChange>,
    pub periods: Vec<Period>,
    pub interventions: Vec<old::Intervention>,
    pub final_cells: [Vec<c::Cell>; 3],
    pub final_state: experiment013::Snapshot,
}
pub fn trial(seed: u64, mode: c::Mode, case: &str, config: Config) -> Trial {
    assert!(CASES.contains(&case));
    let mut sim = setup(seed, case);
    sim.enable_contact_learning(mode).unwrap();
    let initial = sim.encounter_state();
    let initial_profiles = (0..3)
        .map(|id| sim.intentional().unwrap().profiles[&id].clone())
        .collect();
    let mut profile_changes = vec![];
    let mut periods = vec![];
    let mut interventions = vec![];
    let mut wave = 0;
    let count = if case == "longitudinal" {
        config.long_periods
    } else {
        config.periods
    };
    for period in 0..count {
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
            interventions.push(old::Intervention {
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
        } else if case == "longitudinal" && period % 2 == 0 {
            [start, start, start + config.shift]
        } else {
            [start; 3]
        };
        sim.encounter_open(budget, busy_until).unwrap();
        let mut waves = vec![];
        while sim.activity_tick() < end {
            let relative = sim.activity_tick() - start;
            let willing = match case {
                "late_willing" => Some(relative >= config.shift),
                "brief_willing" => Some(relative >= config.shift && relative < config.shift + 10),
                "longitudinal" => Some(period % 3 != 0),
                _ => None,
            };
            if let Some(willing) = willing {
                let before = sim.intentional().unwrap().profiles[&2].clone();
                let after = Profile {
                    relationship_goal: if willing { 100 } else { 0 },
                    privacy: if willing { 0 } else { 100 },
                    ..before.clone()
                };
                if before != after {
                    sim.set_communication_profile(2, after.clone()).unwrap();
                    profile_changes.push(ProfileChange {
                        tick: sim.activity_tick(),
                        actor: 2,
                        before,
                        after,
                    });
                }
            }
            waves.push(sim.contact_wave(config.order, wave).unwrap());
            wave += 1;
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
    let final_cells = sim.contact_cells();
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
        initial_profiles,
        profile_changes,
        periods,
        interventions,
        final_cells,
        final_state,
    }
}
pub fn suite(seed: u64, held: bool) -> Vec<Trial> {
    let configs = if held {
        sensitivity().to_vec()
    } else {
        vec![Config::default()]
    };
    let mut result = vec![];
    for config in configs {
        for case in CASES {
            if config.budget != 24
                && ![
                    "late_willing",
                    "brief_willing",
                    "unavailable",
                    "longitudinal",
                ]
                .contains(&case)
            {
                continue;
            }
            for mode in c::MODES {
                result.push(trial(seed, mode, case, config.clone()));
            }
        }
    }
    result
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Summary {
    pub seed: u64,
    pub case: String,
    pub mode: c::Mode,
    pub config: Config,
    pub invitations: usize,
    pub declined: usize,
    pub unmet: usize,
    pub meetings: usize,
    pub queries: usize,
    pub transfers: u32,
    pub consumed: u32,
    pub final_food: Vec<u32>,
    pub final_hunger: Vec<i32>,
    pub evictions: usize,
    pub bytes: usize,
}
pub fn summary(t: &Trial) -> Summary {
    let waves = t.periods.iter().flat_map(|p| &p.waves).collect::<Vec<_>>();
    let signals = waves.iter().flat_map(|w| &w.updates).collect::<Vec<_>>();
    let events = &t.final_state.base.base.base.base.base.events;
    let final_agents = &t.periods.last().unwrap().after.agents;
    Summary {
        seed: t.seed,
        case: t.case.clone(),
        mode: t.mode,
        config: t.config.clone(),
        invitations: signals.len(),
        declined: signals
            .iter()
            .filter(|u| u.signal.outcome == c::Outcome::Declined)
            .count(),
        unmet: signals
            .iter()
            .filter(|u| u.signal.outcome == c::Outcome::AcceptedUnmet)
            .count(),
        meetings: waves.iter().filter(|w| w.wave.execution.is_some()).count(),
        queries: waves
            .iter()
            .filter(|w| {
                w.wave
                    .execution
                    .as_ref()
                    .is_some_and(|x| x.invitation.kind == e::Kind::Inquiry)
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
        final_food: final_agents.iter().map(|a| a.food).collect(),
        final_hunger: final_agents.iter().map(|a| a.hunger).collect(),
        evictions: signals.iter().filter(|u| u.evicted.is_some()).count(),
        bytes: serde_json::to_vec(t).unwrap().len(),
    }
}
