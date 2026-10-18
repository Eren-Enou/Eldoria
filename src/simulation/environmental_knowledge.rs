//! Isolated021 lifecycle, copied from frozen020 to preserve historical sources.
//! Physical scheduling is unchanged; only owned environmental information differs.
use super::*;
use crate::{
    acquisition as a, contact_learning, encounters as e, environmental_knowledge as k,
    inquiry as q, opportunities as o, recurring as r,
};
impl Simulation {
    pub(crate) fn knowledge_install(
        &mut self,
        cache: a::Cache,
        duration: u64,
        policy: a::Policy,
        memory_mode: k::Mode,
        cadence: u64,
    ) {
        assert!(
            cache.id == a::CacheId(0)
                && cache.stock <= cache.capacity
                && [2, 4, 8].contains(&duration)
        );
        assert!(self.world.resource::<Runtime>().busy.is_empty());
        let initial_total =
            self.agents().iter().map(|a| u64::from(a.food)).sum::<u64>() + u64::from(cache.stock);
        let consumed_before = self.world.resource::<Cognition>().consumption.len();
        self.world.insert_resource(a::Environment {
            cache,
            duration,
            policy,
            choose: a::policy,
            pending: vec![],
            records: vec![],
            next_id: 0,
            initial_total,
            consumed_before,
        });
        self.world
            .insert_resource(k::Store::new(memory_mode, self.activity_tick(), cadence));
    }
    pub(crate) fn knowledge_point(&self) -> a::Point {
        self.knowledge_ledger();
        let env = self.world.resource::<a::Environment>();
        a::Point {
            state: self.encounter_state(),
            scenes: self.world.resource::<Runtime>().scenes.len(),
            cache: env.cache.clone(),
            pending: env.pending.clone(),
            acquisitions: env.records.len(),
        }
    }
    pub(crate) fn knowledge_records(&self) -> Vec<a::Record> {
        self.world.resource::<a::Environment>().records.clone()
    }
    fn knowledge_ledger(&self) {
        let env = self.world.resource::<a::Environment>();
        let consumed = self.world.resource::<Cognition>().consumption[env.consumed_before..]
            .iter()
            .map(|c| u64::from(c.consumed))
            .sum::<u64>();
        assert_eq!(
            env.initial_total,
            self.agents().iter().map(|a| u64::from(a.food)).sum::<u64>()
                + u64::from(env.cache.stock)
                + consumed
        );
    }
    fn knowledge_tick(&mut self, order: e::Order) {
        if self.world.resource::<Runtime>().active.is_empty() {
            self.world.resource_mut::<Runtime>().tick += 1;
            self.sync_recurring();
        } else {
            self.schedule.run(&mut self.world);
        }
        let now = self.activity_tick();
        let mut due = self
            .world
            .resource::<a::Environment>()
            .pending
            .iter()
            .filter(|p| p.end <= now)
            .cloned()
            .collect::<Vec<_>>();
        due.sort_by_key(|p| (p.end, a::rank(p.actor, p.wave, order)));
        for attempt in due {
            assert_eq!(attempt.end, now, "020 deadline skipped");
            let env = self.world.resource::<a::Environment>();
            assert_eq!(attempt.cache, env.cache.id);
            let cache_before = env.cache.stock;
            let entity = self.world.resource::<Index>().0[&attempt.actor];
            let actor = self.world.get::<Agent>(entity).unwrap();
            let food_before = actor.food;
            let hunger = actor.hunger;
            let (cache_after, food_after, outcome) = a::grant(cache_before, food_before);
            self.world.get_mut::<Agent>(entity).unwrap().food = food_after;
            let mut env = self.world.resource_mut::<a::Environment>();
            env.cache.stock = cache_after;
            let id = env.records.len() as u64;
            env.records.push(a::Record {
                id,
                rank: a::rank(attempt.actor, attempt.wave, order),
                attempt: attempt.clone(),
                completed_at: now,
                cache_before,
                cache_after,
                food_before,
                food_after,
                hunger,
                outcome,
            });
            env.pending.retain(|p| p.id != attempt.id);
            let duration = self.world.resource::<a::Environment>().duration;
            self.world
                .resource_mut::<k::Store>()
                .result(k::LocalResult {
                    owner: attempt.actor,
                    cache: attempt.cache,
                    at: now,
                    outcome,
                    physical: id,
                    duration,
                });
            assert!(
                self.world
                    .resource_mut::<Runtime>()
                    .busy
                    .remove(&attempt.actor)
            );
        }
        self.knowledge_ledger();
    }
    fn knowledge_capture(&self, points: &mut Vec<a::Point>) -> usize {
        let p = self.knowledge_point();
        if points.last() != Some(&p) {
            points.push(p);
        }
        points.len() - 1
    }
    fn knowledge_people(&self) -> Vec<e::Public> {
        let busy = &self.world.resource::<Runtime>().busy;
        (0..3)
            .map(|id| e::Public {
                id,
                available: !busy.contains(&id),
            })
            .collect()
    }
    // Same explicit consumption/accounting rule as consume(), completing at the
    // current shared tick. Caller has charged one non-overlapping actor tick.
    fn knowledge_consume(&mut self, owner: u32) -> Consumption {
        assert!(!self.world.resource::<Runtime>().busy.contains(&owner));
        let tick = self.activity_tick();
        let entity = self.world.resource::<Index>().0[&owner];
        let mut a = self.world.get_mut::<Agent>(entity).unwrap();
        assert!(a.food > 0 && a.hunger > 0);
        let record = Consumption {
            tick,
            actor: owner,
            food_before: a.food,
            food_after: a.food - 1,
            hunger_before: a.hunger,
            hunger_after: (a.hunger - 40).max(0),
            consumed: 1,
        };
        a.food = record.food_after;
        a.hunger = record.hunger_after;
        self.world
            .resource_mut::<Cognition>()
            .consumption
            .push(record.clone());
        record
    }
    fn knowledge_complete(&mut self, commit: &o::Commitment) {
        assert!(self.world.resource::<Runtime>().active.is_empty());
        let last = self.world.resource::<Runtime>().events.len();
        self.world.resource_mut::<Runtime>().conversation_cursor = last;
        self.world.resource_mut::<Concerns>().scene_cursor =
            self.world.resource::<Runtime>().scenes.len();
        for owner in [commit.invitation.proposer, commit.invitation.recipient] {
            let prior = self
                .world
                .resource::<Concerns>()
                .items
                .get(&owner)
                .cloned()
                .unwrap_or_default();
            for before in prior.into_iter().filter(|c| c.status.active()) {
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
                    "018 actual resource encounter",
                    Some(before),
                    Some(after),
                    None,
                    None,
                );
            }
        }
        for event in commit.first_event..last {
            if self.world.resource::<Runtime>().events[event].outcome == Some(Outcome::Refusal) {
                self.capture_concern(event as u64);
            }
        }
        self.world.resource_mut::<r::Environment>().active = None;
    }
    /// The points table is write-only observer capture, never a policy input.
    pub(crate) fn knowledge_step(
        &mut self,
        points: &mut Vec<a::Point>,
        period: usize,
        ordinal: usize,
    ) -> Result<k::Step, String> {
        let memory_before = self.world.resource::<k::Store>().records.len();
        let need_before = self.world.resource::<r::History>().0.len();
        let env = self
            .world
            .get_resource::<r::Environment>()
            .ok_or("open018 first")?;
        let (end, mode, order, wave, active, choose) = (
            env.end,
            env.mode,
            env.order,
            env.wave,
            env.active.clone(),
            self.world.resource::<k::Store>().choose,
        );
        let now = self.activity_tick();
        if now >= end {
            return Err("018 horizon closed".into());
        }
        let before = self.knowledge_capture(points);
        if active.is_none() && self.knowledge_people().iter().any(|p| p.available) {
            self.encounter_prepare()?;
        }
        let observed = self.knowledge_capture(points);
        let people = self.knowledge_people();
        let mut decisions = vec![];
        let mut choices = vec![];
        let completed_before = self.world.resource::<a::Environment>().records.len();
        for owner in 0..3 {
            if !people[owner as usize].available {
                decisions.push(None);
                choices.push(None);
                continue;
            }
            let input = self.encounter_input(owner, &people, end)?;
            let env = self.world.resource::<a::Environment>();
            let duration = env.duration;
            let policy = env.policy;
            let sensed = self
                .world
                .resource::<k::Store>()
                .wants_sense(now)
                .then_some(a::Observation {
                    id: env.cache.id,
                    observed_at: now,
                    stock: env.cache.stock,
                    accessible: env.cache.accessible,
                    duration,
                });
            let retained = self
                .world
                .resource_mut::<k::Store>()
                .prepare(owner, now, sensed, observed);
            let local = k::Input {
                legacy: input.clone(),
                retained,
                duration,
                inquiry_lane_free: self.world.resource::<a::Environment>().pending.is_empty(),
            };
            let actual = choose(&local, policy);
            if actual != k::policy(&local, policy) {
                return Err("invalid021 choice".into());
            }
            let d = actual.base.clone();
            let acquiring = actual.selected == a::Action::Acquire;
            choices.push(Some(actual));
            if d.contact.input != input
                || d.mode != mode.mapped()
                || !d
                    .candidates
                    .iter()
                    .any(|c| c.action == d.selected && c.eligible)
                || (d.selected == o::Action::Consume
                    && (!mode.consumes() || input.own.food == 0 || input.own.hunger <= 0))
            {
                return Err("invalid018 activity".into());
            }
            if d.selected == o::Action::Invite
                && (d.contact.selected == e::Intent::Wait
                    || !d
                        .contact
                        .candidates
                        .iter()
                        .any(|c| c.intent == d.contact.selected && c.eligible))
            {
                return Err("invalid018 invitation".into());
            }
            if d.selected == o::Action::Invite {
                let e::Intent::Invite { recipient, kind } = d.contact.selected else {
                    unreachable!()
                };
                if recipient >= 3 || recipient == owner || !people[recipient as usize].available {
                    return Err("invalid018 participants".into());
                }
                let reservation = match kind {
                    e::Kind::Resource => 8,
                    e::Kind::Inquiry => {
                        let q::Action::Ask {
                            source, strategy, ..
                        } = input.query.decision.selected
                        else {
                            return Err("invalid018 inquiry selection".into());
                        };
                        if source != recipient {
                            return Err("invalid018 inquiry source".into());
                        }
                        u64::from(strategy.cost()) + 5
                    }
                };
                if now.checked_add(reservation).is_none_or(|t| t > end) {
                    return Err("unaffordable018 activity".into());
                }
            }
            decisions.push(if acquiring { None } else { Some(d) });
            self.world
                .resource_mut::<k::Store>()
                .after_decision(owner, now);
        }
        let mut admissions = vec![];
        for (owner, choice) in choices.iter().enumerate() {
            if choice
                .as_ref()
                .is_some_and(|d| d.selected == a::Action::Acquire)
            {
                let observation = choice
                    .as_ref()
                    .unwrap()
                    .input
                    .retained
                    .as_ref()
                    .unwrap()
                    .observation
                    .clone();
                let mut env = self.world.resource_mut::<a::Environment>();
                if observation.id != env.cache.id {
                    return Err("unknown021 cache target".into());
                }
                let attempt = a::Attempt {
                    id: env.next_id,
                    cache: env.cache.id,
                    actor: owner as u32,
                    start: now,
                    end: now + env.duration,
                    wave,
                    selection: [period, ordinal, owner],
                    observation,
                };
                env.next_id += 1;
                admissions.push(attempt.clone());
                env.pending.push(attempt);
                assert!(
                    self.world
                        .resource_mut::<Runtime>()
                        .busy
                        .insert(owner as u32)
                );
            }
        }
        let mut skipped_responses = vec![];
        let mut provisional = vec![];
        let mut atomic_blocked = vec![];
        let mut step = o::Step {
            wave,
            before,
            observed,
            people,
            decisions,
            sent: None,
            responses: vec![],
            allocations: vec![],
            admitted: None,
            query: None,
            active_before: active.clone(),
            active_after: None,
            started: None,
            finished: None,
            consumption: vec![],
            after: 0,
        };
        let eating = step
            .decisions
            .iter()
            .enumerate()
            .filter_map(|(id, d)| {
                d.as_ref()
                    .filter(|d| d.selected == o::Action::Consume)
                    .map(|_| id as u32)
            })
            .collect::<Vec<_>>();
        if let Some(commit) = active {
            // With three people, only the uncommitted third can act independently.
            assert!(
                step.decisions
                    .iter()
                    .flatten()
                    .all(|d| d.selected != o::Action::Invite)
            );
            self.knowledge_tick(order);
            for owner in eating {
                step.consumption.push(self.knowledge_consume(owner));
            }
            if self.world.resource::<Runtime>().active.is_empty() {
                self.knowledge_complete(&commit);
                step.finished = Some(commit);
            }
        } else {
            if step.people.iter().any(|p| p.available) {
                self.world.resource_mut::<r::Environment>().wave += 1;
            }
            let outgoing = step
                .decisions
                .iter()
                .enumerate()
                .map(|(id, d)| {
                    d.as_ref().map(|d| d.outgoing()).unwrap_or_else(|| {
                        let input = choices[id]
                            .as_ref()
                            .map(|d| d.input.legacy.clone())
                            .unwrap_or_else(|| {
                                self.encounter_input(id as u32, &step.people, end).unwrap()
                            });
                        let mut wait = e::policy(&input, e::Mode::History);
                        wait.selected = e::Intent::Wait;
                        wait
                    })
                })
                .collect::<Vec<_>>();
            let invitations = outgoing
                .iter()
                .filter_map(|d| match d.selected {
                    e::Intent::Invite { recipient, kind } => Some(e::Invitation {
                        proposer: d.input.own.id,
                        recipient,
                        kind,
                        expires: now + 2,
                    }),
                    _ => None,
                })
                .collect::<Vec<_>>();
            if invitations.is_empty() {
                self.knowledge_tick(order);
                for owner in eating {
                    step.consumption.push(self.knowledge_consume(owner));
                }
            } else {
                if now + 2 > end {
                    return Err("unaffordable018 handshake".into());
                }
                self.knowledge_tick(order);
                for owner in eating {
                    step.consumption.push(self.knowledge_consume(owner));
                }
                step.sent = Some(self.knowledge_capture(points));
                let people = self.knowledge_people();
                for owner in 0..3 {
                    if !people[owner as usize].available {
                        skipped_responses.push(owner);
                        continue;
                    }
                    let input = e::ResponseInput {
                        own: self.encounter_input(owner, &people, end)?,
                        outgoing: outgoing[owner as usize].clone(),
                        incoming: invitations
                            .iter()
                            .filter(|i| i.recipient == owner)
                            .cloned()
                            .collect(),
                    };
                    step.responses.push(contact_learning::respond(&input));
                }
                self.knowledge_tick(order);
                step.admitted = Some(self.knowledge_capture(points));
                step.allocations = e::arbitrate(
                    self.activity_tick(),
                    order,
                    wave,
                    &self.knowledge_people(),
                    &invitations,
                    &step.responses,
                );
                provisional = step.allocations.clone();
                if !self.world.resource::<a::Environment>().pending.is_empty() {
                    for allocation in &mut step.allocations {
                        if allocation.disposition == e::Disposition::Execute
                            && allocation.invitation.kind == e::Kind::Inquiry
                        {
                            atomic_blocked.push(allocation.invitation.clone());
                            allocation.disposition = e::Disposition::Unavailable;
                        }
                    }
                }
                let selected = step
                    .allocations
                    .iter()
                    .filter(|a| a.disposition == e::Disposition::Execute)
                    .collect::<Vec<_>>();
                assert!(selected.len() <= 1);
                if let Some(allocation) = selected.first() {
                    let invite = &allocation.invitation;
                    match invite.kind {
                        e::Kind::Resource => {
                            if self.activity_tick() + 6 > end {
                                return Err("unaffordable018 resource".into());
                            }
                            let first_event = self.world.resource::<Runtime>().events.len();
                            let scene =
                                self.add_scene([invite.proposer, invite.recipient], 1, 6)?;
                            let commit = o::Commitment {
                                invitation: invite.clone(),
                                scene,
                                start: self.activity_tick(),
                                first_event,
                            };
                            self.world.resource_mut::<r::Environment>().active =
                                Some(commit.clone());
                            step.started = Some(commit.clone());
                            if mode.serial() {
                                while !self.world.resource::<Runtime>().active.is_empty() {
                                    self.knowledge_tick(order);
                                }
                                self.knowledge_complete(&commit);
                                step.finished = Some(commit);
                            }
                        }
                        e::Kind::Inquiry => {
                            assert!(self.world.resource::<a::Environment>().pending.is_empty());
                            let query = &outgoing[invite.proposer as usize].input.query;
                            let q::Action::Ask {
                                source, strategy, ..
                            } = query.decision.selected
                            else {
                                return Err("no018 query".into());
                            };
                            if source != invite.recipient
                                || self.activity_tick() + u64::from(strategy.cost()) + 3 > end
                            {
                                return Err("unaffordable018 inquiry".into());
                            }
                            let sources = query
                                .input
                                .base
                                .sources
                                .iter()
                                .map(|s| s.id)
                                .collect::<Vec<_>>();
                            step.query =
                                Some(self.world.resource::<q::Inquiry>().records.len() as u64);
                            let expected = query.decision.selected;
                            self.provenance_follow(
                                invite.proposer,
                                &sources,
                                self.world.resource::<Runtime>().scenes.len() as u64,
                                None,
                            );
                            assert_eq!(
                                self.world
                                    .resource::<q::Inquiry>()
                                    .records
                                    .last()
                                    .unwrap()
                                    .decision
                                    .selected,
                                expected,
                                "018 consent/preview inquiry identity changed"
                            );
                        }
                    }
                }
            }
        }
        step.active_after = self.world.resource::<r::Environment>().active.clone();
        step.after = self.knowledge_capture(points);
        assert!(self.activity_tick() > now && self.activity_tick() <= end);
        Ok(k::Step {
            choices: choices.into_iter().map(|d| d.map(Into::into)).collect(),
            admissions,
            completed: (completed_before..self.world.resource::<a::Environment>().records.len())
                .map(|id| id as u64)
                .collect(),
            skipped_responses,
            provisional,
            atomic_blocked,
            memory_before,
            memory_after: self.world.resource::<k::Store>().records.len(),
            need_before,
            activity: step,
            need_after: self.world.resource::<r::History>().0.len(),
        })
    }
    pub(crate) fn knowledge_finish(&mut self) -> Vec<Option<k::Cell>> {
        let now = self.activity_tick();
        self.world.resource_mut::<k::Store>().finish(now)
    }
    pub(crate) fn knowledge_history(&self) -> Vec<k::Record> {
        self.world.resource::<k::Store>().records.clone()
    }
}
