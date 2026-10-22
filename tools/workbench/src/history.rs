//! Streaming observer projection of the actual 023 envelope. Not a scientific validator.
use crate::{Result, config::Configuration, storage::no_links};
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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Agent {
    pub id: u32,
    pub food: u64,
    pub hunger: u64,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Candidate {
    pub action: String,
    pub eligible: bool,
    pub score: i64,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Choice {
    pub input: Input,
    pub candidates: Vec<Candidate>,
    pub selected: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Input {
    pub legacy: Local,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Local {
    pub own: Agent,
    pub now: u64,
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
struct Event {
    transferred: u64,
}
impl Counted for Event {
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
struct Meal {/* Only count, skip content. */}
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
}
#[derive(Deserialize)]
struct VersionHeader {
    schema: String,
}
fn physical_version<'de, D: Deserializer<'de>>(d: D) -> std::result::Result<String, D::Error> {
    let version = String::deserialize(d)?;
    if version != "experiment-023-physical-history/v1" {
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
    pub perspective: String,
    pub start: u64,
    pub nominal_stop: u64,
    pub end: u64,
    pub completed_activities: usize,
    pub meals: usize,
    pub resource_events: usize,
    pub transferred_units: u64,
    pub environmental_grants: usize,
    pub final_stock: u64,
    pub final_agents: Vec<Agent>,
    pub decisions: Vec<Choice>,
    pub transitions: Vec<Transition>,
    pub initial_total: u64,
    pub food_accounting_matches: bool,
    pub sampled_decisions: bool,
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
    if header.schema != "experiment-023-trial/v1" {
        return Err("Unsupported history version".into());
    }
    let file = File::open(path).map_err(|e| format!("Missing/unreadable history: {e}"))?;
    let mut de = serde_json::Deserializer::from_reader(BufReader::new(file.take(MAX_HISTORY + 1)));
    let e = Envelope::deserialize(&mut de).map_err(parse_error)?;
    de.end()
        .map_err(|e| format!("Extra/corrupt history data: {e}"))?;
    if e.schema != "experiment-023-trial/v1"
        || e.history.schema != "experiment-023-physical-history/v1"
    {
        return Err("Unsupported history version".into());
    }
    let c = &e.config;
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
    let total = h
        .final_agents
        .iter()
        .try_fold(h.final_stock, |sum, a| sum.checked_add(a.food))
        .and_then(|sum| sum.checked_add(meals as u64));
    let mut decisions = Vec::new();
    for b in &h.boundaries.first {
        for choice in &b.choices {
            if decisions.len() < SAMPLE {
                decisions.push(choice.clone());
            }
        }
    }
    Ok(Summary {
        perspective: "Observer Perspective — recorded outcomes, not actor knowledge".into(),
        start: h.start,
        nominal_stop,
        end,
        completed_activities: h.transitions.completed,
        meals,
        resource_events: core.events.count,
        transferred_units: core.events.transferred,
        environmental_grants: h.grants.completed,
        final_stock: h.final_stock,
        final_agents: h.final_agents,
        decisions,
        transitions: h.transitions.first,
        initial_total: h.initial_total,
        food_accounting_matches: total == Some(h.initial_total),
        sampled_decisions: h.boundaries.count > SAMPLE,
    })
}
