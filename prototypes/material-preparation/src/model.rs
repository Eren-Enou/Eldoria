//! Prospective025 law; actor projection is separate from physical authority.
use crate::{
    acquisition as a, continuous as c, encounters as e, environmental_knowledge as k,
    experiment::concurrent as x, model::Agent, valuation as v,
};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceId(pub u32);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationPlan {
    pub period: u64,
    pub raw_offsets: [Option<u64>; 4],
    pub ready_offsets: [Option<u64>; 4],
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub feature_enabled: bool,
    pub access_enabled: bool,
    pub source_id: SourceId,
    pub raw_initial: u32,
    pub extraction_ticks: u64,
    pub preparation_ticks: u64,
    pub observation: ObservationPlan,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            feature_enabled: false,
            access_enabled: false,
            source_id: SourceId(0),
            raw_initial: 0,
            extraction_ticks: 2,
            preparation_ticks: 4,
            observation: ObservationPlan {
                period: 8,
                raw_offsets: [None; 4],
                ready_offsets: [Some(0); 4],
            },
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// Observer metadata; excluded from every local selector input.
    pub family: String,
    pub pair: Option<usize>,
    pub world: c::Config,
    pub valuation: v::Mode,
    pub preparation: Settings,
}
impl Config {
    pub fn validate(&self) -> Result<(), String> {
        self.world.validate()?;
        let s = &self.preparation;
        if self.world.mode != c::Mode::Continuous
            || s.source_id != SourceId(0)
            || s.raw_initial > 16
            || ![(2, 4), (4, 8), (8, 24)].contains(&(s.extraction_ticks, s.preparation_ticks))
            || ![8, 16].contains(&s.observation.period)
            || (!s.feature_enabled && s != &Settings::default())
        {
            return Err("unsupported bounded025configuration".into());
        }
        for (raw, ready) in s
            .observation
            .raw_offsets
            .iter()
            .zip(&s.observation.ready_offsets)
        {
            if raw.is_some_and(|x| x >= s.observation.period)
                || ready.is_some_and(|x| x >= s.observation.period)
                || (raw.is_some() && raw == ready)
            {
                return Err("invalid025observation offsets".into());
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Observation {
    pub id: SourceId,
    pub observed_at: u64,
    pub stock: u32,
    pub accessible: bool,
    pub extraction_ticks: u64,
    pub preparation_ticks: u64,
    pub input_units: u32,
    pub edible_units: u32,
    pub waste_units: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cell {
    pub owner: u32,
    pub receipt: u64,
    pub observation: Observation,
    pub own_takes: u32,
}
impl Cell {
    pub fn upper_bound(&self) -> u32 {
        self.observation.stock.saturating_sub(self.own_takes)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "WireInput", into = "WireInput")]
pub struct Input {
    pub inherited: k::Input,
    /// Disjoint alternative to inherited.retained; never a second usable slot.
    pub raw_retained: Option<Cell>,
}
// An explicit wire type preserves the old flat fields without serde flatten's
// buffered ContentDeserializer losing JSON integer-map-key semantics in Agent.
#[derive(Serialize, Deserialize)]
struct WireInput {
    legacy: e::Input,
    retained: Option<k::Cell>,
    duration: u64,
    inquiry_lane_free: bool,
    raw_retained: Option<Cell>,
}
impl From<WireInput> for Input {
    fn from(wire: WireInput) -> Self {
        Self {
            inherited: k::Input {
                legacy: wire.legacy,
                retained: wire.retained,
                duration: wire.duration,
                inquiry_lane_free: wire.inquiry_lane_free,
            },
            raw_retained: wire.raw_retained,
        }
    }
}
impl From<Input> for WireInput {
    fn from(input: Input) -> Self {
        Self {
            legacy: input.inherited.legacy,
            retained: input.inherited.retained,
            duration: input.inherited.duration,
            inquiry_lane_free: input.inherited.inquiry_lane_free,
            raw_retained: input.raw_retained,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Wait,
    Consume,
    Invite,
    Acquire,
    Prepare,
}
impl From<a::Action> for Action {
    fn from(action: a::Action) -> Self {
        match action {
            a::Action::Wait => Self::Wait,
            a::Action::Consume => Self::Consume,
            a::Action::Invite => Self::Invite,
            a::Action::Acquire => Self::Acquire,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Term {
    pub action: Action,
    pub eligible: bool,
    pub base_score: i32,
    pub meal_relief: i32,
    pub work_effort: i32,
    pub final_score: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Selection {
    pub terms: Vec<Term>,
    pub selected: Action,
}
pub type Selector = fn(&Input, &[v::Term]) -> Selection;
pub fn rank(input: &Input, original: &[v::Term]) -> Selection {
    assert!(input.inherited.retained.is_none() || input.raw_retained.is_none());
    assert_eq!(original.len(), 4);
    let local = &input.inherited.legacy;
    let duration = input.raw_retained.as_ref().and_then(|cell| {
        let o = &cell.observation;
        (cell.owner == local.own.id
            && o.id == SourceId(0)
            && o.observed_at <= local.now
            && cell.upper_bound() >= 2
            && o.accessible
            && o.extraction_ticks > 0
            && o.preparation_ticks > 0
            && (o.input_units, o.edible_units, o.waste_units) == (2, 1, 1))
            .then(|| o.extraction_ticks.checked_add(o.preparation_ticks))
            .flatten()
    });
    let eligible = local.own.food == 0
        && local.own.hunger > 0
        && local
            .people
            .iter()
            .any(|p| p.id == local.own.id && p.available)
        && duration.is_some_and(|d| {
            d > 0 && d <= 33 && local.now.checked_add(d).is_some_and(|end| end <= local.end)
        });
    let mut terms = original
        .iter()
        .map(|t| Term {
            action: t.action.into(),
            eligible: t.eligible,
            base_score: t.base_score,
            meal_relief: t.meal_relief,
            work_effort: 0,
            final_score: t.final_score,
        })
        .collect::<Vec<_>>();
    let work_effort = if eligible {
        i32::try_from(duration.unwrap()).unwrap()
    } else {
        0
    };
    let base_score = if eligible { local.own.hunger } else { -1 };
    terms.push(Term {
        action: Action::Prepare,
        eligible,
        base_score,
        meal_relief: 0,
        work_effort,
        final_score: base_score - work_effort,
    });
    assert_eq!(
        terms.iter().map(|t| t.action).collect::<Vec<_>>(),
        [
            Action::Wait,
            Action::Consume,
            Action::Invite,
            Action::Acquire,
            Action::Prepare
        ]
    );
    let selected = terms
        .iter()
        .fold(&terms[0], |best, next| {
            if next.final_score > best.final_score {
                next
            } else {
                best
            }
        })
        .action;
    Selection { terms, selected }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Choice {
    pub input: Input,
    pub candidates: Vec<Term>,
    pub selected: Action,
    pub contact_candidates: Vec<e::Candidate>,
    pub contact: e::Intent,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub boundary: usize,
    pub choice: usize,
    pub policy: v::Mode,
    pub selection: Selection,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Boundary {
    pub tick: u64,
    pub agents: Vec<Agent>,
    pub stock: u32,
    pub active: Vec<x::Commitment>,
    pub pending: Vec<x::Pending>,
    pub choices: Vec<Choice>,
    pub responses: Vec<e::Response>,
    pub events: usize,
    pub meals: usize,
    pub grants: usize,
    pub concerns: usize,
    pub assessment: usize,
    pub queries: usize,
    /// Observer reference only, never a local policy input.
    pub material_point: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct History {
    pub schema: String,
    pub config: x::Config,
    pub start: u64,
    pub end: u64,
    pub initial: Vec<Agent>,
    pub initial_stock: u32,
    /// Common-mass total includes raw in025, unlike old edible-only schemas.
    pub initial_total: u64,
    pub boundaries: Vec<Boundary>,
    pub transitions: Vec<x::Transition>,
    pub admissions: Vec<x::Admission>,
    pub needs: Vec<x::Need>,
    pub grants: Vec<x::Grant>,
    pub memories: Vec<k::Record>,
    pub final_memory: Vec<Option<k::Cell>>,
    pub profiles: Vec<crate::intentional::Profile>,
    pub bootstrap_events: usize,
    pub initial_contexts: x::LocalContexts,
    pub final_contexts: x::LocalContexts,
    pub final_state: crate::experiment013::Snapshot,
    pub final_agents: Vec<Agent>,
    pub final_stock: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Process {
    pub commitment: u64,
    pub owner: u32,
    pub source: SourceId,
    pub start: u64,
    pub extract_at: u64,
    pub finish_at: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Extracting,
    Preparing,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Active {
    pub process: Process,
    pub phase: Phase,
    pub escrow: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Extracted,
    Insufficient,
    Prepared,
    Failed,
}
pub fn extract(stock: u32, escrow: u32) -> Result<(u32, u32, Outcome), String> {
    if escrow != 0 {
        return Err("duplicate or nonempty extraction escrow".into());
    }
    if stock < 2 {
        return Ok((stock, 0, Outcome::Insufficient));
    }
    Ok((stock.checked_sub(2).unwrap(), 2, Outcome::Extracted))
}
pub fn convert(escrow: u32, food: u32, waste: u32) -> Result<(u32, u32, u32, Outcome), String> {
    match escrow {
        0 => Ok((0, food, waste, Outcome::Failed)),
        2 => Ok((
            0,
            food.checked_add(1).ok_or("food overflow")?,
            waste.checked_add(1).ok_or("waste overflow")?,
            Outcome::Prepared,
        )),
        _ => Err("invalid conversion escrow".into()),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Operation {
    pub id: u64,
    pub tick: u64,
    pub commitment: u64,
    pub owner: u32,
    pub source: SourceId,
    pub outcome: Outcome,
    pub raw_before: u32,
    pub raw_after: u32,
    pub escrow_before: u32,
    pub escrow_after: u32,
    pub food_before: u32,
    pub food_after: u32,
    pub waste_before: u32,
    pub waste_after: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Change {
    Sense,
    Taken,
    Insufficient,
    OverwrittenByReady,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Memory {
    pub id: u64,
    pub owner: u32,
    pub tick: u64,
    pub kind: Change,
    pub before: Option<Cell>,
    pub after: Option<Cell>,
    pub physical: Option<u64>,
    pub cache_record: Option<usize>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub tick: u64,
    pub raw: u32,
    pub waste: u32,
    pub active: Vec<Active>,
}
#[derive(Resource, Clone, Debug)]
pub struct Material {
    pub settings: Settings,
    pub raw: u32,
    pub waste: u32,
    pub active: Vec<Active>,
    pub slots: [Option<Cell>; 4],
    pub processes: Vec<Process>,
    pub operations: Vec<Operation>,
    pub memories: Vec<Memory>,
    pub points: Vec<Point>,
}
impl Material {
    pub fn new(settings: Settings, tick: u64) -> Self {
        let raw = settings.raw_initial;
        Self {
            settings,
            raw,
            waste: 0,
            active: vec![],
            slots: std::array::from_fn(|_| None),
            processes: vec![],
            operations: vec![],
            memories: vec![],
            points: vec![Point {
                tick,
                raw,
                waste: 0,
                active: vec![],
            }],
        }
    }
    pub fn remaining_mass(&self) -> u64 {
        u64::from(self.raw)
            + u64::from(self.waste)
            + self.active.iter().map(|p| u64::from(p.escrow)).sum::<u64>()
    }
    pub fn history(&self) -> MaterialHistory {
        MaterialHistory {
            initial_raw: self.settings.raw_initial,
            processes: self.processes.clone(),
            operations: self.operations.clone(),
            memories: self.memories.clone(),
            points: self.points.clone(),
            final_raw: self.raw,
            final_waste: self.waste,
            final_active: self.active.clone(),
            final_slots: self.slots.clone(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialHistory {
    pub initial_raw: u32,
    pub processes: Vec<Process>,
    pub operations: Vec<Operation>,
    pub memories: Vec<Memory>,
    pub points: Vec<Point>,
    pub final_raw: u32,
    pub final_waste: u32,
    pub final_active: Vec<Active>,
    pub final_slots: [Option<Cell>; 4],
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checkpoint {
    #[serde(flatten)]
    pub base: c::Checkpoint,
    pub material_point: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub schema: String,
    pub config: Config,
    pub history: History,
    pub checkpoints: Vec<Checkpoint>,
    pub valuations: Vec<Record>,
    pub material: MaterialHistory,
}
pub fn matrix() -> Vec<Config> {
    serde_json::from_str(include_str!(
        "../../../experiments/025/registered-matrix.json"
    ))
    .unwrap()
}
pub fn trial(config: Config) -> Trial {
    crate::simulation::Simulation::preparation_trial25(config)
}
/// Observer-only report projection, independent of the runtime report cadence.
pub fn checkpoints(history: &History, config: &c::Config, points: &[Point]) -> Vec<Checkpoint> {
    use std::collections::BTreeMap;
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
        .filter(|r| !r.completed)
        .map(|r| (r.commitment.id, &r.commitment))
        .collect();
    let ends: BTreeMap<_, _> = history
        .transitions
        .iter()
        .filter(|r| r.completed)
        .map(|r| (r.commitment.id, r.tick))
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
            for (source, b) in history.boundaries.iter().enumerate() {
                if b.tick <= tick && tick < b.tick + 2 {
                    for d in &b.choices {
                        if d.selected == Action::Invite {
                            pending.push(c::PendingCut {
                                source,
                                proposer: d.input.inherited.legacy.own.id,
                            });
                        }
                    }
                }
            }
            Checkpoint {
                base: c::Checkpoint {
                    tick,
                    labels,
                    active,
                    pending,
                    boundary: history.boundaries.iter().position(|b| b.tick == tick),
                },
                material_point: points.iter().rposition(|p| p.tick <= tick).unwrap(),
            }
        })
        .collect()
}

/// Feature-off exercises the frozen024 host with new type/physical seams inert.
/// Only additive025schema/audit representation changes; compiled parity checks
/// must reverse this exact projection against every original024serialized body.
pub fn from_legacy(config: Config, old: v::Trial) -> Trial {
    assert_eq!(config.preparation, Settings::default());
    let material = Material::new(config.preparation.clone(), old.history.start).history();
    let mut value = serde_json::to_value(old).unwrap();
    value["schema"] = "experiment-025-trial/v1".into();
    value["config"] = serde_json::to_value(config).unwrap();
    value["history"]["schema"] = "experiment-025-physical-history/v1".into();
    let audit = value["valuations"].as_array().unwrap().clone();
    for (bi, boundary) in value["history"]["boundaries"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .enumerate()
    {
        boundary["material_point"] = 0.into();
        for (ci, choice) in boundary["choices"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .enumerate()
        {
            choice["input"]["raw_retained"] = serde_json::Value::Null;
            let input: Input = serde_json::from_value(choice["input"].clone()).unwrap();
            let record = audit
                .iter()
                .find(|r| r["boundary"] == bi && r["choice"] == ci)
                .unwrap();
            let original: v::Selection =
                serde_json::from_value(record["selection"].clone()).unwrap();
            let selection = rank(&input, &original.terms);
            assert_eq!(selection.selected, original.selected.into());
            choice["candidates"] = serde_json::to_value(selection.terms).unwrap();
        }
    }
    for record in value["valuations"].as_array_mut().unwrap() {
        let bi = record["boundary"].as_u64().unwrap() as usize;
        let ci = record["choice"].as_u64().unwrap() as usize;
        let original: v::Selection = serde_json::from_value(record["selection"].clone()).unwrap();
        // No observer fields are used: feature off has no raw record by law.
        let mut terms = original
            .terms
            .into_iter()
            .map(|t| Term {
                action: t.action.into(),
                eligible: t.eligible,
                base_score: t.base_score,
                meal_relief: t.meal_relief,
                work_effort: 0,
                final_score: t.final_score,
            })
            .collect::<Vec<_>>();
        terms.push(Term {
            action: Action::Prepare,
            eligible: false,
            base_score: -1,
            meal_relief: 0,
            work_effort: 0,
            final_score: -1,
        });
        *record = serde_json::to_value(Record {
            boundary: bi,
            choice: ci,
            policy: serde_json::from_value(record["policy"].clone()).unwrap(),
            selection: Selection {
                terms,
                selected: original.selected.into(),
            },
        })
        .unwrap();
    }
    for cp in value["checkpoints"].as_array_mut().unwrap() {
        cp["material_point"] = 0.into();
    }
    value["material"] = serde_json::to_value(material).unwrap();
    serde_json::from_value(value).unwrap()
}
