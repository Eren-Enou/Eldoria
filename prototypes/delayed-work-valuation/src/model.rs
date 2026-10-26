//!026 opt-in pure valuation; no physical or persistent cognitive state.
use crate::{preparation as m, valuation as v};
use serde::{Deserialize, Serialize};
include!(concat!(env!("OUT_DIR"), "/source_stamp.rs"));

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Baseline,
    ProjectedNeed,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub source025: usize,
    pub mode: Mode,
    pub world: m::Config,
}
pub fn matrix() -> Vec<Config> {
    serde_json::from_str(include_str!(
        "../../../experiments/026/registered-matrix.json"
    ))
    .unwrap()
}
pub fn projected_need(hunger: i32, duration: i32) -> i32 {
    assert!((0..=100).contains(&hunger) && (1..=33).contains(&duration));
    hunger.checked_add(5 * (duration / 4)).unwrap().min(100)
}
/// Inputs are unchanged025 local cognition and its four legitimate prior terms.
pub fn rank(input: &m::Input, original: &[v::Term], mode: Mode) -> m::Selection {
    let mut result = m::rank(input, original);
    let prepare = &mut result.terms[4];
    if mode == Mode::ProjectedNeed && prepare.eligible {
        prepare.base_score = projected_need(input.inherited.legacy.own.hunger, prepare.work_effort);
        prepare.final_score = prepare.base_score - prepare.work_effort;
        result.selected = result
            .terms
            .iter()
            .fold(&result.terms[0], |best, next| {
                if next.final_score > best.final_score {
                    next
                } else {
                    best
                }
            })
            .action;
    }
    result
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Audit {
    pub boundary: usize,
    pub choice: usize,
    pub owner: u32,
    pub tick: u64,
    pub hunger: i32,
    pub duration: Option<i32>,
    pub intervals: Option<i32>,
    pub projected_need: Option<i32>,
    pub baseline_prepare: i32,
    pub final_prepare: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trial {
    pub schema: String,
    pub config: Config,
    pub body: m::Trial,
    pub projection_audit: Vec<Audit>,
}
pub fn trial(config: Config) -> Trial {
    assert!(config.source025 < 50);
    assert_eq!(config.world, m::matrix()[config.source025]);
    assert_eq!(config.world.valuation, v::Mode::Baseline);
    let mut body =
        crate::simulation::Simulation::delayed_trial26(config.world.clone(), config.mode);
    let mut projection_audit = vec![];
    for (boundary, b) in body.history.boundaries.iter().enumerate() {
        for (choice, d) in b.choices.iter().enumerate() {
            let own = &d.input.inherited.legacy.own;
            let term = &d.candidates[4];
            let duration = term.eligible.then_some(term.work_effort);
            projection_audit.push(Audit {
                boundary,
                choice,
                owner: own.id,
                tick: d.input.inherited.legacy.now,
                hunger: own.hunger,
                duration,
                intervals: duration.map(|d| d / 4),
                projected_need: duration.map(|d| projected_need(own.hunger, d)),
                baseline_prepare: duration.map_or(-1, |d| own.hunger - d),
                final_prepare: term.final_score,
            });
        }
    }
    body.schema = "experiment-026-body/v1".into();
    body.history.schema = "experiment-026-physical-history/v1".into();
    Trial {
        schema: "experiment-026-trial/v1".into(),
        config,
        body,
        projection_audit,
    }
}
pub fn baseline_projection(trial: &Trial) -> m::Trial {
    assert_eq!(trial.config.mode, Mode::Baseline);
    let mut body = trial.body.clone();
    body.schema = "experiment-025-trial/v1".into();
    body.history.schema = "experiment-025-physical-history/v1".into();
    body
}

/// Disabled physical feature follows the exact inherited024 path in both hosts.
pub fn feature_off(config: m::Config) -> m::Trial {
    assert_eq!(config.preparation, m::Settings::default());
    crate::simulation::Simulation::delayed_trial26(config, Mode::Baseline)
}
