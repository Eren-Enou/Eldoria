use std::{collections::BTreeMap, fs};
use world_of_individuals::{
    experiment, experiment002,
    experiment003::*,
    intentional::{RULES, TalkAction},
};
fn main() {
    let dir = "experiments/003";
    fs::create_dir_all(dir).unwrap();
    let trials = suite(42);
    fs::write(
        format!("{dir}/seed-42.json"),
        serde_json::to_vec_pretty(&trials).unwrap(),
    )
    .unwrap();
    fs::write(format!("{dir}/report.txt"), human(&trials)).unwrap();
    let mut actions = BTreeMap::new();
    let mut comparisons = BTreeMap::new();
    let mut history_changed_requests = 0;
    let mut motivated_deceptions = 0;
    for seed in 0..128 {
        let t = suite(seed);
        assert_eq!(t, suite(seed));
        let first_listener = |trial: &Trial| {
            trial
                .intentional
                .records
                .iter()
                .find(|r| r.decision.input.event == trial.target && !r.decision.input.is_refuser)
                .unwrap()
                .decision
                .selected
        };
        history_changed_requests += u32::from(first_listener(&t[14]) != first_listener(&t[15]));
        motivated_deceptions += u32::from(
            t[11].intentional.records[0].decision.selected == TalkAction::Mislead
                && t[12].intentional.records[0].decision.selected != TalkAction::Mislead,
        );
        for trial in &t {
            *actions
                .entry(format!("{}:{:?}", trial.name, trial.probe.selected))
                .or_insert(0) += 1;
        }
        for (label, a, b) in [
            ("privacy_changes_probe", 0, 1),
            ("request_changes_probe", 3, 4),
            ("evidence_changes_probe", 5, 6),
            ("communication_history_changes_probe", 7, 8),
        ] {
            *comparisons.entry(label).or_insert(0) +=
                u32::from(t[a].probe.selected != t[b].probe.selected);
        }
    }
    let p = population(42, 1000);
    assert_eq!(p, population(42, 1000));
    fs::write(
        format!("{dir}/population.json"),
        serde_json::to_vec_pretty(&p).unwrap(),
    )
    .unwrap();
    let mut communications = BTreeMap::new();
    for r in &p.intentional.records {
        *communications
            .entry(format!("{:?}", r.decision.selected))
            .or_insert(0) += 1;
    }
    let old1: serde_json::Value =
        serde_json::from_slice(&fs::read("experiments/001/results/seed-42/report.json").unwrap())
            .unwrap();
    let old2: serde_json::Value =
        serde_json::from_slice(&fs::read("experiments/002/seed-42.json").unwrap()).unwrap();
    let summary = serde_json::json!({"rules":RULES,"seed_range":"0..128","reproduced_suites":128,"comparisons":comparisons,"probe_actions":actions,
        "history_changed_request_choice":history_changed_requests,"honesty_changed_deception_choice":motivated_deceptions,
        "compatibility001":serde_json::to_value(experiment::run_experiment(42,"all",1000).unwrap()).unwrap()==old1,
        "compatibility002":serde_json::to_value(experiment002::suite(42)).unwrap()==old2,
        "population":{"agents":1000,"resource_events":p.events.len(),"conversations":p.intentional.conversations.len(),"actions":communications,
            "information":p.cognition.information.len(),"revisions":p.cognition.information.iter().filter(|i|i.revision.is_some()).count(),
            "consumed":p.cognition.consumption.iter().map(|c|u64::from(c.consumed)).sum::<u64>(),"serialized_bytes":serde_json::to_vec(&p).unwrap().len(),
            "deceptive_claims":p.intentional.records.iter().filter(|r|r.decision.selected==TalkAction::Mislead).count()}});
    fs::write(
        format!("{dir}/summary.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}
