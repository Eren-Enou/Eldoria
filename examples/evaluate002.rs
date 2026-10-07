use std::{fs, time::Instant};
use world_of_individuals::{
    cognition::RULES,
    experiment::run_experiment,
    experiment002::{population, suite},
};
fn main() {
    let dir = "experiments/002";
    fs::create_dir_all(dir).unwrap();
    let trials = suite(42);
    fs::write(
        format!("{dir}/seed-42.json"),
        serde_json::to_vec_pretty(&trials).unwrap(),
    )
    .unwrap();
    let mut changed_claim = 0;
    let mut changed_disclosure = 0;
    let mut revised_claim = 0;
    let mut revised_disclosure = 0;
    let mut false_claim_accepted = 0;
    let mut replayed = 0;
    for seed in 0..128 {
        let t = suite(seed);
        changed_claim += usize::from(t[0].probe.selected != t[1].probe.selected);
        changed_disclosure += usize::from(t[0].probe.selected != t[2].probe.selected);
        revised_claim += usize::from(t[1].cognition.information[0].revision.is_some());
        revised_disclosure += usize::from(t[2].cognition.information[1].revision.is_some());
        false_claim_accepted += usize::from(t[4].cognition.information[0].revision.is_some());
        replayed += usize::from(t == suite(seed));
    }
    let start = Instant::now();
    let pop = population(42, 1000);
    let elapsed = start.elapsed().as_secs_f64();
    let pop_replayed = pop == population(42, 1000);
    let consumed: u64 = pop
        .cognition
        .consumption
        .iter()
        .map(|c| u64::from(c.consumed))
        .sum();
    let pop_bytes = serde_json::to_vec(&pop).unwrap().len();
    fs::write(
        format!("{dir}/population.json"),
        serde_json::to_vec_pretty(&pop).unwrap(),
    )
    .unwrap();
    let old = run_experiment(42, "all", 1000).unwrap();
    let archived: serde_json::Value =
        serde_json::from_slice(&fs::read("experiments/001/results/seed-42/report.json").unwrap())
            .unwrap();
    let compatibility = serde_json::to_value(&old).unwrap() == archived;
    let summary = serde_json::json!({
        "rules":RULES,"seeds":"0..128", "full_suites_reproduced":replayed,
        "testimony_changed_probe":changed_claim,"disclosure_changed_probe":changed_disclosure,
        "testimony_revised_memory":revised_claim,"disclosure_added_revision":revised_disclosure,
        "false_claim_initially_accepted_with_helpful_history":false_claim_accepted,
        "population_reproduced":pop_replayed,"population":1000,
        "population_resource_events":pop.events.len(),
        "population_information_scenes":pop.cognition.information.len(),
        "population_revisions":pop.cognition.information.iter().filter(|i|i.revision.is_some()).count(),
        "population_consumed":consumed,"population_serialized_bytes":pop_bytes,
        "experiment001_saved_report_equal":compatibility,
        "seed42_probes":trials.iter().map(|t|serde_json::json!({"name":t.name,"action":t.probe.selected,"trust":t.probe.trust,"recency":t.probe.episodic_valence,"information":t.cognition.information})).collect::<Vec<_>>()
    });
    fs::write(
        format!("{dir}/summary.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    fs::write(
        format!("{dir}/population-benchmark.csv"),
        format!("population,seconds,serialized_bytes\n1000,{elapsed:.6},{pop_bytes}\n"),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}
