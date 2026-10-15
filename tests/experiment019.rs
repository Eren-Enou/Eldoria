use world_of_individuals::{
    encounters as e, experiment018 as old, experiment019 as f, opportunities as o, priorities as p,
};
fn input(budget: u64) -> e::Input {
    let mut s = f::setup(42, "useful_contact", &Default::default());
    s.recurring_open(p::Mode::Baseline.legacy(), e::Order::Ascending, budget, 4)
        .unwrap();
    let people = (0..3)
        .map(|id| e::Public {
            id,
            available: true,
        })
        .collect::<Vec<_>>();
    s.encounter_input(0, &people, s.activity_tick() + budget)
        .unwrap()
}
#[test]
fn public_deadline_factor_has_an_exact_eatfirst_ablation() {
    let i = input(8);
    let base = p::policy(&i, p::Mode::Baseline);
    let eat = p::policy(&i, p::Mode::EatFirst);
    let deadline = p::policy(&i, p::Mode::Deadline);
    assert!(base.candidates[1].eligible && base.candidates[2].eligible);
    assert_eq!(eat.selected, o::Action::Consume);
    assert_eq!(deadline.selected, o::Action::Invite);
    assert_eq!(eat.contact, deadline.contact);
    assert_eq!(eat.candidates, deadline.candidates);
    assert_eq!(
        p::policy(&input(9), p::Mode::Deadline).selected,
        o::Action::Consume
    );
    assert!(!p::room_after_meal(u64::MAX, u64::MAX, 8));
}
#[test]
fn sensitivity_preserves_contact_and_public_reservations() {
    let i = input(24);
    let base = p::policy(&i, p::Mode::Baseline);
    for (mode, multiplier) in [(p::Mode::Scale90, 9), (p::Mode::Scale110, 11)] {
        let d = p::policy(&i, mode);
        assert_eq!(d.contact, base.contact);
        assert_eq!(&d.candidates[..2], &base.candidates[..2]);
        assert_eq!(
            d.candidates[2].score,
            base.candidates[2].score * multiplier / 10
        );
        assert_eq!(d, (mode.selector())(&i, mode.legacy()));
    }
}
#[test]
fn historical_controls_exactly_reproduce018_without_sampler_changes() {
    for case in old::CASES {
        for mode in [p::Mode::Baseline, p::Mode::EatFirst, p::Mode::NoHistory] {
            let a = f::trial(42, mode, case, Default::default());
            let b = old::trial(42, mode.legacy(), case, Default::default());
            assert_eq!(
                (
                    &a.initial,
                    &a.profiles,
                    &a.points,
                    &a.periods,
                    &a.final_state,
                    &a.clock,
                    &a.needs
                ),
                (
                    &b.initial,
                    &b.profiles,
                    &b.points,
                    &b.periods,
                    &b.final_state,
                    &b.clock,
                    &b.needs
                ),
                "{case}/{mode:?}"
            );
            assert!(a.pauses.is_empty());
        }
    }
}
#[test]
fn no_food_pressure_and_area_are_independent_of_policy_frequency() {
    for mode in p::MODES {
        let t = f::trial(42, mode, "no_food", Default::default());
        let s = f::summary(&t);
        assert_eq!(s.final_food, vec![0; 3]);
        assert_eq!(s.final_hunger, vec![100; 3]);
        assert_eq!(s.hunger_area, [10200; 3]);
        assert_eq!(s.consumed, 0);
    }
}
#[test]
fn sampler_advances_real_time_without_fabricating_decisions() {
    let t = f::trial(
        42,
        p::Mode::Baseline,
        "recurring",
        f::Config {
            idle_gap: 5,
            ..Default::default()
        },
    );
    let x = &t.pauses[0];
    assert_eq!((x.from, x.to), (1, 5));
    assert_eq!(
        t.points[x.after]
            .state
            .agents
            .iter()
            .map(|a| a.hunger)
            .collect::<Vec<_>>(),
        vec![5; 3]
    );
    assert_eq!(x.need_after, x.need_before + 1);
    assert_eq!(t.clock.last - t.clock.epoch, 144);
    assert_eq!(
        f::summary(&t).idle_ticks,
        t.periods
            .iter()
            .flat_map(|p| &p.steps)
            .filter(|w| w.activity.active_before.is_none()
                && w.activity
                    .decisions
                    .iter()
                    .flatten()
                    .all(|d| d.selected == o::Action::Wait))
            .map(|w| t.points[w.activity.after].state.tick - t.points[w.activity.before].state.tick)
            .sum::<u64>()
            + t.pauses.iter().map(|p| p.to - p.from).sum::<u64>()
    );
}
#[test]
fn observation_jump_cannot_cross_active_work_or_a_deadline() {
    let mut s = f::setup(42, "ongoing", &Default::default());
    assert!(s.priorities_idle(1).is_err());
    s.recurring_open(p::Mode::Baseline.legacy(), e::Order::Ascending, 24, 4)
        .unwrap();
    assert!(s.priorities_idle(25).is_err());
    let mut points = vec![];
    let step = s.recurring_step(&mut points).unwrap();
    assert!(step.activity.started.is_some());
    let before = s.recurring_point();
    assert!(s.priorities_idle(1).is_err());
    assert_eq!(before, s.recurring_point());
}
#[test]
fn registered_matrix_and_local_only_projection_are_deterministic() {
    assert_eq!(
        f::configurations(false)
            .iter()
            .map(|(_, c)| c.len())
            .sum::<usize>(),
        19
    );
    assert_eq!(
        f::configurations(true)
            .iter()
            .map(|(_, c)| c.len())
            .sum::<usize>(),
        35
    );
    let i = input(24);
    let mut other = f::setup(42, "useful_contact", &Default::default());
    other.set_circumstances(2, 100, 100, 100, 0).unwrap();
    let people = (0..3)
        .map(|id| e::Public {
            id,
            available: true,
        })
        .collect::<Vec<_>>();
    other
        .recurring_open(p::Mode::Baseline.legacy(), e::Order::Ascending, 24, 4)
        .unwrap();
    let j = other
        .encounter_input(0, &people, other.activity_tick() + 24)
        .unwrap();
    assert_eq!(i, j);
    for mode in p::MODES {
        assert_eq!(p::policy(&i, mode), p::policy(&j, mode));
    }
}
