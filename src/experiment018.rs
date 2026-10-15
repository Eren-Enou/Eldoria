//! Preregistered018 finite worlds; no post-setup circumstances or food interventions.
use crate::{
    encounters as e, experiment009, experiment013, experiment017, opportunities as o,
    recurring as r, simulation::Simulation,
};
use serde::{Deserialize, Serialize};
pub const CASES: [&str; 16] = [
    "recurring",
    "no_food",
    "mutual",
    "contended",
    "competing_eater",
    "history",
    "information",
    "misleading",
    "private_mirror",
    "formerly_useful",
    "refusal",
    "unilateral",
    "separate",
    "missed",
    "ongoing",
    "longitudinal",
];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub budget: u64,
    pub periods: usize,
    pub long_periods: usize,
    pub order: e::Order,
    pub interval: u64,
    pub stock: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            budget: 24,
            periods: 6,
            long_periods: 24,
            order: e::Order::Ascending,
            interval: 4,
            stock: 12,
        }
    }
}
pub fn configurations(held: bool) -> Vec<(Config, Vec<&'static str>)> {
    if !held {
        return vec![(Config::default(), CASES.to_vec())];
    }
    let order_cases = vec!["competing_eater", "information", "refusal", "ongoing"];
    let rate_cases = vec!["recurring", "competing_eater", "information", "ongoing"];
    let mut cs = vec![];
    for order in [e::Order::Descending, e::Order::Rotating] {
        cs.push((
            Config {
                order,
                ..Default::default()
            },
            order_cases.clone(),
        ));
    }
    for interval in [2, 16] {
        cs.push((
            Config {
                interval,
                ..Default::default()
            },
            rate_cases.clone(),
        ));
    }
    cs.push((
        Config {
            budget: 48,
            ..Default::default()
        },
        rate_cases,
    ));
    for stock in [4, 20] {
        cs.push((
            Config {
                stock,
                ..Default::default()
            },
            vec!["ongoing"],
        ));
    }
    cs
}
pub fn setup(seed: u64, case: &str, config: &Config) -> Simulation {
    let original = match case {
        "recurring" | "no_food" => "separate",
        "ongoing" => "contended",
        "longitudinal" => "formerly_useful",
        _ => case,
    };
    let mut s = experiment017::setup(seed, original);
    if matches!(case, "recurring" | "no_food") {
        for id in 0..3 {
            s.set_circumstances(id, if case == "recurring" { 3 } else { 0 }, 0, 0, 100)
                .unwrap();
        }
    }
    if case == "ongoing" {
        for id in 0..3 {
            s.set_circumstances(id, config.stock, 20, 60, 20).unwrap();
        }
    }
    s
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Period {
    pub start: u64,
    pub end: u64,
    pub before: usize,
    pub steps: Vec<r::Step>,
    pub after: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub case: String,
    pub mode: r::Mode,
    pub config: Config,
    pub initial: o::Point,
    pub profiles: Vec<crate::intentional::Profile>,
    pub points: Vec<o::Point>,
    pub periods: Vec<Period>,
    pub final_state: experiment013::Snapshot,
    pub clock: r::Clock,
    pub needs: Vec<r::Record>,
}
fn point(s: &Simulation, points: &mut Vec<o::Point>) -> usize {
    let p = s.recurring_point();
    if points.last() != Some(&p) {
        points.push(p);
    }
    points.len() - 1
}
pub fn trial(seed: u64, mode: r::Mode, case: &str, config: Config) -> Trial {
    assert!(CASES.contains(&case));
    let mut sim = setup(seed, case, &config);
    let initial = sim.recurring_point();
    let profiles = (0..3)
        .map(|id| sim.intentional().unwrap().profiles[&id].clone())
        .collect();
    let mut points = vec![];
    let mut periods = vec![];
    let n = if matches!(case, "longitudinal" | "ongoing") {
        config.long_periods
    } else {
        config.periods
    };
    for _ in 0..n {
        let before = point(&sim, &mut points);
        let start = sim.activity_tick();
        let budget = if case == "missed" { 2 } else { config.budget };
        let end = start + budget;
        sim.recurring_open(mode, config.order, budget, config.interval)
            .unwrap();
        let mut steps = vec![];
        while sim.activity_tick() < end {
            steps.push(sim.recurring_step(&mut points).unwrap());
        }
        sim.recurring_finish().unwrap();
        let after = point(&sim, &mut points);
        periods.push(Period {
            start,
            end,
            before,
            steps,
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
        clock: sim.recurring_clock().unwrap(),
        needs: sim.recurring_records(),
    }
}
pub fn suite(seed: u64, held: bool) -> Vec<Trial> {
    let mut ts = vec![];
    for (config, cases) in configurations(held) {
        for case in cases {
            for mode in r::MODES {
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
    pub mode: r::Mode,
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
    pub need_updates: usize,
    pub pressure: u64,
    pub hunger_reduced: i32,
    pub idle_ticks: u64,
    pub consumed: u32,
    pub final_food: Vec<u32>,
    pub final_hunger: Vec<i32>,
    pub bytes: usize,
}
pub fn summary(t: &Trial) -> Summary {
    let steps = t
        .periods
        .iter()
        .flat_map(|p| p.steps.iter().map(|s| &s.activity))
        .collect::<Vec<_>>();
    let voluntary_consumed = steps
        .iter()
        .flat_map(|s| &s.consumption)
        .map(|c| c.consumed)
        .sum::<u32>();
    let consumed = voluntary_consumed;
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
        need_updates: t.needs.len(),
        pressure: t
            .needs
            .iter()
            .map(|r| r.pulses * u64::from(t.clock.settings.increment))
            .sum(),
        hunger_reduced: steps
            .iter()
            .flat_map(|s| &s.consumption)
            .map(|c| c.hunger_before - c.hunger_after)
            .sum(),
        idle_ticks: steps
            .iter()
            .filter(|s| {
                s.active_before.is_none()
                    && s.decisions
                        .iter()
                        .flatten()
                        .all(|d| d.selected == o::Action::Wait)
            })
            .map(|s| t.points[s.after].state.tick - t.points[s.before].state.tick)
            .sum(),
        consumed,
        final_food: last.agents.iter().map(|a| a.food).collect(),
        final_hunger: last.agents.iter().map(|a| a.hunger).collect(),
        bytes: serde_json::to_vec(t).unwrap().len(),
    }
}
