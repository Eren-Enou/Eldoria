use world_of_individuals::{
    cognition::EvidenceKind,
    concerns::*,
    experiment005::*,
    foresight::{Channel, Sensor},
    model::*,
    simulation::Simulation,
};
fn end(t: &Trial) -> &Snapshot {
    t.snapshots.last().unwrap()
}
#[test]
fn controlled_scenarios_and_replay() {
    for seed in 0..128 {
        let t = suite(seed);
        assert_eq!(t, suite(seed));
        assert_eq!(end(&t[0]).concerns.items[&0][0].status, Status::Resolved);
        assert_eq!(end(&t[1]).concerns.items[&0][0].status, Status::Open);
        assert_eq!(end(&t[2]).concerns.items[&0][0].status, Status::Resolved);
        assert_eq!(end(&t[3]).concerns.items[&0][0].status, Status::Open);
        assert_eq!(end(&t[4]).concerns.items[&0][0].status, Status::Abandoned);
        assert_eq!(end(&t[5]).concerns.items[&0][0].status, Status::Partial);
        assert_eq!(end(&t[6]).concerns.items[&0][0].status, Status::Partial);
        assert_eq!(end(&t[6]).concerns.items[&0][0].uncertainty, 100);
        assert_eq!(end(&t[7]).concerns.items[&0][0].status, Status::Resolved);
        assert_eq!(
            end(&t[7])
                .cognition
                .information
                .last()
                .unwrap()
                .after
                .support,
            -80
        );
        assert_eq!(end(&t[8]).concerns.items[&0][0].status, Status::Abandoned);
        let competing = &end(&t[9]).concerns.items[&0];
        assert_eq!(competing.len(), 2);
        assert_eq!(competing[0].status, Status::Open);
        assert_eq!(competing[1].status, Status::Resolved);
        assert_eq!(end(&t[10]).concerns.items[&0][0].status, Status::Partial);
        assert_eq!(t[0].probe.selected, Action::Offer);
        assert_eq!(t[5].probe.selected, Action::Leave);
        assert_eq!(t[7].probe.selected, Action::Leave);
        for trial in &t {
            assert_eq!(trial.history_ablated.selected, FollowAction::Continue);
        }
    }
}
#[test]
fn persistence_creation_threshold_and_idle_run() {
    let mut s = setup(42, Sensor::default(), 60);
    encounter(&mut s);
    let c = s.concerns().unwrap();
    assert_eq!(c.items[&0][0].status, Status::Open);
    assert_eq!(c.items[&0][0].attempts, 0);
    assert!(
        c.records
            .iter()
            .all(|r| r.decision.selected == FollowAction::Continue)
    );
    s.run();
    assert_eq!(c, s.concerns().unwrap());
    let mut low = setup(42, Sensor::default(), 0);
    encounter(&mut low);
    assert!(low.concerns().unwrap().items.is_empty());
}
#[test]
fn transitions_reconstruct_bounded_state_and_accounting() {
    for t in suite(42) {
        let s = end(&t);
        let mut reconstructed = std::collections::BTreeMap::new();
        for tr in &s.concerns.transitions {
            if let Some(b) = &tr.before {
                assert_eq!(reconstructed.remove(&b.id).as_ref(), Some(b));
            }
            if let Some(a) = &tr.after {
                reconstructed.insert(a.id, a.clone());
            }
            if let Some(id) = tr.receipt {
                assert!(s.cognition.information.iter().any(|i| i.id == id));
            }
            if let Some(id) = tr.follow_up {
                assert!(s.concerns.records.iter().any(|r| r.id == id));
            }
        }
        assert_eq!(
            reconstructed.values().cloned().collect::<Vec<_>>(),
            s.concerns
                .items
                .values()
                .flatten()
                .cloned()
                .collect::<Vec<_>>()
        );
        for r in &s.concerns.records {
            assert!(r.valid);
            assert_eq!(r.food_before, r.food_after);
            assert_eq!(
                r.time_spent,
                1 + if matches!(r.decision.selected, FollowAction::Reopen(_)) {
                    r.decision.input.pursuit_cost
                } else {
                    0
                }
            );
            if let Some(id) = r.response_conversation {
                let c = &s.intentional.conversations[id as usize];
                assert!(c.end_record - c.first_record <= 2);
                assert_eq!(c.event, r.before.as_ref().unwrap().event);
                assert_eq!(
                    s.intentional.records[c.first_record].decision.input.last,
                    Some(world_of_individuals::intentional::TalkAction::AskEvidence)
                );
            }
        }
        for e in &s.events {
            assert_eq!(
                e.balances_before.iter().sum::<u32>(),
                e.balances_after.iter().sum::<u32>()
            );
        }
    }
}
#[test]
fn external_evidence_refreshes_concern_and_preserves_original_memory() {
    let mut s = setup(42, Sensor::default(), 60);
    encounter(&mut s);
    let event = s.concerns().unwrap().items[&0][0].event;
    let original = s.events()[event as usize].clone();
    s.communicate(
        1,
        0,
        event,
        EvidenceKind::Fallible {
            scarce: true,
            reliability: 80,
            source: 0,
        },
    )
    .unwrap();
    assert_eq!(s.concerns().unwrap().items[&0][0].status, Status::Resolved);
    s.communicate(
        1,
        0,
        event,
        EvidenceKind::Fallible {
            scarce: false,
            reliability: 80,
            source: 1,
        },
    )
    .unwrap();
    assert_eq!(s.concerns().unwrap().items[&0][0].status, Status::Partial);
    assert_eq!(s.events()[event as usize], original);
    assert_eq!(
        s.cognition().information[0]
            .revision
            .as_ref()
            .unwrap()
            .after
            .interpretation,
        Interpretation::PossibleSelfProtection
    );
    assert_eq!(s.agents()[0].trust[&1], -17);
}
#[test]
fn config_legality_and_invalid_policy_terminate() {
    fn bad(input: &FollowInput) -> FollowDecision {
        let mut d = policy(input);
        d.selected = FollowAction::Reopen(999);
        d
    }
    let mut g = Generator::new(42);
    let mut s = Simulation::new(vec![g.agent(0), g.agent(1)]).unwrap();
    assert!(s.enable_concerns().is_err());
    s.enable_foresight(42, Sensor::default()).unwrap();
    s.enable_concerns().unwrap();
    assert!(s.set_follow_cost(0, 101).is_err());
    assert!(s.set_follow_cost(99, 20).is_err());
    s.set_concern_policy(bad).unwrap();
    s.add_scene([0, 1], 1, 6).unwrap();
    assert!(s.set_follow_cost(0, 20).is_err());
    s.run();
    assert!(
        s.concerns()
            .unwrap()
            .records
            .iter()
            .all(|r| !r.valid && r.response_conversation.is_none())
    );
    let before = s.concerns();
    s.enable_concerns().unwrap();
    assert_eq!(before, s.concerns());
}
#[test]
fn unchanged_fourth_experiment_archive() {
    let actual = serde_json::to_value(world_of_individuals::experiment004::suite(42)).unwrap();
    let archived: serde_json::Value =
        serde_json::from_slice(&std::fs::read("experiments/004/seed-42.json").unwrap()).unwrap();
    assert_eq!(actual, archived);
}
#[test]
fn repeated_failures_and_same_source_do_not_manufacture_certainty() {
    let t = suite(42);
    let silent = &end(&t[8]).concerns.items[&0][0];
    assert!(silent.failures >= 3);
    assert_eq!(silent.uncertainty, 100);
    let weak = &end(&t[10]).concerns.items[&0][0];
    assert!(weak.failures >= 2);
    assert_eq!(weak.uncertainty, 60);
    assert!(
        end(&t[10])
            .cognition
            .information
            .iter()
            .all(|i| i.after.support == 40)
    );
    assert!(
        end(&t[8])
            .concerns
            .records
            .iter()
            .any(|r| r.answer_after < r.decision.input.answer_probability)
    );
}
#[test]
fn population_empty_odd_and_replay() {
    for n in [0, 1, 11, 1000] {
        let p = population(42, n);
        assert_eq!(p, population(42, n));
        assert!(p.concerns.items.values().all(|c| c.len() <= CAPACITY));
        let mut g = Generator::new(42);
        let initial = (0..n).map(|i| u64::from(g.agent(i).food)).sum::<u64>();
        assert_eq!(
            initial,
            p.agents.iter().map(|a| u64::from(a.food)).sum::<u64>()
                + p.cognition
                    .consumption
                    .iter()
                    .map(|c| u64::from(c.consumed))
                    .sum::<u64>()
        );
    }
}
#[test]
fn hidden_truth_is_not_in_follow_policy_and_wrong_evidence_can_close() {
    let a = trial(42, Config::default());
    let b = trial(
        42,
        Config {
            sensor: Sensor {
                channel: Channel::InvertedFixture,
                ..Config::default().sensor
            },
            ..Config::default()
        },
    );
    let fa = &end(&a).concerns.records[2];
    let fb = &end(&b).concerns.records[2];
    assert_eq!(fa.decision, fb.decision);
    assert_eq!(
        fa.after.as_ref().unwrap().status,
        fb.after.as_ref().unwrap().status
    );
    assert_ne!(end(&a).cognition.beliefs, end(&b).cognition.beliefs);
}

