use std::{collections::BTreeMap, fs};
use world_of_individuals::{
    experiment, experiment002, experiment003, experiment004::*, foresight::RULES,
};
fn main() {
    let dir = "experiments/004";
    fs::create_dir_all(dir).unwrap();
    let trials = suite(42);
    fs::write(
        format!("{dir}/seed-42.json"),
        serde_json::to_vec_pretty(&trials).unwrap(),
    )
    .unwrap();
    fs::write(format!("{dir}/report.txt"), human(&trials)).unwrap();
    let mut choices = BTreeMap::new();
    let mut probes = BTreeMap::new();
    let mut errors = 0;
    let mut noise = 0;
    for seed in 0..128 {
        let t = suite(seed);
        assert_eq!(t, suite(seed));
        for trial in &t {
            let first = trial
                .intentional
                .records
                .iter()
                .find(|r| r.decision.input.event == trial.target)
                .unwrap();
            *choices
                .entry(format!(
                    "{}:{:?}",
                    trial.config.name, first.decision.selected
                ))
                .or_insert(0) += 1;
            *probes
                .entry(format!("{}:{:?}", trial.config.name, trial.probe.selected))
                .or_insert(0) += 1;
        }
        noise += t[16].foresight.readings.len();
        errors += t[16]
            .foresight
            .readings
            .iter()
            .filter(|r| !r.objectively_correct)
            .count();
    }
    let p = population(42, 1000);
    assert_eq!(p, population(42, 1000));
    fs::write(
        format!("{dir}/population.json"),
        serde_json::to_vec_pretty(&p).unwrap(),
    )
    .unwrap();
    let mut actions = BTreeMap::new();
    for r in &p.intentional.records {
        *actions
            .entry(format!("{:?}", r.decision.selected))
            .or_insert(0) += 1;
    }
    let archived = |path: &str| -> serde_json::Value {
        serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
    };
    let summary = serde_json::json!({"rules":RULES,"seeds":"0..128","replayed_suites":128,"opening_choices":choices,"probe_choices":probes,
        "noisy_readings":noise,"incorrect_noisy_readings":errors,
        "compatibility001":serde_json::to_value(experiment::run_experiment(42,"all",1000).unwrap()).unwrap()==archived("experiments/001/results/seed-42/report.json"),
        "compatibility002":serde_json::to_value(experiment002::suite(42)).unwrap()==archived("experiments/002/seed-42.json"),
        "compatibility003":serde_json::to_value(experiment003::suite(42)).unwrap()==archived("experiments/003/seed-42.json"),
        "population":{"agents":1000,"resource_events":p.events.len(),"conversations":p.intentional.conversations.len(),"actions":actions,
            "predictions_scored":p.foresight.errors.len(),"readings":p.foresight.readings.len(),"incorrect_readings":p.foresight.readings.iter().filter(|r|!r.objectively_correct).count(),
            "revisions":p.cognition.information.iter().filter(|i|i.revision.is_some()).count(),"serialized_bytes":serde_json::to_vec(&p).unwrap().len(),
            "consumed":p.cognition.consumption.iter().map(|c|u64::from(c.consumed)).sum::<u64>()}});
    fs::write(
        format!("{dir}/summary.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}
