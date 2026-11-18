use bevy_ecs::prelude::{Component, Resource};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

pub const RULES: &str = "eldoria-settlement-rules/v1";
pub const RUNTIME: &str = env!("ELDORIA_RUNTIME");
pub const MEMORY_CAP: usize = 16;
pub const BELIEF_CAP: usize = 8;
pub const RELATIONSHIP_CAP: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CharacterId(pub u64);
pub type PlaceId = u32;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub seed: u64,
    pub population: usize,
    pub horizon: u64,
}
impl Config {
    pub fn validate(&self) -> Result<(), String> {
        if !(1..=128).contains(&self.population)
            || !(8..=4096).contains(&self.horizon)
            || self.population as u64 * self.horizon > 32768
        {
            return Err(
                "population1..128, horizon8..4096, at most32768 actor-ticks required".into(),
            );
        }
        Ok(())
    }
}

/// Pinned SplitMix64 stream; stored as data, not an ECS handle or ambient RNG.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Random {
    pub state: u64,
}
impl Random {
    pub fn draw(&mut self, bound: u64) -> u64 {
        assert!(bound > 0);
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^= z >> 31;
        ((u128::from(z) * u128::from(bound)) >> 64) as u64
    }
}

#[derive(Component, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub place: PlaceId,
    pub food: u32,
    pub hunger: i32,
    pub generosity: i32,
    pub caution: i32,
    pub reserve_goal: u32,
    pub beliefs: VecDeque<SiteBelief>,
    pub relationships: VecDeque<Relationship>,
    pub memories: VecDeque<Episode>,
    pub signal: Option<Signal>,
}
impl Character {
    pub fn trust(&self, other: CharacterId) -> i32 {
        self.relationships
            .iter()
            .find(|r| r.other == other)
            .map_or(0, |r| r.trust)
    }
    pub fn remember(&mut self, episode: Episode) {
        if let Some(other) = episode.other {
            if let Some(relation) = self.relationships.iter_mut().find(|r| r.other == other) {
                relation.trust = (relation.trust + episode.valence).clamp(-100, 100);
                relation.receipt = episode.receipt;
            } else {
                if self.relationships.len() == RELATIONSHIP_CAP {
                    self.relationships.pop_front();
                }
                self.relationships.push_back(Relationship {
                    other,
                    trust: episode.valence.clamp(-100, 100),
                    receipt: episode.receipt,
                });
            }
        }
        if self.memories.len() == MEMORY_CAP {
            self.memories.pop_front();
        }
        self.memories.push_back(episode);
    }
    pub fn observe(&mut self, belief: SiteBelief) {
        if let Some(old) = self.beliefs.iter_mut().find(|b| b.place == belief.place) {
            *old = belief;
        } else {
            if self.beliefs.len() == BELIEF_CAP {
                self.beliefs.pop_front();
            }
            self.beliefs.push_back(belief);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteBelief {
    pub place: PlaceId,
    pub stock: u32,
    pub observed_at: u64,
    pub receipt: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Relationship {
    pub other: CharacterId,
    pub trust: i32,
    pub receipt: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Episode {
    pub receipt: u64,
    pub tick: u64,
    pub other: Option<CharacterId>,
    pub kind: String,
    pub valence: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Signal {
    pub tick: u64,
    pub asking_food: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Place {
    pub id: PlaceId,
    pub name: String,
    pub neighbors: Vec<PlaceId>,
    pub food: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum InvitationKind {
    RequestFood,
    OfferFood,
    GatherTogether,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Action {
    Wait,
    Eat,
    Gather,
    Travel {
        to: PlaceId,
    },
    Invite {
        to: CharacterId,
        kind: InvitationKind,
    },
    Accept {
        invitation: u64,
    },
    Decline {
        invitation: u64,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicPerson {
    pub id: CharacterId,
    pub available: bool,
    pub asking_food: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Invitation {
    pub id: u64,
    pub proposer: CharacterId,
    pub recipient: CharacterId,
    pub kind: InvitationKind,
    pub place: PlaceId,
    pub cause: u64,
}

/// The complete policy boundary. No foreign inventory, motives, hunger or archive.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnedView {
    pub tick: u64,
    pub remaining: u64,
    pub own: Character,
    pub neighbors: Vec<PlaceId>,
    pub people: Vec<PublicPerson>,
    pub incoming: Vec<Invitation>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Candidate {
    pub action: Action,
    pub score: i32,
    pub terms: BTreeMap<String, i32>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Choice {
    pub candidates: Vec<Candidate>,
    pub selected: Action,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Work {
    Eat,
    Gather,
    Travel {
        to: PlaceId,
    },
    Send {
        to: CharacterId,
        kind: InvitationKind,
    },
    Transfer {
        donor: CharacterId,
        receiver: CharacterId,
    },
    JointGather,
    Reply,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Commitment {
    pub id: u64,
    pub participants: Vec<CharacterId>,
    pub place: PlaceId,
    pub start: u64,
    pub due: u64,
    pub cause: u64,
    pub work: Work,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum RecordKind {
    Observation {
        belief: SiteBelief,
    },
    Decision {
        view: Box<OwnedView>,
        choice: Choice,
        controller: String,
    },
    Started {
        commitment: Commitment,
    },
    Finished {
        commitment: u64,
        outcome: String,
        units: u32,
    },
    Refused {
        invitation: u64,
        reason: String,
    },
    Rejected {
        action: Action,
        reason: String,
    },
    Pressure {
        before: i32,
        after: i32,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub id: u64,
    pub tick: u64,
    pub actor: Option<CharacterId>,
    pub cause: Option<u64>,
    pub kind: RecordKind,
}

#[derive(Resource, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub config: Config,
    pub tick: u64,
    pub random: Random,
    pub places: BTreeMap<PlaceId, Place>,
    pub initial_food: u64,
    pub consumed: u64,
    pub next_commitment: u64,
    pub commitments: BTreeMap<u64, Commitment>,
    pub history: Vec<Record>,
}
impl State {
    pub fn record(
        &mut self,
        actor: Option<CharacterId>,
        cause: Option<u64>,
        kind: RecordKind,
    ) -> u64 {
        let id = self.history.len() as u64;
        self.history.push(Record {
            id,
            tick: self.tick,
            actor,
            cause,
            kind,
        });
        id
    }
    pub fn busy(&self, actor: CharacterId) -> bool {
        self.commitments
            .values()
            .any(|c| c.participants.contains(&actor))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checkpoint {
    pub state: State,
    pub characters: Vec<Character>,
}
