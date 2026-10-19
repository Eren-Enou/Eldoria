//! Supplementary mechanical coverage discovered in the unchanged original022 grid.
use world_of_individuals::{experiment::concurrent as x, inquiry};

fn trial(case: &str, capacity: usize) -> x::Trial {
    x::trial(
        x::matrix()
            .into_iter()
            .find(|c| c.case == case && c.capacity == capacity)
            .unwrap(),
    )
}

#[test]
fn a_local_question_cannot_jump_over_another_actors_claim_deadline() {
    let t = trial("consumption", 2);
    let (boundary, choice, claim) = t
        .boundaries
        .iter()
        .find_map(|b| {
            let claim = b.active.iter().find(|c| c.work == x::Work::Claim)?;
            let choice = b.choices.iter().find(|d| {
                !d.input.inquiry_lane_free
                    && matches!(
                        d.input.legacy.query.decision.selected,
                        inquiry::Action::Ask { .. }
                    )
            })?;
            Some((b, choice, claim))
        })
        .expect("registered actual masked-query/live-claim witness");
    assert!(claim.start < boundary.tick && boundary.tick < claim.deadline);
    assert!(!claim.participants.contains(&choice.input.legacy.own.id));
    assert!(choice.contact_candidates.iter().any(|c| {
        matches!(
            c.intent,
            world_of_individuals::encounters::Intent::Invite {
                kind: world_of_individuals::encounters::Kind::Inquiry,
                ..
            }
        ) && !c.eligible
            && c.score == -1
    }));
    let result = t.grants.iter().find(|g| g.commitment == claim.id).unwrap();
    assert_eq!(result.tick, claim.deadline);
    assert!(
        t.transitions
            .iter()
            .filter(|r| !r.completed && r.commitment.work == x::Work::Query)
            .all(|q| {
                let end = t
                    .transitions
                    .iter()
                    .find(|r| r.completed && r.commitment.id == q.commitment.id)
                    .unwrap()
                    .tick;
                end <= claim.start || q.tick >= claim.deadline
            })
    );
}

#[test]
fn a_scene_claim_and_meal_can_occupy_all_four_people_under_one_clock() {
    let t = trial("paired", 1);
    let starts = t
        .transitions
        .iter()
        .filter(|r| !r.completed)
        .collect::<Vec<_>>();
    let active_at = |tick| {
        starts
            .iter()
            .filter(|start| {
                let end = t
                    .transitions
                    .iter()
                    .find(|r| r.completed && r.commitment.id == start.commitment.id)
                    .unwrap()
                    .tick;
                start.tick <= tick && tick < end
            })
            .map(|start| &start.commitment)
            .collect::<Vec<_>>()
    };
    let active = (t.start..t.end)
        .map(active_at)
        .find(|active| {
            active
                .iter()
                .any(|c| matches!(c.work, x::Work::Resource { .. }))
                && active.iter().any(|c| c.work == x::Work::Claim)
                && active.iter().any(|c| c.work == x::Work::Meal)
        })
        .expect("recorded three-activity overlap");
    let actors = active
        .iter()
        .flat_map(|c| c.participants.iter().copied())
        .collect::<Vec<_>>();
    assert_eq!(actors.len(), 4);
    assert_eq!(
        actors
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        4
    );
    assert_eq!(t.needs.len(), (t.end - t.start) as usize / 4);
    assert_eq!(
        t.initial_total,
        t.final_agents
            .iter()
            .map(|a| u64::from(a.food))
            .sum::<u64>()
            + u64::from(t.final_stock)
            + u64::from(
                t.final_state
                    .base
                    .base
                    .base
                    .base
                    .base
                    .cognition
                    .consumption
                    .iter()
                    .map(|m| m.consumed)
                    .sum::<u32>()
            )
    );
}
