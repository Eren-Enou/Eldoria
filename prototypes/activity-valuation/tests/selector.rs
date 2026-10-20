//! Meaningful selector controls: relative change, ties, no forced meal, no resurrection.
use world_of_individuals::{
    acquisition as a, continuous as c, environmental_knowledge as k, valuation as v,
};
fn local(hunger: i32) -> k::Decision {
    let cfg = c::matrix()
        .into_iter()
        .find(|x| x.case == "busy_recipient" && x.mode == c::Mode::Continuous)
        .unwrap();
    let old = c::trial(cfg);
    let d = old
        .history
        .boundaries
        .iter()
        .find(|b| b.tick == 7)
        .unwrap()
        .choices
        .iter()
        .find(|d| d.input.legacy.own.id == 2)
        .unwrap();
    let mut input = d.input.clone();
    input.legacy.own.hunger = hunger;
    input.legacy.query.input.base.hunger = hunger;
    input.legacy.query.decision.input.hunger = hunger;
    k::policy(&input, a::Policy::Need)
}
#[test]
fn own_relief_changes_relative_choice_but_preserves_contact_values() {
    let d = local(40);
    assert_eq!(d.selected, a::Action::Invite);
    let control = (v::Mode::Baseline.selector())(&d.input, &d.candidates);
    let change = (v::Mode::MealRelief.selector())(&d.input, &d.candidates);
    assert_eq!(control.selected, a::Action::Invite);
    assert_eq!(change.selected, a::Action::Consume);
    assert_eq!(control.terms[2], change.terms[2]);
    assert_eq!(control.terms[3], change.terms[3]);
}
#[test]
fn equal_final_values_keep_original_consume_before_invite_order() {
    let d = local(32);
    let change = (v::Mode::MealRelief.selector())(&d.input, &d.candidates);
    assert_eq!(change.terms[1].final_score, change.terms[2].final_score);
    assert_eq!(change.selected, a::Action::Consume);
}
#[test]
fn strong_contact_can_still_outrank_an_eligible_meal() {
    let cfg = c::matrix()
        .into_iter()
        .find(|x| x.case == "exhaustion" && x.mode == c::Mode::Continuous)
        .unwrap();
    let old = c::trial(cfg);
    let d = old
        .history
        .boundaries
        .iter()
        .find(|b| b.tick == 77)
        .unwrap()
        .choices
        .iter()
        .find(|d| d.input.legacy.own.id == 2)
        .unwrap();
    let change = (v::Mode::MealRelief.selector())(&d.input, &d.candidates);
    assert!(change.terms[1].eligible);
    assert_eq!(change.selected, a::Action::Invite);
    assert!(change.terms[2].final_score > change.terms[1].final_score);
}
#[test]
fn no_food_cannot_be_converted_to_a_meal_by_a_score_bonus() {
    let cfg = c::matrix()
        .into_iter()
        .find(|x| x.case == "null" && x.mode == c::Mode::Continuous)
        .unwrap();
    let old = c::trial(cfg);
    let d = &old.history.boundaries[0].choices[0];
    let change = (v::Mode::MealRelief.selector())(&d.input, &d.candidates);
    assert_eq!(change.selected, a::Action::Wait);
    assert!(!change.terms[1].eligible);
    assert_eq!(change.terms[1].final_score, -1);
    assert!(change.terms.iter().all(|t| t.meal_relief == 0));
}
