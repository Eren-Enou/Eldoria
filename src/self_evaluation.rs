//! 012-only bounded learning of observed assessment change, separate from novelty.
use crate::{assessment as a, concerns::Status, inquiry as q, model::AgentId, provenance as p};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

pub const CAPACITY: usize = 32;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Unchanged,
    GlobalProgress,
    Contextual,
}
pub const MODES: [Mode; 3] = [Mode::Unchanged, Mode::GlobalProgress, Mode::Contextual];
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Context {
    Any,
    ClaimFallback,
    EvidencePresent,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cell {
    pub source: AgentId,
    pub strategy: q::Strategy,
    pub context: Context,
    pub attempts: u32,
    pub expected_change: i32,
    pub last_inquiry: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Basis {
    pub concern: u64,
    pub event: u64,
    pub support: i32,
    pub context: Context,
}
pub fn project(concerns: &[crate::concerns::Concern], items: &[a::Item]) -> Vec<Basis> {
    concerns
        .iter()
        .filter(|c| c.status.active())
        .map(|c| Basis {
            concern: c.id,
            event: c.event,
            support: a::grouped(items, c.event),
            context: if items.iter().any(|i| {
                i.event == c.event && i.quality > 0 && !matches!(i.origin, a::Origin::Claim(_))
            }) {
                Context::EvidencePresent
            } else {
                Context::ClaimFallback
            },
        })
        .collect()
}
pub fn key_context(mode: Mode, context: Context) -> Context {
    if mode == Mode::GlobalProgress {
        Context::Any
    } else {
        context
    }
}
/// Only bounded own cells and current own basis enter this replaceable scorer.
pub fn policy(mut base: p::Decision, mode: Mode, basis: &[Basis], cells: &[Cell]) -> p::Decision {
    if mode == Mode::Unchanged {
        return base;
    }
    for c in &mut base.decision.candidates {
        if let q::Action::Ask {
            concern,
            source,
            strategy,
        } = c.action
        {
            let context = key_context(
                mode,
                basis.iter().find(|b| b.concern == concern).unwrap().context,
            );
            if let Some(cell) = cells
                .iter()
                .find(|s| s.source == source && s.strategy == strategy && s.context == context)
            {
                let gain = c
                    .expected_gain
                    .min(cell.expected_change.max(c.opportunity_value));
                c.benefit = c.importance * gain / 100;
                c.score = c.benefit - c.cost as i32;
            }
        }
    }
    base.decision.selected = base
        .decision
        .candidates
        .iter()
        .fold((q::Action::Pause, 0), |best, c| {
            if c.score > best.1 {
                (c.action, c.score)
            } else {
                best
            }
        })
        .0;
    base
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Outcome {
    pub support_before: i32,
    pub support_after: i32,
    pub status_before: Status,
    pub status_after: Status,
    pub receipt_acquired: bool,
    pub novelty: q::Novelty,
    pub novelty_value: i32,
    pub time: u32,
}
/// Current interaction result available to its actor. No responder input, motives,
/// private inventory, objective roots, observer history or predicted future reply.
pub struct Experience {
    pub inquiry: u64,
    pub owner: AgentId,
    pub selected: q::Action,
    pub valid_learning: bool,
    pub status_before: Option<Status>,
    pub status_after: Option<Status>,
    pub receipt_acquired: bool,
    pub novelty: q::Novelty,
    pub novelty_value: i32,
    pub time: u32,
}
impl Outcome {
    pub fn changed(&self) -> bool {
        self.support_before != self.support_after || self.status_before != self.status_after
    }
    pub fn target(&self) -> i32 {
        if self.changed() { 100 } else { 0 }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub inquiry: u64,
    pub owner: AgentId,
    pub basis: Vec<Basis>,
    pub assessment_before: usize,
    pub assessment_after: usize,
    pub selected: q::Action,
    pub outcome: Option<Outcome>,
    pub before: Option<Cell>,
    pub after: Option<Cell>,
    pub evicted: Option<Cell>,
}
#[derive(Resource, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Learning {
    pub mode: Mode,
    pub first_inquiry: usize,
    pub cells: BTreeMap<AgentId, VecDeque<Cell>>,
    /// Observer-only; never consulted to select/update a live cell.
    pub records: Vec<Record>,
}
impl Learning {
    pub fn new(mode: Mode, first_inquiry: usize) -> Self {
        Self {
            mode,
            first_inquiry,
            cells: BTreeMap::new(),
            records: vec![],
        }
    }
    pub fn learn(
        &mut self,
        owner: AgentId,
        key: (AgentId, q::Strategy, Context),
        target: i32,
        inquiry: u64,
    ) -> (Option<Cell>, Cell, Option<Cell>) {
        let (source, strategy, context) = key;
        let context = key_context(self.mode, context);
        let cells = self.cells.entry(owner).or_default();
        let index = cells
            .iter()
            .position(|s| s.source == source && s.strategy == strategy && s.context == context);
        let before = index.map(|i| cells[i].clone());
        let after = Cell {
            source,
            strategy,
            context,
            attempts: before.as_ref().map_or(1, |s| s.attempts.saturating_add(1)),
            expected_change: (before
                .as_ref()
                .map_or(strategy.prior(), |s| s.expected_change)
                + target)
                / 2,
            last_inquiry: inquiry,
        };
        let mut evicted = None;
        if let Some(i) = index {
            cells[i] = after.clone();
        } else {
            if cells.len() == CAPACITY {
                evicted = cells.pop_front();
            }
            cells.push_back(after.clone());
        }
        (before, after, evicted)
    }
}
#[derive(Resource)]
pub(crate) struct Policy(pub fn(p::Decision, Mode, &[Basis], &[Cell]) -> p::Decision);
