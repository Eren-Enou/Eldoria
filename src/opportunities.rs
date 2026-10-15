//! Opt-in017 actor activities and observer references; no new cognition.
use crate::{cognition::Consumption, encounters as e};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Serial,
    Committed,
    SerialActive,
    Active,
    NoHistory,
}
pub const MODES: [Mode; 5] = [
    Mode::Serial,
    Mode::Committed,
    Mode::SerialActive,
    Mode::Active,
    Mode::NoHistory,
];
impl Mode {
    pub fn serial(self) -> bool {
        matches!(self, Self::Serial | Self::SerialActive)
    }
    pub fn consumes(self) -> bool {
        matches!(self, Self::SerialActive | Self::Active | Self::NoHistory)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Wait,
    Consume,
    Invite,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Candidate {
    pub action: Action,
    pub eligible: bool,
    pub score: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub contact: e::Decision,
    pub mode: Mode,
    pub candidates: Vec<Candidate>,
    pub selected: Action,
}
pub fn policy(input: &e::Input, mode: Mode) -> Decision {
    let contact = e::policy(
        input,
        if mode == Mode::NoHistory {
            e::Mode::NoHistory
        } else {
            e::Mode::History
        },
    );
    let eat =
        mode.consumes() && input.own.food > 0 && input.own.hunger > 0 && input.now < input.end;
    let invite = contact.selected != e::Intent::Wait;
    let score = contact
        .candidates
        .iter()
        .find(|c| c.intent == contact.selected)
        .unwrap()
        .score;
    let candidates = vec![
        Candidate {
            action: Action::Wait,
            eligible: true,
            score: 0,
        },
        Candidate {
            action: Action::Consume,
            eligible: eat,
            score: if eat { input.own.hunger } else { -1 },
        },
        Candidate {
            action: Action::Invite,
            eligible: invite,
            score: if invite { score } else { -1 },
        },
    ];
    let selected = candidates
        .iter()
        .fold(&candidates[0], |a, b| if b.score > a.score { b } else { a })
        .action;
    Decision {
        contact,
        mode,
        candidates,
        selected,
    }
}
impl Decision {
    pub fn outgoing(&self) -> e::Decision {
        let mut d = self.contact.clone();
        if self.selected != Action::Invite {
            d.selected = e::Intent::Wait;
        }
        d
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub state: e::State,
    pub scenes: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commitment {
    pub invitation: e::Invitation,
    pub scene: u64,
    pub start: u64,
    pub first_event: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    pub wave: u64,
    pub before: usize,
    pub observed: usize,
    pub people: Vec<e::Public>,
    pub decisions: Vec<Option<Decision>>,
    pub sent: Option<usize>,
    pub responses: Vec<e::Response>,
    pub allocations: Vec<e::Allocation>,
    pub admitted: Option<usize>,
    pub query: Option<u64>,
    pub active_before: Option<Commitment>,
    pub active_after: Option<Commitment>,
    pub started: Option<Commitment>,
    pub finished: Option<Commitment>,
    pub consumption: Vec<Consumption>,
    pub after: usize,
}
#[derive(Resource)]
pub(crate) struct Environment {
    pub end: u64,
    pub mode: Mode,
    pub order: e::Order,
    pub wave: u64,
    pub active: Option<Commitment>,
    pub choose: fn(&e::Input, Mode) -> Decision,
}
