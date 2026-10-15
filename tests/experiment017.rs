use world_of_individuals::{encounters as e, experiment017 as f, opportunities as o};

#[test]
fn independent_resource_steps_conserve_and_release_actual_commitments() {
    for mode in o::MODES {
        for case in [
            "mutual",
            "competing_eater",
            "information",
            "separate",
            "formerly_useful",
        ] {
            let t = f::trial(
                42,
                mode,
                case,
                f::Config {
                    periods: 2,
                    ..Default::default()
                },
            );
            assert_eq!(
                t,
                f::trial(
                    42,
                    mode,
                    case,
                    f::Config {
                        periods: 2,
                        ..Default::default()
                    }
                )
            );
            let _ = f::summary(&t);
            for p in &t.periods {
                for s in &p.steps {
                    let a = &t.points[s.before].state;
                    let b = &t.points[s.after].state;
                    assert!(a.tick < b.tick && b.tick <= p.end);
                    if let Some(c) = &s.active_before {
                        assert_eq!(b.tick, a.tick + 1);
                        for owner in [c.invitation.proposer, c.invitation.recipient] {
                            assert!(!s.people[owner as usize].available);
                            assert!(s.decisions[owner as usize].is_none());
                            assert!(!s.consumption.iter().any(|x| x.actor == owner));
                        }
                    }
                }
            }
        }
    }
}
#[test]
fn actor_policy_uses_current_local_inputs_and_stable_ties() {
    let mut s = f::setup(42, "competing_eater");
    let now = s.activity_tick();
    s.opportunity_open(o::Mode::Active, e::Order::Ascending, 24)
        .unwrap();
    let public = vec![
        e::Public {
            id: 0,
            available: false,
        },
        e::Public {
            id: 1,
            available: false,
        },
        e::Public {
            id: 2,
            available: true,
        },
    ];
    let i = s.encounter_input(2, &public, now + 24).unwrap();
    let d = o::policy(&i, o::Mode::Active);
    assert_eq!(d.selected, o::Action::Consume);
    assert_eq!(o::policy(&i, o::Mode::Committed).selected, o::Action::Wait);
    assert_eq!(d.outgoing().selected, e::Intent::Wait);
    let mut nofood = i.clone();
    nofood.own.food = 0;
    assert_eq!(
        o::policy(&nofood, o::Mode::Active).selected,
        o::Action::Wait
    );
}
#[test]
fn observer_points_cannot_supply_cognition_or_change_decisions() {
    let mut a = f::setup(42, "information");
    let mut b = f::setup(42, "information");
    for s in [&mut a, &mut b] {
        s.opportunity_open(o::Mode::Active, e::Order::Ascending, 24)
            .unwrap();
    }
    let mut clean = vec![];
    let mut unrelated = vec![a.opportunity_point()];
    unrelated[0].state.agents[0].trust.insert(2, 100);
    let x = a.opportunity_step(&mut clean).unwrap();
    let y = b.opportunity_step(&mut unrelated).unwrap();
    assert_eq!(x.decisions, y.decisions);
    assert_eq!(x.responses, y.responses);
    assert_eq!(a.agents(), b.agents());
}
#[test]
fn hidden_recipient_information_cannot_select_current_focal_activity() {
    let mut a = f::setup(42, "information");
    let mut b = f::setup(42, "private_mirror");
    for s in [&mut a, &mut b] {
        s.opportunity_open(o::Mode::Active, e::Order::Ascending, 24)
            .unwrap();
    }
    let x = a.opportunity_step(&mut vec![]).unwrap();
    let y = b.opportunity_step(&mut vec![]).unwrap();
    assert_eq!(x.decisions[0], y.decisions[0]);
}
#[test]
fn admission_bounds_no_scripted_busy_and_no_commitment_crosses_period() {
    for budget in [2, 8, 16, 24, 48] {
        let t = f::trial(
            1,
            o::Mode::Active,
            "competing_eater",
            f::Config {
                budget,
                periods: 2,
                ..Default::default()
            },
        );
        for p in &t.periods {
            assert!(p.steps.first().unwrap().people.iter().all(|x| x.available));
            assert!(p.steps.last().unwrap().active_after.is_none());
        }
    }
}

#[test]
fn commitments_without_independent_activity_preserve_serial_outcomes() {
    for case in [
        "mutual",
        "contended",
        "information",
        "refusal",
        "formerly_useful",
    ] {
        let config = f::Config {
            periods: 2,
            ..Default::default()
        };
        let a = f::trial(42, o::Mode::Serial, case, config.clone());
        let b = f::trial(42, o::Mode::Committed, case, config);
        assert_eq!(a.final_state, b.final_state);
        assert_eq!(
            a.points[a.periods.last().unwrap().after],
            b.points[b.periods.last().unwrap().after]
        );
    }
}

#[test]
fn serial_contact_reuses_protected016_control_execution() {
    use world_of_individuals::{contact_learning as c, experiment016 as old};
    for case in ["mutual", "contended", "information", "refusal", "separate"] {
        let a = f::trial(
            42,
            o::Mode::Serial,
            case,
            f::Config {
                periods: 1,
                ..Default::default()
            },
        );
        let b = old::trial(
            42,
            c::Mode::Control,
            case,
            old::Config {
                periods: 1,
                ..Default::default()
            },
        );
        assert_eq!(a.initial.state, b.initial);
        assert_eq!(a.points[a.periods[0].after].state, b.periods[0].after);
        assert_eq!(
            a.final_state.base.base.base.base.base.events,
            b.final_state.base.base.base.base.base.events
        );
    }
}
