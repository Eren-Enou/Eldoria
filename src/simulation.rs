use crate::cognition::*;
use crate::concerns::{
    self, ConcernPolicy, Concerns, Creation, FollowAction, FollowDecision, FollowInput,
    FollowRecord, Status,
};
use crate::foresight::{
    self, Context, ForecastRecord, Foresight, Plan, PredictionError, Predictor, Reading, Sensor,
    Settings,
};
use crate::history::HistoryIndex;
use crate::intentional::{
    self, Conversation, Intentional, Profile, TalkAction, TalkDecision, TalkInput, TalkMemory,
    TalkPolicy, TalkRecord,
};
use crate::{behavior::*, model::*};
use bevy_ecs::{
    prelude::{Entity, Resource, Schedule, World},
    schedule::SingleThreadedExecutor,
};
use std::collections::{BTreeMap, BTreeSet};
mod adaptive;
mod attention;
mod provenance;

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
    active: BTreeSet<usize>,
    busy: BTreeSet<AgentId>,
}
#[derive(Resource)]
struct Policy(pub fn(&Agent, &Observation) -> Decision);

pub struct Simulation {
    world: World,
    schedule: Schedule,
}
impl Simulation {
    /// 009 retention is prospective, opt-in, and never changes the Grouped rule/cap.
    pub fn enable_retention(&mut self, method: crate::retention::Method) -> Result<(), String> {
        self.require_idle()?;
        let assessment = self
            .world
            .get_resource::<crate::assessment::Assessment>()
            .ok_or("enable Grouped assessment first")?;
        if assessment.method != crate::assessment::Method::Grouped {
            return Err("retention requires finalized Grouped assessment".into());
        }
        if let Some(state) = self.world.get_resource::<crate::retention::Retention>() {
            return if state.method == method {
                Ok(())
            } else {
                Err("retention method is fixed for a run".into())
            };
        }
        let first_assessment = assessment.records.len() as u64;
        self.world.insert_resource(crate::retention::Retention {
            method,
            first_assessment,
            records: vec![],
        });
        self.world
            .insert_resource(crate::retention::Policy(crate::retention::policy));
        Ok(())
    }
    pub fn retention(&self) -> Option<crate::retention::Retention> {
        self.world
            .get_resource::<crate::retention::Retention>()
            .cloned()
    }
    pub fn set_retention_policy(
        &mut self,
        policy: fn(&crate::retention::Input) -> crate::retention::Decision,
    ) -> Result<(), String> {
        self.require_idle()?;
        if !self
            .world
            .contains_resource::<crate::retention::Retention>()
        {
            return Err("enable retention first".into());
        }
        self.world.insert_resource(crate::retention::Policy(policy));
        Ok(())
    }
    /// Prospective only: no import of old receipts, beliefs or forgotten acquisitions.
    pub fn enable_assessment(&mut self, method: crate::assessment::Method) -> Result<(), String> {
        self.require_idle()?;
        if !self
            .world
            .contains_resource::<crate::audit::RuntimeProvenance>()
        {
            return Err("enable provenance first".into());
        }
        if self
            .world
            .contains_resource::<crate::assessment::Assessment>()
        {
            return if self
                .world
                .resource::<crate::assessment::Assessment>()
                .method
                == method
            {
                Ok(())
            } else {
                Err("assessment method is fixed for a run".into())
            };
        }
        self.world
            .insert_resource(crate::assessment::Assessment::new(method));
        self.world
            .insert_resource(crate::assessment::Assessor(method.rule()));
        Ok(())
    }
    pub fn assessment(&self) -> Option<crate::assessment::Assessment> {
        self.world
            .get_resource::<crate::assessment::Assessment>()
            .cloned()
    }
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
        world.insert_resource(HistoryIndex::default());
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
        if !self.world.resource::<Runtime>().active.is_empty() {
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
        if participants.iter().any(|p| runtime.busy.contains(p)) {
            return Err("participant already in an active scene".into());
        }
        let id = runtime.scenes.len() as u64;
        runtime.active.insert(id as usize);
        runtime.busy.extend(participants);
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
        while !self.world.resource::<Runtime>().active.is_empty() {
            self.schedule.run(&mut self.world);
        }
        if self.world.contains_resource::<Concerns>() {
            self.follow_opportunities();
        }
        if self.world.contains_resource::<Intentional>() {
            let start = self.world.resource::<Runtime>().conversation_cursor;
            let end = self.world.resource::<Runtime>().events.len();
            self.world.resource_mut::<Runtime>().conversation_cursor = end;
            for event in start..end {
                if self.world.resource::<Runtime>().events[event].outcome == Some(Outcome::Refusal)
                {
                    self.converse(event as u64);
                    self.capture_concern(event as u64);
                }
            }
        }
    }
    pub fn events(&self) -> Vec<Event> {
        self.world.resource::<Runtime>().events.clone()
    }
    /// Observer-only integrity check. No policy can request an index or recover
    /// forgotten cognition through it. Rebuilding is deliberately an offline scan.
    pub fn validate_history_indexes(&self) -> Result<(), String> {
        let runtime = self.world.resource::<Runtime>();
        let active: BTreeSet<_> = runtime
            .scenes
            .iter()
            .enumerate()
            .filter(|(_, s)| s.outcome.is_none())
            .map(|(i, _)| i)
            .collect();
        let busy: BTreeSet<_> = active
            .iter()
            .flat_map(|&i| runtime.scenes[i].participants)
            .collect();
        if active != runtime.active || busy != runtime.busy {
            return Err("active scene index disagrees with history".into());
        }
        let mut rebuilt = HistoryIndex::default();
        for event in &runtime.events {
            rebuilt.event(event, &runtime.initial_trust);
        }
        for info in &self.world.resource::<Cognition>().information {
            rebuilt.information(info);
        }
        if let Some(state) = self.world.get_resource::<crate::audit::RuntimeProvenance>() {
            for root in &state.roots {
                rebuilt
                    .inspection_slots
                    .insert((root.event, root.observer), root.id);
            }
        }
        if rebuilt != *self.world.resource::<HistoryIndex>() {
            return Err("derived history index disagrees with authoritative records".into());
        }
        Ok(())
    }
    /// Observer measurements, not per-agent knowledge or a policy interface.
    pub fn history_index_counts(&self) -> BTreeMap<String, usize> {
        let mut counts = self.world.resource::<HistoryIndex>().counts();
        counts.insert(
            "active_scenes".into(),
            self.world.resource::<Runtime>().active.len(),
        );
        counts.insert(
            "busy_participants".into(),
            self.world.resource::<Runtime>().busy.len(),
        );
        counts
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

    pub fn enable_foresight(&mut self, seed: u64, sensor: Sensor) -> Result<(), String> {
        self.require_idle()?;
        if !(0..=90).contains(&sensor.reliability) || sensor.effort > 100 {
            return Err("sensor reliability must be 0..90 and effort 0..100".into());
        }
        if self.world.contains_resource::<Foresight>() {
            return Err("foresight already enabled".into());
        }
        self.enable_intentional()?;
        let settings = self
            .agents()
            .iter()
            .map(|a| (a.id, Settings::default()))
            .collect();
        self.world.insert_resource(Foresight {
            seed,
            sensor,
            settings,
            expectations: Default::default(),
            forecasts: vec![],
            errors: vec![],
            readings: vec![],
        });
        self.world.insert_resource(Predictor(foresight::predict));
        Ok(())
    }
    pub fn foresight(&self) -> Option<Foresight> {
        self.world.get_resource::<Foresight>().cloned()
    }
    pub fn set_prediction_settings(
        &mut self,
        id: AgentId,
        settings: Settings,
    ) -> Result<(), String> {
        self.require_idle()?;
        if ![settings.challenge_prior, settings.answer_prior]
            .iter()
            .all(|v| (0..=100).contains(v))
        {
            return Err("prediction priors must be 0..100".into());
        }
        let mut state = self
            .world
            .get_resource_mut::<Foresight>()
            .ok_or("enable foresight first")?;
        *state.settings.get_mut(&id).ok_or("unknown agent")? = settings;
        Ok(())
    }
    pub fn set_predictor(
        &mut self,
        predictor: fn(&TalkInput, &Context) -> Plan,
    ) -> Result<(), String> {
        self.require_idle()?;
        self.world
            .get_resource_mut::<Predictor>()
            .ok_or("enable foresight first")?
            .0 = predictor;
        Ok(())
    }

    /// Enables unfinished intent at a boundary without retroactive concerns.
    pub fn enable_concerns(&mut self) -> Result<(), String> {
        self.require_idle()?;
        if !self.world.contains_resource::<Foresight>() {
            return Err("enable foresight first".into());
        }
        if self.world.contains_resource::<Concerns>() {
            return Ok(());
        }
        self.world.insert_resource(Concerns {
            pursuit_costs: self.agents().iter().map(|a| (a.id, 20)).collect(),
            scene_cursor: self.world.resource::<Runtime>().scenes.len(),
            ..Default::default()
        });
        self.world.insert_resource(ConcernPolicy(concerns::policy));
        Ok(())
    }
    pub fn concerns(&self) -> Option<Concerns> {
        self.world.get_resource::<Concerns>().cloned()
    }
    pub fn set_follow_cost(&mut self, owner: AgentId, cost: u32) -> Result<(), String> {
        self.require_idle()?;
        if cost > 100 {
            return Err("follow cost must be 0..100".into());
        }
        let mut state = self
            .world
            .get_resource_mut::<Concerns>()
            .ok_or("enable concerns first")?;
        *state.pursuit_costs.get_mut(&owner).ok_or("unknown agent")? = cost;
        Ok(())
    }
    pub fn set_concern_policy(
        &mut self,
        policy: fn(&FollowInput) -> FollowDecision,
    ) -> Result<(), String> {
        self.require_idle()?;
        self.world
            .get_resource_mut::<ConcernPolicy>()
            .ok_or("enable concerns first")?
            .0 = policy;
        Ok(())
    }
    fn capture_concern(&mut self, event: u64) {
        if !self.world.contains_resource::<Concerns>() {
            return;
        }
        let original = &self.world.resource::<Runtime>().events[event as usize];
        let target = original.decision.actor;
        let owner = *original
            .participants
            .iter()
            .find(|&&id| id != target)
            .unwrap();
        let actor = self
            .world
            .get::<Agent>(self.world.resource::<Index>().0[&owner])
            .unwrap();
        let importance = (self.world.resource::<Intentional>().profiles[&owner].relationship_goal
            + actor.hunger / 5)
            .clamp(0, 100);
        let belief = self
            .world
            .resource::<Cognition>()
            .beliefs
            .get(&owner)
            .and_then(|b| b.iter().find(|b| b.event == event));
        let support = belief.map(|b| b.support);
        let creation = Creation {
            owner,
            target,
            event,
            scene: original.scene,
            importance,
            support,
            receipt: belief.map(|b| b.evidence),
            retained: concerns::worth_retaining(importance, support),
        };
        self.world.resource_mut::<Concerns>().capture(creation);
    }
    fn follow_opportunities(&mut self) {
        let start = self.world.resource::<Concerns>().scene_cursor;
        let end = self.world.resource::<Runtime>().scenes.len();
        self.world.resource_mut::<Concerns>().scene_cursor = end;
        for scene_index in start..end {
            let scene = self.world.resource::<Runtime>().scenes[scene_index].clone();
            for owner in scene.participants {
                let partner = *scene.participants.iter().find(|&&id| id != owner).unwrap();
                let old = self
                    .world
                    .resource::<Concerns>()
                    .items
                    .get(&owner)
                    .cloned()
                    .unwrap_or_default();
                for before in old
                    .into_iter()
                    .filter(|c| c.status.active() && c.created_scene < scene.id)
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
                    state.change("later encounter", Some(before), Some(after), None, None);
                }
                if self.world.contains_resource::<crate::inquiry::Inquiry>() {
                    self.adaptive_follow(owner, &[partner], scene.id, None);
                } else {
                    self.follow_decision(owner, partner, scene.id);
                }
            }
        }
    }
    fn follow_decision(&mut self, owner: AgentId, partner: AgentId, scene: u64) {
        let entities = [owner, partner].map(|id| self.world.resource::<Index>().0[&id]);
        let context = self.world.resource::<Foresight>().context(owner, partner);
        let state = self.world.resource::<Concerns>();
        let input = FollowInput {
            owner,
            partner,
            scene,
            hunger: self.world.get::<Agent>(entities[0]).unwrap().hunger,
            relationship_goal: self.world.resource::<Intentional>().profiles[&owner]
                .relationship_goal,
            answer_probability: context.expectation.answer,
            reliability: context.reliability,
            pursuit_cost: state.pursuit_costs[&owner],
            concerns: state.items.get(&owner).cloned().unwrap_or_default(),
        };
        let id = state.records.len() as u64;
        let decision = (self.world.resource::<ConcernPolicy>().0)(&input);
        let valid = decision.input == input
            && concerns::legal(&input, decision.selected)
            && decision
                .candidates
                .iter()
                .any(|c| c.action == decision.selected)
            && decision
                .candidates
                .iter()
                .all(|c| concerns::legal(&input, c.action));
        let selected = if valid {
            decision.selected
        } else {
            FollowAction::Continue
        };
        let before = match selected {
            FollowAction::Reopen(cid) | FollowAction::Abandon(cid) => {
                input.concerns.iter().find(|c| c.id == cid).cloned()
            }
            FollowAction::Continue => None,
        };
        let food_before = entities.map(|e| self.world.get::<Agent>(e).unwrap().food);
        let time_spent = 1 + if matches!(selected, FollowAction::Reopen(_)) {
            input.pursuit_cost
        } else {
            0
        };
        self.world.resource_mut::<Runtime>().tick += u64::from(time_spent);
        let tick = self.world.resource::<Runtime>().tick;
        let mut response_conversation = None;
        let mut actual_response = None;
        let mut prediction_error = None;
        let mut answer_after = input.answer_probability;
        let mut learned = false;
        if let FollowAction::Reopen(_) = selected {
            let c = before.as_ref().unwrap();
            let conversation = self.world.resource::<Intentional>().conversations.len() as u64;
            self.converse_response(c.event, true);
            response_conversation = Some(conversation);
            let state = self.world.resource::<Intentional>();
            let first = &state.records[state.conversations[conversation as usize].first_record];
            if first.valid {
                actual_response = Some(first.decision.selected);
                let outcome = if first.decision.selected == TalkAction::ProvideEvidence {
                    100
                } else {
                    0
                };
                prediction_error = Some(outcome - input.answer_probability);
                learned = context.settings.learn;
                if learned {
                    answer_after = foresight::update_probability(input.answer_probability, outcome);
                    let mut expectation = context.expectation.clone();
                    expectation.answer = answer_after;
                    self.world
                        .resource_mut::<Foresight>()
                        .expectations
                        .entry(owner)
                        .or_default()
                        .insert(partner, expectation);
                }
            }
        }
        let mut after = None;
        if let Some(before) = &before {
            let mut state = self.world.resource_mut::<Concerns>();
            let c = state
                .items
                .get_mut(&owner)
                .unwrap()
                .iter_mut()
                .find(|c| c.id == before.id)
                .unwrap();
            let preceding = c.clone();
            match selected {
                FollowAction::Reopen(_) => {
                    c.attempts = c.attempts.saturating_add(1);
                    if c.uncertainty >= before.uncertainty {
                        c.failures = c.failures.saturating_add(1);
                    }
                }
                FollowAction::Abandon(_) => c.status = Status::Abandoned,
                FollowAction::Continue => unreachable!(),
            }
            after = Some(c.clone());
            state.change(
                if matches!(selected, FollowAction::Reopen(_)) {
                    "follow-up outcome"
                } else {
                    "pursuit no longer worthwhile"
                },
                Some(preceding),
                after.clone(),
                None,
                Some(id),
            );
        }
        let food_after = entities.map(|e| self.world.get::<Agent>(e).unwrap().food);
        self.world
            .resource_mut::<Concerns>()
            .records
            .push(FollowRecord {
                id,
                tick,
                decision,
                valid,
                before,
                after,
                response_conversation,
                actual_response,
                prediction_error,
                answer_after,
                learned,
                time_spent,
                food_before,
                food_after,
            });
    }

