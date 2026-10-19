//! Opt-in bounded022 adapter; historical adapters never call this lifecycle.
use super::*;
use crate::{
    acquisition as a, encounters as e, environmental_knowledge as k, experiment::concurrent as x,
    inquiry as q, provenance as p,
};

#[derive(Resource)]
struct State {
    config: x::Config,
    epoch: u64,
    synced: u64,
    end: u64,
    stock: u32,
    total: u64,
    active: Vec<x::Commitment>,
    pending: Vec<x::Pending>,
    transitions: Vec<x::Transition>,
    admissions: Vec<x::Admission>,
    needs: Vec<x::Need>,
    grants: Vec<x::Grant>,
    memories: Vec<k::Record>,
    slots: [Option<k::Cell>; 4],
    receipts: [u64; 4],
    next: u64,
}
fn rank(id: u32, tick: u64, order: e::Order) -> u32 {
    match order {
        e::Order::Ascending => id,
        e::Order::Descending => 3 - id,
        e::Order::Rotating => (id + 4 - (tick % 4) as u32) % 4,
    }
}
impl Simulation {
    fn concurrent_input(
        &self,
        owner: u32,
        people: &[e::Public],
        end: u64,
    ) -> Result<e::Input, String> {
        if people.len() != 4
            || people.iter().enumerate().any(|(i, p)| p.id != i as u32)
            || owner >= 4
            || end < self.activity_tick()
        {
            return Err("invalid015 public context".into());
        }
        let own = self
            .world
            .get::<Agent>(self.world.resource::<Index>().0[&owner])
            .unwrap()
            .clone();
        let talk = self.world.resource::<Intentional>();
        let state = self.world.resource::<q::Inquiry>();
        let concerns = self
            .world
            .resource::<Concerns>()
            .items
            .get(&owner)
            .cloned()
            .unwrap_or_default();
        let concerns = self.concurrent_concern_projection(owner, concerns);
        let sources = people
            .iter()
            .filter(|s| s.id != owner && s.available && !concerns.iter().any(|c| c.target == s.id))
            .map(|s| q::Source {
                id: s.id,
                credibility: talk
                    .credibility
                    .get(&owner)
                    .and_then(|m| m.get(&s.id))
                    .copied()
                    .unwrap_or(0),
                offers: state
                    .offers
                    .get(&owner)
                    .into_iter()
                    .flatten()
                    .filter(|o| o.source == s.id)
                    .cloned()
                    .collect(),
            })
            .collect();
        let base = q::Input {
            owner,
            scene: self.world.resource::<x::LocalContexts>().completed[owner as usize],
            hunger: own.hunger,
            relationship_goal: talk.profiles[&owner].relationship_goal,
            concerns,
            sources,
            cells: state
                .cells
                .get(&owner)
                .into_iter()
                .flatten()
                .cloned()
                .collect(),
            seen: state
                .seen
                .get(&owner)
                .into_iter()
                .flatten()
                .cloned()
                .collect(),
            settings: state.settings[&owner],
        };
        let state = self.world.resource::<crate::audit::RuntimeProvenance>();
        let input = p::Input {
            base,
            knowledge: state
                .knowledge
                .get(&owner)
                .into_iter()
                .flatten()
                .cloned()
                .collect(),
            hints: state
                .hints
                .get(&owner)
                .into_iter()
                .flatten()
                .cloned()
                .collect(),
            settings: state.settings[&owner],
        };
        let (query, _) =
            self.self_evaluation_decide((self.world.resource::<p::Policy>().0)(&input));
        Ok(e::Input {
            own,
            profile: talk.profiles[&owner].clone(),
            now: self.activity_tick(),
            end,
            people: people.to_vec(),
            query,
        })
    }
    pub(in crate::simulation) fn concurrent_concern_projection(
        &self,
        owner: u32,
        mut concerns: Vec<crate::concerns::Concern>,
    ) -> Vec<crate::concerns::Concern> {
        if self.world.contains_resource::<State>() {
            for c in &mut concerns {
                c.created_scene =
                    self.world.resource::<x::LocalContexts>().created[owner as usize][&c.id];
            }
        }
        concerns
    }
    pub(in crate::simulation) fn concurrent_sync(&mut self) {
        if !self.world.contains_resource::<State>() {
            return;
        }
        self.concurrent_sync_to(self.activity_tick());
    }
    fn concurrent_sync_to(&mut self, to: u64) {
        let from = self.world.resource::<State>().synced;
        let epoch = self.world.resource::<State>().epoch;
        assert!(to >= from);
        for tick in (from + 1)..=to {
            if !(tick - epoch).is_multiple_of(4) {
                continue;
            }
            let before = std::array::from_fn(|i| self.agents()[i].hunger);
            for id in 0..4 {
                let entity = self.world.resource::<Index>().0[&id];
                let mut own = self.world.get_mut::<Agent>(entity).unwrap();
                own.hunger = (own.hunger + 5).min(100);
            }
            let after = std::array::from_fn(|i| self.agents()[i].hunger);
            self.world.resource_mut::<State>().needs.push(x::Need {
                tick,
                before,
                after,
            });
        }
        self.world.resource_mut::<State>().synced = to;
    }
    fn concurrent_people(&self) -> Vec<e::Public> {
        let state = self.world.resource::<State>();
        (0..4)
            .map(|id| e::Public {
                id,
                available: !state.active.iter().any(|c| c.participants.contains(&id)),
            })
            .collect()
    }
    fn concurrent_reserve(
        &mut self,
        work: x::Work,
        participants: Vec<u32>,
        deadline: u64,
        source: usize,
    ) -> u64 {
        let now = self.activity_tick();
        assert!(deadline > now && participants.len() <= 2);
        let occupied = self.concurrent_people();
        assert!(
            participants
                .iter()
                .all(|&id| occupied[id as usize].available)
        );
        assert!(participants.len() == 1 || participants[0] != participants[1]);
        let mut state = self.world.resource_mut::<State>();
        assert!(state.active.len() < 4);
        let id = state.next;
        state.next += 1;
        let c = x::Commitment {
            id,
            work,
            participants: participants.clone(),
            start: now,
            deadline,
            source,
        };
        state.active.push(c.clone());
        state.transitions.push(x::Transition {
            tick: now,
            commitment: c.clone(),
            completed: false,
        });
        if !matches!(c.work, x::Work::Resource { .. }) {
            for owner in participants {
                assert!(self.world.resource_mut::<Runtime>().busy.insert(owner));
            }
        }
        id
    }
    fn concurrent_release(&mut self, c: &x::Commitment) {
        let now = self.activity_tick();
        let mut state = self.world.resource_mut::<State>();
        assert_eq!(state.active.iter().filter(|a| a.id == c.id).count(), 1);
        state.active.retain(|a| a.id != c.id);
        state.transitions.push(x::Transition {
            tick: now,
            commitment: c.clone(),
            completed: true,
        });
        if !matches!(c.work, x::Work::Resource { .. }) {
            for owner in &c.participants {
                assert!(self.world.resource_mut::<Runtime>().busy.remove(owner));
            }
        }
    }
    fn concurrent_memory(
        &mut self,
        owner: u32,
        kind: k::Change,
        after: Option<k::Cell>,
        physical: Option<u64>,
    ) {
        let tick = self.activity_tick();
        let mut s = self.world.resource_mut::<State>();
        let before = s.slots[owner as usize].clone();
        let id = s.memories.len() as u64;
        s.memories.push(k::Record {
            id,
            owner,
            tick,
            kind,
            before,
            after: after.clone(),
            physical,
            point: None,
        });
        s.slots[owner as usize] = after;
    }
    fn concurrent_sense(&mut self, owner: u32, kind: k::Change, stock: u32, physical: Option<u64>) {
        let now = self.activity_tick();
        let state = self.world.resource::<State>();
        let receipt = state.receipts[owner as usize];
        let duration = state.config.duration;
        self.world.resource_mut::<State>().receipts[owner as usize] += 1;
        self.concurrent_memory(
            owner,
            kind,
            Some(k::Cell {
                owner,
                receipt,
                observation: a::Observation {
                    id: a::CacheId(0),
                    observed_at: now,
                    stock,
                    accessible: true,
                    duration,
                },
                own_takes: 0,
            }),
            physical,
        );
    }
    fn concurrent_complete_scene(&mut self, scene: u64, participants: &[u32]) {
        for &owner in participants {
            let old = self
                .world
                .resource::<Concerns>()
                .items
                .get(&owner)
                .cloned()
                .unwrap_or_default();
            for before in old.into_iter().filter(|c| c.status.active()) {
                let mut after = before.clone();
                after.age = after.age.saturating_add(1);
                let mut cs = self.world.resource_mut::<Concerns>();
                *cs.items
                    .get_mut(&owner)
                    .unwrap()
                    .iter_mut()
                    .find(|c| c.id == after.id)
                    .unwrap() = after.clone();
                cs.change(
                    "022 actual resource encounter",
                    Some(before),
                    Some(after),
                    None,
                    None,
                );
            }
        }
        let refusals = self
            .world
            .resource::<Runtime>()
            .events
            .iter()
            .filter(|e| e.scene == scene && e.outcome == Some(Outcome::Refusal))
            .map(|e| e.id)
            .collect::<Vec<_>>();
        for id in refusals {
            self.capture_concern(id);
        }
        self.concurrent_context_complete(participants);
        self.world.resource_mut::<Runtime>().conversation_cursor =
            self.world.resource::<Runtime>().events.len();
        self.world.resource_mut::<Concerns>().scene_cursor =
            self.world.resource::<Runtime>().scenes.len();
    }
    fn concurrent_tick(&mut self) {
        let next = self.activity_tick() + 1;
        self.concurrent_sync_to(next);
        if self.world.resource::<Runtime>().active.is_empty() {
            self.world.resource_mut::<Runtime>().tick = next;
        } else {
            self.schedule.run(&mut self.world);
        }
        assert_eq!(self.activity_tick(), next);
        let mut due = self
            .world
            .resource::<State>()
            .active
            .iter()
            .filter(|c| match c.work {
                x::Work::Resource { scene } => self.world.resource::<Runtime>().scenes
                    [scene as usize]
                    .outcome
                    .is_some(),
                _ => c.deadline <= next,
            })
            .cloned()
            .collect::<Vec<_>>();
        let order = self.world.resource::<State>().config.order;
        due.sort_by_key(|c| {
            (
                match c.work {
                    x::Work::Resource { .. } => 0,
                    x::Work::Claim => 1,
                    x::Work::Meal => 2,
                    _ => 3,
                },
                rank(c.participants[0], c.start, order),
                c.id,
            )
        });
        for c in due {
            match c.work {
                x::Work::Resource { scene } => {
                    assert!(next <= c.deadline);
                    self.concurrent_complete_scene(scene, &c.participants);
                }
                x::Work::Claim => {
                    assert_eq!(c.deadline, next);
                    let owner = c.participants[0];
                    let entity = self.world.resource::<Index>().0[&owner];
                    let food_before = self.world.get::<Agent>(entity).unwrap().food;
                    let stock_before = self.world.resource::<State>().stock;
                    let (stock_after, food_after, outcome) = a::grant(stock_before, food_before);
                    self.world.get_mut::<Agent>(entity).unwrap().food = food_after;
                    let mut s = self.world.resource_mut::<State>();
                    s.stock = stock_after;
                    let physical = s.grants.len() as u64;
                    s.grants.push(x::Grant {
                        commitment: c.id,
                        tick: next,
                        owner,
                        stock_before,
                        stock_after,
                        food_before,
                        food_after,
                        outcome,
                    });
                    match outcome {
                        a::Outcome::Granted => {
                            if let Some(mut cell) =
                                self.world.resource::<State>().slots[owner as usize].clone()
                            {
                                cell.own_takes = cell.own_takes.saturating_add(1);
                                self.concurrent_memory(
                                    owner,
                                    k::Change::Taken,
                                    Some(cell),
                                    Some(physical),
                                );
                            }
                        }
                        a::Outcome::Depleted => {
                            self.concurrent_sense(owner, k::Change::Depleted, 0, Some(physical))
                        }
                        a::Outcome::Overflow => {}
                    }
                }
                x::Work::Meal => {
                    assert_eq!(c.deadline, next);
                    let owner = c.participants[0];
                    let entity = self.world.resource::<Index>().0[&owner];
                    let mut a = self.world.get_mut::<Agent>(entity).unwrap();
                    assert!(a.food > 0 && a.hunger > 0);
                    let r = Consumption {
                        tick: next,
                        actor: owner,
                        food_before: a.food,
                        food_after: a.food - 1,
                        hunger_before: a.hunger,
                        hunger_after: (a.hunger - 40).max(0),
                        consumed: 1,
                    };
                    a.food = r.food_after;
                    a.hunger = r.hunger_after;
                    self.world.resource_mut::<Cognition>().consumption.push(r);
                }
                _ => assert_eq!(c.deadline, next),
            }
            self.concurrent_release(&c);
        }
        self.concurrent_ledger();
    }
    fn concurrent_ledger(&self) {
        let s = self.world.resource::<State>();
        assert_eq!(
            s.total,
            self.agents().iter().map(|a| u64::from(a.food)).sum::<u64>()
                + u64::from(s.stock)
                + self
                    .world
                    .resource::<Cognition>()
                    .consumption
                    .iter()
                    .map(|c| u64::from(c.consumed))
                    .sum::<u64>()
        );
        let actors = s
            .active
            .iter()
            .flat_map(|c| c.participants.iter().copied())
            .collect::<Vec<_>>();
        assert_eq!(actors.len(), actors.iter().collect::<BTreeSet<_>>().len());
        assert_eq!(
            actors.into_iter().collect::<BTreeSet<_>>(),
            self.world.resource::<Runtime>().busy
        );
    }
    fn concurrent_boundary(&mut self, index: usize) -> x::Boundary {
        let now = self.activity_tick();
        let end = self.world.resource::<State>().end;
        let people = self.concurrent_people();
        let mut responses = vec![];
        let incoming = self
            .world
            .resource::<State>()
            .pending
            .iter()
            .filter(|p| p.invitation.expires == now + 1)
            .cloned()
            .collect::<Vec<_>>();
        for owner in 0..4 {
            let invites = incoming
                .iter()
                .filter(|p| p.invitation.recipient == owner)
                .map(|p| p.invitation.clone())
                .collect::<Vec<_>>();
            if invites.is_empty() || !people[owner as usize].available {
                continue;
            }
            let own = self.concurrent_input(owner, &people, end).unwrap();
            let mut outgoing = e::policy(&own, e::Mode::History);
            outgoing.selected = self
                .world
                .resource::<State>()
                .pending
                .iter()
                .find(|p| p.invitation.proposer == owner)
                .map_or(e::Intent::Wait, |p| e::Intent::Invite {
                    recipient: p.invitation.recipient,
                    kind: p.invitation.kind,
                });
            // Continue is clamped to zero by the established recipient heuristic.
            let r = crate::contact_learning::respond(&e::ResponseInput {
                own,
                outgoing,
                incoming: invites,
            });
            for pending in &mut self.world.resource_mut::<State>().pending {
                if pending.invitation.recipient == owner && pending.invitation.expires == now + 1 {
                    pending.response = Some(x::ReplyRef {
                        boundary: index,
                        response: responses.len(),
                        selected: r.selected,
                    });
                }
            }
            self.concurrent_reserve(x::Work::Reply, vec![owner], now + 1, index);
            responses.push(r);
        }
        self.concurrent_admit();
        let now = self.activity_tick();
        let people = self.concurrent_people();
        let state = self.world.resource::<State>();
        let mut boundary = x::Boundary {
            tick: now,
            agents: self.agents(),
            stock: state.stock,
            active: state.active.clone(),
            pending: state.pending.clone(),
            choices: vec![],
            responses,
            events: self.events().len(),
            meals: self.cognition().consumption.len(),
            grants: state.grants.len(),
            concerns: self.world.resource::<Concerns>().transitions.len(),
            assessment: self
                .world
                .resource::<crate::assessment::Assessment>()
                .records
                .len(),
            queries: self.world.resource::<q::Inquiry>().records.len(),
        };
        if now >= end {
            return boundary;
        }
        let lane = state.active.is_empty() && state.pending.is_empty();
        let duration = state.config.duration;
        let sense = (now - state.epoch).is_multiple_of(state.config.cadence);
        for owner in 0..4 {
            if !people[owner as usize].available {
                continue;
            }
            if sense {
                let stock = self.world.resource::<State>().stock;
                self.concurrent_sense(owner, k::Change::Sense, stock, None);
            }
            let legacy = self.concurrent_input(owner, &people, end).unwrap();
            let retained = self.world.resource::<State>().slots[owner as usize].clone();
            let mut decision = k::policy(
                &k::Input {
                    legacy,
                    retained,
                    duration,
                    inquiry_lane_free: lane,
                },
                a::Policy::Need,
            );
            if self
                .world
                .resource::<State>()
                .pending
                .iter()
                .any(|p| p.invitation.proposer == owner)
            {
                // A live protocol token cannot be renewed by duplicate invitations.
                let c = decision
                    .candidates
                    .iter_mut()
                    .find(|c| c.action == a::Action::Invite)
                    .unwrap();
                c.eligible = false;
                c.score = -1;
                decision.selected = decision
                    .candidates
                    .iter()
                    .fold(&decision.candidates[0], |a, b| {
                        if b.score > a.score { b } else { a }
                    })
                    .action;
            }
            boundary.choices.push(decision.into());
        }
        for d in &boundary.choices {
            let owner = d.input.legacy.own.id;
            match d.selected {
                a::Action::Wait => {}
                a::Action::Consume => {
                    self.concurrent_reserve(x::Work::Meal, vec![owner], now + 1, index);
                }
                a::Action::Acquire => {
                    self.concurrent_reserve(x::Work::Claim, vec![owner], now + duration, index);
                }
                a::Action::Invite => {
                    let e::Intent::Invite { recipient, kind } = d.contact else {
                        panic!("missing invite")
                    };
                    self.concurrent_reserve(x::Work::Send, vec![owner], now + 1, index);
                    self.world.resource_mut::<State>().pending.push(x::Pending {
                        source: index,
                        invitation: e::Invitation {
                            proposer: owner,
                            recipient,
                            kind,
                            expires: now + 2,
                        },
                        question: d.input.legacy.query.decision.selected,
                        response: None,
                    });
                }
            }
        }
        self.concurrent_ledger();
        boundary
    }
    fn concurrent_admit(&mut self) {
        let now = self.activity_tick();
        let mut due = self
            .world
            .resource::<State>()
            .pending
            .iter()
            .filter(|p| p.invitation.expires <= now)
            .cloned()
            .collect::<Vec<_>>();
        let order = self.world.resource::<State>().config.order;
        due.sort_by_key(|p| rank(p.invitation.proposer, now, order));
        self.world
            .resource_mut::<State>()
            .pending
            .retain(|p| p.invitation.expires > now);
        let people = self.concurrent_people();
        let mut executed: Vec<e::Invitation> = vec![];
        let due_count = due.len();
        for (position, pending) in due.into_iter().enumerate() {
            let i = &pending.invitation;
            let actors = vec![i.proposer, i.recipient];
            let mut reason = if i.expires != now {
                "Expired"
            } else if actors.iter().any(|id| !people[*id as usize].available) {
                "Unavailable"
            } else if pending
                .response
                .as_ref()
                .is_none_or(|r| r.selected != e::Reply::Accept(i.proposer))
            {
                "Declined"
            } else if executed
                .iter()
                .any(|e| e.proposer == i.recipient && e.recipient == i.proposer && e.kind == i.kind)
            {
                "Reciprocal"
            } else if actors
                .iter()
                .any(|id| !self.concurrent_people()[*id as usize].available)
            {
                "Contention"
            } else {
                "Execute"
            };
            let mut commitment = None;
            if reason == "Execute" {
                let state = self.world.resource::<State>();
                if i.kind == e::Kind::Resource
                    && state
                        .active
                        .iter()
                        .filter(|c| matches!(c.work, x::Work::Resource { .. }))
                        .count()
                        >= state.config.capacity
                {
                    reason = "Capacity";
                } else if i.kind == e::Kind::Inquiry
                    && (!state.active.is_empty()
                        || !state.pending.is_empty()
                        || position + 1 < due_count)
                {
                    reason = "AtomicBlocked";
                }
            }
            if reason == "Execute" {
                match i.kind {
                    e::Kind::Resource => {
                        let scene = self.world.resource::<Runtime>().scenes.len() as u64;
                        // Reservation assertion precedes core busy-set insertion.
                        let id = self.concurrent_reserve(
                            x::Work::Resource { scene },
                            actors.clone(),
                            now + 6,
                            pending.source,
                        );
                        self.add_scene([i.proposer, i.recipient], 1, 6).unwrap();
                        commitment = Some(id);
                        executed.push(i.clone());
                    }
                    e::Kind::Inquiry => {
                        let people = self.concurrent_people();
                        let end = self.world.resource::<State>().end;
                        let current = self.concurrent_input(i.proposer, &people, end).unwrap();
                        if current.query.decision.selected != pending.question {
                            reason = "QuestionChanged";
                        } else {
                            let deadline =
                                now + u64::from(match pending.question {
                                    q::Action::Ask { strategy, .. } => strategy.cost(),
                                    _ => 0,
                                }) + 3;
                            let id = self.concurrent_reserve(
                                x::Work::Query,
                                actors,
                                deadline,
                                pending.source,
                            );
                            let sources = current
                                .query
                                .input
                                .base
                                .sources
                                .iter()
                                .map(|s| s.id)
                                .collect::<Vec<_>>();
                            let scene = self.world.resource::<x::LocalContexts>().completed
                                [i.proposer as usize];
                            self.provenance_follow(i.proposer, &sources, scene, None);
                            self.concurrent_sync();
                            let c = self
                                .world
                                .resource::<State>()
                                .active
                                .iter()
                                .find(|c| c.id == id)
                                .unwrap()
                                .clone();
                            assert!(self.activity_tick() <= deadline);
                            self.concurrent_release(&c);
                            commitment = Some(id);
                            executed.push(i.clone());
                        }
                    }
                }
            }
            self.world
                .resource_mut::<State>()
                .admissions
                .push(x::Admission {
                    tick: now,
                    source: pending.source,
                    invitation: i.clone(),
                    reason: reason.into(),
                    commitment,
                });
        }
    }
    pub(crate) fn concurrent_trial(config: x::Config) -> x::Trial {
        assert!([1, 2].contains(&config.capacity) && [2, 4, 8].contains(&config.duration));
        let mut genr = Generator::new(config.seed);
        let mut sim = Self::new((0..4).map(|id| genr.agent(id)).collect()).unwrap();
        sim.set_policy(scarcity_policy);
        let sensor = Sensor {
            reliability: 50,
            effort: 0,
            channel: foresight::Channel::AccurateFixture,
            second: None,
        };
        sim.enable_foresight(config.seed, sensor.clone()).unwrap();
        sim.enable_concerns().unwrap();
        sim.enable_inquiry().unwrap();
        sim.enable_provenance().unwrap();
        sim.enable_assessment(crate::assessment::Method::Grouped)
            .unwrap();
        sim.enable_retention(crate::retention::Method::Fifo)
            .unwrap();
        sim.enable_self_evaluation(crate::self_evaluation::Mode::Contextual)
            .unwrap();
        for id in 0..4 {
            sim.set_communication_profile(
                id,
                Profile {
                    relationship_goal: 100,
                    privacy: 0,
                    ..Default::default()
                },
            )
            .unwrap();
        }
        sim.world.insert_resource(x::LocalContexts::default());
        let case = config.case.as_str();
        let paired = matches!(
            case,
            "paired"
                | "mixed"
                | "durations"
                | "same_end"
                | "consumption"
                | "acquisition"
                | "ongoing"
                | "exhaustion"
        );
        let pairs = if paired {
            vec![[0, 1], [2, 3]]
        } else if matches!(
            case,
            "busy_recipient" | "missed_contact" | "private_helpful" | "private_cautious"
        ) {
            vec![[0, 2]]
        } else {
            vec![]
        };
        for [receiver, donor] in pairs {
            sim.set_circumstances(receiver, 0, 90, 0, 0).unwrap();
            sim.set_circumstances(donor, 4, 0, 100, 0).unwrap();
            let scene = sim.scenes().len() as u64;
            sim.add_scene([receiver, donor], 1, 6).unwrap();
            while !sim.world.resource::<Runtime>().active.is_empty() {
                sim.schedule.run(&mut sim.world);
            }
            sim.concurrent_bootstrap_complete(scene);
        }
        if case.starts_with("inquiry_") {
            sim.set_circumstances(0, 0, 90, 0, 0).unwrap();
            sim.set_circumstances(1, 1, 80, 0, 100).unwrap();
            sim.add_scene([0, 1], 1, 6).unwrap();
            while !sim.world.resource::<Runtime>().active.is_empty() {
                sim.schedule.run(&mut sim.world);
            }
            let refusal = sim.events().last().unwrap().id;
            sim.concurrent_bootstrap_complete(sim.events().last().unwrap().scene);
            sim.inspect_refusal(refusal, &[(2, sensor)]).unwrap();
        }
        let mut food = [2; 4];
        let mut hunger = [20; 4];
        let mut generosity = [60; 4];
        let mut caution = [20; 4];
        let mut stock = 4;
        match case {
            "mixed" => {
                food[3] = 0;
                hunger[2] = 100;
            }
            "durations" => {
                food[2] = 0;
                hunger[2] = 80;
            }
            "consumption" => {
                hunger[2] = 80;
                generosity[2] = 0;
                caution[2] = 100;
            }
            "acquisition" => {
                food[2] = 0;
                food[3] = 0;
                generosity[2] = 0;
                generosity[3] = 0;
                caution[2] = 100;
                caution[3] = 100;
                stock = 1;
            }
            "scarcity" => {
                food = [0; 4];
                hunger = [80; 4];
                generosity = [0; 4];
                caution = [100; 4];
                stock = 1;
            }
            "busy_recipient" | "missed_contact" | "private_helpful" | "private_cautious" => {
                food = [0, 1, 2, 0];
                hunger = [80, 20, 20, 0];
                generosity = [0, 100, 60, 0];
                caution = [100, 0, 20, 100];
                stock = 1;
                if case == "private_cautious" {
                    generosity[1] = 0;
                    caution[1] = 100;
                }
            }
            "inquiry_quiet" | "inquiry_conflict" => {
                food = [0; 4];
                hunger = [0; 4];
                generosity = [0; 4];
                caution = [100; 4];
                stock = 0;
                if case == "inquiry_conflict" {
                    food[1] = 2;
                    food[3] = 2;
                    hunger[1] = 20;
                    hunger[3] = 20;
                    generosity[1] = 60;
                    generosity[3] = 60;
                    caution[1] = 20;
                    caution[3] = 20;
                }
            }
            "null" => {
                food = [0; 4];
                hunger = [0; 4];
                generosity = [0; 4];
                caution = [100; 4];
                stock = 0;
            }
            "exhaustion" => {
                food = [1; 4];
                stock = 1;
            }
            "paired" | "same_end" | "contenders" | "order_bias" | "ongoing" => {}
            _ => panic!("unregistered case"),
        }
        for id in 0..4 {
            let n = id as usize;
            sim.set_circumstances(id, food[n], hunger[n], generosity[n], caution[n])
                .unwrap();
        }
        let start = sim.activity_tick();
        let end = start + config.horizon;
        let initial = sim.agents();
        let bootstrap_events = sim.events().len();
        let initial_contexts = sim.world.resource::<x::LocalContexts>().clone();
        let total = initial.iter().map(|a| u64::from(a.food)).sum::<u64>() + u64::from(stock);
        sim.world.insert_resource(State {
            config: config.clone(),
            epoch: start,
            synced: start,
            end,
            stock,
            total,
            active: vec![],
            pending: vec![],
            transitions: vec![],
            admissions: vec![],
            needs: vec![],
            grants: vec![],
            memories: vec![],
            slots: std::array::from_fn(|_| None),
            receipts: [0; 4],
            next: 0,
        });
        let mut boundaries = vec![];
        while sim.activity_tick() < end {
            let b = sim.concurrent_boundary(boundaries.len());
            boundaries.push(b);
            if sim.activity_tick() < end {
                sim.concurrent_tick();
            }
        }
        assert_eq!(sim.activity_tick(), end);
        assert!(sim.world.resource::<State>().active.is_empty());
        assert!(sim.world.resource::<State>().pending.is_empty());
        sim.concurrent_ledger();
        sim.validate_history_indexes().unwrap();
        let final_state = crate::experiment013::Snapshot {
            base: crate::experiment009::snapshot(&sim),
            learning: sim.self_evaluation().unwrap(),
            mode: crate::experiment013::Mode::Contextual,
        };
        final_state.validate().unwrap();
        let s = sim.world.resource::<State>();
        x::Trial {
            schema: "experiment-022-trial/v1".into(),
            config,
            start,
            end,
            initial,
            initial_stock: stock,
            initial_total: total,
            boundaries,
            transitions: s.transitions.clone(),
            admissions: s.admissions.clone(),
            needs: s.needs.clone(),
            grants: s.grants.clone(),
            memories: s.memories.clone(),
            final_memory: s.slots.to_vec(),
            profiles: (0..4)
                .map(|id| sim.intentional().unwrap().profiles[&id].clone())
                .collect(),
            bootstrap_events,
            initial_contexts,
            final_contexts: sim.world.resource::<x::LocalContexts>().clone(),
            final_agents: sim.agents(),
            final_stock: s.stock,
            final_state,
        }
    }
    fn concurrent_context_complete(&mut self, participants: &[u32]) {
        for &owner in participants {
            let current = self
                .world
                .resource::<Concerns>()
                .items
                .get(&owner)
                .cloned()
                .unwrap_or_default();
            let mut local = self.world.resource_mut::<x::LocalContexts>();
            let n = owner as usize;
            let completed = local.completed[n];
            local.created[n].retain(|id, _| current.iter().any(|c| c.id == *id));
            for c in current {
                local.created[n].entry(c.id).or_insert(completed);
            }
            assert!(local.created[n].len() <= 8);
            local.completed[n] += 1;
        }
    }
    fn concurrent_bootstrap_complete(&mut self, scene: u64) {
        for e in self
            .events()
            .into_iter()
            .filter(|e| e.scene == scene && e.outcome == Some(Outcome::Refusal))
        {
            self.capture_concern(e.id);
        }
        let participants = self.scenes()[scene as usize].participants;
        self.concurrent_context_complete(&participants);
        self.world.resource_mut::<Runtime>().conversation_cursor = self.events().len();
        self.world.resource_mut::<Concerns>().scene_cursor = self.scenes().len();
    }
}
