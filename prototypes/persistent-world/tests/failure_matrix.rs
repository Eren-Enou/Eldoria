use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use woi_persistent_world::{
    continuous as c,
    persistent::session::{Session, reliability},
};
#[derive(Serialize, Deserialize)]
struct Commit {
    binding_sha256: String,
    schema: String,
    world: String,
    runtime: String,
    generation: u64,
    tick: u64,
    journal_bytes: u64,
    journal_sha256: String,
    state_sha256: String,
    #[serde(with = "serde_bytes")]
    state: Vec<u8>,
}
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn rewrite(path: &Path, change: impl FnOnce(&mut Commit)) {
    let raw = fs::read(path).unwrap();
    let mut c: Commit = ciborium::from_reader(raw.as_slice()).unwrap();
    change(&mut c);
    c.binding_sha256 = digest(
        &serde_json::to_vec(&(
            &c.schema,
            &c.world,
            &c.runtime,
            c.generation,
            c.tick,
            c.journal_bytes,
            &c.journal_sha256,
            &c.state_sha256,
        ))
        .unwrap(),
    );
    let mut out = vec![];
    ciborium::into_writer(&c, &mut out).unwrap();
    fs::write(path, out).unwrap();
}
fn fixture(label: &str) -> (PathBuf, c::Config) {
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
    let path = root.canonicalize().unwrap().join("world");
    let mut cfg = c::matrix()
        .into_iter()
        .find(|c| c.case == "paired" && c.mode == c::Mode::Continuous)
        .unwrap();
    cfg.horizon = 32;
    let mut s = Session::create(&path, cfg.clone()).unwrap();
    s.advance(3).unwrap();
    s.save().unwrap();
    drop(s);
    (path, cfg)
}
#[test]
fn missing_corrupt_identity_schema_and_ambiguous_files_never_fall_back() {
    for kind in [
        "missing",
        "corrupt",
        "runtime",
        "schema",
        "world",
        "ambiguous",
    ] {
        let (path, _) = fixture(kind);
        let checkpoint = path.join("checkpoint-00000002.cbor");
        let before = fs::read(path.join("checkpoint-00000001.cbor")).unwrap();
        match kind {
            "missing" => {
                fs::remove_file(&checkpoint).unwrap();
                fs::remove_file(path.join("checkpoint-00000001.cbor")).unwrap();
            }
            "corrupt" => fs::write(&checkpoint, b"broken").unwrap(),
            "runtime" => rewrite(&checkpoint, |c| c.runtime = "0".repeat(64)),
            "schema" => rewrite(&checkpoint, |c| c.schema = "woi-world-commit/v99".into()),
            "world" => rewrite(&checkpoint, |c| c.world = "other-world".into()),
            _ => fs::write(path.join("checkpoint-0000000002.cbor"), b"ambiguous").unwrap(),
        }
        assert!(Session::open(&path).is_err(), "{kind}");
        let report = reliability::inspect(&path, &Default::default()).unwrap();
        if kind != "missing" {
            assert_eq!(
                fs::read(path.join("checkpoint-00000001.cbor")).unwrap(),
                before
            );
        }
        if kind != "ambiguous" {
            assert!(report.latest_valid.is_none_or(|n| n == 1));
        }
        let dest = path.parent().unwrap().join("refused");
        fs::create_dir(&dest).unwrap();
        assert!(
            reliability::recover(&path, &dest.join("world"), 2).is_err() || kind == "ambiguous"
        );
    }
}
#[test]
fn generation_maximum_is_numeric_and_exhaustion_preserves_earlier_commits() {
    let (path, cfg) = fixture("generation-max");
    let file = path.join("checkpoint-00000002.cbor");
    rewrite(&file, |c| c.generation = u64::MAX);
    fs::rename(&file, path.join(format!("checkpoint-{}.cbor", u64::MAX))).unwrap();
    let before = fs::read(path.join("checkpoint-00000001.cbor")).unwrap();
    let mut s = Session::open(&path).unwrap();
    s.advance(1).unwrap();
    assert!(s.save().unwrap_err().contains("exhausted"));
    drop(s);
    assert_eq!(
        fs::read(path.join("checkpoint-00000001.cbor")).unwrap(),
        before
    );
    let report = reliability::inspect(&path, &Default::default()).unwrap();
    assert_eq!(report.latest_valid, Some(u64::MAX));
    let base = path.parent().unwrap().join("branch");
    fs::create_dir(&base).unwrap();
    let branch = base.join("world");
    reliability::recover(&path, &branch, 1).unwrap();
    let mut s = Session::open(&branch).unwrap();
    s.advance(2048).unwrap();
    s.finalize().unwrap();
    assert_eq!(s.world().trial().unwrap(), c::trial(cfg));
}
#[cfg(feature = "failure-probes")]
#[test]
fn failed_publication_hook_retries_without_reexecution_or_overwriting() {
    let (path, _) = fixture("retry");
    let mut s = Session::open(&path).unwrap();
    s.advance(4).unwrap();
    let expected = s.world().checkpoint().unwrap();
    let journal = fs::read(path.join("journal.jsonl")).unwrap();
    assert!(
        s.save_before_publish_probe(|| Err("intentional prepublication failure".into()))
            .is_err()
    );
    let pending = fs::read(path.join("checkpoint-00000003.pending")).unwrap();
    s.save().unwrap();
    assert!(path.join("checkpoint-00000004.cbor").exists());
    assert_eq!(
        fs::read(path.join("checkpoint-00000003.pending")).unwrap(),
        pending
    );
    assert_eq!(fs::read(path.join("journal.jsonl")).unwrap(), journal);
    drop(s);
    assert_eq!(
        Session::open(&path).unwrap().world().checkpoint().unwrap(),
        expected
    );
}

#[test]
fn inspection_rejects_filesystem_root_without_traversal() {
    let root = if cfg!(windows) {
        Path::new("C:/")
    } else {
        Path::new("/")
    };
    let error = reliability::inspect(root, &Default::default())
        .err()
        .unwrap();
    assert!(error.contains("world directory"), "{error}");
}

#[cfg(windows)]
#[test]
fn normalized_archive_alias_is_rejected_before_any_session_write() {
    use woi_persistent_world::persistent::session::check_path;
    let (world, _) = fixture("normalized-path");
    // Ordinary DOS paths normalize the alias; verbatim paths deliberately do not.
    let canonical = world.parent().unwrap().to_string_lossy();
    let root = PathBuf::from(canonical.strip_prefix(r"\\?\").unwrap_or(&canonical));
    for name in ["experiments", "reinforcement", ".git"] {
        let protected = root.join(name);
        fs::create_dir(&protected).unwrap();
        let alias = root.join(format!("{name}.")).join("new-world");
        let error = check_path(&alias).unwrap_err();
        assert!(error.contains("resolved session path"), "{error}");
        assert!(Session::create(&alias, c::matrix().remove(0)).is_err());
        assert!(!protected.join("new-world").exists());
        assert!(fs::read_dir(&protected).unwrap().next().is_none());
    }
}
