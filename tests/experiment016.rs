use world_of_individuals::{
    contact_learning as c, encounters as e, experiment015 as old, experiment016 as f,
};
fn signal(partner: u32, kind: e::Kind, outcome: c::Outcome, t: u64) -> c::Signal {
    c::Signal {
        owner: 0,
        partner,
        kind,
        sent: t - 2,
        received: t,
        outcome,
    }
}
#[test]
fn unchanged_reference_is_exact_original015_execution() {
    for (case, original) in [
        ("mutual", "mutual"),
        ("decline", "private_mirror"),
        ("contended", "contended"),
        ("information", "information"),
        ("refusal", "refusal"),
        ("missed", "missed"),
    ] {
        let a = f::trial(42, c::Mode::Unchanged, case, f::Config::default());
        let b = old::trial(42, e::Mode::History, original, old::Config::default());
        assert_eq!(a.initial, b.initial);
        assert_eq!(a.final_state, b.final_state);
        assert!(a.final_cells.iter().all(Vec::is_empty));
        for (x, y) in a.periods.iter().zip(&b.periods) {
            assert_eq!(x.before, y.before);
            assert_eq!(x.after, y.after);
            assert_eq!(
                x.waves.iter().map(|w| &w.wave).collect::<Vec<_>>(),
                y.waves.iter().collect::<Vec<_>>()
            );
        }
    }
}
#[test]
fn finite_retry_cells_fifo_update_success_and_expiration() {
    let a = c::update(
        &[],
        c::Mode::Typed,
        signal(1, e::Kind::Resource, c::Outcome::Declined, 2),
    );
    assert_eq!(a.after[0].until, 14);
    let b = c::update(
        &a.after,
        c::Mode::Typed,
        signal(2, e::Kind::Resource, c::Outcome::AcceptedUnmet, 3),
    );
    assert_eq!(b.after[1].until, 5);
    let again = c::update(
        &b.after,
        c::Mode::Typed,
        signal(1, e::Kind::Resource, c::Outcome::Declined, 4),
    );
    assert_eq!(again.after[0].partner, 1);
    let full = c::update(
        &again.after,
        c::Mode::Typed,
        signal(2, e::Kind::Inquiry, c::Outcome::Declined, 4),
    );
    assert_eq!(full.evicted, Some(again.after[0].clone()));
    assert_eq!(full.after.len(), 2);
    let met = c::update(
        &full.after,
        c::Mode::Typed,
        signal(2, e::Kind::Inquiry, c::Outcome::Met, 5),
    );
    assert!(met.after.is_empty());
    assert!(c::expire(&a.after, 14).is_empty());
    assert_eq!(c::expire(&a.after, 13), a.after);
}
#[test]
fn recent_is_one_real_cross_wave_record_and_ablation_only_changes_duration() {
    let a = c::update(
        &[],
        c::Mode::Recent,
        signal(1, e::Kind::Resource, c::Outcome::Declined, 2),
    );
    assert_eq!(a.after[0].until, 8);
    let b = c::update(
        &a.after,
        c::Mode::Recent,
        signal(2, e::Kind::Inquiry, c::Outcome::Declined, 3),
    );
    assert_eq!(b.after.len(), 1);
    assert_eq!(b.after[0].partner, 2);
    for outcome in [
        c::Outcome::AcceptedUnmet,
        c::Outcome::Unavailable,
        c::Outcome::Expired,
    ] {
        let a = c::update(
            &[],
            c::Mode::Typed,
            signal(1, e::Kind::Resource, outcome, 2),
        );
        let b = c::update(
            &[],
            c::Mode::Untyped,
            signal(1, e::Kind::Resource, outcome, 2),
        );
        assert_eq!((a.after[0].until, b.after[0].until), (4, 14));
    }
}
#[test]
fn feedback_masks_only_the_own_matching_contact_and_cannot_recover_expired_state() {
    let mut s = f::setup(42, "mutual");
    s.encounter_open(24, [0; 3]).unwrap();
    s.encounter_prepare().unwrap();
    let own = s.encounter_input(0, &s.encounter_people(), 24).unwrap();
    let cells = c::update(
        &[],
        c::Mode::Typed,
        signal(1, e::Kind::Resource, c::Outcome::Declined, 2),
    )
    .after;
    let mut i = c::Input {
        own,
        mode: c::Mode::Typed,
        retained: cells,
    };
    i.own.now = 2;
    let chosen = c::policy(&i);
    assert_eq!(
        chosen.selected,
        e::Intent::Invite {
            recipient: 2,
            kind: e::Kind::Resource
        }
    );
    i.own.now = 14;
    i.own.end = 40;
    i.retained = c::expire(&i.retained, 14);
    assert_eq!(c::policy(&i), e::policy(&i.own, e::Mode::History));
}
#[test]
fn recipient_control_has_no_treatment_outgoing_score_scale() {
    let mut s = f::setup(42, "mutual");
    s.encounter_open(24, [0; 3]).unwrap();
    s.encounter_prepare().unwrap();
    let own = s.encounter_input(1, &s.encounter_people(), 24).unwrap();
    let outgoing = e::policy(&own, e::Mode::History);
    let mut i = e::ResponseInput {
        own,
        outgoing,
        incoming: vec![e::Invitation {
            proposer: 0,
            recipient: 1,
            kind: e::Kind::Resource,
            expires: 2,
        }],
    };
    let a = c::respond(&i);
    for x in &mut i.outgoing.candidates {
        x.score += 10000;
    }
    let b = c::respond(&i);
    assert_eq!(a.candidates, b.candidates);
    assert_eq!(a.selected, b.selected);
}
#[test]
fn hidden_partner_willingness_does_not_change_current_initiation() {
    let mut a = f::setup(42, "information");
    let mut b = f::setup(42, "decline");
    let tick = a.activity_tick();
    for s in [&mut a, &mut b] {
        s.enable_contact_learning(c::Mode::Typed).unwrap();
        s.encounter_open(24, [tick; 3]).unwrap();
        s.encounter_prepare().unwrap();
    }
    let left = a
        .encounter_input(0, &a.encounter_people(), tick + 24)
        .unwrap();
    let right = b
        .encounter_input(0, &b.encounter_people(), tick + 24)
        .unwrap();
    assert_eq!(left, right);
    assert_eq!(
        c::policy(&c::Input {
            own: left,
            mode: c::Mode::Typed,
            retained: vec![]
        }),
        c::policy(&c::Input {
            own: right,
            mode: c::Mode::Typed,
            retained: vec![]
        })
    );
}
#[test]
fn invitation_decline_never_trains_query_and_meeting_is_not_negotiation_success() {
    let mut s = f::setup(42, "decline");
    s.enable_contact_learning(c::Mode::Typed).unwrap();
    let t = s.activity_tick();
    s.encounter_open(24, [t; 3]).unwrap();
    let w = s.contact_wave(e::Order::Ascending, 0).unwrap();
    assert!(
        w.updates
            .iter()
            .any(|u| u.signal.kind == e::Kind::Inquiry && u.signal.outcome == c::Outcome::Declined)
    );
    assert_eq!(w.wave.before.queries, w.wave.after.queries);
    let t = f::trial(42, c::Mode::Typed, "refusal", f::Config::default());
    let events = &t.final_state.base.base.base.base.base.events;
    assert!(t.periods.iter().flat_map(|p| &p.waves).any(|w| {
        w.updates
            .iter()
            .any(|u| u.signal.outcome == c::Outcome::Met)
            && w.wave.execution.as_ref().is_some_and(|x| {
                events[x.before.events..x.after.events]
                    .iter()
                    .any(|e| e.outcome == Some(world_of_individuals::model::Outcome::Refusal))
            })
    }));
}
#[test]
fn all_core_families_replay_conserve_food_and_bound_new_cognition() {
    for case in f::CASES {
        for mode in c::MODES {
            let t = f::trial(42, mode, case, f::Config::default());
            assert_eq!(t, f::trial(42, mode, case, f::Config::default()));
            let summary = f::summary(&t);
            assert_eq!(
                t.initial.agents.iter().map(|a| a.food).sum::<u32>(),
                summary.consumed + summary.final_food.iter().sum::<u32>()
            );
            for p in &t.periods {
                for w in &p.waves {
                    assert!(w.available.iter().all(|cs| cs.len() <= 2));
                    assert!(w.wave.after.tick > w.wave.before.tick && w.wave.after.tick <= p.end);
                    for u in &w.updates {
                        assert_eq!(u.signal.received, w.wave.after.tick);
                        assert!(u.after.len() <= 2);
                    }
                }
            }
        }
    }
}

