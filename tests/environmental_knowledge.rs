use world_of_individuals::{
    acquisition as a, environmental_knowledge as k, experiment020 as old, experiment021 as f,
};

fn obs(id: u32, at: u64, stock: u32) -> a::Observation {
    a::Observation {
        id: a::CacheId(id),
        observed_at: at,
        stock,
        accessible: true,
        duration: 4,
    }
}
fn input(t: &f::Trial) -> k::Input {
    let w = &t.periods[0].steps[0];
    let d = w.choices[0].as_ref().unwrap();
    let base = d
        .acquiring_base
        .as_ref()
        .or(w.activity.decisions[0].as_ref())
        .unwrap();
    k::Input {
        legacy: base.contact.input.clone(),
        retained: d.retained.clone(),
        duration: d.duration,
        inquiry_lane_free: d.inquiry_lane_free,
    }
}
#[test]
fn every_primary_fresh_reference_preserves_020_physics_and_legacy_execution() {
    for case in f::CASES {
        for (policy, mode) in [
            (a::Policy::Need, old::Mode::Need),
            (a::Policy::Acquire, old::Mode::Acquire),
        ] {
            let t = f::trial(42, policy, k::Mode::FreshReference, case, f::config(case));
            let reference = old::trial(42, mode, case, old::config(case));
            assert_eq!(t.initial, reference.initial);
            assert_eq!(t.points, reference.points);
            assert_eq!(t.records, reference.records);
            assert_eq!(t.needs, reference.needs);
            assert_eq!(t.clock, reference.clock);
            assert_eq!(t.final_state, reference.final_state);
            for (p, q) in t.periods.iter().zip(reference.periods) {
                assert_eq!(p.steps.len(), q.steps.len());
                for (x, y) in p.steps.iter().zip(q.steps) {
                    assert_eq!(x.activity, y.activity);
                }
            }
        }
    }
}
#[test]
fn unobserved_is_actual_legacy_ignored_execution_with_same_supply() {
    for case in f::CASES {
        let t = f::trial(
            42,
            a::Policy::Need,
            k::Mode::Unobserved,
            case,
            f::config(case),
        );
        let reference = old::trial(42, old::Mode::Ignored, case, old::config(case));
        assert!(t.memory_records.is_empty() && t.records.is_empty());
        assert_eq!(t.points, reference.points);
        assert_eq!(t.needs, reference.needs);
        assert_eq!(t.final_state, reference.final_state);
        for (p, q) in t.periods.iter().zip(reference.periods) {
            for (x, y) in p.steps.iter().zip(q.steps) {
                assert_eq!(x.activity, y.activity);
            }
        }
    }
}
#[test]
fn forgotten_quantity_is_not_restored_by_success_and_inventory_survives() {
    let mut store = k::Store::new(k::Mode::Expire12, 0, 8);
    store.prepare(0, 0, Some(obs(0, 0, 2)), 0);
    let (stock, food, outcome) = a::grant(2, 0);
    store.result(k::LocalResult {
        owner: 0,
        cache: a::CacheId(0),
        at: 13,
        outcome,
        physical: 0,
        duration: 4,
    });
    assert_eq!((stock, food), (1, 1));
    assert!(store.prepare(0, 13, None, 1).is_none());
    assert_eq!(
        store.records[0].after.as_ref().unwrap().observation.stock,
        2
    );
    assert!(!store.records.iter().any(|r| r.kind == k::Change::Taken));
    let redelivered = store.prepare(0, 16, Some(obs(0, 16, stock)), 2).unwrap();
    assert_eq!((redelivered.receipt, redelivered.observation.stock), (1, 1));
    assert_eq!(redelivered.observation.observed_at, 16);
    assert_eq!(food, 1); // expiry must not erase separately established material state.
}
#[test]
fn own_debit_never_rejuvenates_observation_or_updates_rivals() {
    let mut store = k::Store::new(k::Mode::Expire12, 0, 8);
    store.prepare(0, 0, Some(obs(0, 0, 2)), 0);
    store.prepare(1, 0, Some(obs(0, 0, 2)), 0);
    store.result(k::LocalResult {
        owner: 0,
        cache: a::CacheId(0),
        at: 4,
        outcome: a::Outcome::Granted,
        physical: 0,
        duration: 4,
    });
    let own = store.prepare(0, 5, None, 1).unwrap();
    let rival = store.prepare(1, 5, None, 1).unwrap();
    assert_eq!(own.upper_bound(), 1);
    assert_eq!(rival.upper_bound(), 2);
    assert_eq!(own.observation.observed_at, 0);
    assert!(store.prepare(0, 12, None, 2).is_none());
}
#[test]
fn empty_result_is_new_local_evidence_not_old_quantity_recovery() {
    let mut store = k::Store::new(k::Mode::Expire12, 0, 8);
    store.prepare(0, 0, Some(obs(0, 0, 2)), 0);
    store.result(k::LocalResult {
        owner: 0,
        cache: a::CacheId(0),
        at: 13,
        outcome: a::Outcome::Depleted,
        physical: 5,
        duration: 4,
    });
    let c = store.prepare(0, 13, None, 1).unwrap();
    assert_eq!(
        (c.receipt, c.observation.stock, c.observation.observed_at),
        (1, 0, 13)
    );
    assert_eq!(store.records.last().unwrap().physical, Some(5));
}
#[test]
fn one_slot_overwrites_and_actor_receipt_numbers_are_local() {
    assert_eq!(k::CAPACITY, 1);
    let mut store = k::Store::new(k::Mode::LastSeen, 0, 4);
    store.prepare(0, 0, Some(obs(0, 0, 1)), 0);
    let c = store.prepare(0, 4, Some(obs(1, 4, 3)), 1).unwrap();
    assert_eq!((c.observation.id, c.receipt), (a::CacheId(1), 1));
    assert!(store.prepare(1, 5, None, 2).is_none());
    let other = store.prepare(1, 8, Some(obs(0, 8, 1)), 3).unwrap();
    assert_eq!(other.receipt, 0);
    assert_eq!(store.finish(9).iter().filter(|s| s.is_some()).count(), 2);
}
#[test]
fn foreign_future_and_absent_records_never_enable_acquisition() {
    let t = f::trial(
        42,
        a::Policy::Need,
        k::Mode::FreshReference,
        "single_start",
        f::config("single_start"),
    );
    let i = input(&t);
    assert!(k::eligible(&i));
    let mut absent = i.clone();
    absent.retained = None;
    assert!(!k::eligible(&absent));
    let mut foreign = i.clone();
    foreign.retained.as_mut().unwrap().owner = 1;
    assert!(!k::eligible(&foreign));
    let mut future = i.clone();
    future.retained.as_mut().unwrap().observation.observed_at += 1;
    assert!(!k::eligible(&future));
    let keys = serde_json::to_value(i)
        .unwrap()
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(
        keys,
        vec!["duration", "inquiry_lane_free", "legacy", "retained"]
    );
}
#[test]
fn hidden_partner_circumstances_do_not_enter_opening_local_input() {
    let t = f::trial(
        42,
        a::Policy::Need,
        k::Mode::LastSeen,
        "private_helpful",
        f::config("private_helpful"),
    );
    let other = f::trial(
        42,
        a::Policy::Need,
        k::Mode::LastSeen,
        "private_cautious",
        f::config("private_cautious"),
    );
    assert_eq!(input(&t), input(&other));
    assert_eq!(
        k::policy(&input(&t), a::Policy::Need),
        k::policy(&input(&other), a::Policy::Need)
    );
}
#[test]
fn current_only_discards_while_last_seen_keeps_uncertain_positive_information() {
    let mut current = k::Store::new(k::Mode::CurrentOnly, 0, 8);
    let mut retained = k::Store::new(k::Mode::LastSeen, 0, 8);
    for s in [&mut current, &mut retained] {
        s.prepare(0, 0, Some(obs(0, 0, 1)), 0);
        s.after_decision(0, 0);
    }
    assert!(current.prepare(0, 4, None, 1).is_none());
    assert_eq!(retained.prepare(0, 4, None, 1).unwrap().upper_bound(), 1);
    let (stock, _, _) = a::grant(1, 0);
    let (_, _, outcome) = a::grant(stock, 0);
    assert_eq!(outcome, a::Outcome::Depleted);
}
