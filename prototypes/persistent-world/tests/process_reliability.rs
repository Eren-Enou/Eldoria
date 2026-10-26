use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
#[cfg(feature = "failure-probes")]
use std::{
    process::Stdio,
    time::{Duration, Instant},
};
use woi_persistent_world::{
    continuous as c,
    persistent::{
        World,
        session::{Session, reliability},
    },
    valuation as v,
};
fn root(label: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/recovery-validation")
        .join(format!(
            "{label}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&root).unwrap();
    root.canonicalize().unwrap()
}
fn config(case: &str, horizon: u64, seed: u64) -> c::Config {
    let mut config = c::matrix()
        .into_iter()
        .find(|c| c.case == case && c.mode == c::Mode::Continuous)
        .unwrap();
    config.horizon = horizon;
    config.seed = seed;
    config
}
fn call(path: &Path, steps: usize, compressed: bool) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_woi-persistent-world"));
    command.arg("advance").arg(path).arg(steps.to_string());
    if compressed {
        command.args(["--codec", "zlib"]);
    }
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
#[test]
fn many_real_process_sessions_match_full_state_and_original_histories() {
    let root = root("many-sessions");
    let mut receipts = vec![];
    for (case_index, case) in ["paired", "scarcity", "inquiry_conflict"]
        .into_iter()
        .enumerate()
    {
        for (index, mode) in [None, Some(v::Mode::Baseline), Some(v::Mode::MealRelief)]
            .into_iter()
            .enumerate()
        {
            let config = config(case, 1024, [0, 42, u64::MAX][index]);
            let registered = mode.map(|valuation| v::Config {
                world: config.clone(),
                valuation,
                family: "headless_reliability".into(),
                source023: None,
            });
            let path = root.join(format!("case-{case_index}-{index}"));
            let (session, mut reference) = if let Some(r) = &registered {
                (
                    Session::create_valuation(&path, r.clone()).unwrap(),
                    World::new_valuation(r.clone()).unwrap(),
                )
            } else {
                (
                    Session::create(&path, config.clone()).unwrap(),
                    World::new(config.clone()).unwrap(),
                )
            };
            drop(session);
            let nominal = reference.start() + config.horizon;
            let initial = reference.agents();
            let mut drain_split = false;
            let mut cutoff_crossed = false;
            let mut hunger_changed = false;
            let mut trust_changed = false;
            let mut memory_full = false;
            let mut sessions = 0;
            let mut pending = false;
            let mut active = false;
            for steps in [
                1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
                64, 256, 1, 8, 32, 2048,
            ] {
                if reference.complete() {
                    break;
                }
                // Stop close to the real cutoff, then save during the charged drain.
                // Atomic inquiries may legitimately cross a tick; never split them.
                let remaining = nominal.saturating_sub(reference.tick()) as usize;
                let steps = if remaining > 0 {
                    steps.min(remaining)
                } else {
                    steps
                };
                let before_tick = reference.tick();
                call(&path, steps, sessions % 4 != 0);
                reference.advance(steps).unwrap();
                sessions += 1;
                let reopened = Session::open(&path).unwrap();
                assert_eq!(
                    reopened.world().checkpoint().unwrap(),
                    reference.checkpoint().unwrap(),
                    "{case} {mode:?} session{sessions}"
                );
                cutoff_crossed |= before_tick < nominal && reference.tick() >= nominal;
                drain_split |= reference.tick() >= nominal && !reference.complete();
                for actor in reference.agents() {
                    let old = initial.iter().find(|a| a.id == actor.id).unwrap();
                    hunger_changed |= actor.hunger != old.hunger;
                    trust_changed |= actor.trust != old.trust;
                    memory_full |= actor.memories.len() == 16;
                }
                pending |= !reopened.world().pending().is_empty();
                active |= !reopened.world().commitments().is_empty();
                drop(reopened);
                if sessions % 4 == 0 {
                    reliability::verify_independent(&path, sessions + 1).unwrap();
                }
            }
            assert!(reference.complete());
            assert!(
                cutoff_crossed && drain_split,
                "missing cutoff/drain split {case} {mode:?}"
            );
            assert!(sessions >= 12, "must exercise many processes");
            let mut reopened = Session::open(&path).unwrap();
            reopened.finalize().unwrap();
            let mut expected = reference.export().unwrap();
            expected.push(b'\n');
            assert_eq!(fs::read(path.join("history.jsonl")).unwrap(), expected);
            if let Some(r) = registered {
                assert_eq!(reopened.world().valuation_trial().unwrap(), v::trial(r));
            } else {
                assert_eq!(reopened.world().trial().unwrap(), c::trial(config));
            }
            receipts.push(serde_json::json!({"case":case,"mode":format!("{mode:?}"),"sessions":sessions,"cutoff_crossed":cutoff_crossed,"charged_drain_split":drain_split,"hunger_changed_at_split":hunger_changed,"trust_changed_at_split":trust_changed,"memory_capacity_reached_at_split":memory_full,"pending_split":pending,"active_split":active,"boundaries":reference.boundaries().len(),"tick":reference.tick()}));
        }
    }
    fs::write(
        root.join("coverage.json"),
        serde_json::to_vec_pretty(&receipts).unwrap(),
    )
    .unwrap();
}

#[cfg(feature = "failure-probes")]
#[test]
fn killed_children_release_lock_and_preserve_committed_recovery() {
    for stage in ["journal", "before-publish", "after-publish"] {
        let root = root(stage);
        let path = root.join("world");
        let config = config("paired", 64, 43);
        let mut s = Session::create(&path, config.clone()).unwrap();
        s.advance(3).unwrap();
        s.save().unwrap();
        let saved = s.world().checkpoint().unwrap();
        drop(s);
        let ready = root.join("ready");
        let mut child = Command::new(env!("CARGO_BIN_EXE_woi-persistent-world"))
            .args(["probe", stage])
            .arg(&path)
            .arg("5")
            .arg(&ready)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let started = Instant::now();
        while !ready.exists() {
            if let Some(status) = child.try_wait().unwrap() {
                panic!("probe exited early {stage}: {status}");
            }
            if started.elapsed() > Duration::from_secs(15) {
                let _ = child.kill();
                let _ = child.wait();
                panic!("probe readiness timeout {stage}");
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(
            !reliability::inspect(&path, &Default::default())
                .unwrap()
                .stable_snapshot
        );
        let concurrent = Command::new(env!("CARGO_BIN_EXE_woi-persistent-world"))
            .arg("advance")
            .arg(&path)
            .arg("1")
            .output()
            .unwrap();
        assert!(!concurrent.status.success());
        assert!(String::from_utf8_lossy(&concurrent.stderr).contains("already open"));
        child.kill().unwrap();
        child.wait().unwrap();
        let report = reliability::inspect(&path, &Default::default()).unwrap();
        assert!(report.stable_snapshot);
        if stage == "after-publish" {
            assert_eq!(report.latest_valid, Some(3));
            let mut restored = Session::open(&path).unwrap();
            restored.advance(2048).unwrap();
            assert_eq!(restored.world().trial().unwrap(), c::trial(config));
        } else {
            assert_eq!(report.latest_valid, Some(2));
            assert!(Session::open(&path).is_err());
            if stage == "before-publish" {
                assert_eq!(report.pending.len(), 1);
            }
            let base = root.join("branch");
            fs::create_dir(&base).unwrap();
            let branch = base.join("world");
            reliability::recover(&path, &branch, 2).unwrap();
            let mut restored = Session::open(&branch).unwrap();
            assert_eq!(restored.world().checkpoint().unwrap(), saved);
            restored.advance(2048).unwrap();
            restored.finalize().unwrap();
            assert_eq!(restored.world().trial().unwrap(), c::trial(config));
        }
    }
}
