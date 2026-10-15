//! Isolated017 stepping; no protected resolver or policy changes.
use super::*;
use crate::{contact_learning, encounters as e, inquiry as q, opportunities as o};

impl Simulation {
    pub fn opportunity_open(
        &mut self,
        mode: o::Mode,
        order: e::Order,
        budget: u64,
    ) -> Result<(), String> {
        self.require_idle()?;
        self.enable_encounters()?;
        if budget == 0
            || budget > 128
            || self
                .world
                .get_resource::<o::Environment>()
                .is_some_and(|p| self.activity_tick() < p.end + 3)
        {
            return Err("invalid017 period".into());
        }
        let wave = self
            .world
            .get_resource::<o::Environment>()
            .map_or(0, |p| p.wave);
        self.world.insert_resource(o::Environment {
            end: self.activity_tick() + budget,
            mode,
            order,
            wave,
            active: None,
            choose: o::policy,
        });
        Ok(())
    }
    pub fn set_opportunity_policy(
        &mut self,
        choose: fn(&e::Input, o::Mode) -> o::Decision,
    ) -> Result<(), String> {
        self.require_idle()?;
        self.world
            .get_resource_mut::<o::Environment>()
            .ok_or("open017 first")?
            .choose = choose;
        Ok(())
    }
    pub fn opportunity_point(&self) -> o::Point {
        o::Point {
            state: self.encounter_state(),
            scenes: self.world.resource::<Runtime>().scenes.len(),
        }
    }
    fn opportunity_capture(&self, points: &mut Vec<o::Point>) -> usize {
        let p = self.opportunity_point();
        if points.last() != Some(&p) {
            points.push(p);
        }
        points.len() - 1
    }
    fn opportunity_people(&self) -> Vec<e::Public> {
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
    fn opportunity_consume(&mut self, owner: u32) -> Consumption {
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
    fn opportunity_complete(&mut self, commit: &o::Commitment) {
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
                    "017 actual resource encounter",
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
        self.world.resource_mut::<o::Environment>().active = None;
    }
    /// The points table is write-only observer capture, never a policy input.
    pub fn opportunity_step(&mut self, points: &mut Vec<o::Point>) -> Result<o::Step, String> {
        let env = self
            .world
            .get_resource::<o::Environment>()
            .ok_or("open017 first")?;
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
            return Err("017 horizon closed".into());
        }
        let before = self.opportunity_capture(points);
        if active.is_none() {
            self.encounter_prepare()?;
        }
        let observed = self.opportunity_capture(points);
        let people = self.opportunity_people();
        let mut decisions = vec![];
        for owner in 0..3 {
            if !people[owner as usize].available {
                decisions.push(None);
                continue;
            }
            let input = self.encounter_input(owner, &people, end)?;
            let d = choose(&input, mode);
            if d.contact.input != input
                || d.mode != mode
                || !d
                    .candidates
                    .iter()
                    .any(|c| c.action == d.selected && c.eligible)
                || (d.selected == o::Action::Consume
                    && (!mode.consumes() || input.own.food == 0 || input.own.hunger <= 0))
            {
                return Err("invalid017 activity".into());
            }
            if d.selected == o::Action::Invite
                && (d.contact.selected == e::Intent::Wait
                    || !d
                        .contact
                        .candidates
                        .iter()
                        .any(|c| c.intent == d.contact.selected && c.eligible))
            {
                return Err("invalid017 invitation".into());
            }
            if d.selected == o::Action::Invite {
                let e::Intent::Invite { recipient, kind } = d.contact.selected else {
                    unreachable!()
                };
                if recipient >= 3 || recipient == owner || !people[recipient as usize].available {
                    return Err("invalid017 participants".into());
                }
                let reservation = match kind {
                    e::Kind::Resource => 8,
                    e::Kind::Inquiry => {
                        let q::Action::Ask {
                            source, strategy, ..
                        } = input.query.decision.selected
                        else {
                            return Err("invalid017 inquiry selection".into());
                        };
                        if source != recipient {
                            return Err("invalid017 inquiry source".into());
                        }
                        u64::from(strategy.cost()) + 5
                    }
                };
                if now.checked_add(reservation).is_none_or(|t| t > end) {
                    return Err("unaffordable017 activity".into());
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
                step.consumption.push(self.opportunity_consume(owner));
            }
            if self.world.resource::<Runtime>().active.is_empty() {
                self.opportunity_complete(&commit);
                step.finished = Some(commit);
            }
        } else {
            self.world.resource_mut::<o::Environment>().wave += 1;
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
                self.world.resource_mut::<Runtime>().tick =
                    if eating.is_empty() { end } else { now + 1 };
                for owner in eating {
                    step.consumption.push(self.opportunity_consume(owner));
                }
            } else {
                if now + 2 > end {
                    return Err("unaffordable017 handshake".into());
                }
                self.world.resource_mut::<Runtime>().tick += 1;
                for owner in eating {
                    step.consumption.push(self.opportunity_consume(owner));
                }
                step.sent = Some(self.opportunity_capture(points));
                let people = self.opportunity_people();
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
                step.admitted = Some(self.opportunity_capture(points));
                step.allocations = e::arbitrate(
                    self.activity_tick(),
                    order,
                    wave,
                    &self.opportunity_people(),
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
                                return Err("unaffordable017 resource".into());
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
                            self.world.resource_mut::<o::Environment>().active =
                                Some(commit.clone());
                            step.started = Some(commit.clone());
                            if mode.serial() {
                                while !self.world.resource::<Runtime>().active.is_empty() {
                                    self.schedule.run(&mut self.world);
                                }
                                self.opportunity_complete(&commit);
                                step.finished = Some(commit);
                            }
                        }
                        e::Kind::Inquiry => {
                            let query = &outgoing[invite.proposer as usize].input.query;
                            let q::Action::Ask {
                                source, strategy, ..
                            } = query.decision.selected
                            else {
                                return Err("no017 query".into());
                            };
                            if source != invite.recipient
                                || self.activity_tick() + u64::from(strategy.cost()) + 3 > end
                            {
                                return Err("unaffordable017 inquiry".into());
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
                            self.provenance_follow(
                                invite.proposer,
                                &sources,
                                self.world.resource::<Runtime>().scenes.len() as u64,
                                None,
                            );
                        }
                    }
                }
            }
        }
        step.active_after = self.world.resource::<o::Environment>().active.clone();
        step.after = self.opportunity_capture(points);
        assert!(self.activity_tick() > now && self.activity_tick() <= end);
        Ok(step)
    }
    pub fn opportunity_finish(&mut self) -> Result<Vec<Consumption>, String> {
        self.require_idle()?;
        let end = self
            .world
            .get_resource::<o::Environment>()
            .ok_or("open017 first")?
            .end;
        if self.activity_tick() > end {
            return Err("017 already finished".into());
        }
        self.world.resource_mut::<Runtime>().tick = end;
        (0..3).map(|id| self.consume(id)).collect()
    }
}
