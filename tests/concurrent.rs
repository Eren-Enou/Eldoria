use world_of_individuals::experiment::concurrent::{self as x, Work};
fn config(case: &str, capacity: usize) -> x::Config {
    x::matrix()
        .into_iter()
        .find(|c| c.case == case && c.capacity == capacity)
        .unwrap()
}
#[test]
fn registration_has_fifty_matched_capacity_pairs() {
    let rows = x::matrix();
    assert_eq!(rows.len(), 100);
    for pair in rows.as_chunks::<2>().0 {
        let mut a = pair[0].clone();
        let mut b = pair[1].clone();
        assert_eq!(a.capacity, 1);
        assert_eq!(b.capacity, 2);
        a.capacity = 0;
        b.capacity = 0;
        assert_eq!(a, b);
    }
}
#[test]
fn independent_resource_lifetimes_and_exact_replay() {
    let t = x::trial(config("paired", 2));
    assert_eq!(t.initial_contexts.completed, [1, 1, 1, 1]);
    assert!(
        t.boundaries[0]
            .choices
            .iter()
            .all(|d| d.input.legacy.query.input.base.scene == 1)
    );
    assert_eq!(t, x::trial(t.config.clone()));
    let starts = t
        .transitions
        .iter()
        .filter(|r| !r.completed && matches!(r.commitment.work, Work::Resource { .. }))
        .collect::<Vec<_>>();
    assert!(starts.iter().any(|a| {
        starts
            .iter()
            .any(|b| a.commitment.id != b.commitment.id && a.tick == b.tick)
    }));
    for start in starts {
        let end = t
            .transitions
            .iter()
            .find(|r| r.completed && r.commitment.id == start.commitment.id)
            .unwrap();
        assert!(end.tick > start.tick && end.tick <= start.commitment.deadline);
        assert_eq!(end.commitment, start.commitment);
    }
    assert_eq!(t.needs.len(), (t.config.horizon / 4) as usize);
    assert!(
        t.needs
            .iter()
            .all(|n| n.after == n.before.map(|h| (h + 5).min(100)))
    );
    assert!(t.final_agents.iter().all(|a| a.memories.len() <= 16));
}
#[test]
fn private_traits_do_not_enter_the_focal_initial_input() {
    let a = x::trial(config("private_helpful", 2));
    let b = x::trial(config("private_cautious", 2));
    let input = |t: x::Trial| {
        t.boundaries
            .into_iter()
            .flat_map(|b| b.choices)
            .find(|c| c.input.legacy.own.id == 0)
            .unwrap()
            .input
    };
    assert_eq!(input(a), input(b));
}

#[test]
fn atomic_query_waits_for_work_and_charges_all_four_actors() {
    let quiet = x::trial(config("inquiry_quiet", 2));
    let conflict = x::trial(config("inquiry_conflict", 2));
    let admitted = |t: &x::Trial| {
        t.admissions
            .iter()
            .find(|a| {
                a.invitation.kind == world_of_individuals::encounters::Kind::Inquiry
                    && a.reason == "Execute"
            })
            .unwrap()
            .tick
    };
    assert!(admitted(&quiet) < admitted(&conflict));
    assert!(
        conflict
            .admissions
            .iter()
            .any(|a| a.reason == "AtomicBlocked")
    );
    let start = quiet
        .transitions
        .iter()
        .find(|r| !r.completed && r.commitment.work == Work::Query)
        .unwrap();
    let end = quiet
        .transitions
        .iter()
        .find(|r| r.completed && r.commitment.id == start.commitment.id)
        .unwrap();
    assert!(end.tick > start.tick + 1);
    for tick in ((quiet.start + 4)..=quiet.end).step_by(4) {
        let n = quiet.needs.iter().find(|n| n.tick == tick).unwrap();
        assert_eq!(n.after, n.before.map(|h| (h + 5).min(100)));
    }
    assert!(
        quiet
            .needs
            .iter()
            .any(|n| start.tick < n.tick && n.tick <= end.tick)
    );
}
