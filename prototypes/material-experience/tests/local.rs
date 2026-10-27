use woi_material_experience::experience::{Cell, expectation, retain, update};
#[test]
fn expiry_capacity_and_no_hidden_aggregate() {
    let c = |t, s| Cell {
        owner: 0,
        received: t,
        success: s,
    };
    let (v, e) = update(&[], c(0, true));
    assert!(e.is_none());
    assert_eq!(expectation(&v), 100);
    let (v, _) = update(&v, c(1, false));
    assert_eq!(expectation(&v), 50);
    let (v, e) = update(&v, c(2, false));
    assert_eq!(e, Some(c(0, true)));
    assert_eq!(expectation(&v), 0);
    assert_eq!(retain(&v, 25), vec![c(2, false)]);
    assert!(retain(&v, 26).is_empty());
    assert_eq!(expectation(&[]), 50);
}
#[test]
#[should_panic]
fn private_receipt_cannot_update_other_actor() {
    update(
        &[Cell {
            owner: 1,
            received: 0,
            success: true,
        }],
        Cell {
            owner: 0,
            received: 1,
            success: true,
        },
    );
}
#[test]
#[should_panic]
fn time_reversal_rejected() {
    update(
        &[Cell {
            owner: 0,
            received: 2,
            success: true,
        }],
        Cell {
            owner: 0,
            received: 1,
            success: true,
        },
    );
}

#[test]
fn observer_audit_cannot_restore_expired_learning() {
    use woi_material_experience::{
        experience::{Learning, Mode},
        preparation::Change,
    };
    let mut l = Learning::new(Mode::Experience);
    l.receive(0, 0, Change::Taken, 9);
    l.receive(0, 1, Change::Sense, 10);
    assert_eq!(l.cells[0].len(), 1);
    l.finish(24);
    assert!(l.cells[0].is_empty());
    assert_eq!(l.receipts.len(), 1);
    assert_eq!(expectation(&l.cells[0]), 50);
    let mut fixed = Learning::new(Mode::Fixed);
    fixed.receive(0, 0, Change::Taken, 9);
    assert!(fixed.cells.iter().all(Vec::is_empty));
}

#[test]
fn eligibility_and_first_tie_survive_positive_and_negative_adjustments() {
    use woi_material_experience::{
        experience::{Mode, rank},
        preparation::{Action, Selection, Term},
    };
    let term = |action, eligible, score| Term {
        action,
        eligible,
        base_score: score,
        meal_relief: 0,
        work_effort: 0,
        final_score: score,
    };
    let mut fixed = Selection {
        terms: vec![
            term(Action::Wait, true, 0),
            term(Action::Consume, false, -1),
            term(Action::Invite, true, 20),
            term(Action::Acquire, false, -1),
            term(Action::Prepare, true, 10),
        ],
        selected: Action::Invite,
    };
    let success = vec![Cell {
        owner: 0,
        received: 0,
        success: true,
    }];
    let tied = rank(fixed.clone(), &success, Mode::Experience);
    assert_eq!(tied.terms[4].final_score, 20);
    assert_eq!(tied.selected, Action::Invite);
    fixed.terms[4].eligible = false;
    fixed.terms[4].base_score = -1;
    fixed.terms[4].final_score = -1;
    assert_eq!(rank(fixed.clone(), &success, Mode::Experience), fixed);
    fixed.terms[4].eligible = true;
    fixed.terms[4].base_score = 21;
    fixed.terms[4].final_score = 21;
    fixed.selected = Action::Prepare;
    let failure = vec![Cell {
        owner: 0,
        received: 0,
        success: false,
    }];
    assert_eq!(
        rank(fixed.clone(), &failure, Mode::Experience).selected,
        Action::Invite
    );
    assert_eq!(rank(fixed.clone(), &failure, Mode::Fixed), fixed);
}
