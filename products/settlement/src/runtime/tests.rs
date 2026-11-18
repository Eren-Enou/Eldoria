use super::*;
use crate::{observer, policy, storage};

fn world(population: usize, seed: u64, horizon: u64) -> Settlement {
    Settlement::new(Config {
        population,
        seed,
        horizon,
    })
    .unwrap()
}

#[test]
fn autonomous_settlements_conserve_resources_and_reconstruct_local_choices() {
    let mut aggregate = BTreeMap::<String, usize>::new();
    for population in [8, 12] {
        for seed in [1, 7, 42, 99] {
            let mut sim = world(population, seed, 512);
            sim.advance(512).unwrap();
            assert!(sim.complete());
            assert!(sim.state().commitments.is_empty());
            validate(&sim.checkpoint()).unwrap();
            let snapshot = observer::snapshot(&sim);
            assert_eq!(
                snapshot.food_balance.initial,
                snapshot.food_balance.sources
                    + snapshot.food_balance.owned
                    + snapshot.food_balance.consumed
            );
            assert!(snapshot.food_balance.consumed > 0);
            assert!(
                snapshot
                    .characters
                    .iter()
                    .map(|a| (a.food, a.hunger, a.memories.len()))
                    .collect::<BTreeSet<_>>()
                    .len()
                    > 1
            );
            for (outcome, count) in snapshot.outcomes {
                *aggregate.entry(outcome).or_default() += count;
            }
        }
    }
    for outcome in [
        "ate",
        "gathered",
        "arrived",
        "gathered_together",
        "shared_food",
        "refused:declined",
        "depleted",
    ] {
        assert!(
            aggregate.get(outcome).copied().unwrap_or(0) > 0,
            "missing consequential outcome {outcome}: {aggregate:?}"
        );
    }
}

#[test]
fn every_safe_split_preserves_complete_state_rng_and_pending_work() {
    let mut pending_seen = BTreeSet::new();
    for population in [8, 12] {
        let mut uninterrupted = world(population, 42, 64);
        uninterrupted.advance(64).unwrap();
        let expected = storage::encode(&uninterrupted).unwrap();
        let mut prefix = world(population, 42, 64);
        for split in 0..=64 {
            for c in prefix.state().commitments.values() {
                pending_seen.insert(format!("{:?}", c.work));
            }
            let bytes = storage::encode(&prefix).unwrap();
            let mut resumed = storage::decode(&bytes).unwrap();
            assert_eq!(prefix.checkpoint(), resumed.checkpoint());
            resumed.advance(64 - split).unwrap();
            assert_eq!(
                expected,
                storage::encode(&resumed).unwrap(),
                "population{population}/split{split}"
            );
            prefix.advance(1).unwrap();
        }
    }
    assert!(pending_seen.iter().any(|s| s.starts_with("Send")));
    assert!(pending_seen.iter().any(|s| s.starts_with("Travel")));
}

#[test]
fn continuation_covers_each_autonomously_reached_work_phase() {
    let mut covered = BTreeSet::new();
    for seed in [1, 7, 42, 99] {
        let mut sim = world(8, seed, 512);
        let mut prefixes = vec![];
        while !sim.complete() {
            sim.advance(1).unwrap();
            let kinds: Vec<_> = sim
                .state()
                .commitments
                .values()
                .map(|c| match c.work {
                    Work::Eat => "eat",
                    Work::Gather => "gather",
                    Work::Travel { .. } => "travel",
                    Work::Send { .. } => "send",
                    Work::Transfer { .. } => "transfer",
                    Work::JointGather => "joint",
                    Work::Reply => "reply",
                })
                .collect();
            for kind in kinds {
                if covered.insert(kind) {
                    prefixes.push(storage::encode(&sim).unwrap());
                }
            }
        }
        let expected = storage::encode(&sim).unwrap();
        for prefix in prefixes {
            let mut resumed = storage::decode(&prefix).unwrap();
            resumed.advance(512).unwrap();
            assert_eq!(expected, storage::encode(&resumed).unwrap());
        }
        if covered.len() == 7 {
            break;
        }
    }
    assert_eq!(covered.len(), 7, "missing continuation phase: {covered:?}");
}

