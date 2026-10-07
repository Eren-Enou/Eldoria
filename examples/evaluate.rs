//! Controlled multi-seed evaluation; machine-readable summary on stdout.
use std::collections::BTreeMap;
use world_of_individuals::{experiment::run_experiment, model::Generator, simulation::Simulation};

fn increment(map: &mut BTreeMap<String, usize>, value: impl std::fmt::Debug) {
    *map.entry(format!("{value:?}")).or_default() += 1;
}

fn main() {
    let mut first_actions = BTreeMap::new();
    let mut matched_context_actions = BTreeMap::new();
    let mut outcomes = BTreeMap::new();
    let mut repeated_decisions_changed = 0;
    let mut repeated_first_actions_changed = 0;
    let mut repeated_outcomes_changed = 0;
    let mut history_probes_changed = 0;
    let mut reproducible = 0;
    let mut trust_positive = 0;
    let mut trust_negative = 0;
    for seed in 0..128 {
        let a = run_experiment(seed, "A", 2).unwrap();
        increment(&mut first_actions, a.trials[0].events[0].decision.selected);
        increment(&mut outcomes, a.trials[0].scenes[0].outcome.unwrap());
        trust_positive += a.metrics.positive_relationships;
        trust_negative += a.metrics.negative_relationships;
        let b = run_experiment(seed, "B", 2).unwrap();
        repeated_decisions_changed += b.metrics.decisions_changed_by_learning_ablation;
        let trial = &b.trials[0];
        if trial.events[trial.stages[0].first_event].decision.selected
            != trial.events[trial.stages[1].first_event].decision.selected
        {
            repeated_first_actions_changed += 1;
        }
        if trial.scenes[0].outcome != trial.scenes[1].outcome {
            repeated_outcomes_changed += 1;
        }
        let e = run_experiment(seed, "E", 2).unwrap();
        let c = e.history_comparison.as_ref().unwrap();
        if c.identical_present_except_learning
            && c.helpful_history_action != c.refusal_history_action
        {
            history_probes_changed += 1;
        }
        if e == run_experiment(seed, "E", 2).unwrap() {
            reproducible += 1;
        }
        let mut generator = Generator::new(seed);
        let mut sim = Simulation::new(vec![generator.agent(0), generator.agent(1)]).unwrap();
        let states = sim.agents();
        // Hold resources and hunger fixed; leave only generated dispositions different.
        sim.set_circumstances(0, 4, 40, states[0].generosity, states[0].caution)
            .unwrap();
        sim.set_circumstances(1, 0, 90, states[1].generosity, states[1].caution)
            .unwrap();
        sim.add_scene([0, 1], 1, 6).unwrap();
        sim.run();
        increment(
            &mut matched_context_actions,
            sim.events()[0].decision.selected,
        );
    }
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
        "seed_range": "0..128 (exclusive upper bound)",
        "first_encounter_actions": first_actions,
        "first_encounter_outcomes": outcomes,
        "matched_food_and_hunger_first_actions": matched_context_actions,
        "first_encounter_positive_directed_relationships": trust_positive,
        "first_encounter_negative_directed_relationships": trust_negative,
        "repeated_encounter_decisions_changed_by_learning_ablation": repeated_decisions_changed,
        "repeated_encounters_with_changed_first_action": repeated_first_actions_changed,
        "repeated_encounters_with_changed_outcome": repeated_outcomes_changed,
        "altered_history_probes_with_different_decisions": history_probes_changed,
        "altered_history_full_reports_reproduced": reproducible,
        "note": "Counts are separate measurements, not a composite interestingness score. Ablations compare each local decision, not a full downstream counterfactual trajectory."
    })).unwrap());
}
