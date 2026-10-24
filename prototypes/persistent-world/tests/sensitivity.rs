use woi_persistent_world::{continuous as c, encounters::Order, persistent::World};

#[test]
fn declared_order_phase_timing_and_seed_survive_continuation() {
    let cases = [
        (
            "busy_recipient",
            Order::Descending,
            c::Phase::Absolute,
            8,
            1,
            u64::MAX,
        ),
        (
            "consumption",
            Order::Rotating,
            c::Phase::Absolute,
            2,
            128,
            0,
        ),
        ("durations", Order::Descending, c::Phase::Epoch, 8, 11, 42),
        ("ongoing", Order::Rotating, c::Phase::Epoch, 2, 1, u64::MAX),
        (
            "exhaustion",
            Order::Descending,
            c::Phase::Absolute,
            4,
            128,
            0,
        ),
    ];
    for (case, order, phase, duration, period, seed) in cases {
        let mut config = c::matrix()
            .into_iter()
            .find(|c| c.case == case && c.mode == c::Mode::Continuous)
            .unwrap();
        config.horizon = 128;
        config.order = order;
        config.phase = phase;
        config.duration = duration;
        config.period = period;
        config.seed = seed;
        let original = c::trial(config.clone());
        let mut world = World::new(config).unwrap();
        let mut count = 0_usize;
        while !world.complete() {
            if count.is_multiple_of(11) {
                let bytes = world.checkpoint().unwrap();
                world = World::restore(&bytes).unwrap();
                assert_eq!(bytes, world.checkpoint().unwrap());
            }
            world.step().unwrap();
            count += 1;
        }
        assert_eq!(
            original,
            world.trial().unwrap(),
            "{case} {order:?} {phase:?}"
        );
    }
}
