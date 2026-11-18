//! Deterministic conventional control; no resolver, RNG, ECS or storage access.
use crate::model::*;
use std::collections::BTreeMap;

pub trait Controller {
    fn identity(&self) -> &'static str;
    fn decide(&self, view: &OwnedView) -> Choice;
}

/// Future bounded planning/external proposal seam. No provider is installed in v1.
pub trait ProposalSource {
    fn proposals(&self, view: &OwnedView, limit: usize) -> Vec<Action>;
}

pub struct Utility;
impl Controller for Utility {
    fn identity(&self) -> &'static str {
        "utility/v1"
    }
    fn decide(&self, view: &OwnedView) -> Choice {
        choose(view)
    }
}
fn candidate(action: Action, terms: &[(&str, i32)]) -> Candidate {
    let terms: BTreeMap<_, _> = terms
        .iter()
        .map(|(key, value)| (key.to_string(), *value))
        .collect();
    Candidate {
        action,
        score: terms.values().sum(),
        terms,
    }
}
fn local_stock(view: &OwnedView) -> Option<u32> {
    view.own
        .beliefs
        .iter()
        .find(|b| b.place == view.own.place)
        .map(|b| b.stock)
}
fn cooling(view: &OwnedView, other: CharacterId) -> bool {
    view.own
        .memories
        .iter()
        .any(|m| m.other == Some(other) && view.tick.saturating_sub(m.tick) < 12)
}
pub fn ordinary(view: &OwnedView) -> Vec<Candidate> {
    let own = &view.own;
    let deficit = own.reserve_goal.saturating_sub(own.food) as i32;
    let mut list = vec![candidate(Action::Wait, &[("rest", 0)])];
    if own.food > 0 && own.hunger >= 30 && view.remaining >= 1 {
        list.push(candidate(
            Action::Eat,
            &[("hunger", own.hunger), ("relief", 15)],
        ));
    }
    if local_stock(view).is_some_and(|stock| stock > 0) && deficit > 0 && view.remaining >= 3 {
        list.push(candidate(
            Action::Gather,
            &[
                ("need", own.hunger / 2),
                ("reserve", deficit * 12),
                ("work", 16),
                ("self_reliance", own.caution / 4),
            ],
        ));
    }
    if deficit > 0 && view.remaining >= 2 {
        for to in &view.neighbors {
            let known = own.beliefs.iter().find(|b| b.place == *to);
            let opportunity = match known {
                Some(b) if b.stock > 0 => 32,
                Some(_) => -100,
                None => 18,
            };
            let local_penalty = if local_stock(view).is_some_and(|s| s > 0) {
                30
            } else {
                0
            };
            list.push(candidate(
                Action::Travel { to: *to },
                &[
                    ("reserve", deficit * 8),
                    ("opportunity", opportunity),
                    ("local_work", -local_penalty),
                    ("hunger", own.hunger / 5),
                ],
            ));
        }
    }
    if view.remaining >= 3 {
        for p in view
            .people
            .iter()
            .filter(|p| p.available && p.id != own.id && !cooling(view, p.id))
        {
            let trust = own.trust(p.id) / 8;
            if own.food == 0 {
                list.push(candidate(
                    Action::Invite {
                        to: p.id,
                        kind: InvitationKind::RequestFood,
                    },
                    &[
                        ("need", own.hunger / 2),
                        ("appeal", 50),
                        ("caution", -own.caution / 3),
                        ("trust", trust),
                    ],
                ));
            }
            if own.food >= 2 && p.asking_food {
                list.push(candidate(
                    Action::Invite {
                        to: p.id,
                        kind: InvitationKind::OfferFood,
                    },
                    &[
                        ("generosity", own.generosity),
                        ("own_need", -own.hunger / 2),
                        ("trust", trust),
                    ],
                ));
            }
            if deficit > 0 && local_stock(view).is_some_and(|s| s >= 2) {
                list.push(candidate(
                    Action::Invite {
                        to: p.id,
                        kind: InvitationKind::GatherTogether,
                    },
                    &[
                        ("need", own.hunger / 2),
                        ("reserve", deficit * 12),
                        ("efficient_work", 24),
                        ("caution", -own.caution / 5),
                        ("trust", trust),
                    ],
                ));
            }
        }
    }
    list
}
pub fn choose(view: &OwnedView) -> Choice {
    let mut candidates = ordinary(view);
    if !view.incoming.is_empty() {
        let alternative = candidates.iter().map(|c| c.score).max().unwrap_or(0);
        candidates.clear();
        for invite in &view.incoming {
            candidates.push(candidate(
                Action::Decline {
                    invitation: invite.id,
                },
                &[("own_alternative", alternative)],
            ));
            let trust = view.own.trust(invite.proposer) / 6;
            let own = &view.own;
            let accept = match invite.kind {
                InvitationKind::RequestFood if own.food > 0 => Some(vec![
                    ("generosity", own.generosity),
                    ("own_need", -own.hunger / 2),
                    (
                        "reserve_cost",
                        -(own.reserve_goal.saturating_sub(own.food - 1) as i32) * 12,
                    ),
                    ("trust", trust),
                ]),
                InvitationKind::OfferFood if own.food < 1000 => Some(vec![
                    ("need", own.hunger / 2),
                    ("gift", 40),
                    ("trust", trust),
                ]),
                InvitationKind::GatherTogether if local_stock(view).is_some_and(|s| s >= 2) => {
                    Some(vec![
                        ("need", own.hunger / 2),
                        (
                            "reserve",
                            own.reserve_goal.saturating_sub(own.food) as i32 * 12,
                        ),
                        ("efficient_work", 30),
                        ("caution", -own.caution / 5),
                        ("trust", trust),
                    ])
                }
                _ => None,
            };
            if let Some(terms) = accept {
                candidates.push(candidate(
                    Action::Accept {
                        invitation: invite.id,
                    },
                    &terms,
                ));
            }
        }
    }
    // Canonical enum/ID order breaks ties, never host/map traversal or provider order.
    candidates.sort_by(|a, b| a.action.cmp(&b.action));
    let selected = candidates
        .iter()
        .fold(&candidates[0], |best, next| {
            if next.score > best.score { next } else { best }
        })
        .action
        .clone();
    Choice {
        candidates,
        selected,
    }
}
