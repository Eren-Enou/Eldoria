//! Compatibility evidence stays outside historical experiment directories.
use serde::Serialize;
use std::{collections::BTreeMap, fs};
use world_of_individuals::{
    audit::CompactSnapshot, experiment, experiment002, experiment003, experiment004, experiment005,
    experiment006, experiment007,
};
fn same<T: Serialize>(actual: T, path: &str) -> bool {
    let expected: serde_json::Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    serde_json::to_value(actual).unwrap() == expected
}
fn main() {
    let mut compatibility = BTreeMap::new();
    compatibility.insert(
        "001",
        same(
            experiment::run_experiment(42, "all", 1000).unwrap(),
            "experiments/001/results/seed-42/report.json",
        ),
    );
    compatibility.insert(
        "002",
        same(experiment002::suite(42), "experiments/002/seed-42.json"),
    );
    compatibility.insert(
        "003",
        same(experiment003::suite(42), "experiments/003/seed-42.json"),
    );
    compatibility.insert(
        "004",
        same(experiment004::suite(42), "experiments/004/seed-42.json"),
    );
    compatibility.insert(
        "005",
        same(experiment005::suite(42), "experiments/005/seed-42.json"),
    );
    compatibility.insert(
        "006",
        same(experiment006::suite(42), "experiments/006/seed-42.json"),
    );
    let suite = experiment007::suite(42);
    compatibility.insert("007", same(&suite, "experiments/007/seed-42.json"));
    assert!(compatibility.values().all(|v| *v));
    let mut snapshots = 0;
    for seed in [0, 42, u64::MAX] {
        for t in experiment007::suite(seed) {
            for s in std::iter::once(&t.initial)
                .chain(&t.checkpoints)
                .chain(std::iter::once(&t.final_state))
            {
                let compact = CompactSnapshot::from_legacy(s).unwrap();
                assert_eq!(&compact.expand().unwrap(), s);
                snapshots += 1;
            }
        }
    }
    let result = serde_json::json!({"archived_structural_equality":compatibility,"compact_controlled_snapshots_reconstructed":snapshots,
        "compact_seeds":["0","42","u64::MAX"],"historical_files_rewritten":false});
    fs::write(
        "reinforcement/compatibility.json",
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
