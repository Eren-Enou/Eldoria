use world_of_individuals::{
    assessment::{self as a, Method, Origin, Receipt},
    cognition::EvidenceKind,
    concerns::Status,
    experiment007 as old, experiment008 as e,
    foresight::{Channel, Sensor},
    model::Action,
};
fn result(t: &e::Trial) -> (Option<i32>, Option<Status>, i32, Action) {
    let p = t.points.last().unwrap();
    (p.support, p.concern, p.trust, p.probe.selected)
}
fn delivery_basis(t: &e::Trial) -> Vec<(u64, Origin, bool, i32)> {
    let mut basis = t.final_state.assessment.as_ref().unwrap().items[&0]
        .iter()
        .map(|i| (i.event, i.origin, i.claim, i.quality))
        .collect::<Vec<_>>();
    basis.sort();
    basis.dedup();
    basis
}

#[test]
fn shared_basis_removes_native_testimony_order_and_repeat_reversals() {
    for mode in [e::Mode::Max, e::Mode::Grouped] {
        for (forward, reverse, support) in [
            ("mixed_np", "mixed_pn", 50),
            (
                "corroboration_np",
                "corroboration_pn",
                if mode == e::Mode::Grouped { 75 } else { 50 },
            ),
            ("conflict_np", "conflict_pn", 0),
            ("claim_then_evidence", "evidence_then_claim", -40),
        ] {
            let p = e::trial(42, mode, forward);
            let n = e::trial(42, mode, reverse);
            assert_eq!(result(&p), result(&n));
            assert_eq!(delivery_basis(&p), delivery_basis(&n));
            assert_eq!(p.points[2].support, Some(support));
            assert_eq!(p.points[2].support, p.points[3].support);
            assert_eq!(n.points[2].support, n.points[3].support);
            assert_eq!(
                p.final_state.base.base.base.intentional.credibility,
                n.final_state.base.base.base.intentional.credibility
            );
        }
    }
    assert_ne!(
        result(&e::trial(42, e::Mode::Legacy, "mixed_np")),
        result(&e::trial(42, e::Mode::Legacy, "mixed_pn"))
    );
}

#[test]
fn same_acquisition_keeps_one_origin_across_both_delivery_adapters() {
    for mode in [e::Mode::Max, e::Mode::Grouped] {
        let baseline = e::trial(42, mode, "acquired_pp");
        for channel in ["acquired_nn", "acquired_np", "acquired_pn"] {
            let t = e::trial(42, mode, channel);
            assert_eq!(result(&t), result(&baseline));
            assert_eq!(delivery_basis(&t), delivery_basis(&baseline));
            assert_eq!(t.points[1].support, t.points[2].support);
            assert_eq!(t.points[2].support, Some(50));
            assert!(
                t.final_state.assessment.unwrap().items[&0]
                    .iter()
                    .all(|i| i.origin == Origin::Known(0))
            );
        }
    }
}

#[test]
fn known_independent_shared_unknown_and_revealed_dependence_are_distinct() {
    let independent = e::trial(42, e::Mode::Grouped, "corroboration_np");
    let known = e::trial(42, e::Mode::Grouped, "known_relay");
    let hidden = e::trial(42, e::Mode::Grouped, "hidden_relay");
    let reveal = e::trial(42, e::Mode::Grouped, "revelation");
    assert_eq!(result(&independent).0, Some(75));
    assert_eq!(result(&known).0, Some(50));
    assert_eq!(result(&hidden).0, Some(75));
    assert_eq!(result(&reveal).0, Some(50));
    assert_eq!(
        known.final_state.base.provenance.roots,
        hidden.final_state.base.provenance.roots
    );
    assert_eq!(
        known.final_state.base.base.base.events,
        hidden.final_state.base.base.base.events
    );
    assert!(
        !reveal
            .final_state
            .assessment
            .as_ref()
            .unwrap()
            .records
            .last()
            .unwrap()
            .revised_attribution
            .is_empty()
    );
    assert_eq!(
        result(&e::trial(42, e::Mode::Max, "corroboration_np")).0,
        Some(50)
    );
}

#[test]
fn confidence_can_be_coherently_wrong_and_new_correction_can_remain_ambiguous() {
    let wrong = e::trial(42, e::Mode::Grouped, "wrong");
    assert_eq!(result(&wrong).0, Some(-75));
    assert_eq!(result(&wrong).1, Some(Status::Resolved));
    let corrected = e::trial(42, e::Mode::Grouped, "correction");
    assert_eq!(
        corrected
            .points
            .iter()
            .map(|p| p.support)
            .collect::<Vec<_>>(),
        vec![None, Some(-40), Some(50), Some(55)]
    );
    assert_eq!(result(&corrected).1, Some(Status::Partial));
    assert_eq!(result(&corrected).3, Action::Leave);
    let simple = e::trial(42, e::Mode::Max, "correction");
    assert_eq!(result(&simple).0, Some(50));
}

