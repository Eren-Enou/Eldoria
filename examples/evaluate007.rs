use std::{collections::BTreeMap, fs};
use world_of_individuals::{
    experiment, experiment002, experiment003, experiment004, experiment005, experiment006,
    experiment007::*, provenance::RULES,
};
fn main() {
    let dir = "experiments/007";
    fs::create_dir_all(dir).unwrap();
    let trials = suite(42);
    fs::write(
        format!("{dir}/seed-42.json"),
        serde_json::to_vec_pretty(&trials).unwrap(),
    )
    .unwrap();
    fs::write(format!("{dir}/report.txt"), human(&trials)).unwrap();
    let mut outcomes = BTreeMap::new();
    for seed in 0..128 {
        let run = suite(seed);
        assert_eq!(run, suite(seed));
        for trial in run {
            let p = &trial.final_state.provenance;
            let evaluations: Vec<_> = p
                .receipts
                .iter()
                .filter(|r| r.listener == 0)
                .map(|r| {
                    (
                        r.event,
                        r.evaluation.kind,
                        r.evaluation.incremental_value,
                        r.evaluation.after,
                    )
                })
                .collect();
            let actions: Vec<_> = p
                .queries
                .iter()
                .filter(|q| q.decision.input.base.owner == 0)
                .map(|q| q.decision.decision.selected)
                .collect();
            *outcomes
                .entry(format!("{}:{evaluations:?}:{actions:?}", trial.name))
                .or_insert(0) += 1;
        }
    }
    let population = population(42, 1000);
    assert_eq!(
        population,
        world_of_individuals::experiment007::population(42, 1000)
    );
    fs::write(
        format!("{dir}/population.json"),
        serde_json::to_vec_pretty(&population).unwrap(),
    )
    .unwrap();
    let archived = |path: &str| -> serde_json::Value {
        serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
    };
    let compatibility = serde_json::json!({
        "001":serde_json::to_value(experiment::run_experiment(42,"all",1000).unwrap()).unwrap()==archived("experiments/001/results/seed-42/report.json"),
        "002":serde_json::to_value(experiment002::suite(42)).unwrap()==archived("experiments/002/seed-42.json"),
        "003":serde_json::to_value(experiment003::suite(42)).unwrap()==archived("experiments/003/seed-42.json"),
        "004":serde_json::to_value(experiment004::suite(42)).unwrap()==archived("experiments/004/seed-42.json"),
        "005":serde_json::to_value(experiment005::suite(42)).unwrap()==archived("experiments/005/seed-42.json"),
        "006":serde_json::to_value(experiment006::suite(42)).unwrap()==archived("experiments/006/seed-42.json")});
    assert!(
        compatibility
            .as_object()
            .unwrap()
            .values()
            .all(|v| v == true)
    );
    fs::write(
        format!("{dir}/compatibility.json"),
        serde_json::to_vec_pretty(&compatibility).unwrap(),
    )
    .unwrap();
    let summary = serde_json::json!({"rules":RULES,"seed_range":"0..128","replayed_suites":128,"trials":128*trials.len(),"outcomes":outcomes,"compatibility":compatibility,
        "seed42":trials.iter().map(|t|(t.name.clone(),metrics(&t.final_state))).collect::<BTreeMap<_,_>>(),"population":metrics(&population),
        "population006_serialized_bytes":serde_json::to_vec(&experiment006::population(42,1000)).unwrap().len()});
    fs::write(
        format!("{dir}/summary.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}
