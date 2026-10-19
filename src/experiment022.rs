//! Opt-in research namespace; historical top-level module declarations are frozen.
use crate::{acquisition as a, encounters as e, environmental_knowledge as k, model::*};
use serde::{Deserialize, Serialize};

/// Owned protocol chronology, maintained at actual encounters; no episode content.
#[derive(
    bevy_ecs::prelude::Resource, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize,
)]
pub struct LocalContexts {
    pub completed: [u64; 4],
    pub created: [std::collections::BTreeMap<u64, u64>; 4],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub case: String,
    pub seed: u64,
    pub duration: u64,
    pub cadence: u64,
    pub horizon: u64,
    pub order: e::Order,
    pub capacity: usize,
}
pub fn matrix() -> Vec<Config> {
    serde_json::from_str(include_str!("../experiments/022/registered-matrix.json")).unwrap()
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Work {
    Send,
    Reply,
    Resource { scene: u64 },
    Claim,
    Meal,
    Query,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commitment {
    pub id: u64,
    pub work: Work,
    pub participants: Vec<u32>,
    pub start: u64,
    pub deadline: u64,
    pub source: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transition {
    pub tick: u64,
    pub commitment: Commitment,
    pub completed: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pending {
    pub source: usize,
    pub invitation: e::Invitation,
    pub question: crate::inquiry::Action,
    pub response: Option<ReplyRef>,
}
/// Immutable reference to the one original full recipient decision capture.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyRef {
    pub boundary: usize,
    pub response: usize,
    pub selected: e::Reply,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Admission {
    pub tick: u64,
    pub source: usize,
    pub invitation: e::Invitation,
    pub reason: String,
    pub commitment: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Need {
    pub tick: u64,
    pub before: [i32; 4],
    pub after: [i32; 4],
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Grant {
    pub commitment: u64,
    pub tick: u64,
    pub owner: u32,
    pub stock_before: u32,
    pub stock_after: u32,
    pub food_before: u32,
    pub food_after: u32,
    pub outcome: a::Outcome,
}
/// Store the full actor-local input once, rather than repeating it in base probes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Choice {
    pub input: k::Input,
    pub candidates: Vec<a::Candidate>,
    pub selected: a::Action,
    pub contact_candidates: Vec<e::Candidate>,
    pub contact: e::Intent,
}
impl From<k::Decision> for Choice {
    fn from(d: k::Decision) -> Self {
        Self {
            input: d.input,
            candidates: d.candidates,
            selected: d.selected,
            contact_candidates: d.base.contact.candidates,
            contact: d.base.contact.selected,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Boundary {
    pub tick: u64,
    pub agents: Vec<Agent>,
    pub stock: u32,
    pub active: Vec<Commitment>,
    pub pending: Vec<Pending>,
    pub choices: Vec<Choice>,
    pub responses: Vec<e::Response>,
    pub events: usize,
    pub meals: usize,
    pub grants: usize,
    pub concerns: usize,
    pub assessment: usize,
    pub queries: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub schema: String,
    pub config: Config,
    pub start: u64,
    pub end: u64,
    pub initial: Vec<Agent>,
    pub initial_stock: u32,
    pub initial_total: u64,
    pub boundaries: Vec<Boundary>,
    pub transitions: Vec<Transition>,
    pub admissions: Vec<Admission>,
    pub needs: Vec<Need>,
    pub grants: Vec<Grant>,
    pub memories: Vec<k::Record>,
    pub final_memory: Vec<Option<k::Cell>>,
    pub profiles: Vec<crate::intentional::Profile>,
    pub bootstrap_events: usize,
    pub initial_contexts: LocalContexts,
    pub final_contexts: LocalContexts,
    pub final_state: crate::experiment013::Snapshot,
    pub final_agents: Vec<Agent>,
    pub final_stock: u32,
}
pub fn trial(config: Config) -> Trial {
    crate::simulation::Simulation::concurrent_trial(config)
}
