use world_of_individuals::{
    experiment006, experiment007 as e,
    foresight::{Channel, Sensor},
    inquiry::{self, Action},
    intentional::Profile,
    model::{Interpretation, Outcome},
    provenance as p,
};
fn reports(t: &e::Trial) -> Vec<&p::Receipt> {
    t.final_state
        .provenance
        .receipts
        .iter()
        .filter(|r| r.listener == 0)
        .collect()
}
fn sensor(inverted: bool, quality: i32) -> Sensor {
    Sensor {
        reliability: quality,
        effort: 2,
        channel: if inverted {
            Channel::InvertedFixture
        } else {
            Channel::AccurateFixture
        },
        second: None,
    }
}
#[test]
fn independent_known_hidden_revelation_and_conflict_have_distinct_local_consequences() {
    let independent = e::trial(42, "independent");
    let relay = e::trial(42, "relay");
    let hidden = e::trial(42, "hidden");
    let revelation = e::trial(42, "revelation");
    let conflict = e::trial(42, "conflict");
    let i = reports(&independent);
    let r = reports(&relay);
    let h = reports(&hidden);
    let v = reports(&revelation);
    let f = reports(&conflict);
    assert_ne!(i[0].actual_root, i[1].actual_root);
    assert!(i[1].evaluation.independent);
    assert_eq!(
        (i[1].evaluation.after, i[1].evaluation.incremental_value),
        (75, 30)
    );
    assert_eq!(r[0].actual_root, r[1].actual_root);
    assert!(r[1].evaluation.new_speaker);
    assert!(!r[1].evaluation.independent);
    assert_eq!(
        (r[1].evaluation.after, r[1].evaluation.incremental_value),
        (50, 0)
    );
    assert_eq!(h[0].actual_root, h[1].actual_root);
    assert!(h[1].knowledge.known.is_none());
    assert_eq!(h[1].evaluation.after, 75);
    assert_eq!(v[2].evaluation.kind, p::ValueKind::Revelation);
    assert_eq!((v[2].evaluation.before, v[2].evaluation.after), (75, 50));
    assert!(f[1].evaluation.independent);
    assert_eq!(f[1].evaluation.kind, p::ValueKind::Conflict);
    assert_eq!(f[1].evaluation.after, 0);
    assert!(
        revelation.final_state.base.base.concerns.items[&0][0]
            .status
            .active()
    );
    let cognition = &revelation.final_state.base.base.cognition;
    assert_eq!(
        cognition.information[1]
            .revision
            .as_ref()
            .unwrap()
            .after
            .interpretation,
        Interpretation::PossibleSelfProtection
    );
    assert_ne!(
        cognition.information[2]
            .revision
            .as_ref()
            .unwrap()
            .after
            .interpretation,
        Interpretation::PossibleSelfProtection
    );
    assert_eq!(
        cognition.information[0].trust_before,
        cognition.information[2].trust_after
    );
    // Attribution availability changes public receipt content, not the causal world.
    assert_eq!(
        relay.final_state.base.base.events,
        hidden.final_state.base.base.events
    );
    assert_eq!(
        relay.final_state.provenance.roots,
        hidden.final_state.provenance.roots
    );
    let causal = |t: &e::Trial| {
        t.final_state
            .provenance
            .receipts
            .iter()
            .map(|r| {
                (
                    r.event,
                    r.communicator,
                    r.listener,
                    r.actual_root,
                    r.parent,
                    r.knowledge.claim,
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(causal(&relay), causal(&hidden));
}
#[test]
fn three_speakers_and_ablations_do_not_confuse_communicators_with_lineage() {
    let multi = e::trial(42, "three_speakers");
    let r = reports(&multi);
    assert_eq!(r.len(), 3);
    assert!(
        r.iter()
            .all(|receipt| receipt.actual_root == r[0].actual_root)
    );
    assert_eq!(r[2].evaluation.after, 50);
    assert_eq!(r[2].evaluation.incremental_value, 0);
    let wrong = e::trial(42, "independent_wrong");
    let wrong_reports = reports(&wrong);
    assert_ne!(wrong_reports[0].actual_root, wrong_reports[1].actual_root);
    assert_eq!(wrong_reports[1].evaluation.after, -75);
    assert!(
        !wrong.final_state.base.base.concerns.items[&0][0]
            .status
            .active()
    );
    let disabled = e::trial(42, "provenance_disabled");
    assert_eq!(reports(&disabled)[1].evaluation.after, 75);
    for (name, expected) in [
        ("speaker_disabled_independent", 75),
        ("speaker_disabled_relay", 50),
    ] {
        assert_eq!(
            reports(&e::trial(42, name))
                .last()
                .unwrap()
                .evaluation
                .after,
            expected
        );
    }
}
#[test]
fn credibility_is_learned_from_local_comparisons_and_independence_changes_selection() {
    let trial = e::trial(42, "credibility_independence");
    let state = &trial.final_state.provenance;
    assert!(
        state
            .receipts
            .iter()
            .any(|r| r.credibility_changes == vec![(2, 0, 16), (3, 0, -24)])
    );
    let q = state
        .queries
        .iter()
        .rev()
        .find(|q| q.decision.input.base.owner == 0)
        .unwrap();
    assert!(matches!(
        q.decision.decision.selected,
        Action::Ask { source: 3, .. }
    ));
    let high = q
        .decision
        .factors
        .iter()
        .find(|f| {
            matches!(
                f.action,
                Action::Ask {
                    concern: 1,
                    source: 2,
                    strategy: inquiry::Strategy::Direct
                }
            )
        })
        .unwrap();
    let low = q
        .decision
        .factors
        .iter()
        .find(|f| {
            matches!(
                f.action,
                Action::Ask {
                    concern: 1,
                    source: 3,
                    strategy: inquiry::Strategy::Direct
                }
            )
        })
        .unwrap();
    assert_eq!(
        (high.credibility, high.independence, high.expected),
        (16, 0, 0)
    );
    assert_eq!(
        (low.credibility, low.independence, low.expected),
        (-24, 100, 53)
    );
    assert_eq!(reports(&trial).last().unwrap().evaluation.after, 69);
}
#[test]
fn alternative_source_decision_cannot_see_its_private_acquisition() {
    let mut inputs = Vec::new();
    for (name, expected_value, support) in [
        ("alternative_independent", 30, 64),
        ("alternative_relay", 0, 40),
        ("alternative_empty", 0, 40),
    ] {
        let t = e::trial(42, name);
        let q = t
            .final_state
            .provenance
            .queries
            .iter()
            .rev()
            .find(|q| q.decision.input.base.owner == 0)
            .unwrap();
        assert!(matches!(
            q.decision.decision.selected,
            Action::Ask { source: 3, .. }
        ));
        assert_eq!(q.value, expected_value);
        inputs.push(q.decision.input.clone());
        assert_eq!(reports(&t).last().unwrap().evaluation.after, support);
        let serialized = serde_json::to_string(&q.decision.input).unwrap();
        for private in [
            "actual_root",
            "parent",
            "sensor",
            "seed",
            "depth",
            "channel",
        ] {
            assert!(!serialized.contains(private), "{private}");
        }
    }
    assert_eq!(inputs[0], inputs[1]);
    assert_eq!(inputs[0], inputs[2]);
}
#[test]
fn roots_are_acquired_voluntarily_before_sampling_and_cannot_be_rewritten() {
    let mut sim = e::setup(42);
    let event = e::event(&sim);
    let original = sim.events();
    sim.set_communication_profile(
        1,
        Profile {
            privacy: 100,
            ..Default::default()
        },
    )
    .unwrap();
    sim.inspect_refusal(event, &[(2, sensor(false, 50))])
        .unwrap();
    assert!(sim.provenance().unwrap().roots.is_empty());
    sim.set_communication_profile(1, Profile::default())
        .unwrap();
    sim.inspect_refusal(event, &[(2, sensor(false, 50)), (3, sensor(true, 50))])
        .unwrap();
    assert_eq!(sim.provenance().unwrap().roots.len(), 2);
    let before = sim.provenance().unwrap();
    assert!(
        sim.inspect_refusal(event, &[(2, sensor(true, 50))])
            .is_err()
    );
    assert_eq!(before, sim.provenance().unwrap());
    assert_eq!(sim.events(), original);
    assert_eq!(sim.provenance_exchange(4, 0, event, true).unwrap(), None);
    sim.set_communication_profile(
        2,
        Profile {
            privacy: 100,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(sim.provenance_exchange(2, 0, event, true).unwrap(), None);
    let mut bad = sensor(false, 50);
    bad.reliability = 91;
    assert!(sim.inspect_refusal(event, &[(4, bad)]).is_err());
    assert!(sim.inspect_refusal(999, &[(4, sensor(false, 50))]).is_err());
    assert!(
        sim.inspect_refusal(event, &[(4, sensor(false, 50)), (4, sensor(false, 50))])
            .is_err()
    );
    assert!(sim.provenance_exchange(2, 2, event, true).is_err());
    assert!(sim.provenance_exchange(999, 0, event, true).is_err());
}
#[test]
fn policies_receive_no_hidden_sensor_or_partner_provenance_and_invalid_choices_do_not_learn() {
    fn bad_exchange(input: &p::ExchangeInput) -> p::ExchangeDecision {
        let mut d = p::exchange(input);
        d.input.acquired.as_mut().unwrap().known = Some(999);
        d
    }
    fn bad_inquiry(input: &p::Input) -> p::Decision {
        let mut d = p::policy(input);
        d.decision.selected = Action::Ask {
            concern: 999,
            source: 999,
            strategy: inquiry::Strategy::Direct,
        };
        d
    }
    let mut a = e::setup(42);
    let mut b = e::setup(42);
    let event = e::event(&a);
    a.inspect_refusal(event, &[(2, sensor(false, 50))]).unwrap();
    b.inspect_refusal(event, &[(2, sensor(true, 50))]).unwrap();
    let ia = &a.provenance().unwrap().windows[0].decisions[1].input;
    let ib = &b.provenance().unwrap().windows[0].decisions[1].input;
    assert_eq!(ia, ib);
    a.set_provenance_exchange_policy(bad_exchange).unwrap();
    assert!(a.provenance_exchange(2, 0, event, true).unwrap().is_none());
    assert!(!a.provenance().unwrap().exchanges.last().unwrap().valid);
    assert!(a.cognition().information.is_empty());
    a.set_provenance_policy(bad_inquiry).unwrap();
    let cells = a.inquiry().unwrap().cells;
    a.inquiry_meeting(&[0, 2]).unwrap();
    assert_eq!(cells, a.inquiry().unwrap().cells);
    assert!(!a.provenance().unwrap().queries[0].valid);
}
#[test]
fn relay_limits_and_parent_receipts_are_checked_without_recovering_evicted_source_knowledge() {
    let mut sim = e::setup(42);
    let event = e::event(&sim);
    sim.inspect_refusal(event, &[(2, sensor(false, 50))])
        .unwrap();
    for (source, listener) in [(2, 3), (3, 4), (4, 0)] {
        assert!(
            sim.provenance_exchange(source, listener, event, true)
                .unwrap()
                .is_some()
        );
    }
    assert!(
        sim.provenance_exchange(0, 2, event, true)
            .unwrap()
            .is_none()
    );
    assert!(!sim.provenance().unwrap().exchanges.last().unwrap().valid);
    for r in sim.provenance().unwrap().receipts {
        if let Some(parent) = r.parent {
            assert!(parent < r.id);
        }
        assert!(r.depth <= p::MAX_DEPTH);
    }
}
#[test]
fn forgetting_is_fifo_and_can_make_an_old_lineage_novel_again() {
    let mut sim = e::setup(42);
    let event = e::event(&sim);
    sim.inspect_refusal(event, &[(2, sensor(false, 50)), (3, sensor(false, 50))])
        .unwrap();
    sim.provenance_exchange(2, 0, event, true).unwrap();
    let archived = sim.provenance().unwrap().receipts[2].clone();
    for _ in 0..32 {
        sim.provenance_exchange(3, 0, event, true).unwrap();
    }
    let p = sim.provenance().unwrap();
    assert_eq!(p.knowledge[&0].len(), 32);
    assert_eq!(p.evictions[0].receipt, 2);
    assert_eq!(p.receipts[2], archived);
    sim.provenance_exchange(2, 0, event, true).unwrap();
    assert_eq!(
        sim.provenance()
            .unwrap()
            .receipts
            .last()
            .unwrap()
            .evaluation
            .kind,
        p::ValueKind::Corroboration
    );
    assert_eq!(
        sim.provenance()
            .unwrap()
            .receipts
            .last()
            .unwrap()
            .evaluation
            .after,
        75
    );
}
#[test]
fn all_seed_trials_replay_and_audit_references_and_accounting_are_consistent() {
    for seed in 0..128 {
        let trials = e::suite(seed);
        assert_eq!(trials, e::suite(seed));
        for t in trials {
            let state = &t.final_state.provenance;
            for r in &state.receipts {
                let root = &state.roots[r.actual_root as usize];
                assert_eq!(root.event, r.event);
                assert_eq!(root.claim, r.knowledge.claim);
                assert_eq!(r.food_before, r.food_after);
                if let Some(parent) = r.parent {
                    let prior = &state.receipts[parent as usize];
                    assert_eq!(prior.actual_root, r.actual_root);
                    assert_eq!(prior.listener, r.communicator);
                    assert_eq!(r.depth, prior.depth + 1);
                }
                if let Some(id) = r.cognition_receipt {
                    let c = &t.final_state.base.base.cognition.information[id as usize];
                    assert_eq!(c.event, r.event);
                    assert_eq!(c.after.support, r.evaluation.after);
                }
            }
            for q in &state.queries {
                let audit = &t.final_state.base.inquiry.records[q.inquiry_record as usize];
                assert_eq!(audit.realized_value, q.value);
                assert_eq!(audit.time_spent, q.time_spent);
                assert_eq!(audit.food_before, audit.food_after);
                if let Some(cell) = &q.after {
                    assert_eq!(cell.last_record, q.inquiry_record);
                }
            }
            assert!(
                t.final_state
                    .base
                    .base
                    .events
                    .iter()
                    .any(|e| e.outcome == Some(Outcome::Refusal))
            );
        }
    }
}
#[test]
fn earlier_modes_reproduce_archives_and_population_keeps_original_response_mechanism() {
    let archived: serde_json::Value =
        serde_json::from_str(include_str!("../experiments/006/seed-42.json")).unwrap();
    assert_eq!(
        serde_json::to_value(experiment006::suite(42)).unwrap(),
        archived
    );
    let old = experiment006::population(42, 1000);
    let new = e::population(42, 1000);
    assert_eq!(new, e::population(42, 1000));
    assert_eq!(old.base.events, new.base.base.events);
    assert_eq!(old.base.agents, new.base.base.agents);
    assert_eq!(old.base.cognition, new.base.base.cognition);
    assert!(new.provenance.roots.is_empty());
    assert!(new.provenance.receipts.is_empty());
    assert_eq!(new.provenance.queries.len(), 3000);
}

#[test]
fn all_new_local_collections_are_bounded_and_evicted_provider_cannot_use_archive() {
    fn run() -> p::Provenance {
        let mut sim = e::setup(42);
        let original = e::event(&sim);
        for turn in 0..40 {
            if turn > 0 {
                world_of_individuals::experiment005::refusal_conditions(&mut sim, 60);
                sim.add_scene([0, 1], 1, 6).unwrap();
                sim.run();
                sim.set_communication_profile(1, Profile::default())
                    .unwrap();
            }
            let event = e::event(&sim);
            sim.inspect_refusal(
                event,
                &[
                    (2, sensor(false, 50)),
                    (3, sensor(true, 50)),
                    (4, sensor(false, 80)),
                ],
            )
            .unwrap();
            for source in [2, 3, 4] {
                sim.provenance_exchange(source, 0, event, true).unwrap();
            }
        }
        let before = sim.provenance().unwrap().receipts.len();
        assert!(
            sim.provenance_exchange(2, 0, original, true)
                .unwrap()
                .is_none()
        );
        assert_eq!(before, sim.provenance().unwrap().receipts.len());
        sim.provenance().unwrap()
    }
    let state = run();
    assert_eq!(state, run());
    assert!(state.knowledge.values().all(|v| v.len() <= p::CAPACITY));
    assert_eq!(state.knowledge[&2].len(), 32);
    assert_eq!(state.hints[&0].len(), 32);
    assert_eq!(state.comparisons[&0].len(), 32);
    for collection in ["knowledge", "hints", "comparisons"] {
        assert!(state.evictions.iter().any(|e| e.collection == collection));
    }
    assert_eq!(
        state.knowledge[&2].front().unwrap().event,
        state.roots[24].event
    );
}

#[test]
fn repeated_strong_proof_cannot_bootstrap_credibility_or_independence() {
    let mut sim = e::setup(42);
    let event = e::event(&sim);
    sim.inspect_refusal(event, &[(2, sensor(false, 50)), (3, sensor(false, 80))])
        .unwrap();
    sim.provenance_exchange(2, 0, event, true).unwrap();
    sim.provenance_exchange(3, 0, event, true).unwrap();
    let before = sim.intentional().unwrap().credibility;
    for _ in 0..8 {
        sim.provenance_exchange(3, 0, event, true).unwrap();
    }
    assert_eq!(before, sim.intentional().unwrap().credibility);
    assert_eq!(
        sim.provenance()
            .unwrap()
            .receipts
            .last()
            .unwrap()
            .evaluation
            .incremental_value,
        0
    );
}