#[test]
fn fifo_eviction_is_real_and_observer_native_maxima_do_not_restore_evidence() {
    let t = e::trial(42, e::Mode::Grouped, "eviction");
    assert_eq!(
        t.points.iter().map(|p| p.support).collect::<Vec<_>>(),
        vec![None, Some(90), Some(50), Some(-40), Some(50)]
    );
    let assessment = t.final_state.assessment.unwrap();
    assert_eq!(assessment.items[&0].len(), 32);
    assert_eq!(assessment.records[32].evicted, Some(Receipt::Native(0)));
    assert!(
        assessment.records[33]
            .retained
            .iter()
            .all(|r| *r != Receipt::Native(0))
    );
    assert_eq!(
        assessment.records[34].incoming.origin,
        Origin::NativeReading {
            speaker: 1,
            channel: 0
        }
    );
    let absent = e::trial(42, e::Mode::Grouped, "evicted_memory");
    assert_eq!(result(&absent).0, Some(90));
    assert_eq!(result(&absent).3, Action::Leave);
    assert!(
        absent
            .final_state
            .base
            .base
            .base
            .cognition
            .information
            .last()
            .unwrap()
            .revision
            .is_none()
    );
}

#[test]
fn enable_is_prospective_idempotent_and_does_not_resurrect_archived_proofs() {
    let mut sim = old::setup(42);
    let event = old::event(&sim);
    sim.communicate(
        1,
        0,
        event,
        EvidenceKind::Fallible {
            scarce: true,
            reliability: 90,
            source: 0,
        },
    )
    .unwrap();
    let before = sim.cognition();
    sim.enable_assessment(Method::Grouped).unwrap();
    assert!(sim.assessment().unwrap().items.is_empty());
    assert_eq!(before, sim.cognition());
    sim.enable_assessment(Method::Grouped).unwrap();
    assert!(sim.enable_assessment(Method::Max).is_err());
    sim.communicate(
        1,
        0,
        event,
        EvidenceKind::Fallible {
            scarce: false,
            reliability: 40,
            source: 1,
        },
    )
    .unwrap();
    assert_eq!(sim.cognition().beliefs[&0][0].support, -40);
    assert_eq!(sim.assessment().unwrap().items[&0].len(), 1);
}

#[test]
fn invalid_native_payload_and_unavailable_acquisition_do_not_mutate_assessment() {
    let mut sim = old::setup(42);
    let event = old::event(&sim);
    assert!(sim.native_acquired_exchange(2, 0, event, true).is_err());
    sim.enable_assessment(Method::Grouped).unwrap();
    let before = sim.assessment().unwrap();
    assert!(
        sim.communicate(
            1,
            0,
            event,
            EvidenceKind::Fallible {
                scarce: true,
                reliability: 91,
                source: 0
            }
        )
        .is_err()
    );
    assert_eq!(before, sim.assessment().unwrap());
    assert!(sim.native_acquired_exchange(2, 4, event, true).is_err());
    assert!(sim.native_acquired_exchange(2, 0, 999, true).is_err());
    assert_eq!(
        sim.native_acquired_exchange(4, 0, event, true).unwrap(),
        None
    );
    assert_eq!(before, sim.assessment().unwrap());
    sim.communicate(
        1,
        0,
        event,
        EvidenceKind::Fallible {
            scarce: true,
            reliability: 90,
            source: 0,
        },
    )
    .unwrap();
    let before = sim.assessment().unwrap();
    assert!(
        sim.communicate(
            1,
            0,
            event,
            EvidenceKind::Fallible {
                scarce: false,
                reliability: 90,
                source: 0
            }
        )
        .is_err()
    );
    assert_eq!(before, sim.assessment().unwrap());
}

#[test]
fn local_assessment_does_not_receive_hidden_roots_truth_or_private_sensors() {
    let known = e::trial(42, e::Mode::Grouped, "known_relay");
    let hidden = e::trial(42, e::Mode::Grouped, "hidden_relay");
    let item = hidden.final_state.assessment.as_ref().unwrap().records[1]
        .incoming
        .clone();
    assert_eq!(item.origin, Origin::Unknown(3));
    let text = serde_json::to_string(&item).unwrap();
    for private in ["actual_root", "parent", "sensor", "seed", "depth", "truth"] {
        assert!(!text.contains(private));
    }
    assert_eq!(
        known.final_state.base.provenance.receipts[3].actual_root,
        hidden.final_state.base.provenance.receipts[3].actual_root
    );
    let mut corrupted = hidden.final_state.clone();
    corrupted.assessment.as_mut().unwrap().records[1]
        .incoming
        .origin = Origin::Known(0);
    assert!(corrupted.validate().is_err());
    let mut corrupted = hidden.final_state;
    corrupted.assessment.as_mut().unwrap().records[1].support = 99;
    assert!(corrupted.validate().is_err());
}

