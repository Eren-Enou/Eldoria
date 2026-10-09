use world_of_individuals::{
    attention as t, experiment011 as e, inquiry as q, inquiry_value as v, model::Action,
    provenance as p,
};

fn candidate(id: u64, importance: i32, gain: i32) -> q::Candidate {
    q::Candidate {
        action: q::Action::Ask {
            concern: id,
            source: 1,
            strategy: q::Strategy::Direct,
        },
        importance,
        attempts: 0,
        prior_or_learned: gain,
        opportunity_value: 0,
        expected_gain: gain,
        benefit: importance * gain / 100,
        cost: 13,
        score: importance * gain / 100 - 13,
        history_record: None,
        notice: None,
    }
}
fn decision(candidates: Vec<q::Candidate>) -> p::Decision {
    let input = p::Input {
        base: q::Input {
            owner: 0,
            scene: 2,
            hunger: 30,
            relationship_goal: 100,
            concerns: vec![],
            sources: vec![],
            cells: vec![],
            seen: vec![],
            settings: Default::default(),
        },
        knowledge: vec![],
        hints: vec![],
        settings: Default::default(),
    };
    p::Decision {
        decision: q::Decision {
            input: input.base.clone(),
            candidates,
            selected: q::Action::Pause,
        },
        input,
        factors: vec![],
    }
}
#[test]
fn status_value_distinguishes_near_status_from_uncertainty_with_zero_and_tie_boundaries() {
    let base = decision(vec![candidate(0, 80, 20), candidate(1, 60, 55)]);
    let near = [t::Basis {
        concern: 0,
        event: 1,
        support: 55,
    }];
    let far = [t::Basis {
        concern: 0,
        event: 1,
        support: 5,
    }];
    assert_eq!(
        v::policy(base.clone(), &near).decision.selected,
        base.decision.candidates[0].action
    );
    assert_eq!(
        v::policy(base.clone(), &far).decision.selected,
        base.decision.candidates[1].action
    );
    assert_eq!(
        t::policy(base.clone(), &near).decision.selected,
        base.decision.candidates[1].action
    );
    assert_eq!(v::components(&base.decision.candidates[0], &near).value, 60);
    assert_eq!(v::components(&base.decision.candidates[0], &far).value, 28);
    assert_eq!(
        v::policy(
            decision(vec![candidate(0, 100, 0), candidate(1, 0, 100)]),
            &near
        )
        .decision
        .selected,
        q::Action::Pause
    );
    let tied = decision(vec![candidate(0, 70, 55), candidate(1, 70, 55)]);
    assert_eq!(
        v::policy(tied.clone(), &[]).decision.selected,
        tied.decision.candidates[0].action
    );
    let mut changed = base.clone();
    changed.decision.candidates[1].importance = 100;
    assert_ne!(
        v::policy(base, &near).decision.selected,
        v::policy(changed, &near).decision.selected
    );
}
#[test]
fn exact_controls_and_all_sequential_families_conserve_and_preserve_real_opportunity_cost() {
    for case in e::CASES {
        let group = v::MODES.map(|m| e::trial(42, m, case, e::Config::default()));
        for trial in &group {
            assert_eq!(trial.before_prelude, group[0].before_prelude);
            assert_eq!(
                trial.opportunities[0].before,
                group[0].opportunities[0].before
            );
            assert_eq!(
                trial
                    .before_prelude
                    .concerns
                    .iter()
                    .filter(|c| c.status.active())
                    .count(),
                if case == "wrong_confidence" { 2 } else { 3 }
            );
            assert_eq!(e::queries(trial).len(), 2);
            assert_eq!(trial.opportunities.last().unwrap().remaining, 0);
            trial.final_state.validate().unwrap();
            let compact = &trial.final_state.base.base.base;
            for r in &trial.final_state.value.records {
                let query = compact
                    .query(world_of_individuals::audit::QueryId(r.query))
                    .unwrap();
                let original = p::policy(&query.decision.input);
                assert_eq!(query.decision, v::compare(trial.mode, original, &r.basis));
                assert!(r.basis.len() <= 8);
            }
            for event in &compact.base.base.events {
                assert_eq!(
                    event.balances_before.iter().sum::<u32>(),
                    event.balances_after.iter().sum::<u32>()
                );
            }
            assert_eq!(
                trial.after_action.agents[0].food - trial.persistent.agents[0].food,
                trial.consumption.consumed
            );
        }
    }
}
#[test]
fn future_partner_and_unobserved_future_scarcity_do_not_select_current_queries() {
    for mode in v::MODES {
        for config in e::held_out() {
            let a = e::trial(42, mode, "uncertain_minor", config.clone());
            let b = e::trial(42, mode, "mirrored_future", config);
            assert_eq!(a.opportunities, b.opportunities);
            assert_eq!(e::queries(&a)[0].decision, e::queries(&b)[0].decision);
        }
        let a = e::trial(
            42,
            mode,
            "uncertain_minor",
            e::Config {
                partner: 4,
                unmet_food: 0,
                ..Default::default()
            },
        );
        let b = e::trial(
            42,
            mode,
            "uncertain_minor",
            e::Config {
                partner: 4,
                unmet_food: 9,
                ..Default::default()
            },
        );
        assert_eq!(e::queries(&a)[0].decision, e::queries(&b)[0].decision);
        assert_eq!(
            a.opportunities
                .iter()
                .map(|o| &o.before.basis)
                .collect::<Vec<_>>(),
            b.opportunities
                .iter()
                .map(|o| &o.before.basis)
                .collect::<Vec<_>>()
        );
        assert_ne!(
            a.opportunities[0].before.agents[4].food,
            b.opportunities[0].before.agents[4].food
        );
    }
}
#[test]
fn silent_failed_history_and_wrong_confidence_remain_failures() {
    for mode in v::MODES {
        let silent = e::trial(42, mode, "silent", Default::default());
        for r in e::queries(&silent) {
            assert_eq!(r.realized_value, 0);
            assert!(r.time_spent > 0);
            assert!(
                r.cell_after.as_ref().unwrap().expected
                    < r.cell_before.as_ref().map_or_else(
                        || {
                            if let q::Action::Ask { strategy, .. } = r.decision.selected {
                                strategy.prior()
                            } else {
                                0
                            }
                        },
                        |c| c.expected
                    )
            );
        }
        let wrong = e::trial(42, mode, "wrong_confidence", Default::default());
        assert!(!wrong.opportunities[0].before.concerns[0].status.active());
        assert!(
            e::queries(&wrong)
                .iter()
                .all(|r| !matches!(r.decision.selected, q::Action::Ask { concern: 0, .. }))
        );
        assert_eq!(
            wrong.final_state.base.base.base.base.base.events
                [wrong.opportunities.last().unwrap().after.event_end]
                .decision
                .selected,
            Action::Leave
        );
    }
}
#[test]
fn observer_validation_rejects_altered_projection_missing_capture_and_scores() {
    let t = e::trial(
        42,
        v::Mode::StatusValue,
        "uncertain_minor",
        Default::default(),
    );
    let mut bad = t.final_state.clone();
    bad.value.records[0].basis[0].support += 1;
    assert!(bad.validate().is_err());
    let mut bad = t.final_state.clone();
    bad.value.records.remove(0);
    assert!(bad.validate().is_err());
    let mut bad = t.final_state.clone();
    bad.value.records[0].assessment_end += 1;
    assert!(bad.validate().is_err());
    let mut bad = t.final_state;
    bad.base.base.base.base.inquiry.records[0]
        .decision
        .candidates[1]
        .score += 1;
    assert!(bad.validate().is_err());
}
#[test]
fn held_out_extreme_seed_replays_exactly_without_capacity_expansion() {
    let trials = e::suite(u64::MAX, true);
    assert_eq!(trials, e::suite(u64::MAX, true));
    for t in trials {
        assert!(
            t.final_state
                .base
                .base
                .assessment
                .unwrap()
                .items
                .values()
                .all(|items| items.len() <= 32)
        );
        assert!(t.persistent.concerns.len() <= 8);
    }
}

