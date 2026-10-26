use crate::{
    persistent::{
        World,
        session::{Session, reliability},
    },
    preparation as m,
};
fn scratch(label: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "woi-material-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&path).unwrap();
    path
}
#[test]
fn all_registered_worlds_preserve_original_and_split_state() {
    let mut splits = 0;
    let mut phases = std::collections::BTreeSet::new();
    for (index, config) in m::matrix().into_iter().enumerate() {
        let mut w = World::new(config.clone()).unwrap();
        let mut steps = 0;
        while !w.complete() {
            w.step().unwrap();
            steps += 1;
            for a in &w.material().active {
                phases.insert(format!("{:?}", a.phase));
            }
            if steps % 11 == 0 || w.complete() {
                let bytes = w
                    .checkpoint()
                    .unwrap_or_else(|e| panic!("world{index}/step{steps}: {e}"));
                let restored = World::restore(&bytes)
                    .unwrap_or_else(|e| panic!("world{index}/step{steps}: {e}"));
                assert_eq!(
                    restored.checkpoint().unwrap(),
                    bytes,
                    "full saved state world{index}/step{steps}"
                );
                w = restored;
                splits += 1;
            }
        }
        assert_eq!(
            w.trial().unwrap(),
            m::trial(config),
            "original corrected025 world{index}"
        );
    }
    assert!(splits > 100);
    assert!(
        phases.contains("Extracting") && phases.contains("Preparing") && phases.contains("Failed")
    );
    println!("50 original worlds, {splits} full-state splits, phases {phases:?}");
}
#[test]
fn shared_transport_raw_compressed_and_recovery() {
    let root = scratch("transport");
    let parent = root.join("world-a");
    let config = m::matrix()[1].clone();
    let mut s = Session::create(&parent, config.clone()).unwrap();
    assert!(Session::open(&parent).is_err());
    assert!(reliability::verify_independent(&parent, 1).is_err());
    s.advance(3).unwrap();
    s.save().unwrap();
    let selected = s.world().checkpoint().unwrap();
    s.advance(4).unwrap();
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
    let mut restored = Session::open(&branch).unwrap();
    assert_eq!(restored.world().checkpoint().unwrap(), selected);
    restored.advance(2048).unwrap();
    restored.save().unwrap();
    assert_eq!(restored.world().trial().unwrap(), m::trial(config));
    drop(restored);
    reliability::verify_independent(&branch, 3).unwrap();
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn explicitly_disabled_path_keeps_original024_execution() {
    let mut config = m::matrix()[0].clone();
    config.preparation = m::Settings::default();
    let mut w = World::new(config.clone()).unwrap();
    w.advance(5).unwrap();
    let mut w = World::restore(&w.checkpoint().unwrap()).unwrap();
    w.advance(2048).unwrap();
    assert_eq!(w.trial().unwrap(), m::trial(config));
}