#[test]
fn accepted_unmet_feedback_hides_contention_details_and_notifies_only_proposer() {
    fn alternative(input: &e::ResponseInput) -> e::Response {
        let mut r = e::respond(input);
        if input.own.own.id == 0 && input.incoming.iter().any(|i| i.proposer == 2) {
            r.selected = e::Reply::Accept(2);
        }
        r
    }
    let mut sim = old::setup(42, "contended");
    sim.encounter_open(24, [0; 3]).unwrap();
    sim.set_encounter_policies(e::policy, alternative).unwrap();
    let mut w = sim
        .encounter_wave(e::Mode::History, e::Order::Ascending, 0)
        .unwrap();
    let owner = w
        .allocations
        .iter()
        .find(|a| a.disposition == e::Disposition::Contention)
        .unwrap()
        .invitation
        .proposer;
    let signal = c::signal(&w, owner).unwrap();
    assert_eq!(signal.outcome, c::Outcome::AcceptedUnmet);
    let keys = serde_json::to_value(&signal).unwrap();
    assert_eq!(keys.as_object().unwrap().len(), 6);
    assert!(keys.get("disposition").is_none());
    assert!(keys.get("competing").is_none());
    for (disposition, outcome) in [
        (e::Disposition::Expired, c::Outcome::Expired),
        (e::Disposition::Unavailable, c::Outcome::Unavailable),
    ] {
        w.allocations
            .iter_mut()
            .find(|a| a.invitation.proposer == owner)
            .unwrap()
            .disposition = disposition;
        assert_eq!(c::signal(&w, owner).unwrap().outcome, outcome);
    }
    let mut mutual = old::setup(42, "mutual");
    mutual.encounter_open(24, [0; 3]).unwrap();
    let w = mutual
        .encounter_wave(e::Mode::History, e::Order::Ascending, 0)
        .unwrap();
    assert_eq!(c::signal(&w, 2), None); // Wait gets no invented outgoing experience.
}
