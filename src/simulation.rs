use crate::cognition::*;
use crate::intentional::{
    self, Conversation, Intentional, Profile, TalkAction, TalkDecision, TalkInput, TalkMemory,
    TalkPolicy, TalkRecord,
};
use crate::{behavior::*, model::*};
use bevy_ecs::{
    prelude::{Entity, Resource, Schedule, World},
    schedule::SingleThreadedExecutor,
};
use std::collections::BTreeMap;

#[derive(Resource, Default)]
struct Index(BTreeMap<AgentId, Entity>);
#[derive(Resource, Default)]
struct Runtime {
    tick: u64,
    scenes: Vec<Scene>,
    events: Vec<Event>,
    circumstances: Vec<[(u32, i32); 2]>,
    initial_trust: BTreeMap<AgentId, BTreeMap<AgentId, i32>>,
    conversation_cursor: usize,
}
#[derive(Resource)]
struct Policy(pub fn(&Agent, &Observation) -> Decision);

pub struct Simulation {
    world: World,
    schedule: Schedule,
}
impl Simulation {
    pub fn new(agents: Vec<Agent>) -> Result<Self, String> {
        let mut world = World::new();
        let mut index = Index::default();
        let initial_trust = agents.iter().map(|a| (a.id, a.trust.clone())).collect();
        for agent in agents {
            if !agent.memories.is_empty() || agent.trust.values().any(|v| !(-100..=100).contains(v))
            {
                return Err("new simulations require empty episodic memory and trust in -100..=100; snapshot restoration is not implemented".into());
            }
            if index.0.contains_key(&agent.id) {
                return Err("duplicate agent ID".into());
            }
            if ![
                agent.hunger,
                agent.generosity,
                agent.caution,
                agent.expectation,
            ]
            .iter()
            .all(|v| (0..=100).contains(v))
            {
                return Err("agent attributes must be in 0..=100".into());
            }
            index.0.insert(agent.id, world.spawn(agent).id());
        }
        world.insert_resource(index);
        world.insert_resource(Runtime {
            initial_trust,
            ..Default::default()
        });
        world.insert_resource(Cognition::default());
        world.insert_resource(Policy(utility_policy));
        let mut schedule = Schedule::default();
        schedule.set_executor(SingleThreadedExecutor::new());
        schedule.add_systems(advance);
        Ok(Self { world, schedule })
    }
    pub fn set_policy(&mut self, policy: fn(&Agent, &Observation) -> Decision) {
        self.world.resource_mut::<Policy>().0 = policy;
    }
    pub fn agents(&self) -> Vec<Agent> {
        self.world
            .resource::<Index>()
            .0
            .values()
            .map(|e| self.world.get::<Agent>(*e).unwrap().clone())
            .collect()
    }
    /// Experiment-only intervention; learned state is always retained.
    pub fn set_circumstances(
        &mut self,
        id: AgentId,
        food: u32,
        hunger: i32,
        generosity: i32,
        caution: i32,
    ) -> Result<(), String> {
        if ![hunger, generosity, caution]
            .iter()
            .all(|v| (0..=100).contains(v))
        {
            return Err("invalid intervention bounds".into());
        }
        if self
            .world
            .resource::<Runtime>()
            .scenes
            .iter()
            .any(|s| s.outcome.is_none())
        {
            return Err("intervene only between scenes".into());
        }
        let entity = *self
            .world
            .resource::<Index>()
            .0
            .get(&id)
            .ok_or("unknown agent")?;
        let mut agent = self.world.get_mut::<Agent>(entity).unwrap();
        agent.food = food;
        agent.hunger = hunger;
        agent.generosity = generosity;
        agent.caution = caution;
        Ok(())
    }
    pub fn add_scene(
        &mut self,
        participants: [AgentId; 2],
        amount: u32,
        max_turns: u32,
    ) -> Result<u64, String> {
        let index = &self.world.resource::<Index>().0;
        if participants[0] == participants[1] || participants.iter().any(|p| !index.contains_key(p))
        {
            return Err("scene requires two distinct existing participants".into());
        }
        if amount == 0 || max_turns == 0 || max_turns > 100 {
            return Err("amount must be positive and turn budget in 1..=100".into());
        }
        let mut runtime = self.world.resource_mut::<Runtime>();
        if runtime
            .scenes
            .iter()
            .any(|s| s.outcome.is_none() && s.participants.iter().any(|p| participants.contains(p)))
        {
            return Err("participant already in an active scene".into());
        }
        let id = runtime.scenes.len() as u64;
        runtime.scenes.push(Scene {
            id,
            participants,
            amount,
            max_turns,
            turns: 0,
            signal: None,
            outcome: None,
        });
        Ok(id)
    }
    pub fn run(&mut self) {
        while self
            .world
            .resource::<Runtime>()
            .scenes
            .iter()
            .any(|s| s.outcome.is_none())
        {
            self.schedule.run(&mut self.world);
        }
        if self.world.contains_resource::<Intentional>() {
            let start = self.world.resource::<Runtime>().conversation_cursor;
            let end = self.world.resource::<Runtime>().events.len();
            self.world.resource_mut::<Runtime>().conversation_cursor = end;
            for event in start..end {
                if self.world.resource::<Runtime>().events[event].outcome == Some(Outcome::Refusal)
                {
                    self.converse(event as u64);
                }
            }
        }
    }
    pub fn events(&self) -> Vec<Event> {
        self.world.resource::<Runtime>().events.clone()
    }
    pub fn scenes(&self) -> Vec<Scene> {
        self.world.resource::<Runtime>().scenes.clone()
    }

