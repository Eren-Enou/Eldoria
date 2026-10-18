//! Opt-in021: one owned environmental record, never an observer-history lookup.
use crate::{acquisition as a, encounters as e, opportunities as o, priorities};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};

pub const CAPACITY: usize = 1;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    FreshReference,
    CurrentOnly,
    LastSeen,
    Expire12,
    Unobserved,
}
pub const MODES: [Mode; 5] = [
    Mode::FreshReference,
    Mode::CurrentOnly,
    Mode::LastSeen,
    Mode::Expire12,
    Mode::Unobserved,
];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cell {
    pub owner: u32,
    /// Owner-local receipt namespace; not a count of other people's observations.
    pub receipt: u64,
    pub observation: a::Observation,
    pub own_takes: u32,
}
impl Cell {
    pub fn upper_bound(&self) -> u32 {
        self.observation.stock.saturating_sub(self.own_takes)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Change {
    Sense,
    Depleted,
    Taken,
    Expired,
    Discarded,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub id: u64,
    pub owner: u32,
    pub tick: u64,
    pub kind: Change,
    pub before: Option<Cell>,
    pub after: Option<Cell>,
    /// Observer-only physical result/point reference, excluded from Cell and Input.
    pub physical: Option<u64>,
    pub point: Option<usize>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub legacy: e::Input,
    pub retained: Option<Cell>,
    pub duration: u64,
    pub inquiry_lane_free: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub input: Input,
    pub policy: a::Policy,
    pub base: o::Decision,
    pub candidates: Vec<a::Candidate>,
    pub selected: a::Action,
}
pub fn eligible(i: &Input) -> bool {
    i.retained.as_ref().is_some_and(|c| {
        c.owner == i.legacy.own.id
            && c.observation.observed_at <= i.legacy.now
            && c.upper_bound() > 0
            && c.observation.accessible
    }) && i
        .legacy
        .people
        .iter()
        .any(|p| p.id == i.legacy.own.id && p.available)
        && i.legacy.own.food == 0
        && i.legacy.own.hunger > 0
        && i.legacy
            .now
            .checked_add(i.duration)
            .is_some_and(|t| t <= i.legacy.end)
}
pub fn policy(i: &Input, mode: a::Policy) -> Decision {
    // Exact020 base arithmetic/mask; the only new gate is owned retained knowledge.
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
        .map(|c| a::Candidate {
            action: c.action.into(),
            eligible: c.eligible,
            score: c.score,
        })
        .collect::<Vec<_>>();
    let can = eligible(i);
    candidates.push(a::Candidate {
        action: a::Action::Acquire,
        eligible: can,
        score: if can { i.legacy.own.hunger } else { -1 },
    });
    let selected = match mode {
        a::Policy::Ignored => base.selected.into(),
        a::Policy::Acquire if can => a::Action::Acquire,
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
pub struct Choice {
    pub retained: Option<Cell>,
    pub duration: u64,
    pub inquiry_lane_free: bool,
    pub policy: a::Policy,
    pub candidates: Vec<a::Candidate>,
    pub selected: a::Action,
    pub acquiring_base: Option<o::Decision>,
}
impl From<Decision> for Choice {
    fn from(d: Decision) -> Self {
        Self {
            retained: d.input.retained,
            duration: d.input.duration,
            inquiry_lane_free: d.input.inquiry_lane_free,
            policy: d.policy,
            candidates: d.candidates,
            selected: d.selected,
            acquiring_base: (d.selected == a::Action::Acquire).then_some(d.base),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    pub need_before: usize,
    pub activity: o::Step,
    pub need_after: usize,
    pub choices: Vec<Option<Choice>>,
    pub admissions: Vec<a::Attempt>,
    pub completed: Vec<u64>,
    pub skipped_responses: Vec<u32>,
    pub provisional: Vec<e::Allocation>,
    pub atomic_blocked: Vec<e::Invitation>,
    pub memory_before: usize,
    pub memory_after: usize,
}
/// Local experience projection: no objective stock balances or rival information.
#[derive(Clone, Copy, Debug)]
pub struct LocalResult {
    pub owner: u32,
    pub cache: a::CacheId,
    pub at: u64,
    pub outcome: a::Outcome,
    pub physical: u64,
    pub duration: u64,
}
#[derive(Resource)]
pub struct Store {
    pub mode: Mode,
    pub epoch: u64,
    pub cadence: u64,
    slots: [Option<Cell>; 3],
    receipts: [u64; 3],
    pub records: Vec<Record>,
    pub(crate) choose: fn(&Input, a::Policy) -> Decision,
}
impl Store {
    pub fn new(mode: Mode, epoch: u64, cadence: u64) -> Self {
        assert!([4, 8, 16].contains(&cadence));
        Self {
            mode,
            epoch,
            cadence,
            slots: [None, None, None],
            receipts: [0; 3],
            records: vec![],
            choose: policy,
        }
    }
    pub fn wants_sense(&self, now: u64) -> bool {
        assert!(now >= self.epoch);
        self.mode == Mode::FreshReference
            || (self.mode != Mode::Unobserved && (now - self.epoch).is_multiple_of(self.cadence))
    }
    fn change(
        &mut self,
        owner: u32,
        tick: u64,
        kind: Change,
        after: Option<Cell>,
        physical: Option<u64>,
        point: Option<usize>,
    ) {
        let before = self.slots[owner as usize].clone();
        self.records.push(Record {
            id: self.records.len() as u64,
            owner,
            tick,
            kind,
            before,
            after: after.clone(),
            physical,
            point,
        });
        self.slots[owner as usize] = after;
    }
    pub fn expire(&mut self, owner: u32, now: u64) {
        if self.slots[owner as usize]
            .as_ref()
            .is_some_and(|c| match self.mode {
                Mode::Expire12 => now.saturating_sub(c.observation.observed_at) >= 12,
                Mode::CurrentOnly => c.observation.observed_at != now,
                Mode::Unobserved => true,
                _ => false,
            })
        {
            self.change(owner, now, Change::Expired, None, None, None);
        }
    }
    fn receive(
        &mut self,
        owner: u32,
        observation: a::Observation,
        kind: Change,
        physical: Option<u64>,
        point: Option<usize>,
    ) {
        let receipt = self.receipts[owner as usize];
        self.receipts[owner as usize] += 1;
        let now = observation.observed_at;
        self.change(
            owner,
            now,
            kind,
            Some(Cell {
                owner,
                receipt,
                observation,
                own_takes: 0,
            }),
            physical,
            point,
        );
    }
    pub fn prepare(
        &mut self,
        owner: u32,
        now: u64,
        sensed: Option<a::Observation>,
        point: usize,
    ) -> Option<Cell> {
        self.expire(owner, now);
        assert_eq!(sensed.is_some(), self.wants_sense(now));
        if let Some(obs) = sensed {
            assert_eq!(obs.observed_at, now);
            self.receive(owner, obs, Change::Sense, None, Some(point));
        }
        self.slots[owner as usize].clone()
    }
    pub fn after_decision(&mut self, owner: u32, now: u64) {
        if self.mode == Mode::CurrentOnly && self.slots[owner as usize].is_some() {
            self.change(owner, now, Change::Discarded, None, None, None);
        }
    }
    pub fn result(&mut self, r: LocalResult) {
        self.expire(r.owner, r.at);
        if self.mode == Mode::Unobserved {
            return;
        }
        match r.outcome {
            a::Outcome::Granted => {
                // A live attempt/audit may retain its old observation. Never use
                // that copy to resurrect a missing/expired cognitive record.
                if let Some(mut cell) = self.slots[r.owner as usize].clone() {
                    assert_eq!(cell.observation.id, r.cache);
                    cell.own_takes = cell.own_takes.saturating_add(1);
                    self.change(
                        r.owner,
                        r.at,
                        Change::Taken,
                        Some(cell),
                        Some(r.physical),
                        None,
                    );
                }
            }
            a::Outcome::Depleted => self.receive(
                r.owner,
                a::Observation {
                    id: r.cache,
                    observed_at: r.at,
                    stock: 0,
                    accessible: true,
                    duration: r.duration,
                },
                Change::Depleted,
                Some(r.physical),
                None,
            ),
            a::Outcome::Overflow => {}
        }
    }
    pub fn finish(&mut self, now: u64) -> Vec<Option<Cell>> {
        for owner in 0..3 {
            self.expire(owner, now);
        }
        self.slots.to_vec()
    }
}
