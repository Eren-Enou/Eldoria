use eldoria_observer::{
    host::{Client, Command, Host},
    projection::{PAGE_SIZE, Perspective, View},
};
use eldoria_settlement::{Config, Settlement, model::*, storage};
use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

fn config(population: usize) -> Config {
    Config {
        population,
        seed: 42,
        horizon: 128,
    }
}
fn directory() -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "eldoria-observer-test-{}-{now}",
        std::process::id()
    ));
    std::fs::create_dir(&path).unwrap();
    path
}
#[test]
fn stepping_perspectives_and_history_selection_never_change_rules() {
    for population in [8, 12] {
        let mut host = Host::new(config(population)).unwrap();
        let mut baseline = Settlement::new(config(population)).unwrap();
        for _ in 0..64 {
            host.apply(Command::Step).unwrap();
            let id = host.world.characters().keys().next().copied().unwrap();
            host.apply(Command::View {
                filter: eldoria_observer::projection::EventFilter::All,
                perspective: Perspective::Character(id),
                page: 0,
                selected: Some(0),
            })
            .unwrap();
            host.frame().unwrap();
            host.apply(Command::View {
                filter: eldoria_observer::projection::EventFilter::All,
                perspective: Perspective::Observer,
                page: 1,
                selected: None,
            })
            .unwrap();
        }
        baseline.advance(64).unwrap();
        assert_eq!(
            storage::encode(&host.world).unwrap(),
            storage::encode(&baseline).unwrap()
        );
        let frame = host.frame().unwrap();
        assert_eq!(frame.rows.len(), PAGE_SIZE);
        assert_eq!(frame.page, 1);
    }
}
#[test]
fn actor_projection_and_causal_traversal_cannot_expose_foreign_private_inputs() {
    let mut host = Host::new(config(12)).unwrap();
    for _ in 0..64 {
        host.apply(Command::Step).unwrap();
    }
    let id = host.world.characters().keys().next().copied().unwrap();
    let foreign = host
        .world
        .state()
        .history
        .iter()
        .find(|r| matches!(&r.kind, RecordKind::Decision { view, .. } if view.own.id != id))
        .unwrap()
        .id;
    host.apply(Command::View {
        filter: eldoria_observer::projection::EventFilter::All,
        perspective: Perspective::Character(id),
        page: 0,
        selected: Some(foreign),
    })
    .unwrap();
    let frame = host.frame().unwrap();
    assert!(matches!(&frame.view, View::Character(v) if v.own.id == id));
    assert!(frame.detail.is_none() && frame.detail_error.is_some());
    assert!(
        frame
            .activities
            .iter()
            .all(|c| c.participants.contains(&id))
    );
    let json = serde_json::to_value(&frame).unwrap();
    assert!(json["view"].get("Observer").is_none());
    for person in json["view"]["Character"]["people"].as_array().unwrap() {
        for private in [
            "food",
            "hunger",
            "memories",
            "beliefs",
            "relationships",
            "generosity",
            "caution",
        ] {
            assert!(person.get(private).is_none());
        }
    }
    for row in &frame.rows {
        host.apply(Command::View {
            filter: eldoria_observer::projection::EventFilter::All,
            perspective: Perspective::Character(id),
            page: 0,
            selected: Some(row.id),
        })
        .unwrap();
        let detail = host.frame().unwrap().detail.unwrap();
        if let RecordKind::Decision { view, .. } = &detail.record.kind {
            assert_eq!(view.own.id, id);
        }
        for link in detail.links {
            host.apply(Command::View {
                filter: eldoria_observer::projection::EventFilter::All,
                perspective: Perspective::Character(id),
                page: 0,
                selected: Some(link.id),
            })
            .unwrap();
            let next = host.frame().unwrap().detail.unwrap();
            if let RecordKind::Decision { view, .. } = next.record.kind {
                assert_eq!(view.own.id, id);
            }
        }
    }
    host.apply(Command::View {
        filter: eldoria_observer::projection::EventFilter::All,
        perspective: Perspective::Observer,
        page: 0,
        selected: Some(foreign),
    })
    .unwrap();
    assert!(host.frame().unwrap().detail.is_some());
}
#[test]
fn save_open_pending_work_and_failure_paths_preserve_complete_state() {
    let mut host = Host::new(config(12)).unwrap();
    for _ in 0..13 {
        host.apply(Command::Step).unwrap();
    }
    assert!(!host.world.state().commitments.is_empty());
    let paused = storage::encode(&host.world).unwrap();
    let dir = directory();
    let save = dir.join("generation.json");
    host.apply(Command::Save(save.clone())).unwrap();
    assert!(!host.dirty && !host.running);
    assert_eq!(paused, std::fs::read(&save).unwrap());
    host.apply(Command::Step).unwrap();
    assert!(host.dirty);
    let current = storage::encode(&host.world).unwrap();
    assert!(host.apply(Command::Open(dir.join("missing.json"))).is_err());
    assert_eq!(current, storage::encode(&host.world).unwrap());
    assert!(host.apply(Command::Save(save.clone())).is_err());
    assert_eq!(paused, std::fs::read(&save).unwrap());
    host.apply(Command::Open(save)).unwrap();
    assert_eq!(paused, storage::encode(&host.world).unwrap());
    for _ in 13..128 {
        host.apply(Command::Step).unwrap();
    }
    let mut baseline = Settlement::new(config(12)).unwrap();
    baseline.advance(128).unwrap();
    assert_eq!(
        storage::encode(&host.world).unwrap(),
        storage::encode(&baseline).unwrap()
    );
}
#[test]
fn causal_links_connect_actual_decisions_work_and_consequences() {
    let mut host = Host::new(config(8)).unwrap();
    for _ in 0..32 {
        host.apply(Command::Step).unwrap();
    }
    let finished = host
        .world
        .state()
        .history
        .iter()
        .find(|r| matches!(r.kind, RecordKind::Finished { .. }))
        .unwrap()
        .id;
    host.apply(Command::View {
        filter: eldoria_observer::projection::EventFilter::All,
        perspective: Perspective::Observer,
        page: 0,
        selected: Some(finished),
    })
    .unwrap();
    let detail = host.frame().unwrap().detail.unwrap();
    assert!(detail.links.iter().any(|r| matches!(
        host.world.state().history[r.id as usize].kind,
        RecordKind::Started { .. }
    )));
    assert!(detail.links.iter().any(|r| matches!(
        host.world.state().history[r.id as usize].kind,
        RecordKind::Decision { .. }
    )));
}
#[test]
fn worker_pause_ack_and_dropped_frames_do_not_drop_world_events() {
    let mut client = Client::spawn(config(8)).unwrap();
    let rate = client.send(Command::Rate(64)).unwrap();
    wait(&client, rate);
    let run = client.send(Command::Running(true)).unwrap();
    wait(&client, run);
    thread::sleep(Duration::from_millis(200));
    let pause = client.send(Command::Running(false)).unwrap();
    wait(&client, pause);
    let frame = client.take_frame().unwrap();
    assert!(!frame.running && frame.tick > 0);
    let tick = frame.tick;
    thread::sleep(Duration::from_millis(70));
    let step = client.send(Command::Step).unwrap();
    wait(&client, step);
    let frame = client.take_frame().unwrap();
    assert_eq!(frame.tick, tick + 1);
    let dir = directory();
    let file = dir.join("worker.json");
    let save = client.send(Command::Save(file.clone())).unwrap();
    wait(&client, save);
    let loaded = storage::load(&file).unwrap();
    let mut baseline = Settlement::new(config(8)).unwrap();
    baseline.advance(frame.tick).unwrap();
    assert_eq!(
        storage::encode(&loaded).unwrap(),
        storage::encode(&baseline).unwrap()
    );
}
fn wait(client: &Client, id: u64) {
    let until = Instant::now() + Duration::from_secs(10);
    while Instant::now() < until {
        if let Some(reply) = client.poll().unwrap() {
            assert!(reply.result.is_ok(), "{:?}", reply.result);
            if reply.id == id {
                return;
            }
        }
        thread::sleep(Duration::from_millis(5));
    }
    panic!("worker did not acknowledge request{id}");
}
