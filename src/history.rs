//! Observer/resolver indexes. None of these types are individual cognition.
//! All cached values can be rebuilt from immutable resource/information records.
use crate::{
    cognition::{EvidenceKind, InformationScene},
    model::{AgentId, Event},
};
use bevy_ecs::prelude::Resource;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct EvidenceSummary {
    pub first_trust: i32,
    pub positive: i32,
    pub negative: i32,
    pub has_readings: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Contribution {
    event: u64,
    valence: i32,
    trust_after: i32,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Relationship {
    initial: i32,
    entries: Vec<Contribution>,
    positions: BTreeMap<u64, usize>,
}
impl Relationship {
    fn append(&mut self, event: u64, valence: i32) {
        let trust = self.entries.last().map_or(self.initial, |c| c.trust_after);
        self.positions.insert(event, self.entries.len());
        self.entries.push(Contribution {
            event,
            valence,
            trust_after: (trust + valence).clamp(-100, 100),
        });
    }
    fn revise(&mut self, event: u64, valence: i32) -> i32 {
        let index = self.positions[&event];
        self.entries[index].valence = valence;
        let mut trust = if index == 0 {
            self.initial
        } else {
            self.entries[index - 1].trust_after
        };
        for entry in &mut self.entries[index..] {
            trust = (trust + entry.valence).clamp(-100, 100);
            if trust == entry.trust_after {
                break;
            }
            entry.trust_after = trust;
        }
        self.entries.last().unwrap().trust_after
    }
}
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct HistoryIndex {
    pub evidence: BTreeMap<(u64, AgentId), EvidenceSummary>,
    pub sources: BTreeMap<(u64, AgentId, AgentId, u8), EvidenceKind>,
    pub inspection_slots: BTreeMap<(u64, AgentId), u64>,
    relationships: BTreeMap<(AgentId, AgentId), Relationship>,
}
impl HistoryIndex {
    pub fn counts(&self) -> BTreeMap<String, usize> {
        BTreeMap::from([
            ("relationships".into(), self.relationships.len()),
            (
                "contributions".into(),
                self.relationships.values().map(|r| r.entries.len()).sum(),
            ),
            (
                "event_positions".into(),
                self.relationships.values().map(|r| r.positions.len()).sum(),
            ),
            ("evidence_questions".into(), self.evidence.len()),
            ("received_sources".into(), self.sources.len()),
            ("inspection_slots".into(), self.inspection_slots.len()),
        ])
    }
    pub fn event(&mut self, event: &Event, initial: &BTreeMap<AgentId, BTreeMap<AgentId, i32>>) {
        for (index, &owner) in event.participants.iter().enumerate() {
            let memory = &event.interpretations[index];
            self.relationships
                .entry((owner, memory.partner))
                .or_insert_with(|| Relationship {
                    initial: *initial[&owner].get(&memory.partner).unwrap_or(&0),
                    ..Default::default()
                })
                .append(event.id, memory.valence);
        }
    }
    pub fn information(&mut self, info: &InformationScene) {
        self.remember_evidence(info);
        if let Some(revision) = &info.revision {
            self.revise(
                info.listener,
                info.speaker,
                info.event,
                revision.after.valence,
            );
        }
    }
    pub fn remember_evidence(&mut self, info: &InformationScene) {
        let summary = self
            .evidence
            .entry((info.event, info.listener))
            .or_insert_with(|| EvidenceSummary {
                first_trust: info.trust_before,
                ..Default::default()
            });
        if let EvidenceKind::Fallible {
            source,
            scarce,
            reliability,
        } = info.kind
        {
            summary.has_readings = true;
            if scarce {
                summary.positive = summary.positive.max(reliability);
            } else {
                summary.negative = summary.negative.max(reliability);
            }
            self.sources
                .entry((info.event, info.speaker, info.listener, source))
                .or_insert(info.kind);
        }
    }
    pub fn revise(&mut self, owner: AgentId, partner: AgentId, event: u64, valence: i32) -> i32 {
        self.relationships
            .get_mut(&(owner, partner))
            .expect("validated event relationship")
            .revise(event, valence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prefix_updates_match_full_clamped_replay_with_signed_revisions() {
        for initial in [-100, 0, 100] {
            let mut ledger = Relationship {
                initial,
                ..Default::default()
            };
            let mut values = vec![];
            let mut random = 42u64;
            for step in 0..1000 {
                random = random.wrapping_mul(6364136223846793005).wrapping_add(1);
                let valence = (random % 81) as i32 - 40;
                if step % 3 == 0 || values.is_empty() {
                    ledger.append(values.len() as u64, valence);
                    values.push(valence);
                } else {
                    let index = (random as usize) % values.len();
                    values[index] = valence;
                    ledger.revise(index as u64, valence);
                }
                let mut expected = initial;
                for (entry, value) in ledger.entries.iter().zip(&values) {
                    expected = (expected + value).clamp(-100, 100);
                    assert_eq!(entry.trust_after, expected);
                }
            }
        }
    }
}
