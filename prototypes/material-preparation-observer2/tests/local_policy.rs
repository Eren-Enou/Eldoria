//! Synthetic local/physical controls, not registered025world outcomes.
use world_of_individuals::{
    acquisition as a, continuous as c, environmental_knowledge as k, preparation as m,
    valuation as v,
};

fn local() -> (m::Input, Vec<v::Term>) {
    let cfg = c::matrix()
        .into_iter()
        .find(|x| x.case == "scarcity" && x.mode == c::Mode::Continuous)
        .unwrap();
    let old = c::trial(cfg);
    let mut inherited = old.history.boundaries[0].choices[0].input.clone();
    inherited.retained = None;
    let decision = k::policy(&inherited, a::Policy::Need);
    let terms = (v::Mode::Baseline.selector())(&inherited, &decision.candidates).terms;
    let owner = inherited.legacy.own.id;
    let now = inherited.legacy.now;
    (
        m::Input {
            inherited,
            raw_retained: Some(m::Cell {
                owner,
                receipt: 0,
                observation: m::Observation {
                    id: m::SourceId(0),
                    observed_at: now,
                    stock: 2,
                    accessible: true,
                    extraction_ticks: 2,
                    preparation_ticks: 4,
                    input_units: 2,
                    edible_units: 1,
                    waste_units: 1,
                },
                own_takes: 0,
            }),
        },
        terms,
    )
}
#[test]
fn local_effort_can_win_without_changing_old_terms() {
    let (input, terms) = local();
    let selected = m::rank(&input, &terms);
    assert_eq!(selected.selected, m::Action::Prepare);
    assert_eq!(selected.terms[4].final_score, 74);
    for (new, old) in selected.terms[..4].iter().zip(terms) {
        assert_eq!(new.action, m::Action::from(old.action));
        assert_eq!(
            (
                new.eligible,
                new.base_score,
                new.meal_relief,
                new.work_effort,
                new.final_score
            ),
            (
                old.eligible,
                old.base_score,
                old.meal_relief,
                0,
                old.final_score
            )
        );
    }
}
#[test]
fn unknown_empty_closed_future_and_foreign_records_do_not_authorize_work() {
    let (input, terms) = local();
    for kind in 0..6 {
        let mut changed = input.clone();
        match kind {
            0 => changed.raw_retained = None,
            1 => changed.raw_retained.as_mut().unwrap().observation.stock = 0,
            2 => {
                changed
                    .raw_retained
                    .as_mut()
                    .unwrap()
                    .observation
                    .accessible = false
            }
            3 => {
                changed
                    .raw_retained
                    .as_mut()
                    .unwrap()
                    .observation
                    .observed_at += 1
            }
            4 => changed.raw_retained.as_mut().unwrap().owner = 3,
            _ => changed.raw_retained.as_mut().unwrap().own_takes = 2,
        }
        let result = m::rank(&changed, &terms);
        assert!(!result.terms[4].eligible);
        assert_ne!(result.selected, m::Action::Prepare);
    }
}
#[test]
fn stable_tie_keeps_old_action_and_cost_can_crowd_work_out() {
    let (mut input, mut terms) = local();
    terms[2].base_score = 74;
    terms[2].final_score = 74;
    assert_eq!(m::rank(&input, &terms).selected, m::Action::Invite);
    let cell = input.raw_retained.as_mut().unwrap();
    cell.observation.extraction_ticks = 8;
    cell.observation.preparation_ticks = 24;
    let r = m::rank(&input, &terms);
    assert_eq!(r.terms[4].final_score, 48);
    assert_eq!(r.selected, m::Action::Invite);
}
#[test]
fn owned_food_and_public_deadline_can_preclude_preparation() {
    let (input, terms) = local();
    let mut food = input.clone();
    food.inherited.legacy.own.food = 1;
    assert!(!m::rank(&food, &terms).terms[4].eligible);
    let mut time = input;
    time.inherited.legacy.end = time.inherited.legacy.now + 5;
    assert!(!m::rank(&time, &terms).terms[4].eligible);
}
#[test]
#[should_panic(expected = "input.inherited.retained.is_none()")]
fn two_usable_source_slots_are_rejected() {
    let (mut input, terms) = local();
    input.inherited.retained = Some(k::Cell {
        owner: input.inherited.legacy.own.id,
        receipt: 9,
        observation: a::Observation {
            id: a::CacheId(0),
            observed_at: input.inherited.legacy.now,
            stock: 1,
            accessible: true,
            duration: 4,
        },
        own_takes: 0,
    });
    m::rank(&input, &terms);
}
#[test]
fn competing_batches_and_conversion_conserve_mass() {
    let (raw, escrow, granted) = m::extract(2, 0).unwrap();
    assert_eq!((raw, escrow, granted), (0, 2, m::Outcome::Extracted));
    let (raw, loser, failed) = m::extract(raw, 0).unwrap();
    assert_eq!((raw, loser, failed), (0, 0, m::Outcome::Insufficient));
    let (escrow, food, waste, outcome) = m::convert(escrow, 0, 0).unwrap();
    assert_eq!(
        (escrow, food, waste, outcome),
        (0, 1, 1, m::Outcome::Prepared)
    );
    assert_eq!(raw + escrow + loser + food + waste, 2);
    assert_eq!(
        m::convert(loser, 0, waste).unwrap(),
        (0, 0, 1, m::Outcome::Failed)
    );
}
#[test]
fn invalid_escrow_and_overflow_do_not_partially_convert() {
    assert!(m::extract(8, 2).is_err());
    assert!(m::convert(1, 0, 0).is_err());
    assert!(m::convert(2, u32::MAX, 0).is_err());
    assert!(m::convert(2, 0, u32::MAX).is_err());
    assert_eq!(m::extract(1, 0).unwrap(), (1, 0, m::Outcome::Insufficient));
}
#[test]
fn all_registered_configs_obey_bounds_without_running_them() {
    let matrix = m::matrix();
    assert_eq!(matrix.len(), 50);
    for c in matrix {
        c.validate().unwrap();
        assert_eq!(c.valuation, v::Mode::Baseline);
    }
}

#[test]
fn local_wire_roundtrip_preserves_integer_keyed_trust() {
    let (mut input, _) = local();
    input.inherited.legacy.own.trust.insert(1, 24);
    let value = serde_json::to_value(&input).unwrap();
    assert_eq!(value["legacy"]["own"]["trust"]["1"], 24);
    let restored: m::Input = serde_json::from_value(value).unwrap();
    assert_eq!(restored, input);
    let text = serde_json::to_string(&input).unwrap();
    assert_eq!(serde_json::from_str::<m::Input>(&text).unwrap(), input);
}
