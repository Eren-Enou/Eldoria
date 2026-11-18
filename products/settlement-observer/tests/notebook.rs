use eldoria_observer::{
    host::{Command, Host},
    notebook::{Availability, MAX_BODY, Store},
    projection::{Direction, EventFilter, Index, PAGE_SIZE, Perspective, View},
};
use eldoria_settlement::{Config, Settlement, observer, storage};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

fn config() -> Config {
    Config {
        seed: 42,
        population: 12,
        horizon: 128,
    }
}
fn directory() -> PathBuf {
    std::env::temp_dir().join(format!(
        "eldoria-notebook-test-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}
fn view(host: &mut Host, perspective: Perspective, filter: EventFilter, selected: Option<u64>) {
    host.apply(Command::View {
        perspective,
        filter,
        page: 0,
        selected,
    })
    .unwrap();
}
fn advance(host: &mut Host, ticks: usize) {
    for _ in 0..ticks {
        host.apply(Command::Step).unwrap();
    }
}

#[test]
fn durable_revisions_navigation_and_character_boundary_preserve_complete_world() {
    let root = directory();
    let mut host = Host::with_notebook(config(), root.clone()).unwrap();
    advance(&mut host, 32);
    host.apply(Command::Navigate(Direction::Latest)).unwrap();
    let anchor = host.frame().unwrap().selected_anchor.unwrap();
    let checkpoint = storage::encode(&host.world).unwrap();
    let dirty = host.dirty;
    let save = || Command::SaveBookmark {
        anchor: anchor.clone(),
        expected_revision: 0,
        title: "Private observer title".into(),
        body: "SECRET external analysis".into(),
    };
    host.apply(save()).unwrap();
    let dir = root.join(&anchor.world);
    let original = dir.join(format!("{}-r0001.json", anchor.key()));
    let first = fs::read(&original).unwrap();
    host.apply(Command::SaveBookmark {
        anchor: anchor.clone(),
        expected_revision: 1,
        title: "Revised observer title".into(),
        body: "SECRET revision two".into(),
    })
    .unwrap();
    assert_eq!(first, fs::read(&original).unwrap());
    assert!(host.apply(save()).is_err(), "stale edit must fail");
    host.apply(Command::JumpBookmark(anchor.key())).unwrap();
    assert_eq!(
        host.frame()
            .unwrap()
            .notebook
            .unwrap()
            .selected
            .unwrap()
            .revision,
        2
    );
    for filter in [
        EventFilter::All,
        EventFilter::Consequences,
        EventFilter::Social,
        EventFilter::Failures,
    ] {
        view(
            &mut host,
            Perspective::Observer,
            filter,
            Some(anchor.record),
        );
        for direction in [Direction::Earlier, Direction::Later, Direction::Latest] {
            host.apply(Command::Navigate(direction)).unwrap();
            let frame = host.frame().unwrap();
            assert!(frame.rows.len() <= PAGE_SIZE);
            if let Some(record) = frame.selected_id {
                assert!(frame.rows.iter().any(|r| r.id == record));
            }
        }
    }
    let id = *host.world.characters().keys().next().unwrap();
    view(
        &mut host,
        Perspective::Character(id),
        EventFilter::Consequences,
        None,
    );
    host.apply(Command::Navigate(Direction::Latest)).unwrap();
    let frame = host.frame().unwrap();
    assert!(frame.notebook.is_none() && frame.selected_anchor.is_none());
    assert!(!serde_json::to_string(&frame).unwrap().contains("SECRET"));
    assert!(host.apply(save()).is_err());
    assert!(host.apply(Command::JumpBookmark(anchor.key())).is_err());
    assert!(host.apply(Command::ReloadNotebook).is_err());
    assert_eq!(dirty, host.dirty);
    assert_eq!(checkpoint, storage::encode(&host.world).unwrap());
    // A fresh process-equivalent host reloads metadata only after validated world load.
    let world_path = root.join("world.json");
    storage::save(&host.world, &world_path).unwrap();
    let mut restarted = Host::with_notebook(config(), root).unwrap();
    restarted.apply(Command::Open(world_path)).unwrap();
    restarted
        .apply(Command::JumpBookmark(anchor.key()))
        .unwrap();
    let note = restarted
        .frame()
        .unwrap()
        .notebook
        .unwrap()
        .selected
        .unwrap();
    assert_eq!(note.body, "SECRET revision two");
    assert_eq!(checkpoint, storage::encode(&restarted.world).unwrap());
    assert!(!restarted.dirty);
}

#[test]
fn history_anchors_reject_future_wrong_world_and_changed_prefix_without_execution() {
    let root = directory();
    let mut host = Host::with_notebook(config(), root.clone()).unwrap();
    advance(&mut host, 8);
    let early = root.join("early.json");
    fs::create_dir_all(&root).unwrap();
    storage::save(&host.world, &early).unwrap();
    advance(&mut host, 24);
    host.apply(Command::Navigate(Direction::Latest)).unwrap();
    let anchor = host.frame().unwrap().selected_anchor.unwrap();
    host.apply(Command::SaveBookmark {
        anchor: anchor.clone(),
        expected_revision: 0,
        title: "Later event".into(),
        body: String::new(),
    })
    .unwrap();
    let late = storage::encode(&host.world).unwrap();
    for field in ["world", "prefix", "tick"] {
        let mut invalid = anchor.clone();
        match field {
            "world" => invalid.world = "0".repeat(64),
            "prefix" => invalid.prefix = "0".repeat(64),
            _ => invalid.tick += 1,
        }
        assert!(
            host.apply(Command::SaveBookmark {
                anchor: invalid,
                expected_revision: 0,
                title: "Invalid".into(),
                body: String::new()
            })
            .is_err()
        );
    }
    assert_eq!(late, storage::encode(&host.world).unwrap());
    host.apply(Command::Open(early)).unwrap();
    let frame = host.frame().unwrap();
    assert_eq!(
        frame.notebook.unwrap().entries[0].availability,
        Availability::BeyondPrefix
    );
    let before = storage::encode(&host.world).unwrap();
    assert!(host.apply(Command::JumpBookmark(anchor.key())).is_err());
    assert_eq!(before, storage::encode(&host.world).unwrap());
    advance(&mut host, 24);
    host.apply(Command::JumpBookmark(anchor.key())).unwrap();
    assert_eq!(late, storage::encode(&host.world).unwrap());
    // Well-formed metadata from a different prefix is displayed as unavailable.
    let mut different = anchor.clone();
    different.prefix = "0".repeat(64);
    Store::new(root)
        .save(different.clone(), 0, "Other branch".into(), String::new())
        .unwrap();
    host.apply(Command::ReloadNotebook).unwrap();
    assert!(
        host.frame()
            .unwrap()
            .notebook
            .unwrap()
            .entries
            .iter()
            .any(|n| n.availability == Availability::DifferentHistory)
    );
    assert!(host.apply(Command::JumpBookmark(different.key())).is_err());
}

#[test]
fn corrupt_metadata_and_oversize_notes_fail_without_falling_back_or_harming_world() {
    let root = directory();
    let mut host = Host::with_notebook(config(), root.clone()).unwrap();
    advance(&mut host, 8);
    view(&mut host, Perspective::Observer, EventFilter::All, Some(0));
    let anchor = host.frame().unwrap().selected_anchor.unwrap();
    let store = Store::new(root.clone());
    assert!(
        store
            .save(
                anchor.clone(),
                0,
                "Oversize".into(),
                "a".repeat(MAX_BODY + 1)
            )
            .is_err()
    );
    assert!(!root.exists());
    store
        .save(anchor.clone(), 0, "Original".into(), "retained".into())
        .unwrap();
    store
        .save(anchor.clone(), 1, "Revision".into(), "second".into())
        .unwrap();
    let file = root
        .join(&anchor.world)
        .join(format!("{}-r0002.json", anchor.key()));
    let mut json: serde_json::Value = serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
    json["bookmark"]["body"] = "untrusted edit".into();
    fs::write(file, serde_json::to_vec(&json).unwrap()).unwrap();
    let before = storage::encode(&host.world).unwrap();
    assert!(host.apply(Command::ReloadNotebook).is_err());
    let notebook = host.frame().unwrap().notebook.unwrap();
    assert!(notebook.error.is_some() && notebook.entries.is_empty());
    assert!(
        store
            .save(anchor, 1, "Overwrite".into(), String::new())
            .is_err()
    );
    assert_eq!(before, storage::encode(&host.world).unwrap());
    host.apply(Command::Step).unwrap(); // Notebook corruption does not stop the world.
}

#[test]
fn incremental_index_matches_full_snapshot_categories_and_rebuilt_prefixes() {
    let mut host = Host::new(config()).unwrap();
    for _ in 0..64 {
        host.apply(Command::Step).unwrap();
        let frame = host.frame().unwrap();
        let View::Observer(snapshot) = frame.view else {
            panic!("observer expected")
        };
        assert_eq!(
            serde_json::to_value(snapshot).unwrap(),
            serde_json::to_value(observer::snapshot(&host.world)).unwrap()
        );
    }
    let mut rebuilt = Index::default();
    rebuilt.update(&host.world);
    let mut repeated = Index::default();
    let mut world = Settlement::new(config()).unwrap();
    for _ in 0..64 {
        world.advance(1).unwrap();
        repeated.update(&world);
    }
    for record in &world.state().history {
        assert_eq!(rebuilt.prefix(record.id), repeated.prefix(record.id));
    }
    for filter in [
        EventFilter::All,
        EventFilter::Consequences,
        EventFilter::Social,
        EventFilter::Failures,
    ] {
        let direct: Vec<_> = world
            .state()
            .history
            .iter()
            .filter(|r| filter.matches(r))
            .map(|r| r.id)
            .collect();
        assert!(!direct.is_empty());
        assert_eq!(
            rebuilt
                .visible(&world, Perspective::Observer, filter)
                .unwrap()
                .as_ref(),
            direct
        );
    }
}