    pub fn cognition(&self) -> Cognition {
        self.world.resource::<Cognition>().clone()
    }

    /// Opt-in at a scene boundary; old refusals are not retroactively reopened.
    pub fn enable_intentional(&mut self) -> Result<(), String> {
        self.require_idle()?;
        if self.world.contains_resource::<Intentional>() {
            return Ok(());
        }
        let profiles = self
            .agents()
            .iter()
            .map(|a| {
                (
                    a.id,
                    Profile {
                        privacy: a.caution,
                        relationship_goal: a.generosity,
                        honesty: a.expectation,
                        request_cost: 40,
                    },
                )
            })
            .collect();
        self.world.insert_resource(Intentional {
            profiles,
            ..Default::default()
        });
        self.world.insert_resource(TalkPolicy(intentional::policy));
        let end = self.world.resource::<Runtime>().events.len();
        self.world.resource_mut::<Runtime>().conversation_cursor = end;
        Ok(())
    }

    pub fn set_communication_profile(
        &mut self,
        id: AgentId,
        profile: Profile,
    ) -> Result<(), String> {
        self.require_idle()?;
        if ![
            profile.privacy,
            profile.relationship_goal,
            profile.honesty,
            profile.request_cost,
        ]
        .iter()
        .all(|n| (0..=100).contains(n))
        {
            return Err("invalid communication profile".into());
        }
        let mut state = self
            .world
            .get_resource_mut::<Intentional>()
            .ok_or("enable intentional mode first")?;
        let old = state.profiles.get_mut(&id).ok_or("unknown agent")?;
        *old = profile;
        Ok(())
    }

    pub fn set_communication_policy(
        &mut self,
        policy: fn(&TalkInput) -> TalkDecision,
    ) -> Result<(), String> {
        self.require_idle()?;
        self.world
            .get_resource_mut::<TalkPolicy>()
            .ok_or("enable intentional mode first")?
            .0 = policy;
        Ok(())
    }

    pub fn intentional(&self) -> Option<Intentional> {
        self.world.get_resource::<Intentional>().cloned()
    }

