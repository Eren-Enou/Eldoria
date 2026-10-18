//! Prospectively registered021 worlds;protected020 sources remain untouched.
pub use crate::experiment020::CASES;
use crate::{
    acquisition as a, encounters as e, environmental_knowledge as k, experiment009, experiment013,
    experiment019, recurring as r, simulation::Simulation,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub duration: u64,
    pub window: u64,
    pub order: e::Order,
    pub cadence: u64,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            duration: 4,
            window: 24,
            order: e::Order::Ascending,
            cadence: 8,
        }
    }
}
pub fn config(case: &str) -> Config {
    Config {
        window: if case == "last_window" { 4 } else { 24 },
        order: if case == "lost_contact" {
            e::Order::Descending
        } else {
            e::Order::Ascending
        },
        ..Default::default()
    }
}
fn circumstances(case: &str) -> ([u32; 3], [i32; 3], [i32; 3], [i32; 3], u32) {
    match case {
        "single_start" => ([0, 0, 0], [80, 0, 0], [0, 0, 0], [100, 100, 100], 3),
        "contact_bias" => ([0, 0, 0], [100, 100, 100], [60, 60, 60], [20, 20, 20], 2),
        "empty" => ([0, 0, 0], [80, 80, 80], [60, 60, 60], [20, 20, 20], 0),
        "stale_pair" | "last_window" => ([0, 0, 0], [80, 80, 0], [0, 0, 0], [100, 100, 100], 1),
        "contested" => ([0, 0, 0], [80, 80, 80], [0, 0, 0], [100, 100, 100], 1),
        "lost_contact" => ([0, 1, 0], [80, 20, 80], [60, 100, 0], [20, 0, 100], 1),
        "sharing" => ([0, 0, 0], [20, 80, 0], [100, 0, 0], [0, 100, 100], 1),
        "stranded" => ([0, 0, 0], [100, 100, 100], [60, 60, 60], [20, 20, 20], 2),
        "ongoing" => ([8, 8, 8], [20, 20, 20], [60, 60, 60], [20, 20, 20], 12),
        "information" => ([4, 4, 2], [20, 20, 20], [60, 60, 60], [20, 20, 20], 2),
        "private_helpful" => ([0, 1, 0], [80, 20, 0], [60, 100, 0], [20, 0, 100], 1),
        "private_cautious" => ([0, 1, 0], [80, 20, 0], [60, 0, 0], [20, 100, 100], 1),
        "null" => ([0, 0, 0], [0, 0, 0], [0, 0, 0], [100, 100, 100], 0),
        _ => panic!("unknown020 case"),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Period {
    pub start: u64,
    pub end: u64,
    pub before: usize,
    pub steps: Vec<k::Step>,
    pub after: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub case: String,
    pub policy: a::Policy,
    pub mode: k::Mode,
    pub config: Config,
    pub initial: a::Point,
    pub profiles: Vec<crate::intentional::Profile>,
    pub points: Vec<a::Point>,
    pub periods: Vec<Period>,
    pub final_state: experiment013::Snapshot,
    pub final_point: a::Point,
    pub records: Vec<a::Record>,
    pub clock: r::Clock,
    pub needs: Vec<r::Record>,
    pub initial_total: u64,
    pub memory_records: Vec<k::Record>,
    pub final_memory: Vec<Option<k::Cell>>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Registered {
    pub seed: u64,
    pub policy: a::Policy,
    pub mode: k::Mode,
    pub case: String,
    pub config: Config,
}
pub fn matrix() -> Vec<Registered> {
    let rows: Vec<Registered> =
        serde_json::from_str(include_str!("../experiments/021/registered-matrix.json")).unwrap();
    assert_eq!(rows.len(), 214);
    rows
}
// This wrapper never exposes Simulation while acquisition claims can exist.
struct World {
    sim: Simulation,
}
impl World {
    fn capture(&self, points: &mut Vec<a::Point>) -> usize {
        let p = self.sim.knowledge_point();
        if points.last() != Some(&p) {
            points.push(p);
        }
        points.len() - 1
    }
}
pub fn trial(seed: u64, policy: a::Policy, mode: k::Mode, case: &str, config: Config) -> Trial {
    assert!(
        CASES.contains(&case)
            && [2, 4, 8].contains(&config.duration)
            && [4, 24].contains(&config.window)
            && [4, 8, 16].contains(&config.cadence)
            && matches!(policy, a::Policy::Need | a::Policy::Acquire)
    );
    let base = match case {
        "stranded" => "history",
        "information" => "information",
        _ => "ongoing",
    };
    let mut world = World {
        sim: experiment019::setup(
            seed,
            base,
            &experiment019::Config {
                stock: 0,
                ..Default::default()
            },
        ),
    };
    let (food, hunger, generosity, caution, stock) = circumstances(case);
    for id in 0..3 {
        let n = id as usize;
        world
            .sim
            .set_circumstances(id, food[n], hunger[n], generosity[n], caution[n])
            .unwrap();
    }
    let cache = a::Cache {
        id: a::CacheId(0),
        capacity: stock,
        stock,
        accessible: true,
    };
    let initial_total = food.iter().map(|f| u64::from(*f)).sum::<u64>() + u64::from(cache.stock);
    world
        .sim
        .knowledge_install(cache, config.duration, policy, mode, config.cadence);
    let initial = world.sim.knowledge_point();
    let profiles = (0..3)
        .map(|id| world.sim.intentional().unwrap().profiles[&id].clone())
        .collect();
    let target = world.sim.activity_tick()
        + if matches!(case, "ongoing" | "stranded") {
            576
        } else {
            144
        };
    let mut points = vec![];
    let mut periods = vec![];
    while world.sim.activity_tick() < target {
        let before = world.capture(&mut points);
        let start = world.sim.activity_tick();
        let budget = config.window.min(target - start);
        let end = start + budget;
        world
            .sim
            .recurring_open(r::Mode::Recurring, config.order, budget, 4)
            .unwrap();
        let mut steps = vec![];
        while world.sim.activity_tick() < end {
            steps.push(
                world
                    .sim
                    .knowledge_step(&mut points, periods.len(), steps.len())
                    .unwrap(),
            );
        }
        world.sim.recurring_finish().unwrap();
        assert!(world.sim.knowledge_point().pending.is_empty());
        let after = world.capture(&mut points);
        periods.push(Period {
            start,
            end,
            before,
            steps,
            after,
        });
    }
    world.sim.validate_history_indexes().unwrap();
    let final_memory = world.sim.knowledge_finish();
    let memory_records = world.sim.knowledge_history();
    let final_state = experiment013::Snapshot {
        base: experiment009::snapshot(&world.sim),
        learning: world.sim.self_evaluation().unwrap(),
        mode: experiment013::Mode::Contextual,
    };
    final_state.validate().unwrap();
    Trial {
        seed,
        case: case.into(),
        policy,
        mode,
        config,
        memory_records,
        final_memory,
        initial,
        profiles,
        points,
        periods,
        final_state,
        final_point: world.sim.knowledge_point(),
        records: world.sim.knowledge_records(),
        clock: world.sim.recurring_clock().unwrap(),
        needs: world.sim.recurring_records(),
        initial_total,
    }
}
