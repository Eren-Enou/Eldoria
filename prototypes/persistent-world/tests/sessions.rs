use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
use woi_persistent_world::{continuous as c, persistent::session::Session};
fn scratch(label: &str) -> PathBuf {
    let root =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/persistent-world-validation");
    fs::create_dir_all(&root).unwrap();
    root.canonicalize().unwrap().join(format!(
        "{label}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}
fn config() -> c::Config {
    let mut config = c::matrix()
        .into_iter()
        .find(|c| c.case == "paired" && c.mode == c::Mode::Continuous)
        .unwrap();
    config.horizon = 32;
    config
}
#[test]
fn actual_process_restart_matches_original() {
    let directory = scratch("restart");
    let config = config();
    let mut session = Session::create(&directory, config.clone()).unwrap();
    session.advance(3).unwrap();
    session.save().unwrap();
    let tick = session.world().tick();
    drop(session);
    let executable = env!("CARGO_BIN_EXE_woi-persistent-world");
    let output = Command::new(executable)
        .arg("advance")
        .arg(&directory)
        .arg("2048")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut resumed = Session::open(&directory).unwrap();
    assert!(resumed.world().tick() > tick);
    assert_eq!(resumed.world().trial().unwrap(), c::trial(config));
    resumed.finalize().unwrap();
    assert!(directory.join("checkpoint-00000001.cbor").is_file());
    assert!(directory.join("history.jsonl").is_file());
}
#[test]
fn partial_and_mismatched_files_are_not_saves() {
    let directory = scratch("failure");
    let session = Session::create(&directory, config()).unwrap();
    let saved = session.world().checkpoint().unwrap();
    fs::write(directory.join("checkpoint-00000002.pending"), b"incomplete").unwrap();
    drop(session);
    assert_eq!(
        Session::open(&directory)
            .unwrap()
            .world()
            .checkpoint()
            .unwrap(),
        saved
    );
    let mut session = Session::open(&directory).unwrap();
    session.advance(1).unwrap();
    let advanced = session.world().checkpoint().unwrap();
    session.save().unwrap();
    assert!(directory.join("checkpoint-00000003.cbor").exists());
    drop(session);
    assert_eq!(
        Session::open(&directory)
            .unwrap()
            .world()
            .checkpoint()
            .unwrap(),
        advanced
    );
    let mut session = Session::open(&directory).unwrap();
    session.advance(1).unwrap();
    drop(session);
    assert!(
        Session::open(&directory)
            .err()
            .unwrap()
            .contains("unsaved tail")
    );
    // The incomplete writer and both valid generations remain diagnostic evidence.
    assert!(directory.join("checkpoint-00000001.cbor").exists());
    assert!(directory.join("checkpoint-00000002.pending").exists());
    fs::write(directory.join("journal.jsonl"), b"corrupt").unwrap();
    assert!(Session::open(&directory).is_err());
}

#[test]
fn a_second_process_cannot_advance_an_open_world() {
    let directory = scratch("exclusive");
    let session = Session::create(&directory, config()).unwrap();
    let state = session.world().checkpoint().unwrap();
    let journal = fs::read(directory.join("journal.jsonl")).unwrap();
    assert!(
        Session::open(&directory)
            .err()
            .unwrap()
            .contains("already open")
    );
    let output = Command::new(env!("CARGO_BIN_EXE_woi-persistent-world"))
        .arg("advance")
        .arg(&directory)
        .arg("1")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("already open"));
    assert_eq!(fs::read(directory.join("journal.jsonl")).unwrap(), journal);
    assert_eq!(session.world().checkpoint().unwrap(), state);
    drop(session);
    assert_eq!(
        Session::open(&directory)
            .unwrap()
            .world()
            .checkpoint()
            .unwrap(),
        state
    );
}

#[test]
fn long_world_crosses_multiple_real_process_sessions() {
    let directory = scratch("long-restart");
    let mut config = config();
    config.horizon = 1024;
    config.seed = 42;
    let session = Session::create(&directory, config.clone()).unwrap();
    let start = session.world().tick();
    drop(session);
    let executable = env!("CARGO_BIN_EXE_woi-persistent-world");
    let mut previous = start;
    for steps in [100, 200, 400, 2048] {
        let output = Command::new(executable)
            .arg("advance")
            .arg(&directory)
            .arg(steps.to_string())
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let reopened = Session::open(&directory).unwrap();
        assert!(reopened.world().tick() > previous);
        previous = reopened.world().tick();
        let tick = reopened.world().tick();
        drop(reopened);
        assert_eq!(Session::open(&directory).unwrap().world().tick(), tick);
    }
    let mut resumed = Session::open(&directory).unwrap();
    let original = c::trial(config);
    assert_eq!(original, resumed.world().trial().unwrap());
    resumed.finalize().unwrap();
    let mut expected = serde_json::to_vec(&original).unwrap();
    expected.push(b'\n');
    assert_eq!(fs::read(directory.join("history.jsonl")).unwrap(), expected);
}
