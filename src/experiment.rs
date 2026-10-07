use crate::{behavior::utility_policy, model::*, simulation::Simulation};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stage {
    pub label: String,
    pub intervention: String,
    pub before: Vec<Agent>,
    pub after: Vec<Agent>,
    pub first_event: usize,
    pub end_event: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub name: String,
    pub generated: Vec<Agent>,
    pub stages: Vec<Stage>,
    pub events: Vec<Event>,
    pub scenes: Vec<Scene>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryComparison {
    pub identical_present_except_learning: bool,
    pub helpful_history_action: Action,
    pub refusal_history_action: Action,
    pub history_free_action: Action,
    pub helpful_history_outcome: Outcome,
    pub refusal_history_outcome: Outcome,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metrics {
    pub actions: BTreeMap<String, usize>,
    pub outcomes: BTreeMap<String, usize>,
    pub learned_decisions: usize,
    pub decisions_changed_by_learning_ablation: usize,
    pub divergent_interpretation_events: usize,
    pub positive_relationships: usize,
    pub negative_relationships: usize,
    pub max_memory_len: usize,
    pub total_events: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    pub rules: String,
    pub seed: u64,
    pub scenario: String,
    pub population: usize,
    pub trials: Vec<Trial>,
    pub history_comparison: Option<HistoryComparison>,
    pub reproducible: bool,
    pub metrics: Metrics,
}

fn generate(seed: u64, count: usize) -> Vec<Agent> {
    let mut generator = Generator::new(seed);
    (0..count).map(|id| generator.agent(id as u32)).collect()
}

fn stage(
    sim: &mut Simulation,
    label: &str,
    intervention: &str,
    pairs: &[[AgentId; 2]],
    stages: &mut Vec<Stage>,
) {
    let before = sim.agents();
    let first_event = sim.events().len();
    for &pair in pairs {
        sim.add_scene(pair, 1, 6).unwrap();
    }
    sim.run();
    stages.push(Stage {
        label: label.into(),
        intervention: intervention.into(),
        before,
        after: sim.agents(),
        first_event,
        end_event: sim.events().len(),
    });
}

fn basic_trial(seed: u64, scenario: &str, population: usize) -> Trial {
    let generated = generate(seed, population);
    let mut sim = Simulation::new(generated.clone()).unwrap();
    let mut stages = Vec::new();
    let pairs: Vec<_> = (0..population as u32 - 1)
        .step_by(2)
        .map(|i| [i, i + 1])
        .collect();
    let intervention = match scenario {
        "C" => {
            sim.set_circumstances(0, 0, 100, 10, 80).unwrap();
            sim.set_circumstances(1, 1, 100, 10, 80).unwrap();
            "Both need food; requester has none, potential donor has one unit and high caution."
        }
        "D" => {
            sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
            sim.set_circumstances(1, 1, 90, 0, 90).unwrap();
            "Force need and scarcity context, not actions; inspect each perspective on the resulting refusal."
        }
        _ => "None: generated state.",
    };
    stage(&mut sim, "encounter", intervention, &pairs, &mut stages);
    if scenario == "B" {
        for a in &generated {
            sim.set_circumstances(a.id, a.food, a.hunger, a.generosity, a.caution)
                .unwrap();
        }
        stage(
            &mut sim,
            "repeated encounter",
            "Restore generated food, hunger, generosity, caution; retain all learning.",
            &pairs,
            &mut stages,
        );
    }
    Trial {
        name: scenario.into(),
        generated,
        stages,
        events: sim.events(),
        scenes: sim.scenes(),
    }
}

fn history_trial(seed: u64, helpful: bool) -> Trial {
    let generated = generate(seed, 2);
    let mut sim = Simulation::new(generated.clone()).unwrap();
    let mut stages = Vec::new();
    sim.set_circumstances(0, 0, 90, 30, 20).unwrap();
    sim.set_circumstances(
        1,
        4,
        if helpful { 0 } else { 100 },
        if helpful { 100 } else { 0 },
        if helpful { 0 } else { 100 },
    )
    .unwrap();
    let pair = if helpful { [1, 0] } else { [0, 1] };
    stage(
        &mut sim,
        "history formation",
        "Controlled past: agent 0 needs food; agent 1 has food, with generous/satiated or guarded/hungry dispositions. Actions are selected by policy.",
        &[pair],
        &mut stages,
    );
    sim.set_circumstances(0, 4, 40, 30, 40).unwrap();
    sim.set_circumstances(1, 0, 90, 30, 20).unwrap();
    stage(
        &mut sim,
        "identical present probe",
        "Set identical present food, hunger, generosity, caution in both histories. Preserve expectations, trust and memories. Agent 0 starts.",
        &[[0, 1]],
        &mut stages,
    );
    Trial {
        name: if helpful { "E-helpful" } else { "E-refusal" }.into(),
        generated,
        stages,
        events: sim.events(),
        scenes: sim.scenes(),
    }
}

fn clear_learning(mut agent: Agent) -> Agent {
    agent.trust.clear();
    agent.memories.clear();
    agent
}

fn history_free_trial(seed: u64) -> Trial {
    let generated = generate(seed, 2);
    let mut baseline = Simulation::new(generated.clone()).unwrap();
    baseline.set_circumstances(0, 4, 40, 30, 40).unwrap();
    baseline.set_circumstances(1, 0, 90, 30, 20).unwrap();
    let mut stages = Vec::new();
    stage(
        &mut baseline,
        "history-free present probe",
        "Same present states; explicitly ablate trust and episodic memory.",
        &[[0, 1]],
        &mut stages,
    );
    Trial {
        name: "E-history-free".into(),
        generated,
        stages,
        events: baseline.events(),
        scenes: baseline.scenes(),
    }
}

fn metrics(trials: &[Trial]) -> Metrics {
    let mut m = Metrics {
        actions: BTreeMap::new(),
        outcomes: BTreeMap::new(),
        learned_decisions: 0,
        decisions_changed_by_learning_ablation: 0,
        divergent_interpretation_events: 0,
        positive_relationships: 0,
        negative_relationships: 0,
        max_memory_len: 0,
        total_events: 0,
    };
    for trial in trials {
        for scene in &trial.scenes {
            *m.outcomes
                .entry(format!("{:?}", scene.outcome.unwrap()))
                .or_default() += 1;
        }
        for event in &trial.events {
            m.total_events += 1;
            let d = &event.decision;
            *m.actions.entry(format!("{:?}", d.selected)).or_default() += 1;
            if d.trust != 0 || d.episodic_valence != 0 {
                m.learned_decisions += 1;
                let counterfactual = Agent {
                    id: d.actor,
                    food: d.food,
                    hunger: d.hunger,
                    generosity: d.generosity,
                    caution: d.caution,
                    expectation: d.expectation,
                    trust: BTreeMap::new(),
                    memories: Default::default(),
                };
                if utility_policy(&counterfactual, &d.observation).selected != d.selected {
                    m.decisions_changed_by_learning_ablation += 1;
                }
            }
            if event.interpretations[0].interpretation != event.interpretations[1].interpretation
                || event.interpretations[0].valence != event.interpretations[1].valence
            {
                m.divergent_interpretation_events += 1;
            }
        }
        for agent in &trial.stages.last().unwrap().after {
            m.max_memory_len = m.max_memory_len.max(agent.memories.len());
            for trust in agent.trust.values() {
                if *trust > 0 {
                    m.positive_relationships += 1;
                }
                if *trust < 0 {
                    m.negative_relationships += 1;
                }
            }
        }
    }
    m
}

pub fn run_experiment(seed: u64, scenario: &str, population: usize) -> Result<Report, String> {
    if !(2..=100_000).contains(&population) {
        return Err("population must be in 2..=100000".into());
    }
    if !["A", "B", "C", "D", "E", "F", "G", "all"].contains(&scenario) {
        return Err("scenario must be A..G or all".into());
    }
    let mut trials = Vec::new();
    for name in ["A", "B", "C", "D", "G"] {
        if scenario == name || scenario == "all" {
            trials.push(basic_trial(
                seed,
                name,
                if name == "G" { population } else { 2 },
            ));
        }
    }
    let mut history_comparison = None;
    if scenario == "E" || scenario == "all" {
        let helpful = history_trial(seed, true);
        let refusal = history_trial(seed, false);
        let present_helpful = &helpful.stages[1].before;
        let present_refusal = &refusal.stages[1].before;
        let fresh: Vec<_> = present_helpful
            .iter()
            .cloned()
            .map(clear_learning)
            .collect();
        let baseline = history_free_trial(seed);
        history_comparison = Some(HistoryComparison {
            identical_present_except_learning: fresh == baseline.stages[0].before
                && fresh
                    == present_refusal
                        .iter()
                        .cloned()
                        .map(clear_learning)
                        .collect::<Vec<_>>(),
            helpful_history_action: helpful.events[helpful.stages[1].first_event]
                .decision
                .selected,
            refusal_history_action: refusal.events[refusal.stages[1].first_event]
                .decision
                .selected,
            history_free_action: baseline.events[0].decision.selected,
            helpful_history_outcome: helpful.scenes[1].outcome.unwrap(),
            refusal_history_outcome: refusal.scenes[1].outcome.unwrap(),
        });
        trials.push(helpful);
        trials.push(refusal);
        trials.push(baseline);
    }
    if scenario == "F" || scenario == "all" {
        trials.push(basic_trial(seed, "F", 2));
    }
    // Verify all selected complete traces, including the history-free branch.
    let reproducible = trials.iter().all(|t| match t.name.as_str() {
        "E-helpful" => *t == history_trial(seed, true),
        "E-refusal" => *t == history_trial(seed, false),
        "E-history-free" => *t == history_free_trial(seed),
        name => *t == basic_trial(seed, name, if name == "G" { population } else { 2 }),
    });
    let measurements = metrics(&trials);
    Ok(Report {
        rules: RULES_VERSION.into(),
        seed,
        scenario: scenario.into(),
        population,
        trials,
        history_comparison,
        reproducible,
        metrics: measurements,
    })
}

pub fn human_report(report: &Report) -> String {
    use std::fmt::Write;
    let mut out = format!(
        "World of Individuals / {}\nSeed {} | Scenario {} | Reproducible {}\n",
        report.rules, report.seed, report.scenario, report.reproducible
    );
    for trial in &report.trials {
        writeln!(
            out,
            "\nTrial {} ({} generated agents)",
            trial.name,
            trial.generated.len()
        )
        .unwrap();
        for a in trial.generated.iter().take(8) {
            writeln!(
                out,
                "  Generated {}: food={} hunger={} generosity={} caution={} expectation={}",
                a.id, a.food, a.hunger, a.generosity, a.caution, a.expectation
            )
            .unwrap();
        }
        for stage in &trial.stages {
            writeln!(out, "  {}: {}", stage.label, stage.intervention).unwrap();
            for event in trial.events[stage.first_event..stage.end_event]
                .iter()
                .take(12)
            {
                let d = &event.decision;
                writeln!(out, "    Event {} tick {} scene {}: {} {:?}; candidates={:?}; trust={} episodes={} causes={:?}; food {:?}->{:?}; result {:?}",
                    event.id, event.tick, event.scene, d.actor, d.selected, d.candidates, d.trust, d.episodic_valence, d.memory_causes,
                    event.balances_before, event.balances_after, event.outcome).unwrap();
                writeln!(out, "      Subjective records: {:?}", event.interpretations).unwrap();
            }
            for (before, after) in stage.before.iter().zip(&stage.after).take(8) {
                writeln!(
                    out,
                    "    Agent {} food {}->{} trust {:?}->{:?}; memories {}->{}",
                    before.id,
                    before.food,
                    after.food,
                    before.trust,
                    after.trust,
                    before.memories.len(),
                    after.memories.len()
                )
                .unwrap();
            }
        }
    }
    if let Some(c) = &report.history_comparison {
        writeln!(out, "\nControlled history comparison: {c:?}").unwrap();
    }
    writeln!(out, "\nSeparate measurements: {:?}\nHuman output is capped per stage; JSON retains every agent/event.\nInterpretations and utility scores are model hypotheses, not psychological facts.", report.metrics).unwrap();
    out
}
