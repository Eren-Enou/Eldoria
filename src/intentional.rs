//! Intentional communication policies see only this actor's local input.
use crate::{cognition::Belief, model::*};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};
pub const RULES: &str = "experiment-003-v1";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    pub privacy: i32,
    pub relationship_goal: i32,
    pub honesty: i32,
    pub request_cost: i32,
}
impl Default for Profile {
    fn default() -> Self {
        Self {
            privacy: 20,
            relationship_goal: 60,
            honesty: 80,
            request_cost: 40,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TalkAction {
    Silence,
    Explain,
    ProvideEvidence,
    AskExplanation,
    AskEvidence,
    Mislead,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TalkMemory {
    pub record: u64,
    pub event: u64,
    pub partner: AgentId,
    pub actor: AgentId,
    pub action: TalkAction,
    pub response_to: Option<TalkAction>,
    pub claim: Option<bool>,
    pub verified_claim: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TalkInput {
    pub own: Agent,
    pub profile: Profile,
    pub partner: AgentId,
    pub event: u64,
    pub is_refuser: bool,
    pub own_historical_scarcity: Option<bool>,
    pub last: Option<TalkAction>,
    pub belief: Option<Belief>,
    pub credibility: i32,
    pub memories: Vec<TalkMemory>,
    pub turns_left: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TalkCandidate {
    pub action: TalkAction,
    pub score: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TalkDecision {
    pub input: TalkInput,
    pub candidates: Vec<TalkCandidate>,
    pub selected: TalkAction,
}
pub fn legal(input: &TalkInput) -> Vec<TalkAction> {
    use TalkAction::*;
    let mut actions = vec![Silence];
    if input.turns_left == 0 {
        return actions;
    }
    if input.is_refuser {
        if input.own_historical_scarcity.is_some() {
            actions.push(ProvideEvidence);
            if input.last != Some(AskEvidence) {
                actions.push(Explain);
                if input.own_historical_scarcity == Some(false) {
                    actions.push(Mislead);
                }
            }
        }
    } else if input.turns_left >= 2 {
        match input.last {
            Some(Silence) => actions.push(AskExplanation),
            Some(Explain | Mislead)
                if input.belief.as_ref().is_none_or(|b| b.support.abs() < 100) =>
            {
                actions.push(AskEvidence)
            }
            _ => {}
        }
    }
    actions
}
pub fn policy(input: &TalkInput) -> TalkDecision {
    use TalkAction::*;
    let p = &input.profile;
    let trust = *input.own.trust.get(&input.partner).unwrap_or(&0);
    let value = p.relationship_goal + trust / 4;
    let response_history = input
        .memories
        .iter()
        .filter(|m| m.partner == input.partner && m.actor == input.partner)
        .map(|m| match m.action {
            ProvideEvidence => 10,
            Silence if matches!(m.response_to, Some(AskExplanation | AskEvidence)) => -20,
            _ => 0,
        })
        .sum::<i32>()
        .clamp(-40, 40);
    let candidates: Vec<_> = legal(input)
        .into_iter()
        .map(|action| {
            let score = match action {
                Silence => 0,
                Explain => {
                    (if input.own_historical_scarcity == Some(true) {
                        value
                    } else {
                        0
                    }) - p.privacy
                        - input.own.hunger / 5
                        - 10
                        + if input.last == Some(AskExplanation) {
                            20
                        } else {
                            0
                        }
                }
                ProvideEvidence => {
                    (if input.own_historical_scarcity == Some(true) {
                        value
                    } else {
                        p.honesty / 2
                    }) - p.privacy
                        - input.own.hunger / 5
                        - 30
                        + if input.last == Some(AskEvidence) {
                            50
                        } else {
                            0
                        }
                }
                Mislead => value - p.honesty - p.privacy / 2 - input.own.caution / 2 - 10,
                AskExplanation => {
                    p.relationship_goal + input.own.hunger / 5 + response_history - 30
                }
                AskEvidence => {
                    p.relationship_goal
                        + (100 - input.belief.as_ref().map_or(0, |b| b.support.abs())) / 2
                        + response_history
                        - p.request_cost
                }
            };
            TalkCandidate { action, score }
        })
        .collect();
    let selected = candidates
        .iter()
        .fold(
            &candidates[0],
            |best, c| if c.score > best.score { c } else { best },
        )
        .action;
    TalkDecision {
        input: input.clone(),
        candidates,
        selected,
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TalkRecord {
    pub id: u64,
    pub tick: u64,
    pub conversation: u64,
    pub decision: TalkDecision,
    pub valid: bool,
    pub claim: Option<bool>,
    /// Observer-only annotation, never a policy input or recipient signal.
    pub objectively_true: Option<bool>,
    pub verifiable: bool,
    pub information: Option<u64>,
    pub credibility_before: i32,
    pub credibility_after: i32,
    pub verified_claim: Option<bool>,
    pub time_spent: u32,
    pub food_before: [u32; 2],
    pub food_after: [u32; 2],
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conversation {
    pub id: u64,
    pub event: u64,
    pub participants: [AgentId; 2],
    pub first_record: usize,
    pub end_record: usize,
    pub outcome: String,
}
#[derive(Resource, Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Intentional {
    pub profiles: BTreeMap<AgentId, Profile>,
    pub memories: BTreeMap<AgentId, VecDeque<TalkMemory>>,
    pub credibility: BTreeMap<AgentId, BTreeMap<AgentId, i32>>,
    pub records: Vec<TalkRecord>,
    pub conversations: Vec<Conversation>,
}
pub(crate) fn remember(state: &mut Intentional, owner: AgentId, memory: TalkMemory) {
    let episodes = state.memories.entry(owner).or_default();
    if episodes.len() == MEMORY_CAPACITY {
        episodes.pop_front();
    }
    episodes.push_back(memory);
}
#[derive(Resource)]
pub(crate) struct TalkPolicy(pub fn(&TalkInput) -> TalkDecision);