    fn converse(&mut self, event: u64) {
        self.converse_response(event, false);
    }
    fn converse_response(&mut self, event: u64, follow_up: bool) {
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
        let mut last = follow_up.then_some(AskEvidence);
        let mut previous_claim = None;
        if follow_up {
            previous_claim = self
                .world
                .resource::<Intentional>()
                .memories
                .get(&listener)
                .and_then(|m| {
                    m.iter()
                        .rev()
                        .find(|m| m.event == event && m.actor == speaker && m.action == Explain)
                })
                .and_then(|m| m.claim);
        }
        let mut outcome = "TurnLimit";
        let limit = if follow_up { 2 } else { 4 };
        for turn in 0..limit {
            let tick_before = self.world.resource::<Runtime>().tick;
            let record_id = self.world.resource::<Intentional>().records.len() as u64;
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
                turns_left: (limit - turn) as u32,
            };
            let plan = self.world.get_resource::<Foresight>().map(|state| {
                let context = state.context(actor, partner);
                (self.world.resource::<Predictor>().0)(&input, &context)
            });
            let decision = plan.as_ref().map_or_else(
                || (self.world.resource::<TalkPolicy>().0)(&input),
                |p| p.decision.clone(),
            );
            let valid = decision.input == input
                && intentional::legal(&input).contains(&decision.selected)
                && plan.as_ref().is_none_or(|p| {
                    let context = self.world.resource::<Foresight>().context(actor, partner);
                    p.context == context
                        && p.candidates.len() == decision.candidates.len()
                        && p.candidates.iter().zip(&decision.candidates).all(|(f, c)| {
                            f.action == c.action
                                && f.combined == c.score
                                && (0..=100).contains(&f.probability)
                        })
                        && p.candidates.iter().any(|c| c.action == decision.selected)
                });
            let food_before = entities.map(|e| self.world.get::<Agent>(e).unwrap().food);
            let mut claim = None;
            let mut information = None;
            let mut verified_claim = None;
            let mut credibility_after = credibility_before;
            let mut reading_records = vec![];
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
                    let sensor = self
                        .world
                        .get_resource::<Foresight>()
                        .filter(|_| decision.selected == ProvideEvidence)
                        .map(|s| (s.seed, s.sensor.clone()));
                    let receipt = if let Some((seed, sensor)) = sensor {
                        let mut final_receipt = None;
                        for source in 0..=u8::from(sensor.second.is_some()) {
                            let observed =
                                foresight::reading(seed, event, speaker, source, scarce, &sensor);
                            let receipt = self
                                .receive_information(
                                    speaker,
                                    listener,
                                    event,
                                    EvidenceKind::Fallible {
                                        scarce: observed,
                                        reliability: sensor.reliability,
                                        source,
                                    },
                                    None,
                                )
                                .expect("validated sensor and participants");
                            reading_records.push(Reading {
                                talk_record: record_id,
                                receipt: receipt.id,
                                source,
                                scarce: observed,
                                reliability: sensor.reliability,
                                objectively_correct: observed == scarce,
                            });
                            claim = Some(observed);
                            final_receipt = Some(receipt);
                        }
                        self.world.resource_mut::<Runtime>().tick += u64::from(sensor.effort);
                        final_receipt.unwrap()
                    } else {
                        self.receive_information(
                            speaker,
                            listener,
                            event,
                            kind,
                            Some((50 + credibility_before).clamp(10, 90)),
                        )
                        .expect("validated conversation participants")
                    };
                    information = Some(receipt.id);
                    if decision.selected == ProvideEvidence {
                        if let Some(old_claim) = previous_claim {
                            let weight = if reading_records.is_empty() {
                                100
                            } else {
                                receipt.after.support.abs()
                            };
                            if weight >= 60 {
                                let consistent = old_claim
                                    == if reading_records.is_empty() {
                                        scarce
                                    } else {
                                        receipt.after.support > 0
                                    };
                                verified_claim = Some(consistent);
                                credibility_after = (credibility_before
                                    + (if consistent { 20 } else { -30 }) * weight / 100)
                                    .clamp(-40, 40);
                            }
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
                verifiable: valid
                    && decision.selected == ProvideEvidence
                    && reading_records.is_empty(),
                information,
                credibility_before,
                credibility_after,
                verified_claim,
                time_spent: (tick - tick_before) as u32,
                food_before,
                food_after,
            });

            if let Some(plan) = plan {
                let mut state = self.world.resource_mut::<Foresight>();
                state.forecasts.push(ForecastRecord {
                    talk_record: record_id,
                    context: plan.context,
                    candidates: plan.candidates,
                });
                state.readings.extend(reading_records);

                if valid && record_id as usize > first_record {
                    self.observe_prediction(
                        record_id - 1,
                        record_id,
                        if decision.selected == Mislead {
                            Explain
                        } else {
                            decision.selected
                        },
                    );
                }
            }
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

    fn observe_prediction(&mut self, previous: u64, response: u64, actual: TalkAction) {
        let prior = &self.world.resource::<Intentional>().records[previous as usize];
        if !prior.valid {
            return;
        }
        let actor = prior.decision.input.own.id;
        let partner = prior.decision.input.partner;
        let selected = prior.decision.selected;
        let mut state = self.world.resource_mut::<Foresight>();
        let forecast = state
            .forecasts
            .iter()
            .find(|f| f.talk_record == previous)
            .unwrap()
            .clone();
        let candidate = forecast
            .candidates
            .iter()
            .find(|c| c.action == selected)
            .unwrap();
        if let Some(predicted) = candidate.expected_response {
            let outcome = if actual == predicted { 100 } else { 0 };
            let expected = candidate.probability;
            let learned = forecast.context.settings.learn;
            let updated = if learned {
                foresight::update_probability(expected, outcome)
            } else {
                expected
            };
            if learned {
                let expectation = state
                    .expectations
                    .entry(actor)
                    .or_default()
                    .entry(partner)
                    .or_insert(forecast.context.expectation);
                if predicted == TalkAction::AskEvidence {
                    expectation.challenge = updated;
                } else {
                    expectation.answer = updated;
                }
            }
            state.errors.push(PredictionError {
                forecast_record: previous,
                response_record: response,
                actor,
                partner,
                predicted,
                actual,
                expected,
                outcome,
                error: outcome - expected,
                updated,
                learned,
            });
        }
    }

    fn require_idle(&self) -> Result<(), String> {
        if !self.world.resource::<Runtime>().active.is_empty() {
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
        self.receive_information_support(speaker, listener, event, kind, None, None, true)
    }

    fn receive_information(
        &mut self,
        speaker: AgentId,
        listener: AgentId,
        event: u64,
        kind: EvidenceKind,
        testimony_weight: Option<i32>,
    ) -> Result<InformationScene, String> {
        self.receive_information_support(
            speaker,
            listener,
            event,
            kind,
            testimony_weight,
            None,
            false,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn receive_information_support(
        &mut self,
        speaker: AgentId,
        listener: AgentId,
        event: u64,
        kind: EvidenceKind,
        testimony_weight: Option<i32>,
        local_support: Option<(i32, crate::assessment::Item)>,
        strict_retention: bool,
    ) -> Result<InformationScene, String> {
        self.require_idle()?;
        if let EvidenceKind::Fallible {
            reliability,
            source,
            ..
        } = kind
        {
            if !(0..=90).contains(&reliability) || source > 1 {
                return Err("invalid fallible evidence".into());
            }
            if self
                .world
                .resource::<HistoryIndex>()
                .sources
                .get(&(event, speaker, listener, source))
                .is_some_and(|old| *old != kind)
            {
                return Err("a received source cannot be rewritten".into());
            }
        }
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
            EvidenceKind::Fallible { scarce, .. } => scarce,
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
        let summary = self
            .world
            .resource::<HistoryIndex>()
            .evidence
            .get(&(event, listener))
            .cloned();
        let credibility_trust = summary.as_ref().map_or(trust_before, |s| s.first_trust);
        let reliability = match kind {
            EvidenceKind::Testimony { .. } => {
                testimony_weight.unwrap_or_else(|| (50 + credibility_trust / 2).clamp(0, 90))
            }
            EvidenceKind::Disclosure => 100,
            EvidenceKind::Fallible { reliability, .. } => reliability,
        };
        let mut cognition = self.world.remove_resource::<Cognition>().unwrap();
        let id = cognition.information.len() as u64;
        let mut positive = summary.as_ref().map_or(0, |s| s.positive);
        let mut negative = summary.as_ref().map_or(0, |s| s.negative);
        let mut has_readings = summary.as_ref().is_some_and(|s| s.has_readings);
        {
            if let EvidenceKind::Fallible {
                scarce,
                reliability,
                ..
            } = kind
            {
                has_readings = true;
                if scarce {
                    positive = positive.max(reliability);
                } else {
                    negative = negative.max(reliability);
                }
            }
        }
        let before = cognition
            .beliefs
            .get(&listener)
            .and_then(|beliefs| beliefs.iter().find(|b| b.event == event))
            .cloned();
        let unified = if self
            .world
            .contains_resource::<crate::assessment::Assessment>()
        {
            use crate::assessment::{Assessment, Assessor, Item, Origin, Receipt};
            let incoming = local_support
                .as_ref()
                .map(|(_, item)| item.clone())
                .unwrap_or_else(|| Item {
                    receipt: Receipt::Native(id),
                    event,
                    communicator: speaker,
                    message: None,
                    origin: match kind {
                        EvidenceKind::Testimony { .. } => Origin::Claim(speaker),
                        EvidenceKind::Disclosure => Origin::Disclosure(speaker),
                        EvidenceKind::Fallible { source, .. } => Origin::NativeReading {
                            speaker,
                            channel: source,
                        },
                    },
                    claim: scarce,
                    quality: reliability,
                });
            let rule = self.world.resource::<Assessor>().0;
            let retention_decision = if let Some(state) =
                self.world.get_resource::<crate::retention::Retention>()
            {
                let retained = self
                    .world
                    .resource::<Assessment>()
                    .items
                    .get(&listener)
                    .map(|v| v.iter().cloned().collect::<Vec<_>>())
                    .unwrap_or_default();
                let concerns = self
                    .world
                    .resource::<Concerns>()
                    .items
                    .get(&listener)
                    .cloned()
                    .unwrap_or_default();
                let input = crate::retention::Input::new(
                    listener,
                    &retained,
                    &incoming,
                    concerns,
                    state.method,
                );
                let mut decision = (self.world.resource::<crate::retention::Policy>().0)(&input);
                let fallback_reason = decision.validate(&input).err();
                if let Some(error) = &fallback_reason {
                    if strict_retention {
                        self.world.insert_resource(cognition);
                        return Err(error.clone());
                    }
                    // Containing protocols may already have paid time/contact costs.
                    // Complete their legitimate receipt with local FIFO and audit rejection.
                    decision = crate::retention::fifo_fallback(&input);
                }
                Some((
                    decision,
                    crate::retention::supports(&retained),
                    fallback_reason,
                ))
            } else {
                None
            };
            let support =
                if let Some((decision, support_before, fallback_reason)) = retention_decision {
                    let assessment_id = self.world.resource::<Assessment>().records.len() as u64;
                    let support = self.world.resource_mut::<Assessment>().receive_selected(
                        listener,
                        incoming,
                        before.as_ref().map(|b| b.support),
                        rule,
                        decision.evict,
                    );
                    let retained: Vec<_> = self.world.resource::<Assessment>().items[&listener]
                        .iter()
                        .cloned()
                        .collect();
                    self.world
                        .resource_mut::<crate::retention::Retention>()
                        .records
                        .push(crate::retention::Record {
                            assessment: assessment_id,
                            fallback_reason,
                            decision,
                            event_support_before: support_before,
                            event_support_after: crate::retention::supports(&retained),
                        });
                    support
                } else {
                    self.world.resource_mut::<Assessment>().receive(
                        listener,
                        incoming,
                        before.as_ref().map(|b| b.support),
                        rule,
                    )
                };
            Some(support)
        } else {
            None
        };
        let after = if let Some(support) = unified.or_else(|| local_support.map(|(s, _)| s)) {
            Belief {
                event,
                subject: speaker,
                support,
                evidence: id,
            }
        } else if has_readings && kind != EvidenceKind::Disclosure {
            Belief {
                event,
                subject: speaker,
                support: positive - negative,
                evidence: id,
            }
        } else {
            revise_belief(
                before.as_ref(),
                Belief {
                    event,
                    subject: speaker,
                    support: if scarce { reliability } else { -reliability },
                    evidence: id,
                },
            )
        };
        let beliefs = cognition.beliefs.entry(listener).or_default();
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
        // Replace one contribution in the derived ordered relationship ledger;
        // replay only its changed suffix, preserving every original clamp.
        if let Some(change) = &revision {
            let trust = self.world.resource_mut::<HistoryIndex>().revise(
                listener,
                speaker,
                event,
                change.after.valence,
            );
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
        self.world
            .resource_mut::<HistoryIndex>()
            .remember_evidence(&record);
        cognition.information.push(record.clone());
        self.world.insert_resource(cognition);
        *self.world.get_mut::<Agent>(entity).unwrap() = agent;
        if let Some(mut concerns) = self.world.get_resource_mut::<Concerns>() {
            concerns.receive(listener, event, record.after.support, record.id);
        }
        Ok(record)
    }
}

fn advance(world: &mut World) {
    let mut runtime = world.remove_resource::<Runtime>().unwrap();
    runtime.tick += 1;
    let policy = world.resource::<Policy>().0;
    let active: Vec<_> = runtime.active.iter().copied().collect();
    for scene_index in active {
        let scene = &mut runtime.scenes[scene_index];
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
        let finished = scene.outcome.is_some();
        let event = Event {
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
        };
        world
            .resource_mut::<HistoryIndex>()
            .event(&event, &runtime.initial_trust);
        runtime.events.push(event);
        if finished {
            runtime.active.remove(&scene_index);
            for id in ids {
                runtime.busy.remove(&id);
            }
        }
    }
    world.insert_resource(runtime);
}
