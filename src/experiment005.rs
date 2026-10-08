//! Controlled encounters expose opportunities; policies choose all communication.
use crate::{
    behavior::scarcity_policy,
    cognition::Cognition,
    concerns::*,
    foresight::{Channel, Foresight, Sensor},
    intentional::{Intentional, Profile},
    model::*,
    simulation::Simulation,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub sensor: Sensor,
    pub cost: u32,
    pub later_goal: i32,
    pub later_privacy: i32,
    pub encounters: u32,
    pub competing: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            name: "reopen_strong".into(),
            sensor: Sensor {
                channel: Channel::AccurateFixture,
                ..Default::default()
            },
            cost: 20,
            later_goal: 60,
            later_privacy: 0,
            encounters: 1,
            competing: false,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub label: String,
    pub agents: Vec<Agent>,
    pub concerns: Concerns,
    pub cognition: Cognition,
    pub intentional: Intentional,
    pub foresight: Foresight,
    pub events: Vec<Event>,
    pub scenes: Vec<Scene>,
}
pub fn snapshot(sim: &Simulation, label: &str) -> Snapshot {
    Snapshot {
        label: label.into(),
        agents: sim.agents(),
        concerns: sim.concerns().unwrap(),
        cognition: sim.cognition(),
        intentional: sim.intentional().unwrap(),
        foresight: sim.foresight().unwrap(),
        events: sim.events(),
        scenes: sim.scenes(),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub config: Config,
    pub snapshots: Vec<Snapshot>,
    pub probe: Decision,
    pub history_ablated: FollowDecision,
}

pub fn setup(seed: u64, sensor: Sensor, goal: i32) -> Simulation {
    let mut g = Generator::new(seed);
    let mut sim = Simulation::new(vec![g.agent(0), g.agent(1)]).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(seed, sensor).unwrap();
    sim.enable_concerns().unwrap();
    refusal_conditions(&mut sim, goal);
    sim
}
pub fn refusal_conditions(sim: &mut Simulation, goal: i32) {
    sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
    sim.set_circumstances(1, 1, 90, 0, 90).unwrap();
    sim.set_communication_profile(
        0,
        Profile {
            relationship_goal: goal,
            ..Default::default()
        },
    )
    .unwrap();
    sim.set_communication_profile(
        1,
        Profile {
            privacy: 100,
            ..Default::default()
        },
    )
    .unwrap();
}
pub fn later_conditions(sim: &mut Simulation, goal: i32, privacy: i32, cost: u32) {
    sim.set_circumstances(0, 4, 30, 0, 80).unwrap();
    sim.set_circumstances(1, 1, 30, 0, 90).unwrap();
    sim.set_communication_profile(
        0,
        Profile {
            relationship_goal: goal,
            ..Default::default()
        },
    )
    .unwrap();
    sim.set_communication_profile(
        1,
        Profile {
            privacy,
            ..Default::default()
        },
    )
    .unwrap();
    sim.set_follow_cost(0, cost).unwrap();
}
pub fn encounter(sim: &mut Simulation) {
    sim.add_scene([0, 1], 1, 6).unwrap();
    sim.run();
}
pub fn trial(seed: u64, config: Config) -> Trial {
    let mut sim = setup(
        seed,
        config.sensor.clone(),
        if config.competing { 40 } else { 60 },
    );
    let mut snapshots = vec![snapshot(
        &sim,
        "initial private scarcity; speaker privacy 100",
    )];
    encounter(&mut sim);
    snapshots.push(snapshot(
        &sim,
        "original encounter ended; unresolved concern retained",
    ));
    if config.competing {
        refusal_conditions(&mut sim, 80);
        sim.set_follow_cost(0, 80).unwrap();
        snapshots.push(snapshot(
            &sim,
            "second refusal setup: goal 80, pursuit cost 80",
        ));
        encounter(&mut sim);
        snapshots.push(snapshot(&sim, "two unresolved refusals"));
    }
    later_conditions(
        &mut sim,
        config.later_goal,
        config.later_privacy,
        config.cost,
    );
    snapshots.push(snapshot(
        &sim,
        "matched later circumstances; only documented profile/cost intervention",
    ));
    for _ in 0..config.encounters {
        encounter(&mut sim);
        snapshots.push(snapshot(
            &sim,
            "later encounter and policy-selected follow-up",
        ));
    }
    let state = sim.concerns().unwrap();
    let mut local = state
        .records
        .iter()
        .find(|r| r.decision.input.owner == 0 && !r.decision.input.concerns.is_empty())
        .unwrap()
        .decision
        .input
        .clone();
    local.concerns.clear();
    let history_ablated = policy(&local);
    // Read-only behavioral probe: no new encounter or hidden-state input.
    let mut actor = sim.agents()[0].clone();
    actor.food = 4;
    actor.hunger = 30;
    actor.generosity = 60;
    actor.caution = 80;
    let probe = scarcity_policy(
        &actor,
        &Observation {
            partner: 1,
            last_signal: None,
            amount: 1,
            turns_left: 6,
        },
    );
    Trial {
        seed,
        config,
        snapshots,
        probe,
        history_ablated,
    }
}
pub fn suite(seed: u64) -> Vec<Trial> {
    let mut configs = vec![Config::default()];
    configs.push(Config {
        name: "high_cost".into(),
        cost: 100,
        ..Default::default()
    });
    configs.push(Config {
        name: "relationship_high".into(),
        cost: 60,
        ..Default::default()
    });
    configs.push(Config {
        name: "relationship_low".into(),
        cost: 60,
        later_goal: 0,
        ..Default::default()
    });
    configs.push(Config {
        name: "abandon".into(),
        cost: 100,
        later_goal: 0,
        ..Default::default()
    });
    configs.push(Config {
        name: "weak".into(),
        sensor: Sensor {
            reliability: 40,
            channel: Channel::AccurateFixture,
            ..Default::default()
        },
        ..Default::default()
    });
    configs.push(Config {
        name: "conflicting".into(),
        sensor: Sensor {
            channel: Channel::AccurateFixture,
            second: Some(Channel::InvertedFixture),
            ..Default::default()
        },
        ..Default::default()
    });
    configs.push(Config {
        name: "mistaken_resolution".into(),
        sensor: Sensor {
            channel: Channel::InvertedFixture,
            ..Default::default()
        },
        ..Default::default()
    });
    configs.push(Config {
        name: "repeated_failure".into(),
        later_privacy: 100,
        encounters: 8,
        ..Default::default()
    });
    configs.push(Config {
        name: "competing".into(),
        competing: true,
        ..Default::default()
    });
    configs.push(Config {
        name: "weak_repeated".into(),
        sensor: Sensor {
            reliability: 40,
            channel: Channel::AccurateFixture,
            ..Default::default()
        },
        encounters: 8,
        ..Default::default()
    });
    configs.into_iter().map(|c| trial(seed, c)).collect()
}
pub fn population(seed: u64, count: u32) -> Snapshot {
    let mut g = Generator::new(seed);
    let mut sim = Simulation::new((0..count).map(|id| g.agent(id)).collect()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(seed, Sensor::default()).unwrap();
    sim.enable_concerns().unwrap();
    for _ in 0..3 {
        for id in (0..count.saturating_sub(1)).step_by(2) {
            sim.add_scene([id, id + 1], 1, 6).unwrap();
        }
        sim.run();
        for id in 0..count {
            sim.consume(id).unwrap();
        }
    }
    snapshot(
        &sim,
        "three natural encounters per pair, consumption after each; no replenishment",
    )
}
pub fn human(trials: &[Trial]) -> String {
    use std::fmt::Write;
    let mut out = String::from("Experiment 005: local unfinished intent (seed 42)\n\n");
    for t in trials {
        let s = t.snapshots.last().unwrap();
        writeln!(
            out,
            "{}: probe {:?}; history ablation {:?}",
            t.config.name, t.probe.selected, t.history_ablated.selected
        )
        .unwrap();
        for c in &s.concerns.creations {
            writeln!(
                out,
                "  refusal {}: owner {} importance {} support {:?}, retained {}",
                c.event, c.owner, c.importance, c.support, c.retained
            )
            .unwrap();
        }
        for r in s
            .concerns
            .records
            .iter()
            .filter(|r| !r.decision.input.concerns.is_empty())
        {
            writeln!(out,"  scene {} owner {}: {:?}; answer expectation {} -> {}; actual {:?}; cost ticks {}",
                r.decision.input.scene,r.decision.input.owner,r.decision.selected,r.decision.input.answer_probability,r.answer_after,r.actual_response,r.time_spent).unwrap();
            for c in &r.decision.candidates {
                writeln!(
                    out,
                    "    {:?}: gain {}, benefit {}, cost {}, score {}",
                    c.action, c.expected_gain, c.benefit, c.cost, c.score
                )
                .unwrap();
            }
            if let Some(c) = &r.after {
                writeln!(
                    out,
                    "    concern {} {:?} uncertainty {} attempts {} failures {}",
                    c.id, c.status, c.uncertainty, c.attempts, c.failures
                )
                .unwrap();
            }
        }
        for i in &s.cognition.information {
            writeln!(out,"  receipt {} event {} reliability {} support {:?} -> {}; memory revised {}; trust {} -> {}",
            i.id,i.event,i.reliability,i.before.as_ref().map(|b|b.support),i.after.support,i.revision.is_some(),i.trust_before,i.trust_after).unwrap();
        }
        for r in &s.intentional.records {
            if r.verified_claim.is_some() {
                writeln!(
                    out,
                    "  talk {}: evidence consistency {:?}; credibility {} -> {}",
                    r.id, r.verified_claim, r.credibility_before, r.credibility_after
                )
                .unwrap();
            }
        }
        for c in s.concerns.items.values().flatten() {
            writeln!(
                out,
                "  retained concern {} event {}: {:?}, uncertainty {}, age {}, last receipt {:?}",
                c.id, c.event, c.status, c.uncertainty, c.age, c.last_receipt
            )
            .unwrap();
        }
        writeln!(out).unwrap();
    }
    out
}
