//! Event-specific beliefs and evidence. No function here can inspect a partner.
use crate::model::*;
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

pub const RULES: &str = "experiment-002-v1";
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceKind {
    Testimony {
        scarce: bool,
    },
    /// Speaker voluntarily opens the instrumented record of their past circumstances.
    Disclosure,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Belief {
    pub event: u64,
    pub subject: AgentId,
    pub support: i32,
    pub evidence: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Revision {
    pub before: Memory,
    pub after: Memory,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InformationScene {
    pub id: u64,
    pub tick: u64,
    pub speaker: AgentId,
    pub listener: AgentId,
    pub event: u64,
    pub kind: EvidenceKind,
    pub scarce: bool,
    pub reliability: i32,
    pub before: Option<Belief>,
    pub after: Belief,
    pub revision: Option<Revision>,
    pub trust_before: i32,
    pub trust_after: i32,
    pub turns: u32,
    pub terminated: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Consumption {
    pub tick: u64,
    pub actor: AgentId,
    pub food_before: u32,
    pub food_after: u32,
    pub hunger_before: i32,
    pub hunger_after: i32,
    pub consumed: u32,
}
#[derive(Resource, Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cognition {
    pub beliefs: BTreeMap<AgentId, VecDeque<Belief>>,
    pub information: Vec<InformationScene>,
    pub consumption: Vec<Consumption>,
}

/// Equal or weaker evidence does not replace prior evidence, including repetition.
pub fn revise_belief(prior: Option<&Belief>, incoming: Belief) -> Belief {
    match prior {
        Some(old) if old.support.abs() >= incoming.support.abs() => old.clone(),
        _ => incoming,
    }
}
