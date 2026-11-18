use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
static SEQUENCE: AtomicU64 = AtomicU64::new(0);
fn directory() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "eldoria-cli-{}-{}",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&path).unwrap();
    path
}
fn command(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_eldoria-settlement"))
        .args(args)
        .output()
        .unwrap()
}
fn success(args: &[&str]) -> Output {
    let output = command(args);
    assert!(
        output.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}
#[test]
fn separate_processes_resume_exactly_and_never_overwrite_saves() {
    let dir = directory();
    let paths: Vec<_> = [
        "initial.json",
        "paused.json",
        "resumed.json",
        "whole.json",
        "copy.json",
    ]
    .map(|name| dir.join(name).to_str().unwrap().to_string())
    .into();
    success(&["init", &paths[0], "--population", "12", "--horizon", "96"]);
    success(&["run", &paths[0], "--ticks", "13", "--out", &paths[1]]);
    success(&["resume", &paths[1], "--ticks", "83", "--out", &paths[2]]);
    success(&["run", &paths[0], "--ticks", "96", "--out", &paths[3]]);
    assert_eq!(fs::read(&paths[2]).unwrap(), fs::read(&paths[3]).unwrap());
    success(&["save", &paths[1], "--out", &paths[4]]);
    assert_eq!(fs::read(&paths[1]).unwrap(), fs::read(&paths[4]).unwrap());
    let before = fs::read(&paths[0]).unwrap();
    assert!(!command(&["init", &paths[0]]).status.success());
    assert_eq!(before, fs::read(&paths[0]).unwrap());
    let snapshot: serde_json::Value =
        serde_json::from_slice(&success(&["inspect", &paths[2]]).stdout).unwrap();
    assert_eq!(snapshot["characters"].as_array().unwrap().len(), 12);
    assert_eq!(snapshot["tick"], 96);
    let actor: serde_json::Value =
        serde_json::from_slice(&success(&["inspect", &paths[2], "--actor", "1001"]).stdout)
            .unwrap();
    assert!(actor.get("places").is_none());
    assert!(
        actor["people"]
            .as_array()
            .unwrap()
            .iter()
            .all(|p| p.get("food").is_none() && p.get("hunger").is_none())
    );
    let events: serde_json::Value = serde_json::from_slice(
        &success(&["events", &paths[2], "--from", "3", "--limit", "2"]).stdout,
    )
    .unwrap();
    assert_eq!(events.as_array().unwrap().len(), 2);
    assert_eq!(events[0]["id"], 3);
}
#[test]
fn cli_rejects_unsafe_paths_bad_options_and_incomplete_publications() {
    let dir = directory();
    let incomplete = dir.join("bad.json.pending");
    let initial = dir.join("initial.json");
    success(&["init", initial.to_str().unwrap()]);
    fs::copy(&initial, &incomplete).unwrap();
    assert!(
        !command(&["inspect", incomplete.to_str().unwrap()])
            .status
            .success()
    );
    assert!(
        !command(&[
            "init",
            dir.join("bad.json").to_str().unwrap(),
            "--unknown",
            "1"
        ])
        .status
        .success()
    );
    assert!(
        !command(&["init", dir.join("bad.json").to_str().unwrap(), "--seed"])
            .status
            .success()
    );
    let protected = dir.join("experiments");
    fs::create_dir(&protected).unwrap();
    assert!(
        !command(&["init", protected.join("bad.json").to_str().unwrap()])
            .status
            .success()
    );
    assert_eq!(fs::read_dir(protected).unwrap().count(), 0);
}
