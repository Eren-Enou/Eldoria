//! Shallow predictions and fallible sensors, isolated from partner-private state.
use crate::intentional::{self, TalkAction, TalkDecision, TalkInput};
use crate::model::AgentId;
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
pub const RULES: &str = "experiment-004-v1";
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Channel {
    Seeded,
    AccurateFixture,
    InvertedFixture,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sensor {
    pub reliability: i32,
    pub effort: u32,
    pub channel: Channel,
    pub second: Option<Channel>,
}
impl Default for Sensor {
    fn default() -> Self {
        Self {
            reliability: 80,
            effort: 0,
            channel: Channel::Seeded,
            second: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub anticipate: bool,
    pub learn: bool,
    pub challenge_prior: i32,
    pub answer_prior: i32,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            anticipate: true,
            learn: true,
            challenge_prior: 50,
            answer_prior: 50,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expectation {
    pub challenge: i32,
    pub answer: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Context {
    pub settings: Settings,
    pub expectation: Expectation,
    /// Public description only: no sensor channel, seed, truth, or sampled result.
    pub reliability: i32,
    pub effort: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForecastCandidate {
    pub action: TalkAction,
    pub immediate: i32,
    pub expected_response: Option<TalkAction>,
    pub probability: i32,
    pub consequence: String,
    pub anticipated: i32,
    pub combined: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plan {
    pub decision: TalkDecision,
    pub context: Context,
    pub candidates: Vec<ForecastCandidate>,
}
pub fn predict(input: &TalkInput, context: &Context) -> Plan {
    use TalkAction::*;
    let mut decision = intentional::policy(input);
    let contradicted_own_claim = input.own_historical_scarcity == Some(false)
        && input.memories.iter().any(|m| {
            m.event == input.event
                && m.actor == input.own.id
                && m.claim == Some(true)
                && m.action == Mislead
        });
    let candidates: Vec<_> = decision
        .candidates
        .iter_mut()
        .map(|c| {
            let immediate = c.score
                - if c.action == ProvideEvidence {
                    context.effort as i32
                } else {
                    0
                };
            let (expected_response, probability, consequence, anticipated) = match c.action {
                Mislead => (
                    Some(AskEvidence),
                    context.expectation.challenge,
                    "Unsupported lie may be challenged",
                    -40 * context.expectation.challenge / 100,
                ),
                Explain => (
                    Some(AskEvidence),
                    context.expectation.challenge,
                    "Explanation may require costly substantiation",
                    -8 * context.expectation.challenge / 100,
                ),
                AskEvidence => (
                    Some(ProvideEvidence),
                    context.expectation.answer,
                    "Evidence response may reduce uncertainty",
                    context.expectation.answer * context.reliability / 100 / 4,
                ),
                ProvideEvidence if contradicted_own_claim => (
                    None,
                    context.reliability,
                    "Evidence may contradict own earlier claim",
                    -40 * context.reliability / 100,
                ),
                _ => (None, 0, "No modeled additional consequence", 0),
            };
            let (expected_response, probability, anticipated) =
                if matches!(c.action, Explain | Mislead) && input.turns_left < 3 {
                    (None, 0, 0)
                } else {
                    (expected_response, probability, anticipated)
                };
            let anticipated = if context.settings.anticipate {
                anticipated
            } else {
                0
            };
            c.score = immediate + anticipated;
            ForecastCandidate {
                action: c.action,
                immediate,
                expected_response,
                probability,
                consequence: consequence.into(),
                anticipated,
                combined: c.score,
            }
        })
        .collect();
    decision.selected = decision
        .candidates
        .iter()
        .fold(&decision.candidates[0], |best, c| {
            if c.score > best.score { c } else { best }
        })
        .action;
    Plan {
        decision,
        context: context.clone(),
        candidates,
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForecastRecord {
    pub talk_record: u64,
    pub context: Context,
    pub candidates: Vec<ForecastCandidate>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredictionError {
    pub forecast_record: u64,
    pub response_record: u64,
    pub actor: AgentId,
    pub partner: AgentId,
    pub predicted: TalkAction,
    pub actual: TalkAction,
    pub expected: i32,
    pub outcome: i32,
    pub error: i32,
    pub updated: i32,
    pub learned: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reading {
    pub talk_record: u64,
    pub receipt: u64,
    pub source: u8,
    pub scarce: bool,
    pub reliability: i32,
    /// Observer annotation; excluded from all prediction inputs.
    pub objectively_correct: bool,
}
#[derive(Resource, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Foresight {
    pub seed: u64,
    pub sensor: Sensor,
    pub settings: BTreeMap<AgentId, Settings>,
    pub expectations: BTreeMap<AgentId, BTreeMap<AgentId, Expectation>>,
    pub forecasts: Vec<ForecastRecord>,
    pub errors: Vec<PredictionError>,
    pub readings: Vec<Reading>,
}
impl Foresight {
    pub fn context(&self, actor: AgentId, partner: AgentId) -> Context {
        let settings = self.settings[&actor].clone();
        let expectation = self
            .expectations
            .get(&actor)
            .and_then(|m| m.get(&partner))
            .cloned()
            .unwrap_or(Expectation {
                challenge: settings.challenge_prior,
                answer: settings.answer_prior,
            });
        Context {
            settings,
            expectation,
            reliability: self.sensor.reliability,
            effort: self.sensor.effort,
        }
    }
}
#[derive(Resource)]
pub(crate) struct Predictor(pub fn(&TalkInput, &Context) -> Plan);
pub fn update_probability(expected: i32, outcome: i32) -> i32 {
    (expected + (outcome - expected) / 4).clamp(0, 100)
}
/// Sampling is keyed by event/source, so re-reading a source cannot resample noise.
pub fn reading(
    seed: u64,
    event: u64,
    speaker: AgentId,
    source: u8,
    truth: bool,
    sensor: &Sensor,
) -> bool {
    let channel = if source == 0 {
        sensor.channel
    } else {
        sensor.second.unwrap_or(sensor.channel)
    };
    let correct = match channel {
        Channel::AccurateFixture => true,
        Channel::InvertedFixture => false,
        Channel::Seeded => {
            let mut z = seed
                ^ event.wrapping_mul(0x9e3779b97f4a7c15)
                ^ u64::from(speaker).wrapping_mul(0xbf58476d1ce4e5b9)
                ^ u64::from(source).wrapping_mul(0x94d049bb133111eb);
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z ^= z >> 31;
            z % 100 < (50 + sensor.reliability / 2) as u64
        }
    };
    if correct { truth } else { !truth }
}
