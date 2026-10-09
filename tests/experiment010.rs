use world_of_individuals::{
    attention::Mode, experiment010 as e, inquiry, model::Action, retention::Method,
};
fn run(method: Method, case: &str) -> e::Trial {
    e::trial(42, method, Mode::CurrentNeed, case, e::Config::default())
}
#[test]
fn unchanged_retention_resolution_releases_inquiry_and_persists_real_transfer() {
    let config = e::held_out()[2].clone();
    let trials = [Method::Fifo, Method::Quality, Method::Salient]
        .map(|m| e::trial(42, m, Mode::Unchanged, "redelivery", config.clone()));
    let salient = &trials[2];
    for t in &trials {
        assert_eq!(t.points[0], salient.points[0]);
        let records = &t.final_state.base.base.assessment.as_ref().unwrap().records;
        let deliveries = |t: &e::Trial| {
            t.final_state.base.base.assessment.as_ref().unwrap().records
                [..t.points[3].assessment_end]
                .iter()
                .map(|r| r.incoming.clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(deliveries(t), deliveries(salient));
        assert_eq!(t.points[3].agents[0].trust[&2], -17);
        assert_eq!(
            t.points[3]
                .agents
                .iter()
                .map(|a| a.food)
                .collect::<Vec<_>>(),
            vec![4; 11]
        );
        let fresh = &records[t.points[2].assessment_end];
        assert_ne!(fresh.incoming.receipt, records[0].incoming.receipt);
        assert_eq!(
            fresh.support,
            if t.method == Method::Salient {
                79
            } else if t.method == Method::Fifo {
                55
            } else {
                0
            }
        );
        assert_eq!(
            t.points[3].concerns[0].status.active(),
            t.method != Method::Salient
        );
        let wanted = if t.method == Method::Salient {
            inquiry::Action::Ask {
                concern: 1,
                source: 2,
                strategy: inquiry::Strategy::Evidence,
            }
        } else {
            inquiry::Action::Ask {
                concern: 0,
                source: 1,
                strategy: inquiry::Strategy::Direct,
            }
        };
        assert_eq!(e::selected(t), wanted);
        let events = &t.final_state.base.base.base.base.base.events;
        let resource = &events[t.points[5].event_end..];
        assert_eq!(
            resource[0].decision.selected,
            if t.method == Method::Salient {
                Action::Offer
            } else {
                Action::Leave
            }
        );
        assert_eq!(
            resource.iter().map(|e| e.transferred).sum::<u32>(),
            u32::from(t.method == Method::Salient)
        );
        for event in resource {
            assert_eq!(
                event.balances_before.iter().sum::<u32>(),
                event.balances_after.iter().sum::<u32>()
            );
        }
        assert_eq!(t.consumption.consumed, 1);
        assert_eq!(
            t.points[7].agents[0].food,
            if t.method == Method::Salient { 2 } else { 3 }
        );
        assert_eq!(
            t.points[7].agents[2].food,
            if t.method == Method::Salient { 5 } else { 4 }
        );
        t.final_state.validate().unwrap();
    }
}
#[test]
fn matched_deliveries_local_candidates_and_complete_executed_chain() {
    for case in e::CASES {
        let trials = [
            run(Method::Fifo, case),
            run(Method::Quality, case),
            run(Method::Salient, case),
        ];
        for t in &trials {
            assert_eq!(t.points[0], trials[0].points[0]);
            assert_eq!(
                t.points[0]
                    .concerns
                    .iter()
                    .filter(|c| c.status.active())
                    .count(),
                2
            );
            let records = &t.final_state.base.base.assessment.as_ref().unwrap().records;
            let incoming: Vec<_> = records[..t.points[3].assessment_end]
                .iter()
                .map(|r| r.incoming.clone())
                .collect();
            let base_records = &trials[0]
                .final_state
                .base
                .base
                .assessment
                .as_ref()
                .unwrap()
                .records;
            assert_eq!(
                incoming,
                base_records[..trials[0].points[3].assessment_end]
                    .iter()
                    .map(|r| r.incoming.clone())
                    .collect::<Vec<_>>()
            );
            let inquiry = &t.final_state.base.base.base.base.inquiry;
            let first = inquiry.records[t.points[3].inquiry_end..]
                .iter()
                .find(|r| r.decision.input.owner == 0)
                .unwrap();
            let concerns: std::collections::BTreeSet<_> = first
                .decision
                .candidates
                .iter()
                .filter_map(|c| match c.action {
                    inquiry::Action::Ask { concern, .. } => Some(concern),
                    _ => None,
                })
                .collect();
            if case != "redelivery" {
                assert_eq!(concerns.len(), 2);
            }
            assert_eq!(
                inquiry.records[t.points[3].inquiry_end..t.points[4].inquiry_end]
                    .iter()
                    .filter(|r| r.decision.input.owner == 0)
                    .count(),
                1
            );
            assert!(first.decision.input.concerns.iter().all(|c| c.owner == 0));
            for event in &t.final_state.base.base.base.base.base.events {
                assert_eq!(
                    event.balances_before.iter().sum::<u32>(),
                    event.balances_after.iter().sum::<u32>()
                );
            }
            let before = t.points[6].agents.iter().map(|a| a.food).sum::<u32>();
            assert_eq!(
                before - t.consumption.consumed,
                t.points[7].agents.iter().map(|a| a.food).sum::<u32>()
            );
            assert_eq!(
                t.points[6].agents[1..]
                    .iter()
                    .map(|a| a.food)
                    .collect::<Vec<_>>(),
                t.points[7].agents[1..]
                    .iter()
                    .map(|a| a.food)
                    .collect::<Vec<_>>()
            );
            t.final_state.validate().unwrap();
        }
    }
    let fifo = run(Method::Fifo, "protected");
    let salient = run(Method::Salient, "protected");
    assert_ne!(e::selected(&fifo), e::selected(&salient));
    let first_action = |t: &e::Trial| {
        t.final_state.base.base.base.base.base.events[t.points[5].event_end]
            .decision
            .selected
    };
    assert_eq!(first_action(&fifo), Action::Offer);
    assert_eq!(first_action(&salient), Action::Leave);
    assert_eq!(fifo.points[7].agents[0].food, 2);
    assert_eq!(salient.points[7].agents[0].food, 3);
}
#[test]
fn negative_controls_and_mistakes_are_not_erased() {
    for mode in [Mode::Unchanged, Mode::CurrentNeed] {
        let group: [e::Trial; 3] = [Method::Fifo, Method::Quality, Method::Salient]
            .map(|m| e::trial(42, m, mode, "equivalent", e::Config::default()));
        assert!(
            group
                .iter()
                .all(|t| e::selected(t) == e::selected(&group[0])
                    && t.points[7].agents == group[0].points[7].agents)
        );
    }
    let fifo = run(Method::Fifo, "costly_salience");
    let salient = run(Method::Salient, "costly_salience");
    assert_ne!(e::selected(&fifo), e::selected(&salient));
    assert_eq!(
        fifo.points[7]
            .agents
            .iter()
            .map(|a| a.food)
            .collect::<Vec<_>>(),
        salient.points[7]
            .agents
            .iter()
            .map(|a| a.food)
            .collect::<Vec<_>>()
    );
    let unknown = run(Method::Salient, "late_attribution");
    assert!(unknown.points[4].inquiry_end > unknown.points[3].inquiry_end);
    assert!(unknown.points[5].assessment_end > unknown.points[4].assessment_end);
    assert_eq!(unknown.points[4].inquiry_end, unknown.points[5].inquiry_end);
    assert_eq!(unknown.points[4].basis[0].support, -35);
    assert_eq!(unknown.points[5].basis[0].support, -10);
}
#[test]
fn unavailable_basis_stays_absent_despite_audit_and_redelivery_is_new() {
    let t = run(Method::Fifo, "protected");
    assert_eq!(t.points[3].basis[0].support, 0);
    assert!(
        t.final_state
            .base
            .base
            .assessment
            .as_ref()
            .unwrap()
            .records
            .iter()
            .any(|r| r.incoming.event == t.targets[0] && r.incoming.quality == 40)
    );
    assert_eq!(t.final_state.attention.records[0].basis, t.points[3].basis);
    let t = run(Method::Fifo, "redelivery");
    assert_eq!(t.points[2].basis[0].support, 0);
    assert_eq!(t.points[3].basis[0].support, 55);
    let records = &t.final_state.base.base.assessment.as_ref().unwrap().records;
    assert_ne!(
        records[0].incoming.receipt,
        records[t.points[2].assessment_end].incoming.receipt
    );
    for method in [Method::Fifo, Method::Quality, Method::Salient] {
        assert!(
            run(method, "none_survives").points[3]
                .basis
                .iter()
                .all(|b| b.support == 0)
        );
    }
}
#[test]
fn audit_rejects_missing_reordered_future_boundaries_and_corrupt_scores() {
    let original = run(Method::Salient, "protected").final_state;
    let mut bad = original.clone();
    bad.attention.records[0].basis[0].support = 99;
    assert!(bad.validate().is_err());
    let mut bad = original.clone();
    bad.attention.records.remove(0);
    assert!(bad.validate().is_err());
    let mut bad = original.clone();
    bad.attention.records.swap(0, 1);
    assert!(bad.validate().is_err());
    let mut bad = original.clone();
    bad.attention.records[0].assessment_end += 1;
    assert!(bad.validate().is_err());
    let mut bad = original;
    bad.base.base.base.base.inquiry.records[0]
        .decision
        .candidates[1]
        .score += 1;
    assert!(bad.validate().is_err());
}
#[test]
fn held_out_exact_replay_and_private_input_boundary() {
    for seed in [42, u64::MAX] {
        let trials = e::suite(seed, true);
        assert_eq!(trials, e::suite(seed, true));
        for group in trials.as_chunks::<3>().0 {
            let inputs = |t: &e::Trial| {
                t.final_state.base.base.assessment.as_ref().unwrap().records
                    [..t.points[3].assessment_end]
                    .iter()
                    .map(|r| r.incoming.clone())
                    .collect::<Vec<_>>()
            };
            for t in group {
                assert_eq!(t.points[0], group[0].points[0]);
                assert_eq!(inputs(t), inputs(&group[0]));
            }
        }
        for t in trials {
            for record in t.final_state.attention.records {
                assert!(record.basis.len() <= 8);
                let serialized = serde_json::to_string(&record.basis).unwrap();
                for forbidden in ["root", "seed", "sensor", "future", "actual", "partner"] {
                    assert!(!serialized.contains(forbidden));
                }
            }
        }
    }
}

#[test]
fn future_resource_partner_cannot_select_the_inquiry_and_enable_is_prospective() {
    for method in [Method::Fifo, Method::Quality, Method::Salient] {
        let a = run(method, "protected");
        let b = run(method, "importance");
        let decision = |t: &e::Trial| {
            t.final_state
                .base
                .base
                .base
                .query(world_of_individuals::audit::QueryId(0))
                .unwrap()
                .decision
        };
        assert_eq!(decision(&a), decision(&b));
        assert_ne!(a.resource_partner, b.resource_partner);
    }
    let (mut sim, _) = world_of_individuals::experiment009::setup(42, Method::Fifo, 80, false);
    let before = sim.assessment().unwrap();
    let queries = world_of_individuals::experiment009::snapshot(&sim)
        .base
        .base
        .provenance
        .queries
        .len();
    sim.enable_attention(Mode::CurrentNeed).unwrap();
    sim.enable_attention(Mode::CurrentNeed).unwrap();
    assert!(sim.enable_attention(Mode::Unchanged).is_err());
    assert_eq!(before, sim.assessment().unwrap());
    assert_eq!(sim.attention().unwrap().first_query, queries);
    sim.inquiry_meeting(&[0, 1]).unwrap();
    e::Snapshot {
        base: world_of_individuals::experiment009::snapshot(&sim),
        attention: sim.attention().unwrap(),
    }
    .validate()
    .unwrap();
}
