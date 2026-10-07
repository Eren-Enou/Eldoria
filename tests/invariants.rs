use world_of_individuals::{
    behavior::*, experiment::run_experiment, model::*, simulation::Simulation,
};

fn pair(seed: u64) -> Vec<Agent> {
    let mut g = Generator::new(seed);
    vec![g.agent(0), g.agent(1)]
}

#[test]
fn generated_agents_valid_distinct_and_reproducible() {
    for seed in 0..512 {
        let agents = pair(seed);
        assert_eq!(agents, pair(seed));
        assert_ne!(agents[0].id, agents[1].id);
        for a in &agents {
            assert!(a.food <= 6);
            for value in [a.hunger, a.generosity, a.caution, a.expectation] {
                assert!((0..=100).contains(&value));
            }
            assert!(a.memories.is_empty());
            assert!(a.trust.is_empty());
        }
    }
    assert_ne!(pair(0), pair(1));
}

#[test]
fn history_changes_choices_at_identical_present_across_seeds() {
    for seed in 0..128 {
        let report = run_experiment(seed, "E", 2).unwrap();
        let c = report.history_comparison.unwrap();
        assert!(c.identical_present_except_learning);
        assert_eq!(c.helpful_history_action, Action::Offer);
        assert_eq!(c.refusal_history_action, Action::Leave);
        assert_eq!(c.history_free_action, Action::Leave);
        assert_eq!(c.helpful_history_outcome, Outcome::Agreement);
        assert_eq!(c.refusal_history_outcome, Outcome::Withdrawal);
        let helpful = &report.trials[0];
        let decision = &helpful.events[helpful.stages[1].first_event].decision;
        assert!(decision.trust > 0);
        assert!(decision.episodic_valence > 0);
        assert!(
            decision
                .memory_causes
                .iter()
                .all(|&id| id < helpful.stages[1].first_event as u64)
        );
    }
}

#[test]
fn entire_reports_are_byte_reproducible() {
    for seed in [0, 42, u64::MAX] {
        let a = run_experiment(seed, "all", 101).unwrap();
        let b = run_experiment(seed, "all", 101).unwrap();
        assert_eq!(
            serde_json::to_vec(&a).unwrap(),
            serde_json::to_vec(&b).unwrap()
        );
        assert!(a.reproducible);
    }
}

#[test]
fn private_partner_state_cannot_change_unobserved_decision() {
    let mut agents = pair(42);
    agents[0].food = 0;
    agents[0].hunger = 90;
    let mut other = agents.clone();
    other[1].food = u32::MAX;
    other[1].hunger = 0;
    other[1].generosity = 100;
    other[1].trust.insert(0, -100);
    other[1].caution = 100;
    let mut a = Simulation::new(agents).unwrap();
    let mut b = Simulation::new(other).unwrap();
    a.add_scene([0, 1], 1, 6).unwrap();
    b.add_scene([0, 1], 1, 6).unwrap();
    a.run();
    b.run();
    assert_eq!(a.events()[0].decision, b.events()[0].decision);
    // Later responses may differ: those are public signals legitimately observed.
}

#[test]
fn refusal_has_distinct_subjective_interpretations() {
    let report = run_experiment(42, "D", 2).unwrap();
    let event = report.trials[0]
        .events
        .iter()
        .find(|e| e.decision.selected == Action::Refuse)
        .unwrap();
    assert_eq!(
        event.interpretations[0].interpretation,
        Interpretation::Rejected
    );
    assert!(event.interpretations[0].valence < 0);
    assert_eq!(
        event.interpretations[1].interpretation,
        Interpretation::ProtectedOwnNeeds
    );
    assert_eq!(event.interpretations[1].valence, 0);
}

#[test]
fn property_sweep_accounting_legality_traceability_and_termination() {
    for seed in 0..256 {
        let report = run_experiment(seed, "all", 20).unwrap();
        for trial in &report.trials {
            for scene in &trial.scenes {
                assert!(scene.outcome.is_some());
                assert!(scene.turns <= scene.max_turns);
            }
            for stage in &trial.stages {
                let mut states = stage.before.clone();
                for event in &trial.events[stage.first_event..stage.end_event] {
                    assert_eq!(
                        event.id as usize,
                        trial.events.iter().position(|e| e.id == event.id).unwrap()
                    );
                    let before_total: u64 =
                        event.balances_before.iter().map(|&v| u64::from(v)).sum();
                    let after_total: u64 = event.balances_after.iter().map(|&v| u64::from(v)).sum();
                    assert_eq!(before_total, after_total);
                    let actor = states
                        .iter()
                        .find(|a| a.id == event.decision.actor)
                        .unwrap();
                    assert_eq!(
                        event.decision,
                        utility_policy(actor, &event.decision.observation)
                    );
                    assert!(
                        legal_actions(actor, &event.decision.observation)
                            .contains(&event.decision.selected)
                    );
                    assert!(event.decision.memory_causes.iter().all(|&id| id < event.id));
                    if event.transferred > 0 {
                        assert_eq!(event.decision.selected, Action::Accept);
                        let actor_index = event
                            .participants
                            .iter()
                            .position(|&id| id == event.decision.actor)
                            .unwrap();
                        assert!(event.balances_before[1 - actor_index] >= event.transferred);
                        assert_eq!(
                            event.balances_after[actor_index] - event.balances_before[actor_index],
                            event.transferred
                        );
                        assert_eq!(
                            event.balances_before[1 - actor_index]
                                - event.balances_after[1 - actor_index],
                            event.transferred
                        );
                    }
                    for i in 0..2 {
                        let agent = states
                            .iter_mut()
                            .find(|a| a.id == event.participants[i])
                            .unwrap();
                        assert_eq!(agent.food, event.balances_before[i]);
                        agent.food = event.balances_after[i];
                        assert_eq!(event.interpretations[i].event, event.id);
                        remember(agent, event.interpretations[i].clone());
                    }
                }
                assert_eq!(states, stage.after);
            }
            for agent in &trial.stages.last().unwrap().after {
                assert!(agent.memories.len() <= MEMORY_CAPACITY);
                for memory in &agent.memories {
                    let event = &trial.events[memory.event as usize];
                    assert!(event.participants.contains(&agent.id));
                    assert!(event.participants.contains(&memory.partner));
                    assert_eq!(memory.observed, event.decision.selected);
                }
            }
        }
    }
}

