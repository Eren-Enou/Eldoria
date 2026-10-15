//! Prospective019 stateless activity controls over the protected018 input.
use crate::{encounters as e, opportunities as o, recurring as r};
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Baseline,
    EatFirst,
    Deadline,
    Scale90,
    Scale110,
    NoHistory,
}
pub const MODES: [Mode; 6] = [
    Mode::Baseline,
    Mode::EatFirst,
    Mode::Deadline,
    Mode::Scale90,
    Mode::Scale110,
    Mode::NoHistory,
];
impl Mode {
    pub fn legacy(self) -> r::Mode {
        match self {
            Self::EatFirst => r::Mode::EatFirst,
            Self::NoHistory => r::Mode::NoHistory,
            _ => r::Mode::Recurring,
        }
    }
    pub fn selector(self) -> fn(&e::Input, r::Mode) -> o::Decision {
        match self {
            Self::Baseline => pick::<0>,
            Self::EatFirst => pick::<1>,
            Self::Deadline => pick::<2>,
            Self::Scale90 => pick::<3>,
            Self::Scale110 => pick::<4>,
            Self::NoHistory => pick::<5>,
        }
    }
}
fn pick<const N: usize>(input: &e::Input, carrier: r::Mode) -> o::Decision {
    assert_eq!(carrier, MODES[N].legacy());
    policy(input, MODES[N])
}
/// Public conservative admission geometry only; no actual future duration.
pub fn room_after_meal(now: u64, end: u64, reservation: u64) -> bool {
    now.checked_add(1)
        .and_then(|t| t.checked_add(reservation))
        .is_some_and(|t| t <= end)
}
pub fn policy(input: &e::Input, mode: Mode) -> o::Decision {
    let mut d = r::policy(input, mode.legacy());
    if matches!(mode, Mode::Scale90 | Mode::Scale110) {
        let invite = &mut d.candidates[2];
        assert_eq!(invite.action, o::Action::Invite);
        if invite.eligible {
            invite.score = invite.score * if mode == Mode::Scale90 { 9 } else { 11 } / 10;
        }
        d.selected = d
            .candidates
            .iter()
            .fold(
                &d.candidates[0],
                |a, b| if b.score > a.score { b } else { a },
            )
            .action;
    }
    if mode == Mode::Deadline && d.candidates[1].eligible {
        d.selected = if d.candidates[2].eligible
            && !room_after_meal(
                input.now,
                input.end,
                d.contact
                    .candidates
                    .iter()
                    .find(|c| c.intent == d.contact.selected)
                    .unwrap()
                    .reservation,
            ) {
            o::Action::Invite
        } else {
            o::Action::Consume
        };
    }
    d
}
