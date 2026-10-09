//! 011-only transient heuristic. No future simulation or persistent cognition.
use crate::{
    attention::{self, Basis, Record},
    inquiry as q, provenance as p,
};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Unchanged,
    CurrentNeed,
    StatusValue,
}
pub const MODES: [Mode; 3] = [Mode::Unchanged, Mode::CurrentNeed, Mode::StatusValue];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Components {
    pub action: q::Action,
    pub support: i32,
    pub distance: i32,
    pub expected: i32,
    pub leverage: i32,
    pub value: i32,
}
/// Derived from existing local candidate and <=8 own current support triples.
pub fn components(c: &q::Candidate, basis: &[Basis]) -> Components {
    let support = if let q::Action::Ask { concern, .. } = c.action {
        basis
            .iter()
            .find(|b| b.concern == concern)
            .map_or(0, |b| b.support)
    } else {
        0
    };
    let distance = (60 - support.abs()).max(1);
    let leverage = (c.expected_gain * 100 / distance).min(100);
    Components {
        action: c.action,
        support,
        distance,
        expected: c.expected_gain,
        leverage,
        value: (c.expected_gain + leverage) / 2,
    }
}
pub fn policy(mut base: p::Decision, basis: &[Basis]) -> p::Decision {
    for c in &mut base.decision.candidates {
        if matches!(c.action, q::Action::Ask { .. }) {
            c.benefit = c.importance * components(c, basis).value / 100;
            c.score = c.benefit - c.cost as i32;
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
pub fn compare(mode: Mode, base: p::Decision, basis: &[Basis]) -> p::Decision {
    match mode {
        Mode::Unchanged => base,
        Mode::CurrentNeed => attention::policy(base, basis),
        Mode::StatusValue => policy(base, basis),
    }
}
/// Observer configuration/capture only; never supplied to cognition or scoring.
#[derive(Resource, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueAudit {
    pub mode: Mode,
    pub first_query: usize,
    pub records: Vec<Record>,
}
#[derive(Resource)]
pub(crate) struct Policy(pub fn(p::Decision, &[Basis]) -> p::Decision);
