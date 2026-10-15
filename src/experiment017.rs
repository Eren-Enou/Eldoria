//! Preregistered017 stationary initial conditions; no later busy/profile/need scripts.
use crate::{
    cognition::Consumption, encounters as e, experiment009, experiment013, experiment015,
    opportunities as o, simulation::Simulation,
};
use serde::{Deserialize, Serialize};
pub const CASES: [&str; 13] = [
    "mutual",
    "contended",
    "competing_eater",
    "history",
    "information",
    "misleading",
    "refusal",
    "unilateral",
    "separate",
    "private_mirror",
    "formerly_useful",
    "longitudinal",
    "missed",
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
pub fn setup(seed: u64, case: &str) -> Simulation {
    let original = match case {
        "competing_eater" => "contended",
        "longitudinal" => "formerly_useful",
        _ => case,
    };
    let mut s = experiment015::setup(seed, original);
    if case == "competing_eater" {
        s.set_circumstances(2, 2, 60, 100, 0).unwrap();
    }
    s
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Period {
    pub start: u64,
    pub end: u64,
    pub before: usize,
    pub steps: Vec<o::Step>,
    pub consumption: Vec<Consumption>,
    pub after: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub case: String,
    pub mode: o::Mode,
    pub config: Config,
    pub initial: o::Point,
    pub profiles: Vec<crate::intentional::Profile>,
    pub points: Vec<o::Point>,
    pub periods: Vec<Period>,
    pub final_state: experiment013::Snapshot,
}
fn point(s: &Simulation, points: &mut Vec<o::Point>) -> usize {
    let p = s.opportunity_point();
    if points.last() != Some(&p) {
        points.push(p);
    }
    points.len() - 1
}
pub fn trial(seed: u64, mode: o::Mode, case: &str, config: Config) -> Trial {
    assert!(CASES.contains(&case));
    let mut sim = setup(seed, case);
    let initial = sim.opportunity_point();
    let profiles = (0..3)
        .map(|id| sim.intentional().unwrap().profiles[&id].clone())
        .collect();
    let mut points = vec![];
    let mut periods = vec![];
    let n = if case == "longitudinal" {
        config.long_periods
    } else {
        config.periods
    };
    for _ in 0..n {
        let before = point(&sim, &mut points);
        let start = sim.activity_tick();
        let budget = if case == "missed" { 2 } else { config.budget };
        let end = start + budget;
        sim.opportunity_open(mode, config.order, budget).unwrap();
        let mut steps = vec![];
        while sim.activity_tick() < end {
            steps.push(sim.opportunity_step(&mut points).unwrap());
        }
        let consumption = sim.opportunity_finish().unwrap();
        let after = point(&sim, &mut points);
        periods.push(Period {
            start,
            end,
            before,
            steps,
            consumption,
            after,
        });
    }
    sim.validate_history_indexes().unwrap();
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
        profiles,
        points,
        periods,
        final_state,
    }
}
pub fn suite(seed: u64, held: bool) -> Vec<Trial> {
    let configs = if held {
        sensitivity().to_vec()
    } else {
        vec![Config::default()]
    };
    let mut ts = vec![];
    for config in configs {
        for case in CASES {
            if held
                && config.budget != 24
                && !["competing_eater", "history", "information", "longitudinal"].contains(&case)
            {
                continue;
            }
            for mode in o::MODES {
                ts.push(trial(seed, mode, case, config.clone()));
            }
        }
    }
    ts
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Summary {
    pub seed: u64,
    pub case: String,
    pub mode: o::Mode,
    pub config: Config,
    pub steps: usize,
    pub occupied_steps: usize,
    pub invitations: usize,
    pub declined: usize,
    pub unmet: usize,
    pub meetings: usize,
    pub queries: usize,
    pub transfers: u32,
    pub voluntary_consumed: u32,
    pub consumed: u32,
    pub final_food: Vec<u32>,
    pub final_hunger: Vec<i32>,
    pub bytes: usize,
}
pub fn summary(t: &Trial) -> Summary {
    let steps = t.periods.iter().flat_map(|p| &p.steps).collect::<Vec<_>>();
    let voluntary_consumed = steps
        .iter()
        .flat_map(|s| &s.consumption)
        .map(|c| c.consumed)
        .sum::<u32>();
    let consumed = voluntary_consumed
        + t.periods
            .iter()
            .flat_map(|p| &p.consumption)
            .map(|c| c.consumed)
            .sum::<u32>();
    let last = &t.points[t.periods.last().unwrap().after].state;
    let events = &t.final_state.base.base.base.base.base.events;
    assert_eq!(
        t.initial.state.agents.iter().map(|a| a.food).sum::<u32>(),
        last.agents.iter().map(|a| a.food).sum::<u32>() + consumed
    );
    Summary {
        seed: t.seed,
        case: t.case.clone(),
        mode: t.mode,
        config: t.config.clone(),
        steps: steps.len(),
        occupied_steps: steps.iter().filter(|s| s.active_before.is_some()).count(),
        invitations: steps
            .iter()
            .flat_map(|s| &s.decisions)
            .flatten()
            .filter(|d| d.selected == o::Action::Invite)
            .count(),
        declined: steps
            .iter()
            .flat_map(|s| &s.allocations)
            .filter(|a| a.disposition == e::Disposition::Declined)
            .count(),
        unmet: steps
            .iter()
            .flat_map(|s| &s.allocations)
            .filter(|a| a.disposition == e::Disposition::Contention)
            .count(),
        meetings: steps
            .iter()
            .filter(|s| s.started.is_some() || s.query.is_some())
            .count(),
        queries: last.queries - t.initial.state.queries,
        transfers: events[t.initial.state.events..]
            .iter()
            .map(|e| e.transferred)
            .sum(),
        voluntary_consumed,
        consumed,
        final_food: last.agents.iter().map(|a| a.food).collect(),
        final_hunger: last.agents.iter().map(|a| a.hunger).collect(),
        bytes: serde_json::to_vec(t).unwrap().len(),
    }
}
