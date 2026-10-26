//! Read-only material reconstruction. No simulator transitions or policy calls.
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// Compare immutable material deltas with the saved audit, without replaying them
/// into the individual or filling any cognitive field.
pub fn journal(material: &Value, records: &[Value]) -> Result<(), String> {
    let initial = &records
        .first()
        .ok_or("material audit: missing journal header")?["bootstrap_audit"]["material"];
    if initial["settings"] != material["settings"]
        || initial["raw"] != material["settings"]["raw_initial"]
        || initial["waste"] != 0
        || !list(initial, "active")?.is_empty()
    {
        return Err("material audit: invalid initial journal material".into());
    }
    for (key, delta) in [
        ("processes", "material_processes"),
        ("operations", "material_operations"),
        ("memories", "material_memories"),
        ("points", "material_points"),
    ] {
        let mut combined = list(initial, key)?.clone();
        for record in records.iter().skip(1) {
            combined.extend(list(record, delta)?.iter().cloned());
            let point = record["boundary"]["material_point"]
                .as_u64()
                .ok_or("material audit: missing boundary material point")?
                as usize;
            if point >= list(material, "points")?.len() {
                return Err("material audit: invalid boundary point".into());
            }
        }
        if combined != *list(material, key)? {
            return Err(format!("material audit: {key} journal prefix mismatch"));
        }
    }
    Ok(())
}
fn n(v: &Value, key: &str) -> Result<u64, String> {
    v[key]
        .as_u64()
        .ok_or_else(|| format!("material audit: missing unsigned {key}"))
}
fn list<'a>(v: &'a Value, key: &str) -> Result<&'a Vec<Value>, String> {
    v[key]
        .as_array()
        .ok_or_else(|| format!("material audit: missing array {key}"))
}
pub fn validate(material: &Value, physical: &Value, tick: u64) -> Result<(), String> {
    let settings = &material["settings"];
    let mut raw = n(settings, "raw_initial")?;
    let mut waste = 0;
    let mut processes = BTreeMap::new();
    for p in list(material, "processes")? {
        let id = n(p, "commitment")?;
        if n(p, "owner")? >= 4
            || p["source"] != settings["source_id"]
            || n(p, "start")?.checked_add(n(settings, "extraction_ticks")?)
                != Some(n(p, "extract_at")?)
            || n(p, "extract_at")?.checked_add(n(settings, "preparation_ticks")?)
                != Some(n(p, "finish_at")?)
            || n(p, "start")? > tick
            || processes.insert(id, p).is_some()
        {
            return Err("material audit: process identity/deadline mismatch".into());
        }
        let started = list(physical, "transitions")?
            .iter()
            .find(|t| t["completed"] == false && t["commitment"]["id"] == id)
            .ok_or("material audit: process has no actual admission")?;
        let c = &started["commitment"];
        if c["work"] != "Prepare"
            || c["participants"] != serde_json::json!([n(p, "owner")?])
            || c["start"] != p["start"]
            || c["deadline"] != p["finish_at"]
        {
            return Err("material audit: process/admission mismatch".into());
        }
    }
    let mut extracted = BTreeMap::new();
    let mut completed = BTreeSet::new();
    let mut last_tick = 0;
    let mut extraction_order = None;
    for (i, o) in list(material, "operations")?.iter().enumerate() {
        let id = n(o, "commitment")?;
        let p = processes
            .get(&id)
            .ok_or("material audit: operation without process")?;
        let time = n(o, "tick")?;
        if n(o, "id")? != i as u64
            || time < last_tick
            || time > tick
            || o["owner"] != p["owner"]
            || o["source"] != p["source"]
            || n(o, "raw_before")? != raw
            || n(o, "waste_before")? != waste
        {
            return Err("material audit: operation order/identity/stock mismatch".into());
        }
        match o["outcome"].as_str() {
            Some("Extracted" | "Insufficient") => {
                if time != n(p, "extract_at")?
                    || extracted.contains_key(&id)
                    || n(o, "escrow_before")? != 0
                    || o["food_before"] != o["food_after"]
                    || n(o, "waste_after")? != waste
                {
                    return Err(
                        "material audit: extraction timing/duplicate/premature output".into(),
                    );
                }
                let owner = n(p, "owner")?;
                let rank = match physical["config"]["order"].as_str() {
                    Some("Ascending") => owner,
                    Some("Descending") => 3 - owner,
                    Some("Rotating") => (owner + 4 - n(p, "start")? % 4) % 4,
                    _ => return Err("material audit: unknown deterministic order".into()),
                };
                let key = (time, rank, id);
                if extraction_order.is_some_and(|prior| prior > key) {
                    return Err("material audit: extraction contention order".into());
                }
                extraction_order = Some(key);
                let success = raw >= 2;
                let escrow = if success {
                    raw -= 2;
                    2
                } else {
                    0
                };
                if (o["outcome"] == "Extracted") != success
                    || n(o, "raw_after")? != raw
                    || n(o, "escrow_after")? != escrow
                {
                    return Err("material audit: extraction law mismatch".into());
                }
                extracted.insert(id, escrow);
            }
            Some("Prepared" | "Failed") => {
                let escrow = *extracted
                    .get(&id)
                    .ok_or("material audit: conversion before extraction")?;
                if time != n(p, "finish_at")?
                    || !completed.insert(id)
                    || n(o, "escrow_before")? != escrow
                    || n(o, "escrow_after")? != 0
                    || n(o, "raw_after")? != raw
                    || (o["outcome"] == "Prepared") != (escrow == 2)
                {
                    return Err("material audit: conversion phase/duplicate mismatch".into());
                }
                let output = u64::from(escrow == 2);
                waste += output;
                if n(o, "food_before")?.checked_add(output) != Some(n(o, "food_after")?)
                    || n(o, "waste_after")? != waste
                {
                    return Err("material audit: conversion food/waste law mismatch".into());
                }
            }
            _ => return Err("material audit: unknown operation".into()),
        }
        last_tick = time;
    }
    if raw != n(material, "raw")? || waste != n(material, "waste")? {
        return Err("material audit: final raw/waste mismatch".into());
    }
    let active = list(material, "active")?;
    let physical_active = list(physical, "active")?;
    let mut active_ids = BTreeSet::new();
    for a in active {
        let p = &a["process"];
        let id = n(p, "commitment")?;
        if processes.get(&id).copied() != Some(p)
            || completed.contains(&id)
            || !active_ids.insert(id)
            || n(p, "finish_at")? <= tick
        {
            return Err("material audit: active process identity mismatch".into());
        }
        let expected = if tick < n(p, "extract_at")? {
            ("Extracting", 0)
        } else {
            match extracted.get(&id) {
                Some(2) => ("Preparing", 2),
                Some(0) => ("Failed", 0),
                _ => return Err("material audit: skipped extraction".into()),
            }
        };
        if a["phase"] != expected.0
            || n(a, "escrow")? != expected.1
            || !physical_active.iter().any(|c| {
                c["id"] == id
                    && c["work"] == "Prepare"
                    && c["participants"] == serde_json::json!([n(p, "owner").unwrap_or(99)])
            })
        {
            return Err("material audit: escrow/phase/occupied actor mismatch".into());
        }
    }
    if processes
        .keys()
        .copied()
        .filter(|id| !completed.contains(id))
        .collect::<BTreeSet<_>>()
        != active_ids
        || physical_active
            .iter()
            .filter(|c| c["work"] == "Prepare")
            .count()
            != active.len()
    {
        return Err("material audit: missing/duplicate active process".into());
    }
    let slots = list(material, "slots")?;
    let ready = list(physical, "slots")?;
    if slots.len() != 4 || ready.len() != 4 {
        return Err("material audit: environmental capacity".into());
    }
    let mut retained = vec![Value::Null; 4];
    let mut receipts = [
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
    ];
    for (i, r) in list(material, "memories")?.iter().enumerate() {
        let owner = n(r, "owner")? as usize;
        if owner >= 4
            || n(r, "id")? != i as u64
            || n(r, "tick")? > tick
            || r["before"] != retained[owner]
        {
            return Err("material audit: memory transition mismatch".into());
        }
        if !r["after"].is_null()
            && (r["after"]["owner"] != owner
                || r["after"]["observation"]["id"] != settings["source_id"])
        {
            return Err("material audit: memory owner/source mismatch".into());
        }
        if !r["after"].is_null() {
            let cell = &r["after"];
            let observation = &cell["observation"];
            let observed = n(observation, "observed_at")?;
            let observed_raw = n(settings, "raw_initial")?
                - list(material, "operations")?
                    .iter()
                    .filter(|o| {
                        o["outcome"] == "Extracted"
                            && o["tick"].as_u64().is_some_and(|t| t <= observed)
                    })
                    .count() as u64
                    * 2;
            if observed > n(r, "tick")?
                || n(observation, "stock")? != observed_raw
                || observation["accessible"] != settings["access_enabled"]
                || observation["extraction_ticks"] != settings["extraction_ticks"]
                || observation["preparation_ticks"] != settings["preparation_ticks"]
                || observation["input_units"] != 2
                || observation["edible_units"] != 1
                || observation["waste_units"] != 1
            {
                return Err("material audit: illegitimate raw observation".into());
            }
            receipts[owner].insert(n(cell, "receipt")?);
        }
        match r["kind"].as_str() {
            Some("Taken") => {
                let id = n(r, "physical")? as usize;
                let operation = list(material, "operations")?
                    .get(id)
                    .ok_or("material audit: missing take operation")?;
                let mut expected = r["before"].clone();
                expected["own_takes"] = (n(&expected, "own_takes")?
                    .checked_add(2)
                    .ok_or("own take overflow")?)
                .into();
                if operation["outcome"] != "Extracted"
                    || operation["owner"] != owner
                    || operation["tick"] != r["tick"]
                    || expected != r["after"]
                {
                    return Err("material audit: incorrect own extraction memory".into());
                }
            }
            Some("Sense" | "Insufficient") => {
                if r["after"].is_null()
                    || r["after"]["own_takes"] != 0
                    || r["after"]["observation"]["observed_at"] != r["tick"]
                {
                    return Err("material audit: missing new local observation".into());
                }
                if r["kind"] == "Insufficient" {
                    let id = n(r, "physical")? as usize;
                    let operation = list(material, "operations")?
                        .get(id)
                        .ok_or("material audit: missing insufficient operation")?;
                    if operation["outcome"] != "Insufficient"
                        || operation["owner"] != owner
                        || operation["tick"] != r["tick"]
                    {
                        return Err("material audit: incorrect insufficient memory".into());
                    }
                }
            }
            Some("OverwrittenByReady") if r["after"].is_null() => {}
            _ => return Err("material audit: unknown memory transition".into()),
        }
        retained[owner] = r["after"].clone();
    }
    if retained != *slots
        || slots
            .iter()
            .zip(ready)
            .any(|(raw, ready)| !raw.is_null() && !ready.is_null())
    {
        return Err("material audit: restored unavailable cognition or capacity mismatch".into());
    }
    for r in list(physical, "memories")? {
        if !r["after"].is_null() {
            let owner = n(r, "owner")? as usize;
            if owner >= 4 {
                return Err("material audit: invalid ready owner".into());
            }
            receipts[owner].insert(n(&r["after"], "receipt")?);
        }
    }
    let counters = list(physical, "receipts")?;
    if counters.len() != 4 {
        return Err("material audit: receipt capacity".into());
    }
    for owner in 0..4 {
        let count = counters[owner]
            .as_u64()
            .ok_or("material audit: invalid receipt counter")?;
        if receipts[owner].len() as u64 != count || receipts[owner].iter().copied().ne(0..count) {
            return Err("material audit: missing/shared receipt counter mismatch".into());
        }
    }
    Ok(())
}
