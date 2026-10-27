//!027 actor-local extraction frequency; no world/ledger access in update or rank.
use crate::{delayed, preparation as m};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
include!(concat!(env!("OUT_DIR"), "/experience_stamp.rs"));
pub const CAPACITY: usize = 2;
pub const LIFETIME: u64 = 24;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Fixed,
    Experience,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cell {
    pub owner: u32,
    pub received: u64,
    pub success: bool,
}
pub fn retain(cells: &[Cell], now: u64) -> Vec<Cell> {
    cells
        .iter()
        .filter(|c| now < c.received + LIFETIME)
        .cloned()
        .collect()
}
pub fn update(cells: &[Cell], signal: Cell) -> (Vec<Cell>, Option<Cell>) {
    assert!(cells.len() <= CAPACITY && signal.owner < 4);
    assert!(
        cells
            .iter()
            .all(|c| c.owner == signal.owner && c.received <= signal.received)
    );
    let mut next = retain(cells, signal.received);
    let evicted = (next.len() == CAPACITY).then(|| next.remove(0));
    next.push(signal);
    (next, evicted)
}
pub fn expectation(cells: &[Cell]) -> i32 {
    assert!(cells.len() <= CAPACITY);
    if cells.is_empty() {
        50
    } else {
        100 * cells.iter().filter(|c| c.success).count() as i32 / cells.len() as i32
    }
}
pub fn rank(mut fixed: m::Selection, cells: &[Cell], mode: Mode) -> m::Selection {
    if mode == Mode::Experience && fixed.terms[4].eligible {
        let delta = (expectation(cells) - 50) / 5;
        fixed.terms[4].base_score += delta;
        fixed.terms[4].final_score += delta;
        fixed.selected = fixed
            .terms
            .iter()
            .fold(&fixed.terms[0], |a, b| {
                if b.final_score > a.final_score { b } else { a }
            })
            .action;
    }
    fixed
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt {
    pub memory: u64,
    pub signal: Cell,
    pub before: Vec<Cell>,
    pub after: Vec<Cell>,
    pub evicted: Option<Cell>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub boundary: usize,
    pub choice: usize,
    pub before: Vec<Cell>,
    pub retained: Vec<Cell>,
    pub expected: i32,
    pub adjustment: i32,
}
#[derive(Resource, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Learning {
    pub mode: Mode,
    pub cells: [Vec<Cell>; 4],
    /// Observer output only, never read by receive/choose.
    pub receipts: Vec<Receipt>,
    pub decisions: Vec<Decision>,
}
impl Learning {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            cells: std::array::from_fn(|_| vec![]),
            receipts: vec![],
            decisions: vec![],
        }
    }
    pub fn receive(&mut self, owner: u32, received: u64, kind: m::Change, memory: u64) {
        if self.mode == Mode::Fixed || !matches!(kind, m::Change::Taken | m::Change::Insufficient) {
            return;
        }
        let signal = Cell {
            owner,
            received,
            success: kind == m::Change::Taken,
        };
        let cells = &mut self.cells[owner as usize];
        let before = cells.clone();
        let (after, evicted) = update(cells, signal.clone());
        *cells = after.clone();
        self.receipts.push(Receipt {
            memory,
            signal,
            before,
            after,
            evicted,
        });
    }
    pub fn choose(
        &mut self,
        input: &m::Input,
        fixed: m::Selection,
        boundary: usize,
        choice: usize,
    ) -> m::Selection {
        let own = &input.inherited.legacy;
        let cells = &mut self.cells[own.own.id as usize];
        let before = cells.clone();
        *cells = retain(cells, own.now);
        let expected = expectation(cells);
        let adjustment = if self.mode == Mode::Experience && fixed.terms[4].eligible {
            (expected - 50) / 5
        } else {
            0
        };
        self.decisions.push(Decision {
            boundary,
            choice,
            before,
            retained: cells.clone(),
            expected,
            adjustment,
        });
        rank(fixed, cells, self.mode)
    }
    pub fn finish(&mut self, now: u64) {
        for cells in &mut self.cells {
            *cells = retain(cells, now);
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub source025: usize,
    pub mode: Mode,
    pub world: m::Config,
}
pub fn matrix() -> Vec<Config> {
    serde_json::from_str(include_str!(
        "../../../experiments/027/registered-matrix.json"
    ))
    .unwrap()
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub schema: String,
    pub source: String,
    pub config: Config,
    pub body: m::Trial,
    pub learning: Learning,
}
pub fn trial(config: Config) -> Trial {
    assert_eq!(
        config,
        matrix()[2 * config.source025 + usize::from(config.mode == Mode::Experience)]
    );
    let (body, learning) = crate::simulation::Simulation::experience_trial27(
        config.world.clone(),
        delayed::Mode::ProjectedNeed,
        config.mode,
    );
    Trial {
        schema: "experiment-027/v1".into(),
        source: SOURCE_ID.into(),
        config,
        body,
        learning,
    }
}
