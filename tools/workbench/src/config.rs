//! Typed, bounded interfaces to the unchanged023/024/corrected025 runners.
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
    ActivityValuation,
    MaterialPreparation,
}

impl Mode {
    pub fn label(self) -> &'static str {
        match self {
            Self::ContinuousWorld => "023 Continuous World",
            Self::ActivityValuation => "024 Activity valuation",
            Self::MaterialPreparation => "025 Corrected preparation — research",
        }
    }
    pub fn experiment(self) -> &'static str {
        match self {
            Self::ContinuousWorld => "023",
            Self::ActivityValuation => "024",
            Self::MaterialPreparation => "025",
        }
    }
    pub fn runner(self) -> &'static str {
        match self {
            Self::ContinuousWorld => "continuous-world",
            Self::ActivityValuation => "activity-valuation",
            Self::MaterialPreparation => "material-preparation-observer2",
        }
    }
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Valuation {
    #[default]
    Baseline,
    MealRelief,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreparationPreset {
    #[default]
    KnownRaw,
    UnobservedRaw,
    NoAccess,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Effort {
    #[default]
    Short,
    Medium,
    Long,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Preparation {
    pub preset: PreparationPreset,
    pub raw_initial: u32,
    pub effort: Effort,
}
impl Default for Preparation {
    fn default() -> Self {
        Self {
            preset: PreparationPreset::KnownRaw,
            raw_initial: 6,
            effort: Effort::Short,
        }
    }
}
impl Preparation {
    pub fn settings(&self) -> serde_json::Value {
        let (extraction_ticks, preparation_ticks) = match self.effort {
            Effort::Short => (2, 4),
            Effort::Medium => (4, 8),
            Effort::Long => (8, 24),
        };
        let raw = if self.preset == PreparationPreset::UnobservedRaw {
            None
        } else {
            Some(0)
        };
        serde_json::json!({"feature_enabled":true,"access_enabled": self.preset != PreparationPreset::NoAccess,"source_id":0,"raw_initial":self.raw_initial,"extraction_ticks":extraction_ticks,"preparation_ticks":preparation_ticks,"observation":{"period":8,"raw_offsets":vec![raw;4],"ready_offsets":vec![Some(1);4]}})
    }
}
fn legacy_version() -> u32 {
    1
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Configuration {
    #[serde(default = "legacy_version")]
    pub schema_version: u32,
    pub mode: Mode,
    pub scenario: String,
    pub seed: u64,
    pub ticks: u64,
    pub period: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valuation: Option<Valuation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preparation: Option<Preparation>,
}
impl Default for Configuration {
    fn default() -> Self {
        Self {
            schema_version: 1,
            mode: Mode::ContinuousWorld,
            scenario: "paired".into(),
            seed: 42,
            ticks: 192,
            period: 11,
            valuation: None,
            preparation: None,
        }
    }
}
impl Configuration {
    pub fn policy_label(&self) -> &'static str {
        match self.valuation {
            None => "Unchanged023",
            Some(Valuation::Baseline) => "Baseline",
            Some(Valuation::MealRelief) => "MealRelief",
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        if ![1, 2].contains(&self.schema_version) {
            return Err("Unsupported configuration schema version".into());
        }
        match self.mode {
            Mode::ContinuousWorld if self.valuation.is_none() && self.preparation.is_none() => (),
            Mode::ActivityValuation
                if self.schema_version == 2
                    && self.valuation.is_some()
                    && self.preparation.is_none() => {}
            Mode::MaterialPreparation
                if self.schema_version == 2
                    && self.valuation.is_some()
                    && self
                        .preparation
                        .as_ref()
                        .is_some_and(|p| p.raw_initial <= 16) => {}
            _ => {
                return Err(
                    "Mode requires its explicit version2 options; unrelated options are rejected"
                        .into(),
                );
            }
        }
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
    pub fn world(&self) -> serde_json::Value {
        serde_json::json!({"case":self.scenario,"seed":self.seed,"duration":4,"cadence":8,"horizon":self.ticks,"order":"Ascending","capacity":2,"period":self.period,"phase":"Epoch","mode":"Continuous","role":"demo"})
    }
    pub fn runner_config(&self) -> serde_json::Value {
        match self.mode {
            Mode::ContinuousWorld => self.world(),
            Mode::ActivityValuation => {
                serde_json::json!({"world":self.world(),"valuation":self.valuation,"family":"custom_demo","source023":null})
            }
            Mode::MaterialPreparation => {
                serde_json::json!({"world":self.world(),"valuation":self.valuation,"family":"workbench_demo","pair":null,"preparation":self.preparation.as_ref().map(Preparation::settings)})
            }
        }
    }
}
