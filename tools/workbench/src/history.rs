//! Bounded observer projections of authentic023/024/025 envelopes, not a scientific validator.
use crate::{
    Result,
    config::{Configuration, Mode},
    storage::no_links,
};
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{SeqAccess, Visitor},
};
use std::{
    fmt,
    fs::File,
    io::{BufReader, Read},
    marker::PhantomData,
    path::Path,
};

pub const MAX_HISTORY: u64 = 64 * 1024 * 1024;
const SAMPLE: usize = 32;

trait Counted {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
    fn completed(&self) -> bool {
        false
    }
    fn transfer(&self) -> u64 {
        0
    }
}
#[derive(Debug, Serialize)]
struct Series<T> {
    count: usize,
    completed: usize,
    transferred: u64,
    first: Vec<T>,
    last: Option<T>,
}
impl<T> Default for Series<T> {
    fn default() -> Self {
        Self {
            count: 0,
            completed: 0,
            transferred: 0,
            first: Vec::new(),
            last: None,
        }
    }
}
impl<'de, T: Deserialize<'de> + Counted + Clone> Deserialize<'de> for Series<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct V<T>(PhantomData<T>);
        impl<'de, T: Deserialize<'de> + Counted + Clone> Visitor<'de> for V<T> {
            type Value = Series<T>;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("bounded sequence")
            }
            fn visit_seq<A: SeqAccess<'de>>(
                self,
                mut a: A,
            ) -> std::result::Result<Self::Value, A::Error> {
                let mut out = Series {
                    count: 0,
                    completed: 0,
                    transferred: 0,
                    first: Vec::new(),
                    last: None,
                };
                while let Some(item) = a.next_element::<T>()? {
                    item.validate().map_err(serde::de::Error::custom)?;
                    out.count += 1;
                    if out.count > 65536 {
                        return Err(serde::de::Error::custom("record count exceeds65536"));
                    }
                    out.completed += usize::from(item.completed());
                    out.transferred = out
                        .transferred
                        .checked_add(item.transfer())
                        .ok_or_else(|| serde::de::Error::custom("transfer overflow"))?;
                    if out.first.len() < SAMPLE {
                        out.first.push(item.clone());
                    }
                    out.last = Some(item);
                }
                Ok(out)
            }
        }
        d.deserialize_seq(V(PhantomData))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Agent {
    pub id: u32,
    pub food: u64,
    pub hunger: u64,
    #[serde(default)]
    pub trust: std::collections::BTreeMap<u32, i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Candidate {
    pub action: String,
    pub eligible: bool,
    #[serde(default)]
    pub score: Option<i64>,
    #[serde(default)]
    pub base_score: Option<i64>,
    #[serde(default)]
    pub meal_relief: Option<i64>,
    #[serde(default)]
    pub work_effort: Option<i64>,
    #[serde(default)]
    pub final_score: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Choice {
    pub input: Input,
    pub candidates: Vec<Candidate>,
    pub selected: String,
    #[serde(default)]
    pub contact: serde_json::Value,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Input {
    pub legacy: Local,
    #[serde(default)]
    pub retained: serde_json::Value,
    #[serde(default)]
    pub raw_retained: serde_json::Value,
    #[serde(default)]
    pub duration: Option<u64>,
    #[serde(default)]
    pub inquiry_lane_free: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Local {
    pub own: Agent,
    pub now: u64,
    #[serde(default)]
    pub people: serde_json::Value,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
struct Boundary {
    tick: u64,
    choices: Vec<Choice>,
    meals: usize,
    events: usize,
    grants: usize,
}
impl Counted for Boundary {}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Commitment {
    pub id: u64,
    pub work: serde_json::Value,
    pub participants: Vec<u32>,
    pub start: u64,
    pub deadline: u64,
    #[serde(default)]
    pub source: Option<usize>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transition {
    pub tick: u64,
    pub commitment: Commitment,
    pub completed: bool,
}
impl Counted for Transition {
    fn completed(&self) -> bool {
        self.completed
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub id: u64,
    pub tick: u64,
    pub scene: u64,
    pub participants: Vec<u32>,
    pub decision: ResourceDecision,
    pub balances_before: Vec<u64>,
    pub balances_after: Vec<u64>,
    pub transferred: u64,
    pub outcome: Option<String>,
    pub interpretations: Vec<serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceDecision {
    pub actor: u32,
    pub selected: String,
}
impl Counted for Event {
    fn completed(&self) -> bool {
        self.outcome.is_some()
    }
    fn transfer(&self) -> u64 {
        self.transferred
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
struct Grant {
    outcome: String,
}
impl Counted for Grant {
    fn completed(&self) -> bool {
        self.outcome == "Granted"
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Meal {
    pub tick: u64,
    pub actor: u32,
    pub food_before: u64,
    pub food_after: u64,
    pub hunger_before: u64,
    pub hunger_after: u64,
    pub consumed: u64,
}
impl Counted for Meal {}
#[derive(Deserialize)]
struct Layer<T> {
    base: T,
}
#[derive(Deserialize)]
struct Core {
    events: Series<Event>,
    cognition: Cognition,
}
#[derive(Deserialize)]
struct Cognition {
    consumption: Series<Meal>,
}
#[derive(Deserialize)]
struct Physical {
    #[serde(deserialize_with = "physical_version")]
    schema: String,
    start: u64,
    end: u64,
    initial: Vec<Agent>,
    final_agents: Vec<Agent>,
    initial_total: u64,
    final_stock: u64,
    transitions: Series<Transition>,
    boundaries: Series<Boundary>,
    grants: Series<Grant>,
    #[serde(default)]
    admissions: Series<Admission>,
    final_state: Layer<Layer<Layer<Layer<Layer<Core>>>>>,
}
#[derive(Deserialize)]
struct Checkpoint {
    tick: u64,
    labels: Vec<String>,
    active: Vec<u64>,
    pending: Vec<serde::de::IgnoredAny>,
}
#[derive(Deserialize)]
struct Envelope {
    schema: String,
    config: serde_json::Value,
    history: Physical,
    checkpoints: Vec<Checkpoint>,
    #[serde(default)]
    valuations: Series<ValuationRecord>,
    #[serde(default)]
    material: Option<Material>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admission {
    pub tick: u64,
    pub source: usize,
    pub invitation: serde_json::Value,
    pub reason: String,
    pub commitment: Option<u64>,
}
impl Counted for Admission {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selection {
    pub terms: Vec<Candidate>,
    pub selected: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationRecord {
    pub boundary: usize,
    pub choice: usize,
    pub policy: String,
    pub selection: Selection,
}
impl Counted for ValuationRecord {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub id: u64,
    pub tick: u64,
    pub commitment: u64,
    pub owner: u32,
    pub outcome: String,
    pub raw_before: u64,
    pub raw_after: u64,
    pub escrow_before: u64,
    pub escrow_after: u64,
    pub food_before: u64,
    pub food_after: u64,
    pub waste_before: u64,
    pub waste_after: u64,
}
impl Counted for Operation {
    fn completed(&self) -> bool {
        self.outcome == "Prepared"
    }
    fn transfer(&self) -> u64 {
        u64::from(self.outcome == "Failed")
    }
    fn validate(&self) -> Result<()> {
        let same_raw = self.raw_before == self.raw_after;
        let same_food = self.food_before == self.food_after;
        let same_waste = self.waste_before == self.waste_after;
        let valid = match self.outcome.as_str() {
            "Extracted" => {
                self.raw_before.checked_sub(2) == Some(self.raw_after)
                    && self.escrow_before == 0
                    && self.escrow_after == 2
                    && same_food
                    && same_waste
            }
            "Insufficient" => {
                self.raw_before < 2
                    && same_raw
                    && self.escrow_before == 0
                    && self.escrow_after == 0
                    && same_food
                    && same_waste
            }
            "Prepared" => {
                same_raw
                    && self.escrow_before == 2
                    && self.escrow_after == 0
                    && self.food_before.checked_add(1) == Some(self.food_after)
                    && self.waste_before.checked_add(1) == Some(self.waste_after)
            }
            "Failed" => {
                same_raw
                    && self.escrow_before == 0
                    && self.escrow_after == 0
                    && same_food
                    && same_waste
            }
            _ => false,
        };
        if valid {
            Ok(())
        } else {
            Err("Invalid025material operation accounting".into())
        }
    }
}
#[derive(Debug, Deserialize)]
struct Material {
    initial_raw: u64,
    operations: Series<Operation>,
    final_raw: u64,
    final_waste: u64,
    final_active: Vec<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialSummary {
    pub initial_raw: u64,
    pub final_raw: u64,
    pub final_waste: u64,
    pub final_escrow: u64,
    pub prepared: usize,
    pub failed: usize,
    pub operations: Vec<Operation>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progression {
    pub boundary: usize,
    pub choice_index: usize,
    pub decision_index: usize,
    pub valuation: Option<ValuationRecord>,
    pub started: Vec<Transition>,
    pub completed: Vec<Transition>,
    pub admissions: Vec<Admission>,
}
#[derive(Deserialize)]
struct VersionHeader {
    schema: String,
}
fn physical_version<'de, D: Deserializer<'de>>(d: D) -> std::result::Result<String, D::Error> {
    let version = String::deserialize(d)?;
    if ![
        "experiment-023-physical-history/v1",
        "experiment-024-physical-history/v1",
        "experiment-025-physical-history/v1",
    ]
    .contains(&version.as_str())
    {
        return Err(serde::de::Error::custom(
            "Unsupported physical history version",
        ));
    }
    Ok(version)
}
fn parse_error(e: serde_json::Error) -> String {
    if e.to_string()
        .contains("Unsupported physical history version")
    {
        return "Unsupported physical history version".into();
    }
    if e.is_eof() {
        format!("Incomplete history: {e}")
    } else {
        format!("Corrupt history: {e}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub experiment: String,
    pub perspective: String,
    pub start: u64,
    pub nominal_stop: u64,
    pub end: u64,
    pub completed_activities: usize,
    pub meals: usize,
    pub resource_events: usize,
    pub completed_resource_encounters: usize,
    pub transferred_units: u64,
    pub environmental_grants: usize,
    pub final_stock: u64,
    pub final_agents: Vec<Agent>,
    pub initial_agents: Vec<Agent>,
    pub decisions: Vec<Choice>,
    pub transitions: Vec<Transition>,
    pub initial_total: u64,
    pub food_accounting_matches: bool,
    pub sampled_decisions: bool,
    pub progressions: Vec<Progression>,
    pub material: Option<MaterialSummary>,
    pub resource_records: Vec<Event>,
    pub meal_records: Vec<Meal>,
}

pub fn read(path: &Path, expected: &Configuration) -> Result<Summary> {
    no_links(path)?;
    let file = File::open(path).map_err(|e| format!("Missing/unreadable history: {e}"))?;
    if file.metadata().map_err(|e| e.to_string())?.len() > MAX_HISTORY {
        return Err("History exceeds64MiB reader bound".into());
    }
    // Deliberately skip non-header values without retaining them before interpreting
    // the body. A valid unknown envelope is unsupported, not presumed corrupt.
    let header: VersionHeader =
        serde_json::from_reader(BufReader::new(file.take(MAX_HISTORY + 1))).map_err(parse_error)?;
    let exp = expected.mode.experiment();
    if header.schema != format!("experiment-{exp}-trial/v1") {
        return Err("Unsupported history version".into());
    }
    let file = File::open(path).map_err(|e| format!("Missing/unreadable history: {e}"))?;
    let mut de = serde_json::Deserializer::from_reader(BufReader::new(file.take(MAX_HISTORY + 1)));
    let e = Envelope::deserialize(&mut de).map_err(parse_error)?;
    de.end()
        .map_err(|e| format!("Extra/corrupt history data: {e}"))?;
    if e.schema != format!("experiment-{exp}-trial/v1")
        || e.history.schema != format!("experiment-{exp}-physical-history/v1")
    {
        return Err("Unsupported history version".into());
    }
    let c = if expected.mode == Mode::ContinuousWorld {
        &e.config
    } else {
        &e.config["world"]
    };
    if expected.mode != Mode::ContinuousWorld && e.config != expected.runner_config() {
        return Err("History configuration does not match the declared mode options".into());
    }
    if c["case"] != expected.scenario
        || c["seed"] != expected.seed
        || c["horizon"] != expected.ticks
        || c["period"] != expected.period
        || c["mode"] != "Continuous"
        || c["phase"] != "Epoch"
        || c["order"] != "Ascending"
        || c["duration"] != 4
        || c["cadence"] != 8
        || c["capacity"] != 2
        || c["role"] != "demo"
    {
        return Err("History configuration does not match the declared run".into());
    }
    let h = e.history;
    let nominal_stop = h.start.checked_add(expected.ticks).ok_or("Tick overflow")?;
    let end = nominal_stop.checked_add(33).ok_or("Tick overflow")?;
    if h.end != end
        || !e.checkpoints.last().is_some_and(|p| {
            p.tick == end
                && p.labels.iter().any(|s| s == "Final")
                && p.active.is_empty()
                && p.pending.is_empty()
        })
        || h.transitions.completed * 2 != h.transitions.count
    {
        return Err("Incomplete history: endpoint, drain or commitments unfinished".into());
    }
    for agents in [&h.initial, &h.final_agents] {
        if agents.len() != 4
            || agents
                .iter()
                .map(|a| a.id)
                .collect::<std::collections::BTreeSet<_>>()
                != [0, 1, 2, 3].into()
            || agents
                .iter()
                .any(|a| a.hunger > 100 || a.food > u32::MAX as u64)
        {
            return Err("Invalid four-person state".into());
        }
    }
    let core = h.final_state.base.base.base.base.base;
    let meals = core.cognition.consumption.count;
    h.boundaries.last.as_ref().ok_or("Missing boundaries")?;
    let material = e
        .material
        .map(|m| -> Result<MaterialSummary> {
            if !m.final_active.is_empty() {
                return Err("Incomplete material processes after drain".into());
            }
            Ok(MaterialSummary {
                initial_raw: m.initial_raw,
                final_raw: m.final_raw,
                final_waste: m.final_waste,
                final_escrow: 0,
                prepared: m.operations.completed,
                failed: m.operations.transferred as usize,
                operations: m.operations.first,
            })
        })
        .transpose()?;
    if (expected.mode == Mode::MaterialPreparation) != material.is_some() {
        return Err("History material schema does not match mode".into());
    }
    let total = h
        .final_agents
        .iter()
        .try_fold(h.final_stock, |sum, a| sum.checked_add(a.food))
        .and_then(|sum| sum.checked_add(meals as u64))
        .and_then(|sum| {
            material.as_ref().map_or(Some(sum), |m| {
                sum.checked_add(m.final_raw)?
                    .checked_add(m.final_waste)?
                    .checked_add(m.final_escrow)
            })
        });
    let mut decisions = Vec::new();
    let mut progressions = Vec::new();
    for (boundary, b) in h.boundaries.first.iter().enumerate() {
        if b.choices.len() > 4 {
            return Err("Too many choices in four-person boundary".into());
        }
        for (choice_index, choice) in b.choices.iter().enumerate() {
            if decisions.len() < SAMPLE {
                let valuation = e
                    .valuations
                    .first
                    .iter()
                    .find(|v| v.boundary == boundary && v.choice == choice_index)
                    .cloned();
                if expected.mode != Mode::ContinuousWorld {
                    let v = valuation
                        .as_ref()
                        .ok_or("Missing recorded valuation for native decision sample")?;
                    let terms = &v.selection.terms;
                    let actions: &[&str] = if expected.mode == Mode::MaterialPreparation {
                        &["Wait", "Consume", "Invite", "Acquire", "Prepare"]
                    } else {
                        &["Wait", "Consume", "Invite", "Acquire"]
                    };
                    if v.policy != expected.policy_label()
                        || v.selection.selected != choice.selected
                        || terms.len() != actions.len()
                        || terms.iter().zip(actions).any(|(t, a)| {
                            t.action != *a
                                || t.base_score.is_none()
                                || t.meal_relief.is_none()
                                || t.final_score.is_none()
                                || (expected.mode == Mode::MaterialPreparation
                                    && t.work_effort.is_none())
                        })
                    {
                        return Err(
                            "Recorded valuation schema or references do not match mode/choice"
                                .into(),
                        );
                    }
                }
                let decision_index = decisions.len();
                decisions.push(choice.clone());
                let started: Vec<_> = h
                    .transitions
                    .first
                    .iter()
                    .filter(|t| {
                        !t.completed
                            && t.commitment.source == Some(boundary)
                            && t.commitment.participants.first()
                                == Some(&choice.input.legacy.own.id)
                    })
                    .cloned()
                    .collect();
                let completed = h
                    .transitions
                    .first
                    .iter()
                    .filter(|t| {
                        t.completed && started.iter().any(|s| s.commitment.id == t.commitment.id)
                    })
                    .cloned()
                    .collect();
                let admissions = h
                    .admissions
                    .first
                    .iter()
                    .filter(|a| {
                        a.source == boundary
                            && a.invitation["proposer"] == choice.input.legacy.own.id
                    })
                    .cloned()
                    .collect();
                progressions.push(Progression {
                    boundary,
                    choice_index,
                    decision_index,
                    valuation,
                    started,
                    completed,
                    admissions,
                });
            }
        }
    }
    Ok(Summary {
        experiment: exp.into(),
        perspective: "Observer Perspective — recorded outcomes, not actor knowledge".into(),
        start: h.start,
        nominal_stop,
        end,
        completed_activities: h.transitions.completed,
        meals,
        resource_events: core.events.count,
        completed_resource_encounters: core.events.completed,
        transferred_units: core.events.transferred,
        environmental_grants: h.grants.completed,
        final_stock: h.final_stock,
        final_agents: h.final_agents,
        initial_agents: h.initial,
        decisions,
        transitions: h.transitions.first,
        initial_total: h.initial_total,
        food_accounting_matches: total == Some(h.initial_total),
        sampled_decisions: h.boundaries.count > SAMPLE,
        progressions,
        material,
        resource_records: core.events.first,
        meal_records: core.cognition.consumption.first,
    })
}
