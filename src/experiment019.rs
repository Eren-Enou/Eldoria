//! Preregistered019 matched worlds. Reuse018 physical/resolver implementation.
use crate::{
    encounters as e, experiment009, experiment013, experiment018 as old, opportunities as o,
    priorities as p, recurring as r, simulation::Simulation,
};
use serde::{Deserialize, Serialize};
pub const CASES: [&str; 19] = [
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
    "low_food",
    "useful_contact",
    "unproductive_contact",
];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub window: u64,
    pub idle_gap: u64,
    pub interval: u64,
    pub stock: u32,
    pub hunger: i32,
    pub order: e::Order,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            window: 24,
            idle_gap: 1,
            interval: 4,
            stock: 12,
            hunger: 20,
            order: e::Order::Ascending,
        }
    }
}
pub fn configurations(held: bool) -> Vec<(Config, Vec<&'static str>)> {
    if !held {
        return vec![(Config::default(), CASES.to_vec())];
    }
    let mut result = vec![];
    for order in [e::Order::Descending, e::Order::Rotating] {
        result.push((
            Config {
                order,
                ..Default::default()
            },
            vec!["competing_eater", "information", "refusal", "ongoing"],
        ));
    }
    for window in [6, 8, 12, 48] {
        result.push((
            Config {
                window,
                ..Default::default()
            },
            vec!["useful_contact", "information", "ongoing"],
        ));
    }
    for idle_gap in [2, 5] {
        result.push((
            Config {
                idle_gap,
                ..Default::default()
            },
            vec!["recurring", "information", "history"],
        ));
    }
    for interval in [2, 16] {
        result.push((
            Config {
                interval,
                ..Default::default()
            },
            vec!["information", "ongoing"],
        ));
    }
    for stock in [4, 20] {
        result.push((
            Config {
                stock,
                ..Default::default()
            },
            vec!["ongoing"],
        ));
    }
    for hunger in [0, 60] {
        result.push((
            Config {
                hunger,
                ..Default::default()
            },
            vec!["ongoing"],
        ));
    }
    result.push((
        Config {
            window: 14,
            ..Default::default()
        },
        vec!["information"],
    ));
    result
}
pub fn setup(seed: u64, case: &str, c: &Config) -> Simulation {
    let base = match case {
        "low_food" => "recurring",
        "useful_contact" => "mutual",
        "unproductive_contact" => "refusal",
        _ => case,
    };
    let mut sim = old::setup(
        seed,
        base,
        &old::Config {
            stock: c.stock,
            interval: c.interval,
            ..Default::default()
        },
    );
    if case == "low_food" {
        for id in 0..3 {
            sim.set_circumstances(id, 1, 45, 0, 100).unwrap();
        }
    }
    if case == "ongoing" && c.hunger != 20 {
        for id in 0..3 {
            sim.set_circumstances(id, c.stock, c.hunger, 60, 20)
                .unwrap();
        }
    }
    if matches!(case, "useful_contact" | "unproductive_contact") {
        sim.set_circumstances(0, 2, 80, 60, 20).unwrap();
        sim.set_circumstances(
            1,
            6,
            20,
            if case == "useful_contact" { 100 } else { 0 },
            if case == "useful_contact" { 0 } else { 100 },
        )
        .unwrap();
        sim.set_circumstances(2, 0, 0, 0, 100).unwrap();
    }
    sim
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pause {
    pub period: usize,
    pub step: usize,
    pub from: u64,
    pub to: u64,
    pub before: usize,
    pub after: usize,
    pub need_before: usize,
    pub need_after: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub case: String,
    pub mode: p::Mode,
    pub config: Config,
    pub initial: o::Point,
    pub profiles: Vec<crate::intentional::Profile>,
    pub points: Vec<o::Point>,
    pub periods: Vec<old::Period>,
    pub final_state: experiment013::Snapshot,
    pub clock: r::Clock,
    pub needs: Vec<r::Record>,
    pub pauses: Vec<Pause>,
}
fn capture(sim: &Simulation, points: &mut Vec<o::Point>) -> usize {
    let point = sim.recurring_point();
    if points.last() != Some(&point) {
        points.push(point)
    }
    points.len() - 1
}
pub fn trial(seed: u64, mode: p::Mode, case: &str, config: Config) -> Trial {
    assert!(
        CASES.contains(&case)
            && [1, 2, 5].contains(&config.idle_gap)
            && config.window > 0
            && config.window <= 128
    );
    let mut sim = setup(seed, case, &config);
    let initial = sim.recurring_point();
    let profiles = (0..3)
        .map(|id| sim.intentional().unwrap().profiles[&id].clone())
        .collect();
    let total = if matches!(case, "ongoing" | "longitudinal") {
        576
    } else if case == "missed" {
        12
    } else {
        144
    };
    let target = sim.activity_tick() + total;
    let mut points = vec![];
    let mut periods = vec![];
    let mut pauses = vec![];
    while sim.activity_tick() < target {
        let before = capture(&sim, &mut points);
        let start = sim.activity_tick();
        let budget = (if case == "missed" { 2 } else { config.window }).min(target - start);
        let end = start + budget;
        sim.recurring_open(mode.legacy(), config.order, budget, config.interval)
            .unwrap();
        sim.set_recurring_policy(mode.selector()).unwrap();
        let mut steps = vec![];
        while sim.activity_tick() < end {
            let step = sim.recurring_step(&mut points).unwrap();
            let all_wait = step.activity.active_before.is_none()
                && step.activity.responses.is_empty()
                && step
                    .activity
                    .decisions
                    .iter()
                    .all(|d| d.as_ref().is_some_and(|d| d.selected == o::Action::Wait));
            let extra = (config.idle_gap - 1).min(end - sim.activity_tick());
            if all_wait && extra > 0 {
                let from = sim.activity_tick();
                let need_before = step.need_after;
                sim.priorities_idle(extra).unwrap();
                let after = capture(&sim, &mut points);
                pauses.push(Pause {
                    period: periods.len(),
                    step: steps.len(),
                    from,
                    to: sim.activity_tick(),
                    before: step.activity.after,
                    after,
                    need_before,
                    need_after: sim.priorities_need_count(),
                });
            }
            steps.push(step);
        }
        sim.recurring_finish().unwrap();
        let after = capture(&sim, &mut points);
        periods.push(old::Period {
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
        pauses,
    }
}
/// Observer-only discrete pressure-time, after completed actions at each tick.
pub fn hunger_area(t: &Trial) -> [u64; 3] {
    let mut area = [0; 3];
    for n in &t.needs {
        for tick in n.from..n.to {
            let pulses = (tick - t.clock.epoch) / t.clock.settings.interval
                - (n.from - t.clock.epoch) / t.clock.settings.interval;
            for (i, value) in area.iter_mut().enumerate() {
                *value += r::pressure(n.before[i], pulses, t.clock.settings.increment) as u64;
            }
        }
    }
    area
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Summary {
    pub seed: u64,
    pub case: String,
    pub mode: p::Mode,
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
    pub hunger_area: [u64; 3],
    pub pause_ticks: u64,
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
            .sum::<u64>()
            + t.pauses.iter().map(|p| p.to - p.from).sum::<u64>(),
        consumed,
        final_food: last.agents.iter().map(|a| a.food).collect(),
        final_hunger: last.agents.iter().map(|a| a.hunger).collect(),
        bytes: serde_json::to_vec(t).unwrap().len(),
        hunger_area: hunger_area(t),
        pause_ticks: t.pauses.iter().map(|p| p.to - p.from).sum(),
    }
}