#[test]
fn capacity_eviction_is_explicit_and_concerns_outlive_episodes() {
    fn defer(input: &FollowInput) -> FollowDecision {
        let mut d = policy(input);
        d.selected = FollowAction::Continue;
        d
    }
    let mut s = setup(42, Sensor::default(), 60);
    s.set_concern_policy(defer).unwrap();
    for _ in 0..12 {
        refusal_conditions(&mut s, 60);
        encounter(&mut s);
    }
    let state = s.concerns().unwrap();
    assert_eq!(state.items[&0].len(), CAPACITY);
    assert_eq!(
        state
            .transitions
            .iter()
            .filter(|t| t.reason == "capacity abandonment")
            .count(),
        4
    );
    let event = state.items[&0][0].event;
    later_conditions(&mut s, 60, 100, 20);
    for _ in 0..20 {
        encounter(&mut s);
    }
    assert!(!s.agents()[0].memories.iter().any(|m| m.event == event));
    let receipt = s
        .communicate(
            1,
            0,
            event,
            EvidenceKind::Fallible {
                scarce: true,
                reliability: 80,
                source: 0,
            },
        )
        .unwrap();
    assert!(receipt.revision.is_none());
    assert_eq!(
        s.concerns().unwrap().items[&0]
            .iter()
            .find(|c| c.event == event)
            .unwrap()
            .status,
        Status::Resolved
    );
}

