use world_of_individuals::{
    behavior::scarcity_policy,
    cognition::EvidenceKind,
    experiment004::*,
    foresight::*,
    intentional::{Profile, TalkAction, TalkInput},
    model::*,
    simulation::Simulation,
};
fn records(t: &Trial) -> Vec<&world_of_individuals::intentional::TalkRecord> {
    t.intentional
        .records
        .iter()
        .filter(|r| r.decision.input.event == t.target)
        .collect()
}
fn prepared(sensor: Sensor) -> Simulation {
    let mut g = Generator::new(42);
    let mut s = Simulation::new(vec![g.agent(0), g.agent(1)]).unwrap();
    s.set_policy(scarcity_policy);
    s.enable_foresight(42, sensor).unwrap();
    s.set_circumstances(0, 0, 90, 20, 20).unwrap();
    s.set_circumstances(1, 1, 90, 0, 90).unwrap();
    s.set_communication_profile(
        0,
        Profile {
            request_cost: 0,
            ..Profile::default()
        },
    )
    .unwrap();
    s.set_communication_profile(
        1,
        Profile {
            privacy: 0,
            ..Profile::default()
        },
    )
    .unwrap();
    s
}
#[test]
fn controlled_sweeps_anticipation_cost_quality_and_learning() {
    for seed in 0..128 {
        let t = suite(seed);
        assert_eq!(t, suite(seed));
        assert_eq!(records(&t[0])[0].decision.selected, TalkAction::Mislead);
        assert_eq!(records(&t[1])[0].decision.selected, TalkAction::Silence);
        assert_eq!(records(&t[2])[0].decision.selected, TalkAction::Silence);
        assert_eq!(records(&t[3])[0].decision.selected, TalkAction::Mislead);
        assert_eq!(records(&t[4])[2].decision.selected, TalkAction::Silence); // Avoid self-incrimination.
        assert_eq!(t[5].probe.selected, Action::Offer);
        assert_eq!(t[6].probe.selected, Action::Leave);
        assert_eq!(t[7].cognition.information.last().unwrap().after.support, 40);
        assert_eq!(t[8].cognition.information.last().unwrap().after.support, 80);
        assert_eq!(t[9].cognition.information.last().unwrap().after.support, 0);
        assert_eq!(t[7].probe.selected, Action::Leave);
        assert_eq!(t[8].probe.selected, Action::Offer);
        assert_eq!(t[9].probe.selected, Action::Leave);
        assert_eq!(
            records(&t[10])[1].decision.selected,
            TalkAction::AskEvidence
        );
        assert_eq!(records(&t[11])[1].decision.selected, TalkAction::Silence);
        assert_eq!(records(&t[12])[2].decision.selected, TalkAction::Silence);
        assert!(t[12].foresight.readings.is_empty());
        assert_eq!(records(&t[13])[0].decision.selected, TalkAction::Mislead);
        assert_eq!(records(&t[14])[0].decision.selected, TalkAction::Silence);
        assert_eq!(records(&t[15])[0].decision.selected, TalkAction::Silence);
        assert_eq!(records(&t[17])[2].decision.selected, TalkAction::Mislead); // Explicit horizon loophole.
        assert_eq!(t[0].events[..2], t[1].events[..2]);
    }
}
#[test]
fn expectation_only_ablation_and_actual_learning_records() {
    let t = suite(42);
    let low = records(&t[13])[0];
    let high = records(&t[14])[0];
    assert_eq!(low.decision.input.own, high.decision.input.own);
    assert_eq!(low.decision.input.profile, high.decision.input.profile);
    let l = t[13]
        .foresight
        .forecasts
        .iter()
        .find(|f| f.talk_record == low.id)
        .unwrap();
    let h = t[14]
        .foresight
        .forecasts
        .iter()
        .find(|f| f.talk_record == high.id)
        .unwrap();
    assert_eq!(l.context.expectation.challenge, 10);
    assert_eq!(h.context.expectation.challenge, 90);
    let mut changed = l.context.clone();
    changed.expectation.challenge = h.context.expectation.challenge;
    assert_eq!(
        predict(&low.decision.input, &l.context).decision.selected,
        TalkAction::Mislead
    );
    assert_eq!(
        predict(&low.decision.input, &changed).decision.selected,
        TalkAction::Silence
    );
    let before = t[13]
        .foresight
        .errors
        .iter()
        .filter(|e| e.actor == 1)
        .take(6)
        .map(|e| e.updated)
        .collect::<Vec<_>>();
    assert_eq!(before, vec![38, 29, 22, 17, 13, 10]);
    for trial in &t {
        for error in &trial.foresight.errors {
            assert_eq!(error.error, error.outcome - error.expected);
            assert_eq!(
                error.updated,
                if error.learned {
                    update_probability(error.expected, error.outcome)
                } else {
                    error.expected
                }
            );
            let response = &trial.intentional.records[error.response_record as usize];
            assert_eq!(error.actual, response.decision.selected);
            assert!(error.response_record > error.forecast_record);
        }
    }
}
#[test]
fn fallible_evidence_is_not_truth_and_conflicts_do_not_accumulate() {
    let mut g = Generator::new(42);
    let mut s = Simulation::new(vec![g.agent(0), g.agent(1)]).unwrap();
    s.set_circumstances(0, 0, 90, 20, 20).unwrap();
    s.set_circumstances(1, 4, 90, 0, 90).unwrap();
    s.add_scene([0, 1], 1, 6).unwrap();
    s.run();
    let original = s.events();
    let original_trust = s.agents()[0].trust[&1];
    let wrong = EvidenceKind::Fallible {
        scarce: true,
        reliability: 80,
        source: 0,
    };
    let first = s.communicate(1, 0, 1, wrong).unwrap();
    assert_eq!(first.after.support, 80);
    assert!(first.revision.is_some());
    let opposing = EvidenceKind::Fallible {
        scarce: false,
        reliability: 80,
        source: 1,
    };
    assert_eq!(s.communicate(1, 0, 1, opposing).unwrap().after.support, 0);
    for _ in 0..5 {
        assert_eq!(s.communicate(1, 0, 1, wrong).unwrap().after.support, 0);
    }
    assert_eq!(s.agents()[0].trust[&1], original_trust);
    assert_eq!(s.events(), original);
    let prior = s.cognition();
    let agents = s.agents();
    assert!(
        s.communicate(
            1,
            0,
            1,
            EvidenceKind::Fallible {
                scarce: false,
                reliability: 80,
                source: 0
            }
        )
        .is_err()
    );
    assert!(
        s.communicate(
            1,
            0,
            1,
            EvidenceKind::Fallible {
                scarce: true,
                reliability: 101,
                source: 0
            }
        )
        .is_err()
    );
    assert_eq!(s.cognition(), prior);
    assert_eq!(s.agents(), agents);
    assert_eq!(
        s.communicate(1, 0, 1, EvidenceKind::Testimony { scarce: true })
            .unwrap()
            .after
            .support,
        0
    );
}
#[test]
fn sampled_noise_is_reproducible_but_not_universally_correct() {
    let sensor = Sensor::default();
    let mut wrong = 0;
    for seed in 0..128 {
        let observed = reading(seed, 1, 1, 0, true, &sensor);
        assert_eq!(observed, reading(seed, 1, 1, 0, true, &sensor));
        wrong += usize::from(!observed);
    }
    assert_eq!(wrong, 15);
    let sensor = Sensor {
        reliability: 0,
        ..sensor
    };
    assert!((0..128).any(|seed| reading(seed, 1, 1, 0, true, &sensor)));
    assert!((0..128).any(|seed| !reading(seed, 1, 1, 0, true, &sensor)));
}
#[test]
fn hidden_partner_and_sensor_outcomes_cannot_affect_initial_prediction() {
    let sensor = Sensor {
        channel: Channel::AccurateFixture,
        ..Sensor::default()
    };
    let mut a = prepared(sensor.clone());
    let mut b = prepared(Sensor {
        channel: Channel::InvertedFixture,
        ..sensor
    });
    b.set_communication_profile(
        0,
        Profile {
            relationship_goal: 0,
            request_cost: 100,
            privacy: 100,
            honesty: 0,
        },
    )
    .unwrap();
    for sim in [&mut a, &mut b] {
        sim.add_scene([0, 1], 1, 6).unwrap();
        sim.run();
    }
    assert_eq!(
        a.intentional().unwrap().records[0].decision,
        b.intentional().unwrap().records[0].decision
    );
    assert_eq!(
        a.foresight().unwrap().forecasts[0],
        b.foresight().unwrap().forecasts[0]
    );
}
fn malformed(input: &TalkInput, context: &Context) -> Plan {
    let mut p = predict(input, context);
    p.candidates.clear();
    p
}
#[test]
fn invalid_predictor_terminates_atomically_without_feedback() {
    let mut s = prepared(Sensor::default());
    s.set_predictor(malformed).unwrap();
    s.add_scene([0, 1], 1, 6).unwrap();
    s.run();
    assert_eq!(
        s.intentional().unwrap().conversations[0].outcome,
        "InvalidAction"
    );
    assert!(s.cognition().information.is_empty());
    assert!(s.foresight().unwrap().errors.is_empty());
    let before = s.foresight();
    s.run();
    assert_eq!(before, s.foresight());
}
#[test]
fn profile_sensor_validation_and_no_reset_of_learned_expectations() {
    let mut s = prepared(Sensor::default());
    let before = s.foresight();
    assert!(s.enable_foresight(7, Sensor::default()).is_err());
    assert!(
        s.set_prediction_settings(
            0,
            Settings {
                challenge_prior: -1,
                ..Settings::default()
            }
        )
        .is_err()
    );
    assert!(s.set_prediction_settings(99, Settings::default()).is_err());
    assert_eq!(before, s.foresight());
    s.add_scene([0, 1], 1, 6).unwrap();
    assert!(s.set_prediction_settings(0, Settings::default()).is_err());
    s.run();
    let learned = s.foresight().unwrap().expectations;
    s.set_prediction_settings(
        1,
        Settings {
            challenge_prior: 0,
            ..Settings::default()
        },
    )
    .unwrap();
    assert_eq!(s.foresight().unwrap().expectations, learned);
}
#[test]
fn explicit_effort_ticks_resource_accounting_and_population_replay() {
    for seed in [0, 42, u64::MAX] {
        let p = population(seed, 101);
        assert_eq!(
            serde_json::to_vec(&p).unwrap(),
            serde_json::to_vec(&population(seed, 101)).unwrap()
        );
        let initial: u64 = p.initial.iter().map(|a| u64::from(a.food)).sum();
        let final_food: u64 = p.final_agents.iter().map(|a| u64::from(a.food)).sum();
        assert_eq!(
            initial,
            final_food
                + p.cognition
                    .consumption
                    .iter()
                    .map(|c| u64::from(c.consumed))
                    .sum::<u64>()
        );
        let mut tick = p.events.last().unwrap().tick;
        for r in &p.intentional.records {
            assert_eq!(r.food_before, r.food_after);
            assert_eq!(r.tick, tick + u64::from(r.time_spent));
            tick = r.tick;
            assert_eq!(
                r.time_spent,
                if r.decision.selected == TalkAction::ProvideEvidence {
                    6
                } else {
                    1
                }
            );
        }
        for c in &p.cognition.consumption {
            assert_eq!(c.tick, tick + 1);
            tick = c.tick;
        }
        assert!(
            p.intentional
                .conversations
                .iter()
                .all(|c| c.end_record - c.first_record <= 4)
        );
    }
}
#[test]
fn earlier_saved_reports_are_unchanged() {
    use world_of_individuals::{experiment, experiment002, experiment003};
    let read = |path: &str| {
        serde_json::from_slice::<serde_json::Value>(&std::fs::read(path).unwrap()).unwrap()
    };
    assert_eq!(
        serde_json::to_value(experiment::run_experiment(42, "all", 1000).unwrap()).unwrap(),
        read("experiments/001/results/seed-42/report.json")
    );
    assert_eq!(
        serde_json::to_value(experiment002::suite(42)).unwrap(),
        read("experiments/002/seed-42.json")
    );
    assert_eq!(
        serde_json::to_value(experiment003::suite(42)).unwrap(),
        read("experiments/003/seed-42.json")
    );
}

