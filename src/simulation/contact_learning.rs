//! Prospective016 adapter; protected015 adapter remains byte-identical.
use super::*;
use crate::{contact_learning as c, encounters as e, inquiry as q, provenance as p};
impl Simulation {
    pub fn enable_contact_learning(&mut self, mode: c::Mode) -> Result<(), String> {
        self.require_idle()?;
        self.enable_encounters()?;
        if self.world.contains_resource::<c::Learning>() {
            return Err("016 already enabled".into());
        }
        self.world.insert_resource(c::Learning {
            mode,
            cells: std::array::from_fn(|_| vec![]),
            choose: c::policy,
            learn: c::update,
        });
        Ok(())
    }
    pub fn set_contact_policies(
        &mut self,
        choose: fn(&c::Input) -> e::Decision,
        learn: fn(&[c::Cell], c::Mode, c::Signal) -> c::Update,
    ) -> Result<(), String> {
        self.require_idle()?;
        let mut l = self
            .world
            .get_resource_mut::<c::Learning>()
            .ok_or("enable016 first")?;
        l.choose = choose;
        l.learn = learn;
        Ok(())
    }
    pub fn contact_cells(&self) -> [Vec<c::Cell>; 3] {
        self.world.resource::<c::Learning>().cells.clone()
    }
    pub fn contact_wave(&mut self, order: e::Order, id: u64) -> Result<c::Wave, String> {
        self.require_idle()?;
        let before = self.contact_cells();
        let now = self.activity_tick();
        let available = before.each_ref().map(|cs| c::expire(cs, now));
        self.world.resource_mut::<c::Learning>().cells = available.clone();
        let wave = self.contact_wave_inner(order, id)?;
        let mut updates = vec![];
        for owner in 0..3 {
            if let Some(signal) = c::signal(&wave, owner) {
                let l = self.world.resource::<c::Learning>();
                let u = (l.learn)(&l.cells[owner as usize], l.mode, signal.clone());
                if u.signal != signal
                    || u.before != l.cells[owner as usize]
                    || u.after.len() > 2
                    || u.after.iter().any(|x| {
                        x.partner == owner
                            || x.partner >= 3
                            || x.until <= wave.after.tick
                            || x.received > wave.after.tick
                    })
                {
                    return Err("invalid016 retained feedback".into());
                }
                self.world.resource_mut::<c::Learning>().cells[owner as usize] = u.after.clone();
                updates.push(u);
            }
        }
        Ok(c::Wave {
            memory_before: before,
            available,
            updates,
            wave,
        })
    }
    fn contact_advance(&mut self, end: u64) -> Result<(), String> {
        self.require_idle()?;
        if end < self.activity_tick() {
            return Err("cannot reverse015 clock".into());
        }
        self.world.resource_mut::<Runtime>().tick = end;
        Ok(())
    }
    fn contact_resolve(
        &mut self,
        invite: &e::Invitation,
        people: &[e::Public],
        end: u64,
        query: &p::Decision,
    ) -> Result<u64, String> {
        self.require_idle()?;
        let start = self.activity_tick();
        if start != invite.expires
            || invite.proposer == invite.recipient
            || ![invite.proposer, invite.recipient]
                .iter()
                .all(|id| people.iter().any(|p| p.id == *id && p.available))
        {
            return Err("expired or unavailable015 encounter".into());
        }
        let cap = match invite.kind {
            e::Kind::Resource => 6,
            e::Kind::Inquiry => {
                let q::Action::Ask {
                    source, strategy, ..
                } = query.decision.selected
                else {
                    return Err("no selected015 query".into());
                };
                if source != invite.recipient {
                    return Err("query recipient mismatch".into());
                }
                u64::from(strategy.cost()) + 3
            }
        };
        if start.checked_add(cap).is_none_or(|t| t > end) {
            return Err("unaffordable015 encounter".into());
        }
        match invite.kind {
            e::Kind::Inquiry => {
                let sources = query
                    .input
                    .base
                    .sources
                    .iter()
                    .map(|s| s.id)
                    .collect::<Vec<_>>();
                self.provenance_follow(
                    invite.proposer,
                    &sources,
                    self.world.resource::<Runtime>().scenes.len() as u64,
                    None,
                );
            }
            e::Kind::Resource => {
                self.add_scene([invite.proposer, invite.recipient], 1, 6)?;
                while !self.world.resource::<Runtime>().active.is_empty() {
                    self.schedule.run(&mut self.world);
                }
                let first = self.world.resource::<Runtime>().conversation_cursor;
                let last = self.world.resource::<Runtime>().events.len();
                self.world.resource_mut::<Runtime>().conversation_cursor = last;
                self.world.resource_mut::<Concerns>().scene_cursor =
                    self.world.resource::<Runtime>().scenes.len();
                for owner in [invite.proposer, invite.recipient] {
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
                            "015 actual resource encounter",
                            Some(before),
                            Some(after),
                            None,
                            None,
                        );
                    }
                }
                for event in first..last {
                    if self.world.resource::<Runtime>().events[event].outcome
                        == Some(Outcome::Refusal)
                    {
                        self.capture_concern(event as u64);
                    }
                }
            }
        }
        let elapsed = self.activity_tick() - start;
        assert!(
            elapsed > 0 && elapsed <= cap && self.activity_tick() <= end,
            "015 duration cap violated"
        );
        Ok(elapsed)
    }
    fn contact_wave_inner(&mut self, order: e::Order, id: u64) -> Result<e::Wave, String> {
        self.require_idle()?;
        let end = self
            .world
            .get_resource::<e::Environment>()
            .ok_or("open015 period first")?
            .end;
        if self.activity_tick() >= end {
            return Err("015 horizon closed".into());
        }
        self.encounter_prepare()?;
        let before = self.encounter_state();
        let people = self.encounter_people();
        let inputs = (0..3)
            .map(|owner| self.encounter_input(owner, &people, end))
            .collect::<Result<Vec<_>, _>>()?;
        let proposals = inputs
            .iter()
            .map(|input| {
                let l = self.world.resource::<c::Learning>();
                (l.choose)(&c::Input {
                    own: input.clone(),
                    mode: l.mode,
                    retained: l.cells[input.own.id as usize].clone(),
                })
            })
            .collect::<Vec<_>>();
        for (input, d) in inputs.iter().zip(&proposals) {
            if &d.input != input {
                return Err("015 policy altered captured input".into());
            }
            if let e::Intent::Invite { recipient, kind } = d.selected {
                if recipient == input.own.id
                    || ![recipient, input.own.id]
                        .iter()
                        .all(|id| people.iter().any(|p| p.id == *id && p.available))
                {
                    return Err("invalid015 participants".into());
                }
                let cap = match kind {
                    e::Kind::Resource => 8,
                    e::Kind::Inquiry => {
                        let q::Action::Ask {
                            source, strategy, ..
                        } = input.query.decision.selected
                        else {
                            return Err("invalid015 query proposal".into());
                        };
                        if source != recipient {
                            return Err("invalid015 query source".into());
                        }
                        u64::from(strategy.cost()) + 5
                    }
                };
                if before.tick + cap > end {
                    return Err("unaffordable015 proposal".into());
                }
            }
        }
        let invitations = proposals
            .iter()
            .filter_map(|d| match d.selected {
                e::Intent::Wait => None,
                e::Intent::Invite { recipient, kind } => Some(e::Invitation {
                    proposer: d.input.own.id,
                    recipient,
                    kind,
                    expires: before.tick + 2,
                }),
            })
            .collect::<Vec<_>>();
        if invitations.is_empty() {
            self.contact_advance(end)?;
            return Ok(e::Wave {
                id,
                before,
                proposals,
                responses: vec![],
                allocations: vec![],
                execution: None,
                after: self.encounter_state(),
            });
        }
        if before.tick + 2 > end {
            return Err("invalid015 policy handshake".into());
        }
        for d in &proposals {
            if d.selected != e::Intent::Wait
                && !d
                    .candidates
                    .iter()
                    .any(|c| c.intent == d.selected && c.eligible)
            {
                return Err("invalid015 selected proposal".into());
            }
        }
        self.contact_advance(before.tick + 1)?;
        let people = self.encounter_people();
        let responses = proposals
            .iter()
            .map(|d| {
                let own = self.encounter_input(d.input.own.id, &people, end)?;
                let input=e::ResponseInput {
                    own,
                    outgoing: d.clone(),
                    incoming: invitations
                        .iter()
                        .filter(|i| i.recipient == d.input.own.id)
                        .cloned()
                        .collect(),
                };
                let response=if self.world.resource::<c::Learning>().mode==c::Mode::Unchanged {e::respond(&input)} else {c::respond(&input)};
                if response.input!=input || matches!(response.selected,e::Reply::Accept(sender) if !input.incoming.iter().any(|i|i.proposer==sender) || !input.own.people.iter().any(|p|p.id==input.own.own.id && p.available)) {return Err("invalid015 recipient action".into());}
                Ok(response)
            })
            .collect::<Result<Vec<_>, String>>()?;
        self.contact_advance(before.tick + 2)?;
        let people = self.encounter_people();
        let allocations = e::arbitrate(
            self.activity_tick(),
            order,
            id,
            &people,
            &invitations,
            &responses,
        );
        let mut execution = None;
        for allocation in &allocations {
            if allocation.disposition == e::Disposition::Execute {
                let invite = &allocation.invitation;
                let start = self.encounter_state();
                let query = &proposals[invite.proposer as usize].input.query;
                let cap = match invite.kind {
                    e::Kind::Resource => 6,
                    e::Kind::Inquiry => {
                        let q::Action::Ask { strategy, .. } = query.decision.selected else {
                            unreachable!()
                        };
                        u64::from(strategy.cost()) + 3
                    }
                };
                for participant in [invite.proposer, invite.recipient] {
                    self.world.resource_mut::<e::Environment>().busy_until[participant as usize] =
                        start.tick + cap;
                }
                self.contact_resolve(invite, &people, end, query)?;
                let after = self.encounter_state();
                let mut environment = self.world.resource_mut::<e::Environment>();
                environment.busy_until[invite.proposer as usize] = after.tick;
                environment.busy_until[invite.recipient as usize] = after.tick;
                execution = Some(e::Execution {
                    invitation: invite.clone(),
                    before: start,
                    after,
                });
            }
        }
        Ok(e::Wave {
            id,
            before,
            proposals,
            responses,
            allocations,
            execution,
            after: self.encounter_state(),
        })
    }
}
