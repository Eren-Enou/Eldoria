use world_of_individuals::{
    concerns::Status,
    experiment005,
    experiment006::*,
    foresight::{Channel, Sensor},
    inquiry::{self as q, Action, Novelty, Strategy},
    intentional::{Profile, TalkAction},
    model::*,
    simulation::Simulation,
};
fn owner_records(t: &Trial) -> Vec<&q::Record> {
    t.final_state
        .inquiry
        .records
        .iter()
        .filter(|r| r.decision.input.owner == 0)
        .collect()
}
fn strategy(record: &q::Record) -> Option<Strategy> {
    match record.decision.selected {
        Action::Ask { strategy, .. } => Some(strategy),
        _ => None,
    }
}

#[test]
fn controlled_scenarios_multi_seed_and_full_replay() {
    for seed in 0..128 {
        let trials = suite(seed);
        assert_eq!(trials, suite(seed));
        let weak = owner_records(&trials[0]);
        assert_eq!(weak[0].novelty, Novelty::New);
        assert_eq!(weak[0].realized_value, 40);
        assert_eq!(weak[1].novelty, Novelty::Redundant);
        assert_eq!(weak[1].realized_value, 0);
        assert_eq!(weak[0].cell_after.as_ref().unwrap().expected, 47);
        assert_eq!(weak[1].cell_after.as_ref().unwrap().expected, 23);
        let change = owner_records(&trials[1]);
        assert_eq!(strategy(change[2]), Some(Strategy::Evidence));
        assert_eq!(
            trials[1].history_ablated[2].selected,
            Action::Ask {
                concern: 0,
                source: 1,
                strategy: Strategy::Direct
            }
        );
        for i in [2, 5] {
            let records = owner_records(&trials[i]);
            let last = records.last().unwrap();
            assert_eq!(strategy(last), Some(Strategy::Evidence));
            assert_eq!(last.novelty, Novelty::Stronger);
            assert!(last.realized_value > 0);
            assert_eq!(
                last.concern_after.as_ref().unwrap().status,
                Status::Resolved
            );
        }
        let alternative = owner_records(&trials[3]);
        assert_eq!(
            alternative.last().unwrap().decision.selected,
            Action::Ask {
                concern: 0,
                source: 2,
                strategy: Strategy::Direct
            }
        );
        assert_eq!(
            alternative
                .last()
                .unwrap()
                .response
                .as_ref()
                .unwrap()
                .public_action,
            TalkAction::Silence
        );
        let important = &trials[4];
        assert!(
            owner_records(important)
                .iter()
                .any(|r| r.decision.selected == Action::Pause)
        );
        let concern = &important.final_state.base.concerns.items[&0][0];
        assert_eq!(concern.importance, 98);
        assert_eq!(concern.status, Status::Open);
        assert_eq!(concern.attempts, 3);
        assert!(
            owner_records(&trials[6])
                .iter()
                .all(|r| strategy(r) == Some(Strategy::Direct))
        );
        assert!(
            owner_records(&trials[7])
                .iter()
                .all(|r| r.decision.selected != Action::Pause)
        );
    }
}

#[test]
fn novelty_categories_provenance_and_no_truth_dependency() {
    let claim = q::Signature {
        event: 0,
        origin: q::Origin::Claim(1),
        claim: true,
        quality: 40,
        receipt: 0,
    };
    assert_eq!(q::novelty(&[], Some(&claim)), (Novelty::New, 40));
    let seen = vec![claim.clone()];
    let repeat = q::Signature {
        receipt: 999,
        ..claim.clone()
    };
    assert_eq!(q::novelty(&seen, Some(&repeat)), (Novelty::Redundant, 0));
    let stronger = q::Signature {
        origin: q::Origin::Evidence {
            subject: 1,
            channel: 0,
        },
        quality: 80,
        ..claim.clone()
    };
    assert_eq!(q::novelty(&seen, Some(&stronger)), (Novelty::Stronger, 60));
    let conflict = q::Signature {
        claim: false,
        ..stronger.clone()
    };
    assert_eq!(q::novelty(&seen, Some(&conflict)), (Novelty::Conflict, 30));
    assert_eq!(q::novelty(&seen, None), (Novelty::None, 0));
    let zero = q::Signature {
        quality: 0,
        ..stronger
    };
    assert_eq!(q::novelty(&seen, Some(&zero)), (Novelty::None, 0));
}

