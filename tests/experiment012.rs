use world_of_individuals::{experiment012 as e, inquiry as q, self_evaluation as s};

#[test]
fn bounded_learning_is_revisable_and_eviction_does_not_restore_a_cell() {
    let mut state = s::Learning::new(s::Mode::Contextual, 0);
    let (_, first, _) = state.learn(
        0,
        (1, q::Strategy::Direct, s::Context::EvidencePresent),
        0,
        0,
    );
    assert_eq!(first.expected_change, 27);
    let (_, revised, _) = state.learn(
        0,
        (1, q::Strategy::Direct, s::Context::EvidencePresent),
        100,
        1,
    );
    assert_eq!(revised.expected_change, 63);
    for id in 2..35 {
        state.learn(
            0,
            (id, q::Strategy::Direct, s::Context::EvidencePresent),
            0,
            id as u64,
        );
    }
    assert_eq!(state.cells[&0].len(), 32);
    assert!(!state.cells[&0].iter().any(|c| c.source == 1));
    let (before, new, evicted) = state.learn(
        0,
        (1, q::Strategy::Direct, s::Context::EvidencePresent),
        0,
        40,
    );
    assert!(before.is_none() && evicted.is_some());
    assert_eq!(new.expected_change, 27);
}
#[test]
fn controlled_scenarios_use_real_local_updates_and_exact_replay() {
    for case in e::CASES {
        for mode in s::MODES {
            let t = e::trial(42, mode, case, e::Config::default());
            assert_eq!(t, e::trial(42, mode, case, e::Config::default()));
            t.final_state.validate().unwrap();
            assert!(
                t.final_state
                    .learning
                    .cells
                    .values()
                    .all(|v| v.len() <= s::CAPACITY)
            );
            let compact = &t.final_state.base.base.base;
            assert!(
                compact
                    .base
                    .inquiry
                    .records
                    .iter()
                    .all(|r| r.food_before == r.food_after)
            );
            assert_eq!(
                t.after_action.agents.iter().map(|a| a.food).sum::<u32>()
                    - t.persistent.agents.iter().map(|a| a.food).sum::<u32>(),
                t.consumption.consumed
            );
        }
    }
}
#[test]
fn ambiguous_hidden_causes_and_future_partners_do_not_change_local_choices() {
    for mode in s::MODES {
        let ignorance = e::trial(42, mode, "ignorance", e::Config::default());
        let withholding = e::trial(42, mode, "withholding", e::Config::default());
        let a = e::summary(&ignorance, false);
        let b = e::summary(&withholding, false);
        assert_eq!(a["steps"], b["steps"]);
        let current = e::trial(42, mode, "context_specific", e::Config::default());
        let mirror = e::trial(42, mode, "mirrored_future", e::Config::default());
        assert_eq!(
            e::summary(&current, false)["steps"],
            e::summary(&mirror, false)["steps"]
        );
    }
}
#[test]
fn audit_rejects_altered_context_update_and_boundary() {
    let t = e::trial(
        42,
        s::Mode::Contextual,
        "novel_no_progress",
        e::Config::default(),
    );
    let mut corrupted = t.final_state.clone();
    corrupted.learning.records[0].basis[0].context = s::Context::Any;
    assert!(corrupted.validate().is_err());
    let mut corrupted = t.final_state.clone();
    corrupted.learning.records[0]
        .after
        .as_mut()
        .unwrap()
        .expected_change += 1;
    assert!(corrupted.validate().is_err());
    let mut corrupted = t.final_state;
    corrupted.learning.records[0].assessment_before = usize::MAX;
    assert!(corrupted.validate().is_err());
}

#[test]
fn new_learning_transfers_only_matching_observable_context_and_preserves_ties() {
    let t = e::trial(
        42,
        s::Mode::Unchanged,
        "context_specific",
        e::Config::default(),
    );
    let compact = &t.final_state.base.base.base;
    let r = &t.final_state.learning.records[0];
    let reference = compact
        .provenance
        .queries
        .iter()
        .find(|q| q.inquiry.0 == r.inquiry)
        .unwrap();
    let query = compact.query(reference.id).unwrap();
    let baseline = world_of_individuals::provenance::policy(&query.decision.input);
    assert_eq!(
        s::policy(baseline.clone(), s::Mode::Contextual, &r.basis, &[]),
        baseline
    );
    assert_eq!(
        s::policy(baseline.clone(), s::Mode::GlobalProgress, &r.basis, &[]),
        baseline
    );
    let cell = s::Cell {
        source: 1,
        strategy: q::Strategy::Direct,
        context: s::Context::EvidencePresent,
        attempts: 1,
        expected_change: 0,
        last_inquiry: 0,
    };
    let contextual = s::policy(
        baseline.clone(),
        s::Mode::Contextual,
        &r.basis,
        std::slice::from_ref(&cell),
    );
    let b = t.targets[1];
    let b_id = r.basis.iter().find(|x| x.event == b).unwrap().concern;
    for (old, new) in baseline
        .decision
        .candidates
        .iter()
        .zip(&contextual.decision.candidates)
    {
        if matches!(new.action,q::Action::Ask{concern,..} if concern==b_id) {
            assert_eq!(old, new);
        }
    }
    let global_cell = s::Cell {
        context: s::Context::Any,
        ..cell
    };
    let global = s::policy(
        baseline.clone(),
        s::Mode::GlobalProgress,
        &r.basis,
        &[global_cell],
    );
    assert!(global.decision.candidates.iter().any(|c|matches!(c.action,q::Action::Ask{concern,source:1,strategy:q::Strategy::Direct} if concern==b_id)&&c.score<0));
    let mut tie = baseline;
    for c in &mut tie.decision.candidates {
        c.score = 0;
        c.benefit = c.cost as i32;
    }
    assert_eq!(
        s::policy(tie, s::Mode::Contextual, &r.basis, &[])
            .decision
            .selected,
        q::Action::Pause
    );
}

