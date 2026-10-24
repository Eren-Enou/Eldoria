use woi_persistent_world::{continuous as c, persistent::World};

#[test]
fn long_segmented_full_state_and_original_history() {
    let mut checkpoints = 0;
    for case in ["null", "paired", "scarcity", "inquiry_conflict"] {
        for seed in [0, 42, u64::MAX] {
            let mut config = c::matrix()
                .into_iter()
                .find(|c| c.case == case && c.mode == c::Mode::Continuous)
                .unwrap();
            config.horizon = 1024;
            config.period = 11;
            config.seed = seed;
            let reference = c::trial(config.clone());
            let mut uninterrupted = World::new(config.clone()).unwrap();
            let mut segmented = World::new(config).unwrap();
            let splits = [
                0, 1, 2, 3, 10, 11, 12, 31, 63, 100, 300, 700, 1022, 1023, 1024, 1025, 1055, 1056,
                1057,
            ];
            let mut split = 0;
            loop {
                while split < splits.len()
                    && uninterrupted.tick() - uninterrupted.start() >= splits[split]
                {
                    let bytes = segmented.checkpoint().unwrap();
                    assert_eq!(
                        bytes,
                        uninterrupted.checkpoint().unwrap(),
                        "{case} seed{seed} split{}",
                        splits[split]
                    );
                    segmented = World::restore(&bytes).unwrap();
                    assert_eq!(bytes, segmented.checkpoint().unwrap());
                    checkpoints += 1;
                    split += 1;
                }
                if uninterrupted.complete() {
                    break;
                }
                uninterrupted.step().unwrap();
                segmented.step().unwrap();
            }
            assert_eq!(reference, segmented.trial().unwrap(), "{case} seed{seed}");
            assert_eq!(reference, uninterrupted.trial().unwrap());
        }
    }
    assert_eq!(checkpoints, 12 * 19);
}

#[test]
fn continuous_horizon_only_changes_later_cutoff_not_prior_input() {
    let mut short = c::matrix()
        .into_iter()
        .find(|c| c.case == "paired" && c.mode == c::Mode::Continuous)
        .unwrap();
    short.horizon = 32;
    let mut long = short.clone();
    long.horizon = 64;
    let a = c::trial(short);
    let b = c::trial(long);
    let cutoff = a.history.start + 32;
    assert_eq!(
        a.history
            .boundaries
            .iter()
            .filter(|r| r.tick < cutoff)
            .collect::<Vec<_>>(),
        b.history
            .boundaries
            .iter()
            .filter(|r| r.tick < cutoff)
            .collect::<Vec<_>>()
    );
    assert_ne!(a.history.end, b.history.end);
}
