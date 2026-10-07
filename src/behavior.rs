use crate::model::*;

pub fn legal_actions(agent: &Agent, view: &Observation) -> Vec<Action> {
    let mut actions = vec![Action::Leave];
    match view.last_signal.map(|s| s.action) {
        Some(Action::Offer) => {
            if agent.food.checked_add(view.amount).is_some() {
                actions.push(Action::Accept);
            }
            actions.push(Action::Refuse);
        }
        Some(Action::Request) => {
            if agent.food >= view.amount {
                actions.push(Action::Offer);
            }
            actions.push(Action::Refuse);
        }
        _ => {
            actions.push(Action::Request);
            if agent.food >= view.amount {
                actions.push(Action::Offer);
            }
        }
    }
    actions.sort();
    actions
}

/// Pure replaceable policy. Every input used is recorded in the decision.
pub fn utility_policy(agent: &Agent, view: &Observation) -> Decision {
    let trust = *agent.trust.get(&view.partner).unwrap_or(&0);
    let episodes: Vec<_> = agent
        .memories
        .iter()
        .filter(|m| m.partner == view.partner)
        .collect();
    let episodic_valence = episodes
        .iter()
        .rev()
        .take(4)
        .map(|m| m.valence)
        .sum::<i32>()
        .clamp(-40, 40);
    let candidates: Vec<_> = legal_actions(agent, view)
        .into_iter()
        .map(|action| {
            let score = match action {
                Action::Accept => agent.hunger + 20 + trust / 4 - agent.caution / 5,
                Action::Offer => {
                    agent.generosity - agent.hunger / 2 - agent.caution / 2
                        + trust
                        + episodic_valence
                }
                Action::Request => agent.hunger - 35 + trust / 4 - agent.caution / 4,
                Action::Refuse => {
                    agent.hunger / 2 + agent.caution / 2 - agent.generosity / 2 - trust / 2
                }
                Action::Leave => 0,
            };
            Candidate { action, score }
        })
        .collect();
    // First candidate wins equal scores; enum ordering is the documented tie rule.
    let selected = candidates
        .iter()
        .fold(
            &candidates[0],
            |best, c| if c.score > best.score { c } else { best },
        )
        .action;
    Decision {
        actor: agent.id,
        observation: view.clone(),
        food: agent.food,
        hunger: agent.hunger,
        generosity: agent.generosity,
        caution: agent.caution,
        expectation: agent.expectation,
        trust,
        episodic_valence,
        memory_causes: episodes.iter().rev().take(4).map(|m| m.event).collect(),
        candidates,
        selected,
    }
}

pub fn interpret(
    agent: &Agent,
    partner: AgentId,
    event: u64,
    signal: Signal,
    transferred: u32,
    failed: bool,
) -> Memory {
    let is_actor = agent.id == signal.actor;
    let (interpretation, valence) = if failed {
        (Interpretation::FailedTransfer, 0)
    } else if transferred > 0 {
        if is_actor {
            (Interpretation::HelpReceived, 24 + agent.expectation / 10)
        } else {
            (Interpretation::HelpGiven, 8 - agent.hunger / 10)
        }
    } else {
        match signal.action {
            Action::Refuse if is_actor => (Interpretation::ProtectedOwnNeeds, 0),
            Action::Refuse => (
                Interpretation::Rejected,
                -(8 + agent.expectation / 4 + agent.hunger / 10),
            ),
            Action::Request => (Interpretation::RequestHeard, 0),
            Action::Offer => (Interpretation::OfferHeard, 0),
            _ => (Interpretation::Departure, 0),
        }
    };
    Memory {
        event,
        partner,
        observed: signal.action,
        interpretation,
        valence,
    }
}

pub fn remember(agent: &mut Agent, memory: Memory) {
    let trust = agent.trust.entry(memory.partner).or_default();
    *trust = (*trust + memory.valence).clamp(-100, 100);
    if agent.memories.len() == MEMORY_CAPACITY {
        agent.memories.pop_front();
    }
    agent.memories.push_back(memory);
}

/// Experiment 002 opportunity cost: a hungry donor retains their last meal.
/// Inventory abundance matters without exposing the partner's circumstances.
pub fn scarcity_policy(agent: &Agent, view: &Observation) -> Decision {
    let mut decision = utility_policy(agent, view);
    if agent.hunger >= 60 && agent.food <= view.amount {
        for candidate in &mut decision.candidates {
            if candidate.action == Action::Offer {
                candidate.score -= 60;
            }
        }
        decision.selected = decision
            .candidates
            .iter()
            .fold(&decision.candidates[0], |best, c| {
                if c.score > best.score { c } else { best }
            })
            .action;
    }
    decision
}
