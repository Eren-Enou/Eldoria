//! Experiment 003: fixtures configure circumstances, never individual messages.
use crate::{
    behavior::scarcity_policy, cognition::Cognition, intentional::*, model::*,
    simulation::Simulation,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub intervention: String,
    pub agents: Vec<Agent>,
    pub cognition: Cognition,
    pub intentional: Intentional,
    pub events: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub name: String,
    pub seed: u64,
    pub snapshots: Vec<Snapshot>,
    pub events: Vec<Event>,
    pub scenes: Vec<Scene>,
    pub cognition: Cognition,
    pub intentional: Intentional,
    pub target: u64,
    pub probe: Decision,
}
fn snap(sim: &Simulation, label: &str, out: &mut Vec<Snapshot>) {
    out.push(Snapshot {
        intervention: label.into(),
        agents: sim.agents(),
        cognition: sim.cognition(),
        intentional: sim.intentional().unwrap(),
        events: sim.events().len(),
    });
}
fn encounter(sim: &mut Simulation) {
    sim.add_scene([0, 1], 1, 6).unwrap();
    sim.run();
}
fn circumstances(sim: &mut Simulation, food: u32) {
    sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
    sim.set_circumstances(1, food, 90, 0, 90).unwrap();
}
fn profiles(sim: &mut Simulation, privacy: i32, goal: i32, cost: i32, honesty: i32) {
    sim.set_communication_profile(
        0,
        Profile {
            privacy: 20,
            relationship_goal: goal,
            honesty: 80,
            request_cost: cost,
        },
    )
    .unwrap();
    sim.set_communication_profile(
        1,
        Profile {
            privacy,
            relationship_goal: 60,
            honesty,
            request_cost: 40,
        },
    )
    .unwrap();
}
/// history: 0 none, 1 verified, 2 unanswered request, 3 contradicted claim.
pub fn trial(
    seed: u64,
    name: &str,
    privacy: i32,
    listener_goal: i32,
    request_cost: i32,
    honesty: i32,
    history: u8,
) -> Trial {
    let mut g = Generator::new(seed);
    let mut sim = Simulation::new(vec![g.agent(0), g.agent(1)]).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_intentional().unwrap();
    let mut snapshots = vec![];
    for _ in 0..u32::from(history > 0) {
        profiles(
            &mut sim,
            if history == 2 { 100 } else { 0 },
            60,
            40,
            if history == 3 { 0 } else { 80 },
        );
        circumstances(&mut sim, if history == 3 { 4 } else { 1 });
        snap(
            &sim,
            "history formation: profile/physical intervention, messages selected by policy",
            &mut snapshots,
        );
        encounter(&mut sim);
        snap(&sim, "after communication history", &mut snapshots);
    }
    profiles(&mut sim, privacy, listener_goal, request_cost, honesty);
    circumstances(&mut sim, if name.starts_with("G") { 4 } else { 1 });
    snap(
        &sim,
        "target refusal: matched physical state, configured disclosure preferences",
        &mut snapshots,
    );
    encounter(&mut sim);
    let target = sim.events().last().unwrap().id;
    assert_eq!(
        sim.events().last().unwrap().decision.selected,
        Action::Refuse
    );
    snap(
        &sim,
        "after automatically selected communication",
        &mut snapshots,
    );
    sim.consume(0).unwrap();
    sim.consume(1).unwrap();
    snap(&sim, "explicit consumption phase", &mut snapshots);
    // A small positive unlearned offer score allows revised blame to cross a boundary.
    sim.set_circumstances(0, 4, 30, 60, 80).unwrap();
    sim.set_circumstances(1, 0, 90, 30, 20).unwrap();
    snap(
        &sim,
        "matched subsequent resource probe: only learned state differs",
        &mut snapshots,
    );
    let start = sim.events().len();
    encounter(&mut sim);
    let probe = sim.events()[start].decision.clone();
    snap(&sim, "after resource probe", &mut snapshots);
    Trial {
        name: name.into(),
        seed,
        snapshots,
        events: sim.events(),
        scenes: sim.scenes(),
        cognition: sim.cognition(),
        intentional: sim.intentional().unwrap(),
        target,
        probe,
    }
}
pub fn suite(seed: u64) -> Vec<Trial> {
    vec![
        trial(seed, "A-disclose", 0, 60, 40, 80, 0),
        trial(seed, "A-silent", 100, 60, 40, 80, 0),
        trial(seed, "B-cost-40", 40, 60, 40, 80, 0),
        trial(seed, "C-no-request", 35, 0, 40, 80, 0),
        trial(seed, "C-request", 35, 60, 40, 80, 0),
        trial(seed, "D-unsupported", 0, 0, 100, 80, 0),
        trial(seed, "D-evidence", 0, 0, 0, 80, 0),
        trial(seed, "E-honest", 0, 0, 100, 80, 1),
        trial(seed, "E-unhelpful", 0, 0, 100, 80, 2),
        trial(seed, "E-contradicted", 0, 0, 100, 80, 3),
        trial(seed, "F-conceal", 100, 0, 40, 80, 0),
        trial(seed, "G-deceive", 0, 0, 100, 0, 0),
        trial(seed, "G-honest-control", 0, 0, 100, 100, 0),
        trial(seed, "G-challenged", 0, 60, 0, 0, 0),
        trial(seed, "E-request-honest", 35, 15, 40, 80, 1),
        trial(seed, "E-request-unhelpful", 35, 15, 40, 80, 2),
    ]
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Population {
    pub initial: Vec<Agent>,
    pub final_agents: Vec<Agent>,
    pub events: Vec<Event>,
    pub scenes: Vec<Scene>,
    pub cognition: Cognition,
    pub intentional: Intentional,
}
pub fn population(seed: u64, count: u32) -> Population {
    let mut g = Generator::new(seed);
    let initial: Vec<_> = (0..count).map(|id| g.agent(id)).collect();
    let mut sim = Simulation::new(initial.clone()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_intentional().unwrap();
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
    }
}
pub fn human(trials: &[Trial]) -> String {
    use std::fmt::Write;
    let mut text = String::from(
        "Experiment 003: intentional communication\nScores describe rules, not inferred psychology.\n",
    );
    for t in trials {
        writeln!(
            text,
            "\n{} seed {} target event {} -> later {:?} (trust {}, recency {})",
            t.name, t.seed, t.target, t.probe.selected, t.probe.trust, t.probe.episodic_valence
        )
        .unwrap();
        for conversation in t
            .intentional
            .conversations
            .iter()
            .filter(|c| c.event == t.target)
        {
            for r in &t.intentional.records[conversation.first_record..conversation.end_record] {
                writeln!(text,"  record {} actor {} {:?}; alternatives {:?}; claim {:?}, objective true {:?}, evidence {:?}; credibility {} -> {}",r.id,r.decision.input.own.id,r.decision.selected,r.decision.candidates,r.claim,r.objectively_true,r.information,r.credibility_before,r.credibility_after).unwrap();
                if let Some(id) = r.information {
                    let info = &t.cognition.information[id as usize];
                    writeln!(
                        text,
                        "    support {:?} -> {}; revision {:?}; trust {} -> {}",
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
    }
    text
}
