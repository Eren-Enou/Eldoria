//! Experiment024 actor-local ordinary valuation; no new persistent cognition.
use crate::{
    acquisition as a, continuous as c, environmental_knowledge as k, experiment::concurrent as x,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Baseline,
    MealRelief,
}
pub type Selector = fn(&k::Input, &[a::Candidate]) -> Selection;
impl Mode {
    pub fn selector(self) -> Selector {
        match self {
            Self::Baseline => rank::<false>,
            Self::MealRelief => rank::<true>,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub world: c::Config,
    pub valuation: Mode,
    /// Observer/fixture metadata, never passed to the selector.
    pub family: String,
    pub source023: Option<usize>,
}
impl Config {
    pub fn validate(&self) -> Result<(), String> {
        self.world.validate()?;
        if self.world.mode != c::Mode::Continuous {
            return Err("024 requires continuous opportunities".into());
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Term {
    pub action: a::Action,
    pub eligible: bool,
    pub base_score: i32,
    pub meal_relief: i32,
    pub final_score: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Selection {
    pub terms: Vec<Term>,
    pub selected: a::Action,
}
fn rank<const RELIEF: bool>(input: &k::Input, candidates: &[a::Candidate]) -> Selection {
    assert_eq!(
        candidates.iter().map(|c| c.action).collect::<Vec<_>>(),
        [
            a::Action::Wait,
            a::Action::Consume,
            a::Action::Invite,
            a::Action::Acquire
        ]
    );
    assert!((0..=100).contains(&input.legacy.own.hunger));
    let terms: Vec<_> = candidates
        .iter()
        .map(|candidate| {
            let meal_relief =
                if RELIEF && candidate.action == a::Action::Consume && candidate.eligible {
                    assert_eq!(candidate.score, input.legacy.own.hunger);
                    input.legacy.own.hunger.min(40)
                } else {
                    0
                };
            Term {
                action: candidate.action,
                eligible: candidate.eligible,
                base_score: candidate.score,
                meal_relief,
                final_score: candidate.score + meal_relief,
            }
        })
        .collect();
    let selected = terms
        .iter()
        .fold(&terms[0], |best, c| {
            if c.final_score > best.final_score {
                c
            } else {
                best
            }
        })
        .action;
    Selection { terms, selected }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub boundary: usize,
    pub choice: usize,
    pub policy: Mode,
    pub selection: Selection,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub schema: String,
    pub config: Config,
    pub history: x::Trial,
    pub checkpoints: Vec<c::Checkpoint>,
    /// Immutable observer audit references; not an ECS/actor resource.
    pub valuations: Vec<Record>,
}
pub fn matrix() -> Vec<Config> {
    serde_json::from_str(include_str!(
        "../../../experiments/024/registered-matrix.json"
    ))
    .unwrap()
}
pub fn trial(config: Config) -> Trial {
    crate::simulation::Simulation::valuation_trial24(config)
}