#[test]
fn local_history_alone_changes_strategy_and_source_choice() {
    let t = trial(42, "strategy_change");
    let r = owner_records(&t)[2];
    let mut ablated = r.decision.input.clone();
    ablated.cells.clear();
    assert_ne!(q::policy(&ablated).selected, r.decision.selected);
    let a = trial(42, "alternative_source");
    let records = owner_records(&a);
    let last = records.last().unwrap();
    assert!(last.decision.input.sources.iter().any(|s| s.id == 2));
    assert!(a.final_state.inquiry.contacts[&0].contains(&2));
    let meeting = &a.final_state.inquiry.meetings[last.meeting.unwrap() as usize];
    assert!(meeting.participants.contains(&0) && meeting.participants.contains(&2));
    let mut unavailable = last.decision.input.clone();
    unavailable.sources.retain(|s| s.id != 2);
    assert_eq!(q::policy(&unavailable).selected, Action::Pause);
    let mut stranger = last.decision.input.clone();
    stranger.sources.clear();
    assert!(!q::legal(&stranger, last.decision.selected));
    // Preserve the entire current context; removing only learned cells restores Source A.
    let mut fresh = last.decision.input.clone();
    fresh.cells.clear();
    assert_eq!(
        q::policy(&fresh).selected,
        Action::Ask {
            concern: 0,
            source: 1,
            strategy: Strategy::Direct
        }
    );
}

#[test]
fn readings_hidden_until_actual_response_and_not_necessarily_correct() {
    let run = |channel| {
        let mut sim = setup(42, 60, 30, q::Settings::default());
        let event = sim.concerns().unwrap().items[&0][0].event;
        sim.set_inquiry_capability(
            1,
            event,
            1,
            Sensor {
                reliability: 80,
                channel,
                ..Default::default()
            },
        )
        .unwrap();
        sim.inquiry_meeting(&[0, 1]).unwrap();
        snapshot(&sim, "private channel comparison")
    };
    let correct = run(Channel::AccurateFixture);
    let mistaken = run(Channel::InvertedFixture);
    assert_eq!(
        correct.inquiry.records[0].decision,
        mistaken.inquiry.records[0].decision
    );
    assert_eq!(correct.inquiry.notices[0], mistaken.inquiry.notices[0]);
    assert_ne!(
        correct.inquiry.records[0]
            .response
            .as_ref()
            .unwrap()
            .signatures,
        mistaken.inquiry.records[0]
            .response
            .as_ref()
            .unwrap()
            .signatures
    );
    assert_eq!(mistaken.base.concerns.items[&0][0].status, Status::Resolved);
    assert_eq!(mistaken.base.cognition.beliefs[&0][0].support, -80);
    let text = serde_json::to_string(&correct.inquiry.records[0].decision.input).unwrap();
    for private in [
        "AccurateFixture",
        "InvertedFixture",
        "own_historical_scarcity",
        "objectively_true",
        "sensor",
        "seed",
    ] {
        assert!(!text.contains(private));
    }
}

#[test]
fn invalid_inquiry_and_responder_terminate_without_learning_or_food_changes() {
    fn bad(input: &q::Input) -> q::Decision {
        let mut d = q::policy(input);
        d.selected = Action::Ask {
            concern: 999,
            source: 999,
            strategy: Strategy::Direct,
        };
        d
    }
    let mut sim = setup(42, 60, 30, q::Settings::default());
    sim.set_inquiry_policy(bad).unwrap();
    let food = sim.agents();
    sim.inquiry_meeting(&[0, 1]).unwrap();
    assert!(
        sim.inquiry()
            .unwrap()
            .records
            .iter()
            .all(|r| !r.valid && r.response.is_none())
    );
    assert!(sim.inquiry().unwrap().cells.is_empty());
    assert_eq!(sim.agents(), food);
    fn bad_response(
        input: &world_of_individuals::intentional::TalkInput,
        context: &world_of_individuals::foresight::Context,
    ) -> world_of_individuals::foresight::Plan {
        let mut p = world_of_individuals::foresight::predict(input, context);
        p.decision.selected = TalkAction::AskEvidence;
        p
    }
    let mut sim = setup(42, 60, 30, q::Settings::default());
    sim.set_predictor(bad_response).unwrap();
    sim.inquiry_meeting(&[0, 1]).unwrap();
    let r = &sim.inquiry().unwrap().records[0];
    assert!(r.valid);
    assert!(!r.response.as_ref().unwrap().valid);
    assert!(r.cell_after.is_none());
    assert!(sim.cognition().information.is_empty());
}

