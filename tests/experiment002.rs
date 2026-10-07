use world_of_individuals::{
    behavior::*, cognition::*, experiment002::*, model::*, simulation::Simulation,
};

fn refusal(food: u32) -> Simulation {
    let mut g = Generator::new(42);
    let mut sim = Simulation::new(vec![g.agent(0), g.agent(1)]).unwrap();
    sim.set_policy(scarcity_policy);
    sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
    sim.set_circumstances(1, food, 90, 0, 90).unwrap();
    sim.add_scene([0, 1], 1, 6).unwrap();
    sim.run();
    assert_eq!(sim.events()[1].decision.selected, Action::Refuse);
    sim
}
#[test]
fn controlled_information_history_and_conflict_sweep() {
    for seed in 0..128 {
        let t = suite(seed);
        assert_eq!(t, suite(seed));
        assert_eq!(t[0].probe.selected, Action::Leave);
        assert_eq!(t[1].probe.selected, Action::Offer);
        assert_eq!(t[4].probe.selected, Action::Leave);
        assert!(
            t[4].cognition
                .information
                .iter()
                .all(|i| i.revision.is_some())
        );
        assert!(t[3].cognition.information[0].revision.is_none());
        assert!(t[5].cognition.information[0].revision.is_none());
        assert!(t[7].cognition.information[1].revision.is_some());
        assert_eq!(t[7].probe.selected, Action::Leave); // One revised event cannot erase the rest.
        let physical = |trial: &Trial| {
            let mut agents = trial.snapshots[trial.snapshots.len() - 2].agents.clone();
            for a in &mut agents {
                a.trust.clear();
                a.memories.clear();
            }
            agents
        };
        assert_eq!(physical(&t[0]), physical(&t[1]));
        assert_eq!(physical(&t[5]), physical(&t[6]));
        assert_eq!(t[0].events[..6], t[1].events[..6]);
        for trial in t {
            for info in &trial.cognition.information {
                assert!(info.terminated && info.turns == 1);
                let e = &trial.events[info.event as usize];
                assert_eq!(e.decision.selected, Action::Refuse);
                assert!(e.tick < info.tick);
                assert_eq!(
                    e.interpretations[0].interpretation,
                    Interpretation::Rejected
                );
                if let Some(r) = &info.revision {
                    assert_eq!(r.before.event, r.after.event);
                    assert_eq!(r.before.observed, r.after.observed);
                }
            }
        }
    }
}
#[test]
fn consumption_accounting_unmet_need_and_bounds() {
    let mut sim = refusal(1);
    let before = sim.agents();
    let no_food = sim.consume(0).unwrap();
    assert_eq!(no_food.consumed, 0);
    assert_eq!(no_food.hunger_after, 90);
    let meal = sim.consume(1).unwrap();
    assert_eq!(
        (meal.food_after, meal.hunger_after, meal.consumed),
        (0, 50, 1)
    );
    assert_eq!(sim.consume(1).unwrap().consumed, 0);
    sim.set_circumstances(1, u32::MAX, 10, 0, 0).unwrap();
    assert_eq!(sim.consume(1).unwrap().hunger_after, 0);
    assert_eq!(sim.consume(1).unwrap().consumed, 0);
    assert_eq!(before[0].food, sim.agents()[0].food);
    for c in sim.cognition().consumption {
        assert_eq!(c.food_before - c.food_after, c.consumed);
        assert!((0..=100).contains(&c.hunger_after));
    }
    assert!(sim.consume(99).is_err());
}
#[test]
fn claims_are_local_not_truth_checked_and_disclosure_is_event_specific() {
    let mut scarce = refusal(1);
    let mut abundant = refusal(4);
    let claim = EvidenceKind::Testimony { scarce: true };
    assert_eq!(
        scarce.communicate(1, 0, 1, claim).unwrap(),
        abundant.communicate(1, 0, 1, claim).unwrap()
    );
    assert_eq!(scarce.agents()[0], abundant.agents()[0]);
    // Changing present state cannot rewrite the historical observation.
    scarce.set_circumstances(1, 100, 0, 0, 0).unwrap();
    assert!(
        scarce
            .communicate(1, 0, 1, EvidenceKind::Disclosure)
            .unwrap()
            .scarce
    );
    assert!(
        !abundant
            .communicate(1, 0, 1, EvidenceKind::Disclosure)
            .unwrap()
            .scarce
    );
    assert_eq!(
        scarce.events()[1].interpretations[0].interpretation,
        Interpretation::Rejected
    );
}
#[test]
fn duplicate_evidence_does_not_double_credit_or_escalate() {
    let mut generator = Generator::new(42);
    let mut recipient = generator.agent(0);
    recipient.trust.insert(1, 80);
    let mut trusted = Simulation::new(vec![recipient, generator.agent(1)]).unwrap();
    trusted.set_circumstances(0, 0, 90, 20, 20).unwrap();
    trusted.set_circumstances(1, 1, 90, 0, 90).unwrap();
    trusted.add_scene([0, 1], 1, 6).unwrap();
    trusted.run();
    let first_claim = trusted
        .communicate(1, 0, 1, EvidenceKind::Testimony { scarce: true })
        .unwrap();
    assert!(first_claim.revision.is_some());
    let repeated = trusted
        .communicate(1, 0, 1, EvidenceKind::Testimony { scarce: true })
        .unwrap();
    assert_eq!(first_claim.after, repeated.after);
    assert_eq!(first_claim.reliability, repeated.reliability);
    assert!(repeated.revision.is_none());
    let mut sim = refusal(1);
    let first = sim.communicate(1, 0, 1, EvidenceKind::Disclosure).unwrap();
    let agents = sim.agents();
    for _ in 0..20 {
        let duplicate = sim.communicate(1, 0, 1, EvidenceKind::Disclosure).unwrap();
        assert!(duplicate.revision.is_none());
        assert_eq!(duplicate.after, first.after);
        assert_eq!(sim.agents(), agents);
    }
    let weak = sim
        .communicate(1, 0, 1, EvidenceKind::Testimony { scarce: false })
        .unwrap();
    assert_eq!(weak.after.support, 100);
    assert!(weak.revision.is_none());
}
#[test]
fn information_validation_is_atomic_and_scenes_terminate() {
    let mut sim = refusal(1);
    let before = sim.cognition();
    for (speaker, listener, event) in [
        (0, 1, 1),
        (1, 1, 1),
        (99, 0, 1),
        (1, 99, 1),
        (1, 0, 0),
        (1, 0, u64::MAX),
    ] {
        assert!(
            sim.communicate(speaker, listener, event, EvidenceKind::Disclosure)
                .is_err()
        );
    }
    assert_eq!(before, sim.cognition());
    sim.add_scene([0, 1], 1, 1).unwrap();
    assert!(sim.communicate(1, 0, 1, EvidenceKind::Disclosure).is_err());
    assert!(sim.consume(1).is_err());
    sim.run();
    assert!(
        sim.scenes()
            .iter()
            .all(|s| s.outcome.is_some() && s.turns <= s.max_turns)
    );
}
#[test]
fn scarce_and_abundant_donors_have_distinct_choices() {
    let mut g = Generator::new(0);
    let mut a = g.agent(0);
    a.food = 1;
    a.hunger = 60;
    a.generosity = 80;
    a.caution = 0;
    let view = Observation {
        partner: 1,
        last_signal: Some(Signal {
            actor: 1,
            action: Action::Request,
        }),
        amount: 1,
        turns_left: 4,
    };
    assert_eq!(scarcity_policy(&a, &view).selected, Action::Leave);
    a.food = 4;
    assert_eq!(scarcity_policy(&a, &view).selected, Action::Offer);
    assert!(legal_actions(&a, &view).contains(&scarcity_policy(&a, &view).selected));
}
#[test]
fn bounded_beliefs_and_evicted_memory_cannot_be_rewritten() {
    let mut sim = refusal(1);
    for _ in 0..30 {
        sim.set_circumstances(0, 0, 100, 0, 0).unwrap();
        sim.set_circumstances(1, 1, 90, 0, 90).unwrap();
        sim.add_scene([0, 1], 1, 6).unwrap();
        sim.run();
        let event = sim.events().last().unwrap().id;
        sim.communicate(1, 0, event, EvidenceKind::Disclosure)
            .unwrap();
    }
    assert_eq!(sim.cognition().beliefs[&0].len(), MEMORY_CAPACITY);
    assert_eq!(sim.agents()[0].memories.len(), MEMORY_CAPACITY);
    let trust = sim.agents()[0].trust.clone();
    let old = sim.communicate(1, 0, 1, EvidenceKind::Disclosure).unwrap();
    assert!(old.revision.is_none());
    assert_eq!(trust, sim.agents()[0].trust);
}
#[test]
fn saturated_trust_replay_uses_replacement_not_naive_refund() {
    let mut sim = refusal(1);
    for _ in 0..10 {
        sim.set_circumstances(0, 0, 100, 0, 0).unwrap();
        sim.set_circumstances(1, 1, 90, 0, 90).unwrap();
        sim.add_scene([0, 1], 1, 6).unwrap();
        sim.run();
    }
    assert_eq!(sim.agents()[0].trust[&1], -100);
    let event = sim.events().last().unwrap().id;
    sim.communicate(1, 0, event, EvidenceKind::Disclosure)
        .unwrap();
    assert_eq!(sim.agents()[0].trust[&1], -100);
}
#[test]
fn population_accounting_replay_and_policy_locality() {
    for seed in [0, 42, u64::MAX] {
        let p = population(seed, 101);
        assert_eq!(
            serde_json::to_vec(&p).unwrap(),
            serde_json::to_vec(&population(seed, 101)).unwrap()
        );
        let initial: u64 = p.initial.iter().map(|a| u64::from(a.food)).sum();
        let final_food: u64 = p.final_agents.iter().map(|a| u64::from(a.food)).sum();
        let consumed: u64 = p
            .cognition
            .consumption
            .iter()
            .map(|c| u64::from(c.consumed))
            .sum();
        assert_eq!(initial, final_food + consumed);
        for e in &p.events {
            assert_eq!(
                e.balances_before.iter().map(|&n| u64::from(n)).sum::<u64>(),
                e.balances_after.iter().map(|&n| u64::from(n)).sum::<u64>()
            );
        }
        assert!(p.scenes.iter().all(|s| s.outcome.is_some()));
    }
}

