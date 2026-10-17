//! Opt-in020 physical cache, local choice and bounded live claims. No new cognition.
use crate::{encounters as e, opportunities as o, priorities};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheId(pub u32);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cache {
    pub id: CacheId,
    pub capacity: u32,
    pub stock: u32,
    pub accessible: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Observation {
    pub id: CacheId,
    pub observed_at: u64,
    pub stock: u32,
    pub accessible: bool,
    pub duration: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub legacy: e::Input,
    pub cache: Observation,
    pub inquiry_lane_free: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Policy {
    Ignored,
    Need,
    Acquire,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Wait,
    Consume,
    Invite,
    Acquire,
}
impl From<o::Action> for Action {
    fn from(value: o::Action) -> Self {
        match value {
            o::Action::Wait => Self::Wait,
            o::Action::Consume => Self::Consume,
            o::Action::Invite => Self::Invite,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Candidate {
    pub action: Action,
    pub eligible: bool,
    pub score: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub input: Input,
    pub policy: Policy,
    pub base: o::Decision,
    pub candidates: Vec<Candidate>,
    pub selected: Action,
}
pub fn eligible(i: &Input) -> bool {
    i.legacy
        .people
        .iter()
        .any(|p| p.id == i.legacy.own.id && p.available)
        && i.legacy.own.food == 0
        && i.legacy.own.hunger > 0
        && i.cache.stock > 0
        && i.cache.accessible
        && i.legacy
            .now
            .checked_add(i.cache.duration)
            .is_some_and(|t| t <= i.legacy.end)
}
pub fn policy(i: &Input, mode: Policy) -> Decision {
    let mut base = priorities::policy(&i.legacy, priorities::Mode::Baseline);
    if !i.inquiry_lane_free {
        for c in &mut base.contact.candidates {
            if matches!(
                c.intent,
                e::Intent::Invite {
                    kind: e::Kind::Inquiry,
                    ..
                }
            ) {
                c.eligible = false;
                c.score = -1;
            }
        }
        let top = base
            .contact
            .candidates
            .iter()
            .fold(&base.contact.candidates[0], |a, b| {
                if b.score > a.score { b } else { a }
            });
        base.contact.selected = top.intent;
        base.candidates[2].eligible = top.intent != e::Intent::Wait;
        base.candidates[2].score = if base.candidates[2].eligible {
            top.score
        } else {
            -1
        };
        base.selected = base
            .candidates
            .iter()
            .fold(
                &base.candidates[0],
                |a, b| if b.score > a.score { b } else { a },
            )
            .action;
    }
    let mut candidates = base
        .candidates
        .iter()
        .map(|c| Candidate {
            action: c.action.into(),
            eligible: c.eligible,
            score: c.score,
        })
        .collect::<Vec<_>>();
    let can = eligible(i);
    candidates.push(Candidate {
        action: Action::Acquire,
        eligible: can,
        score: if can { i.legacy.own.hunger } else { -1 },
    });
    let selected = match mode {
        Policy::Ignored => base.selected.into(),
        Policy::Acquire if can => Action::Acquire,
        _ => {
            candidates
                .iter()
                .fold(&candidates[0], |a, b| if b.score > a.score { b } else { a })
                .action
        }
    };
    Decision {
        input: i.clone(),
        policy: mode,
        base,
        candidates,
        selected,
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attempt {
    pub id: u64,
    pub cache: CacheId,
    pub actor: u32,
    pub start: u64,
    pub end: u64,
    pub wave: u64,
    pub selection: [usize; 3],
    pub observation: Observation,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Granted,
    Depleted,
    Overflow,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub id: u64,
    pub attempt: Attempt,
    pub completed_at: u64,
    pub rank: u32,
    pub cache_before: u32,
    pub cache_after: u32,
    pub food_before: u32,
    pub food_after: u32,
    pub hunger: i32,
    pub outcome: Outcome,
}
/// Atomic scalar operation; the resolver checks identity/availability separately.
pub fn grant(stock: u32, food: u32) -> (u32, u32, Outcome) {
    if stock == 0 {
        (stock, food, Outcome::Depleted)
    } else if let Some(next) = food.checked_add(1) {
        (stock - 1, next, Outcome::Granted)
    } else {
        (stock, food, Outcome::Overflow)
    }
}
pub fn rank(actor: u32, wave: u64, order: e::Order) -> u32 {
    match order {
        e::Order::Ascending => actor,
        e::Order::Descending => 2 - actor,
        e::Order::Rotating => (actor + 3 - (wave % 3) as u32) % 3,
    }
}
#[derive(Resource)]
pub(crate) struct Environment {
    pub cache: Cache,
    pub duration: u64,
    pub policy: Policy,
    pub choose: fn(&Input, Policy) -> Decision,
    pub pending: Vec<Attempt>,
    pub records: Vec<Record>,
    pub next_id: u64,
    pub initial_total: u64,
    pub consumed_before: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub state: e::State,
    pub scenes: usize,
    pub cache: Cache,
    pub pending: Vec<Attempt>,
    pub acquisitions: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    pub need_before: usize,
    pub activity: o::Step,
    pub need_after: usize,
    pub choices: Vec<Option<Choice>>,
    pub admissions: Vec<Attempt>,
    pub completed: Vec<u64>,
    pub skipped_responses: Vec<u32>,
    pub provisional: Vec<e::Allocation>,
    pub atomic_blocked: Vec<e::Invitation>,
}
/// The legacy input lives in activity.decisions[owner], or acquiring_base when
/// Acquire suppresses a legacy activity. It is never copied a second time.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Choice {
    pub cache: Observation,
    pub inquiry_lane_free: bool,
    pub policy: Policy,
    pub candidates: Vec<Candidate>,
    pub selected: Action,
    pub acquiring_base: Option<o::Decision>,
}
impl From<Decision> for Choice {
    fn from(d: Decision) -> Self {
        Self {
            cache: d.input.cache,
            inquiry_lane_free: d.input.inquiry_lane_free,
            policy: d.policy,
            candidates: d.candidates,
            selected: d.selected,
            acquiring_base: (d.selected == Action::Acquire).then_some(d.base),
        }
    }
}
