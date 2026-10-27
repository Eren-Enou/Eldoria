pub(crate) fn require_all100_archived_parity() {
    static ONCE: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    ONCE.get_or_init(|| {
        use std::io::BufRead;
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../experiments/026/continuation-c1/trials.jsonl.gz"
        );
        let stream = std::io::BufReader::new(flate2::read::GzDecoder::new(
            std::fs::File::open(path).unwrap(),
        ));
        let mut count = 0;
        for (index, line) in stream.lines().enumerate() {
            let expected: continuation_c1::Trial = serde_json::from_str(&line.unwrap()).unwrap();
            assert_eq!(expected.trial.config, delayed::matrix()[index]);
            let mut world = World::new(expected.trial.config.clone()).unwrap();
            world.advance(2048).unwrap();
            assert_eq!(
                world.trial().unwrap(),
                expected,
                "archived uninterrupted parity row{index}"
            );
            count += 1;
        }
        assert_eq!(count, 100);
        println!("Gate D: exact100 original archived C1 histories before any new persistence test");
    });
}
use crate::{
    continuation_c1, delayed,
    persistent::{
        World,
        session::{Session, reliability},
    },
};
fn scratch(label: &str) -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!(
        "woi-C1-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&p).unwrap();
    p
}
#[test]
fn all100_original_histories_and_complete_state_splits() {
    require_all100_archived_parity();
    let mut splits = 0;
    for config in delayed::matrix() {
        let mut world = World::new(config.clone()).unwrap();
        let mut steps = 0;
        while !world.complete() {
            world.step().unwrap();
            steps += 1;
            if steps % 29 == 0 || world.complete() {
                let bytes = world.checkpoint().unwrap();
                let restored = World::restore(&bytes).unwrap();
                assert_eq!(restored.checkpoint().unwrap(), bytes);
                world = restored;
                splits += 1;
            }
        }
        assert_eq!(world.trial().unwrap(), continuation_c1::trial(config));
    }
    assert!(splits > 500);
    println!("100 original C1 worlds; {splits} exact complete-state splits");
}
#[test]
fn raw_zlib_recovery_excludes_committed_and_unverified_history() {
    require_all100_archived_parity();
    let root = scratch("recovery");
    let parent = root.join("world-a");
    let cfg = delayed::matrix()[47].clone();
    let mut s = Session::create(&parent, cfg.clone()).unwrap();
    assert!(Session::open(&parent).is_err());
    assert!(reliability::verify_independent(&parent, 1).is_err());
    s.advance(10).unwrap();
    s.save().unwrap();
    let selected = s.world().checkpoint().unwrap();
    s.advance(3).unwrap();
    s.save_with_codec(crate::storage_codec::Codec::Zlib)
        .unwrap();
    s.advance(2).unwrap();
    drop(s);
    assert!(Session::open(&parent).is_err());
    let before = std::fs::read(parent.join("journal.jsonl")).unwrap();
    let branch_root = root.join("branch");
    std::fs::create_dir(&branch_root).unwrap();
    let branch = branch_root.join("world-a");
    reliability::recover(&parent, &branch, 2).unwrap();
    assert_eq!(before, std::fs::read(parent.join("journal.jsonl")).unwrap());
    let mut s = Session::open(&branch).unwrap();
    assert_eq!(s.world().checkpoint().unwrap(), selected);
    s.advance(2048).unwrap();
    s.save().unwrap();
    assert_eq!(s.world().trial().unwrap(), continuation_c1::trial(cfg));
    drop(s);
    reliability::verify_independent(&branch, 3).unwrap();
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn unknown_or_implicit_configuration_never_creates_a_world() {
    require_all100_archived_parity();
    let mut c = delayed::matrix()[3].clone();
    c.world.world.horizon += 1;
    assert!(World::new(c).is_err());
    let mut value = serde_json::to_value(delayed::matrix()[3].clone()).unwrap();
    value.as_object_mut().unwrap().remove("mode");
    assert!(serde_json::from_value::<delayed::Config>(value).is_err());
}
