//! The small, verified subset of the unchanged 023 CLI exposed to humans.
use serde::{Deserialize, Serialize};

pub const SCENARIOS: [&str; 12] = [
    "paired",
    "acquisition",
    "busy_recipient",
    "consumption",
    "durations",
    "exhaustion",
    "inquiry_conflict",
    "inquiry_quiet",
    "mixed",
    "null",
    "ongoing",
    "scarcity",
];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    ContinuousWorld,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Configuration {
    pub mode: Mode,
    pub scenario: String,
    pub seed: u64,
    pub ticks: u64,
    pub period: u64,
}
impl Default for Configuration {
    fn default() -> Self {
        Self {
            mode: Mode::ContinuousWorld,
            scenario: "paired".into(),
            seed: 42,
            ticks: 192,
            period: 11,
        }
    }
}
impl Configuration {
    pub fn validate(&self) -> Result<(), String> {
        if !SCENARIOS.contains(&self.scenario.as_str()) {
            return Err("Unknown 023 scenario".into());
        }
        if !(1..=1024).contains(&self.ticks) {
            return Err("Ordinary duration must be 1–1024 ticks".into());
        }
        if !(1..=128).contains(&self.period) {
            return Err("Reporting period must be 1–128 ticks".into());
        }
        Ok(())
    }
}