#[test]
fn sparse_ids_and_input_order_do_not_change_roster_semantics() {
    let ids: Vec<_> = (0..12).map(|i| CharacterId(100 + 7919 * i)).collect();
    let config = Config {
        population: 12,
        seed: 7,
        horizon: 48,
    };
    let mut a = Settlement::with_ids(config.clone(), ids.clone()).unwrap();
    let mut b = Settlement::with_ids(config, ids.into_iter().rev().collect()).unwrap();
    a.advance(48).unwrap();
    b.advance(48).unwrap();
    assert_eq!(a.checkpoint(), b.checkpoint());
    assert!(a.characters().contains_key(&CharacterId(100 + 7919 * 11)));
}

#[test]
fn pause_and_observer_reads_are_inert_and_exports_are_not_saves() {
    let mut sim = world(8, 42, 64);
    sim.advance(9).unwrap();
    let before = storage::encode(&sim).unwrap();
    sim.advance(0).unwrap();
    let mut snapshot = observer::snapshot(&sim);
    snapshot.characters[0].food = 999;
    for actor in sim.characters().keys() {
        sim.owned_view(*actor).unwrap();
    }
    assert!(observer::records(&sim, usize::MAX, usize::MAX).is_empty());
    assert_eq!(before, storage::encode(&sim).unwrap());
    assert!(storage::decode(&serde_json::to_vec(&snapshot).unwrap()).is_err());
}

#[test]
fn hidden_partner_components_and_remote_stock_never_enter_decisions() {
    let sim = world(8, 42, 64);
    let mut a = sim.checkpoint();
    a.state.tick = 1;
    let mut actors: BTreeMap<_, _> = a.characters.into_iter().map(|a| (a.id, a)).collect();
    let owner = *actors.keys().next().unwrap();
    observe(&mut a.state, &mut actors, owner, None);
    let local = view(&a.state, &actors, owner, &[]);
    let expected = policy::choose(&local);
    for (id, character) in &mut actors {
        if *id != owner {
            character.food += 70;
            character.hunger = 100 - character.hunger;
            character.generosity = 100 - character.generosity;
            character.caution = 100 - character.caution;
            character.memories.clear();
            character.relationships.clear();
            character.beliefs.clear();
        }
    }
    a.state.places.get_mut(&2).unwrap().food = 0;
    // Even unobserved local physical stock is not a policy read.
    a.state.places.get_mut(&0).unwrap().food = 123;
    let twin = view(&a.state, &actors, owner, &[]);
    assert_eq!(local, twin);
    assert_eq!(expected, policy::choose(&twin));
}

#[test]
fn unknown_stock_differs_from_zero_and_forgetting_preserves_trust() {
    let sim = world(8, 7, 32);
    let mut actor = sim.characters().into_values().next().unwrap();
    let other = CharacterId(2000);
    actor.remember(Episode {
        receipt: 1,
        tick: 1,
        other: Some(other),
        kind: "help".into(),
        valence: 20,
    });
    for i in 0..20 {
        actor.remember(Episode {
            receipt: i + 2,
            tick: i + 2,
            other: None,
            kind: "work".into(),
            valence: 0,
        });
    }
    assert_eq!(actor.memories.len(), MEMORY_CAP);
    assert_eq!(actor.trust(other), 20);
    assert!(actor.beliefs.iter().all(|b| b.place != 2));
    actor.observe(SiteBelief {
        place: 2,
        stock: 0,
        observed_at: 1,
        receipt: 100,
    });
    assert_eq!(actor.beliefs.back().unwrap().stock, 0);
}

#[test]
fn refusal_uses_recipient_priority_and_acceptance_reserves_both_people() {
    let sim = world(2, 7, 32);
    let mut saved = sim.checkpoint();
    saved.state.tick = 1;
    let mut actors: BTreeMap<_, _> = saved
        .characters
        .drain(..)
        .map(|mut a| {
            a.place = 1;
            (a.id, a)
        })
        .collect();
    let ids: Vec<_> = actors.keys().copied().collect();
    let recipient = actors.get_mut(&ids[1]).unwrap();
    recipient.food = 1;
    recipient.hunger = 90;
    recipient.generosity = 0;
    observe(&mut saved.state, &mut actors, ids[1], None);
    let offer = Invitation {
        id: 99,
        proposer: ids[0],
        recipient: ids[1],
        kind: InvitationKind::RequestFood,
        place: 1,
        cause: 0,
    };
    let local = view(&saved.state, &actors, ids[1], std::slice::from_ref(&offer));
    assert_eq!(
        policy::choose(&local).selected,
        Action::Decline { invitation: 99 }
    );
    let recipient = actors.get_mut(&ids[1]).unwrap();
    recipient.food = 4;
    recipient.hunger = 0;
    recipient.generosity = 100;
    let local = view(&saved.state, &actors, ids[1], std::slice::from_ref(&offer));
    let choice = policy::choose(&local);
    assert_eq!(choice.selected, Action::Accept { invitation: 99 });
    let cause = saved.state.record(
        Some(ids[1]),
        None,
        RecordKind::Decision {
            view: Box::new(local),
            choice: choice.clone(),
            controller: Utility.identity().into(),
        },
    );
    assert!(admit(
        &mut saved.state,
        &mut actors,
        ids[1],
        choice.selected,
        cause,
        &[offer]
    ));
    assert!(saved.state.busy(ids[0]) && saved.state.busy(ids[1]));
    assert!(validate_intent(&saved.state, &actors, ids[0], &Action::Eat, &[]).is_err());
}

