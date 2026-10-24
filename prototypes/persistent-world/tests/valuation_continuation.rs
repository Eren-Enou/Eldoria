use woi_persistent_world::{continuous as c, persistent::World, valuation as v};

#[test]
fn both_registered_valuation_modes_match_original_across_all_cases() {
    for case in [
        "paired",
        "acquisition",
        "busy_recipient",
        "consumption",
        "durations",
        "exhaustion",
        "inquiry_conflict",
        "inquiry_quiet",
        "mixed",
        "null",
        "ongoing",
        "scarcity",
    ] {
        for valuation in [v::Mode::Baseline, v::Mode::MealRelief] {
            let mut config = c::matrix()
                .into_iter()
                .find(|c| c.case == case && c.mode == c::Mode::Continuous)
                .unwrap();
            config.horizon = 64;
            config.period = 11;
            let registered = v::Config {
                world: config,
                valuation,
                family: "persistent_engineering".into(),
                source023: None,
            };
            let original = v::trial(registered.clone());
            let mut world = World::new_valuation(registered).unwrap();
            let mut step = 0_usize;
            while !world.complete() {
                if step.is_multiple_of(7) {
                    let bytes = world.checkpoint().unwrap();
                    world = World::restore(&bytes).unwrap();
                    assert_eq!(bytes, world.checkpoint().unwrap());
                }
                world.step().unwrap();
                step += 1;
            }
            assert!(world.trial().is_err());
            assert_eq!(
                serde_json::to_vec(&original).unwrap(),
                world.export().unwrap(),
                "{case} {valuation:?}"
            );
        }
    }
}

#[test]
fn long_valuation_restarts_preserve_original_outcomes() {
    for valuation in [v::Mode::Baseline, v::Mode::MealRelief] {
        let mut config = c::matrix()
            .into_iter()
            .find(|c| c.case == "consumption" && c.mode == c::Mode::Continuous)
            .unwrap();
        config.horizon = 1024;
        config.seed = 42;
        config.period = 11;
        let registered = v::Config {
            world: config,
            valuation,
            family: "persistent_long".into(),
            source023: None,
        };
        let original = v::trial(registered.clone());
        let mut world = World::new_valuation(registered).unwrap();
        for steps in [0, 1, 2, 10, 100, 200, 300, 400, 2048] {
            world.advance(steps).unwrap();
            let bytes = world.checkpoint().unwrap();
            world = World::restore(&bytes).unwrap();
            assert_eq!(bytes, world.checkpoint().unwrap());
        }
        assert_eq!(original, world.valuation_trial().unwrap());
    }
}
