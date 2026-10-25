//! Narrow independent storage/primitive reader: no World, Session or simulation validation calls.
use ciborium::value::Value;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Cursor,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Commit {
    binding_sha256: String,
    schema: String,
    world: String,
    runtime: String,
    generation: u64,
    tick: u64,
    journal_bytes: u64,
    journal_sha256: String,
    state_sha256: String,
    #[serde(with = "serde_bytes")]
    state: Vec<u8>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    schema: String,
    runtime: String,
    checksum: String,
    #[serde(with = "serde_bytes")]
    payload: Vec<u8>,
}
#[derive(Debug, Serialize)]
pub struct Verified {
    pub schema: &'static str,
    pub world: String,
    pub generation: u64,
    pub runtime: String,
    pub journal_runtime: String,
    pub tick: u64,
    pub prefix_bytes: u64,
    pub state_payload_sha256: String,
    pub events: usize,
    pub started_commitments: usize,
    pub completed_commitments: usize,
    pub conserved_food: u64,
    pub scope: &'static str,
}
fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, String> {
    let mut c = Cursor::new(bytes);
    let value = ciborium::from_reader(&mut c).map_err(|e| e.to_string())?;
    if c.position() != bytes.len() as u64 {
        return Err("independent reader: trailing CBOR bytes".into());
    }
    Ok(value)
}
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn field<'a>(v: &'a Value, key: &str) -> Result<&'a Value, String> {
    if let Value::Map(fields) = v {
        return fields
            .iter()
            .find_map(|(k, v)| matches!(k,Value::Text(s) if s==key).then_some(v))
            .ok_or_else(|| format!("independent reader: missing {key}"));
    }
    Err("independent reader: expected map".into())
}
fn array(v: &Value) -> Result<&[Value], String> {
    if let Value::Array(a) = v {
        Ok(a)
    } else {
        Err("independent reader: expected array".into())
    }
}
fn number(v: &Value) -> Result<u64, String> {
    if let Value::Integer(i) = v {
        u64::try_from(*i).map_err(|e| e.to_string())
    } else {
        Err("independent reader: expected unsigned integer".into())
    }
}
fn n(v: &Value, key: &str) -> Result<u64, String> {
    number(field(v, key)?)
}
fn same_json(v: &Value, json: &serde_json::Value) -> Result<bool, String> {
    let mut bytes = vec![];
    ciborium::into_writer(json, &mut bytes).map_err(|e| e.to_string())?;
    let candidate: Value = decode(&bytes)?;
    // Serialize to JSON only for string-keyed configuration values, never private tuple-keyed maps.
    Ok(serde_json::to_value(v).map_err(|e| e.to_string())?
        == serde_json::to_value(candidate).map_err(|e| e.to_string())?)
}