    fn converse(&mut self, event: u64) {
        use TalkAction::*;
        let original = self.world.resource::<Runtime>().events[event as usize].clone();
        let speaker = original.decision.actor;
        let listener = *original
            .participants
            .iter()
            .find(|&&id| id != speaker)
            .unwrap();
        let ids = [speaker, listener];
        let entities = ids.map(|id| self.world.resource::<Index>().0[&id]);
        let speaker_index = original
            .participants
            .iter()
            .position(|&id| id == speaker)
            .unwrap();
        let (food, hunger) =
            self.world.resource::<Runtime>().circumstances[event as usize][speaker_index];
        let scarce = food <= 1 && hunger >= 60;
        let conversation = self.world.resource::<Intentional>().conversations.len() as u64;
        let first_record = self.world.resource::<Intentional>().records.len();
        let mut last = None;
        let mut previous_claim = None;
        let mut outcome = "TurnLimit";
        for turn in 0..4 {
            let actor_index = turn % 2;
            let actor = ids[actor_index];
            let partner = ids[1 - actor_index];
            let state = self.world.resource::<Intentional>();
            let credibility = |owner, other| {
                state
                    .credibility
                    .get(&owner)
                    .and_then(|m| m.get(&other))
                    .copied()
                    .unwrap_or(0)
            };
            let credibility_before = credibility(listener, speaker);
            let input = TalkInput {
                own: self
                    .world
                    .get::<Agent>(entities[actor_index])
                    .unwrap()
                    .clone(),
                profile: state.profiles[&actor].clone(),
                partner,
                event,
                is_refuser: actor == speaker,
                own_historical_scarcity: (actor == speaker).then_some(scarce),
                last,
                belief: self
                    .world
                    .resource::<Cognition>()
                    .beliefs
                    .get(&actor)
                    .and_then(|b| b.iter().find(|b| b.event == event))
                    .cloned(),
                credibility: credibility(actor, partner),
                memories: state
                    .memories
                    .get(&actor)
                    .map_or_else(Vec::new, |m| m.iter().cloned().collect()),
                turns_left: (4 - turn) as u32,
            };
            let decision = (self.world.resource::<TalkPolicy>().0)(&input);
            let valid =
                decision.input == input && intentional::legal(&input).contains(&decision.selected);
            let food_before = entities.map(|e| self.world.get::<Agent>(e).unwrap().food);
            let mut claim = None;
            let mut information = None;
            let mut verified_claim = None;
            let mut credibility_after = credibility_before;
            if valid {
                let kind = match decision.selected {
                    Explain => {
                        claim = Some(scarce);
                        Some(EvidenceKind::Testimony { scarce })
                    }
                    Mislead => {
                        claim = Some(true);
                        Some(EvidenceKind::Testimony { scarce: true })
                    }
                    ProvideEvidence => {
                        claim = Some(scarce);
                        Some(EvidenceKind::Disclosure)
                    }
                    _ => None,
                };
                if let Some(kind) = kind {
                    let receipt = self
                        .receive_information(
                            speaker,
                            listener,
                            event,
                            kind,
                            Some((50 + credibility_before).clamp(10, 90)),
                        )
                        .expect("validated conversation participants");
                    information = Some(receipt.id);
                    if decision.selected == ProvideEvidence {
                        if let Some(old_claim) = previous_claim {
                            let consistent = old_claim == scarce;
                            verified_claim = Some(consistent);
                            credibility_after = (credibility_before
                                + if consistent { 20 } else { -30 })
                            .clamp(-40, 40);
                        }
                    } else {
                        previous_claim = claim;
                    }
                }
            }
            // Receiving information already consumes one tick; other turns do too.
            if information.is_none() {
                self.world.resource_mut::<Runtime>().tick += 1;
            }
            let tick = self.world.resource::<Runtime>().tick;
            let food_after = entities.map(|e| self.world.get::<Agent>(e).unwrap().food);
            let mut state = self.world.resource_mut::<Intentional>();
            let record_id = state.records.len() as u64;
            if valid {
                if verified_claim.is_some() {
                    state
                        .credibility
                        .entry(listener)
                        .or_default()
                        .insert(speaker, credibility_after);
                }
                for owner in ids {
                    // A listener sees testimony, never the speaker's private intent to lie.
                    let public_action = if owner != actor && decision.selected == Mislead {
                        Explain
                    } else {
                        decision.selected
                    };
                    intentional::remember(
                        &mut state,
                        owner,
                        TalkMemory {
                            record: record_id,
                            event,
                            partner: if owner == speaker { listener } else { speaker },
                            actor,
                            action: public_action,
                            response_to: last,
                            claim,
                            verified_claim,
                        },
                    );
                }
            }
            state.records.push(TalkRecord {
                id: record_id,
                tick,
                conversation,
                decision: decision.clone(),
                valid,
                claim,
                objectively_true: claim.map(|c| c == scarce),
                verifiable: valid && decision.selected == ProvideEvidence,
                information,
                credibility_before,
                credibility_after,
                verified_claim,
                time_spent: 1,
                food_before,
                food_after,
            });
            if !valid {
                outcome = "InvalidAction";
                break;
            }
            if actor == listener && decision.selected == Silence {
                outcome = "Closed";
                break;
            }
            last = Some(if decision.selected == Mislead {
                Explain
            } else {
                decision.selected
            });
        }
        let mut state = self.world.resource_mut::<Intentional>();
        let end_record = state.records.len();
        state.conversations.push(Conversation {
            id: conversation,
            event,
            participants: ids,
            first_record,
            end_record,
            outcome: outcome.into(),
        });
    }

    fn require_idle(&self) -> Result<(), String> {
        if self
            .world
            .resource::<Runtime>()
            .scenes
            .iter()
            .any(|s| s.outcome.is_none())
        {
            Err("finish active resource scenes first".into())
        } else {
            Ok(())
        }
    }