#[test]
fn prospective_fixed_mode_enablement_does_not_import_prior_queries() {
    use world_of_individuals::{
        assessment,
        foresight::{Channel, Sensor},
        model::Generator,
        simulation::Simulation,
    };
    let mut g = Generator::new(42);
    let mut sim = Simulation::new((0..2).map(|id| g.agent(id)).collect()).unwrap();
    assert!(sim.enable_inquiry_value(v::Mode::StatusValue).is_err());
    sim.enable_foresight(
        42,
        Sensor {
            reliability: 40,
            channel: Channel::AccurateFixture,
            effort: 0,
            second: None,
        },
    )
    .unwrap();
    sim.enable_concerns().unwrap();
    sim.enable_inquiry().unwrap();
    sim.enable_provenance().unwrap();
    sim.enable_assessment(assessment::Method::Grouped).unwrap();
    sim.inquiry_meeting(&[0, 1]).unwrap();
    assert!(sim.inquiry_value().is_none());
    sim.enable_inquiry_value(v::Mode::StatusValue).unwrap();
    sim.enable_inquiry_value(v::Mode::StatusValue).unwrap();
    assert!(sim.enable_attention(t::Mode::Unchanged).is_err());
    assert!(sim.enable_inquiry_value(v::Mode::CurrentNeed).is_err());
    assert_eq!(sim.inquiry_value().unwrap().first_query, 2);
    assert!(sim.inquiry_value().unwrap().records.is_empty());
    sim.inquiry_meeting(&[0, 1]).unwrap();
    assert_eq!(sim.inquiry_value().unwrap().records.len(), 2);
    assert_eq!(sim.inquiry_value().unwrap().records[0].query, 2);
    let mut old = Simulation::new((0..2).map(|id| g.agent(id)).collect()).unwrap();
    old.enable_foresight(
        42,
        Sensor {
            reliability: 40,
            channel: Channel::AccurateFixture,
            effort: 0,
            second: None,
        },
    )
    .unwrap();
    old.enable_concerns().unwrap();
    old.enable_inquiry().unwrap();
    old.enable_provenance().unwrap();
    old.enable_assessment(assessment::Method::Grouped).unwrap();
    old.enable_attention(t::Mode::Unchanged).unwrap();
    assert!(old.enable_inquiry_value(v::Mode::StatusValue).is_err());
    assert!(old.inquiry_value().is_none());
}

