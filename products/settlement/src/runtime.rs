use crate::{
    model::*,
    policy::{Controller, Utility},
};
use bevy_ecs::{prelude::*, schedule::SingleThreadedExecutor};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Resource)]
struct Index(BTreeMap<CharacterId, Entity>);

pub struct Settlement {
    world: World,
    schedule: Schedule,
}
impl Settlement {
    pub fn new(config: Config) -> Result<Self, String> {
        let ids = (0..config.population)
            .map(|i| CharacterId(1001 + i as u64 * 17))
            .collect();
        Self::with_ids(config, ids)
    }
    pub fn with_ids(config: Config, mut ids: Vec<CharacterId>) -> Result<Self, String> {
        config.validate()?;
        ids.sort();
        if ids.len() != config.population || ids.windows(2).any(|w| w[0] == w[1]) {
            return Err("unique stable IDs must match population".into());
        }
        let mut random = Random { state: config.seed };
        let characters: Vec<_> = ids
            .into_iter()
            .enumerate()
            .map(|(i, id)| Character {
                id,
                name: format!(
                    "{}-{}",
                    [
                        "Aster", "Bryn", "Cedar", "Dara", "Ember", "Fern", "Galen", "Hazel"
                    ][i % 8],
                    id.0
                ),
                place: (i % 3) as u32,
                food: random.draw(3) as u32,
                hunger: 25 + random.draw(56) as i32,
                generosity: 25 + random.draw(71) as i32,
                caution: 10 + random.draw(71) as i32,
                reserve_goal: 2 + random.draw(2) as u32,
                beliefs: Default::default(),
                relationships: Default::default(),
                memories: Default::default(),
                signal: None,
            })
            .collect();
        let n = config.population as u32;
        let places = [
            Place {
                id: 0,
                name: "Lantern Square".into(),
                neighbors: vec![1],
                food: 0,
            },
            Place {
                id: 1,
                name: "Bramble Grove".into(),
                neighbors: vec![0, 2],
                food: n * 7,
            },
            Place {
                id: 2,
                name: "Old Orchard".into(),
                neighbors: vec![1],
                food: n * 4,
            },
        ]
        .into_iter()
        .map(|p| (p.id, p))
        .collect();
        let initial_food =
            u64::from(n * 11) + characters.iter().map(|a| u64::from(a.food)).sum::<u64>();
        Self::restore(Checkpoint {
            state: State {
                config,
                tick: 0,
                random,
                places,
                initial_food,
                consumed: 0,
                next_commitment: 0,
                commitments: BTreeMap::new(),
                history: vec![],
            },
            characters,
        })
    }
    pub fn restore(saved: Checkpoint) -> Result<Self, String> {
        validate(&saved)?;
        let mut world = World::new();
        let mut index = BTreeMap::new();
        for actor in saved.characters {
            index.insert(actor.id, world.spawn(actor).id());
        }
        world.insert_resource(Index(index));
        world.insert_resource(saved.state);
        let mut schedule = Schedule::default();
        schedule.set_executor(SingleThreadedExecutor::new());
        schedule.add_systems(step_system);
        Ok(Self { world, schedule })
    }
    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint {
            state: self.state().clone(),
            characters: self.characters().into_values().collect(),
        }
    }
    pub fn state(&self) -> &State {
        self.world.resource::<State>()
    }
    pub fn tick(&self) -> u64 {
        self.state().tick
    }
    pub fn complete(&self) -> bool {
        self.tick() == self.state().config.horizon
    }
    pub fn characters(&self) -> BTreeMap<CharacterId, Character> {
        self.world
            .resource::<Index>()
            .0
            .iter()
            .map(|(id, entity)| (*id, self.world.get::<Character>(*entity).unwrap().clone()))
            .collect()
    }
    pub fn owned_view(&self, id: CharacterId) -> Result<OwnedView, String> {
        let actors = self.characters();
        if !actors.contains_key(&id) {
            return Err("unknown character".into());
        }
        Ok(view(self.state(), &actors, id, &[]))
    }
    pub fn advance(&mut self, ticks: u64) -> Result<u64, String> {
        let steps = ticks.min(self.state().config.horizon - self.tick());
        for _ in 0..steps {
            self.schedule.run(&mut self.world);
        }
        live_invariants(self.state(), &self.characters())?;
        Ok(steps)
    }
}

