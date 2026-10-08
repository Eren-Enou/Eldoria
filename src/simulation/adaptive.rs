//! Opt-in integration; prior communication/follow-up paths remain untouched.
use super::*;
use crate::inquiry::{self as q, Inquiry, InquiryPolicy};

pub(super) fn signature(info: &InformationScene) -> q::Signature {
    q::Signature {
        event: info.event,
        origin: match info.kind {
            EvidenceKind::Testimony { .. } => q::Origin::Claim(info.speaker),
            EvidenceKind::Fallible { source, .. } => q::Origin::Evidence {
                subject: info.speaker,
                channel: source,
            },
            EvidenceKind::Disclosure => q::Origin::Evidence {
                subject: info.speaker,
                channel: 2,
            },
        },
        claim: info.scarce,
        quality: info.reliability,
        receipt: info.id,
    }
}

impl Simulation {
    pub fn enable_inquiry(&mut self) -> Result<(), String> {
        self.require_idle()?;
        if !self.world.contains_resource::<Concerns>() {
            return Err("enable concerns first".into());
        }
        if self.world.contains_resource::<Inquiry>() {
            return Ok(());
        }
        self.world.insert_resource(Inquiry {
            settings: self
                .agents()
                .iter()
                .map(|a| (a.id, q::Settings::default()))
                .collect(),
            ..Default::default()
        });
        self.world.insert_resource(InquiryPolicy(q::policy));
        Ok(())
    }
    pub fn inquiry(&self) -> Option<Inquiry> {
        self.world.get_resource::<Inquiry>().cloned()
    }
    pub fn set_inquiry_policy(
        &mut self,
        policy: fn(&q::Input) -> q::Decision,
    ) -> Result<(), String> {
        self.require_idle()?;
        self.world
            .get_resource_mut::<InquiryPolicy>()
            .ok_or("enable inquiry first")?
            .0 = policy;
        Ok(())
    }
    pub fn set_inquiry_settings(
        &mut self,
        owner: AgentId,
        settings: q::Settings,
    ) -> Result<(), String> {
        self.require_idle()?;
        *self
            .world
            .get_resource_mut::<Inquiry>()
            .ok_or("enable inquiry first")?
            .settings
            .get_mut(&owner)
            .ok_or("unknown agent")? = settings;
        Ok(())
    }
    /// Experimental capability intervention; no announcement or reading is forced.
    pub fn set_inquiry_capability(
        &mut self,
        source: AgentId,
        event: u64,
        channel: u8,
        sensor: Sensor,
    ) -> Result<(), String> {
        self.require_idle()?;
        if !self.world.contains_resource::<Inquiry>() {
            return Err("enable inquiry first".into());
        }
        if channel > 1
            || !(0..=90).contains(&sensor.reliability)
            || sensor.effort > 100
            || sensor.second.is_some()
        {
            return Err(
                "capability requires one stable channel 0..1, quality 0..90, effort 0..100".into(),
            );
        }
        let original = self
            .world
            .resource::<Runtime>()
            .events
            .get(event as usize)
            .ok_or("unknown event")?;
        if original.outcome != Some(Outcome::Refusal) || original.decision.actor != source {
            return Err("source can offer only their own recorded refusal observation".into());
        }
        let speaker_index = original
            .participants
            .iter()
            .position(|&id| id == source)
            .unwrap();
        let (food, hunger) =
            self.world.resource::<Runtime>().circumstances[event as usize][speaker_index];
        let proposed = foresight::reading(
            self.world.resource::<Foresight>().seed,
            event,
            source,
            channel,
            food <= 1 && hunger >= 60,
            &sensor,
        );
        let listener = original.participants[1 - speaker_index];
        let received = self
            .world
            .resource::<HistoryIndex>()
            .sources
            .get(&(event, source, listener, channel));
        if matches!(received,Some(EvidenceKind::Fallible{scarce,..}) if *scarce != proposed) {
            return Err(
                "received evidence content cannot be rewritten under the same provenance".into(),
            );
        }
        if matches!(received,Some(EvidenceKind::Fallible{reliability,..}) if *reliability != sensor.reliability)
        {
            return Err(
                "received evidence quality cannot be rewritten under the same provenance".into(),
            );
        }
        let mut state = self.world.resource_mut::<Inquiry>();
        let items = state.capabilities.entry(source).or_default();
        let before = items.iter().find(|c| c.event == event).cloned();
        if let Some(old) = &before
            && old.channel == channel
            && old.sensor != sensor
        {
            return Err("use a new channel for a changed capability".into());
        }
        items.retain(|c| c.event != event);
        let after = q::Capability {
            event,
            channel,
            sensor,
        };
        if items.len() == q::CELL_CAPACITY {
            let removed = items.pop_front().unwrap().event;
            state.evictions.push(q::Eviction {
                owner: source,
                collection: "capabilities".into(),
                reference: removed,
            });
        }
        state
            .capabilities
            .entry(source)
            .or_default()
            .push_back(after.clone());
        state.interventions.push(q::Intervention {
            source,
            before,
            after,
        });
        Ok(())
    }
    /// A bounded public social encounter; all participants choose voluntarily.
    /// Contacts are learned through this recorded co-presence, never a global lookup.
    pub fn inquiry_meeting(&mut self, participants: &[AgentId]) -> Result<(), String> {
        self.require_idle()?;
        if !self.world.contains_resource::<Inquiry>() {
            return Err("enable inquiry first".into());
        }
        let mut ids = participants.to_vec();
        ids.sort_unstable();
        ids.dedup();
        if !(2..=8).contains(&ids.len())
            || ids.len() != participants.len()
            || ids
                .iter()
                .any(|id| !self.world.resource::<Index>().0.contains_key(id))
        {
            return Err("meeting requires 2..8 distinct existing participants".into());
        }
        self.world.resource_mut::<Runtime>().tick += 1;
        let tick = self.world.resource::<Runtime>().tick;
        let meeting = self.world.resource::<Inquiry>().meetings.len() as u64;
        self.world
            .resource_mut::<Inquiry>()
            .meetings
            .push(q::Meeting {
                id: meeting,
                participants: ids.clone(),
                tick,
            });
        let scene = self.world.resource::<Runtime>().scenes.len() as u64;
        for &owner in &ids {
            let old = self
                .world
                .resource::<Concerns>()
                .items
                .get(&owner)
                .cloned()
                .unwrap_or_default();
            for before in old
                .into_iter()
                .filter(|c| c.status.active() && c.created_scene < scene)
            {
                let mut after = before.clone();
                after.age = after.age.saturating_add(1);
                let mut state = self.world.resource_mut::<Concerns>();
                *state
                    .items
                    .get_mut(&owner)
                    .unwrap()
                    .iter_mut()
                    .find(|c| c.id == after.id)
                    .unwrap() = after.clone();
                state.change(
                    "later inquiry meeting",
                    Some(before),
                    Some(after),
                    None,
                    None,
                );
            }
            let available: Vec<_> = ids.iter().copied().filter(|&id| id != owner).collect();
            self.adaptive_follow(owner, &available, scene, Some(meeting));
        }
        Ok(())
    }
    pub(super) fn advertise_opportunities(&mut self, owner: AgentId, available: &[AgentId]) {
        let concerns = self
            .world
            .resource::<Concerns>()
            .items
            .get(&owner)
            .cloned()
            .unwrap_or_default();
        for c in concerns
            .iter()
            .filter(|c| c.status.active() && available.contains(&c.target))
        {
            let capability = self
                .world
                .resource::<Inquiry>()
                .capabilities
                .get(&c.target)
                .and_then(|v| v.iter().find(|v| v.event == c.event))
                .cloned();
            let Some(capability) = capability else {
                continue;
            };
            let profile = self.world.resource::<Intentional>().profiles[&c.target].clone();
            let hunger = self
                .world
                .get::<Agent>(self.world.resource::<Index>().0[&c.target])
                .unwrap()
                .hunger;
            // Voluntary announcement: a public possibility, not the forthcoming reading.
            let score = profile.relationship_goal
                - profile.privacy
                - hunger / 5
                - capability.sensor.effort as i32;
            let offered = score > 0;
            self.world.resource_mut::<Runtime>().tick += 1;
            let tick = self.world.resource::<Runtime>().tick;
            let mut state = self.world.resource_mut::<Inquiry>();
            let id = state.notices.len() as u64;
            let offer = q::Offer {
                source: c.target,
                event: c.event,
                channel: capability.channel,
                quality: capability.sensor.reliability,
                effort: capability.sensor.effort,
                notice: id,
            };
            state.notices.push(q::Notice {
                id,
                tick,
                owner,
                source: c.target,
                event: c.event,
                profile,
                hunger,
                can_observe_self: true,
                score,
                offered,
                offer: offer.clone(),
                time_spent: 1,
            });
            if offered {
                let list = state.offers.entry(owner).or_default();
                list.retain(|o| !(o.source == c.target && o.event == c.event));
                if list.len() == q::SIGNATURE_CAPACITY {
                    let reference = list.pop_front().unwrap().notice;
                    state.evictions.push(q::Eviction {
                        owner,
                        collection: "offers".into(),
                        reference,
                    });
                }
                state.offers.entry(owner).or_default().push_back(offer);
            }
        }
    }
    pub(super) fn adaptive_follow(
        &mut self,
        owner: AgentId,
        available: &[AgentId],
        scene: u64,
        meeting: Option<u64>,
    ) {
        if self
            .world
            .contains_resource::<crate::provenance::Provenance>()
        {
            self.provenance_follow(owner, available, scene, meeting);
            return;
        }
        for &source in available {
            self.world.resource_mut::<Inquiry>().contact(owner, source);
        }
        self.advertise_opportunities(owner, available);
        // Recover only receipts referenced by this actor's own bounded beliefs.
        let prior: Vec<_> = self
            .world
            .resource::<Cognition>()
            .beliefs
            .get(&owner)
            .into_iter()
            .flatten()
            .filter_map(|b| {
                self.world
                    .resource::<Cognition>()
                    .information
                    .get(b.evidence as usize)
            })
            .map(signature)
            .collect();
        for info in prior {
            self.world.resource_mut::<Inquiry>().observe(owner, info);
        }
        let state = self.world.resource::<Inquiry>();
        let talk = self.world.resource::<Intentional>();
        let input = q::Input {
            owner,
            scene,
            hunger: self
                .world
                .get::<Agent>(self.world.resource::<Index>().0[&owner])
                .unwrap()
                .hunger,
            relationship_goal: talk.profiles[&owner].relationship_goal,
            concerns: self
                .world
                .resource::<Concerns>()
                .items
                .get(&owner)
                .cloned()
                .unwrap_or_default(),
            sources: available
                .iter()
                .copied()
                .map(|id| q::Source {
                    id,
                    credibility: talk
                        .credibility
                        .get(&owner)
                        .and_then(|v| v.get(&id))
                        .copied()
                        .unwrap_or(0),
                    offers: state
                        .offers
                        .get(&owner)
                        .into_iter()
                        .flatten()
                        .filter(|o| o.source == id)
                        .cloned()
                        .collect(),
                })
                .collect(),
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
        let decision = (self.world.resource::<InquiryPolicy>().0)(&input);
        let valid = decision.input == input
            && q::legal(&input, decision.selected)
            && decision
                .candidates
                .iter()
                .any(|c| c.action == decision.selected)
            && decision
                .candidates
                .iter()
                .all(|c| q::legal(&input, c.action));
        let selected = if valid {
            decision.selected
        } else {
            q::Action::Pause
        };
        let before = match selected {
            q::Action::Ask { concern, .. } => {
                input.concerns.iter().find(|c| c.id == concern).cloned()
            }
            _ => None,
        };
        let food_before: Vec<_> = std::iter::once(owner)
            .chain(available.iter().copied())
            .map(|id| {
                (
                    id,
                    self.world
                        .get::<Agent>(self.world.resource::<Index>().0[&id])
                        .unwrap()
                        .food,
                )
            })
            .collect();
        let time_spent = 1 + match selected {
            q::Action::Ask { strategy, .. } => strategy.cost(),
            _ => 0,
        };
        self.world.resource_mut::<Runtime>().tick += u64::from(time_spent);
        let id = self.world.resource::<Inquiry>().records.len() as u64;
        let mut response = None;
        let mut category = q::Novelty::None;
        let mut value = 0;
        let mut information_values = Vec::new();
        let mut cell_before = None;
        let mut cell_after = None;
        let mut after = None;
        if let q::Action::Ask {
            concern,
            source,
            strategy,
        } = selected
        {
            let c = before.as_ref().unwrap();
            let reply = self.inquiry_response(owner, source, c, strategy);
            let mut seen = input.seen.clone();
            for sig in &reply.signatures {
                let (kind, mut useful) = q::novelty(&seen, Some(sig));
                if !input.settings.track_novelty {
                    useful = sig.quality.min(40);
                }
                information_values.push((sig.receipt, kind, useful));
                if kind == q::Novelty::Conflict
                    || (category != q::Novelty::Conflict && useful >= value)
                {
                    category = kind;
                }
                value = value.max(useful);
                seen.push(sig.clone());
            }
            if reply.valid {
                let mut state = self.world.resource_mut::<Inquiry>();
                let attempted_offer = (strategy == q::Strategy::Evidence)
                    .then(|| {
                        input
                            .sources
                            .iter()
                            .find(|s| s.id == source)
                            .and_then(|s| s.offers.iter().rev().find(|o| o.event == c.event))
                            .map(|o| (o.channel, o.quality))
                    })
                    .flatten();
                let (old, new) = state.learn(
                    owner,
                    (concern, source, strategy),
                    value,
                    id,
                    attempted_offer,
                );
                cell_before = old;
                cell_after = Some(new);
                for sig in &reply.signatures {
                    state.observe(owner, sig.clone());
                }
                let episodes = state.episodes.entry(owner).or_default();
                if episodes.len() == q::EPISODE_CAPACITY {
                    let reference = episodes.pop_front().unwrap().record;
                    state.evictions.push(q::Eviction {
                        owner,
                        collection: "episodes".into(),
                        reference,
                    });
                }
                state
                    .episodes
                    .entry(owner)
                    .or_default()
                    .push_back(q::Episode {
                        record: id,
                        concern,
                        source,
                        strategy,
                        novelty: category,
                        value,
                        receipt: reply.signatures.last().map(|s| s.receipt),
                    });
            }
            let mut state = self.world.resource_mut::<Concerns>();
            let actual = state
                .items
                .get_mut(&owner)
                .unwrap()
                .iter_mut()
                .find(|x| x.id == concern)
                .unwrap();
            let preceding = actual.clone();
            actual.attempts = actual.attempts.saturating_add(1);
            if value == 0 {
                actual.failures = actual.failures.saturating_add(1);
            }
            after = Some(actual.clone());
            // Inquiry record IDs have their own namespace; no false FollowRecord link.
            state.change(
                "adaptive inquiry outcome",
                Some(preceding),
                after.clone(),
                reply.signatures.last().map(|s| s.receipt),
                None,
            );
            response = Some(reply);
        }
        let tick = self.world.resource::<Runtime>().tick;
        let food_after = food_before
            .iter()
            .map(|&(agent, _)| {
                (
                    agent,
                    self.world
                        .get::<Agent>(self.world.resource::<Index>().0[&agent])
                        .unwrap()
                        .food,
                )
            })
            .collect();
        self.world
            .resource_mut::<Inquiry>()
            .records
            .push(q::Record {
                id,
                tick,
                meeting,
                decision,
                valid,
                response,
                novelty: category,
                realized_value: value,
                information_values,
                cell_before,
                cell_after,
                concern_before: before,
                concern_after: after,
                time_spent,
                food_after,
                food_before,
            });
    }
    pub(super) fn inquiry_response(
        &mut self,
        owner: AgentId,
        source: AgentId,
        concern: &concerns::Concern,
        strategy: q::Strategy,
    ) -> q::Response {
        let original = &self.world.resource::<Runtime>().events[concern.event as usize];
        let is_refuser = original.decision.actor == source;
        let scarce = if is_refuser {
            let index = original
                .participants
                .iter()
                .position(|&id| id == source)
                .unwrap();
            let (food, hunger) =
                self.world.resource::<Runtime>().circumstances[concern.event as usize][index];
            Some(food <= 1 && hunger >= 60)
        } else {
            None
        };
        let state = self.world.resource::<Intentional>();
        let credibility_before = state
            .credibility
            .get(&owner)
            .and_then(|m| m.get(&source))
            .copied()
            .unwrap_or(0);
        let input = TalkInput {
            own: self
                .world
                .get::<Agent>(self.world.resource::<Index>().0[&source])
                .unwrap()
                .clone(),
            profile: state.profiles[&source].clone(),
            partner: owner,
            event: concern.event,
            is_refuser,
            own_historical_scarcity: scarce,
            last: Some(match strategy {
                q::Strategy::Direct => TalkAction::AskExplanation,
                q::Strategy::Evidence => TalkAction::AskEvidence,
            }),
            belief: self
                .world
                .resource::<Cognition>()
                .beliefs
                .get(&source)
                .and_then(|v| v.iter().find(|b| b.event == concern.event))
                .cloned(),
            credibility: state
                .credibility
                .get(&source)
                .and_then(|m| m.get(&owner))
                .copied()
                .unwrap_or(0),
            memories: state
                .memories
                .get(&source)
                .into_iter()
                .flatten()
                .cloned()
                .collect(),
            turns_left: 1,
        };
        let capability = self
            .world
            .resource::<Inquiry>()
            .capabilities
            .get(&source)
            .and_then(|v| v.iter().find(|c| c.event == concern.event))
            .cloned();
        let mut context = self.world.resource::<Foresight>().context(source, owner);
        if let Some(c) = &capability {
            context.reliability = c.sensor.reliability;
            context.effort = c.sensor.effort;
        }
        let plan = (self.world.resource::<Predictor>().0)(&input, &context);
        let decision = plan.decision;
        let valid = decision.input == input
            && plan.context == context
            && intentional::legal(&input).contains(&decision.selected)
            && [
                TalkAction::Silence,
                TalkAction::Explain,
                TalkAction::Mislead,
                TalkAction::ProvideEvidence,
            ]
            .contains(&decision.selected)
            && decision
                .candidates
                .iter()
                .any(|c| c.action == decision.selected)
            && decision
                .candidates
                .iter()
                .all(|c| intentional::legal(&input).contains(&c.action));
        let action = if valid {
            decision.selected
        } else {
            TalkAction::Silence
        };
        let start_tick = self.world.resource::<Runtime>().tick;
        let mut sig = Vec::new();
        let mut receipts = Vec::new();
        let mut verified_claim = None;
        let mut credibility_after = credibility_before;
        if valid && is_refuser {
            let kind = match action {
                TalkAction::Explain => Some(EvidenceKind::Testimony {
                    scarce: scarce.unwrap(),
                }),
                TalkAction::Mislead => Some(EvidenceKind::Testimony { scarce: true }),
                _ => None,
            };
            if let Some(kind) = kind {
                let info = self
                    .receive_information(
                        source,
                        owner,
                        concern.event,
                        kind,
                        Some((50 + credibility_before).clamp(10, 90)),
                    )
                    .unwrap();
                sig.push(signature(&info));
                receipts.push(info.id);
            } else if action == TalkAction::ProvideEvidence {
                let state = self.world.resource::<Foresight>();
                let seed = state.seed;
                let (channel, sensor) =
                    capability.map_or((0, state.sensor.clone()), |c| (c.channel, c.sensor));
                // Preview the bounded batch in the resolver, never in a policy.
                let channels = if sensor.second.is_some() {
                    vec![0, 1]
                } else {
                    vec![channel]
                };
                let readings: Vec<_> = channels
                    .iter()
                    .map(|&ch| {
                        (
                            ch,
                            foresight::reading(
                                seed,
                                concern.event,
                                source,
                                ch,
                                scarce.unwrap(),
                                &sensor,
                            ),
                        )
                    })
                    .collect();
                let incompatible = readings.iter().any(|&(ch,observed)|
                    matches!(self.world.resource::<HistoryIndex>().sources.get(&(concern.event,source,owner,ch)),
                        Some(EvidenceKind::Fallible{scarce,reliability,..}) if *scarce!=observed || *reliability!=sensor.reliability));
                if incompatible {
                    self.world.resource_mut::<Runtime>().tick += 1;
                    return q::Response {
                        decision,
                        valid: false,
                        public_action: TalkAction::Silence,
                        signatures: Vec::new(),
                        receipts: Vec::new(),
                        time_spent: 1,
                        credibility_before,
                        credibility_after: credibility_before,
                        verified_claim: None,
                        failure: Some(
                            "immutable evidence provenance mismatch; no batch applied".into(),
                        ),
                    };
                }
                // A repeated channel is the same observation, with no resampling credit.
                for (ch, observed) in readings {
                    let info = self
                        .receive_information(
                            source,
                            owner,
                            concern.event,
                            EvidenceKind::Fallible {
                                scarce: observed,
                                reliability: sensor.reliability,
                                source: ch,
                            },
                            None,
                        )
                        .unwrap();
                    let s = signature(&info);
                    let previous = self
                        .world
                        .resource::<Inquiry>()
                        .seen
                        .get(&owner)
                        .and_then(|v| {
                            v.iter().find(|s| {
                                s.event == concern.event && s.origin == q::Origin::Claim(source)
                            })
                        })
                        .cloned();
                    let already_compared = self
                        .world
                        .resource::<Inquiry>()
                        .seen
                        .get(&owner)
                        .is_some_and(|v| {
                            v.iter().any(|old| {
                                old.event == s.event
                                    && old.origin == s.origin
                                    && old.quality >= s.quality
                                    && old.claim == s.claim
                            })
                        });
                    if let Some(claim) = previous
                        && info.after.support.abs() >= 60
                        && !already_compared
                    {
                        let consistent = claim.claim == observed;
                        verified_claim = Some(consistent);
                        let amount =
                            (if consistent { 20 } else { -30 }) * info.after.support.abs() / 100;
                        credibility_after = (credibility_before + amount).clamp(-40, 40);
                        self.world
                            .resource_mut::<Intentional>()
                            .credibility
                            .entry(owner)
                            .or_default()
                            .insert(source, credibility_after);
                    }
                    sig.push(s);
                    receipts.push(info.id);
                }
                self.world.resource_mut::<Runtime>().tick += u64::from(sensor.effort);
            }
        }
        self.world.resource_mut::<Runtime>().tick += 1;
        let time_spent = (self.world.resource::<Runtime>().tick - start_tick) as u32;
        q::Response {
            decision,
            valid,
            public_action: if action == TalkAction::Mislead {
                TalkAction::Explain
            } else {
                action
            },
            signatures: sig,
            receipts,
            time_spent,
            credibility_before,
            credibility_after,
            verified_claim,
            failure: (!valid).then(|| "illegal or mismatched local response".into()),
        }
    }
}