#[test]
fn population_audit_reconstructs_every_state_and_decision() {
    let p = population(42, 1000);
    let mut agents = p.initial.clone();
    for e in &p.events {
        let actor = &agents[e.decision.actor as usize];
        assert_eq!(e.decision, scarcity_policy(actor, &e.decision.observation));
        for i in 0..2 {
            let a = &mut agents[e.participants[i] as usize];
            assert_eq!(a.food, e.balances_before[i]);
            a.food = e.balances_after[i];
            remember(a, e.interpretations[i].clone());
        }
    }
    let mut previous_tick = 0;
    for info in &p.cognition.information {
        assert!(info.tick > previous_tick);
        previous_tick = info.tick;
        let a = &mut agents[info.listener as usize];
        assert_eq!(*a.trust.get(&info.speaker).unwrap_or(&0), info.trust_before);
        if let Some(r) = &info.revision {
            let m = a
                .memories
                .iter_mut()
                .find(|m| m.event == info.event)
                .unwrap();
            assert_eq!(*m, r.before);
            *m = r.after.clone();
            a.trust.insert(info.speaker, info.trust_after);
        } else {
            assert_eq!(info.trust_before, info.trust_after);
        }
    }
    for c in &p.cognition.consumption {
        assert!(c.tick > previous_tick);
        previous_tick = c.tick;
        let a = &mut agents[c.actor as usize];
        assert_eq!((a.food, a.hunger), (c.food_before, c.hunger_before));
        a.food = c.food_after;
        a.hunger = c.hunger_after;
    }
    assert_eq!(agents, p.final_agents);
}
