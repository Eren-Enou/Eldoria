#![cfg(feature = "world-sessions")]
use std::{fs, path::Path, process::Command};
fn call(args: &[&str]) -> serde_json::Value {
    let out = Command::new(env!("CARGO_BIN_EXE_woi-workbench"))
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).unwrap()
}
#[test]
fn actual_workbench_cli_recovers_and_finalizes_all_supported_modes() {
    let root = woi_workbench::repository()
        .join("target/workbench-headless-validation")
        .join(format!(
            "cli-{}-{}",
            std::process::id(),
            woi_workbench::storage::now()
        ));
    fs::create_dir_all(&root).unwrap();
    for (index, mode) in [None, Some("Baseline"), Some("MealRelief")]
        .into_iter()
        .enumerate()
    {
        let config = root.join(format!("config-{index}.json"));
        let mut value = serde_json::json!({"schema_version":1,"mode":"ContinuousWorld","scenario":"paired","seed":43,"ticks":32,"period":11});
        if let Some(mode) = mode {
            value["schema_version"] = 2.into();
            value["mode"] = "ActivityValuation".into();
            value["valuation"] = mode.into();
        }
        fs::write(&config, serde_json::to_vec(&value).unwrap()).unwrap();
        let created = call(&["world-create", config.to_str().unwrap()]);
        let id = created["id"].as_str().unwrap();
        let saved = call(&["world-advance", id, "3", "--codec", "zlib"]);
        assert!(!saved["complete"].as_bool().unwrap());
        let parent = Path::new(created["path"].as_str().unwrap());
        let bytes = fs::read(parent.join("journal.jsonl")).unwrap();
        let interrupted = call(&["world-interrupt", id, "5"]);
        assert_eq!(interrupted["checkpoint_saved"], false);
        let original = fs::read(parent.join("journal.jsonl")).unwrap();
        assert!(original.len() > bytes.len());
        let report = call(&["world-check", id]);
        assert_eq!(report["latest_valid"], 2);
        let warning = call(&["world-storage", id, "--budget-bytes", "1"]);
        assert!(!warning["warnings"].as_array().unwrap().is_empty());
        call(&["world-verify", id, "--generation", "2"]);
        let base = root.join(format!("branch-{index}"));
        fs::create_dir(&base).unwrap();
        let branch = base.join(id);
        let name = branch.to_str().unwrap();
        let recovered = call(&["world-recover", id, name, "--generation", "2"]);
        assert_eq!(recovered["tick"], saved["tick"]);
        let terminal = call(&["world-advance", name, "2048", "--codec", "zlib"]);
        assert_eq!(terminal["complete"], true);
        call(&["world-finalize", name]);
        call(&["world-verify", name, "--generation", "3"]);
        assert_eq!(fs::read(parent.join("journal.jsonl")).unwrap(), original);
        assert!(branch.join("history.jsonl").exists());
        let retry = Command::new(env!("CARGO_BIN_EXE_woi-workbench"))
            .args(["world-recover", id, name, "--generation", "2"])
            .output()
            .unwrap();
        assert!(!retry.status.success());
    }
}
