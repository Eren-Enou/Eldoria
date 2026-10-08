//! Controlled foresight experiments; communication remains policy-selected.
use crate::{
    behavior::scarcity_policy,
    cognition::Cognition,
    foresight::*,
    intentional::{Intentional, Profile},
    model::*,
    simulation::Simulation,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub sensor: Sensor,
    pub settings: Settings,
    pub food: u32,
    pub privacy: i32,
    pub listener_goal: i32,
    pub request_cost: i32,
    pub training: Option<bool>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            name: "base".into(),
            sensor: Sensor {
                channel: Channel::AccurateFixture,
                ..Sensor::default()
            },
            settings: Settings::default(),
            food: 1,
            privacy: 0,
            listener_goal: 60,
            request_cost: 0,
            training: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub intervention: String,
    pub agents: Vec<Agent>,
    pub cognition: Cognition,
    pub intentional: Intentional,
    pub foresight: Foresight,
    pub event_count: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub config: Config,
    pub snapshots: Vec<Snapshot>,
    pub target: u64,
    pub probe: Decision,
    pub events: Vec<Event>,
    pub scenes: Vec<Scene>,
    pub cognition: Cognition,
    pub intentional: Intentional,
    pub foresight: Foresight,
}
fn snapshot(sim: &Simulation, label: &str, out: &mut Vec<Snapshot>) {
    out.push(Snapshot {
        intervention: label.into(),
        agents: sim.agents(),
        cognition: sim.cognition(),
        intentional: sim.intentional().unwrap(),
        foresight: sim.foresight().unwrap(),
        event_count: sim.events().len(),
    });
}
fn configure(sim: &mut Simulation, food: u32, privacy: i32, goal: i32, cost: i32) {
    sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
    sim.set_circumstances(1, food, 90, 0, 90).unwrap();
    sim.set_communication_profile(
        0,
        Profile {
            relationship_goal: goal,
            request_cost: cost,
            ..Profile::default()
        },
    )
    .unwrap();
    sim.set_communication_profile(
        1,
        Profile {
            privacy,
            relationship_goal: 60,
            honesty: 0,
            request_cost: 40,
        },
    )
    .unwrap();
}
fn encounter(sim: &mut Simulation) {
    sim.add_scene([0, 1], 1, 6).unwrap();
    sim.run();
}
pub fn trial(seed: u64, config: Config) -> Trial {
    let mut g = Generator::new(seed);
    let mut sim = Simulation::new(vec![g.agent(0), g.agent(1)]).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(seed, config.sensor.clone()).unwrap();
    sim.set_prediction_settings(1, config.settings.clone())
        .unwrap();
    let mut snapshots = vec![];
    if let Some(challenge) = config.training {
        for _ in 0..6 {
            configure(
                &mut sim,
                1,
                0,
                if challenge { 60 } else { 0 },
                if challenge { 0 } else { 100 },
            );
            snapshot(
                &sim,
                "training intervention: scarce refuser; listener profile encourages or discourages evidence requests",
                &mut snapshots,
            );
            encounter(&mut sim);
            snapshot(
                &sim,
                "after observed response and prediction update",
                &mut snapshots,
            );
        }
    }
    configure(
        &mut sim,
        config.food,
        config.privacy,
        config.listener_goal,
        config.request_cost,
    );
    snapshot(
        &sim,
        "target intervention: matched current physical/profile state; preserve learned expectations",
        &mut snapshots,
    );
    encounter(&mut sim);
    let target = sim.events().last().unwrap().id;
    assert_eq!(
        sim.events()[target as usize].decision.selected,
        Action::Refuse
    );
    snapshot(
        &sim,
        "after automatic communication and evidence",
        &mut snapshots,
    );
    sim.consume(0).unwrap();
    sim.consume(1).unwrap();
    snapshot(&sim, "explicit consumption phase", &mut snapshots);
    sim.set_circumstances(0, 4, 30, 60, 80).unwrap();
    sim.set_circumstances(1, 0, 90, 30, 20).unwrap();
    snapshot(&sim, "identical subsequent physical probe", &mut snapshots);
    let start = sim.events().len();
    encounter(&mut sim);
    let probe = sim.events()[start].decision.clone();
    snapshot(&sim, "after resource probe", &mut snapshots);
    Trial {
        seed,
        config,
        snapshots,
        target,
        probe,
        events: sim.events(),
        scenes: sim.scenes(),
        cognition: sim.cognition(),
        intentional: sim.intentional().unwrap(),
        foresight: sim.foresight().unwrap(),
    }
}
pub fn configs() -> Vec<Config> {
    let deception = Config {
        food: 4,
        listener_goal: 0,
        request_cost: 100,
        ..Config::default()
    };
    vec![
        Config {
            name: "A-immediate".into(),
            settings: Settings {
                anticipate: false,
                ..Settings::default()
            },
            ..deception.clone()
        },
        Config {
            name: "A-anticipating".into(),
            ..deception.clone()
        },
        Config {
            name: "B-expect-challenge".into(),
            settings: Settings {
                challenge_prior: 90,
                ..Settings::default()
            },
            ..deception.clone()
        },
        Config {
            name: "B-expect-no-challenge".into(),
            settings: Settings {
                challenge_prior: 0,
                ..Settings::default()
            },
            ..deception.clone()
        },
        Config {
            name: "B-challenged-lie".into(),
            listener_goal: 60,
            request_cost: 0,
            settings: Settings {
                challenge_prior: 0,
                ..Settings::default()
            },
            ..deception.clone()
        },
        Config {
            name: "C-low-privacy".into(),
            ..Config::default()
        },
        Config {
            name: "C-high-privacy".into(),
            privacy: 100,
            ..Config::default()
        },
        Config {
            name: "D-weak".into(),
            sensor: Sensor {
                reliability: 40,
                channel: Channel::AccurateFixture,
                ..Sensor::default()
            },
            ..Config::default()
        },
        Config {
            name: "D-strong".into(),
            ..Config::default()
        },
        Config {
            name: "E-conflict".into(),
            sensor: Sensor {
                channel: Channel::AccurateFixture,
                second: Some(Channel::InvertedFixture),
                ..Sensor::default()
            },
            ..Config::default()
        },
        Config {
            name: "F-cheap-request".into(),
            listener_goal: 0,
            request_cost: 0,
            ..Config::default()
        },
        Config {
            name: "F-expensive-request".into(),
            listener_goal: 0,
            request_cost: 100,
            ..Config::default()
        },
        Config {
            name: "F-expensive-disclosure".into(),
            sensor: Sensor {
                effort: 80,
                channel: Channel::AccurateFixture,
                ..Sensor::default()
            },
            ..Config::default()
        },
        Config {
            name: "G-no-challenge-history".into(),
            training: Some(false),
            ..deception.clone()
        },
        Config {
            name: "H-challenge-history".into(),
            training: Some(true),
            ..deception.clone()
        },
        Config {
            name: "G-learning-disabled".into(),
            training: Some(false),
            settings: Settings {
                learn: false,
                ..Settings::default()
            },
            ..deception
        },
        Config {
            name: "I-noisy-evidence".into(),
            sensor: Sensor::default(),
            ..Config::default()
        },
        Config {
            name: "B-delayed-lie".into(),
            food: 4,
            ..Config::default()
        },
    ]
}
pub fn suite(seed: u64) -> Vec<Trial> {
    configs().into_iter().map(|c| trial(seed, c)).collect()
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Population {
    pub initial: Vec<Agent>,
    pub final_agents: Vec<Agent>,
    pub events: Vec<Event>,
    pub scenes: Vec<Scene>,
    pub cognition: Cognition,
    pub intentional: Intentional,
    pub foresight: Foresight,
}
pub fn population(seed: u64, count: u32) -> Population {
    let mut g = Generator::new(seed);
    let initial: Vec<_> = (0..count).map(|id| g.agent(id)).collect();
    let mut sim = Simulation::new(initial.clone()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(
        seed,
        Sensor {
            effort: 5,
            ..Sensor::default()
        },
    )
    .unwrap();
    for id in (0..count.saturating_sub(1)).step_by(2) {
        sim.add_scene([id, id + 1], 1, 6).unwrap();
    }
    sim.run();
    for id in 0..count {
        sim.consume(id).unwrap();
    }
    Population {
        initial,
        final_agents: sim.agents(),
        events: sim.events(),
        scenes: sim.scenes(),
        cognition: sim.cognition(),
        intentional: sim.intentional().unwrap(),
        foresight: sim.foresight().unwrap(),
    }
}
pub fn human(trials: &[Trial]) -> String {
    use std::fmt::Write;
    let mut text =
        String::from("Experiment 004: local expectations, predictions, actual responses\n");
    for t in trials {
        writeln!(
            text,
            "\n{} seed {} event {} -> later {:?}",
            t.config.name, t.seed, t.target, t.probe.selected
        )
        .unwrap();
        for r in t
            .intentional
            .records
            .iter()
            .filter(|r| r.decision.input.event == t.target)
        {
            let forecast = t
                .foresight
                .forecasts
                .iter()
                .find(|f| f.talk_record == r.id)
                .unwrap();
            writeln!(text,"  record {} actor {} selects {:?}; expectation {:?}; evidence quality {}, effort {}",r.id,r.decision.input.own.id,r.decision.selected,forecast.context.expectation,forecast.context.reliability,forecast.context.effort).unwrap();
            for c in &forecast.candidates {
                writeln!(
                    text,
                    "    {:?}: immediate {} + anticipated {} = {}; predicts {:?} at {}%; {}",
                    c.action,
                    c.immediate,
                    c.anticipated,
                    c.combined,
                    c.expected_response,
                    c.probability,
                    c.consequence
                )
                .unwrap();
            }
            for error in t
                .foresight
                .errors
                .iter()
                .filter(|e| e.forecast_record == r.id)
            {
                writeln!(
                    text,
                    "    observed {:?}, error {}, expectation {} -> {}",
                    error.actual, error.error, error.expected, error.updated
                )
                .unwrap();
            }
            for info in t.cognition.information.iter().filter(|i| {
                i.event == t.target
                    && (r.information == Some(i.id)
                        || t.foresight
                            .readings
                            .iter()
                            .any(|v| v.talk_record == r.id && v.receipt == i.id))
            }) {
                writeln!(
                    text,
                    "    {:?}: support {:?} -> {}, revision {:?}, trust {} -> {}",
                    info.kind,
                    info.before.as_ref().map(|b| b.support),
                    info.after.support,
                    info.revision,
                    info.trust_before,
                    info.trust_after
                )
                .unwrap();
            }
        }
    }
    text
}
