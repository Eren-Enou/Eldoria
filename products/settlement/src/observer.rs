//! Observer data is copied out. It cannot be fed to a policy or restored as a save.
use crate::{Settlement, model::*};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize)]
pub struct Snapshot {
    pub schema: &'static str,
    pub rules: &'static str,
    pub runtime: &'static str,
    pub tick: u64,
    pub horizon: u64,
    pub complete: bool,
    pub characters: Vec<Character>,
    pub places: Vec<Place>,
    pub commitments: Vec<Commitment>,
    pub food_balance: FoodBalance,
    pub history_records: usize,
    pub outcomes: BTreeMap<String, usize>,
}
#[derive(Serialize)]
pub struct FoodBalance {
    pub initial: u64,
    pub sources: u64,
    pub owned: u64,
    pub consumed: u64,
}
pub fn snapshot(world: &Settlement) -> Snapshot {
    let state = world.state();
    let characters: Vec<_> = world.characters().into_values().collect();
    let mut outcomes = BTreeMap::new();
    for record in &state.history {
        let name = match &record.kind {
            RecordKind::Finished { outcome, .. } => outcome.clone(),
            RecordKind::Refused { reason, .. } => format!("refused:{reason}"),
            RecordKind::Rejected { reason, .. } => format!("rejected:{reason}"),
            _ => continue,
        };
        *outcomes.entry(name).or_insert(0) += 1;
    }
    let food_balance = FoodBalance {
        initial: state.initial_food,
        sources: state.places.values().map(|p| u64::from(p.food)).sum(),
        owned: characters.iter().map(|a| u64::from(a.food)).sum(),
        consumed: state.consumed,
    };
    Snapshot {
        schema: "eldoria-settlement-observer/v1",
        rules: RULES,
        runtime: RUNTIME,
        tick: state.tick,
        horizon: state.config.horizon,
        complete: world.complete(),
        characters,
        places: state.places.values().cloned().collect(),
        commitments: state.commitments.values().cloned().collect(),
        food_balance,
        history_records: state.history.len(),
        outcomes,
    }
}
pub fn records(world: &Settlement, from: usize, limit: usize) -> &[Record] {
    let records = &world.state().history;
    let from = from.min(records.len());
    &records[from..from.saturating_add(limit.min(1000)).min(records.len())]
}
