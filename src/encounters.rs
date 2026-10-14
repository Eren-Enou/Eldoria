//! 015-only local invitation policies and objective bounded matching.
use crate::{behavior::scarcity_policy, intentional::Profile, model::*, provenance};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Simple,
    History,
    NoHistory,
}
pub const MODES: [Mode; 3] = [Mode::Simple, Mode::History, Mode::NoHistory];
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Order {
    Ascending,
    Descending,
    Rotating,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    Resource,
    Inquiry,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Intent {
    Wait,
    Invite { recipient: u32, kind: Kind },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Public {
    pub id: u32,
    pub available: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub own: Agent,
    pub profile: Profile,
    pub now: u64,
    pub end: u64,
    pub people: Vec<Public>,
    pub query: provenance::Decision,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Candidate {
    pub intent: Intent,
    pub reservation: u64,
    pub eligible: bool,
    pub score: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub input: Input,
    pub mode: Mode,
    pub candidates: Vec<Candidate>,
    pub selected: Intent,
}
pub fn resource_value(own: &Agent, partner: u32) -> i32 {
    let d = scarcity_policy(
        own,
        &Observation {
            partner,
            last_signal: None,
            amount: 1,
            turns_left: 6,
        },
    );
    d.candidates.iter().map(|c| c.score).max().unwrap().max(0) + own.hunger.max(0) / 2 - 2
}
pub fn policy(input: &Input, mode: Mode) -> Decision {
    let free = |id| input.people.iter().any(|p| p.id == id && p.available);
    let mut candidates = vec![Candidate {
        intent: Intent::Wait,
        reservation: 0,
        eligible: true,
        score: 0,
    }];
    let mut own = input.own.clone();
    if mode == Mode::NoHistory {
        own.trust.clear();
        own.memories.clear();
    }
    for p in input.people.iter().filter(|p| p.id != own.id) {
        let eligible =
            free(own.id) && p.available && input.now.checked_add(8).is_some_and(|t| t <= input.end);
        candidates.push(Candidate {
            intent: Intent::Invite {
                recipient: p.id,
                kind: Kind::Resource,
            },
            reservation: 8,
            eligible,
            score: if !eligible {
                -1
            } else if mode == Mode::Simple {
                1
            } else {
                resource_value(&own, p.id)
            },
        });
    }
    if let crate::inquiry::Action::Ask {
        source, strategy, ..
    } = input.query.decision.selected
    {
        let reservation = u64::from(strategy.cost()) + 5;
        let eligible = free(own.id)
            && free(source)
            && input
                .now
                .checked_add(reservation)
                .is_some_and(|t| t <= input.end);
        let value = input
            .query
            .decision
            .candidates
            .iter()
            .find(|c| c.action == input.query.decision.selected)
            .unwrap()
            .score
            - 2;
        candidates.push(Candidate {
            intent: Intent::Invite {
                recipient: source,
                kind: Kind::Inquiry,
            },
            reservation,
            eligible,
            score: if !eligible {
                -1
            } else if mode == Mode::Simple {
                1
            } else {
                value
            },
        });
    }
    let selected = candidates
        .iter()
        .fold(&candidates[0], |a, b| if b.score > a.score { b } else { a })
        .intent;
    Decision {
        input: input.clone(),
        mode,
        candidates,
        selected,
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Invitation {
    pub proposer: u32,
    pub recipient: u32,
    pub kind: Kind,
    pub expires: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reply {
    Wait,
    Accept(u32),
    Continue,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseInput {
    pub own: Input,
    pub outgoing: Decision,
    pub incoming: Vec<Invitation>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyCandidate {
    pub reply: Reply,
    pub score: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    pub input: ResponseInput,
    pub candidates: Vec<ReplyCandidate>,
    pub selected: Reply,
}
pub fn respond(input: &ResponseInput) -> Response {
    let mut candidates = vec![ReplyCandidate {
        reply: Reply::Wait,
        score: 0,
    }];
    for invite in &input.incoming {
        let score = if !input
            .own
            .people
            .iter()
            .any(|p| p.id == input.own.own.id && p.available)
        {
            -1
        } else {
            match invite.kind {
                Kind::Resource => resource_value(&input.own.own, invite.proposer),
                Kind::Inquiry => {
                    input.own.profile.relationship_goal - input.own.profile.privacy - 2
                }
            }
        };
        candidates.push(ReplyCandidate {
            reply: Reply::Accept(invite.proposer),
            score,
        });
    }
    if input.outgoing.selected != Intent::Wait {
        let score = input
            .outgoing
            .candidates
            .iter()
            .find(|c| c.intent == input.outgoing.selected)
            .unwrap()
            .score;
        candidates.push(ReplyCandidate {
            reply: Reply::Continue,
            score,
        });
    }
    let selected = candidates
        .iter()
        .fold(&candidates[0], |a, b| if b.score > a.score { b } else { a })
        .reply;
    Response {
        input: input.clone(),
        candidates,
        selected,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Disposition {
    Execute,
    Reciprocal,
    Declined,
    Expired,
    Unavailable,
    Contention,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Allocation {
    pub invitation: Invitation,
    pub disposition: Disposition,
}
pub fn arbitrate(
    now: u64,
    order: Order,
    wave: u64,
    people: &[Public],
    invitations: &[Invitation],
    responses: &[Response],
) -> Vec<Allocation> {
    let mut pending = invitations.to_vec();
    pending.sort_by_key(|i| match order {
        Order::Ascending => i.proposer,
        Order::Descending => 2 - i.proposer,
        Order::Rotating => (i.proposer + 3 - (wave % 3) as u32) % 3,
    });
    let mut occupied = Vec::new();
    let mut executed: Vec<Invitation> = Vec::new();
    pending
        .into_iter()
        .map(|invitation| {
            let i = &invitation;
            let disposition = if now != i.expires {
                Disposition::Expired
            } else if ![i.proposer, i.recipient]
                .iter()
                .all(|id| people.iter().any(|p| p.id == *id && p.available))
            {
                Disposition::Unavailable
            } else if !responses.iter().any(|r| {
                r.input.own.own.id == i.recipient && r.selected == Reply::Accept(i.proposer)
            }) {
                Disposition::Declined
            } else if executed
                .iter()
                .any(|p| p.proposer == i.recipient && p.recipient == i.proposer && p.kind == i.kind)
            {
                Disposition::Reciprocal
            } else if [i.proposer, i.recipient]
                .iter()
                .any(|id| occupied.contains(id))
            {
                Disposition::Contention
            } else {
                occupied.extend([i.proposer, i.recipient]);
                executed.push(i.clone());
                Disposition::Execute
            };
            Allocation {
                invitation,
                disposition,
            }
        })
        .collect()
}
#[derive(Resource)]
pub(crate) struct Policies {
    pub initiation: fn(&Input, Mode) -> Decision,
    pub response: fn(&ResponseInput) -> Response,
}
impl Default for Policies {
    fn default() -> Self {
        Self {
            initiation: policy,
            response: respond,
        }
    }
}
#[derive(Resource)]
pub(crate) struct Environment {
    pub end: u64,
    pub busy_until: [u64; 3],
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub tick: u64,
    pub agents: Vec<Agent>,
    pub events: usize,
    pub queries: usize,
    pub assessment: usize,
    pub concerns: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Execution {
    pub invitation: Invitation,
    pub before: State,
    pub after: State,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wave {
    pub id: u64,
    pub before: State,
    pub proposals: Vec<Decision>,
    pub responses: Vec<Response>,
    pub allocations: Vec<Allocation>,
    pub execution: Option<Execution>,
    pub after: State,
}