    /// An explicit phase; no spontaneous time-dependent metabolism.
    pub fn consume(&mut self, id: AgentId) -> Result<Consumption, String> {
        self.require_idle()?;
        let entity = *self
            .world
            .resource::<Index>()
            .0
            .get(&id)
            .ok_or("unknown agent")?;
        self.world.resource_mut::<Runtime>().tick += 1;
        let tick = self.world.resource::<Runtime>().tick;
        let mut a = self.world.get_mut::<Agent>(entity).unwrap();
        let food_before = a.food;
        let hunger_before = a.hunger;
        let consumed = u32::from(a.food > 0 && a.hunger > 0);
        a.food -= consumed;
        if consumed > 0 {
            a.hunger = (a.hunger - 40).max(0);
        }
        let record = Consumption {
            tick,
            actor: id,
            food_before,
            food_after: a.food,
            hunger_before,
            hunger_after: a.hunger,
            consumed,
        };
        self.world
            .resource_mut::<Cognition>()
            .consumption
            .push(record.clone());
        Ok(record)
    }

    /// One-turn communication scene. Claims are not truth-checked. Disclosure is
    /// an explicit voluntary release of an instrumented historical observation.
    pub fn communicate(
        &mut self,
        speaker: AgentId,
        listener: AgentId,
        event: u64,
        kind: EvidenceKind,
    ) -> Result<InformationScene, String> {
        self.receive_information(speaker, listener, event, kind, None)
    }

    fn receive_information(
        &mut self,
        speaker: AgentId,
        listener: AgentId,
        event: u64,
        kind: EvidenceKind,
        testimony_weight: Option<i32>,
    ) -> Result<InformationScene, String> {
        self.require_idle()?;
        let runtime = self.world.resource::<Runtime>();
        let original = runtime
            .events
            .get(usize::try_from(event).map_err(|_| "invalid event")?)
            .ok_or("unknown event")?
            .clone();
        if speaker == listener
            || original.decision.actor != speaker
            || original.decision.selected != Action::Refuse
            || original.outcome != Some(Outcome::Refusal)
            || !original.participants.contains(&listener)
        {
            return Err("information must concern the speaker's refusal to this listener".into());
        }
        let speaker_index = original
            .participants
            .iter()
            .position(|&id| id == speaker)
            .unwrap();
        let scarce = match kind {
            EvidenceKind::Testimony { scarce } => scarce,
            EvidenceKind::Disclosure => {
                let (food, hunger) = runtime.circumstances[event as usize][speaker_index];
                food <= 1 && hunger >= 60
            }
        };
        let entity = self.world.resource::<Index>().0[&listener];
        let mut agent = self.world.get::<Agent>(entity).unwrap().clone();
        let trust_before = *agent.trust.get(&speaker).unwrap_or(&0);
        // Freeze credibility for this claim at first contact: a claim cannot
        // bootstrap its own credibility through the trust revision it causes.
        let credibility_trust = self
            .world
            .resource::<Cognition>()
            .information
            .iter()
            .find(|i| i.event == event && i.listener == listener)
            .map_or(trust_before, |i| i.trust_before);
        let reliability = match kind {
            EvidenceKind::Testimony { .. } => {
                testimony_weight.unwrap_or_else(|| (50 + credibility_trust / 2).clamp(0, 90))
            }
            EvidenceKind::Disclosure => 100,
        };
        let mut cognition = self.world.remove_resource::<Cognition>().unwrap();
        let id = cognition.information.len() as u64;
        let beliefs = cognition.beliefs.entry(listener).or_default();
        let before = beliefs.iter().find(|b| b.event == event).cloned();
        let after = revise_belief(
            before.as_ref(),
            Belief {
                event,
                subject: speaker,
                support: if scarce { reliability } else { -reliability },
                evidence: id,
            },
        );
        beliefs.retain(|b| b.event != event);
        if beliefs.len() == MEMORY_CAPACITY {
            beliefs.pop_front();
        }
        beliefs.push_back(after.clone());
        let mut revision = None;
        if let Some(memory) = agent.memories.iter_mut().find(|m| m.event == event) {
            let old = memory.clone();
            let initial = &original.interpretations[1 - speaker_index];
            if after.support >= 60 {
                memory.interpretation = Interpretation::PossibleSelfProtection;
                memory.valence = 0;
            } else {
                *memory = initial.clone();
            }
            if *memory != old {
                revision = Some(Revision {
                    before: old,
                    after: memory.clone(),
                });
            }
        }
        // Replay the ordered ledger with revised contributions substituted. This
        // preserves clamping semantics even for saturated trust and evicted episodes.
        if let Some(change) = &revision {
            let mut replacements = BTreeMap::new();
            for info in &cognition.information {
                if info.listener == listener
                    && info.speaker == speaker
                    && let Some(r) = &info.revision
                {
                    replacements.insert(info.event, r.after.valence);
                }
            }
            replacements.insert(event, change.after.valence);
            let runtime = self.world.resource::<Runtime>();
            let mut trust = *runtime.initial_trust[&listener].get(&speaker).unwrap_or(&0);
            for e in &runtime.events {
                if let Some(m) = e.interpretations.iter().find(|m| m.partner == speaker)
                    && e.participants.contains(&listener)
                {
                    trust = (trust + replacements.get(&e.id).copied().unwrap_or(m.valence))
                        .clamp(-100, 100);
                }
            }
            agent.trust.insert(speaker, trust);
        }
        self.world.resource_mut::<Runtime>().tick += 1;
        let record = InformationScene {
            id,
            tick: self.world.resource::<Runtime>().tick,
            speaker,
            listener,
            event,
            kind,
            scarce,
            reliability,
            before,
            after,
            revision,
            trust_before,
            trust_after: *agent.trust.get(&speaker).unwrap_or(&0),
            turns: 1,
            terminated: true,
        };
        cognition.information.push(record.clone());
        self.world.insert_resource(cognition);
        *self.world.get_mut::<Agent>(entity).unwrap() = agent;
        Ok(record)
    }
}

