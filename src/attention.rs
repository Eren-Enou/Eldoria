//! 010-only projection of currently retained own support into existing inquiry.
use crate::{assessment as a, concerns::Concern, inquiry as q, provenance as p};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Unchanged,
    CurrentNeed,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Basis {
    pub concern: u64,
    pub event: u64,
    pub support: i32,
}
pub fn project(concerns: &[Concern], items: &[a::Item]) -> Vec<Basis> {
    concerns
        .iter()
        .filter(|c| c.status.active())
        .map(|c| Basis {
            concern: c.id,
            event: c.event,
            support: a::grouped(items, c.event),
        })
        .collect()
}
/// Receives an already actor-local 007 decision and <=8 own summaries only.
pub fn policy(mut base: p::Decision, basis: &[Basis]) -> p::Decision {
    for candidate in &mut base.decision.candidates {
        if let q::Action::Ask { concern, .. } = candidate.action {
            let support = basis
                .iter()
                .find(|b| b.concern == concern)
                .map_or(0, |b| b.support);
            candidate.benefit = candidate.benefit * (100 - support.abs()) / 100;
            candidate.score = candidate.benefit - candidate.cost as i32;
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
pub struct Record {
    pub query: u64,
    pub inquiry: u64,
    pub assessment_end: usize,
    pub basis: Vec<Basis>,
}
#[derive(Resource, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attention {
    pub mode: Mode,
    pub first_query: usize,
    pub records: Vec<Record>,
}
#[derive(Resource)]
pub(crate) struct Policy(pub fn(p::Decision, &[Basis]) -> p::Decision);
