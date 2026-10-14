//! 014-only pure activity selection and bounded public environment windows.
use crate::{
    model::{Agent, Decision as ResourceDecision},
    provenance,
};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    ResourceFirst,
    InquiryFirst,
    Local,
    NoDeadline,
    LocalPulse,
}
pub const MODES: [Mode; 5] = [
    Mode::ResourceFirst,
    Mode::InquiryFirst,
    Mode::Local,
    Mode::NoDeadline,
    Mode::LocalPulse,
];
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Wait,
    Resource,
    Inquiry,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Window {
    pub action: Action,
    pub closes: u64,
    pub reservation: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub own: Agent,
    pub now: u64,
    pub end: u64,
    pub windows: Vec<Window>,
    pub inquiry: Option<provenance::Decision>,
    pub resource: Option<ResourceDecision>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Candidate {
    pub action: Action,
    pub eligible: bool,
    pub base: i32,
    pub urgency: i32,
    pub score: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub input: Input,
    pub mode: Mode,
    pub candidates: Vec<Candidate>,
    pub selected: Action,
}
pub fn eligible(input: &Input, w: &Window) -> bool {
    input.now < w.closes
        && input
            .now
            .checked_add(w.reservation)
            .is_some_and(|end| end <= w.closes.min(input.end))
}
pub fn policy(input: &Input, mode: Mode) -> Decision {
    let mut candidates = vec![Candidate {
        action: Action::Wait,
        eligible: true,
        base: 0,
        urgency: 0,
        score: 0,
    }];
    for action in [Action::Resource, Action::Inquiry] {
        let Some(w) = input.windows.iter().find(|w| w.action == action) else {
            continue;
        };
        let available = eligible(input, w);
        let base = match action {
            Action::Resource => {
                input
                    .resource
                    .as_ref()
                    .unwrap()
                    .candidates
                    .iter()
                    .map(|c| c.score)
                    .max()
                    .unwrap()
                    .max(0)
                    + input.own.hunger.max(0) / 2
            }
            Action::Inquiry => {
                let d = &input.inquiry.as_ref().unwrap().decision;
                d.candidates
                    .iter()
                    .find(|c| c.action == d.selected)
                    .unwrap()
                    .score
            }
            Action::Wait => 0,
        };
        let urgency = if matches!(mode, Mode::Local | Mode::LocalPulse)
            && input.windows.iter().any(|other| {
                other.action != action
                    && eligible(input, other)
                    && input.now + other.reservation + w.reservation > w.closes.min(input.end)
            }) {
            25
        } else {
            0
        };
        let score = if !available {
            -1
        } else {
            match mode {
                Mode::ResourceFirst => {
                    if action == Action::Resource {
                        2
                    } else {
                        1
                    }
                }
                Mode::InquiryFirst => {
                    if action == Action::Inquiry {
                        2
                    } else {
                        1
                    }
                }
                _ => base + urgency,
            }
        };
        candidates.push(Candidate {
            action,
            eligible: available,
            base,
            urgency,
            score,
        });
    }
    let selected = candidates
        .iter()
        .fold(&candidates[0], |a, b| if b.score > a.score { b } else { a })
        .action;
    Decision {
        input: input.clone(),
        mode,
        candidates,
        selected,
    }
}
#[derive(Resource, Clone, Debug)]
pub(crate) struct Clock {
    pub owner: u32,
    pub source: Option<u32>,
    pub partner: Option<u32>,
    pub end: u64,
    pub inquiry_close: u64,
    pub resource_close: u64,
    pub inquiry_used: bool,
    pub resource_used: bool,
    pub turns: u32,
}
