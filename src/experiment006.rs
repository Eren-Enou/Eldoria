//! Controlled opportunities and interventions; actions are selected by local policies.
use crate::{
    behavior::scarcity_policy,
    concerns::{Concern, Status},
    experiment005,
    foresight::{Channel, Sensor},
    inquiry::{self, Action, Inquiry},
    intentional::Profile,
    model::Generator,
    simulation::Simulation,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub base: experiment005::Snapshot,
    pub inquiry: Inquiry,
}
pub fn snapshot(sim: &Simulation, label: &str) -> Snapshot {
    Snapshot {
        base: experiment005::snapshot(sim, label),
        inquiry: sim.inquiry().unwrap(),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checkpoint {
    pub label: String,
    pub first_record: usize,
    pub end_record: usize,
    pub concerns: Vec<Concern>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub name: String,
    pub initial: Snapshot,
    pub interventions: Vec<String>,
    pub checkpoints: Vec<Checkpoint>,
    pub final_state: Snapshot,
    pub history_ablated: Vec<inquiry::Decision>,
}

pub fn setup(
    seed: u64,
    importance_goal: i32,
    privacy: i32,
    settings: inquiry::Settings,
) -> Simulation {
    let mut generator = Generator::new(seed);
    let mut sim = Simulation::new((0..3).map(|id| generator.agent(id)).collect()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(
        seed,
        Sensor {
            reliability: 40,
            channel: Channel::AccurateFixture,
            ..Default::default()
        },
    )
    .unwrap();
    sim.enable_concerns().unwrap();
    experiment005::refusal_conditions(&mut sim, importance_goal);
    sim.add_scene([0, 1], 1, 6).unwrap();
    sim.run();
    sim.enable_inquiry().unwrap();
    sim.set_inquiry_settings(0, settings).unwrap();
    // Same current resources for every inquiry; no further resource refusals.
    sim.set_circumstances(0, 4, 30, 0, 80).unwrap();
    sim.set_circumstances(1, 1, 30, 0, 90).unwrap();
    sim.set_communication_profile(
        1,
        Profile {
            privacy,
            ..Default::default()
        },
    )
    .unwrap();
    sim
}
fn opportunity(
    sim: &mut Simulation,
    participants: &[u32],
    label: &str,
    checkpoints: &mut Vec<Checkpoint>,
) {
    let first_record = sim.inquiry().unwrap().records.len();
    sim.inquiry_meeting(participants).unwrap();
    checkpoints.push(Checkpoint {
        label: label.into(),
        first_record,
        end_record: sim.inquiry().unwrap().records.len(),
        concerns: sim
            .concerns()
            .unwrap()
            .items
            .get(&0)
            .cloned()
            .unwrap_or_default(),
    });
}
pub fn trial(seed: u64, name: &str) -> Trial {
    let settings = inquiry::Settings {
        use_history: name != "history_disabled",
        track_novelty: name != "novelty_disabled",
    };
    let mut sim = setup(
        seed,
        if name == "important_uninformative" {
            80
        } else {
            60
        },
        if name == "important_uninformative" {
            100
        } else {
            30
        },
        settings,
    );
    let initial = snapshot(
        &sim,
        "original unresolved refusal; present traits matched; inquiry opt-in",
    );
    let mut interventions = vec!["Original private scarcity: requester food0/hunger90, refuser food1/hunger90/privacy100. Current requester food4/hunger30, refuser food1/hunger30; source privacy30 (100 for silent case). Meeting supply does not force messages.".into()];
    let rounds = match name {
        "repeated_weak" => 2,
        "strategy_change" => 3,
        "useful_repetition" => 5,
        _ => 7,
    };
    let mut checkpoints = Vec::new();
    for _ in 0..rounds {
        opportunity(
            &mut sim,
            &[0, 1],
            "later public opportunity",
            &mut checkpoints,
        );
    }
    if name == "alternative_source" {
        opportunity(
            &mut sim,
            &[0, 1, 2],
            "new individual introduced through recorded co-presence; expertise is unknown",
            &mut checkpoints,
        );
    }
    if name == "renewed_evidence" || name == "useful_repetition" {
        let event = sim.concerns().unwrap().items[&0][0].event;
        sim.set_inquiry_capability(
            1,
            event,
            1,
            Sensor {
                reliability: 80,
                channel: Channel::AccurateFixture,
                ..Default::default()
            },
        )
        .unwrap();
        interventions.push("Source now has channel1 evidence at quality80 (old channel0 quality40). The next meeting permits voluntary announcement; no observation is supplied to the inquirer until the source chooses to disclose.".into());
        opportunity(
            &mut sim,
            &[0, 1],
            "new locally advertised evidence opportunity",
            &mut checkpoints,
        );
    }
    let final_state = snapshot(&sim, "final audited state");
    let history_ablated = final_state
        .inquiry
        .records
        .iter()
        .filter(|r| r.decision.input.owner == 0)
        .map(|r| {
            let mut input = r.decision.input.clone();
            input.cells.clear();
            inquiry::policy(&input)
        })
        .collect();
    Trial {
        seed,
        name: name.into(),
        initial,
        interventions,
        checkpoints,
        final_state,
        history_ablated,
    }
}
pub fn suite(seed: u64) -> Vec<Trial> {
    [
        "repeated_weak",
        "strategy_change",
        "renewed_evidence",
        "alternative_source",
        "important_uninformative",
        "useful_repetition",
        "history_disabled",
        "novelty_disabled",
    ]
    .into_iter()
    .map(|name| trial(seed, name))
    .collect()
}
pub fn population(seed: u64, count: u32) -> Snapshot {
    let mut generator = Generator::new(seed);
    let mut sim = Simulation::new((0..count).map(|id| generator.agent(id)).collect()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(seed, Sensor::default()).unwrap();
    sim.enable_concerns().unwrap();
    sim.enable_inquiry().unwrap();
    for _ in 0..3 {
        for id in (0..count.saturating_sub(1)).step_by(2) {
            sim.add_scene([id, id + 1], 1, 6).unwrap();
        }
        sim.run();
        for id in 0..count {
            sim.consume(id).unwrap();
        }
    }
    snapshot(
        &sim,
        "three natural encounters per pair; consumption after each, no replenishment",
    )
}
pub fn metrics(snapshot: &Snapshot) -> serde_json::Value {
    let inquiry = &snapshot.inquiry;
    let asks: Vec<_> = inquiry
        .records
        .iter()
        .filter(|r| r.valid && matches!(r.decision.selected, Action::Ask { .. }))
        .collect();
    let mut last = std::collections::BTreeMap::new();
    let mut repeated = 0;
    let mut repeated_inquiries = 0;
    let mut strategies = 0;
    let mut sources = 0;
    let mut pursued = std::collections::BTreeSet::new();
    for record in &asks {
        if record.cell_before.as_ref().is_some_and(|c| c.attempts > 0) {
            repeated_inquiries += 1;
        }
        if let Action::Ask {
            concern,
            source,
            strategy,
        } = record.decision.selected
        {
            pursued.insert(concern);
            if let Some((old_source, old_strategy)) = last.insert(concern, (source, strategy)) {
                if old_source == source && old_strategy == strategy {
                    repeated += 1;
                }
                if old_strategy != strategy {
                    strategies += 1;
                }
                if old_source != source {
                    sources += 1;
                }
            }
        }
    }
    let paused: Vec<_> = inquiry
        .records
        .iter()
        .filter(|r| {
            r.valid
                && r.decision.selected == Action::Pause
                && r.decision
                    .candidates
                    .iter()
                    .any(|c| matches!(c.action, Action::Ask { .. }))
                && r.decision
                    .candidates
                    .iter()
                    .filter(|c| matches!(c.action, Action::Ask { .. }))
                    .all(|c| c.score <= 0)
        })
        .collect();
    let unresolved: std::collections::BTreeSet<_> = paused
        .iter()
        .flat_map(|r| {
            r.decision
                .input
                .concerns
                .iter()
                .filter(|c| c.status.active())
                .map(|c| c.id)
        })
        .collect();
    serde_json::json!({"concerns_pursued":pursued.len(),"inquiries":asks.len(),"repeated_inquiries":repeated_inquiries,"same_source_strategy_repetitions":repeated,
        "strategy_changes":strategies,"source_changes":sources,"low_value_pauses":paused.len(),"unresolved_concerns_at_pause":unresolved.len(),
        "active_final_concerns":snapshot.base.concerns.items.values().flatten().filter(|c|c.status.active()).count(),
        "resolved_final_concerns":snapshot.base.concerns.items.values().flatten().filter(|c|c.status==Status::Resolved).count(),
        "cells":inquiry.cells.values().map(|v|v.len()).sum::<usize>(),"signatures":inquiry.seen.values().map(|v|v.len()).sum::<usize>(),
        "episodes":inquiry.episodes.values().map(|v|v.len()).sum::<usize>(),"decision_records":inquiry.records.len(),
        "resource_events":snapshot.base.events.len(),"talk_turns":snapshot.base.intentional.records.len(),"receipts":snapshot.base.cognition.information.len(),
        "serialized_bytes":serde_json::to_vec(snapshot).unwrap().len()})
}
pub fn human(trials: &[Trial]) -> String {
    use std::fmt::Write;
    let mut text = String::from("Experiment 006: adaptive local inquiry\n");
    for trial in trials {
        writeln!(text, "\n{} seed {}", trial.name, trial.seed).unwrap();
        for intervention in &trial.interventions {
            writeln!(text, "  Intervention: {intervention}").unwrap();
        }
        for record in trial
            .final_state
            .inquiry
            .records
            .iter()
            .filter(|r| r.decision.input.owner == 0)
        {
            writeln!(text,"  Record {}: {:?}; importance {:?}; novelty {:?}, useful {}; expected {:?}->{:?}; concern {:?}",record.id,record.decision.selected,
                record.decision.input.concerns.iter().map(|c|c.importance).collect::<Vec<_>>(),record.novelty,record.realized_value,
                record.cell_before.as_ref().map(|c|c.expected),record.cell_after.as_ref().map(|c|c.expected),record.concern_after.as_ref().map(|c|c.status)).unwrap();
            for c in &record.decision.candidates {
                writeln!(text,"    {:?}: gain {}, benefit {}, cost {}, score {}; learned {}, opportunity {}; prior record {:?}, notice {:?}",c.action,c.expected_gain,c.benefit,c.cost,c.score,c.prior_or_learned,c.opportunity_value,c.history_record,c.notice).unwrap();
            }
            if let Some(response) = &record.response {
                writeln!(
                    text,
                    "    Public response {:?}, receipts {:?}, novelty per receipt {:?}",
                    response.public_action, response.receipts, record.information_values
                )
                .unwrap();
            }
        }
        writeln!(text, "  Metrics {}", metrics(&trial.final_state)).unwrap();
    }
    text
}
