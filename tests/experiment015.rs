use world_of_individuals::{encounters as e, experiment015 as f};
#[test]
fn simultaneous_accepted_edges_cannot_double_book_and_order_is_explicit() {
    fn alternative_tie(input: &e::ResponseInput) -> e::Response {
        let mut r = e::respond(input);
        if input.own.own.id == 0 && input.incoming.iter().any(|i| i.proposer == 2) {
            r.selected = e::Reply::Accept(2);
        }
        r
    }
    let mut s = f::setup(42, "contended");
    let now = s.activity_tick();
    s.encounter_open(24, [now; 3]).unwrap();
    s.set_encounter_policies(e::policy, alternative_tie)
        .unwrap();
    let w = s
        .encounter_wave(e::Mode::History, e::Order::Ascending, 0)
        .unwrap();
    assert_eq!(
        w.allocations
            .iter()
            .filter(|a| a.disposition == e::Disposition::Execute)
            .count(),
        1
    );
    assert!(
        w.allocations
            .iter()
            .any(|a| a.disposition == e::Disposition::Contention)
    );
    let invitations = w
        .allocations
        .iter()
        .map(|a| a.invitation.clone())
        .collect::<Vec<_>>();
    let other = e::arbitrate(
        now + 2,
        e::Order::Descending,
        0,
        &w.proposals[0].input.people,
        &invitations,
        &w.responses,
    );
    assert_ne!(
        other
            .iter()
            .find(|a| a.disposition == e::Disposition::Execute)
            .unwrap()
            .invitation,
        w.execution.unwrap().invitation
    );
}
#[test]
fn matched_worlds_replay_time_food_and_cognitive_caps() {
    for case in f::CASES {
        for mode in e::MODES {
            let t = f::trial(42, mode, case, f::Config::default());
            assert_eq!(t, f::trial(42, mode, case, f::Config::default()));
            let consumed = t
                .periods
                .iter()
                .flat_map(|p| &p.consumption)
                .map(|c| c.consumed)
                .sum::<u32>();
            assert_eq!(
                t.initial.agents.iter().map(|a| a.food).sum::<u32>(),
                consumed
                    + t.periods
                        .last()
                        .unwrap()
                        .after
                        .agents
                        .iter()
                        .map(|a| a.food)
                        .sum::<u32>()
            );
            for p in &t.periods {
                assert_eq!(p.after.tick, p.end + 3);
                for w in &p.waves {
                    assert!(w.after.tick > w.before.tick && w.after.tick <= p.end);
                    assert!(
                        w.proposals.len() <= 3
                            && w.responses.len() <= 3
                            && w.allocations.len() <= 3
                    );
                    assert!(
                        w.allocations
                            .iter()
                            .filter(|a| a.disposition == e::Disposition::Execute)
                            .count()
                            <= 1
                    );
                    for a in &w.after.agents {
                        assert!(a.memories.len() <= 16);
                    }
                }
            }
        }
    }
}
#[test]
fn mutual_acceptance_executes_resource_but_unilateral_does_not_force_it() {
    let mutual = f::trial(42, e::Mode::History, "mutual", f::Config::default());
    assert!(f::summary(&mutual).transfers > 0);
    let unilateral = f::trial(42, e::Mode::History, "unilateral", f::Config::default());
    let first = &unilateral.periods[0];
    assert!(first.waves.iter().any(|w| {
        w.allocations
            .iter()
            .any(|a| a.disposition == e::Disposition::Declined)
    }));
    assert!(first.waves.iter().all(|w| w.execution.is_none()));
    assert!(first.waves[0].after.tick >= first.start + 2);
}
#[test]
fn hidden_partner_profile_cannot_change_proposer_current_choice() {
    let mut a = f::setup(42, "information");
    let mut b = f::setup(42, "private_mirror");
    let now = a.activity_tick();
    assert_eq!(now, b.activity_tick());
    for s in [&mut a, &mut b] {
        s.encounter_open(24, [now; 3]).unwrap();
        s.encounter_prepare().unwrap();
    }
    let left = a
        .encounter_input(0, &a.encounter_people(), now + 24)
        .unwrap();
    let right = b
        .encounter_input(0, &b.encounter_people(), now + 24)
        .unwrap();
    assert_eq!(left, right);
    assert_eq!(
        e::policy(&left, e::Mode::History),
        e::policy(&right, e::Mode::History)
    );
}
#[test]
fn old_resource_history_changes_partner_ranking_without_private_inputs() {
    let mut s = f::setup(42, "history");
    let now = s.activity_tick();
    s.encounter_open(24, [now; 3]).unwrap();
    s.encounter_prepare().unwrap();
    let i = s
        .encounter_input(0, &s.encounter_people(), now + 24)
        .unwrap();
    assert_eq!(
        e::policy(&i, e::Mode::History).selected,
        e::Intent::Invite {
            recipient: 2,
            kind: e::Kind::Resource
        }
    );
    assert_eq!(
        e::policy(&i, e::Mode::NoHistory).selected,
        e::Intent::Invite {
            recipient: 1,
            kind: e::Kind::Resource
        }
    );
}
#[test]
fn expired_invitations_and_double_execution_cannot_be_resolved() {
    let mut s = f::setup(42, "mutual");
    let now = s.activity_tick();
    s.encounter_open(8, [now; 3]).unwrap();
    let wave = s
        .encounter_wave(e::Mode::History, e::Order::Ascending, 0)
        .unwrap();
    let invites = wave
        .allocations
        .iter()
        .map(|a| a.invitation.clone())
        .collect::<Vec<_>>();
    let expired = e::arbitrate(
        now + 3,
        e::Order::Ascending,
        0,
        &s.encounter_people(),
        &invites,
        &wave.responses,
    );
    assert!(
        expired
            .iter()
            .all(|a| a.disposition == e::Disposition::Expired)
    );
    s.encounter_finish().unwrap();
    let tick = s.activity_tick();
    assert!(
        s.encounter_wave(e::Mode::History, e::Order::Ascending, 1)
            .is_err()
    );
    assert!(s.encounter_finish().is_err());
    assert_eq!(s.activity_tick(), tick);
}
#[test]
fn public_availability_has_no_future_release_and_existing_commitment_is_respected() {
    let mut a = f::setup(42, "mutual");
    let mut b = f::setup(42, "mutual");
    let now = a.activity_tick();
    a.encounter_open(24, [now, now + 8, now]).unwrap();
    b.encounter_open(24, [now, now + 20, now]).unwrap();
    assert_eq!(a.encounter_people(), b.encounter_people());
    let left = a
        .encounter_input(0, &a.encounter_people(), now + 24)
        .unwrap();
    let right = b
        .encounter_input(0, &b.encounter_people(), now + 24)
        .unwrap();
    assert_eq!(left, right);
    let w = a
        .encounter_wave(e::Mode::History, e::Order::Ascending, 0)
        .unwrap();
    assert!(
        w.execution
            .as_ref()
            .is_none_or(|x| x.invitation.proposer != 1 && x.invitation.recipient != 1)
    );
}
#[test]
fn resource_encounters_never_run_free_automatic_inquiry_and_refusal_is_real() {
    let t = f::trial(42, e::Mode::History, "refusal", f::Config::default());
    let events = &t.final_state.base.base.base.base.base.events;
    assert!(
        events[t.initial.events..]
            .iter()
            .any(|x| x.outcome == Some(world_of_individuals::model::Outcome::Refusal))
    );
    for x in t
        .periods
        .iter()
        .flat_map(|p| &p.waves)
        .filter_map(|w| w.execution.as_ref())
    {
        if x.invitation.kind == e::Kind::Resource {
            assert_eq!(x.before.queries, x.after.queries);
            assert_eq!(
                x.after.tick - x.before.tick,
                (x.after.events - x.before.events) as u64
            );
        }
    }
}
#[test]
fn evicted_provider_knowledge_cannot_be_restored_from_its_full_archive() {
    let mut sim = f::setup(42, "formerly_useful");
    let event = sim.concerns().unwrap().items[&0][0].event;
    let archive = sim.provenance().unwrap();
    assert!(!archive.knowledge[&2].iter().any(|k| k.event == event));
    assert!(
        archive
            .receipts
            .iter()
            .any(|r| r.listener == 2 && r.event == event)
    );
    let count = archive.receipts.len();
    assert_eq!(sim.provenance_exchange(2, 0, event, true).unwrap(), None);
    assert_eq!(sim.provenance().unwrap().receipts.len(), count);
    let t = f::trial(
        42,
        e::Mode::History,
        "formerly_useful",
        f::Config::default(),
    );
    let records = &t.final_state.learning.records;
    assert!(
        records
            .iter()
            .any(|r| r.inquiry < (t.initial.queries as u64)
                && r.outcome.as_ref().is_some_and(|o| o.receipt_acquired))
    );
    assert!(
        records
            .iter()
            .any(|r| r.inquiry >= (t.initial.queries as u64)
                && r.outcome.as_ref().is_some_and(|o| o.receipt_acquired
                    && o.novelty_value == 0
                    && o.support_before == o.support_after))
    );
}
