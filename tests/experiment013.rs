use world_of_individuals::{experiment013 as e, self_evaluation as s};

#[test]
fn all_controlled_families_replay_with_fixed_bounds_and_continuous_resources() {
    for case in e::CASES {
        for mode in e::MODES {
            let t = e::trial(42, mode, case, e::Config::default());
            assert_eq!(t, e::trial(42, mode, case, e::Config::default()));
            t.final_state.validate().unwrap();
            assert!(t.final_state.learning.cells.values().all(|c| c.len() <= 32));
            let mut food = vec![4; 7];
            for r in t.resources {
                assert_eq!(
                    r.actors_before.iter().map(|a| a.food).collect::<Vec<_>>(),
                    food
                );
                let before = r.actors_before.iter().map(|a| a.food).sum::<u32>();
                let after = r.actors_after.iter().map(|a| a.food).sum::<u32>();
                assert_eq!(before, after);
                assert_eq!(
                    after - r.persistent_food.iter().sum::<u32>(),
                    r.consumption.consumed
                );
                food = r.persistent_food;
            }
        }
    }
}
#[test]
fn locally_identical_private_improvement_does_not_select_the_next_question() {
    for mode in e::MODES {
        let left = e::trial(42, mode, "later_useful", e::Config::default());
        let right = e::trial(42, mode, "hidden_twin", e::Config::default());
        assert_eq!(left.opportunities[..8], right.opportunities[..8]);
        let selected = |t: &e::Trial| {
            let o = &t.opportunities[8];
            t.final_state.base.base.base.base.inquiry.records
                [o.before.inquiry_end..o.after.inquiry_end]
                .iter()
                .find(|r| r.decision.input.owner == 0)
                .unwrap()
                .decision
                .clone()
        };
        assert_eq!(selected(&left), selected(&right));
        let mirror = e::trial(42, mode, "mirrored_future", e::Config::default());
        assert_eq!(left.opportunities[..16], mirror.opportunities[..16]);
    }
}
#[test]
fn earlier_discovery_changes_execution_and_persists_without_rewriting_learning() {
    let old = e::trial(
        42,
        e::Mode::Contextual,
        "later_useful",
        e::Config::default(),
    );
    let new = e::trial(42, e::Mode::Pulse, "later_useful", e::Config::default());
    let a = e::summary(&old, false);
    let b = e::summary(&new, false);
    assert_eq!(a["actions"][0], serde_json::json!(["Leave"]));
    assert_eq!(b["actions"][0], serde_json::json!(["Offer", "Accept"]));
    assert_eq!(
        a["persistent_food"],
        serde_json::json!([2, 5, 4, 4, 4, 4, 4])
    );
    assert_eq!(
        b["persistent_food"],
        serde_json::json!([1, 6, 4, 4, 4, 4, 4])
    );
    assert_eq!(new.final_state.learning.mode, s::Mode::Contextual);
    assert!(new.final_state.learning.records.iter().any(|r| {
        r.before.as_ref().is_some_and(|c| c.expected_change < 20)
            && r.outcome.as_ref().is_some_and(|o| o.changed())
    }));
}
#[test]
fn unchanged_world_preserves_costly_reconsideration_and_ungated_probing() {
    let count = |mode| {
        let t = e::trial(42, mode, "unchanged", e::Config::default());
        let summary = e::summary(&t, false);
        summary["steps"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| r["selected"] != "Pause")
            .count()
    };
    assert!(count(e::Mode::Pulse) > count(e::Mode::Contextual));
    assert_eq!(count(e::Mode::Ungated), 24);
}
#[test]
fn long_history_has_actual_gain_loss_signals_and_never_resets_world() {
    let t = e::trial(42, e::Mode::Pulse, "longitudinal", e::Config::default());
    assert_eq!(t.opportunities.len(), 48);
    assert!(
        t.changes
            .iter()
            .any(|c| c.source == 4 && c.kind == "acquire")
    );
    assert!(
        t.changes
            .iter()
            .any(|c| c.source == 5 && c.kind == "acquire")
    );
    assert!(t.changes.iter().any(|c| c.kind.contains("eviction")));
    assert_eq!(t.resources.len(), 3);
    t.final_state.validate().unwrap();
}
#[test]
fn evicted_basis_is_not_restored_from_observer_records_and_corruptions_fail() {
    let t = e::trial(42, e::Mode::Pulse, "forgetting", e::Config::default());
    let first = &t.opportunities[8].before;
    let a = first
        .basis
        .iter()
        .find(|b| b.event == t.targets[0])
        .unwrap();
    assert_eq!(a.support, 0);
    assert!(
        first
            .cells
            .iter()
            .any(|c| c.context == s::Context::EvidencePresent)
    );
    let mut corrupted = t.final_state.clone();
    corrupted.learning.records[0].basis[0].support += 1;
    assert!(corrupted.validate().is_err());
    let mut corrupted = t.final_state;
    corrupted.mode = e::Mode::Unchanged;
    assert!(corrupted.validate().is_err());
}