#[test]
fn all_distinguishing_seeded_scenarios_replay_with_valid_causal_references() {
    for seed in [0, 42, 127, u64::MAX] {
        let suite = e::suite(seed);
        assert_eq!(suite, e::suite(seed));
        for t in suite {
            t.final_state.validate().unwrap();
            for receipt in &t.final_state.base.provenance.receipts {
                assert_eq!(receipt.food_before, receipt.food_after);
                if let Some(cognitive) = receipt.cognition_receipt {
                    assert_eq!(
                        receipt.evaluation.after,
                        t.final_state.base.base.base.cognition.information[cognitive as usize]
                            .after
                            .support
                    );
                }
            }
            if let Some(a) = t.final_state.assessment {
                assert!(a.items.values().all(|v| v.len() <= 32));
                assert!(a.records.iter().all(|r| r.support.abs() <= 100));
            }
        }
    }
}

#[test]
fn voluntary_inquiry_consumes_actual_unified_receipt_without_forcing_resolution() {
    let t = e::trial(42, e::Mode::Grouped, "inquiry");
    assert_eq!(result(&t).0, Some(55));
    assert_eq!(result(&t).1, Some(Status::Partial));
    let q = t
        .final_state
        .base
        .provenance
        .queries
        .iter()
        .find(|q| q.owner == 0)
        .unwrap();
    let native = &t.final_state.base.base.inquiry.records[q.inquiry.0 as usize];
    assert!(matches!(
        native.decision.selected,
        world_of_individuals::inquiry::Action::Ask { source: 3, .. }
    ));
    assert_eq!(native.food_before, native.food_after);
    assert_eq!(
        t.final_state
            .assessment
            .unwrap()
            .records
            .last()
            .unwrap()
            .support,
        55
    );
}

#[test]
fn pure_rules_only_use_supplied_local_items_and_repeats_are_idempotent() {
    let positive = a::Item {
        receipt: Receipt::Native(0),
        event: 1,
        communicator: 1,
        message: None,
        origin: Origin::NativeReading {
            speaker: 1,
            channel: 0,
        },
        claim: true,
        quality: 90,
    };
    let negative = a::Item {
        receipt: Receipt::Provenance(0),
        event: 1,
        communicator: 2,
        message: Some(1),
        origin: Origin::Known(0),
        claim: false,
        quality: 40,
    };
    for rule in [a::strongest, a::grouped] {
        assert_eq!(rule(&[positive.clone(), negative.clone()], 1), 50);
        assert_eq!(
            rule(
                &[
                    negative.clone(),
                    positive.clone(),
                    negative.clone(),
                    positive.clone()
                ],
                1
            ),
            50
        );
    }
    let claim = a::Item {
        origin: Origin::Claim(1),
        quality: 50,
        ..positive
    };
    let zero = a::Item {
        quality: 0,
        ..negative
    };
    assert_eq!(a::grouped(&[claim, zero], 1), 50);
}

#[test]
fn population_is_bounded_reproducible_and_native_only_outputs_are_preserved() {
    for count in [0, 1, 11, 100] {
        let legacy = e::population(42, count, e::Mode::Legacy);
        let grouped = e::population(42, count, e::Mode::Grouped);
        assert_eq!(grouped, e::population(42, count, e::Mode::Grouped));
        assert_eq!(legacy.base.base.base.events, grouped.base.base.base.events);
        assert_eq!(legacy.base.base.base.agents, grouped.base.base.base.agents);
        assert_eq!(
            legacy.base.base.base.cognition,
            grouped.base.base.base.cognition
        );
        assert!(
            grouped
                .assessment
                .unwrap()
                .items
                .values()
                .all(|v| v.len() <= 32)
        );
    }
}

#[test]
fn new_adapter_samples_nothing_and_keeps_policy_input_and_weights_matched() {
    let mut sim = old::setup(42);
    sim.enable_assessment(Method::Grouped).unwrap();
    let event = old::event(&sim);
    sim.inspect_refusal(
        event,
        &[(
            2,
            Sensor {
                reliability: 50,
                channel: Channel::AccurateFixture,
                ..Default::default()
            },
        )],
    )
    .unwrap();
    sim.provenance_exchange(2, 0, event, true).unwrap();
    let first = sim
        .provenance()
        .unwrap()
        .exchanges
        .last()
        .unwrap()
        .decision
        .clone();
    sim.native_acquired_exchange(2, 0, event, true).unwrap();
    let second = sim
        .provenance()
        .unwrap()
        .exchanges
        .last()
        .unwrap()
        .decision
        .clone();
    assert_eq!(first, second);
    assert_eq!(sim.provenance().unwrap().roots.len(), 1);
    assert_eq!(sim.assessment().unwrap().records[1].support, 50);
    sim.validate_history_indexes().unwrap();
    e::snapshot(&sim).validate().unwrap();
}
