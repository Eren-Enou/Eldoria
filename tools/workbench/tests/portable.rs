use std::{fs, path::PathBuf};
use woi_workbench::{execution, portable::*, recorded, storage};
fn scratch() -> PathBuf {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let root = std::env::temp_dir().join(format!(
        "WOI safety {} {} {}",
        std::process::id(),
        storage::now(),
        NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    fs::create_dir_all(&root).unwrap();
    root
}
#[test]
fn installation_checkout_archives_and_traversal_are_rejected_before_writes() {
    let root = scratch();
    let install = root.join("Installed app");
    fs::create_dir(&install).unwrap();
    for path in [
        install.join("data"),
        root.join("experiments/new"),
        root.join("reinforcement/new"),
        root.join("data/../bad"),
    ] {
        assert!(
            safe_workspace(&path, &install).is_err(),
            "{}",
            path.display()
        );
        assert!(!path.exists());
    }
    let checkout = root.join("checkout");
    fs::create_dir_all(checkout.join(".git")).unwrap();
    assert!(safe_workspace(&checkout.join("data"), &install).is_err());
    assert!(!checkout.join("data").exists());
    assert!(safe_workspace(&root.join("User data"), &install).is_ok());
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn manifest_missing_corrupt_or_changed_runner_fails_without_fallback() {
    let root = scratch();
    assert!(InstalledRelease::load(&root).is_err());
    let mut identity = build_identity();
    identity.profile = "release".into();
    identity.target = "x86_64-pc-windows-msvc".into();
    fs::write(root.join(APP_FILE), b"app").unwrap();
    fs::write(root.join(RUNNER_FILE), b"runner").unwrap();
    let manifest = ReleaseManifest {
        schema: RELEASE_SCHEMA.into(),
        version: "test".into(),
        platform: "windows-x86_64".into(),
        application: Binary {
            file: APP_FILE.into(),
            sha256: execution::hash(&root.join(APP_FILE)).unwrap(),
            build: identity.clone(),
        },
        runner: Binary {
            file: RUNNER_FILE.into(),
            sha256: execution::hash(&root.join(RUNNER_FILE)).unwrap(),
            build: identity,
        },
        modes: vec![
            woi_workbench::config::Mode::ContinuousWorld,
            woi_workbench::config::Mode::ActivityValuation,
        ],
        history_schemas: vec![
            "experiment-023-trial/v1".into(),
            "experiment-024-trial/v1".into(),
        ],
        runtime_requirements: vec![],
    };
    storage::write_new(&root.join("release.json"), &manifest).unwrap();
    let installed = InstalledRelease::load(&root).unwrap();
    assert!(
        installed
            .supports(woi_workbench::config::Mode::MaterialPreparation)
            .is_err()
    );
    fs::write(root.join(RUNNER_FILE), b"changed").unwrap();
    assert!(
        InstalledRelease::load(&root)
            .unwrap_err()
            .contains("checksum")
    );
    fs::remove_file(root.join(RUNNER_FILE)).unwrap();
    assert!(InstalledRelease::load(&root).is_err());
    let mut invalid = manifest;
    invalid.schema = "future/v2".into();
    assert!(invalid.validate().is_err());
    fs::remove_dir_all(root).unwrap();
}
#[test]
#[ignore = "independent read-only reconstruction from genuine retained developer run"]
fn every_recorded_moment_and_late_decision_matches_original() {
    let path = woi_workbench::repository()
        .join("target/workbench-runs/run-1792734727380-122968-0/history.jsonl");
    let full: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    let boundaries = full["history"]["boundaries"].as_array().unwrap();
    let rows = recorded::moments(&path).unwrap();
    assert_eq!(rows.len(), boundaries.len());
    for (row, b) in rows.iter().zip(boundaries) {
        assert_eq!(row.tick, b["tick"].as_u64().unwrap());
        for actor in 0..4 {
            let expected = b["choices"]
                .as_array()
                .unwrap()
                .iter()
                .find(|c| c["input"]["legacy"]["own"]["id"] == actor);
            assert_eq!(
                row.lanes[actor].as_deref(),
                expected.and_then(|c| c["selected"].as_str())
            );
            let transitions = full["history"]["transitions"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|t| {
                    t["tick"] == row.tick
                        && t["commitment"]["participants"]
                            .as_array()
                            .unwrap()
                            .contains(&serde_json::json!(actor))
                })
                .count();
            assert_eq!(row.executed[actor].len(), transitions);
            let core = &full["history"]["final_state"]["base"]["base"]["base"]["base"]["base"];
            let events = core["events"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|e| {
                    e["tick"] == row.tick
                        && e["participants"]
                            .as_array()
                            .unwrap()
                            .contains(&serde_json::json!(actor))
                })
                .count();
            let meals = core["cognition"]["consumption"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|m| m["tick"] == row.tick && m["actor"] == actor)
                .count();
            assert_eq!(row.consequences[actor].len(), events + meals);
        }
    }
    let index = boundaries
        .iter()
        .enumerate()
        .rev()
        .find(|(i, b)| *i > 32 && !b["choices"].as_array().unwrap().is_empty())
        .unwrap()
        .0;
    let c = &boundaries[index]["choices"][0];
    let actor = c["input"]["legacy"]["own"]["id"].as_u64().unwrap() as u32;
    let detail = recorded::detail(&path, index, actor).unwrap();
    let choice = detail.choice.unwrap();
    assert_eq!(choice.selected, c["selected"]);
    assert_eq!(
        choice.input.legacy.own.food,
        c["input"]["legacy"]["own"]["food"]
    );
    let expected = full["history"]["transitions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|t| {
            t["commitment"]["source"] == index
                && t["commitment"]["participants"]
                    .as_array()
                    .unwrap()
                    .contains(&serde_json::json!(actor))
        })
        .count();
    assert_eq!(detail.transitions.len(), expected);
    assert!(recorded::detail(&path, usize::MAX, actor).is_err());
}
