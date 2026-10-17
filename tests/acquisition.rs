use world_of_individuals::{acquisition as a, experiment020 as f};
#[test]
fn scalar_grant_conserves_and_never_wraps() {
    assert_eq!(a::grant(1, 0), (0, 1, a::Outcome::Granted));
    assert_eq!(a::grant(0, 0), (0, 0, a::Outcome::Depleted));
    assert_eq!(a::grant(1, u32::MAX), (1, u32::MAX, a::Outcome::Overflow));
}
#[test]
fn last_unit_claims_pay_full_time_and_preserve_stock() {
    let t = f::trial(42, f::Mode::Acquire, "contested", f::config("contested"));
    let first = t.records.iter().take(3).collect::<Vec<_>>();
    assert_eq!(first.len(), 3);
    assert_eq!(
        first
            .iter()
            .filter(|r| r.outcome == a::Outcome::Granted)
            .count(),
        1
    );
    assert_eq!(
        first
            .iter()
            .filter(|r| r.outcome == a::Outcome::Depleted)
            .count(),
        2
    );
    for r in first {
        assert_eq!(r.completed_at, r.attempt.start + 4);
    }
    assert_eq!(t.final_point.cache.stock, 0);
}
#[test]
fn owned_ongoing_is_exact_019_control() {
    let t = f::trial(42, f::Mode::Owned, "ongoing", f::config("ongoing"));
    let old = world_of_individuals::experiment019::trial(
        42,
        world_of_individuals::priorities::Mode::Baseline,
        "ongoing",
        Default::default(),
    );
    assert_eq!(t.final_state, old.final_state);
    assert_eq!(t.needs, old.needs);
    for (new, old) in t.periods.iter().zip(old.periods) {
        assert_eq!(new.steps.len(), old.steps.len());
        for (new, old) in new.steps.iter().zip(old.steps) {
            assert_eq!(new.activity, old.activity);
        }
    }
}
#[test]
fn private_partner_changes_do_not_enter_first_actor_input() {
    let helpful = f::trial(
        42,
        f::Mode::Need,
        "private_helpful",
        f::config("private_helpful"),
    );
    let cautious = f::trial(
        42,
        f::Mode::Need,
        "private_cautious",
        f::config("private_cautious"),
    );
    assert_eq!(
        helpful.periods[0].steps[0].choices[0],
        cautious.periods[0].steps[0].choices[0]
    );
}
#[test]
fn finishing_short_claim_cannot_invent_prior_consent() {
    let t = f::trial(
        42,
        f::Mode::Acquire,
        "lost_contact",
        f::Config {
            duration: 2,
            ..f::config("lost_contact")
        },
    );
    let first = &t.periods[0].steps[0];
    assert_eq!(first.skipped_responses, vec![0, 2]);
    assert_eq!(first.completed.len(), 2);
    assert!(first.activity.started.is_none());
    assert!(
        first
            .activity
            .allocations
            .iter()
            .all(|a| a.disposition != world_of_individuals::encounters::Disposition::Execute)
    );
    assert!(
        first
            .activity
            .allocations
            .iter()
            .any(|a| a.disposition == world_of_individuals::encounters::Disposition::Declined)
    );
}
