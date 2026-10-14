//! Opt-in014 adapter. Legacy run/meeting/resolvers are left unchanged.
use super::*;
use crate::{activity as a, inquiry as q, provenance as p};
impl Simulation {
    pub fn activity_tick(&self) -> u64 {
        self.world.resource::<Runtime>().tick
    }
    fn activity_age(&mut self, owner: u32) {
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
            let mut s = self.world.resource_mut::<Concerns>();
            *s.items
                .get_mut(&owner)
                .unwrap()
                .iter_mut()
                .find(|c| c.id == after.id)
                .unwrap() = after.clone();
            s.change(
                "014 public encounter",
                Some(before),
                Some(after),
                None,
                None,
            );
        }
    }
    pub fn activity_open(
        &mut self,
        owner: u32,
        source: Option<u32>,
        partner: Option<u32>,
        budget: u64,
        qwidth: u64,
        rwidth: u64,
    ) -> Result<(), String> {
        self.require_idle()?;
        if budget == 0 || budget > 1000 || qwidth > budget || rwidth > budget {
            return Err("invalid activity horizon".into());
        }
        if !self
            .world
            .contains_resource::<crate::self_evaluation::Learning>()
        {
            return Err("enable local learning first".into());
        }
        let index = &self.world.resource::<Index>().0;
        if !index.contains_key(&owner)
            || source
                .into_iter()
                .chain(partner)
                .any(|id| id == owner || !index.contains_key(&id))
        {
            return Err("invalid public participants".into());
        }
        if self
            .world
            .get_resource::<a::Clock>()
            .is_some_and(|c| self.activity_tick() < c.end + 3)
        {
            return Err("previous period unfinished".into());
        }
        // This narrow protocol excludes native refuser replies/variable sensor effort.
        if source.is_some_and(|s| {
            self.world
                .resource::<Concerns>()
                .items
                .get(&owner)
                .is_some_and(|cs| cs.iter().any(|c| c.target == s))
        }) {
            return Err("014 requires a third-party acquired-reply source".into());
        }
        self.activity_age(owner);
        if let Some(source) = source {
            self.world
                .resource_mut::<q::Inquiry>()
                .contact(owner, source);
        }
        let prior: Vec<_> = self
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
        for sig in prior {
            self.world.resource_mut::<q::Inquiry>().observe(owner, sig);
        }
        let start = self.activity_tick();
        self.world.insert_resource(a::Clock {
            owner,
            source,
            partner,
            end: start + budget,
            inquiry_close: start + qwidth,
            resource_close: start + rwidth,
            inquiry_used: false,
            resource_used: false,
            turns: 6,
        });
        Ok(())
    }
    pub fn activity_input(&self) -> Result<a::Input, String> {
        let c = self
            .world
            .get_resource::<a::Clock>()
            .ok_or("open014 period first")?;
        let own = self
            .world
            .get::<Agent>(self.world.resource::<Index>().0[&c.owner])
            .unwrap()
            .clone();
        let mut inquiry = None;
        let mut resource = None;
        let mut windows = vec![];
        if let Some(source) = c
            .source
            .filter(|_| !c.inquiry_used && self.activity_tick() < c.inquiry_close)
        {
            let state = self.world.resource::<q::Inquiry>();
            let talk = self.world.resource::<Intentional>();
            let base = q::Input {
                owner: c.owner,
                scene: self.world.resource::<Runtime>().scenes.len() as u64,
                hunger: own.hunger,
                relationship_goal: talk.profiles[&c.owner].relationship_goal,
                concerns: self
                    .world
                    .resource::<Concerns>()
                    .items
                    .get(&c.owner)
                    .cloned()
                    .unwrap_or_default(),
                sources: vec![q::Source {
                    id: source,
                    credibility: talk
                        .credibility
                        .get(&c.owner)
                        .and_then(|m| m.get(&source))
                        .copied()
                        .unwrap_or(0),
                    offers: state
                        .offers
                        .get(&c.owner)
                        .into_iter()
                        .flatten()
                        .filter(|o| o.source == source)
                        .cloned()
                        .collect(),
                }],
                cells: state
                    .cells
                    .get(&c.owner)
                    .into_iter()
                    .flatten()
                    .cloned()
                    .collect(),
                seen: state
                    .seen
                    .get(&c.owner)
                    .into_iter()
                    .flatten()
                    .cloned()
                    .collect(),
                settings: state.settings[&c.owner],
            };
            let pstate = self.world.resource::<crate::audit::RuntimeProvenance>();
            let input = p::Input {
                base,
                knowledge: pstate
                    .knowledge
                    .get(&c.owner)
                    .into_iter()
                    .flatten()
                    .cloned()
                    .collect(),
                hints: pstate
                    .hints
                    .get(&c.owner)
                    .into_iter()
                    .flatten()
                    .cloned()
                    .collect(),
                settings: pstate.settings[&c.owner],
            };
            let (decision, _) =
                self.self_evaluation_decide((self.world.resource::<p::Policy>().0)(&input));
            if let q::Action::Ask { strategy, .. } = decision.decision.selected {
                windows.push(a::Window {
                    action: a::Action::Inquiry,
                    closes: c.inquiry_close,
                    reservation: u64::from(strategy.cost()) + 3,
                });
            }
            inquiry = Some(decision);
        }
        if let Some(partner) = c
            .partner
            .filter(|_| !c.resource_used && self.activity_tick() < c.resource_close)
        {
            resource = Some((self.world.resource::<Policy>().0)(
                &own,
                &Observation {
                    partner,
                    last_signal: None,
                    amount: 1,
                    turns_left: c.turns,
                },
            ));
            windows.push(a::Window {
                action: a::Action::Resource,
                closes: c.resource_close,
                reservation: u64::from(c.turns),
            });
        }
        Ok(a::Input {
            own,
            now: self.activity_tick(),
            end: c.end,
            windows,
            inquiry,
            resource,
        })
    }
    pub fn activity_execute(&mut self, action: a::Action) -> Result<u64, String> {
        self.require_idle()?;
        let input = self.activity_input()?;
        let c = self.world.resource::<a::Clock>().clone();
        if input.now >= input.end {
            return Err("activity horizon closed".into());
        }
        if action != a::Action::Wait
            && !input
                .windows
                .iter()
                .any(|w| w.action == action && a::eligible(&input, w))
        {
            return Err("expired, used or unaffordable activity".into());
        }
        let start = self.activity_tick();
        match action {
            a::Action::Wait => self.world.resource_mut::<Runtime>().tick = c.end,
            a::Action::Inquiry => {
                self.world.resource_mut::<a::Clock>().inquiry_used = true;
                self.provenance_follow(
                    c.owner,
                    &[c.source.unwrap()],
                    self.world.resource::<Runtime>().scenes.len() as u64,
                    None,
                );
            }
            a::Action::Resource => {
                self.world.resource_mut::<a::Clock>().resource_used = true;
                self.add_scene([c.owner, c.partner.unwrap()], 1, c.turns)?;
                while !self.world.resource::<Runtime>().active.is_empty() {
                    self.schedule.run(&mut self.world);
                }
                // Core legality/food/episodes remain unchanged. Explicitly omit all
                // automatic conversations/inquiries from this budgeted entry point.
                let start = self.world.resource::<Runtime>().conversation_cursor;
                let end = self.world.resource::<Runtime>().events.len();
                self.world.resource_mut::<Runtime>().conversation_cursor = end;
                self.world.resource_mut::<Concerns>().scene_cursor =
                    self.world.resource::<Runtime>().scenes.len();
                self.activity_age(c.owner);
                self.activity_age(c.partner.unwrap());
                for event in start..end {
                    if self.world.resource::<Runtime>().events[event].outcome
                        == Some(Outcome::Refusal)
                    {
                        self.capture_concern(event as u64);
                    }
                }
            }
        }
        let elapsed = self.activity_tick() - start;
        if action != a::Action::Wait {
            let w = input.windows.iter().find(|w| w.action == action).unwrap();
            assert!(
                elapsed > 0
                    && elapsed <= w.reservation
                    && self.activity_tick() <= c.end.min(w.closes),
                "014 duration bound violated"
            );
        }
        Ok(elapsed)
    }
    pub fn activity_finish(&mut self) -> Result<Vec<Consumption>, String> {
        let end = self
            .world
            .get_resource::<a::Clock>()
            .ok_or("open014 period first")?
            .end;
        if self.activity_tick() > end {
            return Err("period already consumed".into());
        }
        self.world.resource_mut::<Runtime>().tick = end;
        (0..3).map(|id| self.consume(id)).collect()
    }
}