/// Checks only a selected checkpoint and its exact journal prefix. No tail is replayed.
/// Runtime eligibility is a separate caller decision; this returns the actual identities.
pub fn verify(checkpoint: &[u8], prefix: &[u8]) -> Result<Verified, String> {
    if checkpoint.len() > 160 * 1024 * 1024 || prefix.len() > 256 * 1024 * 1024 {
        return Err("independent input size limit".into());
    }
    let commit: Commit = decode(checkpoint)?;
    let binding = serde_json::to_vec(&(
        &commit.schema,
        &commit.world,
        &commit.runtime,
        commit.generation,
        commit.tick,
        commit.journal_bytes,
        &commit.journal_sha256,
        &commit.state_sha256,
    ))
    .map_err(|e| e.to_string())?;
    if commit.schema != "woi-world-commit/v1"
        || digest(&binding) != commit.binding_sha256
        || digest(&commit.state) != commit.state_sha256
        || prefix.len() as u64 != commit.journal_bytes
        || digest(prefix) != commit.journal_sha256
        || !prefix.ends_with(b"\n")
    {
        return Err("independent commit binding/state/prefix verification failed".into());
    }
    let envelope: Envelope = decode(&commit.state)?;
    if envelope.schema != "woi-world-checkpoint/v1"
        || envelope.runtime != commit.runtime
        || digest(&envelope.payload) != envelope.checksum
    {
        return Err("independent state envelope verification failed".into());
    }
    let state: Value = decode(&envelope.payload)?;
    let runtime = field(&state, "runtime")?;
    let physical = field(&state, "state")?;
    let start = n(&state, "start")?;
    let end = n(&state, "end")?;
    let tick = n(runtime, "tick")?;
    if tick != commit.tick
        || tick < start
        || tick > end
        || n(physical, "synced")? != tick
        || start
            .checked_add(n(field(&state, "config")?, "horizon")?)
            .and_then(|n| n.checked_add(33))
            != Some(end)
    {
        return Err("independent clock/endpoint mismatch".into());
    }
    let records = prefix
        .split(|b| *b == b'\n')
        .filter(|r| !r.is_empty())
        .map(serde_json::from_slice::<serde_json::Value>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    let header = records
        .first()
        .ok_or("independent missing journal header")?;
    if header["schema"] != "woi-world-journal/v1"
        || header["world"] != commit.world
        || !same_json(field(&state, "config")?, &header["config"])?
        || !same_json(field(&state, "valuation")?, &header["valuation"])?
    {
        return Err("independent journal/configuration mismatch".into());
    }
    let boundaries = array(field(&state, "boundaries")?)?;
    if records.len() != boundaries.len() + 1 {
        return Err("independent boundary count mismatch".into());
    }
    for (index, record) in records.iter().skip(1).enumerate() {
        if record["schema"] != "woi-world-step/v1"
            || record["world"] != commit.world
            || record["sequence"] != index
            || !same_json(&boundaries[index], &record["boundary"])?
        {
            return Err("independent journal boundary mismatch".into());
        }
    }
    let events = array(field(runtime, "events")?)?;
    let mut last = 0;
    for (index, event) in events.iter().enumerate() {
        let time = n(event, "tick")?;
        if n(event, "id")? != index as u64 || time < last || time > tick {
            return Err("independent event identity/order mismatch".into());
        }
        last = time;
    }
    let agents = array(field(&state, "agents")?)?;
    let mut food = n(physical, "stock")?;
    if agents.len() != 4 {
        return Err("independent actor count mismatch".into());
    }
    for (index, agent) in agents.iter().enumerate() {
        if n(agent, "id")? != index as u64 || n(agent, "hunger")? > 100 {
            return Err("independent actor identity/need mismatch".into());
        }
        food = food
            .checked_add(n(agent, "food")?)
            .ok_or("independent food overflow")?;
    }
    for meal in array(field(field(&state, "cognition")?, "consumption")?)? {
        food = food
            .checked_add(n(meal, "consumed")?)
            .ok_or("independent consumption overflow")?;
    }
    if food != n(&state, "total")? || food != n(physical, "total")? {
        return Err("independent resource conservation mismatch".into());
    }
    let mut starts = BTreeMap::new();
    let mut completions = BTreeSet::new();
    let mut prior = 0;
    for transition in array(field(physical, "transitions")?)? {
        let commitment = field(transition, "commitment")?;
        let id = n(commitment, "id")?;
        let time = n(transition, "tick")?;
        let begin = n(commitment, "start")?;
        let deadline = n(commitment, "deadline")?;
        if time < prior
            || time > tick
            || time < begin
            || time > deadline
            || id >= n(physical, "next")?
        {
            return Err("independent commitment timing mismatch".into());
        }
        prior = time;
        match field(transition, "completed")? {
            Value::Bool(false) => {
                if starts.insert(id, commitment).is_some() {
                    return Err("independent duplicate commitment start".into());
                }
            }
            Value::Bool(true) => {
                if starts.get(&id).copied() != Some(commitment) || !completions.insert(id) {
                    return Err("independent duplicate/unmatched completion".into());
                }
            }
            _ => return Err("independent invalid completion flag".into()),
        }
    }
    let active = array(field(physical, "active")?)?;
    let mut active_ids = BTreeSet::new();
    let mut owners = BTreeSet::new();
    for c in active {
        let id = n(c, "id")?;
        if starts.get(&id).copied() != Some(c)
            || completions.contains(&id)
            || !active_ids.insert(id)
            || n(c, "deadline")? <= tick
        {
            return Err("independent active commitment mismatch".into());
        }
        for owner in array(field(c, "participants")?)? {
            let owner = number(owner)?;
            if owner >= 4 || !owners.insert(owner) {
                return Err("independent overlapping activity".into());
            }
        }
    }
    if starts
        .keys()
        .copied()
        .filter(|id| !completions.contains(id))
        .collect::<BTreeSet<_>>()
        != active_ids
    {
        return Err("independent missing active commitment".into());
    }
    match field(&state, "execution_phase")? {
        Value::Text(s)
            if s == "Terminal"
                && tick == end
                && active.is_empty()
                && array(field(physical, "pending")?)?.is_empty() => {}
        Value::Text(s) if s == "BeforeBoundary" && tick < end => {}
        _ => return Err("independent execution phase mismatch".into()),
    }
    Ok(Verified {
        schema: "woi-independent-persistence-check/v1",
        world: commit.world,
        generation: commit.generation,
        runtime: commit.runtime,
        journal_runtime: header["runtime"]
            .as_str()
            .ok_or("missing header runtime")?
            .into(),
        tick,
        prefix_bytes: commit.journal_bytes,
        state_payload_sha256: envelope.checksum,
        events: events.len(),
        started_commitments: starts.len(),
        completed_commitments: completions.len(),
        conserved_food: food,
        scope: "Independent raw CBOR/JSON hashes, configuration, event ordering, commitment lifecycle and aggregate food ledger; not a cognition validator or behavioral equivalence proof",
    })
}
