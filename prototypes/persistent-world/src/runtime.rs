//! Genuine continuation state. Observer exports are never used to restore actors.
use super::*;
use crate::valuation as v;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const IDENTITY: &str = include_str!(concat!(env!("OUT_DIR"), "/runtime_identity.txt"));
pub const SCHEMA: &str = "woi-world-checkpoint/v1";
#[path = "session.rs"]
pub mod session;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionPhase {
    BeforeBoundary,
    Terminal,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Saved {
    execution_phase: ExecutionPhase,
    valuation: Option<v::Config>,
    valuations: Vec<v::Record>,
    config: c::Config,
    start: u64,
    end: u64,
    initial: Vec<Agent>,
    bootstrap_events: usize,
    initial_contexts: x::LocalContexts,
    stock: u32,
    total: u64,
    boundaries: Vec<x::Boundary>,
    agents: Vec<Agent>,
    runtime: Runtime,
    state: State,
    cognition: Cognition,
    index: HistoryIndex,
    intentional: Intentional,
    foresight: Foresight,
    concerns: Concerns,
    inquiry: q::Inquiry,
    provenance: crate::audit::RuntimeProvenance,
    contexts: crate::audit::Contexts,
    assessment: crate::assessment::Assessment,
    retention: crate::retention::Retention,
    learning: crate::self_evaluation::Learning,
    local: x::LocalContexts,
}

pub struct World {
    valuation: Option<v::Config>,
    valuations: Vec<v::Record>,
    sim: Simulation,
    config: c::Config,
    start: u64,
    end: u64,
    initial: Vec<Agent>,
    bootstrap_events: usize,
    initial_contexts: x::LocalContexts,
    stock: u32,
    total: u64,
    boundaries: Vec<x::Boundary>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    schema: String,
    runtime: String,
    checksum: String,
    #[serde(with = "serde_bytes")]
    payload: Vec<u8>,
}
fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, String> {
    let mut bytes = vec![];
    ciborium::into_writer(value, &mut bytes).map_err(|e| e.to_string())?;
    Ok(bytes)
}
fn decode<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, String> {
    let mut cursor = std::io::Cursor::new(bytes);
    let result = ciborium::from_reader(&mut cursor).map_err(|e| e.to_string())?;
    if cursor.position() != bytes.len() as u64 {
        return Err("trailing checkpoint bytes".into());
    }
    Ok(result)
}
impl World {
    pub fn new_valuation(registered: v::Config) -> Result<Self, String> {
        registered.validate()?;
        let mut world = Self::new(registered.world.clone())?;
        world.valuation = Some(registered);
        Ok(world)
    }
    pub fn new(config: c::Config) -> Result<Self, String> {
        config.validate()?;
        if config.mode != c::Mode::Continuous {
            return Err("persistent successor supports explicit023 Continuous only".into());
        }
        let (mut sim, start, end, initial, bootstrap_events, initial_contexts, stock, total) =
            Simulation::continuous_bootstrap(config.physics());
        if config.phase == c::Phase::Absolute {
            sim.world.resource_mut::<State>().epoch = 0;
        }
        Ok(Self {
            valuation: None,
            valuations: vec![],
            sim,
            config,
            start,
            end,
            initial,
            bootstrap_events,
            initial_contexts,
            stock,
            total,
            boundaries: vec![],
        })
    }
    pub fn tick(&self) -> u64 {
        self.sim.activity_tick()
    }
    pub fn start(&self) -> u64 {
        self.start
    }
    pub fn end(&self) -> u64 {
        self.end
    }
    pub fn complete(&self) -> bool {
        self.tick() == self.end
    }
    pub fn execution_phase(&self) -> ExecutionPhase {
        if self.complete() {
            ExecutionPhase::Terminal
        } else {
            ExecutionPhase::BeforeBoundary
        }
    }
    pub fn agents(&self) -> Vec<Agent> {
        self.sim.agents()
    }
    pub fn configuration(&self) -> &c::Config {
        &self.config
    }
    pub fn valuation(&self) -> Option<v::Mode> {
        self.valuation.as_ref().map(|c| c.valuation)
    }
    pub fn stock(&self) -> u32 {
        self.sim.world.resource::<State>().stock
    }
    pub fn boundaries(&self) -> &[x::Boundary] {
        &self.boundaries
    }
    pub fn transitions(&self) -> &[x::Transition] {
        &self.sim.world.resource::<State>().transitions
    }
    pub fn commitments(&self) -> Vec<x::Commitment> {
        self.sim.world.resource::<State>().active.clone()
    }
    pub fn pending(&self) -> Vec<x::Pending> {
        self.sim.world.resource::<State>().pending.clone()
    }
    /// One original loop iteration, including any indivisible inquiry jump.
    pub fn step(&mut self) -> Result<bool, String> {
        if self.complete() {
            return Ok(false);
        }
        let boundary = if let Some(registered) = &self.valuation {
            self.sim.valuation_boundary(
                self.boundaries.len(),
                &self.config,
                self.start,
                self.start + self.config.horizon,
                registered.valuation,
                &mut self.valuations,
            )
        } else {
            self.sim.continuous_boundary(
                self.boundaries.len(),
                &self.config,
                self.start,
                self.start + self.config.horizon,
            )
        };
        self.boundaries.push(boundary);
        if self.tick() > self.end {
            return Err("original terminal drain bound exceeded".into());
        }
        if self.tick() < self.end {
            self.sim.concurrent_tick();
        }
        Ok(true)
    }
    pub fn advance(&mut self, steps: usize) -> Result<(), String> {
        if steps > 2048 {
            return Err("advance is bounded at2048 boundary iterations".into());
        }
        for _ in 0..steps {
            if !self.step()? {
                break;
            }
        }
        Ok(())
    }
    fn saved(&self) -> Saved {
        let world = &self.sim.world;
        Saved {
            execution_phase: self.execution_phase(),
            valuation: self.valuation.clone(),
            valuations: self.valuations.clone(),
            config: self.config.clone(),
            start: self.start,
            end: self.end,
            initial: self.initial.clone(),
            bootstrap_events: self.bootstrap_events,
            initial_contexts: self.initial_contexts.clone(),
            stock: self.stock,
            total: self.total,
            boundaries: self.boundaries.clone(),
            agents: self.agents(),
            runtime: world.resource::<Runtime>().clone(),
            state: world.resource::<State>().clone(),
            cognition: world.resource::<Cognition>().clone(),
            index: world.resource::<HistoryIndex>().clone(),
            intentional: world.resource::<Intentional>().clone(),
            foresight: world.resource::<Foresight>().clone(),
            concerns: world.resource::<Concerns>().clone(),
            inquiry: world.resource::<q::Inquiry>().clone(),
            provenance: world.resource::<crate::audit::RuntimeProvenance>().clone(),
            contexts: world.resource::<crate::audit::Contexts>().clone(),
            assessment: world.resource::<crate::assessment::Assessment>().clone(),
            retention: world.resource::<crate::retention::Retention>().clone(),
            learning: world.resource::<crate::self_evaluation::Learning>().clone(),
            local: world.resource::<x::LocalContexts>().clone(),
        }
    }
    pub fn checkpoint(&self) -> Result<Vec<u8>, String> {
        self.validate()?;
        let payload = encode(&self.saved())?;
        encode(&Envelope {
            schema: SCHEMA.into(),
            runtime: IDENTITY.into(),
            checksum: format!("{:x}", Sha256::digest(&payload)),
            payload,
        })
    }
    pub fn restore(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() > 128 * 1024 * 1024 {
            return Err("checkpoint exceeds128MiB limit".into());
        }
        let envelope: Envelope = decode(bytes)?;
        if envelope.schema != SCHEMA
            || !session::accepts_runtime(&envelope.runtime)
            || envelope.checksum != format!("{:x}", Sha256::digest(&envelope.payload))
        {
            return Err("checkpoint schema/runtime/checksum mismatch".into());
        }
        let saved: Saved = decode(&envelope.payload)?;
        saved.config.validate()?;
        if saved.execution_phase
            != if saved.runtime.tick == saved.end {
                ExecutionPhase::Terminal
            } else {
                ExecutionPhase::BeforeBoundary
            }
        {
            return Err("incompatible saved execution phase".into());
        }
        if saved.foresight.seed != saved.config.seed
            || saved.assessment.method != crate::assessment::Method::Grouped
            || saved.retention.method != crate::retention::Method::Fifo
            || saved.learning.mode != crate::self_evaluation::Mode::Contextual
        {
            return Err("incompatible persisted policy or deterministic sensor seed".into());
        }
        // Construct only the executable wiring and stable entity mapping. No bootstrap,
        // history replay, sensing, selection or physics runs on this path.
        let mut empty = saved.agents.clone();
        for agent in &mut empty {
            agent.memories.clear();
        }
        let mut sim = Simulation::new(empty)?;
        sim.set_policy(scarcity_policy);
        sim.enable_foresight(saved.foresight.seed, saved.foresight.sensor.clone())?;
        sim.enable_concerns()?;
        sim.enable_inquiry()?;
        sim.enable_provenance()?;
        sim.enable_assessment(crate::assessment::Method::Grouped)?;
        sim.enable_retention(crate::retention::Method::Fifo)?;
        sim.enable_self_evaluation(crate::self_evaluation::Mode::Contextual)?;
        for agent in saved.agents {
            let entity = sim.world.resource::<Index>().0[&agent.id];
            *sim.world.get_mut::<Agent>(entity).unwrap() = agent;
        }
        macro_rules! put { ($($field:ident),*) => { $(sim.world.insert_resource(saved.$field);)* }; }
        put!(
            runtime,
            state,
            cognition,
            index,
            intentional,
            foresight,
            concerns,
            inquiry,
            provenance,
            contexts,
            assessment,
            retention,
            learning,
            local
        );
        let result = Self {
            valuation: saved.valuation,
            valuations: saved.valuations,
            sim,
            config: saved.config,
            start: saved.start,
            end: saved.end,
            initial: saved.initial,
            bootstrap_events: saved.bootstrap_events,
            initial_contexts: saved.initial_contexts,
            stock: saved.stock,
            total: saved.total,
            boundaries: saved.boundaries,
        };
        result.validate()?;
        Ok(result)
    }
    pub fn validate(&self) -> Result<(), String> {
        self.config.validate()?;
        if self.initial.iter().map(|a| a.id).collect::<Vec<_>>() != vec![0, 1, 2, 3]
            || self.start.checked_add(self.config.horizon + c::DRAIN) != Some(self.end)
        {
            return Err("invalid initial identities or finite endpoint".into());
        }
        if let Some(registered) = &self.valuation {
            registered.validate()?;
            if registered.world != self.config
                || self.valuations.len()
                    != self
                        .boundaries
                        .iter()
                        .map(|b| b.choices.len())
                        .sum::<usize>()
                || self.valuations.iter().any(|r| {
                    r.policy != registered.valuation
                        || self
                            .boundaries
                            .get(r.boundary)
                            .is_none_or(|b| r.choice >= b.choices.len())
                })
            {
                return Err("inconsistent024 continuation configuration/records".into());
            }
        } else if !self.valuations.is_empty() {
            return Err("023 contains024 valuation records".into());
        }
        let state = self.sim.world.resource::<State>();
        // Audit can reject an inconsistent saved cell; it must never fill one in.
        for owner in 0..4 {
            let expected = state
                .memories
                .iter()
                .rev()
                .find(|r| r.owner == owner)
                .and_then(|r| r.after.as_ref());
            if state.slots[owner as usize].as_ref() != expected {
                return Err("saved environmental knowledge disagrees with its established retention transitions".into());
            }
        }
        if self.complete() && (!state.active.is_empty() || !state.pending.is_empty()) {
            return Err("terminal checkpoint contains unfinished work".into());
        }
        let assessment = self.sim.world.resource::<crate::assessment::Assessment>();
        let provenance = self.sim.world.resource::<crate::audit::RuntimeProvenance>();
        let inquiry = self.sim.world.resource::<q::Inquiry>();
        let concerns = self.sim.world.resource::<Concerns>();
        let learning = self
            .sim
            .world
            .resource::<crate::self_evaluation::Learning>();
        if self
            .sim
            .world
            .resource::<Cognition>()
            .beliefs
            .values()
            .any(|v| v.len() > crate::model::MEMORY_CAPACITY)
            || self
                .sim
                .world
                .resource::<Intentional>()
                .memories
                .values()
                .any(|v| v.len() > crate::model::MEMORY_CAPACITY)
            || inquiry
                .offers
                .values()
                .any(|v| v.len() > q::SIGNATURE_CAPACITY)
            || inquiry
                .capabilities
                .values()
                .any(|v| v.len() > q::CELL_CAPACITY)
        {
            return Err("persisted legacy beliefs/communication/offers/capabilities exceed their own bounds".into());
        }
        if assessment.items.values().any(|v| v.len() > 32)
            || provenance.knowledge.values().any(|v| v.len() > 32)
            || provenance.hints.values().any(|v| v.len() > 32)
            || provenance.comparisons.values().any(|v| v.len() > 32)
            || concerns.items.values().any(|v| v.len() > 8)
            || learning.cells.values().any(|v| v.len() > 32)
            || inquiry.cells.values().any(|v| v.len() > 32)
            || inquiry.seen.values().any(|v| v.len() > 32)
            || inquiry
                .episodes
                .values()
                .any(|v| v.len() > q::EPISODE_CAPACITY)
            || inquiry
                .contacts
                .values()
                .any(|v| v.len() > q::CONTACT_CAPACITY)
        {
            return Err("persisted cognition exceeds established bounds".into());
        }
        if self.config.mode != c::Mode::Continuous
            || state.config != self.config.physics()
            || self.end != self.start + self.config.horizon + c::DRAIN
            || !(self.start..=self.end).contains(&self.tick())
            || state.synced != self.tick()
            || state.total != self.total
            || self.sim.agents().iter().map(|a| a.id).collect::<Vec<_>>() != vec![0, 1, 2, 3]
        {
            return Err("inconsistent continuation clock/config/actors".into());
        }
        let agents = self.agents();
        if agents.iter().any(|a| {
            a.memories.len() > crate::model::MEMORY_CAPACITY
                || !(0..=100).contains(&a.hunger)
                || a.trust.values().any(|v| !(-100..=100).contains(v))
        }) {
            return Err("invalid owned cognition/needs".into());
        }
        let consumed: u64 = self
            .sim
            .cognition()
            .consumption
            .iter()
            .map(|m| u64::from(m.consumed))
            .sum();
        if agents.iter().map(|a| u64::from(a.food)).sum::<u64>() + u64::from(state.stock) + consumed
            != self.total
        {
            return Err("food ledger mismatch".into());
        }
        if state.active.len() > 4
            || state.pending.len() > 4
            || state
                .active
                .iter()
                .any(|c| c.deadline <= self.tick() || c.start > self.tick() || c.id >= state.next)
        {
            return Err("illegal active commitment".into());
        }
        let runtime = self.sim.world.resource::<Runtime>();
        if runtime.circumstances.len() != runtime.events.len()
            || runtime.conversation_cursor > runtime.events.len()
            || concerns.scene_cursor > runtime.scenes.len()
            || runtime.events.iter().any(|e| {
                e.participants.iter().any(|id| *id >= 4) || e.participants[0] == e.participants[1]
            })
            || runtime.scenes.iter().any(|s| {
                s.participants.iter().any(|id| *id >= 4) || s.participants[0] == s.participants[1]
            })
        {
            return Err("invalid physical participant/cursor references".into());
        }
        let occupied = state
            .active
            .iter()
            .flat_map(|c| c.participants.iter().copied())
            .collect::<Vec<_>>();
        if occupied.iter().copied().collect::<BTreeSet<_>>().len() != occupied.len() {
            return Err("overlapping participant commitments".into());
        }
        for agent in &agents {
            let id = agent.id as usize;
            let mut hunger = ((self.start, 0), self.initial[id].hunger);
            let mut food = ((self.start, 0), self.initial[id].food);
            for need in &state.needs {
                if (need.tick, 1) >= hunger.0 {
                    hunger = ((need.tick, 1), need.after[id]);
                }
            }
            for event in runtime.events.iter().skip(self.bootstrap_events) {
                if let Some(participant) = event
                    .participants
                    .iter()
                    .position(|owner| *owner == agent.id)
                    && (event.tick, 1) >= food.0
                {
                    food = ((event.tick, 1), event.balances_after[participant]);
                }
            }
            for grant in state.grants.iter().filter(|g| g.owner == agent.id) {
                if (grant.tick, 2) >= food.0 {
                    food = ((grant.tick, 2), grant.food_after);
                }
            }
            for meal in self
                .sim
                .world
                .resource::<Cognition>()
                .consumption
                .iter()
                .filter(|m| m.actor == agent.id)
            {
                if (meal.tick, 3) >= hunger.0 {
                    hunger = ((meal.tick, 3), meal.hunger_after);
                }
                if (meal.tick, 3) >= food.0 {
                    food = ((meal.tick, 3), meal.food_after);
                }
            }
            if food.1 != agent.food || hunger.1 != agent.hunger {
                return Err("owned resource/need state disagrees with executed prefix".into());
            }
            let expected: Vec<_> = runtime
                .events
                .iter()
                .filter(|e| e.participants.contains(&agent.id))
                .map(|e| e.id)
                .rev()
                .take(crate::model::MEMORY_CAPACITY)
                .collect();
            if agent
                .memories
                .iter()
                .rev()
                .map(|m| m.event)
                .collect::<Vec<_>>()
                != expected
            {
                return Err(
                    "saved episodic retention disagrees with bounded event membership".into(),
                );
            }
            for partner in 0..4 {
                let expected = self
                    .sim
                    .world
                    .resource::<HistoryIndex>()
                    .saved_trust(agent.id, partner)
                    .unwrap_or_else(|| {
                        runtime
                            .initial_trust
                            .get(&agent.id)
                            .and_then(|m| m.get(&partner))
                            .copied()
                            .unwrap_or(0)
                    });
                if agent.trust.get(&partner).copied().unwrap_or(0) != expected {
                    return Err("persistent trust disagrees with resolver contributions".into());
                }
            }
        }
        if runtime
            .events
            .iter()
            .enumerate()
            .any(|(i, e)| e.id != i as u64 || e.scene as usize >= runtime.scenes.len())
            || runtime
                .scenes
                .iter()
                .enumerate()
                .any(|(i, s)| s.id != i as u64)
            || self
                .boundaries
                .windows(2)
                .any(|pair| pair[0].tick >= pair[1].tick)
        {
            return Err("duplicate/out-of-order historical identity".into());
        }
        let mut unfinished = BTreeMap::new();
        let mut next = 0;
        for transition in &state.transitions {
            let commitment = &transition.commitment;
            if commitment.participants.is_empty()
                || commitment.participants.len() > 2
                || commitment.participants.iter().any(|id| *id >= 4)
                || commitment.start >= commitment.deadline
                || commitment.source >= self.boundaries.len()
            {
                return Err("illegal commitment reference/interval".into());
            }
            if transition.completed {
                if unfinished.remove(&commitment.id) != Some(commitment) {
                    return Err("duplicate or unmatched completion".into());
                }
            } else {
                if commitment.id != next || unfinished.insert(commitment.id, commitment).is_some() {
                    return Err("duplicate commitment identity".into());
                }
                next += 1;
            }
        }
        if next != state.next
            || unfinished.len() != state.active.len()
            || state
                .active
                .iter()
                .any(|c| unfinished.get(&c.id) != Some(&c))
        {
            return Err("pending commitment state disagrees with completed prefix".into());
        }
        if state.pending.iter().any(|p| {
            p.source >= self.boundaries.len()
                || p.invitation.proposer >= 4
                || p.invitation.recipient >= 4
                || p.invitation.proposer == p.invitation.recipient
                || p.invitation.expires < self.tick()
                || p.response.as_ref().is_some_and(|r| {
                    self.boundaries
                        .get(r.boundary)
                        .and_then(|b| b.responses.get(r.response))
                        .is_none_or(|response| response.selected != r.selected)
                })
        }) {
            return Err("invalid pending invitation/response link".into());
        }
        for pending in &state.pending {
            let expected = self
                .boundaries
                .iter()
                .enumerate()
                .find_map(|(boundary, b)| {
                    b.responses
                        .iter()
                        .enumerate()
                        .find(|(_, r)| r.input.incoming.contains(&pending.invitation))
                        .map(|(response, r)| x::ReplyRef {
                            boundary,
                            response,
                            selected: r.selected,
                        })
                });
            if pending.response != expected {
                return Err("missing or changed pending response".into());
            }
        }
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
            .chain(
                state
                    .active
                    .iter()
                    .filter(|c| !matches!(c.work, x::Work::Resource { .. }))
                    .flat_map(|c| c.participants.iter().copied()),
            )
            .collect();
        if active != runtime.active || busy != runtime.busy {
            return Err("live scene/busy index mismatch".into());
        }
        let mut rebuilt = HistoryIndex::default();
        for event in &runtime.events {
            rebuilt.event(event, &runtime.initial_trust);
        }
        for info in &self.sim.world.resource::<Cognition>().information {
            rebuilt.information(info);
        }
        for root in &self
            .sim
            .world
            .resource::<crate::audit::RuntimeProvenance>()
            .roots
        {
            rebuilt
                .inspection_slots
                .insert((root.event, root.observer), root.id);
        }
        if rebuilt != *self.sim.world.resource::<HistoryIndex>() {
            return Err("resolver history index mismatch".into());
        }
        Ok(())
    }
    pub fn trial(&self) -> Result<c::Trial, String> {
        if self.valuation.is_some() {
            return Err("024 world requires its explicit valuation export".into());
        }
        self.common_trial()
    }
    pub fn valuation_trial(&self) -> Result<v::Trial, String> {
        let registered = self.valuation.as_ref().ok_or("not a024 world")?;
        let trial = self.common_trial()?;
        Ok(v::Trial {
            schema: "experiment-024-trial/v1".into(),
            config: registered.clone(),
            history: trial.history,
            checkpoints: trial.checkpoints,
            valuations: self.valuations.clone(),
        })
    }
    pub fn export(&self) -> Result<Vec<u8>, String> {
        if self.valuation.is_some() {
            serde_json::to_vec(&self.valuation_trial()?).map_err(|e| e.to_string())
        } else {
            serde_json::to_vec(&self.trial()?).map_err(|e| e.to_string())
        }
    }
    fn common_trial(&self) -> Result<c::Trial, String> {
        if !self.complete() {
            return Err("unfinished world is not a completed Trial".into());
        }
        if !self.commitments().is_empty() || !self.pending().is_empty() {
            return Err("unsettled terminal world".into());
        }
        self.validate()?;
        let sim = &self.sim;
        let final_state = crate::experiment013::Snapshot {
            base: crate::experiment009::snapshot(sim),
            learning: sim.self_evaluation().unwrap(),
            mode: crate::experiment013::Mode::Contextual,
        };
        final_state.validate()?;
        let s = sim.world.resource::<State>();
        let history = x::Trial {
            schema: if self.valuation.is_some() {
                "experiment-024-physical-history/v1"
            } else {
                "experiment-023-physical-history/v1"
            }
            .into(),
            config: self.config.physics(),
            start: self.start,
            end: self.end,
            initial: self.initial.clone(),
            initial_stock: self.stock,
            initial_total: self.total,
            boundaries: self.boundaries.clone(),
            transitions: s.transitions.clone(),
            admissions: s.admissions.clone(),
            needs: s.needs.clone(),
            grants: s.grants.clone(),
            memories: s.memories.clone(),
            final_memory: s.slots.to_vec(),
            profiles: (0..4)
                .map(|id| sim.intentional().unwrap().profiles[&id].clone())
                .collect(),
            bootstrap_events: self.bootstrap_events,
            initial_contexts: self.initial_contexts.clone(),
            final_contexts: sim.world.resource::<x::LocalContexts>().clone(),
            final_agents: sim.agents(),
            final_stock: s.stock,
            final_state,
        };
        Ok(c::Trial {
            schema: "experiment-023-trial/v1".into(),
            config: self.config.clone(),
            checkpoints: c::checkpoints(&history, &self.config),
            history,
        })
    }
}

#[cfg(test)]
mod continuation_checks {
    use super::*;
    fn world() -> World {
        let mut config = c::matrix()
            .into_iter()
            .find(|c| c.case == "paired" && c.mode == c::Mode::Continuous)
            .unwrap();
        config.horizon = 64;
        let mut world = World::new(config).unwrap();
        world.advance(6).unwrap();
        world
    }
    fn altered(mut saved: Saved, change: impl FnOnce(&mut Saved)) -> Vec<u8> {
        change(&mut saved);
        let payload = encode(&saved).unwrap();
        encode(&Envelope {
            schema: SCHEMA.into(),
            runtime: IDENTITY.into(),
            checksum: format!("{:x}", Sha256::digest(&payload)),
            payload,
        })
        .unwrap()
    }
    #[test]
    fn semantic_corruption_rejected_even_with_recomputed_checksum() {
        let world = world();
        let mutations: Vec<fn(&mut Saved)> = vec![
            |s| s.agents[0].hunger = (s.agents[0].hunger + 1) % 101,
            |s| s.agents[0].food += 1,
            |s| s.agents[1].food = s.agents[1].food.saturating_sub(1),
            |s| {
                s.agents[0].trust.insert(1, 99);
            },
            |s| s.runtime.tick += 1,
            |s| s.runtime.events[0].id = 999,
            |s| s.foresight.seed ^= 1,
            |s| s.assessment.method = crate::assessment::Method::Max,
            |s| s.retention.method = crate::retention::Method::Salient,
            |s| s.config.horizon += 1,
            |s| s.execution_phase = ExecutionPhase::Terminal,
        ];
        for (index, mutation) in mutations.into_iter().enumerate() {
            assert!(
                World::restore(&altered(world.saved(), mutation)).is_err(),
                "corruption{index}"
            );
        }
    }
    #[test]
    fn invalid_version_identity_and_deadline_are_rejected() {
        let mut world = world();
        while world.commitments().is_empty() && !world.complete() {
            world.step().unwrap();
        }
        assert!(!world.commitments().is_empty());
        let bytes = altered(world.saved(), |s| s.state.active[0].deadline += 1);
        assert!(World::restore(&bytes).is_err());
        for field in [0, 1] {
            let mut envelope: Envelope = decode(&world.checkpoint().unwrap()).unwrap();
            if field == 0 {
                envelope.schema = "unknown/v9".into();
            } else {
                envelope.runtime = "different-runtime".into();
            }
            assert!(World::restore(&encode(&envelope).unwrap()).is_err());
        }
    }
    #[test]
    fn evicted_episode_and_pending_response_are_not_recreated() {
        let mut config = c::matrix()
            .into_iter()
            .find(|c| c.case == "paired" && c.mode == c::Mode::Continuous)
            .unwrap();
        config.horizon = 192;
        let mut world = World::new(config.clone()).unwrap();
        world.advance(128).unwrap();
        let old = world.sim.events()[0].interpretations[0].clone();
        let owner = world.sim.events()[0].participants[0] as usize;
        assert!(
            !world.agents()[owner]
                .memories
                .iter()
                .any(|m| m.event == old.event)
        );
        let bytes = altered(world.saved(), |s| {
            s.agents[owner].memories.pop_back();
            s.agents[owner].memories.push_front(old);
        });
        assert!(World::restore(&bytes).is_err());
        let saved = world.saved();
        let (owner, old_cell) = saved
            .state
            .memories
            .iter()
            .find_map(|record| {
                record
                    .after
                    .as_ref()
                    .filter(|cell| Some(*cell) != saved.state.slots[record.owner as usize].as_ref())
                    .map(|cell| (record.owner as usize, cell.clone()))
            })
            .expect("an overwritten environmental observation remains in audit");
        let bytes = altered(saved, |s| s.state.slots[owner] = Some(old_cell));
        assert!(World::restore(&bytes).is_err());
        let mut pending = World::new(config).unwrap();
        pending.advance(2).unwrap();
        assert!(pending.pending().iter().any(|p| p.response.is_some()));
        let bytes = altered(pending.saved(), |s| {
            s.state
                .pending
                .iter_mut()
                .find(|p| p.response.is_some())
                .unwrap()
                .response = None
        });
        assert!(World::restore(&bytes).is_err());
    }
}
