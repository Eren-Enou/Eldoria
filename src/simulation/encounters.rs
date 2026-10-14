//! Isolated015 adapter; no protected014 implementation is changed.
use super::*;
use crate::{encounters as e, inquiry as q, provenance as p};
impl Simulation {
    pub fn enable_encounters(&mut self) -> Result<(), String> {
        self.require_idle()?;
        if self
            .world
            .resource::<Index>()
            .0
            .keys()
            .copied()
            .collect::<Vec<_>>()
            != vec![0, 1, 2]
            || !self
                .world
                .contains_resource::<crate::self_evaluation::Learning>()
        {
            return Err("015 requires IDs0/1/2 and Contextual learning".into());
        }
        if self
            .world
            .resource::<crate::self_evaluation::Learning>()
            .mode
            != crate::self_evaluation::Mode::Contextual
        {
            return Err("015 requires Contextual".into());
        }
        if !self.world.contains_resource::<e::Policies>() {
            self.world.insert_resource(e::Policies::default());
        }
        Ok(())
    }
    pub fn set_encounter_policies(
        &mut self,
        initiation: fn(&e::Input, e::Mode) -> e::Decision,
        response: fn(&e::ResponseInput) -> e::Response,
    ) -> Result<(), String> {
        self.require_idle()?;
        *self
            .world
            .get_resource_mut::<e::Policies>()
            .ok_or("enable encounters first")? = e::Policies {
            initiation,
            response,
        };
        Ok(())
    }
    pub fn encounter_prepare(&mut self) -> Result<(), String> {
        self.require_idle()?;
        if !self.world.contains_resource::<e::Policies>() {
            return Err("enable encounters first".into());
        }
        for owner in 0..3 {
            let before = self
                .world
                .resource::<Concerns>()
                .items
                .get(&owner)
                .cloned()
                .unwrap_or_default();
            for c in before.into_iter().filter(|c| c.status.active()) {
                let mut after = c.clone();
                after.age = after.age.saturating_add(1);
                let mut cs = self.world.resource_mut::<Concerns>();
                *cs.items
                    .get_mut(&owner)
                    .unwrap()
                    .iter_mut()
                    .find(|v| v.id == after.id)
                    .unwrap() = after.clone();
                cs.change(
                    "015 observed encounter wave",
                    Some(c),
                    Some(after),
                    None,
                    None,
                );
            }
            let signatures: Vec<_> = self
                .world
                .resource::<Cognition>()
                .beliefs
                .get(&owner)
                .into_iter()
                .flatten()
                .map(|b| {
                    super::adaptive::signature(
                        &self.world.resource::<Cognition>().information[b.evidence as usize],
                    )
                })
                .collect();
            for sig in signatures {
                self.world.resource_mut::<q::Inquiry>().observe(owner, sig);
            }
        }
        Ok(())
    }
    pub fn encounter_open(&mut self, budget: u64, busy_until: [u64; 3]) -> Result<(), String> {
        self.require_idle()?;
        if !self.world.contains_resource::<e::Policies>() || budget == 0 || budget > 128 {
            return Err("invalid015 horizon/enablement".into());
        }
        if self
            .world
            .get_resource::<e::Environment>()
            .is_some_and(|p| self.activity_tick() < p.end + 3)
        {
            return Err("unfinished015 period".into());
        }
        self.world.insert_resource(e::Environment {
            end: self.activity_tick() + budget,
            busy_until,
        });
        Ok(())
    }
    pub fn encounter_people(&self) -> Vec<e::Public> {
        let state = self.world.resource::<e::Environment>();
        (0..3)
            .map(|id| e::Public {
                id,
                available: self.activity_tick() >= state.busy_until[id as usize],
            })
            .collect()
    }
    pub fn encounter_state(&self) -> e::State {
        e::State {
            tick: self.activity_tick(),
            agents: self.agents(),
            events: self.world.resource::<Runtime>().events.len(),
            queries: self.world.resource::<q::Inquiry>().records.len(),
            assessment: self
                .world
                .resource::<crate::assessment::Assessment>()
                .records
                .len(),
            concerns: self.world.resource::<Concerns>().transitions.len(),
        }
    }
    pub fn encounter_input(
        &self,
        owner: u32,
        people: &[e::Public],
        end: u64,
    ) -> Result<e::Input, String> {
        if people.len() != 3
            || people.iter().enumerate().any(|(i, p)| p.id != i as u32)
            || owner >= 3
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
            scene: self.world.resource::<Runtime>().scenes.len() as u64,
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
    pub fn encounter_decide(&self, input: &e::Input, mode: e::Mode) -> e::Decision {
        (self.world.resource::<e::Policies>().initiation)(input, mode)
    }
    pub fn encounter_respond(&self, input: &e::ResponseInput) -> e::Response {
        (self.world.resource::<e::Policies>().response)(input)
    }
    fn encounter_advance(&mut self, end: u64) -> Result<(), String> {
        self.require_idle()?;
        if end < self.activity_tick() {
            return Err("cannot reverse015 clock".into());
        }
        self.world.resource_mut::<Runtime>().tick = end;
        Ok(())
    }
    fn encounter_resolve(
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
    pub fn encounter_wave(
        &mut self,
        mode: e::Mode,
        order: e::Order,
        id: u64,
    ) -> Result<e::Wave, String> {
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
            .map(|input| self.encounter_decide(input, mode))
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
            self.encounter_advance(end)?;
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
        self.encounter_advance(before.tick + 1)?;
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
                let response=self.encounter_respond(&input);
                if response.input!=input || matches!(response.selected,e::Reply::Accept(sender) if !input.incoming.iter().any(|i|i.proposer==sender) || !input.own.people.iter().any(|p|p.id==input.own.own.id && p.available)) {return Err("invalid015 recipient action".into());}
                Ok(response)
            })
            .collect::<Result<Vec<_>, String>>()?;
        self.encounter_advance(before.tick + 2)?;
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
                self.encounter_resolve(invite, &people, end, query)?;
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
    pub fn encounter_finish(&mut self) -> Result<Vec<Consumption>, String> {
        self.require_idle()?;
        let end = self
            .world
            .get_resource::<e::Environment>()
            .ok_or("open015 period first")?
            .end;
        if self.activity_tick() > end {
            return Err("015 period already consumed".into());
        }
        self.encounter_advance(end)?;
        (0..3).map(|id| self.consume(id)).collect()
    }
}
