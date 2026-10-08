//! Experiment 009: local retention ordering, never observer archive recovery.
use crate::{
    assessment::{self as a, Item, Origin, Receipt},
    concerns::Concern,
    model::AgentId,
};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};

pub const RULES: &str = "experiment-009-v1";
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Method {
    Fifo,
    Quality,
    Salient,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub owner: AgentId,
    /// Existing retained candidates in arrival order, followed by incoming.
    pub items: Vec<Item>,
    pub concerns: Vec<Concern>,
    pub method: Method,
}
impl Input {
    pub(crate) fn new(
        owner: AgentId,
        retained: &[Item],
        incoming: &Item,
        concerns: Vec<Concern>,
        method: Method,
    ) -> Self {
        let mut items = retained.to_vec();
        if matches!(incoming.origin, Origin::Known(_)) && incoming.message.is_some() {
            for item in &mut items {
                if item.event == incoming.event
                    && item.communicator == incoming.communicator
                    && item.message == incoming.message
                    && matches!(item.origin, Origin::Unknown(_))
                {
                    item.origin = incoming.origin;
                }
            }
        }
        items.push(incoming.clone());
        Self {
            owner,
            items,
            concerns,
            method,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Candidate {
    pub receipt: Receipt,
    pub representative: bool,
    pub importance: i32,
    pub score: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub input: Input,
    pub candidates: Vec<Candidate>,
    pub evict: Option<usize>,
}
pub fn policy(input: &Input) -> Decision {
    let candidates: Vec<_> = input
        .items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let representative = !input.items.iter().enumerate().any(|(other, next)| {
                next.event == item.event
                    && next.origin == item.origin
                    && next.claim == item.claim
                    && (next.quality > item.quality
                        || (next.quality == item.quality && other > index))
            });
            let importance = input
                .concerns
                .iter()
                .filter(|c| c.owner == input.owner && c.event == item.event && c.status.active())
                .map(|c| c.importance)
                .max()
                .unwrap_or(0);
            let score = match input.method {
                Method::Fifo => 0,
                _ if !representative => 0,
                Method::Quality => item.quality,
                Method::Salient => item.quality + importance,
            };
            Candidate {
                receipt: item.receipt,
                representative,
                importance,
                score,
            }
        })
        .collect();
    let evict = (input.items.len() > a::CAPACITY).then(|| {
        candidates
            .iter()
            .enumerate()
            .min_by_key(|(index, c)| (c.score, *index))
            .unwrap()
            .0
    });
    Decision {
        input: input.clone(),
        candidates,
        evict,
    }
}
pub(crate) fn fifo_fallback(input: &Input) -> Decision {
    let mut fifo = input.clone();
    fifo.method = Method::Fifo;
    let mut decision = policy(&fifo);
    decision.input = input.clone();
    decision
}
impl Decision {
    pub fn validate(&self, actual: &Input) -> Result<(), String> {
        if &self.input != actual
            || actual.items.is_empty()
            || actual.items.len() > a::CAPACITY + 1
            || actual.concerns.len() > crate::concerns::CAPACITY
            || actual.concerns.iter().any(|c| c.owner != actual.owner)
            || self.candidates.len() != actual.items.len()
            || self.candidates.iter().zip(&actual.items).any(|(c, i)| {
                c.receipt != i.receipt
                    || !(0..=200).contains(&c.score)
                    || !(0..=100).contains(&c.importance)
            })
        {
            return Err("invalid local retention input/candidates".into());
        }
        let expected = (actual.items.len() > a::CAPACITY).then(|| {
            self.candidates
                .iter()
                .enumerate()
                .min_by_key(|(index, c)| (c.score, *index))
                .unwrap()
                .0
        });
        if self.evict != expected {
            return Err("invalid retention eviction/order".into());
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub assessment: u64,
    /// Invalid replacement output uses an audited FIFO fallback inside paid protocols.
    pub fallback_reason: Option<String>,
    pub decision: Decision,
    pub event_support_before: Vec<(u64, i32)>,
    pub event_support_after: Vec<(u64, i32)>,
}
#[derive(Resource, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Retention {
    pub method: Method,
    /// First assessment receipt governed by this prospective mode.
    pub first_assessment: u64,
    pub records: Vec<Record>,
}
impl Retention {
    /// Reconstruct historical choices from captured inputs, never current policy.
    pub fn validate(&self, assessment: &a::Assessment) -> Result<(), String> {
        if assessment.method != a::Method::Grouped {
            return Err("009 assessment must be Grouped".into());
        }
        let mut rebuilt = a::Assessment::new(assessment.method);
        if self.first_assessment > assessment.records.len() as u64 {
            return Err("retention start exceeds history".into());
        }
        let mut choices = self.records.iter().peekable();
        for record in &assessment.records {
            let choice = if record.id >= self.first_assessment {
                Some(
                    choices
                        .next()
                        .filter(|r| r.assessment == record.id)
                        .ok_or("missing retention receipt")?,
                )
            } else {
                None
            };
            let eviction = if let Some(choice) = choice {
                let retained: Vec<_> = rebuilt
                    .items
                    .get(&record.owner)
                    .map(|v| v.iter().cloned().collect())
                    .unwrap_or_default();
                let input = Input::new(
                    record.owner,
                    &retained,
                    &record.incoming,
                    choice.decision.input.concerns.clone(),
                    self.method,
                );
                choice.decision.validate(&input)?;
                if let Some(reason) = &choice.fallback_reason
                    && (![
                        "invalid local retention input/candidates",
                        "invalid retention eviction/order",
                    ]
                    .contains(&reason.as_str())
                        || choice.decision != fifo_fallback(&input))
                {
                    return Err("invalid retention fallback".into());
                }
                if choice.event_support_before != supports(&retained) {
                    return Err("retention prior basis mismatch".into());
                }
                choice.decision.evict
            } else {
                if choices.peek().is_some_and(|r| r.assessment < record.id) {
                    return Err("retention record order mismatch".into());
                }
                Some(0)
            };
            rebuilt.receive_selected(
                record.owner,
                record.incoming.clone(),
                record.prior_belief,
                a::grouped,
                eviction,
            );
            if rebuilt.records.last() != Some(record) {
                return Err("009 assessment history does not replay".into());
            }
            if let Some(choice) = choice {
                let retained: Vec<_> = rebuilt.items[&record.owner].iter().cloned().collect();
                if choice.event_support_after != supports(&retained) {
                    return Err("retention resulting basis mismatch".into());
                }
            }
        }
        if choices.next().is_some() || rebuilt != *assessment {
            return Err("retention live basis/history mismatch".into());
        }
        Ok(())
    }
}
#[derive(Resource)]
pub(crate) struct Policy(pub fn(&Input) -> Decision);

pub(crate) fn supports(items: &[Item]) -> Vec<(u64, i32)> {
    let events: std::collections::BTreeSet<_> = items.iter().map(|i| i.event).collect();
    events
        .into_iter()
        .map(|event| (event, a::grouped(items, event)))
        .collect()
}