#[test]
fn only_relevant_partner_can_be_asked_and_history_alone_changes_choice() {
    let t = trial(42, Config::default());
    let d = &end(&t).concerns.records[2].decision;
    assert!(matches!(d.selected, FollowAction::Reopen(_)));
    let mut input = d.input.clone();
    input.partner = 99;
    assert_eq!(policy(&input).selected, FollowAction::Continue);
    let mut ablated = d.input.clone();
    ablated.concerns.clear();
    assert_eq!(t.history_ablated.input, ablated);
    assert_eq!(t.history_ablated.selected, FollowAction::Continue);
    let t = trial(
        42,
        Config {
            name: "failures".into(),
            later_privacy: 100,
            encounters: 4,
            ..Default::default()
        },
    );
    let r = end(&t)
        .concerns
        .records
        .iter()
        .find(|r| matches!(r.decision.selected, FollowAction::Abandon(_)))
        .unwrap();
    let mut no_failures = r.decision.input.clone();
    for c in &mut no_failures.concerns {
        c.failures = 0;
    }
    assert!(matches!(
        policy(&no_failures).selected,
        FollowAction::Reopen(_)
    ));
}

#[test]
fn costly_proof_can_be_withheld_and_learning_can_be_disabled() {
    let mut s = setup(
        42,
        Sensor {
            effort: 100,
            ..Default::default()
        },
        60,
    );
    encounter(&mut s);
    s.set_prediction_settings(
        0,
        world_of_individuals::foresight::Settings {
            learn: false,
            ..Default::default()
        },
    )
    .unwrap();
    later_conditions(&mut s, 60, 0, 20);
    encounter(&mut s);
    let state = s.concerns().unwrap();
    let r = &state.records[2];
    assert!(matches!(r.decision.selected, FollowAction::Reopen(_)));
    assert_eq!(
        r.actual_response,
        Some(world_of_individuals::intentional::TalkAction::Silence)
    );
    assert_eq!(r.answer_after, 50);
    assert!(!r.learned);
    assert_eq!(r.prediction_error, Some(-50));
    assert_eq!(state.items[&0][0].failures, 1);
    assert!(s.cognition().information.is_empty());
}

