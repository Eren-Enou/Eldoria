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
        world.insert_resource(Runtime::default());
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
    }
    pub fn events(&self) -> Vec<Event> {
        self.world.resource::<Runtime>().events.clone()
    }
    pub fn scenes(&self) -> Vec<Scene> {
        self.world.resource::<Runtime>().scenes.clone()
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