#[test]
fn invalid_participants_amounts_and_overlapping_scenes_rejected() {
    let mut sim = Simulation::new(pair(0)).unwrap();
    assert!(sim.add_scene([0, 0], 1, 6).is_err());
    assert!(sim.add_scene([0, 99], 1, 6).is_err());
    assert!(sim.add_scene([0, 1], 0, 6).is_err());
    assert!(sim.add_scene([0, 1], 1, 0).is_err());
    sim.add_scene([0, 1], 1, 6).unwrap();
    assert!(sim.add_scene([1, 0], 1, 6).is_err());
    assert!(sim.set_circumstances(0, 4, 50, 50, 50).is_err());
    sim.run();
    assert!(sim.add_scene([1, 0], 1, 6).is_ok());
    assert!(Simulation::new(vec![pair(0)[0].clone(); 2]).is_err());
    let mut invalid = pair(0);
    invalid[0].trust.insert(1, i32::MAX);
    assert!(Simulation::new(invalid).is_err());
    let mut invalid = pair(0);
    invalid[0].hunger = -1;
    assert!(Simulation::new(invalid).is_err());
    let mut invalid = pair(0);
    invalid[0].memories.push_back(Memory {
        event: 99,
        partner: 1,
        observed: Action::Refuse,
        interpretation: Interpretation::Rejected,
        valence: -10,
    });
    assert!(Simulation::new(invalid).is_err());
}

fn requests_only(agent: &Agent, observation: &Observation) -> Decision {
    let mut d = utility_policy(agent, observation);
    // Invalid on a request response, exercises resolver rejection.
    d.selected = Action::Request;
    d
}
fn offers_only(agent: &Agent, observation: &Observation) -> Decision {
    let mut d = utility_policy(agent, observation);
    d.selected = Action::Offer;
    d
}

#[test]
fn inability_timeout_and_overflow_are_safe() {
    let mut sim = Simulation::new(pair(0)).unwrap();
    sim.set_policy(requests_only);
    sim.add_scene([0, 1], 1, 6).unwrap();
    sim.run();
    assert_eq!(sim.scenes()[0].outcome, Some(Outcome::Inability));
    assert_eq!(sim.events().last().unwrap().transferred, 0);
    let mut sim = Simulation::new(pair(0)).unwrap();
    sim.set_policy(offers_only);
    sim.set_circumstances(0, 4, 0, 100, 0).unwrap();
    sim.add_scene([0, 1], 1, 1).unwrap();
    sim.run();
    assert_eq!(sim.scenes()[0].outcome, Some(Outcome::Timeout));
    let mut sim = Simulation::new(pair(0)).unwrap();
    sim.set_circumstances(0, 4, 0, 100, 0).unwrap();
    sim.set_circumstances(1, u32::MAX, 100, 0, 0).unwrap();
    sim.add_scene([0, 1], 1, 6).unwrap();
    sim.run();
    assert_ne!(sim.scenes()[0].outcome, Some(Outcome::Agreement));
    assert_eq!(sim.agents()[1].food, u32::MAX);
}

#[test]
fn bounded_memory_retains_aggregate_relationship_and_valid_event_ids() {
    let mut sim = Simulation::new(pair(0)).unwrap();
    for _ in 0..40 {
        sim.set_circumstances(0, 4, 0, 100, 0).unwrap();
        sim.set_circumstances(1, 0, 100, 0, 0).unwrap();
        sim.add_scene([0, 1], 1, 6).unwrap();
        sim.run();
    }
    let events = sim.events();
    for a in sim.agents() {
        assert_eq!(a.memories.len(), MEMORY_CAPACITY);
        assert!(*a.trust.get(&(1 - a.id)).unwrap() > 0);
        assert!(a.memories.front().unwrap().event > 0);
        for m in a.memories {
            assert!((m.event as usize) < events.len());
        }
    }
}

#[test]
fn population_handles_odd_count_and_thousand_agents() {
    for size in [101, 1000] {
        let report = run_experiment(42, "G", size).unwrap();
        let trial = &report.trials[0];
        assert_eq!(trial.scenes.len(), size / 2);
        assert!(
            trial
                .events
                .iter()
                .all(|e| e.participants.iter().all(|&id| (id as usize) < size))
        );
        if size % 2 == 1 {
            assert_eq!(trial.generated.last(), trial.stages[0].after.last());
        }
    }
}

#[test]
fn own_history_changes_policy_without_partner_information() {
    let report = run_experiment(42, "E", 2).unwrap();
    let trial = &report.trials[0];
    let a = &trial.stages[1].before[0];
    let d = &trial.events[trial.stages[1].first_event].decision;
    let mut fresh = a.clone();
    fresh.trust.clear();
    fresh.memories.clear();
    assert_ne!(
        utility_policy(a, &d.observation).selected,
        utility_policy(&fresh, &d.observation).selected
    );
}
