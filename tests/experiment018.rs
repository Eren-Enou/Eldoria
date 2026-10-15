use world_of_individuals::{
    encounters as e, experiment018 as f, opportunities as o, recurring as r,
};

#[test]
fn elapsed_phase_chunking_and_saturation_do_not_count_evaluations() {
    for interval in [2, 4, 16] {
        for start in [0, 41] {
            for hunger in [0, 20, 95, 100] {
                let mut clock = r::Clock {
                    epoch: start,
                    last: start,
                    settings: r::Settings {
                        interval,
                        increment: 5,
                    },
                };
                let mut h = hunger;
                for tick in start + 1..=start + 101 {
                    h = r::pressure(h, clock.pulses(tick), 5);
                    clock.last = tick;
                }
                let initial = r::Clock {
                    last: start,
                    ..clock
                };
                assert_eq!(h, r::pressure(hunger, initial.pulses(start + 101), 5));
                assert_eq!(initial.pulses(start), 0);
                assert_eq!(r::pressure(hunger, u64::MAX, 5), 100);
                assert_eq!(r::pressure(hunger, u64::MAX, 0), hunger);
            }
        }
    }
}
#[test]
fn real_steps_have_consistent_need_time_and_finite_food() {
    for mode in r::MODES {
        for case in [
            "recurring",
            "no_food",
            "competing_eater",
            "information",
            "misleading",
            "ongoing",
        ] {
            let t = f::trial(
                42,
                mode,
                case,
                f::Config {
                    periods: 2,
                    long_periods: 2,
                    ..Default::default()
                },
            );
            let _ = f::summary(&t);
            assert_eq!(t.clock.last, t.initial.state.tick + 48);
            assert_eq!(t.clock.epoch, t.initial.state.tick);
            let mut last = t.clock.epoch;
            for n in &t.needs {
                assert_eq!(n.from, last);
                assert!(n.to > n.from);
                assert_eq!(
                    n.pulses,
                    (n.to - t.clock.epoch) / t.config.interval
                        - (n.from - t.clock.epoch) / t.config.interval
                );
                assert_eq!(
                    n.after,
                    n.before.map(|h| r::pressure(h, n.pulses, mode.increment()))
                );
                last = n.to;
            }
            assert_eq!(last, t.clock.last);
            for p in &t.periods {
                assert_eq!(p.end - p.start, 24);
                assert!(p.steps.last().unwrap().activity.active_after.is_none());
                for st in &p.steps {
                    let s = &st.activity;
                    let a = &t.points[s.before];
                    let b = &t.points[s.after];
                    assert!(a.state.tick < b.state.tick && b.state.tick <= p.end);
                    for c in &s.consumption {
                        assert_eq!(c.consumed, 1);
                        assert_eq!(c.food_after + 1, c.food_before);
                        assert_eq!(c.hunger_after, (c.hunger_before - 40).max(0));
                        assert!(s.people[c.actor as usize].available);
                    }
                    if let Some(c) = &s.active_before {
                        for id in [c.invitation.proposer, c.invitation.recipient] {
                            assert!(!s.people[id as usize].available);
                            assert!(s.decisions[id as usize].is_none());
                        }
                    }
                }
            }
        }
    }
}
#[test]
fn no_food_cannot_manufacture_consumption_and_scheduling_alone_is_null() {
    for seed in [1, 42] {
        let config = f::Config {
            periods: 2,
            ..Default::default()
        };
        let a = f::trial(seed, r::Mode::Serial, "no_food", config.clone());
        let b = f::trial(seed, r::Mode::Recurring, "no_food", config);
        assert_eq!(a.final_state, b.final_state);
        assert_eq!(f::summary(&a).consumed, 0);
        assert_eq!(f::summary(&b).consumed, 0);
        assert!(b.needs.iter().any(|n| n.after != n.before));
    }
}
#[test]
fn local_priority_and_history_mask_preserve_exact_old_candidates() {
    let mut s = f::setup(42, "ongoing", &Default::default());
    s.recurring_open(r::Mode::Recurring, e::Order::Ascending, 24, 4)
        .unwrap();
    let people = (0..3)
        .map(|id| e::Public {
            id,
            available: true,
        })
        .collect::<Vec<_>>();
    let i = s.encounter_input(0, &people, 24).unwrap();
    for mode in r::MODES {
        let d = r::policy(&i, mode);
        let old = o::policy(&i, mode.mapped());
        assert_eq!(d.contact, old.contact);
        assert_eq!(d.candidates, old.candidates);
        if mode == r::Mode::EatFirst {
            assert_eq!(d.selected, o::Action::Consume);
        } else {
            assert_eq!(d, old);
        }
    }
}
#[test]
fn observer_exports_do_not_restore_cognition_or_supply_physics() {
    let mut a = f::setup(42, "information", &Default::default());
    let mut b = f::setup(42, "information", &Default::default());
    for s in [&mut a, &mut b] {
        s.recurring_open(r::Mode::Recurring, e::Order::Ascending, 24, 4)
            .unwrap();
    }
    let mut clean = vec![];
    let mut fake = vec![a.recurring_point()];
    fake[0].state.agents[0].trust.insert(2, 100);
    fake[0].state.agents[0].hunger = 100;
    let x = a.recurring_step(&mut clean).unwrap();
    let y = b.recurring_step(&mut fake).unwrap();
    assert_eq!(x.activity.decisions, y.activity.decisions);
    assert_eq!(x.activity.responses, y.activity.responses);
    assert_eq!(a.agents(), b.agents());
    assert_eq!(a.recurring_records(), b.recurring_records());
}
#[test]
fn hidden_partner_profile_cannot_choose_focal_activity() {
    for mode in r::MODES {
        let mut a = f::setup(42, "information", &Default::default());
        let mut b = f::setup(42, "private_mirror", &Default::default());
        for s in [&mut a, &mut b] {
            s.recurring_open(mode, e::Order::Ascending, 24, 4).unwrap();
        }
        let x = a.recurring_step(&mut vec![]).unwrap();
        let y = b.recurring_step(&mut vec![]).unwrap();
        assert_eq!(x.activity.decisions[0], y.activity.decisions[0]);
    }
}
#[test]
fn physical_settings_and_treatment_cannot_reset_at_period_boundaries() {
    let mut s = f::setup(42, "recurring", &Default::default());
    assert!(
        s.recurring_open(r::Mode::Recurring, e::Order::Ascending, 24, 0)
            .is_err()
    );
    s.recurring_open(r::Mode::Recurring, e::Order::Ascending, 2, 4)
        .unwrap();
    assert!(
        s.recurring_open(r::Mode::Recurring, e::Order::Ascending, 24, 4)
            .is_err()
    );
    let mut points = vec![];
    while s.activity_tick() < 2 {
        s.recurring_step(&mut points).unwrap();
    }
    s.recurring_finish().unwrap();
    assert!(
        s.recurring_open(r::Mode::Recurring, e::Order::Ascending, 24, 16)
            .is_err()
    );
    assert!(
        s.recurring_open(r::Mode::Fixed, e::Order::Ascending, 24, 4)
            .is_err()
    );
    s.recurring_open(r::Mode::Recurring, e::Order::Ascending, 24, 4)
        .unwrap();
    assert_eq!(s.recurring_clock().unwrap().epoch, 0);
}
#[test]
fn existing_inquiry_remains_consented_and_batch_time_advances_everyones_need() {
    for interval in [2, 4, 16] {
        for mode in r::MODES {
            let t = f::trial(
                42,
                mode,
                "information",
                f::Config {
                    periods: 2,
                    interval,
                    ..Default::default()
                },
            );
            for p in &t.periods {
                for st in &p.steps {
                    let s = &st.activity;
                    if s.query.is_some() {
                        let a = t.points[s.admitted.unwrap()].state.tick;
                        let b = t.points[s.after].state.tick;
                        assert!(b > a + 1);
                        assert!(
                            t.needs[st.need_before..st.need_after]
                                .iter()
                                .any(|n| n.to - n.from >= 10)
                        );
                        assert!(
                            s.allocations
                                .iter()
                                .any(|v| v.disposition == e::Disposition::Execute
                                    && v.invitation.kind == e::Kind::Inquiry)
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn received_information_cannot_overwrite_elapsed_need_progression() {
    let t = f::trial(
        42,
        r::Mode::EatFirst,
        "information",
        f::Config {
            periods: 2,
            interval: 2,
            ..Default::default()
        },
    );
    let final_agents = &t.points[t.periods.last().unwrap().after].state.agents;
    for (id, final_agent) in final_agents.iter().enumerate() {
        let added: i32 = t.needs.iter().map(|n| n.after[id] - n.before[id]).sum();
        let reduced: i32 = t
            .periods
            .iter()
            .flat_map(|p| &p.steps)
            .flat_map(|s| &s.activity.consumption)
            .filter(|c| c.actor == id as u32)
            .map(|c| c.hunger_before - c.hunger_after)
            .sum();
        assert_eq!(
            final_agent.hunger,
            t.initial.state.agents[id].hunger + added - reduced,
            "all pressure and consumption must persist for actor{id}"
        );
    }
}