#[test]
fn forgetting_recomputes_context_without_erasing_separate_learning_or_restoring_content() {
    let t = e::trial(42, s::Mode::Contextual, "forgetting", e::Config::default());
    let p = &t
        .opportunities
        .iter()
        .find(|o| o.phase == "limited")
        .unwrap()
        .before;
    let a = p.basis.iter().find(|b| b.event == t.targets[0]).unwrap();
    assert_eq!(a.support, 0);
    assert_eq!(a.context, s::Context::ClaimFallback);
    assert!(
        p.cells
            .iter()
            .any(|c| c.context == s::Context::EvidencePresent)
    );
    let assessment = t.final_state.base.base.assessment.as_ref().unwrap();
    assert!(
        assessment.records[..p.assessment_end]
            .iter()
            .any(|r| r.evicted.is_some())
    );
    // Prefix is audited for the forgetting check; live scoring has no archive input.
    t.final_state.validate().unwrap();
}

#[test]
fn enablement_is_prospective_fixed_and_excludes_old_projections_in_both_orders() {
    use world_of_individuals::{assessment, foresight, model::Generator, simulation::Simulation};
    let make = || {
        let mut g = Generator::new(42);
        let mut sim = Simulation::new((0..2).map(|id| g.agent(id)).collect()).unwrap();
        sim.enable_foresight(42, foresight::Sensor::default())
            .unwrap();
        sim.enable_concerns().unwrap();
        sim.enable_inquiry().unwrap();
        sim.enable_provenance().unwrap();
        sim.enable_assessment(assessment::Method::Grouped).unwrap();
        sim
    };
    let mut sim = make();
    sim.inquiry_meeting(&[0, 1]).unwrap();
    let old = sim.inquiry().unwrap().records.len();
    sim.enable_self_evaluation(s::Mode::Contextual).unwrap();
    assert_eq!(sim.self_evaluation().unwrap().first_inquiry, old);
    assert!(sim.self_evaluation().unwrap().cells.is_empty());
    assert!(sim.enable_self_evaluation(s::Mode::GlobalProgress).is_err());
    assert!(
        sim.enable_attention(world_of_individuals::attention::Mode::Unchanged)
            .is_err()
    );
    assert!(
        sim.enable_inquiry_value(world_of_individuals::inquiry_value::Mode::Unchanged)
            .is_err()
    );
    let mut sim = make();
    sim.enable_attention(world_of_individuals::attention::Mode::Unchanged)
        .unwrap();
    assert!(sim.enable_self_evaluation(s::Mode::Contextual).is_err());
    let mut sim = make();
    sim.enable_inquiry_value(world_of_individuals::inquiry_value::Mode::Unchanged)
        .unwrap();
    assert!(sim.enable_self_evaluation(s::Mode::Contextual).is_err());
}

#[test]
fn assessment_error_changes_later_inquiry_and_an_executed_persistent_consequence() {
    let old = e::trial(
        42,
        s::Mode::Unchanged,
        "changed_circumstances",
        e::Config::default(),
    );
    let new = e::trial(
        42,
        s::Mode::Contextual,
        "changed_circumstances",
        e::Config::default(),
    );
    assert_eq!(old.opportunities[0].before, new.opportunities[0].before);
    let a = e::summary(&old, false);
    let b = e::summary(&new, false);
    assert_eq!(a["steps"][0]["outcome"]["novelty_value"], 50);
    assert_eq!(
        b["steps"][0]["outcome"]["support_before"],
        b["steps"][0]["outcome"]["support_after"]
    );
    assert_eq!(b["steps"][0]["after"]["expected_change"], 27);
    assert_eq!(a["steps"][1]["selected"]["Ask"]["strategy"], "Direct");
    assert_eq!(b["steps"][1]["selected"]["Ask"]["strategy"], "Evidence");
    assert_eq!(a["actions"], serde_json::json!(["Leave"]));
    assert_eq!(b["actions"], serde_json::json!(["Offer", "Accept"]));
    assert_eq!(
        (old.persistent.agents[0].food, old.persistent.agents[1].food),
        (3, 4)
    );
    assert_eq!(
        (new.persistent.agents[0].food, new.persistent.agents[1].food),
        (2, 5)
    );
}

#[test]
fn context_transfer_helps_one_opportunity_but_misses_a_later_changed_source() {
    let contextual = e::trial(
        42,
        s::Mode::Contextual,
        "context_specific",
        e::Config::default(),
    );
    let global = e::trial(
        42,
        s::Mode::GlobalProgress,
        "context_specific",
        e::Config::default(),
    );
    let a = e::summary(&contextual, false);
    let b = e::summary(&global, false);
    assert_eq!(a["steps"][2]["selected"]["Ask"]["concern"], 1);
    assert_eq!(a["steps"][2]["outcome"]["support_after"], 50);
    assert_eq!(b["steps"][2]["selected"]["Ask"]["concern"], 0);
    let contextual = e::trial(
        42,
        s::Mode::Contextual,
        "later_informed",
        e::Config::default(),
    );
    let global = e::trial(
        42,
        s::Mode::GlobalProgress,
        "later_informed",
        e::Config::default(),
    );
    assert_eq!(
        e::summary(&contextual, false)["actions"],
        serde_json::json!(["Leave"])
    );
    assert_eq!(
        e::summary(&global, false)["actions"],
        serde_json::json!(["Offer", "Accept"])
    );
    assert!(
        contextual
            .final_state
            .base
            .base
            .base
            .base
            .inquiry
            .records
            .iter()
            .all(|r| r.response.as_ref().is_none_or(|r| r.valid))
    );
}