#[test]
fn authoritative_validation_rejects_illegal_controller_intents() {
    let sim = world(8, 7, 32);
    let actors = sim.characters();
    let owner = *actors.keys().next().unwrap();
    for action in [
        Action::Travel { to: 999 },
        Action::Invite {
            to: owner,
            kind: InvitationKind::OfferFood,
        },
        Action::Accept { invitation: 999 },
        Action::Invite {
            to: CharacterId(1),
            kind: InvitationKind::RequestFood,
        },
    ] {
        assert!(validate_intent(sim.state(), &actors, owner, &action, &[]).is_err());
    }
}

#[test]
fn completion_rechecks_rival_depletion_without_creating_food() {
    let sim = world(2, 7, 32);
    let mut saved = sim.checkpoint();
    let mut actors: BTreeMap<_, _> = saved
        .characters
        .drain(..)
        .map(|mut a| {
            a.place = 1;
            (a.id, a)
        })
        .collect();
    saved.state.places.get_mut(&1).unwrap().food = 1;
    let ids: Vec<_> = actors.keys().copied().collect();
    let initial = actors.values().map(|a| a.food).sum::<u32>();
    for (id, actor) in ids.iter().enumerate() {
        finish(
            &mut saved.state,
            &mut actors,
            Commitment {
                id: id as u64,
                participants: vec![*actor],
                place: 1,
                start: 0,
                due: 3,
                cause: 0,
                work: Work::Gather,
            },
        );
    }
    assert_eq!(actors.values().map(|a| a.food).sum::<u32>(), initial + 1);
    assert_eq!(saved.state.places[&1].food, 0);
    assert!(
        saved.state.history.iter().any(
            |r| matches!(&r.kind, RecordKind::Finished { outcome, .. } if outcome == "depleted")
        )
    );
}

#[test]
fn malformed_checkpoint_state_and_wrong_schema_are_rejected() {
    let sim = world(8, 42, 32);
    let bytes = storage::encode(&sim).unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    for field in ["schema", "rules", "runtime", "checksum"] {
        let mut corrupt = value.clone();
        corrupt[field] = "wrong".into();
        assert!(storage::decode(&serde_json::to_vec(&corrupt).unwrap()).is_err());
    }
    assert!(storage::decode(&bytes[..bytes.len() - 1]).is_err());
    let mut corrupt = value.clone();
    corrupt["payload"]["characters"][0]["food"] = 99.into();
    assert!(storage::decode(&serde_json::to_vec(&corrupt).unwrap()).is_err());
    let mut state = sim.checkpoint();
    state.state.consumed = u64::MAX;
    assert!(Settlement::restore(state).is_err());
    let mut state = sim.checkpoint();
    state.characters[0].place = 99;
    assert!(Settlement::restore(state).is_err());
    let mut state = sim.checkpoint();
    state.characters[0].hunger = -1;
    assert!(Settlement::restore(state).is_err());
    let mut active = world(8, 42, 32);
    active.advance(5).unwrap();
    let mut corrupt = active.checkpoint();
    corrupt.state.next_commitment += 1;
    assert!(Settlement::restore(corrupt).is_err());
    let mut corrupt = active.checkpoint();
    let now = corrupt.state.tick;
    corrupt.state.commitments.values_mut().next().unwrap().due = now;
    assert!(Settlement::restore(corrupt).is_err());
}

#[test]
fn single_actor_and_seed_variation_work_without_pair_assumptions() {
    let mut a = world(1, 1, 64);
    a.advance(1000).unwrap();
    assert!(a.complete());
    validate(&a.checkpoint()).unwrap();
    let mut b = world(1, 2, 64);
    b.advance(64).unwrap();
    assert_ne!(a.checkpoint(), b.checkpoint());
}
