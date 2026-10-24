use woi_persistent_world::{continuous as c, persistent::World};

#[test]
fn original_and_segmented_complete_history() {
    let mut count = 0;
    for case in [
        "null",
        "paired",
        "acquisition",
        "scarcity",
        "inquiry_quiet",
        "inquiry_conflict",
        "mixed",
    ] {
        let mut config = c::matrix()
            .into_iter()
            .find(|c| c.case == case && c.mode == c::Mode::Continuous)
            .unwrap();
        config.horizon = 64;
        config.period = 11;
        let reference = c::trial(config.clone());
        let mut world = World::new(config).unwrap();
        while !world.complete() {
            let bytes = world.checkpoint().unwrap();
            let restored = World::restore(&bytes).unwrap();
            assert_eq!(
                bytes,
                restored.checkpoint().unwrap(),
                "restored full state at{}",
                world.tick()
            );
            world = restored;
            world.step().unwrap();
            count += 1;
        }
        assert_eq!(
            serde_json::to_vec(&reference).unwrap(),
            serde_json::to_vec(&world.trial().unwrap()).unwrap(),
            "{case}"
        );
    }
    assert!(count > 300);
}

#[test]
fn unfinished_is_not_terminal_and_corruption_is_rejected() {
    let config = c::matrix()
        .into_iter()
        .find(|c| c.case == "paired" && c.mode == c::Mode::Continuous)
        .unwrap();
    let mut world = World::new(config).unwrap();
    assert!(world.trial().is_err());
    world.advance(1).unwrap();
    let bytes = world.checkpoint().unwrap();
    assert!(!world.commitments().is_empty() || !world.pending().is_empty());
    for index in [0, bytes.len() / 2, bytes.len() - 1] {
        let mut damaged = bytes.clone();
        damaged[index] ^= 1;
        assert!(World::restore(&damaged).is_err());
    }
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(World::restore(&trailing).is_err());
    assert_eq!(bytes, world.checkpoint().unwrap());
}
