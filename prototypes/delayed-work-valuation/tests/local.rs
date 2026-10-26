use serde::Deserialize;
use world_of_individuals::{delayed as n, preparation as m, valuation as v};
#[derive(Deserialize)]
struct Fixture {
    input: m::Input,
    terms: Vec<v::Term>,
}
fn fixture() -> Fixture {
    serde_json::from_str(include_str!("local-fixture.json")).unwrap()
}

#[test]
fn every_need_phase_bounds_projection_and_saturation() {
    for h in 0..=100 {
        for duration in 1..=33 {
            let projected = n::projected_need(h, duration);
            assert!((h..=100).contains(&projected));
            let mut lowest = 100;
            for phase in 0..4 {
                let mut actual = h;
                for step in 1..=duration {
                    if (phase + step) % 4 == 0 {
                        actual = (actual + 5).min(100);
                    }
                }
                assert!(projected <= actual);
                lowest = lowest.min(actual);
            }
            assert_eq!(projected, lowest);
        }
    }
    assert_eq!(n::projected_need(99, 32), 100);
}
#[test]
fn inherited_first_maximum_survives_treatment_tie() {
    let mut f = fixture();
    let target = n::rank(&f.input, &f.terms, n::Mode::ProjectedNeed).terms[4].final_score;
    f.terms[2].base_score = target;
    f.terms[2].final_score = target;
    f.terms[2].eligible = true;
    assert_eq!(
        n::rank(&f.input, &f.terms, n::Mode::ProjectedNeed).selected,
        m::Action::Invite
    );
    f.terms[2].base_score -= 1;
    f.terms[2].final_score -= 1;
    assert_eq!(
        n::rank(&f.input, &f.terms, n::Mode::ProjectedNeed).selected,
        m::Action::Prepare
    );
}
#[test]
fn treatment_cannot_restore_unavailable_source_or_erase_eligibility_masks() {
    for kind in 0..5 {
        let mut f = fixture();
        match kind {
            0 => f.input.raw_retained = None,
            1 => f.input.inherited.legacy.own.food = 1,
            2 => {
                f.input
                    .raw_retained
                    .as_mut()
                    .unwrap()
                    .observation
                    .accessible = false
            }
            3 => {
                f.input
                    .raw_retained
                    .as_mut()
                    .unwrap()
                    .observation
                    .observed_at = f.input.inherited.legacy.now + 1
            }
            _ => f.input.raw_retained.as_mut().unwrap().own_takes = 8,
        }
        let inherited = m::rank(&f.input, &f.terms);
        assert!(!inherited.terms[4].eligible);
        assert_eq!(
            n::rank(&f.input, &f.terms, n::Mode::ProjectedNeed),
            inherited
        );
    }
}
#[test]
fn one_term_changes_without_mutating_input_or_old_winner_monotonicity() {
    let mut f = fixture();
    for h in 1..=100 {
        f.input.inherited.legacy.own.hunger = h;
        let before = f.input.clone();
        let old = m::rank(&f.input, &f.terms);
        assert_eq!(n::rank(&f.input, &f.terms, n::Mode::Baseline), old);
        let new = n::rank(&f.input, &f.terms, n::Mode::ProjectedNeed);
        assert_eq!(new.terms[..4], old.terms[..4]);
        assert!(new.terms[4].final_score >= old.terms[4].final_score);
        if old.selected == m::Action::Prepare {
            assert_eq!(new.selected, m::Action::Prepare);
        }
        assert_eq!(f.input, before);
    }
}
