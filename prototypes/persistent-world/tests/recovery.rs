use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use woi_persistent_world::{
    continuous as c,
    persistent::{
        World,
        session::{
            Session,
            reliability::{self, Budget},
        },
    },
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
fn config() -> c::Config {
    let mut c = c::matrix()
        .into_iter()
        .find(|c| c.case == "paired" && c.mode == c::Mode::Continuous)
        .unwrap();
    c.horizon = 32;
    c
}
fn snapshot(path: &Path) -> BTreeMap<String, Vec<u8>> {
    fs::read_dir(path)
        .unwrap()
        .filter(|e| e.as_ref().unwrap().file_name() != "session.lock")
        .map(|e| {
            let e = e.unwrap();
            (
                e.file_name().to_string_lossy().to_string(),
                fs::read(e.path()).unwrap(),
            )
        })
        .collect()
}
#[test]
fn inspection_is_read_only_even_when_busy_or_malformed() {
    let root = root("readonly");
    let path = root.join("world");
    let session = Session::create(&path, config()).unwrap();
    let before = snapshot(&path);
    let report = reliability::inspect(&path, &Budget::default()).unwrap();
    assert!(!report.stable_snapshot);
    assert_eq!(snapshot(&path), before);
    drop(session);
    let report = reliability::inspect(
        &path,
        &Budget {
            bytes: Some(1),
            generations: Some(0),
            projected_saves: 10,
        },
    )
    .unwrap();
    assert_eq!(report.latest_valid, Some(1));
    assert_eq!(report.oldest_valid, Some(1));
    assert_eq!(
        report.total_retained_bytes,
        before.values().map(|v| v.len() as u64).sum::<u64>()
    );
    assert!(report.warnings.len() >= 2);
    assert_eq!(snapshot(&path), before);
    assert!(
        reliability::recover(&path, &path.join("world"), 1)
            .unwrap_err()
            .contains("outside")
    );
    assert_eq!(snapshot(&path), before);
    let malformed = root.join("malformed");
    fs::create_dir(&malformed).unwrap();
    reliability::inspect(&malformed, &Budget::default()).unwrap();
    assert_eq!(fs::read_dir(malformed).unwrap().count(), 0);
}
#[test]
fn explicit_recovery_preserves_tail_parent_and_complete_state() {
    let root = root("branch");
    let parent = root.join("world");
    let mut session = Session::create(&parent, config()).unwrap();
    session.advance(3).unwrap();
    session.save().unwrap();
    let selected = session.world().checkpoint().unwrap();
    let tick = session.world().tick();
    let prefix = fs::read(parent.join("journal.jsonl")).unwrap();
    session.advance(5).unwrap();
    drop(session);
    let before = snapshot(&parent);
    assert!(Session::open(&parent).is_err());
    let report = reliability::inspect(&parent, &Budget::default()).unwrap();
    assert_eq!(report.latest_valid, Some(2));
    let branch_root = root.join("recovered");
    fs::create_dir(&branch_root).unwrap();
    let branch = branch_root.join("world");
    let receipt = reliability::recover(&parent, &branch, 2).unwrap();
    assert_eq!(receipt.tick, tick);
    assert!(receipt.tail_bytes > 0);
    assert_eq!(snapshot(&parent), before);
    assert_eq!(fs::read(branch.join("journal.jsonl")).unwrap(), prefix);
    assert_eq!(
        fs::read(branch.join("uncommitted-tail.bin")).unwrap(),
        before["journal.jsonl"][prefix.len()..]
    );
    let mut restored = Session::open(&branch).unwrap();
    assert_eq!(restored.world().checkpoint().unwrap(), selected);
    restored.advance(2048).unwrap();
    assert_eq!(restored.world().trial().unwrap(), c::trial(config()));
    restored.finalize().unwrap();
    assert_eq!(snapshot(&parent), before);
    assert!(
        reliability::recover(&parent, &branch, 2)
            .unwrap_err()
            .contains("already exists")
    );
}
#[test]
fn corrupt_latest_requires_explicit_older_selection_and_truncated_tail_is_preserved() {
    let root = root("corrupt");
    let path = root.join("world");
    let mut s = Session::create(&path, config()).unwrap();
    let expected = s.world().checkpoint().unwrap();
    s.advance(2).unwrap();
    s.save().unwrap();
    drop(s);
    fs::write(path.join("checkpoint-00000002.cbor"), b"corrupt").unwrap();
    fs::OpenOptions::new()
        .append(true)
        .open(path.join("journal.jsonl"))
        .unwrap();
    use std::io::Write;
    let mut f = fs::OpenOptions::new()
        .append(true)
        .open(path.join("journal.jsonl"))
        .unwrap();
    f.write_all(b"{truncated").unwrap();
    drop(f);
    let before = snapshot(&path);
    let report = reliability::inspect(&path, &Budget::default()).unwrap();
    assert_eq!(report.latest_committed, Some(2));
    assert_eq!(report.latest_valid, Some(1));
    assert!(Session::open(&path).is_err());
    let base = root.join("branch");
    fs::create_dir(&base).unwrap();
    let dest = base.join("world");
    assert!(reliability::recover(&path, &dest, 2).is_err());
    assert!(!dest.exists());
    reliability::recover(&path, &dest, 1).unwrap();
    assert_eq!(
        Session::open(&dest).unwrap().world().checkpoint().unwrap(),
        expected
    );
    assert_eq!(snapshot(&path), before);
    let journal = path.join("journal.jsonl");
    let mut bytes = fs::read(&journal).unwrap();
    bytes[1] ^= 1;
    fs::write(journal, bytes).unwrap();
    let bad = root.join("bad");
    fs::create_dir(&bad).unwrap();
    assert!(reliability::recover(&path, &bad.join("world"), 1).is_err());
}
#[test]
fn original_v1_bytes_remain_readable_with_guarded_compatibility() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/v1");
    let meta: serde_json::Value =
        serde_json::from_slice(&fs::read(fixture.join("fixture.json")).unwrap()).unwrap();
    let id = meta["world"].as_str().unwrap();
    let runtime = meta["runtime"].as_str().unwrap();
    if !woi_persistent_world::persistent::session::accepts_runtime(runtime) {
        // Other compiler/target/dependency sets intentionally cannot claim compatibility.
        return;
    }
    let path = root("v1").join(id);
    fs::create_dir(&path).unwrap();
    for name in ["checkpoint-00000001.cbor", "journal.jsonl"] {
        fs::copy(fixture.join(id).join(name), path.join(name)).unwrap();
    }
    fs::write(path.join("session.lock"), b"").unwrap();
    let mut session = Session::open(&path).unwrap();
    let mut reference = World::new(session.world().configuration().clone()).unwrap();
    assert_eq!(
        session.world().checkpoint().unwrap(),
        reference.checkpoint().unwrap()
    );
    session.advance(5).unwrap();
    session.save().unwrap();
    drop(session);
    let mut resumed = Session::open(&path).unwrap();
    resumed.advance(2048).unwrap();
    reference.advance(2048).unwrap();
    assert_eq!(
        resumed.world().checkpoint().unwrap(),
        reference.checkpoint().unwrap()
    );
    assert_eq!(
        resumed.world().export().unwrap(),
        reference.export().unwrap()
    );
}
