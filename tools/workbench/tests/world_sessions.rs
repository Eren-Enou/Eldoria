#![cfg(feature = "world-sessions")]
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use woi_workbench::{
    config::{Configuration, Mode, Valuation},
    storage::Store,
    world_sessions::Sessions,
};
fn store() -> Store {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    let root = repo
        .join("target/persistent-world-validation")
        .join(format!(
            "workbench-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&root).unwrap();
    Store {
        repo,
        root,
        installed: None,
    }
}
#[test]
fn active_state_reopens_without_entering_finished_run_store() {
    let store = store();
    let sessions = Sessions::new(&store).unwrap();
    let mut session = sessions
        .create(&Configuration {
            ticks: 64,
            ..Default::default()
        })
        .unwrap();
    session.advance(3).unwrap();
    session.save().unwrap();
    let state = session.world().checkpoint().unwrap();
    let id = session
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    assert!(sessions.open(&id).is_err());
    drop(session);
    let mut session = sessions.open(&id).unwrap();
    assert_eq!(state, session.world().checkpoint().unwrap());
    assert!(store.list().unwrap().is_empty());
    assert!(sessions.path("../experiments").is_err());
    assert!(session.finalize().is_err());
}
#[test]
fn explicit024_modes_keep_original_rules_and025_stays_deferred() {
    let store = store();
    let sessions = Sessions::new(&store).unwrap();
    for valuation in [Valuation::Baseline, Valuation::MealRelief] {
        let config = Configuration {
            schema_version: 2,
            mode: Mode::ActivityValuation,
            valuation: Some(valuation),
            ticks: 64,
            ..Default::default()
        };
        let registered: persistent_world::valuation::Config =
            serde_json::from_value(config.runner_config()).unwrap();
        let original = persistent_world::valuation::trial(registered);
        let mut session = sessions.create(&config).unwrap();
        session.advance(7).unwrap();
        session.save().unwrap();
        let id = session
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        drop(session);
        let mut session = sessions.open(&id).unwrap();
        session.advance(2048).unwrap();
        session.finalize().unwrap();
        assert_eq!(session.world().valuation_trial().unwrap(), original);
        let export = fs::read(session.path().join("history.jsonl")).unwrap();
        session.finalize().unwrap();
        assert_eq!(
            export,
            fs::read(session.path().join("history.jsonl")).unwrap()
        );
    }
    let config = Configuration {
        schema_version: 2,
        mode: Mode::MaterialPreparation,
        valuation: Some(Valuation::Baseline),
        preparation: Some(Default::default()),
        ..Default::default()
    };
    assert!(sessions.create(&config).err().unwrap().contains("deferred"));
}