#[test]
fn followup_can_substantiate_an_earlier_claim_and_age_ties_are_stable() {
    let mut s = setup(
        42,
        Sensor {
            channel: Channel::AccurateFixture,
            ..Default::default()
        },
        60,
    );
    s.set_communication_profile(
        1,
        world_of_individuals::intentional::Profile {
            privacy: 30,
            ..Default::default()
        },
    )
    .unwrap();
    encounter(&mut s);
    assert_eq!(s.concerns().unwrap().items[&0][0].status, Status::Partial);
    later_conditions(&mut s, 60, 0, 20);
    encounter(&mut s);
    assert_eq!(s.concerns().unwrap().items[&0][0].status, Status::Resolved);
    assert_eq!(s.intentional().unwrap().credibility[&0][&1], 16);
    let t = trial(
        42,
        Config {
            competing: true,
            ..Default::default()
        },
    );
    let mut input = end(&t)
        .concerns
        .records
        .iter()
        .find(|r| r.decision.input.concerns.len() == 2)
        .unwrap()
        .decision
        .input
        .clone();
    for c in &mut input.concerns {
        c.age = 1;
    }
    assert_eq!(policy(&input).selected, FollowAction::Reopen(1));
    input.concerns[0].importance = 100;
    input.concerns[1].importance = 50;
    assert_eq!(policy(&input).selected, FollowAction::Reopen(0));
    input.concerns[1].importance = 100;
    assert_eq!(policy(&input).selected, FollowAction::Reopen(0));
}

#[test]
fn combined_audits_account_for_every_tick_and_reconstruct_population_concerns() {
    let p = population(42, 101);
    let mut state = std::collections::BTreeMap::new();
    for t in &p.concerns.transitions {
        if let Some(b) = &t.before {
            assert_eq!(state.remove(&b.id).as_ref(), Some(b));
        }
        if let Some(a) = &t.after {
            state.insert(a.id, a.clone());
        }
    }
    let actual: std::collections::BTreeMap<_, _> = p
        .concerns
        .items
        .values()
        .flatten()
        .map(|c| (c.id, c.clone()))
        .collect();
    assert_eq!(state, actual);
    let resource_ticks: std::collections::BTreeSet<_> = p.events.iter().map(|e| e.tick).collect();
    let accounted = resource_ticks.len() as u64
        + p.cognition.consumption.len() as u64
        + p.intentional
            .records
            .iter()
            .map(|r| u64::from(r.time_spent))
            .sum::<u64>()
        + p.concerns
            .records
            .iter()
            .map(|r| u64::from(r.time_spent))
            .sum::<u64>();
    assert_eq!(accounted, p.cognition.consumption.last().unwrap().tick);
}