#[test]
fn lifecycle_atomic_validation_and_received_provenance_immutable() {
    let mut generator = Generator::new(42);
    let mut sim = Simulation::new(vec![generator.agent(0), generator.agent(1)]).unwrap();
    assert!(sim.enable_inquiry().is_err());
    assert!(sim.inquiry_meeting(&[0, 1]).is_err());
    let mut sim = setup(42, 60, 30, q::Settings::default());
    let before = sim.inquiry();
    sim.enable_inquiry().unwrap();
    assert_eq!(before, sim.inquiry());
    assert!(sim.inquiry_meeting(&[0, 0]).is_err());
    assert!(sim.inquiry_meeting(&[0, 99]).is_err());
    let event = sim.concerns().unwrap().items[&0][0].event;
    assert!(
        sim.set_inquiry_capability(2, event, 1, Sensor::default())
            .is_err()
    );
    assert!(
        sim.set_inquiry_capability(1, event, 3, Sensor::default())
            .is_err()
    );
    for _ in 0..3 {
        sim.inquiry_meeting(&[0, 1]).unwrap();
    }
    let before = sim.inquiry();
    assert!(
        sim.set_inquiry_capability(
            1,
            event,
            0,
            Sensor {
                reliability: 40,
                channel: Channel::InvertedFixture,
                ..Default::default()
            }
        )
        .is_err()
    );
    assert!(
        sim.set_inquiry_capability(
            1,
            event,
            0,
            Sensor {
                reliability: 80,
                ..Default::default()
            }
        )
        .is_err()
    );
    assert_eq!(before, sim.inquiry());
    sim.add_scene([0, 1], 1, 6).unwrap();
    assert!(sim.inquiry_meeting(&[0, 1]).is_err());
    assert!(sim.set_inquiry_settings(0, q::Settings::default()).is_err());
}

#[test]
fn new_opportunity_is_voluntary_and_repeat_notice_does_not_manufacture_value() {
    let mut sim = setup(42, 60, 30, q::Settings::default());
    for _ in 0..7 {
        sim.inquiry_meeting(&[0, 1]).unwrap();
    }
    let event = sim.concerns().unwrap().items[&0][0].event;
    sim.set_inquiry_capability(
        1,
        event,
        0,
        Sensor {
            reliability: 40,
            channel: Channel::AccurateFixture,
            ..Default::default()
        },
    )
    .unwrap();
    for _ in 0..3 {
        sim.inquiry_meeting(&[0, 1]).unwrap();
    }
    let state = sim.inquiry().unwrap();
    assert!(state.notices.iter().all(|n| n.offered));
    assert!(
        state
            .records
            .iter()
            .rev()
            .take(6)
            .filter(|r| r.decision.input.owner == 0)
            .all(|r| r.decision.selected == Action::Pause)
    );
    assert_eq!(sim.concerns().unwrap().items[&0][0].status, Status::Partial);
    // Source privacy can suppress announcement even after capability is configured.
    sim.set_communication_profile(
        1,
        Profile {
            privacy: 100,
            ..Default::default()
        },
    )
    .unwrap();
    sim.set_inquiry_capability(
        1,
        event,
        1,
        Sensor {
            reliability: 80,
            channel: Channel::AccurateFixture,
            ..Default::default()
        },
    )
    .unwrap();
    sim.inquiry_meeting(&[0, 1]).unwrap();
    assert!(!sim.inquiry().unwrap().notices.last().unwrap().offered);
    assert_eq!(
        sim.inquiry()
            .unwrap()
            .records
            .iter()
            .rev()
            .find(|r| r.decision.input.owner == 0)
            .unwrap()
            .decision
            .selected,
        Action::Pause
    );
}

#[test]
fn inquiry_audit_reconstructs_selected_learning_and_valid_receipts() {
    for trial in suite(42) {
        let mut cells = std::collections::BTreeMap::new();
        for r in &trial.final_state.inquiry.records {
            assert!(r.valid);
            assert_eq!(q::policy(&r.decision.input), r.decision);
            assert_eq!(r.food_before, r.food_after);
            if let Some(after) = &r.cell_after {
                let key = (
                    r.decision.input.owner,
                    after.concern,
                    after.source,
                    after.strategy,
                );
                assert_eq!(cells.get(&key), r.cell_before.as_ref());
                let prior = r
                    .cell_before
                    .as_ref()
                    .map_or(after.strategy.prior(), |c| c.expected);
                assert_eq!(after.expected, (prior + r.realized_value) / 2);
                cells.insert(key, after.clone());
            }
            if let Some(response) = &r.response {
                assert!(response.valid);
                for sig in &response.signatures {
                    let info = &trial.final_state.base.cognition.information[sig.receipt as usize];
                    assert_eq!(info.listener, r.decision.input.owner);
                    assert_eq!(sig.claim, info.scarce);
                    assert_eq!(sig.quality, info.reliability);
                    assert_eq!(sig.event, r.concern_before.as_ref().unwrap().event);
                }
            }
        }
        let actual: std::collections::BTreeMap<_, _> = trial
            .final_state
            .inquiry
            .cells
            .iter()
            .flat_map(|(&owner, v)| {
                v.iter()
                    .map(move |c| ((owner, c.concern, c.source, c.strategy), c.clone()))
            })
            .collect();
        assert_eq!(cells, actual);
        assert_eq!(trial.initial.base.events, trial.final_state.base.events);
    }
}

