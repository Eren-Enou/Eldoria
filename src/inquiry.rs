//! Local expected useful information, distinct from unfinished intent and trust.
use crate::{
    concerns::Concern,
    foresight::Sensor,
    intentional::{Profile, TalkAction, TalkDecision},
    model::AgentId,
};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};
pub const RULES: &str = "experiment-006-v1";
pub const CELL_CAPACITY: usize = 32;
pub const SIGNATURE_CAPACITY: usize = 32;
pub const EPISODE_CAPACITY: usize = 16;
pub const CONTACT_CAPACITY: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Strategy {
    Direct,
    Evidence,
}
impl Strategy {
    pub fn prior(self) -> i32 {
        match self {
            Self::Direct => 55,
            Self::Evidence => 65,
        }
    }
    pub fn cost(self) -> u32 {
        match self {
            Self::Direct => 10,
            Self::Evidence => 28,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub use_history: bool,
    pub track_novelty: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            use_history: true,
            track_novelty: true,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cell {
    pub concern: u64,
    pub source: AgentId,
    pub strategy: Strategy,
    pub attempts: u32,
    pub expected: i32,
    pub last_record: u64,
    pub attempted_offer: Option<(u8, i32)>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Origin {
    Claim(AgentId),
    Evidence { subject: AgentId, channel: u8 },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature {
    pub event: u64,
    pub origin: Origin,
    pub claim: bool,
    pub quality: i32,
    pub receipt: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Novelty {
    New,
    Stronger,
    Conflict,
    Redundant,
    None,
}
/// Compare provenance/content, never truth. Conflicting evidence can be informative
/// without resolving a belief. Same source repetition is not independent evidence.
pub fn novelty(seen: &[Signature], incoming: Option<&Signature>) -> (Novelty, i32) {
    let Some(info) = incoming else {
        return (Novelty::None, 0);
    };
    if info.quality == 0 {
        return (Novelty::None, 0);
    }
    let prior: Vec<_> = seen.iter().filter(|s| s.event == info.event).collect();
    if prior
        .iter()
        .any(|s| s.origin == info.origin && s.claim == info.claim && s.quality >= info.quality)
    {
        return (Novelty::Redundant, 0);
    }
    if prior.iter().any(|s| s.claim != info.claim) {
        return (Novelty::Conflict, info.quality.min(30));
    }
    let strongest = prior
        .iter()
        .filter(|s| s.claim == info.claim)
        .map(|s| s.quality)
        .max();
    if strongest.is_some_and(|q| info.quality > q) {
        return (
            Novelty::Stronger,
            (info.quality - strongest.unwrap() + 20).min(90),
        );
    }
    (
        Novelty::New,
        if strongest.is_some() {
            info.quality.min(20)
        } else {
            info.quality.min(40)
        },
    )
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Offer {
    pub source: AgentId,
    pub event: u64,
    pub channel: u8,
    pub quality: i32,
    pub effort: u32,
    pub notice: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    pub id: AgentId,
    pub credibility: i32,
    pub offers: Vec<Offer>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub owner: AgentId,
    pub scene: u64,
    pub hunger: i32,
    pub relationship_goal: i32,
    pub concerns: Vec<Concern>,
    pub sources: Vec<Source>,
    pub cells: Vec<Cell>,
    pub seen: Vec<Signature>,
    pub settings: Settings,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Pause,
    Ask {
        concern: u64,
        source: AgentId,
        strategy: Strategy,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Candidate {
    pub action: Action,
    pub importance: i32,
    pub attempts: u32,
    pub prior_or_learned: i32,
    pub opportunity_value: i32,
    pub expected_gain: i32,
    pub benefit: i32,
    pub cost: u32,
    pub score: i32,
    pub history_record: Option<u64>,
    pub notice: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub input: Input,
    pub candidates: Vec<Candidate>,
    pub selected: Action,
}
pub fn legal(input: &Input, action: Action) -> bool {
    match action {
        Action::Pause => true,
        Action::Ask {
            concern, source, ..
        } => {
            input
                .sources
                .iter()
                .any(|s| s.id == source && source != input.owner)
                && input.concerns.iter().any(|c| {
                    c.id == concern
                        && c.owner == input.owner
                        && c.status.active()
                        && c.created_scene < input.scene
                })
        }
    }
}
pub fn policy(input: &Input) -> Decision {
    let mut candidates = vec![Candidate {
        action: Action::Pause,
        importance: 0,
        attempts: 0,
        prior_or_learned: 0,
        opportunity_value: 0,
        expected_gain: 0,
        benefit: 0,
        cost: 0,
        score: 0,
        history_record: None,
        notice: None,
    }];
    for concern in &input.concerns {
        for source in &input.sources {
            for strategy in [Strategy::Direct, Strategy::Evidence] {
                let action = Action::Ask {
                    concern: concern.id,
                    source: source.id,
                    strategy,
                };
                if !legal(input, action) {
                    continue;
                }
                let cell = input.cells.iter().find(|c| {
                    c.concern == concern.id && c.source == source.id && c.strategy == strategy
                });
                let base = if input.settings.use_history {
                    cell.map_or(strategy.prior(), |c| c.expected)
                } else {
                    strategy.prior()
                };
                let offer = source
                    .offers
                    .iter()
                    .rev()
                    .find(|o| o.event == concern.event);
                let opportunity =
                    offer
                        .filter(|_| strategy == Strategy::Evidence)
                        .map_or(0, |offer| {
                            let origin = Origin::Evidence {
                                subject: concern.target,
                                channel: offer.channel,
                            };
                            let already_seen = input.seen.iter().any(|s| {
                                s.event == concern.event
                                    && s.origin == origin
                                    && s.quality >= offer.quality
                            });
                            if already_seen
                                || cell.is_some_and(|c| {
                                    c.attempted_offer == Some((offer.channel, offer.quality))
                                })
                            {
                                0
                            } else {
                                let strongest = input
                                    .seen
                                    .iter()
                                    .filter(|s| s.event == concern.event)
                                    .map(|s| s.quality)
                                    .max()
                                    .unwrap_or(0);
                                (offer.quality - strongest + 30).clamp(0, 90)
                            }
                        });
                let expected = (base.max(opportunity) + source.credibility / 10).clamp(0, 100);
                let benefit = concern.importance * expected / 100;
                let cost = strategy.cost()
                    + input.hunger.max(0) as u32 / 10
                    + if strategy == Strategy::Evidence {
                        offer.map_or(0, |o| o.effort)
                    } else {
                        0
                    };
                candidates.push(Candidate {
                    action,
                    importance: concern.importance,
                    attempts: cell.map_or(0, |c| c.attempts),
                    prior_or_learned: base,
                    opportunity_value: opportunity,
                    expected_gain: expected,
                    benefit,
                    cost,
                    score: benefit - cost as i32,
                    history_record: cell
                        .filter(|_| input.settings.use_history)
                        .map(|c| c.last_record),
                    notice: offer.map(|o| o.notice),
                });
            }
        }
    }
    let selected = candidates
        .iter()
        .fold(
            &candidates[0],
            |best, c| if c.score > best.score { c } else { best },
        )
        .action;
    Decision {
        input: input.clone(),
        candidates,
        selected,
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    pub decision: TalkDecision,
    pub valid: bool,
    pub public_action: TalkAction,
    pub signatures: Vec<Signature>,
    pub receipts: Vec<u64>,
    pub time_spent: u32,
    pub credibility_before: i32,
    pub credibility_after: i32,
    pub verified_claim: Option<bool>,
    pub failure: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub id: u64,
    pub tick: u64,
    pub meeting: Option<u64>,
    pub decision: Decision,
    pub valid: bool,
    pub response: Option<Response>,
    pub novelty: Novelty,
    pub realized_value: i32,
    pub information_values: Vec<(u64, Novelty, i32)>,
    pub cell_before: Option<Cell>,
    pub cell_after: Option<Cell>,
    pub concern_before: Option<Concern>,
    pub concern_after: Option<Concern>,
    pub time_spent: u32,
    pub food_before: Vec<(AgentId, u32)>,
    pub food_after: Vec<(AgentId, u32)>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Episode {
    pub record: u64,
    pub concern: u64,
    pub source: AgentId,
    pub strategy: Strategy,
    pub novelty: Novelty,
    pub value: i32,
    pub receipt: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Notice {
    pub id: u64,
    pub tick: u64,
    pub owner: AgentId,
    pub source: AgentId,
    pub event: u64,
    pub profile: Profile,
    pub hunger: i32,
    pub can_observe_self: bool,
    pub score: i32,
    pub offered: bool,
    pub offer: Offer,
    pub time_spent: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Capability {
    pub event: u64,
    pub channel: u8,
    pub sensor: Sensor,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Intervention {
    pub source: AgentId,
    pub before: Option<Capability>,
    pub after: Capability,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Eviction {
    pub owner: AgentId,
    pub collection: String,
    pub reference: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meeting {
    pub id: u64,
    pub participants: Vec<AgentId>,
    pub tick: u64,
}
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inquiry {
    pub cells: BTreeMap<AgentId, VecDeque<Cell>>,
    pub seen: BTreeMap<AgentId, VecDeque<Signature>>,
    pub episodes: BTreeMap<AgentId, VecDeque<Episode>>,
    pub contacts: BTreeMap<AgentId, VecDeque<AgentId>>,
    pub offers: BTreeMap<AgentId, VecDeque<Offer>>,
    pub capabilities: BTreeMap<AgentId, VecDeque<Capability>>,
    pub settings: BTreeMap<AgentId, Settings>,
    pub records: Vec<Record>,
    pub notices: Vec<Notice>,
    pub meetings: Vec<Meeting>,
    pub interventions: Vec<Intervention>,
    pub evictions: Vec<Eviction>,
}
impl Inquiry {
    pub(crate) fn contact(&mut self, owner: AgentId, source: AgentId) {
        let list = self.contacts.entry(owner).or_default();
        if list.contains(&source) {
            return;
        }
        if list.len() == CONTACT_CAPACITY {
            self.evictions.push(Eviction {
                owner,
                collection: "contacts".into(),
                reference: u64::from(list.pop_front().unwrap()),
            });
        }
        list.push_back(source);
    }
    pub(crate) fn observe(&mut self, owner: AgentId, incoming: Signature) {
        let list = self.seen.entry(owner).or_default();
        if list.iter().any(|s| {
            s.event == incoming.event
                && s.origin == incoming.origin
                && s.claim == incoming.claim
                && s.quality >= incoming.quality
        }) {
            return;
        }
        list.retain(|s| {
            !(s.event == incoming.event && s.origin == incoming.origin && s.claim == incoming.claim)
        });
        if list.len() == SIGNATURE_CAPACITY {
            self.evictions.push(Eviction {
                owner,
                collection: "signatures".into(),
                reference: list.pop_front().unwrap().receipt,
            });
        }
        list.push_back(incoming);
    }
    pub(crate) fn learn(
        &mut self,
        owner: AgentId,
        key: (u64, AgentId, Strategy),
        value: i32,
        record: u64,
        attempted_offer: Option<(u8, i32)>,
    ) -> (Option<Cell>, Cell) {
        let (concern, source, strategy) = key;
        let cells = self.cells.entry(owner).or_default();
        let index = cells
            .iter()
            .position(|c| c.concern == concern && c.source == source && c.strategy == strategy);
        let before = index.and_then(|i| cells.remove(i));
        let after = Cell {
            concern,
            source,
            strategy,
            attempts: before.as_ref().map_or(1, |c| c.attempts.saturating_add(1)),
            expected: (before.as_ref().map_or(strategy.prior(), |c| c.expected) + value) / 2,
            last_record: record,
            attempted_offer: attempted_offer
                .or_else(|| before.as_ref().and_then(|c| c.attempted_offer)),
        };
        if cells.len() == CELL_CAPACITY {
            self.evictions.push(Eviction {
                owner,
                collection: "cells".into(),
                reference: cells.pop_front().unwrap().last_record,
            });
        }
        cells.push_back(after.clone());
        (before, after)
    }
}
#[derive(Resource)]
pub(crate) struct InquiryPolicy(pub fn(&Input) -> Decision);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_source_strategy_and_information_storage_audits_eviction() {
        let mut state = Inquiry::default();
        for id in 0..40 {
            state.learn(0, (id, 1, Strategy::Direct), 0, id, None);
            state.observe(
                0,
                Signature {
                    event: id,
                    origin: Origin::Claim(1),
                    claim: true,
                    quality: 40,
                    receipt: id,
                },
            );
        }
        assert_eq!(state.cells[&0].len(), CELL_CAPACITY);
        assert_eq!(state.seen[&0].len(), SIGNATURE_CAPACITY);
        assert_eq!(
            state
                .evictions
                .iter()
                .filter(|e| e.collection == "cells")
                .count(),
            8
        );
        assert_eq!(
            state
                .evictions
                .iter()
                .filter(|e| e.collection == "signatures")
                .count(),
            8
        );
        assert_eq!(state.cells[&0].front().unwrap().concern, 8);
        // Refreshing one bounded signature replaces it without new capacity credit.
        state.observe(
            0,
            Signature {
                event: 39,
                origin: Origin::Claim(1),
                claim: true,
                quality: 80,
                receipt: 100,
            },
        );
        assert_eq!(state.seen[&0].len(), SIGNATURE_CAPACITY);
        assert_eq!(state.seen[&0].back().unwrap().quality, 80);
    }
}