fn traveling(state: &State, id: CharacterId) -> bool {
    state
        .commitments
        .values()
        .any(|c| c.participants.contains(&id) && matches!(c.work, Work::Travel { .. }))
}
fn view(
    state: &State,
    actors: &BTreeMap<CharacterId, Character>,
    id: CharacterId,
    offers: &[Invitation],
) -> OwnedView {
    let own = actors[&id].clone();
    let people = if traveling(state, id) {
        vec![]
    } else {
        actors
            .values()
            .filter(|p| p.id != id && p.place == own.place && !traveling(state, p.id))
            .map(|p| PublicPerson {
                id: p.id,
                available: !state.busy(p.id),
                asking_food: p
                    .signal
                    .as_ref()
                    .is_some_and(|s| s.asking_food && state.tick.saturating_sub(s.tick) <= 12),
            })
            .collect()
    };
    OwnedView {
        tick: state.tick,
        remaining: state.config.horizon - state.tick,
        neighbors: state.places[&own.place].neighbors.clone(),
        own,
        people,
        incoming: offers
            .iter()
            .filter(|i| i.recipient == id)
            .cloned()
            .collect(),
    }
}
fn observe(
    state: &mut State,
    actors: &mut BTreeMap<CharacterId, Character>,
    id: CharacterId,
    cause: Option<u64>,
) {
    if traveling(state, id) {
        return;
    }
    let actor = actors.get_mut(&id).unwrap();
    let belief = SiteBelief {
        place: actor.place,
        stock: state.places[&actor.place].food,
        observed_at: state.tick,
        receipt: state.history.len() as u64,
    };
    state.record(
        Some(id),
        cause,
        RecordKind::Observation {
            belief: belief.clone(),
        },
    );
    actor.observe(belief);
}
fn remember(
    state: &State,
    actors: &mut BTreeMap<CharacterId, Character>,
    id: CharacterId,
    other: Option<CharacterId>,
    receipt: u64,
    kind: &str,
    valence: i32,
) {
    actors.get_mut(&id).unwrap().remember(Episode {
        receipt,
        tick: state.tick,
        other,
        kind: kind.into(),
        valence,
    });
}
fn finish(
    state: &mut State,
    actors: &mut BTreeMap<CharacterId, Character>,
    job: Commitment,
) -> Option<Invitation> {
    let owner = job.participants[0];
    let mut units = 0;
    let outcome = match job.work {
        Work::Eat => {
            let a = actors.get_mut(&owner).unwrap();
            if a.food > 0 {
                a.food -= 1;
                a.hunger = (a.hunger - 35).max(0);
                state.consumed += 1;
                units = 1;
                "ate"
            } else {
                "no_food"
            }
        }
        Work::Gather | Work::JointGather => {
            let required = job.participants.len() as u32;
            let place = state.places.get_mut(&job.place).unwrap();
            if place.food >= required {
                place.food -= required;
                for id in &job.participants {
                    actors.get_mut(id).unwrap().food += 1;
                }
                units = required;
                if required == 2 {
                    "gathered_together"
                } else {
                    "gathered"
                }
            } else {
                "depleted"
            }
        }
        Work::Travel { to } => {
            actors.get_mut(&owner).unwrap().place = to;
            "arrived"
        }
        Work::Send { .. } => "invitation_delivered",
        Work::Transfer { donor, receiver } => {
            if actors[&donor].food > 0 {
                actors.get_mut(&donor).unwrap().food -= 1;
                actors.get_mut(&receiver).unwrap().food += 1;
                units = 1;
                "shared_food"
            } else {
                "transfer_failed"
            }
        }
        Work::Reply => "replied",
    };
    let receipt = state.record(
        Some(owner),
        Some(job.cause),
        RecordKind::Finished {
            commitment: job.id,
            outcome: outcome.into(),
            units,
        },
    );
    for id in &job.participants {
        let other = job.participants.iter().find(|p| *p != id).copied();
        let valence = if units > 0 && other.is_some() { 8 } else { 0 };
        remember(state, actors, *id, other, receipt, outcome, valence);
        observe(state, actors, *id, Some(receipt));
    }
    if let Work::Send { to, kind } = job.work {
        return Some(Invitation {
            id: job.id,
            proposer: owner,
            recipient: to,
            kind,
            place: job.place,
            cause: receipt,
        });
    }
    None
}
fn duration(work: &Work) -> u64 {
    match work {
        Work::Gather => 3,
        Work::Travel { .. } | Work::JointGather => 2,
        _ => 1,
    }
}
fn reserve(
    state: &mut State,
    mut participants: Vec<CharacterId>,
    place: PlaceId,
    cause: u64,
    work: Work,
) {
    // Preserve sender/donor ownership in work fields; canonical participants support stable snapshots.
    participants.sort();
    let id = state.next_commitment;
    state.next_commitment += 1;
    let job = Commitment {
        id,
        due: state.tick + duration(&work),
        start: state.tick,
        participants,
        place,
        cause,
        work,
    };
    state.record(
        job.participants.first().copied(),
        Some(cause),
        RecordKind::Started {
            commitment: job.clone(),
        },
    );
    state.commitments.insert(id, job);
}

