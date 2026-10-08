use std::{collections::BTreeMap, fs};
use world_of_individuals::{
    concerns::{FollowAction, RULES},
    experiment, experiment002, experiment003, experiment004,
    experiment005::*,
};
fn main() {
    let dir = "experiments/005";
    let trials = suite(42);
    fs::write(
        format!("{dir}/seed-42.json"),
        serde_json::to_vec_pretty(&trials).unwrap(),
    )
    .unwrap();
    fs::write(format!("{dir}/report.txt"), human(&trials)).unwrap();
    let mut outcomes = BTreeMap::new();
    for seed in 0..128 {
        let trials = suite(seed);
        assert_eq!(trials, suite(seed));
        for t in trials {
            let s = t.snapshots.last().unwrap();
            let statuses: Vec<_> = s
                .concerns
                .items
                .get(&0)
                .unwrap()
                .iter()
                .map(|c| c.status)
                .collect();
            *outcomes
                .entry(format!(
                    "{}:{statuses:?}:probe={:?}",
                    t.config.name, t.probe.selected
                ))
                .or_insert(0) += 1;
        }
    }
    let p = population(42, 1000);
    assert_eq!(p, population(42, 1000));
    fs::write(
        format!("{dir}/population.json"),
        serde_json::to_vec_pretty(&p).unwrap(),
    )
    .unwrap();
    let archived = |path: &str| -> serde_json::Value {
        serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
    };
    let summary = serde_json::json!({"rules":RULES,"seeds":"0..128","replayed_suites":128,"outcomes":outcomes,
        "compatibility001":serde_json::to_value(experiment::run_experiment(42,"all",1000).unwrap()).unwrap()==archived("experiments/001/results/seed-42/report.json"),
        "compatibility002":serde_json::to_value(experiment002::suite(42)).unwrap()==archived("experiments/002/seed-42.json"),
        "compatibility003":serde_json::to_value(experiment003::suite(42)).unwrap()==archived("experiments/003/seed-42.json"),
        "compatibility004":serde_json::to_value(experiment004::suite(42)).unwrap()==archived("experiments/004/seed-42.json"),
        "population":{"agents":1000,"encounters_per_pair":3,"resource_events":p.events.len(),"talk_turns":p.intentional.records.len(),
            "concerns_created":p.concerns.creations.iter().filter(|c|c.retained).count(),
            "reopened":p.concerns.records.iter().filter(|r|matches!(r.decision.selected,FollowAction::Reopen(_))).count(),
            "abandoned":p.concerns.records.iter().filter(|r|matches!(r.decision.selected,FollowAction::Abandon(_))).count(),
            "receipts":p.cognition.information.len(),"serialized_bytes":serde_json::to_vec(&p).unwrap().len()}});
    fs::write(
        format!("{dir}/summary.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}