#[test]
fn population_replay_accounting_bounds_and_explicit_ticks() {
    for count in [0, 1, 11, 1000] {
        let p = population(42, count);
        assert_eq!(p, population(42, count));
        assert!(
            p.inquiry
                .cells
                .values()
                .all(|v| v.len() <= q::CELL_CAPACITY)
        );
        assert!(
            p.inquiry
                .seen
                .values()
                .all(|v| v.len() <= q::SIGNATURE_CAPACITY)
        );
        assert!(
            p.inquiry
                .episodes
                .values()
                .all(|v| v.len() <= q::EPISODE_CAPACITY)
        );
        let mut g = Generator::new(42);
        let total = (0..count)
            .map(|id| u64::from(g.agent(id).food))
            .sum::<u64>();
        assert_eq!(
            total,
            p.base.agents.iter().map(|a| u64::from(a.food)).sum::<u64>()
                + p.base
                    .cognition
                    .consumption
                    .iter()
                    .map(|r| u64::from(r.consumed))
                    .sum::<u64>()
        );
        if count > 0 {
            assert_eq!(
                accounted_ticks(&p),
                p.base.cognition.consumption.last().unwrap().tick
            );
        }
    }
    for trial in suite(42) {
        assert_eq!(
            accounted_ticks(&trial.final_state),
            trial.final_state.inquiry.records.last().unwrap().tick
        );
    }
}
fn accounted_ticks(p: &Snapshot) -> u64 {
    let resource_ticks: std::collections::BTreeSet<_> =
        p.base.events.iter().map(|e| e.tick).collect();
    resource_ticks.len() as u64
        + p.base.cognition.consumption.len() as u64
        + p.base
            .intentional
            .records
            .iter()
            .map(|r| u64::from(r.time_spent))
            .sum::<u64>()
        + p.base
            .concerns
            .records
            .iter()
            .map(|r| u64::from(r.time_spent))
            .sum::<u64>()
        + p.inquiry
            .records
            .iter()
            .map(|r| {
                u64::from(r.time_spent)
                    + r.response
                        .as_ref()
                        .map_or(0, |reply| u64::from(reply.time_spent))
            })
            .sum::<u64>()
        + p.inquiry.meetings.len() as u64
        + p.inquiry
            .notices
            .iter()
            .map(|n| u64::from(n.time_spent))
            .sum::<u64>()
}

#[test]
fn old_experiment005_archive_unchanged() {
    let current = serde_json::to_value(experiment005::suite(42)).unwrap();
    let old: serde_json::Value =
        serde_json::from_slice(&std::fs::read("experiments/005/seed-42.json").unwrap()).unwrap();
    assert_eq!(current, old);
}

#[test]
fn contact_episode_offer_and_capability_capacity_is_audited() {
    let mut generator = Generator::new(42);
    let mut sim = Simulation::new((0..21).map(|id| generator.agent(id)).collect()).unwrap();
    sim.enable_foresight(42, Sensor::default()).unwrap();
    sim.enable_concerns().unwrap();
    sim.enable_inquiry().unwrap();
    for source in 1..21 {
        sim.inquiry_meeting(&[0, source]).unwrap();
    }
    assert_eq!(
        sim.inquiry().unwrap().contacts[&0].len(),
        q::CONTACT_CAPACITY
    );
    assert_eq!(
        sim.inquiry()
            .unwrap()
            .evictions
            .iter()
            .filter(|e| e.owner == 0 && e.collection == "contacts")
            .count(),
        4
    );
    let mut sim = setup(
        42,
        60,
        100,
        q::Settings {
            use_history: false,
            ..Default::default()
        },
    );
    for _ in 0..25 {
        sim.inquiry_meeting(&[0, 1]).unwrap();
    }
    assert_eq!(
        sim.inquiry().unwrap().episodes[&0].len(),
        q::EPISODE_CAPACITY
    );
    assert_eq!(
        sim.inquiry()
            .unwrap()
            .evictions
            .iter()
            .filter(|e| e.collection == "episodes")
            .count(),
        9
    );
    let mut sim = setup(42, 60, 100, q::Settings::default());
    for _ in 0..40 {
        experiment005::refusal_conditions(&mut sim, 60);
        sim.add_scene([0, 1], 1, 6).unwrap();
        sim.run();
        let event = sim.concerns().unwrap().items[&0].last().unwrap().event;
        sim.set_inquiry_capability(
            1,
            event,
            0,
            Sensor {
                reliability: 40,
                channel: Channel::AccurateFixture,
                ..Default::default()
            },
        )
        .unwrap();
        sim.set_communication_profile(
            1,
            Profile {
                privacy: 0,
                ..Default::default()
            },
        )
        .unwrap();
        sim.inquiry_meeting(&[0, 1]).unwrap();
    }
    let state = sim.inquiry().unwrap();
    assert_eq!(state.capabilities[&1].len(), q::CELL_CAPACITY);
    assert_eq!(state.offers[&0].len(), q::SIGNATURE_CAPACITY);
    assert_eq!(
        state
            .evictions
            .iter()
            .filter(|e| e.collection == "capabilities")
            .count(),
        8
    );
    assert_eq!(
        state
            .evictions
            .iter()
            .filter(|e| e.collection == "offers")
            .count(),
        8
    );
}

