use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

pub type AgentId = u32;
pub const MEMORY_CAPACITY: usize = 16;
pub const RULES_VERSION: &str = "experiment-001-v1";

#[derive(Component, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Agent {
    pub id: AgentId,
    pub food: u32,
    pub hunger: i32,
    pub generosity: i32,
    pub caution: i32,
    pub expectation: i32,
    pub trust: BTreeMap<AgentId, i32>,
    pub memories: VecDeque<Memory>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Memory {
    pub event: u64,
    pub partner: AgentId,
    pub observed: Action,
    pub interpretation: Interpretation,
    pub valence: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Interpretation {
    HelpReceived,
    HelpGiven,
    Rejected,
    ProtectedOwnNeeds,
    RequestHeard,
    OfferHeard,
    Departure,
    FailedTransfer,
    PossibleSelfProtection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Action {
    Accept,
    Offer,
    Request,
    Refuse,
    Leave,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signal {
    pub actor: AgentId,
    pub action: Action,
}

/// Deliberately contains no partner inventory, traits, motives, or memories.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Observation {
    pub partner: AgentId,
    pub last_signal: Option<Signal>,
    pub amount: u32,
    pub turns_left: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Candidate {
    pub action: Action,
    pub score: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub actor: AgentId,
    pub observation: Observation,
    pub food: u32,
    pub hunger: i32,
    pub generosity: i32,
    pub caution: i32,
    pub expectation: i32,
    pub trust: i32,
    pub episodic_valence: i32,
    pub memory_causes: Vec<u64>,
    pub candidates: Vec<Candidate>,
    pub selected: Action,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Agreement,
    Refusal,
    Withdrawal,
    Timeout,
    Inability,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scene {
    pub id: u64,
    pub participants: [AgentId; 2],
    pub amount: u32,
    pub max_turns: u32,
    pub turns: u32,
    pub signal: Option<Signal>,
    pub outcome: Option<Outcome>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub id: u64,
    pub tick: u64,
    pub scene: u64,
    pub participants: [AgentId; 2],
    pub decision: Decision,
    pub balances_before: [u32; 2],
    pub balances_after: [u32; 2],
    pub transferred: u32,
    pub outcome: Option<Outcome>,
    pub interpretations: [Memory; 2],
}

/// SplitMix64: wrapping arithmetic and high-word bounded draws are pinned rules.
pub struct Generator(u64);
impl Generator {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }
    fn draw(&mut self, bound: u32) -> u32 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^= z >> 31;
        (((z >> 32) * u64::from(bound)) >> 32) as u32
    }
    pub fn agent(&mut self, id: AgentId) -> Agent {
        Agent {
            id,
            food: self.draw(7),
            hunger: self.draw(101) as i32,
            generosity: self.draw(101) as i32,
            caution: self.draw(101) as i32,
            expectation: self.draw(101) as i32,
            trust: BTreeMap::new(),
            memories: VecDeque::new(),
        }
    }
}
