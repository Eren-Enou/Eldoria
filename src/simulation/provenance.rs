//! Privileged resolution stays here; pure policies receive only local projections.
use super::*;
use crate::audit::{
    CompactSnapshot, Contexts, InquiryId, LocalContext, QueryId, QueryRef, ReceiptId,
};
use crate::{inquiry as q, provenance as p};

impl Simulation {
    pub fn enable_provenance(&mut self) -> Result<(), String> {
        self.require_idle()?;
        if !self.world.contains_resource::<q::Inquiry>() {
            return Err("enable inquiry first".into());
        }
        if self
            .world
            .contains_resource::<crate::audit::RuntimeProvenance>()
        {
            return Ok(());
        }
        self.world.insert_resource(crate::audit::RuntimeProvenance {
            settings: self
                .agents()
                .iter()
                .map(|a| (a.id, p::Settings::default()))
                .collect(),
            ..Default::default()
        });
        self.world.insert_resource(p::Policy(p::policy));
        self.world.insert_resource(Contexts::default());
        self.world.insert_resource(p::ExchangePolicy(p::exchange));
        self.world
            .insert_resource(p::InspectionPolicy(p::inspection));
        Ok(())
    }
    pub fn provenance(&self) -> Option<p::Provenance> {
        let state = self
            .world
            .get_resource::<crate::audit::RuntimeProvenance>()?;
        let contexts = &self.world.resource::<Contexts>().0;
        let inquiry = self.world.resource::<q::Inquiry>();
        let queries = state
            .queries
            .iter()
            .map(|r| {
                r.reconstruct(inquiry, contexts)
                    .expect("internally valid audit reference")
            })
            .collect();
        Some(state.clone().with_queries(queries))
    }
    /// Observer export with lossless, checked references. No legacy query expansion.
    pub fn compact_snapshot(&self, label: &str) -> Option<CompactSnapshot> {
        Some(CompactSnapshot {
            format: CompactSnapshot::FORMAT.into(),
            base: crate::experiment006::snapshot(self, label),
            provenance: self
                .world
                .get_resource::<crate::audit::RuntimeProvenance>()?
                .clone(),
            contexts: self.world.resource::<Contexts>().0.clone(),
        })
    }
    fn provenance_ready(&self) -> Result<(), String> {
        self.require_idle()?;
        if !self
            .world
            .contains_resource::<crate::audit::RuntimeProvenance>()
        {
            return Err("enable provenance first".into());
        }
        Ok(())
    }
    pub fn set_provenance_settings(
        &mut self,
        owner: AgentId,
        settings: p::Settings,
    ) -> Result<(), String> {
        self.provenance_ready()?;
        if !self.world.resource::<Index>().0.contains_key(&owner) {
            return Err("unknown owner".into());
        }
        self.world
            .resource_mut::<crate::audit::RuntimeProvenance>()
            .settings
            .insert(owner, settings);
        Ok(())
    }
    pub fn set_provenance_policy(
        &mut self,
        policy: fn(&p::Input) -> p::Decision,
    ) -> Result<(), String> {
        self.provenance_ready()?;
        self.world.insert_resource(p::Policy(policy));
        Ok(())
    }
    pub fn set_provenance_exchange_policy(
        &mut self,
        policy: fn(&p::ExchangeInput) -> p::ExchangeDecision,
    ) -> Result<(), String> {
        self.provenance_ready()?;
        self.world.insert_resource(p::ExchangePolicy(policy));
        Ok(())
    }
    pub fn set_inspection_policy(
        &mut self,
        policy: fn(&p::InspectionInput) -> p::InspectionDecision,
    ) -> Result<(), String> {
        self.provenance_ready()?;
        self.world.insert_resource(p::InspectionPolicy(policy));
        Ok(())
    }
    fn provenance_event(&self, event: u64) -> Result<Event, String> {
        let e = self
            .world
            .resource::<Runtime>()
            .events
            .get(usize::try_from(event).map_err(|_| "invalid event")?)
            .ok_or("unknown event")?;
        if e.outcome != Some(Outcome::Refusal) || e.decision.selected != Action::Refuse {
            return Err("inspection requires an actual refusal".into());
        }
        Ok(e.clone())
    }
    fn provenance_own(&self, id: AgentId) -> Result<Agent, String> {
        let entity = self
            .world
            .resource::<Index>()
            .0
            .get(&id)
            .ok_or("unknown individual")?;
        Ok(self.world.get::<Agent>(*entity).unwrap().clone())
    }
    fn local_knowledge(&self, owner: AgentId) -> Vec<p::Knowledge> {
        self.world
            .resource::<crate::audit::RuntimeProvenance>()
            .knowledge
            .get(&owner)
            .into_iter()
            .flatten()
            .cloned()
            .collect()
    }
    /// Recorded retrospective inspection. The refuser voluntarily opens a narrow
    /// historical record; inspectors choose before fallible sensor resolution.
    pub fn inspect_refusal(
        &mut self,
        event: u64,
        inspectors: &[(AgentId, Sensor)],
    ) -> Result<(), String> {
        self.provenance_ready()?;
        let original = self.provenance_event(event)?;
        let actor = original.decision.actor;
        let mut ids = std::collections::BTreeSet::new();
        if inspectors.is_empty() || inspectors.len() > 7 {
            return Err("inspection needs 1..7 inspectors".into());
        }
        for (id, sensor) in inspectors {
            self.provenance_own(*id)?;
            if *id == actor
                || !ids.insert(*id)
                || !(0..=90).contains(&sensor.reliability)
                || sensor.effort > 100
                || sensor.second.is_some()
            {
                return Err("invalid inspection participant/sensor".into());
            }
            if self
                .world
                .resource::<HistoryIndex>()
                .inspection_slots
                .contains_key(&(event, *id))
            {
                return Err("observation slot is immutable; cannot resample or rewrite".into());
            }
        }
        let start = self.world.resource::<Runtime>().tick;
        let profile = self.world.resource::<Intentional>().profiles[&actor].clone();
        let input = p::InspectionInput {
            own: self.provenance_own(actor)?,
            profile,
            event,
            effort: 0,
            quality: 0,
            opening: true,
        };
        let opening = (self.world.resource::<p::InspectionPolicy>().0)(&input);
        let open = opening.input == input && opening.selected;
        let mut decisions = vec![opening];
        let mut roots = Vec::new();
        self.world.resource_mut::<Runtime>().tick += 1;
        if open {
            let index = original
                .participants
                .iter()
                .position(|&id| id == actor)
                .unwrap();
            let (food, hunger) =
                self.world.resource::<Runtime>().circumstances[event as usize][index];
            let truth = food <= 1 && hunger >= 60;
            let seed = self.world.resource::<Foresight>().seed;
            let mut sorted = inspectors.to_vec();
            sorted.sort_by_key(|(id, _)| *id);
            for (owner, sensor) in sorted {
                let input = p::InspectionInput {
                    own: self.provenance_own(owner)?,
                    profile: self.world.resource::<Intentional>().profiles[&owner].clone(),
                    event,
                    effort: sensor.effort,
                    quality: sensor.reliability,
                    opening: false,
                };
                let decision = (self.world.resource::<p::InspectionPolicy>().0)(&input);
                self.world.resource_mut::<Runtime>().tick += 1;
                if decision.input == input && decision.selected {
                    let claim = foresight::reading(seed, event, owner, 0, truth, &sensor);
                    self.world.resource_mut::<Runtime>().tick += u64::from(sensor.effort);
                    let tick = self.world.resource::<Runtime>().tick;
                    let root_id = self
                        .world
                        .resource::<crate::audit::RuntimeProvenance>()
                        .roots
                        .len() as u64;
                    self.world
                        .resource_mut::<crate::audit::RuntimeProvenance>()
                        .roots
                        .push(p::Root {
                            id: root_id,
                            event,
                            observer: owner,
                            claim,
                            quality: sensor.reliability,
                            tick,
                            sensor: sensor.clone(),
                        });
                    self.world
                        .resource_mut::<HistoryIndex>()
                        .inspection_slots
                        .insert((event, owner), root_id);
                    self.provenance_receipt(
                        owner,
                        owner,
                        event,
                        root_id,
                        None,
                        true,
                        claim,
                        sensor.reliability,
                        1 + sensor.effort,
                        false,
                    )?;
                    roots.push(root_id);
                }
                decisions.push(decision);
            }
        }
        let tick = self.world.resource::<Runtime>().tick;
        let mut state = self.world.resource_mut::<crate::audit::RuntimeProvenance>();
        let id = state.windows.len() as u64;
        state.windows.push(p::Window {
            id,
            event,
            tick,
            decisions,
            roots,
            time_spent: (tick - start) as u32,
        });
        Ok(())
    }
    fn exchange_input(
        &self,
        source: AgentId,
        listener: AgentId,
        event: u64,
    ) -> Result<p::ExchangeInput, String> {
        self.provenance_own(listener)?;
        if source == listener {
            return Err("exchange needs two distinct individuals".into());
        }
        Ok(p::ExchangeInput {
            own: self.provenance_own(source)?,
            profile: self.world.resource::<Intentional>().profiles[&source].clone(),
            partner: listener,
            event,
            acquired: self
                .local_knowledge(source)
                .into_iter()
                .rev()
                .find(|k| k.event == event),
        })
    }
    /// One co-present exchange opportunity, not an instruction to send a claim.
    pub fn provenance_exchange(
        &mut self,
        source: AgentId,
        listener: AgentId,
        event: u64,
        attribution_available: bool,
    ) -> Result<Option<u64>, String> {
        self.acquired_exchange(source, listener, event, attribution_available, false)
    }
    /// 008-only native delivery of the same retained structured acquisition.
    /// Origin/content come from the provider's actual local payload, never history.
    pub fn native_acquired_exchange(
        &mut self,
        source: AgentId,
        listener: AgentId,
        event: u64,
        attribution_available: bool,
    ) -> Result<Option<u64>, String> {
        self.provenance_ready()?;
        if !self
            .world
            .contains_resource::<crate::assessment::Assessment>()
        {
            return Err("enable assessment first".into());
        }
        let original = self.provenance_event(event)?;
        if listener == original.decision.actor || !original.participants.contains(&listener) {
            return Err("native acquisition requires the original recipient".into());
        }
        self.acquired_exchange(source, listener, event, attribution_available, true)
    }
    fn acquired_exchange(
        &mut self,
        source: AgentId,
        listener: AgentId,
        event: u64,
        attribution_available: bool,
        native_delivery: bool,
    ) -> Result<Option<u64>, String> {
        self.provenance_ready()?;
        self.provenance_event(event)?;
        let input = self.exchange_input(source, listener, event)?;
        let decision = (self.world.resource::<p::ExchangePolicy>().0)(&input);
        let mut valid = decision.input == input
            && (decision.selected == p::ExchangeAction::Silence || input.acquired.is_some());
        let parent = input
            .acquired
            .as_ref()
            .and_then(|k| {
                self.world
                    .resource::<crate::audit::RuntimeProvenance>()
                    .receipts
                    .get(k.receipt as usize)
            })
            .cloned();
        if decision.selected == p::ExchangeAction::Share
            && parent.as_ref().is_none_or(|r| r.depth >= p::MAX_DEPTH)
        {
            valid = false;
        }
        let start = self.world.resource::<Runtime>().tick;
        self.world.resource_mut::<Runtime>().tick += 1;
        self.world
            .resource_mut::<q::Inquiry>()
            .contact(listener, source);
        let mut receipt = None;
        if valid && decision.selected == p::ExchangeAction::Share {
            let k = input.acquired.as_ref().unwrap();
            let parent = parent.unwrap();
            let cred = self
                .world
                .resource::<Intentional>()
                .credibility
                .get(&listener)
                .and_then(|m| m.get(&source))
                .copied()
                .unwrap_or(0);
            // Credibility adjusts reliability, never lineage; a relay cannot improve the root.
            let quality = (k.quality * (100 + cred.clamp(-40, 40)) / 100).clamp(0, k.quality);
            let attributed = attribution_available && k.known.is_some();
            let id = self.provenance_receipt(
                source,
                listener,
                event,
                parent.actual_root,
                Some(k.receipt),
                attributed,
                k.claim,
                quality,
                1,
                native_delivery,
            )?;
            receipt = Some(id);
        }
        let tick = self.world.resource::<Runtime>().tick;
        let mut state = self.world.resource_mut::<crate::audit::RuntimeProvenance>();
        let id = state.exchanges.len() as u64;
        state.exchanges.push(p::ExchangeRecord {
            id,
            tick,
            decision,
            attribution_available,
            valid,
            receipt,
            time_spent: (tick - start) as u32,
            metadata_only: false,
        });
        Ok(receipt)
    }
    /// Public metadata announcement, selected by the same voluntary source policy.
    /// It discloses no reading and never trains usefulness as though a reply occurred.
    pub fn provenance_offer(
        &mut self,
        source: AgentId,
        listener: AgentId,
        event: u64,
        attribution_available: bool,
    ) -> Result<(), String> {
        self.provenance_ready()?;
        self.provenance_event(event)?;
        let input = self.exchange_input(source, listener, event)?;
        let decision = (self.world.resource::<p::ExchangePolicy>().0)(&input);
        let valid = decision.input == input
            && (decision.selected == p::ExchangeAction::Silence || input.acquired.is_some());
        self.world.resource_mut::<Runtime>().tick += 1;
        self.world
            .resource_mut::<q::Inquiry>()
            .contact(listener, source);
        if valid && decision.selected == p::ExchangeAction::Share {
            let k = input.acquired.unwrap();
            self.world
                .resource_mut::<crate::audit::RuntimeProvenance>()
                .hint(
                    listener,
                    p::Hint {
                        source,
                        event,
                        known: if attribution_available { k.known } else { None },
                        quality: k.quality,
                        receipt: k.receipt,
                    },
                );
        }
        let tick = self.world.resource::<Runtime>().tick;
        let mut state = self.world.resource_mut::<crate::audit::RuntimeProvenance>();
        let id = state.exchanges.len() as u64;
        state.exchanges.push(p::ExchangeRecord {
            id,
            tick,
            decision,
            attribution_available,
            valid,
            receipt: None,
            time_spent: 1,
            metadata_only: true,
        });
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    fn provenance_receipt(
        &mut self,
        communicator: AgentId,
        listener: AgentId,
        event: u64,
        root: u64,
        parent: Option<u64>,
        attributed: bool,
        claim: bool,
        quality: i32,
        time_spent: u32,
        native_delivery: bool,
    ) -> Result<u64, String> {
        let state = self.world.resource::<crate::audit::RuntimeProvenance>();
        let origin = state.roots.get(root as usize).ok_or("invalid root")?;
        if origin.event != event
            || origin.claim != claim
            || quality > origin.quality
            || !(0..=90).contains(&quality)
        {
            return Err("invalid immutable provenance/content".into());
        }
        let depth = if let Some(id) = parent {
            let r = state.receipts.get(id as usize).ok_or("invalid parent")?;
            if r.listener != communicator
                || r.actual_root != root
                || r.event != event
                || r.depth >= p::MAX_DEPTH
            {
                return Err("invalid relay provenance".into());
            }
            r.depth + 1
        } else {
            0
        };
        let id = state.receipts.len() as u64;
        let k = p::Knowledge {
            receipt: id,
            event,
            communicator,
            message: parent.unwrap_or(id),
            claim,
            quality,
            known: attributed.then_some(root),
        };
        let local = self.local_knowledge(listener);
        let settings = state.settings[&listener];
        let mut evaluation = p::evaluate(&local, &k, settings);
        let original = self.provenance_event(event)?;
        let subject = original.decision.actor;
        let participants: Vec<_> = if communicator == listener {
            vec![listener]
        } else {
            vec![communicator, listener]
        };
        let food_before = participants
            .iter()
            .map(|&id| Ok((id, self.provenance_own(id)?.food)))
            .collect::<Result<Vec<_>, String>>()?;
        // Apply the locally computed support to the established belief/revision
        // pipeline only for the original harmed participant. Relay listeners retain
        // bounded acquired testimony; they do not gain someone else's memory.
        let cognition_receipt = if listener != subject && original.participants.contains(&listener)
        {
            Some(
                self.receive_information_support(
                    subject,
                    listener,
                    event,
                    EvidenceKind::Testimony { scarce: claim },
                    Some(quality),
                    Some((
                        evaluation.after,
                        crate::assessment::Item {
                            receipt: if native_delivery {
                                crate::assessment::Receipt::Native(
                                    self.world.resource::<Cognition>().information.len() as u64,
                                )
                            } else {
                                crate::assessment::Receipt::Provenance(id)
                            },
                            event,
                            communicator,
                            message: Some(k.message),
                            origin: k.known.map_or(
                                crate::assessment::Origin::Unknown(communicator),
                                crate::assessment::Origin::Known,
                            ),
                            claim,
                            quality,
                        },
                    )),
                    false,
                )?
                .id,
            )
        } else {
            None
        };
        // The receipt's applied support must agree with its cognitive bridge in 008.
        // Preserve the original provenance novelty classification and utility rule.
        if self
            .world
            .contains_resource::<crate::assessment::Assessment>()
            && let Some(receipt) = cognition_receipt
        {
            let info = &self.world.resource::<Cognition>().information[receipt as usize];
            evaluation.before = info.before.as_ref().map_or(0, |b| b.support);
            evaluation.after = info.after.support;
        }
        let tick = self.world.resource::<Runtime>().tick;
        let food_after = participants
            .iter()
            .map(|&id| Ok((id, self.provenance_own(id)?.food)))
            .collect::<Result<Vec<_>, String>>()?;
        let mut credibility_changes = Vec::new();
        // Fallible comparison, not objective truth. Require two explicitly known
        // distinct observations and strong incoming evidence. Repeats cannot earn
        // credit twice while the bounded comparison key is retained.
        if quality >= 60 && attributed {
            for prior in &local {
                if prior.event != event
                    || prior.known.is_none()
                    || prior.known == k.known
                    || prior.communicator == listener
                {
                    continue;
                }
                let compared = self
                    .world
                    .resource::<crate::audit::RuntimeProvenance>()
                    .comparisons
                    .get(&listener)
                    .is_some_and(|v| {
                        v.iter().any(|c| {
                            c.event == event
                                && c.source == prior.communicator
                                && c.claim == prior.claim
                                && c.evidence == root
                        })
                    });
                if compared {
                    continue;
                }
                let old = self
                    .world
                    .resource::<Intentional>()
                    .credibility
                    .get(&listener)
                    .and_then(|v| v.get(&prior.communicator))
                    .copied()
                    .unwrap_or(0);
                let amount = (if prior.claim == claim { 20 } else { -30 }) * quality / 100;
                let new = (old + amount).clamp(-40, 40);
                self.world
                    .resource_mut::<Intentional>()
                    .credibility
                    .entry(listener)
                    .or_default()
                    .insert(prior.communicator, new);
                credibility_changes.push((prior.communicator, old, new));
                let mut state = self.world.resource_mut::<crate::audit::RuntimeProvenance>();
                let list = state.comparisons.entry(listener).or_default();
                if list.len() == p::CAPACITY {
                    let removed = list.pop_front().unwrap().receipt;
                    state.evictions.push(p::Eviction {
                        owner: listener,
                        collection: "comparisons".into(),
                        receipt: removed,
                    });
                }
                state
                    .comparisons
                    .entry(listener)
                    .or_default()
                    .push_back(p::Comparison {
                        event,
                        source: prior.communicator,
                        claim: prior.claim,
                        evidence: root,
                        receipt: id,
                    });
            }
        }
        let mut state = self.world.resource_mut::<crate::audit::RuntimeProvenance>();
        state.retain(listener, k.clone());
        if listener != communicator {
            state.hint(
                listener,
                p::Hint {
                    source: communicator,
                    event,
                    known: k.known,
                    quality,
                    receipt: id,
                },
            );
        }
        state.receipts.push(p::Receipt {
            id,
            tick,
            event,
            communicator,
            listener,
            actual_root: root,
            parent,
            depth,
            attributed,
            knowledge: k,
            evaluation,
            cognition_receipt,
            food_before,
            food_after,
            time_spent: time_spent + u32::from(cognition_receipt.is_some()),
            credibility_changes,
        });
        Ok(id)
    }
    pub(super) fn provenance_follow(
        &mut self,
        owner: AgentId,
        available: &[AgentId],
        scene: u64,
        meeting: Option<u64>,
    ) {
        for &source in available {
            self.world
                .resource_mut::<q::Inquiry>()
                .contact(owner, source);
        }
        self.advertise_opportunities(owner, available);
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
            .map(super::adaptive::signature)
            .collect();
        for sig in prior {
            self.world.resource_mut::<q::Inquiry>().observe(owner, sig);
        }
        let talk = self.world.resource::<Intentional>();
        let state = self.world.resource::<q::Inquiry>();
        let base = q::Input {
            owner,
            scene,
            hunger: self.provenance_own(owner).unwrap().hunger,
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
                .map(|&id| q::Source {
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
        let state = self.world.resource::<crate::audit::RuntimeProvenance>();
        let input = p::Input {
            base,
            knowledge: self.local_knowledge(owner),
            hints: state
                .hints
                .get(&owner)
                .into_iter()
                .flatten()
                .cloned()
                .collect(),
            settings: state.settings[&owner],
        };
        let mut decision = (self.world.resource::<p::Policy>().0)(&input);
        if let Some(attention) = self.world.get_resource::<crate::attention::Attention>() {
            let mode = attention.mode;
            let assessment = self.world.resource::<crate::assessment::Assessment>();
            let items: Vec<_> = assessment
                .items
                .get(&owner)
                .into_iter()
                .flatten()
                .cloned()
                .collect();
            let basis = crate::attention::project(&input.base.concerns, &items);
            let record = crate::attention::Record {
                query: self
                    .world
                    .resource::<crate::audit::RuntimeProvenance>()
                    .queries
                    .len() as u64,
                inquiry: self.world.resource::<q::Inquiry>().records.len() as u64,
                assessment_end: assessment.records.len(),
                basis,
            };
            if mode == crate::attention::Mode::CurrentNeed {
                decision =
                    (self.world.resource::<crate::attention::Policy>().0)(decision, &record.basis);
            }
            self.world
                .resource_mut::<crate::attention::Attention>()
                .records
                .push(record);
        }
        // Prospective 011 opt-in; absence leaves every previous path untouched.
        if let Some(value) = self
            .world
            .get_resource::<crate::inquiry_value::ValueAudit>()
        {
            let mode = value.mode;
            let assessment = self.world.resource::<crate::assessment::Assessment>();
            let items: Vec<_> = assessment
                .items
                .get(&owner)
                .into_iter()
                .flatten()
                .cloned()
                .collect();
            let record = crate::attention::Record {
                query: self
                    .world
                    .resource::<crate::audit::RuntimeProvenance>()
                    .queries
                    .len() as u64,
                inquiry: self.world.resource::<q::Inquiry>().records.len() as u64,
                assessment_end: assessment.records.len(),
                basis: crate::attention::project(&input.base.concerns, &items),
            };
            decision = match mode {
                crate::inquiry_value::Mode::StatusValue => {
                    (self.world.resource::<crate::inquiry_value::Policy>().0)(
                        decision,
                        &record.basis,
                    )
                }
                _ => crate::inquiry_value::compare(mode, decision, &record.basis),
            };
            self.world
                .resource_mut::<crate::inquiry_value::ValueAudit>()
                .records
                .push(record);
        }
        let (decision, self_evaluation_pending) = self.self_evaluation_decide(decision);
        let valid = decision.input == input
            && decision.decision.input == input.base
            && q::legal(&input.base, decision.decision.selected)
            && decision
                .decision
                .candidates
                .iter()
                .any(|c| c.action == decision.decision.selected)
            && decision
                .decision
                .candidates
                .iter()
                .all(|c| q::legal(&input.base, c.action));
        let selected = if valid {
            decision.decision.selected
        } else {
            q::Action::Pause
        };
        let start = self.world.resource::<Runtime>().tick;
        self.world.resource_mut::<Runtime>().tick += 1;
        let mut receipt = None;
        let mut value = 0;
        let mut before = None;
        let mut after = None;
        let mut legacy_response = None;
        let mut category = q::Novelty::None;
        let mut information_values = Vec::new();
        let mut concern_before = None;
        let mut concern_after = None;
        let food_before: Vec<_> = std::iter::once(owner)
            .chain(available.iter().copied())
            .map(|id| (id, self.provenance_own(id).unwrap().food))
            .collect();
        let query_id = self
            .world
            .resource::<crate::audit::RuntimeProvenance>()
            .queries
            .len() as u64;
        let inquiry_id = self.world.resource::<q::Inquiry>().records.len() as u64;
        if let q::Action::Ask {
            concern,
            source,
            strategy,
        } = selected
        {
            let c = input
                .base
                .concerns
                .iter()
                .find(|c| c.id == concern)
                .unwrap();
            concern_before = Some(c.clone());
            self.world.resource_mut::<Runtime>().tick += u64::from(strategy.cost());
            // Preserve the original refuser's established self-knowledge channel.
            // It remains the 006 resolver when no new acquisition is being relayed.
            let reply_valid;
            if self
                .local_knowledge(source)
                .iter()
                .all(|k| k.event != c.event)
                && self.provenance_event(c.event).unwrap().decision.actor == source
            {
                let reply = self.inquiry_response(owner, source, c, strategy);
                let mut seen = input.base.seen.clone();
                for sig in &reply.signatures {
                    let (kind, mut useful) = q::novelty(&seen, Some(sig));
                    if !input.base.settings.track_novelty {
                        useful = sig.quality.min(40);
                    }
                    if kind == q::Novelty::Conflict
                        || (category != q::Novelty::Conflict && useful >= value)
                    {
                        category = kind;
                    }
                    value = value.max(useful);
                    information_values.push((sig.receipt, kind, useful));
                    seen.push(sig.clone());
                }
                reply_valid = reply.valid;
                if reply_valid {
                    for sig in &reply.signatures {
                        self.world
                            .resource_mut::<q::Inquiry>()
                            .observe(owner, sig.clone());
                    }
                }
                legacy_response = Some(reply);
            } else {
                receipt = self
                    .provenance_exchange(source, owner, c.event, true)
                    .unwrap();
                reply_valid = self
                    .world
                    .resource::<crate::audit::RuntimeProvenance>()
                    .exchanges
                    .last()
                    .unwrap()
                    .valid;
            }
            if let Some(id) = receipt {
                let r = &self
                    .world
                    .resource::<crate::audit::RuntimeProvenance>()
                    .receipts[id as usize];
                value = r.evaluation.incremental_value;
                category = match r.evaluation.kind {
                    p::ValueKind::Shared => q::Novelty::Redundant,
                    p::ValueKind::Conflict => q::Novelty::Conflict,
                    p::ValueKind::Revelation => q::Novelty::Stronger,
                    _ => q::Novelty::New,
                };
                if let Some(cognitive) = r.cognition_receipt {
                    information_values.push((cognitive, category, value));
                }
            }
            if reply_valid {
                let attempted_offer = (strategy == q::Strategy::Evidence)
                    .then(|| {
                        input
                            .base
                            .sources
                            .iter()
                            .find(|s| s.id == source)
                            .and_then(|s| s.offers.iter().rev().find(|o| o.event == c.event))
                            .map(|o| (o.channel, o.quality))
                    })
                    .flatten();
                let (old, new) = self.world.resource_mut::<q::Inquiry>().learn(
                    owner,
                    (concern, source, strategy),
                    value,
                    inquiry_id,
                    attempted_offer,
                );
                before = old;
                after = Some(new);
                let cognitive = receipt
                    .and_then(|id| {
                        self.world
                            .resource::<crate::audit::RuntimeProvenance>()
                            .receipts[id as usize]
                            .cognition_receipt
                    })
                    .or_else(|| {
                        legacy_response
                            .as_ref()
                            .and_then(|r| r.receipts.last().copied())
                    });
                let mut state = self.world.resource_mut::<q::Inquiry>();
                let list = state.episodes.entry(owner).or_default();
                if list.len() == q::EPISODE_CAPACITY {
                    let reference = list.pop_front().unwrap().record;
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
                        record: inquiry_id,
                        concern,
                        source,
                        strategy,
                        novelty: category,
                        value,
                        receipt: cognitive,
                    });
            }
            let cognitive_link = receipt
                .and_then(|id| {
                    self.world
                        .resource::<crate::audit::RuntimeProvenance>()
                        .receipts[id as usize]
                        .cognition_receipt
                })
                .or_else(|| {
                    legacy_response
                        .as_ref()
                        .and_then(|r| r.receipts.last().copied())
                });
            let mut state = self.world.resource_mut::<Concerns>();
            let actual = state
                .items
                .get_mut(&owner)
                .unwrap()
                .iter_mut()
                .find(|c| c.id == concern)
                .unwrap();
            let old = actual.clone();
            actual.attempts = actual.attempts.saturating_add(1);
            if value == 0 {
                actual.failures = actual.failures.saturating_add(1);
            }
            let new = actual.clone();
            concern_after = Some(new.clone());
            state.change(
                "provenance inquiry outcome",
                Some(old),
                Some(new),
                cognitive_link,
                None,
            );
        }
        let tick = self.world.resource::<Runtime>().tick;
        let food_after = food_before
            .iter()
            .map(|&(id, _)| (id, self.provenance_own(id).unwrap().food))
            .collect();
        let attempted_base =
            (decision.input.base != decision.decision.input).then_some(decision.input.base);
        let outcome_record = q::Record {
            id: inquiry_id,
            tick,
            meeting,
            decision: decision.decision,
            valid,
            response: legacy_response,
            novelty: category,
            realized_value: value,
            information_values,
            cell_before: before,
            cell_after: after,
            concern_before,
            concern_after,
            time_spent: (tick - start) as u32,
            food_before,
            food_after,
        };
        if self_evaluation_pending.is_some() {
            self.self_evaluation_finish(
                crate::self_evaluation::Experience {
                    inquiry: outcome_record.id,
                    owner,
                    selected: outcome_record.decision.selected,
                    valid_learning: outcome_record.valid && outcome_record.cell_after.is_some(),
                    status_before: outcome_record.concern_before.as_ref().map(|c| c.status),
                    status_after: outcome_record.concern_after.as_ref().map(|c| c.status),
                    receipt_acquired: !outcome_record.information_values.is_empty(),
                    novelty: outcome_record.novelty,
                    novelty_value: outcome_record.realized_value,
                    time: outcome_record.time_spent,
                },
                self_evaluation_pending,
            );
        }
        self.world
            .resource_mut::<q::Inquiry>()
            .records
            .push(outcome_record);
        let context = self
            .world
            .resource_mut::<Contexts>()
            .0
            .intern(LocalContext {
                knowledge: decision.input.knowledge,
                hints: decision.input.hints,
                settings: decision.input.settings,
            });
        self.world
            .resource_mut::<crate::audit::RuntimeProvenance>()
            .queries
            .push(QueryRef {
                id: QueryId(query_id),
                inquiry: InquiryId(inquiry_id),
                owner,
                observed_at: start,
                completed_at: tick,
                context,
                factors: decision.factors,
                receipt: receipt.map(ReceiptId),
                attempted_base,
            });
    }
}