#[test]
fn partial_answers_reprioritize_without_resolving_and_resolved_concerns_leave_candidates() {
    let partial = e::trial(
        42,
        v::Mode::CurrentNeed,
        "partial_answers",
        Default::default(),
    );
    let queries = e::queries(&partial);
    assert!(matches!(
        queries[0].decision.selected,
        q::Action::Ask { concern: 0, .. }
    ));
    assert!(matches!(
        queries[1].decision.selected,
        q::Action::Ask { concern: 1, .. }
    ));
    for (i, o) in partial.opportunities.iter().enumerate() {
        assert!(o.after.basis[i].support > o.before.basis[i].support);
        assert!(o.after.concerns[i].status.active());
    }
    let resolved = e::trial(42, v::Mode::Unchanged, "resolved", Default::default());
    let id = if let q::Action::Ask { concern, .. } = e::queries(&resolved)[0].decision.selected {
        concern
    } else {
        panic!("expected inquiry")
    };
    assert!(
        !resolved.opportunities[0]
            .after
            .concerns
            .iter()
            .find(|c| c.id == id)
            .unwrap()
            .status
            .active()
    );
    assert!(
        e::queries(&resolved)[1]
            .decision
            .candidates
            .iter()
            .all(|c| !matches!(c.action,q::Action::Ask {concern,..} if concern==id))
    );
}

#[test]
fn cheaper_status_value_tie_misses_executed_opportunity_and_inventory_gap_persists() {
    let a = e::trial(
        42,
        v::Mode::Unchanged,
        "same_uncertainty",
        Default::default(),
    );
    let b = e::trial(
        42,
        v::Mode::StatusValue,
        "same_uncertainty",
        Default::default(),
    );
    assert_eq!(a.opportunities[0].before, b.opportunities[0].before);
    let view = world_of_individuals::model::Observation {
        partner: 1,
        last_signal: None,
        amount: 1,
        turns_left: 6,
    };
    let probe = |p: &e::Point| {
        world_of_individuals::behavior::scarcity_policy(&p.agents[0], &view).selected
    };
    assert_eq!(probe(&a.opportunities[0].before), Action::Leave);
    assert_eq!(probe(&b.opportunities[0].before), Action::Leave);
    assert_eq!(probe(&a.opportunities.last().unwrap().after), Action::Offer);
    assert_eq!(probe(&b.opportunities.last().unwrap().after), Action::Leave);
    let first = e::queries(&b)[0];
    let direct = first
        .decision
        .candidates
        .iter()
        .find(|c| {
            matches!(
                c.action,
                q::Action::Ask {
                    concern: 0,
                    source: 1,
                    strategy: q::Strategy::Direct
                }
            )
        })
        .unwrap();
    let evidence = first
        .decision
        .candidates
        .iter()
        .find(|c| {
            matches!(
                c.action,
                q::Action::Ask {
                    concern: 0,
                    source: 1,
                    strategy: q::Strategy::Evidence
                }
            )
        })
        .unwrap();
    assert_eq!(direct.score, evidence.score);
    assert_eq!(first.decision.selected, direct.action);
    for t in [&a, &b] {
        let events = &t.final_state.base.base.base.base.base.events
            [t.opportunities.last().unwrap().after.event_end..];
        assert_eq!(
            events[0].decision.selected,
            probe(&t.opportunities.last().unwrap().after)
        );
    }
    assert_eq!(
        [a.persistent.agents[0].food, a.persistent.agents[1].food],
        [2, 5]
    );
    assert_eq!(
        [b.persistent.agents[0].food, b.persistent.agents[1].food],
        [3, 4]
    );
}
