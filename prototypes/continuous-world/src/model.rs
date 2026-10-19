//! Isolated023 research configuration and observer checkpoint references.
use crate::{encounters::Order, experiment::concurrent as x, model::Agent};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
pub const DRAIN: u64 = 33;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Windowed,
    Continuous,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Epoch,
    Absolute,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub case: String,
    pub seed: u64,
    pub duration: u64,
    pub cadence: u64,
    pub horizon: u64,
    pub order: Order,
    pub capacity: usize,
    pub period: u64,
    pub phase: Phase,
    pub mode: Mode,
    pub role: String,
}
impl Config {
    pub fn validate(&self) -> Result<(), String> {
        if !(1..=1024).contains(&self.horizon)
            || !(1..=128).contains(&self.period)
            || ![2, 4, 8].contains(&self.duration)
            || self.cadence != 8
            || self.capacity != 2
            || !matrix().iter().any(|c| c.case == self.case)
        {
            return Err("unsupported bounded023configuration".into());
        }
        assert_eq!(
            u64::from(crate::inquiry::Strategy::Evidence.cost()) + 5,
            DRAIN
        );
        Ok(())
    }
    pub fn public_end(&self, now: u64, start: u64) -> u64 {
        match self.mode {
            Mode::Continuous => now + DRAIN,
            Mode::Windowed => start + ((now - start) / self.period + 1) * self.period,
        }
    }
    pub fn physics(&self) -> x::Config {
        x::Config {
            case: self.case.clone(),
            seed: self.seed,
            duration: self.duration,
            cadence: self.cadence,
            horizon: self.horizon + DRAIN,
            order: self.order,
            capacity: self.capacity,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingCut {
    pub source: usize,
    pub proposer: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checkpoint {
    pub tick: u64,
    pub labels: Vec<String>,
    pub active: Vec<u64>,
    pub pending: Vec<PendingCut>,
    pub boundary: Option<usize>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub schema: String,
    pub config: Config,
    pub history: x::Trial,
    pub checkpoints: Vec<Checkpoint>,
}
pub fn matrix() -> Vec<Config> {
    serde_json::from_str(include_str!(
        "../../../experiments/023/registered-matrix.json"
    ))
    .unwrap()
}
pub fn trial(config: Config) -> Trial {
    crate::simulation::Simulation::continuous_trial23(config)
}
pub fn checkpoints(history: &x::Trial, config: &Config) -> Vec<Checkpoint> {
    let mut labels: BTreeMap<u64, Vec<String>> = BTreeMap::new();
    let mut tick = history.start + config.period;
    while tick <= history.end {
        labels.entry(tick).or_default().push("Reporting".into());
        tick += config.period;
    }
    labels
        .entry(history.start + config.horizon)
        .or_default()
        .push("NominalStop".into());
    labels.entry(history.end).or_default().push("Final".into());
    let starts: BTreeMap<_, _> = history
        .transitions
        .iter()
        .filter(|t| !t.completed)
        .map(|t| (t.commitment.id, &t.commitment))
        .collect();
    let ends: BTreeMap<_, _> = history
        .transitions
        .iter()
        .filter(|t| t.completed)
        .map(|t| (t.commitment.id, t.tick))
        .collect();
    labels
        .into_iter()
        .map(|(tick, labels)| {
            let active = starts
                .iter()
                .filter(|(id, c)| c.start <= tick && tick < ends[id])
                .map(|(id, _)| *id)
                .collect();
            let mut pending = vec![];
            for (source, boundary) in history.boundaries.iter().enumerate() {
                if boundary.tick <= tick && tick < boundary.tick + 2 {
                    for d in &boundary.choices {
                        if d.selected == crate::acquisition::Action::Invite {
                            pending.push(PendingCut {
                                source,
                                proposer: d.input.legacy.own.id,
                            });
                        }
                    }
                }
            }
            let boundary = history.boundaries.iter().position(|b| b.tick == tick);
            Checkpoint {
                tick,
                labels,
                active,
                pending,
                boundary,
            }
        })
        .collect()
}
/// Physical final actors are captured by the unchanged underlying snapshot.
pub fn final_actors(trial: &Trial) -> &[Agent] {
    &trial.history.final_agents
}
