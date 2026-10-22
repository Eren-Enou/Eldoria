use std::{fs, path::Path};
use woi_workbench::{
    config::Configuration,
    execution, history,
    storage::{self, Store},
};

fn store() -> Store {
    let mut s = Store::open(&woi_workbench::repository()).unwrap();
    s.root = s.root.join("acceptance-tests");
    fs::create_dir_all(&s.root).unwrap();
    s
}
fn fixture() -> serde_json::Value {
    serde_json::json!({
        "schema":"experiment-023-trial/v1",
        "config":{"case":"paired","seed":42,"horizon":192,"period":11,"mode":"Continuous","phase":"Epoch","order":"Ascending","duration":4,"cadence":8,"capacity":2,"role":"demo"},
        "checkpoints":[{"tick":231,"labels":["Final"],"active":[],"pending":[]}],
        "history": {"schema":"experiment-023-physical-history/v1","start":6,"end":231,"initial_total":4,"final_stock":0,
            "initial":[{"id":0,"food":1,"hunger":20},{"id":1,"food":1,"hunger":20},{"id":2,"food":1,"hunger":20},{"id":3,"food":1,"hunger":20}],
            "final_agents":[{"id":0,"food":1,"hunger":20},{"id":1,"food":1,"hunger":20},{"id":2,"food":1,"hunger":20},{"id":3,"food":1,"hunger":20}],
            "transitions":[],"grants":[],"boundaries":[{"tick":230,"choices":[],"events":0,"meals":0,"grants":0}],
            "final_state":{"base":{"base":{"base":{"base":{"base":{"events":[],"cognition":{"consumption":[]}}}}}}}
        }
    })
}
#[test]
fn typed_configuration_rejects_invalid_or_executable_inputs() {
    let mut c = Configuration::default();
    assert!(c.validate().is_ok());
    for ticks in [0, 1025, u64::MAX] {
        c.ticks = ticks;
        assert!(c.validate().is_err());
    }
    c.ticks = 1;
    c.period = 129;
    assert!(c.validate().is_err());
    c.period = 1;
    c.scenario = "unknown".into();
    assert!(c.validate().is_err());
    let mut v = serde_json::to_value(Configuration::default()).unwrap();
    v["mode"] = "MaterialPreparation".into();
    assert!(serde_json::from_value::<Configuration>(v.clone()).is_err());
    v["mode"] = "ContinuousWorld".into();
    v["program"] = "cmd.exe".into();
    assert!(serde_json::from_value::<Configuration>(v).is_err());
}
#[test]
fn outputs_are_unique_confined_and_never_overwritten() {
    let s = store();
    let (a, p) = s.create().unwrap();
    let (b, q) = s.create().unwrap();
    assert_ne!(a, b);
    assert_ne!(p, q);
    for bad in [
        "../experiments/023",
        "..",
        "C:\\experiments",
        "a/b",
        "a\\b",
        "",
        "/tmp/foo",
    ] {
        assert!(s.run(bad).is_err(), "{bad}");
    }
    storage::write_new(&p.join("configuration.json"), &Configuration::default()).unwrap();
    assert!(storage::write_new(&p.join("configuration.json"), &Configuration::default()).is_err());
    assert!(execution::metadata(&s, &a).is_err());
}
#[test]
fn reader_distinguishes_missing_corrupt_incomplete_unsupported_and_mismatch() {
    let s = store();
    let (_, p) = s.create().unwrap();
    let h = p.join("history.jsonl");
    let c = Configuration::default();
    assert!(history::read(&h, &c).unwrap_err().contains("Missing"));
    fs::write(&h, "{").unwrap();
    assert!(history::read(&h, &c).unwrap_err().contains("Incomplete"));
    fs::write(&h, "not json").unwrap();
    assert!(history::read(&h, &c).unwrap_err().contains("Corrupt"));
    let v = fixture();
    fs::write(&h, serde_json::to_vec(&v).unwrap()).unwrap();
    let summary = history::read(&h, &c).unwrap();
    assert_eq!(summary.final_agents.len(), 4);
    assert_eq!(summary.meals, 0);
    assert!(summary.food_accounting_matches);
    let mut v = fixture();
    v["schema"] = "future/v9".into();
    fs::write(&h, serde_json::to_vec(&v).unwrap()).unwrap();
    assert!(history::read(&h, &c).unwrap_err().contains("Unsupported"));
    fs::write(
        &h,
        r#"{"schema":"unknown-future/v2","entirely_different_body":[1,2]}"#,
    )
    .unwrap();
    assert!(history::read(&h, &c).unwrap_err().contains("Unsupported"));
    let mut v = fixture();
    v["checkpoints"] = serde_json::json!([]);
    fs::write(&h, serde_json::to_vec(&v).unwrap()).unwrap();
    assert!(history::read(&h, &c).unwrap_err().contains("Incomplete"));
    let mut v = fixture();
    v["config"]["seed"] = 43.into();
    fs::write(&h, serde_json::to_vec(&v).unwrap()).unwrap();
    assert!(history::read(&h, &c).unwrap_err().contains("configuration"));
    assert!(storage::read::<Configuration>(Path::new("missing-configuration.json")).is_err());
}
