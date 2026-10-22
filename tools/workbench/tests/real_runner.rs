//! Explicit end-to-end acceptance; writes exploratory output only, never archives.
use woi_workbench::{
    config::{Configuration, SCENARIOS},
    execution,
    storage::Store,
};

#[test]
#[ignore = "explicit real runner scenario and boundary acceptance"]
fn all_exposed_presets_and_duration_period_seed_bounds_execute() {
    let mut store = Store::open(&woi_workbench::repository()).unwrap();
    store.root = store.root.join("acceptance-tests");
    std::fs::create_dir_all(&store.root).unwrap();
    let matrix: Vec<serde_json::Value> = serde_json::from_str(include_str!(
        "../../../experiments/023/registered-matrix.json"
    ))
    .unwrap();
    let actual: std::collections::BTreeSet<_> =
        matrix.iter().map(|m| m["case"].as_str().unwrap()).collect();
    assert_eq!(actual, SCENARIOS.into_iter().collect());
    for scenario in SCENARIOS {
        let config = Configuration {
            scenario: scenario.into(),
            ticks: 64,
            ..Default::default()
        };
        let result = execution::run(&store, config, None).unwrap();
        assert_eq!(
            result.status, "Completed",
            "{scenario}: {:?}",
            result.diagnostic
        );
        let summary = execution::inspect(&store, &result.id).unwrap();
        assert!(summary.food_accounting_matches);
    }
    for (ticks, period, seed) in [(1, 128, u64::MAX), (1024, 1, 0)] {
        let result = execution::run(
            &store,
            Configuration {
                ticks,
                period,
                seed,
                ..Default::default()
            },
            None,
        )
        .unwrap();
        assert_eq!(
            result.status, "Completed",
            "{ticks}/{period}: {:?}",
            result.diagnostic
        );
        let summary = execution::inspect(&store, &result.id).unwrap();
        assert_eq!(summary.end - summary.start, ticks + 33);
        assert_eq!(summary.final_agents.len(), 4);
    }
}
