use world_of_individuals::{
    assessment::{Item, Origin, Receipt},
    cognition::EvidenceKind,
    experiment009 as e,
    model::Action,
    retention::{self as r, Input, Method},
};
fn item(id: u64, quality: i32) -> Item {
    Item {
        receipt: Receipt::Native(id),
        event: id,
        communicator: 1,
        message: None,
        origin: Origin::NativeReading {
            speaker: 1,
            channel: 0,
        },
        claim: true,
        quality,
    }
}
#[test]
fn equal_capacity_ties_redundancy_and_conflict_use_only_retained_local_inputs() {
    let mut input = Input {
        owner: 0,
        items: (0..33).map(|id| item(id, 50)).collect(),
        concerns: vec![],
        method: Method::Salient,
    };
    assert_eq!(r::policy(&input).evict, Some(0));
    input.items[32].event = 0;
    let decision = r::policy(&input);
    assert_eq!(decision.candidates[0].score, 0);
    assert_eq!(decision.candidates[32].score, 50);
    input.items[32].claim = false;
    let decision = r::policy(&input);
    assert_eq!(decision.candidates[0].score, 50);
    assert_eq!(decision.candidates[32].score, 50);
    input.items[32].origin = Origin::Unknown(2);
    input.items[0].origin = Origin::Unknown(3);
    input.items[32].claim = true;
    assert!(r::policy(&input).candidates.iter().all(|c| c.score == 50));
}
#[test]
fn held_out_inputs_are_matched_and_replay_at_extreme_seeds() {
    for seed in [42, u64::MAX] {
        let suite = e::suite(seed, true);
        assert_eq!(suite, e::suite(seed, true));
        for group in suite.as_chunks::<3>().0 {
            assert!(
                group
                    .iter()
                    .all(|t| t.initial.base == group[0].initial.base)
            );
            let deliveries = |t: &e::Trial| {
                t.final_state
                    .base
                    .assessment
                    .as_ref()
                    .unwrap()
                    .records
                    .iter()
                    .map(|r| (r.owner, r.incoming.clone()))
                    .collect::<Vec<_>>()
            };
            for t in group {
                assert_eq!(deliveries(t), deliveries(&group[0]));
                assert!(
                    t.final_state
                        .base
                        .assessment
                        .as_ref()
                        .unwrap()
                        .items
                        .values()
                        .all(|v| v.len() <= 32)
                );
                for choice in &t.final_state.retention.records {
                    assert_eq!(choice.decision, r::policy(&choice.decision.input));
                    assert!(
                        choice
                            .decision
                            .input
                            .concerns
                            .iter()
                            .all(|c| c.owner == choice.decision.input.owner)
                    );
                }
            }
        }
    }
}
#[test]
fn selective_retention_preserves_distinctions_but_fifo_can_make_the_better_action() {
    let final_point = |method, scenario| {
        e::trial(42, method, scenario, e::Config::default())
            .points
            .last()
            .unwrap()
            .clone()
    };
    assert_eq!(
        final_point(Method::Fifo, "unresolved_concern").basis_support,
        0
    );
    assert_eq!(
        final_point(Method::Quality, "unresolved_concern").basis_support,
        0
    );
    assert_eq!(
        final_point(Method::Salient, "unresolved_concern").basis_support,
        40
    );
    let conflict = final_point(Method::Salient, "conflict");
    assert!(conflict.positive && conflict.negative);
    assert_eq!(conflict.basis_support, 0);
    let wrong = e::trial(42, Method::Salient, "salient_wrong", e::Config::default());
    assert_eq!(
        wrong
            .points
            .iter()
            .find(|p| p.label == "after pressure")
            .unwrap()
            .basis_support,
        -80
    );
    assert_eq!(wrong.points.last().unwrap().basis_support, 10);
    assert_eq!(
        final_point(Method::Salient, "relevance_expires").basis_support,
        0
    );
    assert_eq!(
        final_point(Method::Fifo, "minor_later_useful").action,
        Action::Offer
    );
    assert_eq!(
        final_point(Method::Salient, "minor_later_useful").action,
        Action::Leave
    );
}
#[test]
fn missing_tampered_and_reordered_audit_decisions_are_rejected() {
    let t = e::trial(
        42,
        Method::Salient,
        "later_attribution",
        e::Config::default(),
    );
    let mut state = t.final_state.clone();
    state.retention.records.remove(0);
    assert!(state.validate().is_err());
    let mut state = t.final_state.clone();
    state.retention.records.swap(0, 1);
    assert!(state.validate().is_err());
    let mut state = t.final_state.clone();
    state.retention.records[0].decision.input.items[0].quality = 100;
    assert!(state.validate().is_err());
    let mut state = t.final_state;
    state
        .retention
        .records
        .last_mut()
        .unwrap()
        .event_support_after
        .clear();
    assert!(state.validate().is_err());
}
#[test]
fn fifo_mode_replays_the_frozen008_validator_and_redelivery_is_new() {
    for scenario in e::SCENARIOS {
        let t = e::trial(42, Method::Fifo, scenario, e::Config::default());
        t.final_state.base.validate().unwrap();
    }
    for method in [Method::Fifo, Method::Quality, Method::Salient] {
        let t = e::trial(42, method, "redelivery", e::Config::default());
        let forgotten = t.points.iter().find(|p| p.label == "forgotten").unwrap();
        assert_eq!(forgotten.target_items, 0);
        assert_eq!(t.points.last().unwrap().basis_support, 90);
        assert!(
            t.final_state
                .base
                .assessment
                .unwrap()
                .records
                .iter()
                .any(|r| r.incoming.event == t.target && r.incoming.quality == 90)
        );
    }
}
fn invalid(input: &Input) -> r::Decision {
    let mut d = r::policy(input);
    d.input.owner += 1;
    d
}
#[test]
fn invalid_native_retention_decision_is_rejected_before_cognition_or_clock_changes() {
    let (mut sim, events) = e::setup(42, Method::Salient, 80, false);
    sim.set_retention_policy(invalid).unwrap();
    let before = e::snapshot(&sim);
    assert!(
        sim.communicate(
            1,
            0,
            events[0],
            EvidenceKind::Fallible {
                scarce: true,
                reliability: 80,
                source: 0
            }
        )
        .is_err()
    );
    assert_eq!(before, e::snapshot(&sim));
    sim.set_retention_policy(r::policy).unwrap();
    sim.communicate(
        1,
        0,
        events[0],
        EvidenceKind::Fallible {
            scarce: true,
            reliability: 80,
            source: 0,
        },
    )
    .unwrap();
    e::snapshot(&sim).validate().unwrap();
}
#[test]
fn saturated_population_is_bounded_reproducible_and_preserves_receipt_accounting() {
    for count in [100, 1000] {
        let fifo = e::population(42, count, Method::Fifo);
        let selected = e::population(42, count, Method::Salient);
        assert_eq!(selected, e::population(42, count, Method::Salient));
        let f = fifo.base.assessment.unwrap();
        let s = selected.base.assessment.as_ref().unwrap();
        assert_eq!(f.records.len(), s.records.len());
        assert_eq!(f.records.len(), count as usize / 2 * 36);
        assert!(s.items.values().all(|items| items.len() == 32));
        assert!(
            s.items
                .values()
                .all(|items| items.iter().any(|i| i.quality == 80))
        );
        assert!(
            f.items
                .values()
                .all(|items| items.iter().all(|i| i.quality == 40))
        );
    }
}
#[test]
fn enabling_retention_preserves_existing_local_basis_and_cannot_import_history() {
    let mut sim = world_of_individuals::experiment007::setup(42);
    assert!(sim.enable_retention(Method::Salient).is_err());
    sim.enable_assessment(world_of_individuals::assessment::Method::Grouped)
        .unwrap();
    let event = world_of_individuals::experiment007::event(&sim);
    sim.communicate(
        1,
        0,
        event,
        EvidenceKind::Fallible {
            scarce: true,
            reliability: 40,
            source: 0,
        },
    )
    .unwrap();
    let before = sim.assessment().unwrap();
    sim.enable_retention(Method::Salient).unwrap();
    assert_eq!(before, sim.assessment().unwrap());
    assert_eq!(sim.retention().unwrap().first_assessment, 1);
    assert!(sim.retention().unwrap().records.is_empty());
    sim.enable_retention(Method::Salient).unwrap();
    assert!(sim.enable_retention(Method::Quality).is_err());
    sim.communicate(
        1,
        0,
        event,
        EvidenceKind::Fallible {
            scarce: false,
            reliability: 60,
            source: 1,
        },
    )
    .unwrap();
    e::snapshot(&sim).validate().unwrap();
}

#[test]
fn invalid_replacement_inside_paid_provenance_delivery_is_audited_and_terminates() {
    let mut sim = world_of_individuals::experiment007::setup(42);
    sim.enable_assessment(world_of_individuals::assessment::Method::Grouped)
        .unwrap();
    sim.enable_retention(Method::Salient).unwrap();
    let event = world_of_individuals::experiment007::event(&sim);
    sim.inspect_refusal(
        event,
        &[(
            2,
            world_of_individuals::foresight::Sensor {
                reliability: 50,
                channel: world_of_individuals::foresight::Channel::AccurateFixture,
                effort: 0,
                second: None,
            },
        )],
    )
    .unwrap();
    sim.set_retention_policy(invalid).unwrap();
    assert!(
        sim.provenance_exchange(2, 0, event, true)
            .unwrap()
            .is_some()
    );
    let state = e::snapshot(&sim);
    state.validate().unwrap();
    assert!(
        state
            .retention
            .records
            .last()
            .unwrap()
            .fallback_reason
            .is_some()
    );
    assert_eq!(
        state
            .base
            .assessment
            .unwrap()
            .records
            .last()
            .unwrap()
            .support,
        50
    );
}