#[test]
fn an_unfulfilled_offer_cannot_perpetually_restore_an_exhausted_method() {
    fn silent(
        input: &world_of_individuals::intentional::TalkInput,
        context: &world_of_individuals::foresight::Context,
    ) -> world_of_individuals::foresight::Plan {
        let mut plan = world_of_individuals::foresight::predict(input, context);
        plan.decision.selected = TalkAction::Silence;
        plan
    }
    let mut sim = setup(42, 60, 30, q::Settings::default());
    for _ in 0..7 {
        sim.inquiry_meeting(&[0, 1]).unwrap();
    }
    let event = sim.concerns().unwrap().items[&0][0].event;
    sim.set_inquiry_capability(
        1,
        event,
        1,
        Sensor {
            reliability: 80,
            channel: Channel::AccurateFixture,
            ..Default::default()
        },
    )
    .unwrap();
    sim.set_predictor(silent).unwrap();
    sim.inquiry_meeting(&[0, 1]).unwrap();
    assert!(matches!(
        sim.inquiry()
            .unwrap()
            .records
            .iter()
            .rev()
            .find(|r| r.decision.input.owner == 0)
            .unwrap()
            .decision
            .selected,
        Action::Ask {
            strategy: Strategy::Evidence,
            ..
        }
    ));
    sim.inquiry_meeting(&[0, 1]).unwrap();
    let state = sim.inquiry().unwrap();
    let last = state
        .records
        .iter()
        .rev()
        .find(|r| r.decision.input.owner == 0)
        .unwrap();
    assert_eq!(last.decision.selected, Action::Pause);
    assert_eq!(sim.concerns().unwrap().items[&0][0].status, Status::Partial);
}

#[test]
fn private_source_changes_are_not_in_inquiry_inputs_and_external_mismatch_is_safe() {
    let mut a = setup(42, 60, 30, q::Settings::default());
    let mut b = setup(42, 60, 100, q::Settings::default());
    b.set_circumstances(1, 999, 90, 100, 100).unwrap();
    a.inquiry_meeting(&[0, 1]).unwrap();
    b.inquiry_meeting(&[0, 1]).unwrap();
    assert_eq!(
        a.inquiry().unwrap().records[0].decision,
        b.inquiry().unwrap().records[0].decision
    );
    let state = a.inquiry();
    a.run();
    assert_eq!(state, a.inquiry());
    let mut sim = setup(42, 60, 30, q::Settings::default());
    let event = sim.concerns().unwrap().items[&0][0].event;
    sim.communicate(
        1,
        0,
        event,
        world_of_individuals::cognition::EvidenceKind::Fallible {
            scarce: true,
            reliability: 20,
            source: 0,
        },
    )
    .unwrap();
    for _ in 0..3 {
        sim.inquiry_meeting(&[0, 1]).unwrap();
    }
    let state = sim.inquiry().unwrap();
    let r = state
        .records
        .iter()
        .rev()
        .find(|r| r.decision.input.owner == 0)
        .unwrap();
    assert_eq!(strategy(r), Some(Strategy::Evidence));
    assert!(!r.response.as_ref().unwrap().valid);
    assert!(
        r.response
            .as_ref()
            .unwrap()
            .failure
            .as_ref()
            .unwrap()
            .contains("provenance")
    );
    assert!(r.response.as_ref().unwrap().receipts.is_empty());
    assert!(r.cell_after.is_none());
}
