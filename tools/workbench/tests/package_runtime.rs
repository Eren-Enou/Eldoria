//! Explicit package acceptance; all spawned applications have no developer tools
//! on PATH. The checkout used by this test host still exists.
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use woi_workbench::{execution, storage};
fn call(app: &Path, data: &Path, args: &[&str]) -> Output {
    let mut c = Command::new(app);
    c.arg("--workspace")
        .arg(data)
        .args(args)
        .current_dir(app.parent().unwrap())
        .env_clear();
    for name in ["SYSTEMROOT", "WINDIR", "TEMP", "TMP"] {
        if let Some(v) = std::env::var_os(name) {
            c.env(name, v);
        }
    }
    c.env(
        "PATH",
        PathBuf::from(std::env::var_os("SYSTEMROOT").unwrap()).join("System32"),
    );
    c.output().unwrap()
}
fn json(out: Output) -> serde_json::Value {
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).unwrap()
}
#[test]
#[ignore = "set WOI_PORTABLE_PACKAGE to a complete extracted release"]
fn tool_free_installed_run_replay_notes_relocation_and_failures() {
    let package =
        PathBuf::from(std::env::var_os("WOI_PORTABLE_PACKAGE").expect("WOI_PORTABLE_PACKAGE"));
    let root = std::env::temp_dir().join(format!(
        "WOI Installed Acceptance {} {}",
        std::process::id(),
        storage::now()
    ));
    fs::create_dir(&root).unwrap();
    let mut first = None;
    for location in ["First extracted app", "Relocated Application With Spaces"] {
        let install = root.join(location);
        fs::create_dir(&install).unwrap();
        for name in [
            "woi-workbench.exe",
            "woi-portable-runner.exe",
            "release.json",
        ] {
            fs::copy(package.join(name), install.join(name)).unwrap();
        }
        let app = install.join("woi-workbench.exe");
        let data = root.join(format!("User data {location}"));
        let manifest_before = execution::hash(&install.join("release.json")).unwrap();
        let run = json(call(&app, &data, &["run"]));
        assert_eq!(run["status"], "Completed");
        assert_eq!(run["schema"], "woi-workbench-run/v2");
        let id = run["id"].as_str().unwrap();
        let hash = run["history_sha256"].as_str().unwrap().to_owned();
        if let Some(old) = &first {
            assert_eq!(old, &hash);
        } else {
            first = Some(hash.clone());
        }
        let summary = json(call(&app, &data, &["inspect", id]));
        assert_eq!(summary["food_accounting_matches"], true);
        assert_eq!(summary["final_agents"].as_array().unwrap().len(), 4);
        let reproduction = json(call(&app, &data, &["reproduce", id]));
        assert_eq!(reproduction["exact_match"], true);
        assert_eq!(reproduction["history_sha256"], hash);
        let note = root.join(format!("note {location}.json"));
        storage::write_new(
            &note,
            &woi_workbench::observations::Observation {
                individual: 0,
                tick: 6,
                notes: "Portable acceptance: external human interpretation".into(),
                ..Default::default()
            },
        )
        .unwrap();
        let out = call(&app, &data, &["observe", id, note.to_str().unwrap()]);
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let recovered = json(call(&app, &data, &["notes", id]));
        assert_eq!(
            recovered[0]["observation"]["notes"],
            "Portable acceptance: external human interpretation"
        );
        assert_eq!(
            execution::hash(&data.join("runs").join(id).join("history.jsonl")).unwrap(),
            hash
        );
        assert_eq!(
            execution::hash(&install.join("release.json")).unwrap(),
            manifest_before
        );
        assert_eq!(
            fs::read_dir(&install).unwrap().count(),
            3,
            "Normal operation wrote inside installation"
        );
        assert!(
            !call(&app, &install.join("forbidden data"), &["run"])
                .status
                .success()
        );
        assert!(!install.join("forbidden data").exists());
        let bad = root.join(format!("bad {location}.json"));
        fs::write(&bad, r#"{"schema_version":999}"#).unwrap();
        assert!(
            !call(&app, &data, &["run", bad.to_str().unwrap()])
                .status
                .success()
        );
        let runner = install.join("woi-portable-runner.exe");
        fs::write(&runner, b"corrupt runner").unwrap();
        let out = call(&app, &data, &["run"]);
        assert!(!out.status.success());
        assert!(String::from_utf8_lossy(&out.stderr).contains("checksum"));
        fs::remove_file(&runner).unwrap();
        assert!(!call(&app, &data, &["list"]).status.success());
        fs::remove_file(install.join("release.json")).unwrap();
        let out = call(&app, &data, &["list"]);
        assert!(!out.status.success());
        assert!(!String::from_utf8_lossy(&out.stderr).contains("Git is required"));
    }
    println!("Acceptance files retained at {}", root.display());
}
