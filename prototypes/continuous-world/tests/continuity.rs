use world_of_individuals::{continuous as c, experiment::concurrent::Work};
fn configuration(case: &str) -> c::Config {
    c::matrix()
        .into_iter()
        .find(|v| v.case == case && v.mode == c::Mode::Continuous && v.role == "controlled")
        .unwrap()
}
#[test]
fn reporting_period_is_not_an_actor_or_physical_intervention() {
    for case in ["paired", "inquiry_quiet"] {
        let first = configuration(case);
        let mut second = first.clone();
        second.period = 17;
        let a = c::trial(first);
        let b = c::trial(second);
        assert_eq!(
            a.history, b.history,
            "reporting changed actual history for {case}"
        );
        assert_ne!(a.checkpoints, b.checkpoints);
    }
}
#[test]
fn atomic_query_spans_nominal_stop_without_new_poststop_choices() {
    let mut config = configuration("inquiry_quiet");
    config.horizon = 3;
    let t = c::trial(config);
    let cutoff = t.history.start + 3;
    let nominal = t
        .checkpoints
        .iter()
        .find(|r| r.labels.iter().any(|l| l == "NominalStop"))
        .unwrap();
    assert!(!nominal.active.is_empty());
    let query = t
        .history
        .transitions
        .iter()
        .find(|tr| !tr.completed && tr.commitment.work == Work::Query)
        .unwrap();
    assert!(query.commitment.start < cutoff);
    let finish = t
        .history
        .transitions
        .iter()
        .find(|tr| tr.completed && tr.commitment.id == query.commitment.id)
        .unwrap();
    assert!(finish.tick > cutoff);
    assert!(
        t.history
            .boundaries
            .iter()
            .filter(|b| !b.choices.is_empty())
            .all(|b| b.tick < cutoff)
    );
    assert_eq!(t.history.end, cutoff + c::DRAIN);
    assert!(t.checkpoints.last().unwrap().active.is_empty());
}
#[test]
fn long_world_keeps_one_clock_and_bounded_owned_memory() {
    let config = configuration("exhaustion");
    let t = c::trial(config);
    assert_eq!(t.history.end - t.history.start, 384 + c::DRAIN);
    assert!(
        t.history
            .boundaries
            .windows(2)
            .all(|w| w[0].tick < w[1].tick)
    );
    assert!(
        t.history
            .boundaries
            .iter()
            .flat_map(|b| &b.agents)
            .all(|a| a.memories.len() <= 32)
    );
    assert_eq!(t.history.final_memory.len(), 4);
    let mut live = std::collections::BTreeMap::new();
    for tr in &t.history.transitions {
        if tr.completed {
            assert_eq!(live.remove(&tr.commitment.id), Some(tr.commitment.clone()));
        } else {
            live.insert(tr.commitment.id, tr.commitment.clone());
        }
        let owners: Vec<_> = live.values().flat_map(|c| &c.participants).collect();
        assert_eq!(
            owners.len(),
            owners
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len()
        );
        assert!(live.len() <= 4);
    }
    assert!(live.is_empty());
}