#[test]
fn population_forecast_audit_reconstructs_expectations_from_public_responses() {
    use std::collections::BTreeMap;
    let p = population(42, 1000);
    let mut expected: BTreeMap<AgentId, BTreeMap<AgentId, Expectation>> = BTreeMap::new();
    for (r, f) in p.intentional.records.iter().zip(&p.foresight.forecasts) {
        assert_eq!(r.id, f.talk_record);
        let actor = r.decision.input.own.id;
        let partner = r.decision.input.partner;
        let prior = expected
            .get(&actor)
            .and_then(|m| m.get(&partner))
            .cloned()
            .unwrap_or(Expectation {
                challenge: f.context.settings.challenge_prior,
                answer: f.context.settings.answer_prior,
            });
        assert_eq!(f.context.expectation, prior);
        let calculated = predict(&r.decision.input, &f.context);
        assert_eq!(calculated.decision, r.decision);
        assert_eq!(calculated.candidates, f.candidates);
        for c in &f.candidates {
            assert_eq!(c.combined, c.immediate + c.anticipated);
        }
        for e in p
            .foresight
            .errors
            .iter()
            .filter(|e| e.response_record == r.id)
        {
            assert_eq!(e.actual, r.decision.selected);
            assert_eq!(e.outcome, if e.actual == e.predicted { 100 } else { 0 });
            let f = p
                .foresight
                .forecasts
                .iter()
                .find(|f| f.talk_record == e.forecast_record)
                .unwrap();
            let state = expected
                .entry(e.actor)
                .or_default()
                .entry(e.partner)
                .or_insert(f.context.expectation.clone());
            assert_eq!(
                e.expected,
                if e.predicted == TalkAction::AskEvidence {
                    state.challenge
                } else {
                    state.answer
                }
            );
            let value = update_probability(e.expected, e.outcome);
            assert_eq!(value, e.updated);
            if e.predicted == TalkAction::AskEvidence {
                state.challenge = value;
            } else {
                state.answer = value;
            }
        }
    }
    assert_eq!(expected, p.foresight.expectations);
}
#[test]
fn population_received_evidence_reconstructs_agent_state() {
    use world_of_individuals::behavior::remember;
    let p = population(42, 1000);
    let mut agents = p.initial.clone();
    for e in &p.events {
        assert_eq!(
            e.decision,
            scarcity_policy(&agents[e.decision.actor as usize], &e.decision.observation)
        );
        for i in 0..2 {
            let a = &mut agents[e.participants[i] as usize];
            assert_eq!(a.food, e.balances_before[i]);
            a.food = e.balances_after[i];
            remember(a, e.interpretations[i].clone());
        }
    }
    for r in &p.intentional.records {
        assert_eq!(
            r.decision.input.own,
            agents[r.decision.input.own.id as usize]
        );
        if let Some(id) = r.information {
            let info = &p.cognition.information[id as usize];
            let a = &mut agents[info.listener as usize];
            assert_eq!(info.trust_before, *a.trust.get(&info.speaker).unwrap_or(&0));
            if let Some(rev) = &info.revision {
                let memory = a
                    .memories
                    .iter_mut()
                    .find(|m| m.event == info.event)
                    .unwrap();
                assert_eq!(*memory, rev.before);
                *memory = rev.after.clone();
                a.trust.insert(info.speaker, info.trust_after);
            }
        }
    }
    for c in &p.cognition.consumption {
        let a = &mut agents[c.actor as usize];
        assert_eq!((a.food, a.hunger), (c.food_before, c.hunger_before));
        a.food = c.food_after;
        a.hunger = c.hunger_after;
    }
    assert_eq!(agents, p.final_agents);
}