/// The authority check is shared by every intent source, independently of policy scores.
pub fn validate_intent(
    state: &State,
    actors: &BTreeMap<CharacterId, Character>,
    actor: CharacterId,
    action: &Action,
    offers: &[Invitation],
) -> Result<(), String> {
    let own = actors.get(&actor).ok_or("unknown actor")?;
    if state.busy(actor) || state.tick >= state.config.horizon {
        return Err("actor unavailable or endpoint reached".into());
    }
    let remaining = state.config.horizon - state.tick;
    match action {
        Action::Wait => (),
        Action::Eat if own.food > 0 && remaining >= 1 => (),
        Action::Gather if own.place != 0 && remaining >= 3 && own.food < 1000 => (),
        Action::Travel { to }
            if state.places[&own.place].neighbors.contains(to) && remaining >= 2 => {}
        Action::Invite { to, kind } => {
            let recipient = actors.get(to).ok_or("unknown recipient")?;
            if *to == actor || recipient.place != own.place || state.busy(*to) || remaining < 3 {
                return Err(
                    "invitation needs a distinct available co-present recipient and time".into(),
                );
            }
            if *kind == InvitationKind::OfferFood && own.food == 0 {
                return Err("no food to offer".into());
            }
        }
        Action::Accept { invitation } | Action::Decline { invitation } => {
            let offer = offers
                .iter()
                .find(|i| i.id == *invitation && i.recipient == actor)
                .ok_or("no addressed live invitation")?;
            if remaining < 1 {
                return Err("no reply time".into());
            }
            if matches!(action, Action::Accept { .. }) {
                let proposer = actors.get(&offer.proposer).ok_or("unknown proposer")?;
                if proposer.place != own.place
                    || offer.place != own.place
                    || state.busy(proposer.id)
                {
                    return Err("proposer no longer available here".into());
                }
                let enough = match offer.kind {
                    InvitationKind::RequestFood => own.food > 0 && proposer.food < 1000,
                    InvitationKind::OfferFood => proposer.food > 0 && own.food < 1000,
                    InvitationKind::GatherTogether => {
                        remaining >= 2 && own.place != 0 && own.food < 1000 && proposer.food < 1000
                    }
                };
                if !enough {
                    return Err("cannot execute accepted invitation".into());
                }
            }
        }
        _ => return Err("illegal action or insufficient time/resources".into()),
    }
    Ok(())
}
fn refuse(
    state: &mut State,
    actors: &mut BTreeMap<CharacterId, Character>,
    offer: &Invitation,
    cause: Option<u64>,
    reason: &str,
    explicit: bool,
) {
    let receipt = state.record(
        Some(offer.recipient),
        cause.or(Some(offer.cause)),
        RecordKind::Refused {
            invitation: offer.id,
            reason: reason.into(),
        },
    );
    remember(
        state,
        actors,
        offer.proposer,
        Some(offer.recipient),
        receipt,
        reason,
        if explicit { -4 } else { 0 },
    );
    if explicit {
        remember(
            state,
            actors,
            offer.recipient,
            Some(offer.proposer),
            receipt,
            "declined",
            0,
        );
    }
}
fn admit(
    state: &mut State,
    actors: &mut BTreeMap<CharacterId, Character>,
    actor: CharacterId,
    action: Action,
    cause: u64,
    offers: &[Invitation],
) -> bool {
    if let Err(reason) = validate_intent(state, actors, actor, &action, offers) {
        state.record(
            Some(actor),
            Some(cause),
            RecordKind::Rejected { action, reason },
        );
        return false;
    }
    let place = actors[&actor].place;
    match action {
        Action::Wait => (),
        Action::Eat => reserve(state, vec![actor], place, cause, Work::Eat),
        Action::Gather => reserve(state, vec![actor], place, cause, Work::Gather),
        Action::Travel { to } => reserve(state, vec![actor], place, cause, Work::Travel { to }),
        Action::Invite { to, kind } => {
            if kind == InvitationKind::RequestFood {
                actors.get_mut(&actor).unwrap().signal = Some(Signal {
                    tick: state.tick,
                    asking_food: true,
                });
            }
            reserve(state, vec![actor], place, cause, Work::Send { to, kind });
        }
        Action::Accept { invitation } => {
            let offer = offers.iter().find(|i| i.id == invitation).unwrap();
            let work = match offer.kind {
                InvitationKind::RequestFood => Work::Transfer {
                    donor: actor,
                    receiver: offer.proposer,
                },
                InvitationKind::OfferFood => Work::Transfer {
                    donor: offer.proposer,
                    receiver: actor,
                },
                InvitationKind::GatherTogether => Work::JointGather,
            };
            reserve(state, vec![actor, offer.proposer], place, cause, work);
        }
        Action::Decline { invitation } => {
            let offer = offers.iter().find(|i| i.id == invitation).unwrap();
            refuse(state, actors, offer, Some(cause), "declined", true);
            reserve(state, vec![actor], place, cause, Work::Reply);
        }
    }
    true
}
fn decisions(
    state: &mut State,
    actors: &BTreeMap<CharacterId, Character>,
    roster: &[CharacterId],
    offers: &[Invitation],
    replies: bool,
) -> Vec<(CharacterId, Action, u64)> {
    let mut out = vec![];
    for id in roster {
        if state.busy(*id) {
            continue;
        }
        let input = view(state, actors, *id, offers);
        if replies != !input.incoming.is_empty() {
            continue;
        }
        let choice = Utility.decide(&input);
        let selected = choice.selected.clone();
        let cause = state.record(
            Some(*id),
            None,
            RecordKind::Decision {
                view: Box::new(input),
                choice,
                controller: Utility.identity().into(),
            },
        );
        out.push((*id, selected, cause));
    }
    out
}
fn step_system(world: &mut World) {
    let mut actors: BTreeMap<_, _> = world
        .resource::<Index>()
        .0
        .iter()
        .map(|(id, e)| (*id, world.get::<Character>(*e).unwrap().clone()))
        .collect();
    world.resource_scope(|_, mut state: Mut<State>| {
        if state.tick >= state.config.horizon {
            return;
        }
        state.tick += 1;
        let now = state.tick;
        let due: Vec<_> = state
            .commitments
            .values()
            .filter(|c| c.due == now)
            .map(|c| c.id)
            .collect();
        let mut offers = vec![];
        for id in due {
            let job = state.commitments.remove(&id).unwrap();
            if let Some(offer) = finish(&mut state, &mut actors, job) {
                offers.push(offer);
            }
        }
        for a in actors.values_mut() {
            if now.is_multiple_of(4) {
                let before = a.hunger;
                a.hunger = (a.hunger + 3).min(100);
                state.record(
                    Some(a.id),
                    None,
                    RecordKind::Pressure {
                        before,
                        after: a.hunger,
                    },
                );
            }
        }
        let mut roster: Vec<_> = actors.keys().copied().collect();
        let rotation = state.random.draw(roster.len() as u64) as usize;
        roster.rotate_left(rotation);
        for id in &roster {
            if now == 1 || now.is_multiple_of(8) {
                observe(&mut state, &mut actors, *id, None);
            }
        }
        if now < state.config.horizon {
            let mut handled = BTreeSet::new();
            for (actor, action, cause) in decisions(&mut state, &actors, &roster, &offers, true) {
                let invitation = match action {
                    Action::Accept { invitation } | Action::Decline { invitation } => {
                        Some(invitation)
                    }
                    _ => None,
                };
                if admit(&mut state, &mut actors, actor, action, cause, &offers)
                    && let Some(id) = invitation
                {
                    handled.insert(id);
                }
            }
            for offer in offers.iter().filter(|i| !handled.contains(&i.id)) {
                refuse(
                    &mut state,
                    &mut actors,
                    offer,
                    None,
                    "unavailable_or_other_priority",
                    false,
                );
            }
            for (actor, action, cause) in decisions(&mut state, &actors, &roster, &[], false) {
                admit(&mut state, &mut actors, actor, action, cause, &[]);
            }
        } else {
            for offer in &offers {
                refuse(&mut state, &mut actors, offer, None, "endpoint", false);
            }
        }
        debug_assert!(live_invariants(&state, &actors).is_ok());
    });
    let index = &world.resource::<Index>().0;
    let updates: Vec<_> = actors.into_iter().map(|(id, a)| (index[&id], a)).collect();
    for (entity, actor) in updates {
        *world.get_mut::<Character>(entity).unwrap() = actor;
    }
}

