//! Bounded local lineage knowledge, separate from observer-only causal history.
use crate::{
    inquiry as q,
    intentional::Profile,
    model::{Agent, AgentId},
};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};
pub const RULES: &str = "experiment-007-v1";
pub const CAPACITY: usize = 32;
pub const MAX_DEPTH: u8 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Lineage {
    Observation(u64),
    Unattributed(AgentId),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub provenance: bool,
    pub speakers: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            provenance: true,
            speakers: true,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Knowledge {
    pub receipt: u64,
    pub event: u64,
    pub communicator: AgentId,
    /// Public message handle permits later attribution to replace the same report.
    pub message: u64,
    pub claim: bool,
    pub quality: i32,
    pub known: Option<u64>,
}
pub fn lineage(k: &Knowledge, settings: Settings) -> Lineage {
    if settings.provenance
        && let Some(root) = k.known
    {
        return Lineage::Observation(root);
    }
    Lineage::Unattributed(if settings.speakers { k.communicator } else { 0 })
}
pub fn support(items: &[Knowledge], event: u64, settings: Settings) -> i32 {
    let mut groups = BTreeMap::<Lineage, (i32, i32)>::new();
    for k in items.iter().filter(|k| k.event == event) {
        let pair = groups.entry(lineage(k, settings)).or_default();
        if k.claim {
            pair.0 = pair.0.max(k.quality);
        } else {
            pair.1 = pair.1.max(k.quality);
        }
    }
    let (mut positive, mut negative) = (0, 0);
    for (p, n) in groups.values() {
        positive += (100 - positive) * p / 100;
        negative += (100 - negative) * n / 100;
    }
    positive - negative
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueKind {
    First,
    Corroboration,
    Shared,
    Conflict,
    Revelation,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evaluation {
    pub new_speaker: bool,
    pub lineage: Lineage,
    pub independent: bool,
    pub kind: ValueKind,
    pub incremental_value: i32,
    pub before: i32,
    pub after: i32,
}
pub fn evaluate(items: &[Knowledge], incoming: &Knowledge, settings: Settings) -> Evaluation {
    let prior: Vec<_> = items.iter().filter(|k| k.event == incoming.event).collect();
    let key = lineage(incoming, settings);
    let shared = prior.iter().any(|k| lineage(k, settings) == key);
    let revelation = settings.provenance
        && incoming.known.is_some()
        && prior.iter().any(|k| {
            k.communicator == incoming.communicator
                && k.message == incoming.message
                && k.known.is_none()
        });
    let conflict = prior
        .iter()
        .any(|k| lineage(k, settings) != key && k.claim != incoming.claim);
    let before = support(items, incoming.event, settings);
    let mut after_items = items.to_vec();
    // Replace local assumptions only when a real message identifies the old report.
    if revelation {
        for k in &mut after_items {
            if k.event == incoming.event
                && k.communicator == incoming.communicator
                && k.message == incoming.message
            {
                k.known = incoming.known;
            }
        }
    }
    if after_items.len() == CAPACITY {
        after_items.remove(0);
    }
    after_items.push(incoming.clone());
    let after = support(&after_items, incoming.event, settings);
    let kind = if revelation {
        ValueKind::Revelation
    } else if shared {
        ValueKind::Shared
    } else if conflict {
        ValueKind::Conflict
    } else if prior.is_empty() {
        ValueKind::First
    } else {
        ValueKind::Corroboration
    };
    let incremental_value = match kind {
        ValueKind::Revelation => (after - before).abs().min(40),
        ValueKind::Shared => (after - before).abs().min(10),
        ValueKind::Conflict => incoming.quality.min(30),
        ValueKind::First => incoming.quality.min(40),
        ValueKind::Corroboration => incoming.quality.min(30),
    };
    Evaluation {
        new_speaker: !prior
            .iter()
            .any(|k| k.communicator == incoming.communicator),
        lineage: key,
        independent: !shared && !revelation,
        kind,
        incremental_value,
        before,
        after,
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Root {
    pub id: u64,
    pub event: u64,
    pub observer: AgentId,
    pub claim: bool,
    pub quality: i32,
    pub tick: u64,
    pub sensor: crate::foresight::Sensor,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt {
    pub id: u64,
    pub tick: u64,
    pub event: u64,
    pub communicator: AgentId,
    pub listener: AgentId,
    /// These are observer-only; they are never included in Input/ExchangeInput.
    pub actual_root: u64,
    pub parent: Option<u64>,
    pub depth: u8,
    pub attributed: bool,
    pub knowledge: Knowledge,
    pub evaluation: Evaluation,
    pub cognition_receipt: Option<u64>,
    pub food_before: Vec<(AgentId, u32)>,
    pub food_after: Vec<(AgentId, u32)>,
    pub time_spent: u32,
    pub credibility_changes: Vec<(AgentId, i32, i32)>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hint {
    pub source: AgentId,
    pub event: u64,
    pub known: Option<u64>,
    pub quality: i32,
    pub receipt: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub base: q::Input,
    pub knowledge: Vec<Knowledge>,
    pub hints: Vec<Hint>,
    pub settings: Settings,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Factor {
    pub action: q::Action,
    pub credibility: i32,
    pub known: Option<u64>,
    pub independence: i32,
    pub expected: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub input: Input,
    pub decision: q::Decision,
    pub factors: Vec<Factor>,
}
pub fn policy(input: &Input) -> Decision {
    let mut decision = q::policy(&input.base);
    let mut factors = Vec::new();
    for candidate in &mut decision.candidates {
        let q::Action::Ask {
            concern, source, ..
        } = candidate.action
        else {
            continue;
        };
        let c = input
            .base
            .concerns
            .iter()
            .find(|c| c.id == concern)
            .unwrap();
        let hint = input
            .hints
            .iter()
            .rev()
            .find(|h| h.source == source && h.event == c.event);
        let known = hint.and_then(|h| h.known);
        let overlapping = input.settings.provenance
            && known.is_some_and(|root| {
                input
                    .knowledge
                    .iter()
                    .any(|k| k.event == c.event && k.known == Some(root))
            });
        let independence = if overlapping { 0 } else { 100 };
        let credibility = input
            .base
            .sources
            .iter()
            .find(|s| s.id == source)
            .unwrap()
            .credibility;
        candidate.expected_gain = candidate.expected_gain * independence / 100;
        candidate.benefit = candidate.importance * candidate.expected_gain / 100;
        candidate.score = candidate.benefit - candidate.cost as i32;
        factors.push(Factor {
            action: candidate.action,
            credibility,
            known,
            independence,
            expected: candidate.expected_gain,
        });
    }
    decision.selected = decision
        .candidates
        .iter()
        .fold(&decision.candidates[0], |best, c| {
            if c.score > best.score { c } else { best }
        })
        .action;
    Decision {
        input: input.clone(),
        decision,
        factors,
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExchangeInput {
    pub own: Agent,
    pub profile: Profile,
    pub partner: AgentId,
    pub event: u64,
    pub acquired: Option<Knowledge>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExchangeAction {
    Silence,
    Share,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExchangeDecision {
    pub input: ExchangeInput,
    pub score: i32,
    pub selected: ExchangeAction,
}
pub fn exchange(input: &ExchangeInput) -> ExchangeDecision {
    let score = input.profile.relationship_goal
        - input.profile.privacy
        - input.own.hunger / 10
        - input.profile.request_cost / 4;
    let legal = input.acquired.is_some();
    ExchangeDecision {
        input: input.clone(),
        score,
        selected: if legal && score > 0 {
            ExchangeAction::Share
        } else {
            ExchangeAction::Silence
        },
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectionInput {
    pub own: Agent,
    pub profile: Profile,
    pub event: u64,
    pub effort: u32,
    pub quality: i32,
    pub opening: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectionDecision {
    pub input: InspectionInput,
    pub score: i32,
    pub selected: bool,
}
pub fn inspection(input: &InspectionInput) -> InspectionDecision {
    let score = input.profile.relationship_goal
        - input.profile.privacy
        - input.own.hunger / 10
        - input.effort as i32;
    InspectionDecision {
        input: input.clone(),
        score,
        selected: score > 0,
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Window {
    pub id: u64,
    pub event: u64,
    pub tick: u64,
    pub decisions: Vec<InspectionDecision>,
    pub roots: Vec<u64>,
    pub time_spent: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExchangeRecord {
    pub id: u64,
    pub tick: u64,
    pub decision: ExchangeDecision,
    pub attribution_available: bool,
    pub valid: bool,
    pub receipt: Option<u64>,
    pub time_spent: u32,
    pub metadata_only: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Query {
    pub id: u64,
    pub tick: u64,
    pub decision: Decision,
    pub valid: bool,
    pub receipt: Option<u64>,
    pub value: i32,
    pub before: Option<q::Cell>,
    pub after: Option<q::Cell>,
    pub time_spent: u32,
    pub inquiry_record: u64,
    pub legacy_response: Option<q::Response>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Eviction {
    pub owner: AgentId,
    pub collection: String,
    pub receipt: u64,
}
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provenance {
    pub knowledge: BTreeMap<AgentId, VecDeque<Knowledge>>,
    pub hints: BTreeMap<AgentId, VecDeque<Hint>>,
    pub settings: BTreeMap<AgentId, Settings>,
    pub roots: Vec<Root>,
    pub receipts: Vec<Receipt>,
    pub windows: Vec<Window>,
    pub exchanges: Vec<ExchangeRecord>,
    pub queries: Vec<Query>,
    pub evictions: Vec<Eviction>,
    pub comparisons: BTreeMap<AgentId, VecDeque<Comparison>>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comparison {
    pub event: u64,
    pub source: AgentId,
    pub claim: bool,
    pub evidence: u64,
    pub receipt: u64,
}
impl Provenance {
    pub(crate) fn retain(&mut self, owner: AgentId, incoming: Knowledge) {
        let items = self.knowledge.entry(owner).or_default();
        if incoming.known.is_some() {
            for k in items.iter_mut().filter(|k| {
                k.event == incoming.event
                    && k.communicator == incoming.communicator
                    && k.message == incoming.message
            }) {
                k.known = incoming.known;
            }
        }
        if items.len() == CAPACITY {
            self.evictions.push(Eviction {
                owner,
                collection: "knowledge".into(),
                receipt: items.pop_front().unwrap().receipt,
            });
        }
        items.push_back(incoming);
    }
    pub(crate) fn hint(&mut self, owner: AgentId, hint: Hint) {
        let list = self.hints.entry(owner).or_default();
        list.retain(|h| !(h.source == hint.source && h.event == hint.event));
        if list.len() == CAPACITY {
            self.evictions.push(Eviction {
                owner,
                collection: "hints".into(),
                receipt: list.pop_front().unwrap().receipt,
            });
        }
        list.push_back(hint);
    }
}
#[derive(Resource)]
pub(crate) struct Policy(pub fn(&Input) -> Decision);
#[derive(Resource)]
pub(crate) struct ExchangePolicy(pub fn(&ExchangeInput) -> ExchangeDecision);
#[derive(Resource)]
pub(crate) struct InspectionPolicy(pub fn(&InspectionInput) -> InspectionDecision);
