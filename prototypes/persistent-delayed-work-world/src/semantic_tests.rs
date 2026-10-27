use super::*;
#[derive(Clone, Serialize, Deserialize)]
struct RawCommit {
    binding_sha256: String,
    schema: String,
    world: String,
    runtime: String,
    generation: u64,
    tick: u64,
    journal_bytes: u64,
    journal_sha256: String,
    state_sha256: String,
    #[serde(with = "serde_bytes")]
    state: Vec<u8>,
}
fn rebind(saved: &Saved) -> Vec<u8> {
    let payload = encode(saved).unwrap();
    encode(&Envelope {
        schema: SCHEMA.into(),
        runtime: IDENTITY.into(),
        checksum: format!("{:x}", Sha256::digest(&payload)),
        payload,
    })
    .unwrap()
}
fn reject_saved(s: &Saved, template: &RawCommit, prefix: &[u8]) {
    let mut forged = template.clone();
    forged.state = rebind(s);
    assert!(World::restore(&forged.state).is_err());
    forged.state_sha256 = format!("{:x}", Sha256::digest(&forged.state));
    let binding = serde_json::to_vec(&(
        &forged.schema,
        &forged.world,
        &forged.runtime,
        forged.generation,
        forged.tick,
        forged.journal_bytes,
        &forged.journal_sha256,
        &forged.state_sha256,
    ))
    .unwrap();
    forged.binding_sha256 = format!("{:x}", Sha256::digest(binding));
    assert!(crate::verification::verify(&encode(&forged).unwrap(), prefix).is_err());
}
#[test]
fn recomputed_hashes_do_not_hide_material_corruption() {
    crate::tests::require_all100_archived_parity();
    let path = std::env::temp_dir().join(format!(
        "woi-C1-independent-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut session = session::Session::create(&path, crate::delayed::matrix()[3].clone()).unwrap();
    while session
        .world()
        .material()
        .active
        .iter()
        .all(|a| a.escrow != 2)
    {
        session.advance(1).unwrap();
    }
    session.save().unwrap();
    let world = session.world();
    let template: RawCommit =
        decode(&std::fs::read(path.join("checkpoint-00000002.cbor")).unwrap()).unwrap();
    let prefix = std::fs::read(path.join("journal.jsonl")).unwrap();
    crate::verification::verify(&encode(&template).unwrap(), &prefix).unwrap();
    let mut checks = 0;
    macro_rules! reject {
        ($change:expr) => {{
            let mut s = world.saved();
            $change(&mut s);
            reject_saved(&s, &template, &prefix);
            checks += 1;
        }};
    }
    reject!(|s: &mut Saved| s.material.raw += 1);
    reject!(|s: &mut Saved| s.material.active[0].escrow = 0);
    reject!(|s: &mut Saved| s.material.active[0].phase = m::Phase::Extracting);
    reject!(|s: &mut Saved| s.material.active[0].process.finish_at += 1);
    reject!(|s: &mut Saved| s.material.active[0].process.owner = 3);
    reject!(|s: &mut Saved| s.material.active.clear());
    reject!(|s: &mut Saved| s.material.operations.push(s.material.operations[0].clone()));
    reject!(|s: &mut Saved| s.material.settings.raw_initial += 1);
    reject!(|s: &mut Saved| s.state.active.push(s.state.active[0].clone()));
    reject!(|s: &mut Saved| s.material.memories.pop().unwrap());
    reject!(|s: &mut Saved| s.agents[0].food += 1);
    reject!(|s: &mut Saved| s.agents[0].hunger += 1);
    reject!(|s: &mut Saved| s.delayed.mode = crate::delayed::Mode::Baseline);
    reject!(|s: &mut Saved| s
        .boundaries
        .iter_mut()
        .flat_map(|b| &mut b.choices)
        .find(|c| c.candidates[4].eligible)
        .unwrap()
        .candidates[4]
        .final_score += 1);
    reject!(|s: &mut Saved| s
        .boundaries
        .iter_mut()
        .flat_map(|b| &mut b.choices)
        .find(|c| c.candidates[4].eligible)
        .unwrap()
        .input
        .raw_retained
        .as_mut()
        .unwrap()
        .observation
        .preparation_ticks += 1);
    assert_eq!(checks, 15);
    session.advance(8).unwrap();
    session.save().unwrap();
    let template: RawCommit =
        decode(&std::fs::read(path.join("checkpoint-00000003.cbor")).unwrap()).unwrap();
    let prefix = std::fs::read(path.join("journal.jsonl")).unwrap();
    let mut s = session.world().saved();
    assert!(s.material.waste > 0);
    s.material.waste -= 1;
    reject_saved(&s, &template, &prefix);
    drop(session);
    std::fs::remove_dir_all(path).unwrap();
}
#[test]
fn archive_cannot_restore_an_overwritten_raw_observation() {
    crate::tests::require_all100_archived_parity();
    let path = std::env::temp_dir().join(format!(
        "woi-C1-forgetting-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut session =
        session::Session::create(&path, crate::delayed::matrix()[29].clone()).unwrap();
    let (owner, forgotten) = loop {
        session.advance(1).unwrap();
        if let Some(memory) = session.world().material().memories.iter().rev().find(|r| {
            r.kind == m::Change::OverwrittenByReady
                && session.world().material().slots[r.owner as usize].is_none()
        }) {
            break (memory.owner, memory.before.clone().unwrap());
        }
        assert!(
            !session.world().complete(),
            "registered overwrite witness absent"
        );
    };
    session.save().unwrap();
    let template: RawCommit =
        decode(&std::fs::read(path.join("checkpoint-00000002.cbor")).unwrap()).unwrap();
    let prefix = std::fs::read(path.join("journal.jsonl")).unwrap();
    let restored = World::restore(&session.world().checkpoint().unwrap()).unwrap();
    assert!(restored.material().slots[owner as usize].is_none());
    let mut saved = session.world().saved();
    saved.material.slots[owner as usize] = Some(forgotten);
    reject_saved(&saved, &template, &prefix);
    drop(session);
    std::fs::remove_dir_all(path).unwrap();
}

#[test]
fn pending_response_and_continue_corruptions_reject_without_repair() {
    crate::tests::require_all100_archived_parity();
    let path = std::env::temp_dir().join(format!(
        "woi-C1-pending-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut session =
        session::Session::create(&path, crate::delayed::matrix()[43].clone()).unwrap();
    while !session
        .world()
        .pending()
        .iter()
        .any(|p| p.response.is_some())
    {
        session.advance(1).unwrap();
        assert!(!session.world().complete());
    }
    session.save().unwrap();
    let template: RawCommit =
        decode(&std::fs::read(path.join("checkpoint-00000002.cbor")).unwrap()).unwrap();
    let prefix = std::fs::read(path.join("journal.jsonl")).unwrap();
    let mut saved = session.world().saved();
    saved
        .state
        .pending
        .iter_mut()
        .find(|p| p.response.is_some())
        .unwrap()
        .response = None;
    reject_saved(&saved, &template, &prefix);
    let mut saved = session.world().saved();
    saved.state.pending.clear();
    reject_saved(&saved, &template, &prefix);
    while !session
        .world()
        .boundaries()
        .iter()
        .flat_map(|b| &b.responses)
        .flat_map(|r| &r.candidates)
        .any(|c| c.reply == crate::encounters::Reply::Continue)
    {
        session.advance(1).unwrap();
        assert!(!session.world().complete());
    }
    session.save().unwrap();
    let template: RawCommit = decode(
        &std::fs::read(path.join(format!("checkpoint-{:08}.cbor", session.generation()))).unwrap(),
    )
    .unwrap();
    let prefix = std::fs::read(path.join("journal.jsonl")).unwrap();
    let mut saved = session.world().saved();
    saved
        .boundaries
        .iter_mut()
        .flat_map(|b| &mut b.responses)
        .flat_map(|r| &mut r.candidates)
        .find(|c| c.reply == crate::encounters::Reply::Continue)
        .unwrap()
        .score = 1;
    reject_saved(&saved, &template, &prefix);
    drop(session);
    std::fs::remove_dir_all(path).unwrap();
}
#[test]
fn observer_events_cannot_restore_an_evicted_episode() {
    crate::tests::require_all100_archived_parity();
    let path = std::env::temp_dir().join(format!(
        "woi-C1-episode-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut session =
        session::Session::create(&path, crate::delayed::matrix()[87].clone()).unwrap();
    session.advance(2048).unwrap();
    session.save().unwrap();
    let w = session.world();
    let mut saved = w.saved();
    let (owner, old) = saved
        .runtime
        .events
        .iter()
        .find_map(|e| {
            e.participants
                .iter()
                .enumerate()
                .find_map(|(position, owner)| {
                    (!saved.agents[*owner as usize]
                        .memories
                        .iter()
                        .any(|m| m.event == e.id))
                    .then(|| (*owner as usize, e.interpretations[position].clone()))
                })
        })
        .expect("actual evicted episode retained only in observer history");
    assert_eq!(saved.agents[owner].memories.len(), 16);
    saved.agents[owner].memories.pop_back();
    saved.agents[owner].memories.push_front(old);
    let template: RawCommit =
        decode(&std::fs::read(path.join("checkpoint-00000002.cbor")).unwrap()).unwrap();
    let prefix = std::fs::read(path.join("journal.jsonl")).unwrap();
    reject_saved(&saved, &template, &prefix);
    drop(session);
    std::fs::remove_dir_all(path).unwrap();
}
