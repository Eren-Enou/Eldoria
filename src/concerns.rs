//! Bounded unfinished intent. Policies receive no partner-private state.
use crate::{intentional::TalkAction, model::AgentId};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
pub const RULES: &str = "experiment-005-v1";
pub const CAPACITY: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Open,
    Partial,
    Resolved,
    Abandoned,
}
impl Status {
    pub fn active(self) -> bool {
        matches!(self, Self::Open | Self::Partial)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Question {
    RefusalScarcity,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Concern {
    pub id: u64,
    pub owner: AgentId,
    pub target: AgentId,
    pub event: u64,
    pub created_scene: u64,
    pub question: Question,
    pub importance: i32,
    /// Last locally assessed uncertainty, retained if the bounded belief is evicted.
    pub uncertainty: i32,
    pub age: u32,
    pub attempts: u32,
    pub failures: u32,
    pub status: Status,
    pub last_receipt: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Creation {
    pub owner: AgentId,
    pub target: AgentId,
    pub event: u64,
    pub scene: u64,
    pub importance: i32,
    pub support: Option<i32>,
    pub receipt: Option<u64>,
    pub retained: bool,
}
pub fn worth_retaining(importance: i32, support: Option<i32>) -> bool {
    importance >= 40 && support.unwrap_or(0).abs() < 60
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FollowAction {
    Continue,
    Reopen(u64),
    Abandon(u64),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowInput {
    pub owner: AgentId,
    pub partner: AgentId,
    pub scene: u64,
    pub hunger: i32,
    pub relationship_goal: i32,
    pub answer_probability: i32,
    pub reliability: i32,
    pub pursuit_cost: u32,
    pub concerns: Vec<Concern>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowCandidate {
    pub action: FollowAction,
    pub expected_gain: i32,
    pub benefit: i32,
    pub cost: i32,
    pub score: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowDecision {
    pub input: FollowInput,
    pub candidates: Vec<FollowCandidate>,
    pub selected: FollowAction,
}
pub fn legal(input: &FollowInput, action: FollowAction) -> bool {
    match action {
        FollowAction::Continue => true,
        FollowAction::Reopen(id) | FollowAction::Abandon(id) => input.concerns.iter().any(|c| {
            c.id == id
                && c.owner == input.owner
                && c.target == input.partner
                && c.created_scene < input.scene
                && c.status.active()
        }),
    }
}
pub fn policy(input: &FollowInput) -> FollowDecision {
    let mut candidates = vec![FollowCandidate {
        action: FollowAction::Continue,
        expected_gain: 0,
        benefit: 0,
        cost: 0,
        score: 0,
    }];
    for c in &input.concerns {
        if !legal(input, FollowAction::Reopen(c.id)) {
            continue;
        }
        let gain = input.answer_probability * c.uncertainty * input.reliability / 10000;
        let benefit = c.importance / 2 + input.relationship_goal / 2 + gain / 2;
        // Saturate long-lived counters before converting to signed utility.
        let cost = input.pursuit_cost as i32
            + c.age.min(1000) as i32 * 4
            + c.failures.min(1000) as i32 * 20
            + input.hunger / 10;
        candidates.push(FollowCandidate {
            action: FollowAction::Reopen(c.id),
            expected_gain: gain,
            benefit,
            cost,
            score: benefit - cost,
        });
        candidates.push(FollowCandidate {
            action: FollowAction::Abandon(c.id),
            expected_gain: gain,
            benefit,
            cost,
            score: cost - benefit - 20,
        });
    }
    let selected = candidates
        .iter()
        .fold(
            &candidates[0],
            |best, c| if c.score > best.score { c } else { best },
        )
        .action;
    FollowDecision {
        input: input.clone(),
        candidates,
        selected,
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transition {
    pub id: u64,
    pub reason: String,
    pub before: Option<Concern>,
    pub after: Option<Concern>,
    pub receipt: Option<u64>,
    pub follow_up: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowRecord {
    pub id: u64,
    pub tick: u64,
    pub decision: FollowDecision,
    pub valid: bool,
    pub before: Option<Concern>,
    pub after: Option<Concern>,
    pub response_conversation: Option<u64>,
    pub actual_response: Option<TalkAction>,
    pub prediction_error: Option<i32>,
    pub answer_after: i32,
    pub learned: bool,
    /// Request/decision time only; linked talk records account for response time.
    pub time_spent: u32,
    pub food_before: [u32; 2],
    pub food_after: [u32; 2],
}
#[derive(Resource, Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Concerns {
    pub items: BTreeMap<AgentId, Vec<Concern>>,
    pub pursuit_costs: BTreeMap<AgentId, u32>,
    pub creations: Vec<Creation>,
    pub transitions: Vec<Transition>,
    pub records: Vec<FollowRecord>,
    pub next_id: u64,
    pub scene_cursor: usize,
}
impl Concerns {
    pub(crate) fn change(
        &mut self,
        reason: &str,
        before: Option<Concern>,
        after: Option<Concern>,
        receipt: Option<u64>,
        follow_up: Option<u64>,
    ) {
        self.transitions.push(Transition {
            id: self.transitions.len() as u64,
            reason: reason.into(),
            before,
            after,
            receipt,
            follow_up,
        });
    }
    pub(crate) fn capture(&mut self, creation: Creation) {
        if creation.retained {
            let owner = creation.owner;
            let items = self.items.entry(owner).or_default();
            if items.len() == CAPACITY {
                let index = items
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, c)| {
                        (
                            c.status.active(),
                            if c.status.active() { c.importance } else { 0 },
                            c.id,
                        )
                    })
                    .unwrap()
                    .0;
                let removed = items.remove(index);
                if removed.status.active() {
                    let mut abandoned = removed.clone();
                    abandoned.status = Status::Abandoned;
                    self.change(
                        "capacity abandonment",
                        Some(removed),
                        Some(abandoned.clone()),
                        None,
                        None,
                    );
                    self.change("capacity eviction", Some(abandoned), None, None, None);
                } else {
                    self.change("capacity eviction", Some(removed), None, None, None);
                }
            }
            let c = Concern {
                id: self.next_id,
                owner,
                target: creation.target,
                event: creation.event,
                created_scene: creation.scene,
                question: Question::RefusalScarcity,
                importance: creation.importance,
                uncertainty: 100 - creation.support.unwrap_or(0).abs(),
                age: 0,
                attempts: 0,
                failures: 0,
                status: if creation.support.is_some() {
                    Status::Partial
                } else {
                    Status::Open
                },
                last_receipt: creation.receipt,
            };
            self.next_id += 1;
            self.items.entry(owner).or_default().push(c.clone());
            self.change(
                "consequential unresolved refusal",
                None,
                Some(c),
                creation.receipt,
                None,
            );
        }
        self.creations.push(creation);
    }
    pub(crate) fn receive(&mut self, owner: AgentId, event: u64, support: i32, receipt: u64) {
        let Some(items) = self.items.get_mut(&owner) else {
            return;
        };
        let Some(c) = items.iter_mut().find(|c| c.event == event) else {
            return;
        };
        let before = c.clone();
        c.uncertainty = 100 - support.abs();
        c.last_receipt = Some(receipt);
        if c.status != Status::Abandoned {
            c.status = if support.abs() >= 60 {
                Status::Resolved
            } else {
                Status::Partial
            };
        }
        let after = c.clone();
        self.change(
            "local belief update",
            Some(before),
            Some(after),
            Some(receipt),
            None,
        );
    }
}
#[derive(Resource)]
pub(crate) struct ConcernPolicy(pub fn(&FollowInput) -> FollowDecision);
