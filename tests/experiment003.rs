use std::collections::BTreeMap;
use world_of_individuals::{
    behavior::{remember, scarcity_policy},
    cognition::revise_belief,
    experiment003::*,
    intentional::*,
    model::*,
    simulation::Simulation,
};
fn target_records(t: &Trial) -> Vec<&TalkRecord> {
    t.intentional
        .records
        .iter()
        .filter(|r| r.decision.input.event == t.target)
        .collect()
}
fn configured() -> Simulation {
    let mut g = Generator::new(42);
    let mut sim = Simulation::new(vec![g.agent(0), g.agent(1)]).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_intentional().unwrap();
    sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
    sim.set_circumstances(1, 1, 90, 0, 90).unwrap();
    sim.set_communication_profile(0, Profile::default())
        .unwrap();
    sim.set_communication_profile(
        1,
        Profile {
            privacy: 0,
            ..Profile::default()
        },
    )
    .unwrap();
    sim
}
#[test]
fn counterfactual_sweep_disclosure_requests_evidence_and_history() {
    for seed in 0..128 {
        let t = suite(seed);
        assert_eq!(t, suite(seed));
        assert_eq!(t[0].probe.selected, Action::Offer);
        assert_eq!(t[1].probe.selected, Action::Leave);
        assert_eq!(
            target_records(&t[0])[0].decision.selected,
            TalkAction::Explain
        );
        assert_eq!(
            target_records(&t[1])[0].decision.selected,
            TalkAction::Silence
        );
        assert_eq!(
            target_records(&t[3])[1].decision.selected,
            TalkAction::Silence
        );
        assert_eq!(
            target_records(&t[4])[1].decision.selected,
            TalkAction::AskExplanation
        );
        // Requests produce information, but unverified information need not change behavior.
        assert_eq!(t[3].probe.selected, t[4].probe.selected);
        assert!(t[3].cognition.information.is_empty());
        assert_eq!(t[4].cognition.information[0].after.support, 50);
        assert_eq!(t[5].probe.selected, Action::Leave);
        assert_eq!(t[6].probe.selected, Action::Offer);
        assert_eq!(t[7].probe.selected, Action::Offer);
        assert_eq!(t[8].probe.selected, Action::Leave);
        assert_eq!(
            target_records(&t[14])[1].decision.selected,
            TalkAction::AskExplanation
        );
        assert_eq!(
            target_records(&t[15])[1].decision.selected,
            TalkAction::Silence
        );
        let physical = |trial: &Trial| {
            let mut a = trial.snapshots[trial.snapshots.len() - 2].agents.clone();
            for agent in &mut a {
                agent.trust.clear();
                agent.memories.clear();
            }
            a
        };
        for (a, b) in [(0, 1), (3, 4), (5, 6), (7, 8), (14, 15)] {
            assert_eq!(physical(&t[a]), physical(&t[b]));
        }
        assert_eq!(t[0].events[..2], t[1].events[..2]);
        assert_eq!(t[5].events[..2], t[6].events[..2]);
    }
}
#[test]
fn history_ablation_isolates_request_choice_and_credibility() {
    let t = suite(42);
    let a = target_records(&t[14]);
    let b = target_records(&t[15]);
    let mut without_history = b[1].decision.input.clone();
    without_history.memories.clear();
    assert_eq!(
        policy(&without_history).selected,
        TalkAction::AskExplanation
    );
    let mut swapped = b[1].decision.input.clone();
    swapped.memories = a[1].decision.input.memories.clone();
    assert_eq!(policy(&swapped).selected, TalkAction::AskExplanation);
    assert_eq!(t[7].cognition.information.last().unwrap().after.support, 70);
    assert_eq!(t[8].cognition.information.last().unwrap().after.support, 50);
    assert_eq!(t[9].cognition.information.last().unwrap().after.support, 20);
}
#[test]
fn deception_has_private_knowledge_incentive_and_no_intent_leak() {
    let t = suite(42);
    let deceptive = target_records(&t[11]);
    let honest = target_records(&t[12]);
    assert_eq!(deceptive[0].decision.selected, TalkAction::Mislead);
    assert_eq!(
        deceptive[0].decision.input.own_historical_scarcity,
        Some(false)
    );
    assert_eq!(deceptive[0].objectively_true, Some(false));
    assert_ne!(honest[0].decision.selected, TalkAction::Mislead);
    assert_eq!(deceptive[1].decision.input.last, Some(TalkAction::Explain));
    assert!(
        deceptive[1]
            .decision
            .input
            .own_historical_scarcity
            .is_none()
    );
    assert!(
        deceptive[1]
            .decision
            .input
            .memories
            .iter()
            .all(|m| m.action != TalkAction::Mislead && m.verified_claim.is_none())
    );
    let challenge = target_records(&t[13]);
    assert_eq!(challenge[2].verified_claim, Some(false));
    assert_eq!(challenge[2].credibility_after, -30);
    assert_eq!(
        t[13].cognition.information.last().unwrap().after.support,
        -100
    );
    // Removing the relationship goal removes the motive while keeping knowledge fixed.
    let mut no_gain = deceptive[0].decision.input.clone();
    no_gain.profile.relationship_goal = 0;
    assert_eq!(policy(&no_gain).selected, TalkAction::Silence);
}
#[test]
fn false_and_true_claims_are_indistinguishable_until_evidence() {
    let a = trial(42, "D-true", 0, 0, 100, 0, 0);
    let b = trial(42, "G-false", 0, 0, 100, 0, 0);
    let ar = target_records(&a);
    let br = target_records(&b);
    assert_ne!(ar[0].objectively_true, br[0].objectively_true);
    assert_eq!(ar[1].decision, br[1].decision);
    assert_eq!(a.cognition.information, b.cognition.information);
}
fn invalid(input: &TalkInput) -> TalkDecision {
    let mut d = policy(input);
    d.selected = TalkAction::AskEvidence;
    d
}
#[test]
fn invalid_policy_terminates_without_information_or_state_changes() {
    let mut sim = configured();
    sim.set_communication_policy(invalid).unwrap();
    sim.add_scene([0, 1], 1, 6).unwrap();
    sim.run();
    let state = sim.intentional().unwrap();
    assert_eq!(state.conversations[0].outcome, "InvalidAction");
    assert_eq!(state.records.len(), 1);
    assert!(!state.records[0].valid);
    assert!(state.memories.is_empty());
    assert!(sim.cognition().information.is_empty());
    assert_eq!(state.records[0].food_before, state.records[0].food_after);
    sim.run();
    assert_eq!(sim.intentional().unwrap(), state);
}
#[test]
fn profile_validation_and_mode_lifecycle_are_atomic() {
    let mut sim = configured();
    let before = sim.intentional();
    assert!(
        sim.set_communication_profile(99, Profile::default())
            .is_err()
    );
    assert!(
        sim.set_communication_profile(
            0,
            Profile {
                privacy: -1,
                ..Profile::default()
            }
        )
        .is_err()
    );
    assert_eq!(before, sim.intentional());
    sim.add_scene([0, 1], 1, 6).unwrap();
    assert!(
        sim.set_communication_profile(0, Profile::default())
            .is_err()
    );
    assert!(sim.enable_intentional().is_err());
    sim.run();
    let before = sim.intentional();
    sim.enable_intentional().unwrap();
    sim.run();
    assert_eq!(before, sim.intentional());
}
#[test]
fn episode_bounds_and_credibility_saturation() {
    let mut sim = configured();
    for _ in 0..30 {
        sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
        sim.set_circumstances(1, 1, 90, 0, 90).unwrap();
        sim.add_scene([0, 1], 1, 6).unwrap();
        sim.run();
    }
    let state = sim.intentional().unwrap();
    assert_eq!(state.credibility[&0][&1], 40);
    for m in state.memories.values() {
        assert_eq!(m.len(), MEMORY_CAPACITY);
        assert!(m.front().unwrap().record > 0);
    }
    assert!(
        state
            .conversations
            .iter()
            .all(|c| c.end_record - c.first_record <= 4)
    );
    assert!(
        sim.agents()
            .iter()
            .all(|a| a.memories.len() <= MEMORY_CAPACITY)
    );
    assert!(
        sim.cognition()
            .beliefs
            .values()
            .all(|b| b.len() <= MEMORY_CAPACITY)
    );
}
#[test]
fn deterministic_population_and_explicit_accounting() {
    for seed in [0, 42, u64::MAX] {
        let p = population(seed, 101);
        assert_eq!(
            serde_json::to_vec(&p).unwrap(),
            serde_json::to_vec(&population(seed, 101)).unwrap()
        );
        let before: u64 = p.initial.iter().map(|a| u64::from(a.food)).sum();
        let after: u64 = p.final_agents.iter().map(|a| u64::from(a.food)).sum();
        assert_eq!(
            before,
            after
                + p.cognition
                    .consumption
                    .iter()
                    .map(|c| u64::from(c.consumed))
                    .sum::<u64>()
        );
        assert!(
            p.intentional
                .records
                .iter()
                .all(|r| r.food_before == r.food_after && r.time_spent == 1)
        );
        assert!(
            p.intentional
                .conversations
                .iter()
                .all(|c| c.end_record - c.first_record <= 4 && c.end_record > c.first_record)
        );
    }
}
#[test]
fn complete_population_audit_reconstructs_local_inputs_and_consequences() {
    let p = population(42, 1000);
    let mut agents = p.initial.clone();
    let mut beliefs = BTreeMap::new();
    let mut memories: BTreeMap<AgentId, Vec<TalkMemory>> = BTreeMap::new();
    let mut credibility: BTreeMap<(AgentId, AgentId), i32> = BTreeMap::new();
    for e in &p.events {
        assert_eq!(
            e.decision,
            scarcity_policy(&agents[e.decision.actor as usize], &e.decision.observation)
        );
        for i in 0..2 {
            let a = &mut agents[e.participants[i] as usize];
            a.food = e.balances_after[i];
            remember(a, e.interpretations[i].clone());
        }
    }
    let mut tick = p.events.last().map_or(0, |e| e.tick);
    for r in &p.intentional.records {
        let input = &r.decision.input;
        assert!(r.valid);
        assert_eq!(r.decision, policy(input));
        assert_eq!(input.own, agents[input.own.id as usize]);
        assert_eq!(
            input.memories,
            memories.get(&input.own.id).cloned().unwrap_or_default()
        );
        assert_eq!(
            input.credibility,
            *credibility
                .get(&(input.own.id, input.partner))
                .unwrap_or(&0)
        );
        assert_eq!(
            input.belief,
            beliefs.get(&(input.own.id, input.event)).cloned()
        );
        assert_eq!(r.tick, tick + 1);
        tick = r.tick;
        let conversation = &p.intentional.conversations[r.conversation as usize];
        let [speaker, listener] = conversation.participants;
        if let Some(id) = r.information {
            let info = &p.cognition.information[id as usize];
            assert_eq!(info.before, beliefs.get(&(listener, input.event)).cloned());
            assert_eq!(
                revise_belief(info.before.as_ref(), info.after.clone()),
                info.after
            );
            beliefs.insert((listener, input.event), info.after.clone());
            if let Some(revision) = &info.revision {
                let a = &mut agents[listener as usize];
                let m = a
                    .memories
                    .iter_mut()
                    .find(|m| m.event == input.event)
                    .unwrap();
                assert_eq!(*m, revision.before);
                *m = revision.after.clone();
                assert_eq!(a.trust[&speaker], info.trust_before);
                a.trust.insert(speaker, info.trust_after);
            }
        }
        if r.verified_claim.is_some() {
            credibility.insert((listener, speaker), r.credibility_after);
        }
        for owner in [speaker, listener] {
            let action = if owner != input.own.id && r.decision.selected == TalkAction::Mislead {
                TalkAction::Explain
            } else {
                r.decision.selected
            };
            let episodes = memories.entry(owner).or_default();
            if episodes.len() == MEMORY_CAPACITY {
                episodes.remove(0);
            }
            episodes.push(TalkMemory {
                record: r.id,
                event: input.event,
                partner: if owner == speaker { listener } else { speaker },
                actor: input.own.id,
                action,
                response_to: input.last,
                claim: r.claim,
                verified_claim: r.verified_claim,
            });
        }
    }
    for c in &p.cognition.consumption {
        assert_eq!(c.tick, tick + 1);
        tick = c.tick;
        let a = &mut agents[c.actor as usize];
        assert_eq!((a.food, a.hunger), (c.food_before, c.hunger_before));
        a.food = c.food_after;
        a.hunger = c.hunger_after;
    }
    assert_eq!(agents, p.final_agents);
}

#[test]
fn hidden_partner_preferences_do_not_change_opening_decision() {
    let mut a = configured();
    let mut b = configured();
    b.set_communication_profile(
        0,
        Profile {
            privacy: 100,
            relationship_goal: 0,
            honesty: 0,
            request_cost: 100,
        },
    )
    .unwrap();
    for sim in [&mut a, &mut b] {
        sim.add_scene([0, 1], 1, 6).unwrap();
        sim.run();
    }
    assert_eq!(
        a.intentional().unwrap().records[0].decision,
        b.intentional().unwrap().records[0].decision
    );
}
#[test]
fn enabling_mode_does_not_reopen_historical_refusals() {
    let mut g = Generator::new(42);
    let mut sim = Simulation::new(vec![g.agent(0), g.agent(1)]).unwrap();
    sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
    sim.set_circumstances(1, 1, 90, 0, 90).unwrap();
    sim.add_scene([0, 1], 1, 6).unwrap();
    sim.run();
    assert_eq!(sim.events().len(), 2);
    sim.enable_intentional().unwrap();
    sim.run();
    assert!(sim.intentional().unwrap().conversations.is_empty());
}
