//! Referential observer audit. This module never supplies inputs to policies.
use crate::{inquiry as q, model::AgentId, provenance as p};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
};

macro_rules! id {
    ($name:ident) => {
        #[derive(
            Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(pub u64);
    };
}
id!(InquiryId);
id!(QueryId);
id!(ContextId);
id!(ReceiptId);

/// Immutable, first-occurrence ordered contexts. Hashes only accelerate exact
/// equality checks; collisions cannot merge unequal contexts. Hashes are not IDs,
/// are not serialized, and cannot affect policy execution or output order.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContextStore<T> {
    values: Vec<T>,
    #[serde(skip)]
    lookup: BTreeMap<u64, Vec<usize>>,
}
impl<T> Default for ContextStore<T> {
    fn default() -> Self {
        Self {
            values: vec![],
            lookup: BTreeMap::new(),
        }
    }
}
impl<T: PartialEq> PartialEq for ContextStore<T> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}
impl<T: Eq> Eq for ContextStore<T> {}
impl<T: Serialize + Eq> ContextStore<T> {
    fn hash(value: &T) -> u64 {
        let mut h = std::collections::hash_map::DefaultHasher::new();
        serde_json::to_vec(value)
            .expect("serializable audit context")
            .hash(&mut h);
        h.finish()
    }
    pub fn intern(&mut self, value: T) -> ContextId {
        if self.lookup.is_empty() && !self.values.is_empty() {
            for (i, v) in self.values.iter().enumerate() {
                self.lookup.entry(Self::hash(v)).or_default().push(i);
            }
        }
        let hash = Self::hash(&value);
        let bucket = self.lookup.entry(hash).or_default();
        if let Some(&id) = bucket.iter().find(|&&i| self.values[i] == value) {
            return ContextId(id as u64);
        }
        let id = self.values.len();
        self.values.push(value);
        bucket.push(id);
        ContextId(id as u64)
    }
}
impl<T> ContextStore<T> {
    pub fn get(&self, id: ContextId) -> Result<&T, String> {
        usize::try_from(id.0)
            .ok()
            .and_then(|i| self.values.get(i))
            .ok_or_else(|| format!("dangling context {}", id.0))
    }
    pub fn values(&self) -> &[T] {
        &self.values
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalContext {
    pub knowledge: Vec<p::Knowledge>,
    pub hints: Vec<p::Hint>,
    pub settings: p::Settings,
}
#[derive(Resource, Clone, Debug, Default)]
pub(crate) struct Contexts(pub ContextStore<LocalContext>);

/// Extension of an existing native inquiry, not a second copy of that inquiry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryRef {
    pub id: QueryId,
    pub inquiry: InquiryId,
    pub owner: AgentId,
    pub observed_at: u64,
    pub completed_at: u64,
    pub context: ContextId,
    pub factors: Vec<p::Factor>,
    pub receipt: Option<ReceiptId>,
    /// Invalid replacement policies may return mismatched inputs. Retain their
    /// attempted output exactly rather than normalizing away the failure evidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempted_base: Option<q::Input>,
}
pub(crate) type RuntimeProvenance = p::ProvenanceArchive<QueryRef>;

impl QueryRef {
    pub fn reconstruct(
        &self,
        inquiry: &q::Inquiry,
        contexts: &ContextStore<LocalContext>,
    ) -> Result<p::Query, String> {
        let record = usize::try_from(self.inquiry.0)
            .ok()
            .and_then(|i| inquiry.records.get(i))
            .ok_or_else(|| format!("query {}: dangling inquiry {}", self.id.0, self.inquiry.0))?;
        if record.id != self.inquiry.0
            || record.tick != self.completed_at
            || record.tick.checked_sub(u64::from(record.time_spent)) != Some(self.observed_at)
            || record.food_before.first().map(|p| p.0) != Some(self.owner)
        {
            return Err(format!(
                "query {}: inquiry ownership/time mismatch",
                self.id.0
            ));
        }
        if record.valid
            && (record.decision.input.owner != self.owner || self.attempted_base.is_some())
        {
            return Err(format!("query {}: inconsistent valid input", self.id.0));
        }
        let local = contexts.get(self.context)?;
        Ok(p::Query {
            id: self.id.0,
            tick: record.tick,
            decision: p::Decision {
                input: p::Input {
                    base: self
                        .attempted_base
                        .as_ref()
                        .unwrap_or(&record.decision.input)
                        .clone(),
                    knowledge: local.knowledge.clone(),
                    hints: local.hints.clone(),
                    settings: local.settings,
                },
                decision: record.decision.clone(),
                factors: self.factors.clone(),
            },
            valid: record.valid,
            receipt: self.receipt.map(|r| r.0),
            value: record.realized_value,
            before: record.cell_before.clone(),
            after: record.cell_after.clone(),
            time_spent: record.time_spent,
            inquiry_record: record.id,
            legacy_response: record.response.clone(),
        })
    }
}

/// New versioned export; legacy experiment snapshot schemas remain unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompactSnapshot {
    pub format: String,
    pub base: crate::experiment006::Snapshot,
    pub provenance: p::ProvenanceArchive<QueryRef>,
    pub contexts: ContextStore<LocalContext>,
}
impl CompactSnapshot {
    pub const FORMAT: &str = "causal-audit-v1";
    /// Checked migration of a legacy observer snapshot. No simulation is resumed
    /// and no individual memory is reconstructed from this observer archive.
    pub fn from_legacy(legacy: &crate::experiment007::Snapshot) -> Result<Self, String> {
        let mut contexts = ContextStore::default();
        let mut queries = Vec::new();
        for query in &legacy.provenance.queries {
            let record = legacy
                .base
                .inquiry
                .records
                .get(query.inquiry_record as usize)
                .ok_or("missing linked inquiry")?;
            let context = contexts.intern(LocalContext {
                knowledge: query.decision.input.knowledge.clone(),
                hints: query.decision.input.hints.clone(),
                settings: query.decision.input.settings,
            });
            let entry = QueryRef {
                id: QueryId(query.id),
                inquiry: InquiryId(query.inquiry_record),
                owner: record.food_before.first().ok_or("missing inquiry owner")?.0,
                observed_at: query
                    .tick
                    .checked_sub(u64::from(query.time_spent))
                    .ok_or("invalid query duration")?,
                completed_at: query.tick,
                context,
                factors: query.decision.factors.clone(),
                receipt: query.receipt.map(ReceiptId),
                attempted_base: (query.decision.input.base != query.decision.decision.input)
                    .then(|| query.decision.input.base.clone()),
            };
            if entry.reconstruct(&legacy.base.inquiry, &contexts)? != *query {
                return Err(format!("query {} disagrees with linked inquiry", query.id));
            }
            queries.push(entry);
        }
        let compact = Self {
            format: Self::FORMAT.into(),
            base: legacy.base.clone(),
            provenance: legacy.provenance.clone().with_queries(queries),
            contexts,
        };
        compact.validate()?;
        Ok(compact)
    }
    pub fn query(&self, id: QueryId) -> Result<p::Query, String> {
        let query = usize::try_from(id.0)
            .ok()
            .and_then(|i| self.provenance.queries.get(i))
            .ok_or_else(|| format!("dangling query {}", id.0))?;
        if query.id != id {
            return Err("query ID does not match archive slot".into());
        }
        query.reconstruct(&self.base.inquiry, &self.contexts)
    }
    pub fn expand(&self) -> Result<crate::experiment007::Snapshot, String> {
        self.validate()?;
        let queries = self
            .provenance
            .queries
            .iter()
            .map(|r| r.reconstruct(&self.base.inquiry, &self.contexts))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(crate::experiment007::Snapshot {
            base: self.base.clone(),
            provenance: self.provenance.clone().with_queries(queries),
        })
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.format != Self::FORMAT {
            return Err(format!("unsupported audit format {}", self.format));
        }
        let p = &self.provenance;
        let events = &self.base.base.events;
        let cognition = &self.base.base.cognition;
        for (index, record) in self.base.inquiry.records.iter().enumerate() {
            if record.id != index as u64 {
                return Err("inquiry ID does not match archive slot".into());
            }
            if !record.valid {
                continue;
            }
            let input = &record.decision.input;
            let observed = record
                .tick
                .checked_sub(u64::from(record.time_spent))
                .ok_or("invalid inquiry time span")?;
            if record.food_before.first().map(|p| p.0) != Some(input.owner) {
                return Err("inquiry actor ownership mismatch".into());
            }
            for cell in &input.cells {
                let earlier = self
                    .base
                    .inquiry
                    .records
                    .get(cell.last_record as usize)
                    .ok_or("cell references missing inquiry")?;
                if cell.last_record >= record.id
                    || earlier.tick > observed
                    || earlier.decision.input.owner != input.owner
                    || earlier.decision.selected
                        != (q::Action::Ask {
                            concern: cell.concern,
                            source: cell.source,
                            strategy: cell.strategy,
                        })
                {
                    return Err("cell references future/foreign/unrelated inquiry".into());
                }
            }
            for signature in &input.seen {
                let info = cognition
                    .information
                    .get(signature.receipt as usize)
                    .ok_or("signature references missing information")?;
                if info.listener != input.owner
                    || info.event != signature.event
                    || info.tick > observed
                {
                    return Err("signature references private/future information".into());
                }
            }
            for concern in &input.concerns {
                let event = events
                    .get(concern.event as usize)
                    .ok_or("concern references missing event")?;
                if concern.owner != input.owner
                    || concern.target != event.decision.actor
                    || event.tick > observed
                    || !event.participants.contains(&input.owner)
                    || concern.created_scene != event.scene
                {
                    return Err("concern event ownership mismatch".into());
                }
            }
        }
        for (i, root) in p.roots.iter().enumerate() {
            let e = events
                .get(root.event as usize)
                .ok_or("root references missing event")?;
            if root.id != i as u64
                || e.id != root.event
                || e.tick > root.tick
                || e.outcome != Some(crate::model::Outcome::Refusal)
            {
                return Err(format!("root {i}: invalid event/time/identity"));
            }
        }
        for (i, r) in p.receipts.iter().enumerate() {
            let root = p
                .roots
                .get(r.actual_root as usize)
                .ok_or("receipt references missing root")?;
            if r.id != i as u64
                || root.event != r.event
                || root.tick > r.tick
                || root.claim != r.knowledge.claim
                || r.knowledge.event != r.event
                || r.knowledge.receipt != r.id
                || r.knowledge.communicator != r.communicator
                || r.knowledge.quality > root.quality
                || r.knowledge.quality < 0
                || r.knowledge.known != r.attributed.then_some(root.id)
            {
                return Err(format!("receipt {i}: invalid immutable provenance"));
            }
            if let Some(parent) = r.parent {
                let old = p
                    .receipts
                    .get(parent as usize)
                    .ok_or("receipt references missing parent")?;
                if parent >= r.id
                    || old.tick > r.tick
                    || old.listener != r.communicator
                    || old.event != r.event
                    || old.actual_root != r.actual_root
                    || Some(r.depth) != old.depth.checked_add(1)
                    || r.depth > p::MAX_DEPTH
                    || r.knowledge.message != parent
                    || r.knowledge.quality > old.knowledge.quality
                    || (r.attributed && old.knowledge.known.is_none())
                {
                    return Err(format!("receipt {i}: invalid relay parent/ownership"));
                }
            } else if r.depth != 0
                || root.observer != r.listener
                || r.communicator != r.listener
                || r.knowledge.message != r.id
            {
                return Err(format!("receipt {i}: invalid acquisition ownership"));
            }
            if let Some(id) = r.cognition_receipt {
                let info = cognition
                    .information
                    .get(id as usize)
                    .ok_or("missing cognition receipt")?;
                let e = &events[r.event as usize];
                if info.id != id
                    || info.event != r.event
                    || info.listener != r.listener
                    || info.speaker != e.decision.actor
                    || info.after.support != r.evaluation.after
                    || info.tick > r.tick
                {
                    return Err(format!("receipt {i}: invalid cognitive bridge"));
                }
            }
        }
        let mut last = None;
        for (i, r) in p.queries.iter().enumerate() {
            if r.id.0 != i as u64 || last.is_some_and(|id| id >= r.inquiry.0) {
                return Err("query/inquiry references out of order".into());
            }
            last = Some(r.inquiry.0);
            let expanded = r.reconstruct(&self.base.inquiry, &self.contexts)?;
            if let Some(id) = r.receipt {
                let receipt = p
                    .receipts
                    .get(id.0 as usize)
                    .ok_or("query references missing receipt")?;
                if receipt.listener != r.owner
                    || receipt.tick < r.observed_at
                    || receipt.tick > r.completed_at
                {
                    return Err(format!("query {i}: receipt ownership/time mismatch"));
                }
            }
            // Invalid attempted policy data remains inspectable; it is not trusted
            // local state and is not interpreted as authoritative causal references.
            if expanded.valid {
                let local = self.contexts.get(r.context)?;
                if local.knowledge.len() > p::CAPACITY || local.hints.len() > p::CAPACITY {
                    return Err("context exceeds cognition bound".into());
                }
                for k in &local.knowledge {
                    let receipt = p
                        .receipts
                        .get(k.receipt as usize)
                        .ok_or("context references missing acquisition")?;
                    if receipt.listener != r.owner
                        || receipt.tick > r.observed_at
                        || receipt.event != k.event
                        || receipt.knowledge.claim != k.claim
                        || receipt.knowledge.quality != k.quality
                        || receipt.knowledge.message != k.message
                        || receipt.communicator != k.communicator
                        || k.known.is_some_and(|id| id != receipt.actual_root)
                    {
                        return Err(format!(
                            "query {i}: private/future/invalid acquisition reference"
                        ));
                    }
                    if k.known != receipt.knowledge.known
                        && !p.receipts.iter().any(|revelation| {
                            revelation.listener == r.owner
                                && revelation.tick <= r.observed_at
                                && revelation.event == k.event
                                && revelation.communicator == k.communicator
                                && revelation.knowledge.message == k.message
                                && revelation.knowledge.known == k.known
                                && k.known.is_some()
                        })
                    {
                        return Err(format!("query {i}: attribution was not locally disclosed"));
                    }
                }
                for h in &local.hints {
                    let receipt = p
                        .receipts
                        .get(h.receipt as usize)
                        .ok_or("context references missing hint receipt")?;
                    if receipt.tick > r.observed_at
                        || receipt.event != h.event
                        || h.known.is_some_and(|id| id != receipt.actual_root)
                        || !(receipt.listener == h.source
                            || (receipt.listener == r.owner && receipt.communicator == h.source))
                    {
                        return Err(format!("query {i}: invalid hint ownership/time"));
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hash_bucket_collision_cannot_alias_unequal_contexts() {
        let mut store = ContextStore::default();
        let old = store.intern(vec![1]);
        let other = vec![2];
        let hash = ContextStore::hash(&other);
        store.lookup.insert(hash, vec![0]);
        let new = store.intern(other);
        assert_ne!(new, old);
        assert_eq!(store.get(old).unwrap(), &vec![1]);
        assert_eq!(store.get(new).unwrap(), &vec![2]);
    }
}
