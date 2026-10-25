use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use woi_persistent_world::{
    continuous as c,
    persistent::session::{Session, reliability},
    valuation as v,
};
#[test]
fn raw_reader_verifies_all_modes_without_restoring_worlds() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/recovery-validation")
        .join(format!(
            "independent-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&root).unwrap();
    let root = root.canonicalize().unwrap();
    for case in [
        "null",
        "paired",
        "scarcity",
        "inquiry_quiet",
        "inquiry_conflict",
        "mixed",
    ] {
        for (index, mode) in [None, Some(v::Mode::Baseline), Some(v::Mode::MealRelief)]
            .into_iter()
            .enumerate()
        {
            let mut config = c::matrix()
                .into_iter()
                .find(|c| c.case == case && c.mode == c::Mode::Continuous)
                .unwrap();
            config.horizon = 64;
            config.seed = [0, 42, u64::MAX][index];
            let path = root.join(format!("{}-{index}", case.replace('_', "-")));
            let mut s = match mode {
                None => Session::create(&path, config).unwrap(),
                Some(valuation) => Session::create_valuation(
                    &path,
                    v::Config {
                        world: config,
                        valuation,
                        family: "independent_reading".into(),
                        source023: None,
                    },
                )
                .unwrap(),
            };
            drop(s);
            let initial = reliability::verify_independent(&path, 1)
                .unwrap_or_else(|e| panic!("{case} {mode:?} initial: {e}"));
            s = Session::open(&path).unwrap();
            s.advance(3).unwrap();
            s.save().unwrap();
            drop(s);
            let active = reliability::verify_independent(&path, 2)
                .unwrap_or_else(|e| panic!("{case} {mode:?} active: {e}"));
            assert!(active.tick > initial.tick);
            s = Session::open(&path).unwrap();
            s.advance(2048).unwrap();
            s.save().unwrap();
            drop(s);
            let end = reliability::verify_independent(&path, 3)
                .unwrap_or_else(|e| panic!("{case} {mode:?} terminal: {e}"));
            assert_eq!(end.started_commitments, end.completed_commitments);
            assert_eq!(end.conserved_food, initial.conserved_food);
        }
    }
}
