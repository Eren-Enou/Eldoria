//! Prospective016 finite own retry feedback, separate from inquiry/trust.
use crate::encounters as e;
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Unchanged,
    Control,
    Recent,
    Typed,
    Untyped,
}
pub const MODES: [Mode; 5] = [
    Mode::Unchanged,
    Mode::Control,
    Mode::Recent,
    Mode::Typed,
    Mode::Untyped,
];
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Declined,
    AcceptedUnmet,
    Met,
    Expired,
    Unavailable,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signal {
    pub owner: u32,
    pub partner: u32,
    pub kind: e::Kind,
    pub sent: u64,
    pub received: u64,
    pub outcome: Outcome,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cell {
    pub partner: u32,
    pub kind: e::Kind,
    pub outcome: Outcome,
    pub received: u64,
    pub until: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub own: e::Input,
    pub mode: Mode,
    pub retained: Vec<Cell>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Update {
    pub signal: Signal,
    pub before: Vec<Cell>,
    pub after: Vec<Cell>,
    pub evicted: Option<Cell>,
}
pub fn expire(cells: &[Cell], now: u64) -> Vec<Cell> {
    cells.iter().filter(|c| c.until > now).cloned().collect()
}
pub fn policy(input: &Input) -> e::Decision {
    let mut d = e::policy(&input.own, e::Mode::History);
    if matches!(input.mode, Mode::Recent | Mode::Typed | Mode::Untyped) {
        for c in &mut d.candidates {
            if let e::Intent::Invite { recipient, kind } = c.intent
                && input
                    .retained
                    .iter()
                    .any(|r| r.partner == recipient && r.kind == kind && r.until > input.own.now)
            {
                c.eligible = false;
                c.score = -1;
            }
        }
        d.selected = d
            .candidates
            .iter()
            .fold(
                &d.candidates[0],
                |a, b| if b.score > a.score { b } else { a },
            )
            .intent;
    }
    d
}
pub fn respond(input: &e::ResponseInput) -> e::Response {
    let mut r = e::respond(input);
    for c in &mut r.candidates {
        if c.reply == e::Reply::Continue {
            c.score = 0;
        }
    }
    r.selected = r
        .candidates
        .iter()
        .fold(
            &r.candidates[0],
            |a, b| if b.score > a.score { b } else { a },
        )
        .reply;
    r
}
/// Pure projection of only one's outgoing result; never expose arbitration detail.
pub fn signal(w: &e::Wave, owner: u32) -> Option<Signal> {
    let d = &w.proposals[owner as usize];
    let e::Intent::Invite { recipient, kind } = d.selected else {
        return None;
    };
    let a = w
        .allocations
        .iter()
        .find(|a| a.invitation.proposer == owner)
        .unwrap();
    let outcome = match a.disposition {
        e::Disposition::Execute | e::Disposition::Reciprocal => Outcome::Met,
        e::Disposition::Expired => Outcome::Expired,
        e::Disposition::Unavailable => Outcome::Unavailable,
        e::Disposition::Declined => Outcome::Declined,
        e::Disposition::Contention => Outcome::AcceptedUnmet,
    };
    Some(Signal {
        owner,
        partner: recipient,
        kind,
        sent: w.before.tick,
        received: w.after.tick,
        outcome,
    })
}
pub fn update(cells: &[Cell], mode: Mode, signal: Signal) -> Update {
    assert!(
        signal.received >= signal.sent + 2
            && signal.owner < 3
            && signal.partner < 3
            && signal.owner != signal.partner
    );
    let before = cells.to_vec();
    let mut after = expire(cells, signal.received);
    let mut evicted = None;
    if matches!(mode, Mode::Unchanged | Mode::Control) {
        after.clear();
    } else {
        if mode == Mode::Recent {
            after.clear();
        } else {
            after.retain(|c| {
                !(c.partner == signal.partner
                    && c.kind == signal.kind
                    && signal.outcome == Outcome::Met)
            });
        }
        if signal.outcome != Outcome::Met {
            let delay = if mode == Mode::Recent {
                6
            } else if mode == Mode::Untyped || signal.outcome == Outcome::Declined {
                12
            } else {
                2
            };
            let cell = Cell {
                partner: signal.partner,
                kind: signal.kind,
                outcome: signal.outcome,
                received: signal.received,
                until: signal.received + delay,
            };
            if let Some(existing) = after
                .iter_mut()
                .find(|c| c.partner == cell.partner && c.kind == cell.kind)
            {
                *existing = cell;
            } else {
                let cap = if mode == Mode::Recent { 1 } else { 2 };
                if after.len() == cap {
                    evicted = Some(after.remove(0));
                }
                after.push(cell);
            }
        }
    }
    Update {
        signal,
        before,
        after,
        evicted,
    }
}
#[derive(Resource)]
pub(crate) struct Learning {
    pub mode: Mode,
    pub cells: [Vec<Cell>; 3],
    pub choose: fn(&Input) -> e::Decision,
    pub learn: fn(&[Cell], Mode, Signal) -> Update,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wave {
    pub memory_before: [Vec<Cell>; 3],
    pub available: [Vec<Cell>; 3],
    pub updates: Vec<Update>,
    pub wave: e::Wave,
}
