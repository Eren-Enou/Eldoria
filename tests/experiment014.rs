use world_of_individuals::{activity as a, experiment014 as e};
#[test]
fn controlled_worlds_replay_and_have_real_bounded_time_and_food() {
    for case in e::CASES {
        for mode in a::MODES {
            let t = e::trial(42, mode, case, e::Config::default());
            assert_eq!(t, e::trial(42, mode, case, e::Config::default()));
            for p in &t.periods {
                assert_eq!(p.after.tick, p.end + 3);
                for r in &p.records {
                    assert!(r.after.tick <= p.end);
                    assert!(r.after.tick >= r.before.tick);
                }
            }
            let initial = t.initial.agents.iter().map(|a| a.food).sum::<u32>();
            let final_food = t
                .periods
                .last()
                .unwrap()
                .after
                .agents
                .iter()
                .map(|a| a.food)
                .sum::<u32>();
            assert_eq!(
                initial,
                final_food
                    + t.periods
                        .iter()
                        .flat_map(|p| &p.consumption)
                        .map(|c| c.consumed)
                        .sum::<u32>()
            );
        }
    }
}
#[test]
fn expired_and_reused_activities_never_execute_or_move_time_backwards() {
    let (mut sim, _) = e::setup(42, a::Mode::Local, "competition", &e::Config::default());
    sim.activity_open(0, Some(2), Some(1), 40, 40, 8).unwrap();
    sim.activity_execute(a::Action::Inquiry).unwrap();
    let before = sim.activity_tick();
    assert!(sim.activity_execute(a::Action::Inquiry).is_err());
    assert!(sim.activity_execute(a::Action::Resource).is_err());
    assert_eq!(sim.activity_tick(), before);
    sim.activity_execute(a::Action::Wait).unwrap();
    sim.activity_finish().unwrap();
    let before = sim.activity_tick();
    assert!(sim.activity_execute(a::Action::Wait).is_err());
    assert!(sim.activity_finish().is_err());
    assert_eq!(sim.activity_tick(), before);
}
#[test]
fn resource_resolution_cannot_launch_unbudgeted_automatic_queries() {
    let (mut sim, _) = e::setup(42, a::Mode::Local, "material_first", &e::Config::default());
    sim.activity_open(0, Some(2), Some(1), 40, 40, 8).unwrap();
    let q = sim.inquiry().unwrap().records.len();
    let spent = sim.activity_execute(a::Action::Resource).unwrap();
    assert!(spent <= 6);
    assert_eq!(sim.inquiry().unwrap().records.len(), q);
    assert!(sim.events().iter().any(|e| e.transferred == 1));
}
#[test]
fn private_partner_differences_do_not_choose_the_first_activity() {
    for mode in a::MODES {
        let a = e::trial(42, mode, "material_first", e::Config::default());
        let b = e::trial(42, mode, "mirrored", e::Config::default());
        assert_eq!(
            a.periods[0].records[0].decision,
            b.periods[0].records[0].decision
        );
    }
}
#[test]
fn conservative_reservation_rejects_short_remaining_time_without_free_execution() {
    let (mut sim, _) = e::setup(
        42,
        a::Mode::Local,
        "information_first",
        &e::Config::default(),
    );
    sim.activity_open(0, Some(2), Some(1), 15, 15, 15).unwrap();
    sim.activity_execute(a::Action::Inquiry).unwrap();
    let input = sim.activity_input().unwrap();
    let w = input
        .windows
        .iter()
        .find(|w| w.action == a::Action::Resource)
        .unwrap();
    assert!(!a::eligible(&input, w));
    let events = sim.events().len();
    assert!(sim.activity_execute(a::Action::Resource).is_err());
    assert_eq!(sim.events().len(), events);
}

#[test]
fn unknown_reply_willingness_and_duration_cannot_enter_current_choice() {
    use world_of_individuals::intentional::Profile;
    let (mut left, _) = e::setup(
        42,
        a::Mode::Local,
        "information_first",
        &e::Config::default(),
    );
    let (mut right, _) = e::setup(
        42,
        a::Mode::Local,
        "information_first",
        &e::Config::default(),
    );
    right
        .set_communication_profile(
            2,
            Profile {
                privacy: 100,
                relationship_goal: 0,
                ..Default::default()
            },
        )
        .unwrap();
    for sim in [&mut left, &mut right] {
        sim.activity_open(0, Some(2), Some(1), 40, 40, 25).unwrap();
    }
    assert_eq!(
        left.activity_input().unwrap(),
        right.activity_input().unwrap()
    );
    assert_eq!(left.activity_execute(a::Action::Inquiry).unwrap(), 13);
    assert_eq!(right.activity_execute(a::Action::Inquiry).unwrap(), 12);
    assert_eq!(
        left.assessment().unwrap().items[&0].len(),
        right.assessment().unwrap().items[&0].len() + 1
    );
}