pub fn validate(saved: &Checkpoint) -> Result<(), String> {
    let state = &saved.state;
    state.config.validate()?;
    let actors: BTreeMap<_, _> = saved.characters.iter().map(|a| (a.id, a.clone())).collect();
    if actors.len() != saved.characters.len() || actors.len() != state.config.population {
        return Err("invalid/duplicate character roster".into());
    }
    live_invariants(state, &actors)?;
    let record_ok = |id: u64| id < state.history.len() as u64;
    for a in actors.values() {
        if a.beliefs.iter().any(|b| !record_ok(b.receipt) || b.observed_at > state.tick || !state.places.contains_key(&b.place)
            || !matches!(&state.history[b.receipt as usize].kind, RecordKind::Observation { belief } if belief == b)
            || state.history[b.receipt as usize].actor != Some(a.id))
            || a.relationships.iter().any(|r| !actors.contains_key(&r.other) || r.other == a.id || !(-100..=100).contains(&r.trust) || !record_ok(r.receipt))
            || a.memories.iter().any(|m| !record_ok(m.receipt) || m.tick > state.tick || m.other.is_some_and(|id| !actors.contains_key(&id)))
            || a.signal.as_ref().is_some_and(|s| s.tick > state.tick)
        { return Err("invalid retained cognition/receipt".into()); }
        if a.beliefs
            .iter()
            .map(|b| b.place)
            .collect::<BTreeSet<_>>()
            .len()
            != a.beliefs.len()
            || a.relationships
                .iter()
                .map(|r| r.other)
                .collect::<BTreeSet<_>>()
                .len()
                != a.relationships.len()
        {
            return Err("duplicate retained cells".into());
        }
    }
    let mut open = BTreeMap::<u64, Commitment>::new();
    let mut next_commitment = 0;
    let mut recorded_consumption = 0;
    for (i, r) in state.history.iter().enumerate() {
        if r.id != i as u64
            || r.tick > state.tick
            || (i > 0 && r.tick < state.history[i - 1].tick)
            || r.cause.is_some_and(|cause| cause >= r.id)
            || r.actor.is_some_and(|id| !actors.contains_key(&id))
        {
            return Err("invalid history chronology/reference".into());
        }
        if let RecordKind::Decision {
            view,
            choice,
            controller,
        } = &r.kind
            && (r.actor != Some(view.own.id)
                || view.tick != r.tick
                || controller != Utility.identity()
                || *choice != Utility.decide(view))
        {
            return Err("decision does not reconstruct from its local input".into());
        }
        match &r.kind {
            RecordKind::Started { commitment } => {
                if commitment.id != next_commitment
                    || commitment.start != r.tick
                    || r.cause != Some(commitment.cause)
                    || !matches!(
                        state
                            .history
                            .get(commitment.cause as usize)
                            .map(|r| &r.kind),
                        Some(RecordKind::Decision { .. })
                    )
                    || open.values().any(|c| {
                        c.participants
                            .iter()
                            .any(|id| commitment.participants.contains(id))
                    })
                {
                    return Err("invalid commitment chronology or overlapping work".into());
                }
                open.insert(commitment.id, commitment.clone());
                next_commitment += 1;
            }
            RecordKind::Finished {
                commitment, units, ..
            } => {
                let job = open
                    .remove(commitment)
                    .ok_or("completion without live commitment")?;
                if job.due != r.tick || r.cause != Some(job.cause) {
                    return Err("invalid completion boundary/cause".into());
                }
                if matches!(job.work, Work::Eat) {
                    recorded_consumption += u64::from(*units);
                }
            }
            _ => (),
        }
    }
    if open != state.commitments
        || next_commitment != state.next_commitment
        || recorded_consumption != state.consumed
    {
        return Err(
            "checkpoint work/counter/consumption does not match its executed prefix".into(),
        );
    }
    Ok(())
}
fn live_invariants(state: &State, actors: &BTreeMap<CharacterId, Character>) -> Result<(), String> {
    if state.tick > state.config.horizon
        || state.places.len() != 3
        || (0..3).any(|id| !state.places.contains_key(&id))
    {
        return Err("invalid clock or locations".into());
    }
    for (id, p) in &state.places {
        if p.id != *id
            || p.neighbors.iter().any(|n| {
                *n == *id
                    || !state.places.contains_key(n)
                    || !state.places[n].neighbors.contains(id)
            })
        {
            return Err("invalid public topology".into());
        }
    }
    let physical = state
        .places
        .values()
        .map(|p| u64::from(p.food))
        .sum::<u64>()
        + actors.values().map(|a| u64::from(a.food)).sum::<u64>();
    let total = physical
        .checked_add(state.consumed)
        .ok_or("food accounting overflow")?;
    if total != state.initial_food {
        return Err("food conservation failed".into());
    }
    for a in actors.values() {
        if !state.places.contains_key(&a.place)
            || a.food > 1000
            || !(0..=100).contains(&a.hunger)
            || !(0..=100).contains(&a.generosity)
            || !(0..=100).contains(&a.caution)
            || !(1..=4).contains(&a.reserve_goal)
            || a.memories.len() > MEMORY_CAP
            || a.beliefs.len() > BELIEF_CAP
            || a.relationships.len() > RELATIONSHIP_CAP
        {
            return Err("invalid character state/capacity".into());
        }
    }
    let mut busy = BTreeSet::new();
    for (id, c) in &state.commitments {
        if *id != c.id
            || *id >= state.next_commitment
            || c.start >= c.due
            || c.start > state.tick
            || c.due <= state.tick
            || c.due > state.config.horizon
            || c.due - c.start != duration(&c.work)
            || c.cause >= state.history.len() as u64
            || !state.places.contains_key(&c.place)
            || c.participants.is_empty()
            || c.participants.len() > 2
            || c.participants.iter().any(|id| {
                !actors.contains_key(id) || !busy.insert(*id) || actors[id].place != c.place
            })
        {
            return Err("invalid/overlapping commitment".into());
        }
        let pair = matches!(c.work, Work::JointGather | Work::Transfer { .. });
        if c.participants.len() != if pair { 2 } else { 1 } {
            return Err("invalid work participants".into());
        }
        match c.work {
            Work::Transfer { donor, receiver }
                if donor == receiver
                    || !c.participants.contains(&donor)
                    || !c.participants.contains(&receiver) =>
            {
                return Err("invalid transfer".into());
            }
            Work::Travel { to } if !state.places[&c.place].neighbors.contains(&to) => {
                return Err("invalid travel".into());
            }
            Work::Send { to, .. } if !actors.contains_key(&to) || c.participants.contains(&to) => {
                return Err("invalid recipient".into());
            }
            _ => (),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