fn advance(world: &mut World) {
    let mut runtime = world.remove_resource::<Runtime>().unwrap();
    runtime.tick += 1;
    let policy = world.resource::<Policy>().0;
    for scene in &mut runtime.scenes {
        if scene.outcome.is_some() {
            continue;
        }
        let ids = scene.participants;
        let entities = ids.map(|id| world.resource::<Index>().0[&id]);
        let mut agents = entities.map(|e| world.get::<Agent>(e).unwrap().clone());
        let actor_index = (scene.turns % 2) as usize;
        let partner_index = 1 - actor_index;
        let view = Observation {
            partner: ids[partner_index],
            last_signal: scene.signal,
            amount: scene.amount,
            turns_left: scene.max_turns - scene.turns,
        };
        let decision = policy(&agents[actor_index], &view);
        let before = agents.each_ref().map(|a| a.food);
        runtime
            .circumstances
            .push(agents.each_ref().map(|a| (a.food, a.hunger)));
        let legal = decision.actor == ids[actor_index]
            && decision.observation == view
            && legal_actions(&agents[actor_index], &view).contains(&decision.selected);
        let mut transferred = 0;
        if !legal {
            scene.outcome = Some(Outcome::Inability);
        } else {
            match decision.selected {
                Action::Accept => {
                    if scene
                        .signal
                        .is_some_and(|s| s.actor == ids[partner_index] && s.action == Action::Offer)
                        && agents[partner_index].food >= scene.amount
                        && agents[actor_index].food.checked_add(scene.amount).is_some()
                    {
                        agents[partner_index].food -= scene.amount;
                        agents[actor_index].food += scene.amount;
                        transferred = scene.amount;
                        scene.outcome = Some(Outcome::Agreement);
                    } else {
                        scene.outcome = Some(Outcome::Inability);
                    }
                }
                Action::Refuse => scene.outcome = Some(Outcome::Refusal),
                Action::Leave => scene.outcome = Some(Outcome::Withdrawal),
                Action::Offer | Action::Request => {}
            }
        }
        scene.turns += 1;
        if scene.outcome.is_none() && scene.turns >= scene.max_turns {
            scene.outcome = Some(Outcome::Timeout);
        }
        let signal = Signal {
            actor: ids[actor_index],
            action: decision.selected,
        };
        scene.signal = Some(signal);
        let event_id = runtime.events.len() as u64;
        let interpretations = [0, 1].map(|i| {
            interpret(
                &agents[i],
                ids[1 - i],
                event_id,
                signal,
                transferred,
                scene.outcome == Some(Outcome::Inability),
            )
        });
        let after = agents.each_ref().map(|a| a.food);
        for i in 0..2 {
            remember(&mut agents[i], interpretations[i].clone());
            *world.get_mut::<Agent>(entities[i]).unwrap() = agents[i].clone();
        }
        runtime.events.push(Event {
            id: event_id,
            tick: runtime.tick,
            scene: scene.id,
            participants: ids,
            decision,
            balances_before: before,
            balances_after: after,
            transferred,
            outcome: scene.outcome,
            interpretations,
        });
    }
    world.insert_resource(runtime);
}
