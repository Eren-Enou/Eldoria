//! Versioned023 coordination; protected physical/cognitive primitives are reused.
use super::*;
use crate::continuous as c;
impl Simulation {
    fn continuous_boundary(
        &mut self,
        index: usize,
        config: &c::Config,
        start: u64,
        cutoff: u64,
    ) -> x::Boundary {
        let now = self.activity_tick();
        let end = config.public_end(now, start);
        self.world.resource_mut::<State>().end = end;
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
        // Atomic inquiry may advance time: reproject rather than reuse a stale budget.
        let end = config.public_end(now, start);
        self.world.resource_mut::<State>().end = end;
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
        if now >= cutoff {
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

    pub(crate) fn continuous_trial23(cfg: c::Config) -> c::Trial {
        cfg.validate().unwrap();
        let config = cfg.physics();
        let (mut sim, start, end, initial, bootstrap_events, initial_contexts, stock, total) =
            Self::continuous_bootstrap(config.clone());
        if cfg.phase == c::Phase::Absolute {
            sim.world.resource_mut::<State>().epoch = 0;
        }
        let cutoff = start + cfg.horizon;
        let mut boundaries = vec![];
        while sim.activity_tick() < end {
            let boundary = sim.continuous_boundary(boundaries.len(), &cfg, start, cutoff);
            boundaries.push(boundary);
            assert!(sim.activity_tick() <= end, "drain bound exceeded");
            if sim.activity_tick() < end {
                sim.concurrent_tick();
            }
        }
        assert_eq!(sim.activity_tick(), end);
        assert!(sim.world.resource::<State>().active.is_empty());
        assert!(sim.world.resource::<State>().pending.is_empty());
        sim.concurrent_ledger();
        sim.validate_history_indexes().unwrap();
        let history = {
            let final_state = crate::experiment013::Snapshot {
                base: crate::experiment009::snapshot(&sim),
                learning: sim.self_evaluation().unwrap(),
                mode: crate::experiment013::Mode::Contextual,
            };
            final_state.validate().unwrap();
            let s = sim.world.resource::<State>();
            x::Trial {
                schema: "experiment-023-physical-history/v1".into(),
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
        };
        let checkpoints = c::checkpoints(&history, &cfg);
        c::Trial {
            schema: "experiment-023-trial/v1".into(),
            config: cfg,
            history,
            checkpoints,
        }
    }
}
