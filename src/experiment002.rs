//! Controlled Experiment 002 fixtures: no scripted policy actions or inserted memories.
use crate::{behavior::scarcity_policy, cognition::*, model::*, simulation::Simulation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub label: String,
    pub agents: Vec<Agent>,
    pub cognition: Cognition,
    pub event_count: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub name: String,
    pub generated: Vec<Agent>,
    pub snapshots: Vec<Snapshot>,
    pub events: Vec<Event>,
    pub scenes: Vec<Scene>,
    pub cognition: Cognition,
    pub target: u64,
    pub probe: Decision,
}
fn snapshot(sim: &Simulation, label: &str, snapshots: &mut Vec<Snapshot>) {
    snapshots.push(Snapshot {
        label: label.into(),
        agents: sim.agents(),
        cognition: sim.cognition(),
        event_count: sim.events().len(),
    });
}
fn encounter(sim: &mut Simulation, pair: [AgentId; 2]) {
    sim.add_scene(pair, 1, 6).unwrap();
    sim.run();
}
/// History is generated through resolved offers, not fabricated trust or episodes.
pub fn trial(
    seed: u64,
    name: &str,
    helpful: bool,
    abundant: bool,
    evidence: &[EvidenceKind],
) -> Trial {
    let mut generator = Generator::new(seed);
    let generated = vec![generator.agent(0), generator.agent(1)];
    let mut sim = Simulation::new(generated.clone()).unwrap();
    sim.set_policy(scarcity_policy);
    let mut snapshots = vec![];
    for _ in 0..2 {
        sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
        sim.set_circumstances(
            1,
            4,
            if helpful { 0 } else { 90 },
            if helpful { 100 } else { 0 },
            if helpful { 0 } else { 90 },
        )
        .unwrap();
        snapshot(
            &sim,
            "history intervention: recipient needs food; donor helpful or guarded",
            &mut snapshots,
        );
        encounter(&mut sim, if helpful { [1, 0] } else { [0, 1] });
    }
    sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
    sim.set_circumstances(1, if abundant { 4 } else { 1 }, 90, 0, 90)
        .unwrap();
    snapshot(
        &sim,
        "target intervention: hungry guarded donor, scarce or abundant inventory",
        &mut snapshots,
    );
    encounter(&mut sim, [0, 1]);
    let target = sim.events().last().unwrap().id;
    assert_eq!(
        sim.events()[target as usize].decision.selected,
        Action::Refuse
    );
    snapshot(&sim, "refusal before new information", &mut snapshots);
    for &kind in evidence {
        sim.communicate(1, 0, target, kind).unwrap();
        snapshot(&sim, "after information scene", &mut snapshots);
    }
    sim.consume(0).unwrap();
    sim.consume(1).unwrap();
    snapshot(
        &sim,
        "consumption phase: food sink and unmet need",
        &mut snapshots,
    );
    sim.set_circumstances(0, 4, 30, 0, 80).unwrap();
    sim.set_circumstances(1, 0, 90, 30, 20).unwrap();
    snapshot(
        &sim,
        "matched present probe: identical physical state, retain learning",
        &mut snapshots,
    );
    let start = sim.events().len();
    encounter(&mut sim, [0, 1]);
    let probe = sim.events()[start].decision.clone();
    snapshot(&sim, "after probe", &mut snapshots);
    Trial {
        name: name.into(),
        generated,
        snapshots,
        events: sim.events(),
        scenes: sim.scenes(),
        cognition: sim.cognition(),
        target,
        probe,
    }
}

pub fn suite(seed: u64) -> Vec<Trial> {
    let claim = EvidenceKind::Testimony { scarce: true };
    let reveal = EvidenceKind::Disclosure;
    vec![
        trial(seed, "A-silent", true, false, &[]),
        trial(seed, "B-communicated", true, false, &[claim]),
        trial(seed, "C-delayed", true, false, &[claim, reveal]),
        trial(seed, "D-unverified-false", false, true, &[claim]),
        trial(seed, "E-conflicting", true, true, &[claim, reveal]),
        trial(seed, "F-negative-history", false, false, &[claim]),
        trial(seed, "F-positive-history", true, false, &[claim]),
        trial(
            seed,
            "C-negative-history-disclosure",
            false,
            false,
            &[claim, reveal],
        ),
    ]
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Population {
    pub initial: Vec<Agent>,
    pub final_agents: Vec<Agent>,
    pub events: Vec<Event>,
    pub scenes: Vec<Scene>,
    pub cognition: Cognition,
}
pub fn population(seed: u64, count: u32) -> Population {
    let mut generator = Generator::new(seed);
    let initial: Vec<_> = (0..count).map(|id| generator.agent(id)).collect();
    let mut sim = Simulation::new(initial.clone()).unwrap();
    sim.set_policy(scarcity_policy);
    for id in (0..count.saturating_sub(1)).step_by(2) {
        sim.add_scene([id, id + 1], 1, 6).unwrap();
    }
    sim.run();
    for event in sim.events() {
        if event.outcome == Some(Outcome::Refusal) {
            let speaker = event.decision.actor;
            let listener = *event
                .participants
                .iter()
                .find(|&&id| id != speaker)
                .unwrap();
            sim.communicate(speaker, listener, event.id, EvidenceKind::Disclosure)
                .unwrap();
        }
    }
    for id in 0..count {
        sim.consume(id).unwrap();
    }
    Population {
        initial,
        final_agents: sim.agents(),
        events: sim.events(),
        scenes: sim.scenes(),
        cognition: sim.cognition(),
    }
}
