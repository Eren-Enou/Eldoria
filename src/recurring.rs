//! Prospective018 elapsed-time need law and local activity controls.
use crate::{encounters as e, opportunities as o};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Fixed,
    Recurring,
    Serial,
    FixedContact,
    ContactOnly,
    EatFirst,
    NoHistory,
}
pub const MODES: [Mode; 7] = [
    Mode::Fixed,
    Mode::Recurring,
    Mode::Serial,
    Mode::FixedContact,
    Mode::ContactOnly,
    Mode::EatFirst,
    Mode::NoHistory,
];
impl Mode {
    pub fn mapped(self) -> o::Mode {
        match self {
            Self::Serial => o::Mode::SerialActive,
            Self::FixedContact | Self::ContactOnly => o::Mode::Committed,
            Self::NoHistory => o::Mode::NoHistory,
            _ => o::Mode::Active,
        }
    }
    pub fn increment(self) -> u32 {
        if matches!(self, Self::Fixed | Self::FixedContact) {
            0
        } else {
            5
        }
    }
    pub fn consumes(self) -> bool {
        self.mapped().consumes()
    }
    pub fn serial(self) -> bool {
        self.mapped().serial()
    }
}
pub fn policy(input: &e::Input, mode: Mode) -> o::Decision {
    let mut d = o::policy(input, mode.mapped());
    if mode == Mode::EatFirst
        && d.candidates
            .iter()
            .any(|c| c.action == o::Action::Consume && c.eligible)
    {
        d.selected = o::Action::Consume;
    }
    d
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub interval: u64,
    pub increment: u32,
}
impl Settings {
    pub fn valid(self) -> bool {
        (2..=128).contains(&self.interval) && [0, 5].contains(&self.increment)
    }
}
/// Saturating pressure accumulation depends on absolute elapsed phase, not calls.
pub fn pressure(hunger: i32, pulses: u64, increment: u32) -> i32 {
    assert!((0..=100).contains(&hunger));
    (hunger + pulses.saturating_mul(u64::from(increment)).min(100) as i32).min(100)
}
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Clock {
    pub epoch: u64,
    pub last: u64,
    pub settings: Settings,
}
impl Clock {
    pub fn pulses(&self, to: u64) -> u64 {
        assert!(self.settings.valid() && to >= self.last && self.last >= self.epoch);
        (to - self.epoch) / self.settings.interval
            - (self.last - self.epoch) / self.settings.interval
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub from: u64,
    pub to: u64,
    pub pulses: u64,
    pub before: [i32; 3],
    pub after: [i32; 3],
}
#[derive(Resource, Default)]
pub(crate) struct History(pub Vec<Record>);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    pub need_before: usize,
    pub activity: o::Step,
    pub need_after: usize,
}
#[derive(Resource)]
pub(crate) struct Environment {
    pub end: u64,
    pub mode: Mode,
    pub order: e::Order,
    pub wave: u64,
    pub active: Option<o::Commitment>,
    pub choose: fn(&e::Input, Mode) -> o::Decision,
}
