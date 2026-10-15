//! Isolated018 recurring physical time and voluntary activities.
use super::*;
use crate::{contact_learning, encounters as e, inquiry as q, opportunities as o, recurring as r};

impl Simulation {
    pub fn recurring_open(
        &mut self,
        mode: r::Mode,
        order: e::Order,
        budget: u64,
        interval: u64,
    ) -> Result<(), String> {
        self.require_idle()?;
        self.enable_encounters()?;
        if budget == 0
            || budget > 128
            || self
                .world
                .get_resource::<r::Environment>()
                .is_some_and(|p| self.activity_tick() < p.end)
        {
            return Err("invalid018 period".into());
        }
        let settings = r::Settings {
            interval,
            increment: mode.increment(),
        };
        if !settings.valid() || self.agents().iter().any(|a| !(0..=100).contains(&a.hunger)) {
            return Err("invalid018 physical settings".into());
        }
        if let Some(clock) = self.world.get_resource::<r::Clock>() {
            if clock.settings != settings || self.world.resource::<r::Environment>().mode != mode {
                return Err("018 treatment fixed for trial".into());
            }
        } else {
            let epoch = self.activity_tick();
            self.world.insert_resource(r::Clock {
                epoch,
                last: epoch,
                settings,
            });
            self.world.insert_resource(r::History::default());
        }
        let wave = self
            .world
            .get_resource::<r::Environment>()
            .map_or(0, |p| p.wave);
        self.world.insert_resource(r::Environment {
            end: self.activity_tick() + budget,
            mode,
            order,
            wave,
            active: None,
            choose: r::policy,
        });
        Ok(())
    }
    pub fn set_recurring_policy(
        &mut self,
        choose: fn(&e::Input, r::Mode) -> o::Decision,
    ) -> Result<(), String> {
        self.require_idle()?;
        self.world
            .get_resource_mut::<r::Environment>()
            .ok_or("open018 first")?
            .choose = choose;
        Ok(())
    }
    pub fn recurring_point(&self) -> o::Point {
        o::Point {
            state: self.encounter_state(),
            scenes: self.world.resource::<Runtime>().scenes.len(),
        }
    }
    fn recurring_capture(&self, points: &mut Vec<o::Point>) -> usize {
        let p = self.recurring_point();
        if points.last() != Some(&p) {
            points.push(p);
        }
        points.len() - 1
    }
    fn recurring_people(&self) -> Vec<e::Public> {
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
    fn recurring_consume(&mut self, owner: u32) -> Consumption {
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
    fn recurring_complete(&mut self, commit: &o::Commitment) {
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
    pub fn recurring_step(&mut self, points: &mut Vec<o::Point>) -> Result<r::Step, String> {
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
            env.choose,
        );
        let now = self.activity_tick();
        if now >= end {
            return Err("018 horizon closed".into());
        }
        let before = self.recurring_capture(points);
        if active.is_none() {
            self.encounter_prepare()?;
        }
        let observed = self.recurring_capture(points);
        let people = self.recurring_people();
        let mut decisions = vec![];
        for owner in 0..3 {
            if !people[owner as usize].available {
                decisions.push(None);
                continue;
            }
            let input = self.encounter_input(owner, &people, end)?;
            let d = choose(&input, mode);
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
            decisions.push(Some(d));
        }
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
            self.schedule.run(&mut self.world);
            for owner in eating {
                step.consumption.push(self.recurring_consume(owner));
            }
            if self.world.resource::<Runtime>().active.is_empty() {
                self.recurring_complete(&commit);
                step.finished = Some(commit);
            }
        } else {
            self.world.resource_mut::<r::Environment>().wave += 1;
            let outgoing = step
                .decisions
                .iter()
                .map(|d| d.as_ref().unwrap().outgoing())
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
                self.world.resource_mut::<Runtime>().tick = now + 1;
                self.sync_recurring();
                for owner in eating {
                    step.consumption.push(self.recurring_consume(owner));
                }
            } else {
                if now + 2 > end {
                    return Err("unaffordable018 handshake".into());
                }
                self.world.resource_mut::<Runtime>().tick += 1;
                self.sync_recurring();
                for owner in eating {
                    step.consumption.push(self.recurring_consume(owner));
                }
                step.sent = Some(self.recurring_capture(points));
                let people = self.recurring_people();
                for owner in 0..3 {
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
                self.world.resource_mut::<Runtime>().tick += 1;
                self.sync_recurring();
                step.admitted = Some(self.recurring_capture(points));
                step.allocations = e::arbitrate(
                    self.activity_tick(),
                    order,
                    wave,
                    &self.recurring_people(),
                    &invitations,
                    &step.responses,
                );
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
                                    self.schedule.run(&mut self.world);
                                }
                                self.recurring_complete(&commit);
                                step.finished = Some(commit);
                            }
                        }
                        e::Kind::Inquiry => {
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
        step.after = self.recurring_capture(points);
        assert!(self.activity_tick() > now && self.activity_tick() <= end);
        Ok(r::Step {
            need_before,
            activity: step,
            need_after: self.world.resource::<r::History>().0.len(),
        })
    }
    pub fn recurring_finish(&mut self) -> Result<(), String> {
        self.require_idle()?;
        let end = self
            .world
            .get_resource::<r::Environment>()
            .ok_or("open018 first")?
            .end;
        if self.activity_tick() != end {
            return Err("018 period must finish at horizon".into());
        }
        Ok(())
    }
    pub fn recurring_clock(&self) -> Option<r::Clock> {
        self.world.get_resource::<r::Clock>().copied()
    }
    pub fn recurring_records(&self) -> Vec<r::Record> {
        self.world.resource::<r::History>().0.clone()
    }
    pub(super) fn sync_recurring(&mut self) {
        let tick = self.activity_tick();
        sync(&mut self.world, tick);
    }
}
/// All physical state is live ECS. History is appended, never queried for updates.
pub(super) fn sync(world: &mut World, to: u64) {
    let Some(clock) = world.get_resource::<r::Clock>().copied() else {
        return;
    };
    let pulses = clock.pulses(to);
    if to == clock.last {
        return;
    }
    let entities = std::array::from_fn::<_, 3, _>(|i| world.resource::<Index>().0[&(i as u32)]);
    let before = entities.map(|e| world.get::<Agent>(e).unwrap().hunger);
    let after = before.map(|h| r::pressure(h, pulses, clock.settings.increment));
    for (entity, hunger) in entities.into_iter().zip(after) {
        world.get_mut::<Agent>(entity).unwrap().hunger = hunger;
    }
    world.resource_mut::<r::Clock>().last = to;
    world.resource_mut::<r::History>().0.push(r::Record {
        from: clock.last,
        to,
        pulses,
        before,
        after,
    });
}
