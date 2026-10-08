//! Experiment 008: prospective bounded local assessment, never observer lookup.
use crate::model::AgentId;
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};
pub const CAPACITY: usize = 32;
pub const RULES: &str = "experiment-008-v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Receipt {
    Native(u64),
    Provenance(u64),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Origin {
    Claim(AgentId),
    Disclosure(AgentId),
    NativeReading { speaker: AgentId, channel: u8 },
    Known(u64),
    Unknown(AgentId),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub receipt: Receipt,
    pub event: u64,
    pub communicator: AgentId,
    pub message: Option<u64>,
    pub origin: Origin,
    pub claim: bool,
    pub quality: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Method {
    Max,
    Grouped,
}
impl Method {
    pub fn rule(self) -> fn(&[Item], u64) -> i32 {
        match self {
            Self::Max => strongest,
            Self::Grouped => grouped,
        }
    }
}
pub(crate) type Rule = fn(&[Item], u64) -> i32;
#[derive(Resource)]
pub(crate) struct Assessor(pub Rule);

fn groups(items: &[Item], event: u64) -> BTreeMap<Origin, (i32, i32)> {
    let evidence = items
        .iter()
        .any(|i| i.event == event && i.quality > 0 && !matches!(i.origin, Origin::Claim(_)));
    let mut groups = BTreeMap::<Origin, (i32, i32)>::new();
    for i in items
        .iter()
        .filter(|i| i.event == event && (!evidence || !matches!(i.origin, Origin::Claim(_))))
    {
        let pair = groups.entry(i.origin).or_default();
        if i.claim {
            pair.0 = pair.0.max(i.quality);
        } else {
            pair.1 = pair.1.max(i.quality);
        }
    }
    groups
}
pub fn strongest(items: &[Item], event: u64) -> i32 {
    let groups = groups(items, event);
    groups.values().map(|v| v.0).max().unwrap_or(0)
        - groups.values().map(|v| v.1).max().unwrap_or(0)
}
pub fn grouped(items: &[Item], event: u64) -> i32 {
    let mut positive = 0;
    let mut negative = 0;
    for (p, n) in groups(items, event).values() {
        positive += (100 - positive) * p / 100;
        negative += (100 - negative) * n / 100;
    }
    positive - negative
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub id: u64,
    pub owner: AgentId,
    pub incoming: Item,
    pub prior_belief: Option<i32>,
    pub basis_before: i32,
    pub support: i32,
    pub revised_attribution: Vec<Receipt>,
    pub evicted: Option<Receipt>,
    pub retained: Vec<Receipt>,
}
#[derive(Resource, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assessment {
    pub method: Method,
    pub items: BTreeMap<AgentId, VecDeque<Item>>,
    pub records: Vec<Record>,
}
impl Assessment {
    pub fn new(method: Method) -> Self {
        Self {
            method,
            items: BTreeMap::new(),
            records: vec![],
        }
    }
    pub(crate) fn receive(
        &mut self,
        owner: AgentId,
        incoming: Item,
        prior_belief: Option<i32>,
        rule: Rule,
    ) -> i32 {
        self.receive_selected(owner, incoming, prior_belief, rule, Some(0))
    }
    /// 009 may reject incoming or evict a selected retained slot; old receive stays FIFO.
    pub(crate) fn receive_selected(
        &mut self,
        owner: AgentId,
        incoming: Item,
        prior_belief: Option<i32>,
        rule: Rule,
        eviction: Option<usize>,
    ) -> i32 {
        let items = self.items.entry(owner).or_default();
        let before = rule(items.make_contiguous(), incoming.event);
        let mut revised = vec![];
        if matches!(incoming.origin, Origin::Known(_)) && incoming.message.is_some() {
            for item in items.iter_mut().filter(|i| {
                i.event == incoming.event
                    && i.communicator == incoming.communicator
                    && i.message == incoming.message
                    && matches!(i.origin, Origin::Unknown(_))
            }) {
                item.origin = incoming.origin;
                revised.push(item.receipt);
            }
        }
        let mut retained_incoming = true;
        let evicted = if items.len() == CAPACITY {
            let index = eviction.expect("validated overflow eviction");
            if index == CAPACITY {
                retained_incoming = false;
                Some(incoming.receipt)
            } else {
                items.remove(index).map(|i| i.receipt)
            }
        } else {
            None
        };
        if retained_incoming {
            items.push_back(incoming.clone());
        }
        let support = rule(items.make_contiguous(), incoming.event);
        assert!(
            (-100..=100).contains(&support),
            "assessment rule returned invalid support"
        );
        self.records.push(Record {
            id: self.records.len() as u64,
            owner,
            incoming,
            prior_belief,
            basis_before: before,
            support,
            revised_attribution: revised,
            evicted,
            retained: items.iter().map(|i| i.receipt).collect(),
        });
        support
    }
    /// Observer audit replay, never used to repopulate live cognition.
    pub fn validate(&self) -> Result<(), String> {
        let mut rebuilt = Self::new(self.method);
        for record in &self.records {
            rebuilt.receive(
                record.owner,
                record.incoming.clone(),
                record.prior_belief,
                self.method.rule(),
            );
            if rebuilt.records.last() != Some(record) {
                return Err(format!("assessment record {} does not replay", record.id));
            }
        }
        if rebuilt != *self {
            return Err("assessment live basis differs from replay".into());
        }
        Ok(())
    }
}
