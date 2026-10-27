//! Independent local arithmetic/mask checks; no policy or transition calls.
use serde_json::Value;
fn num(v: &Value, key: &str) -> Result<i64, String> {
    v[key]
        .as_i64()
        .ok_or_else(|| format!("missing numeric {key}"))
}
pub fn validate(state: &Value, mode: &str) -> Result<(), String> {
    if !matches!(mode, "Baseline" | "ProjectedNeed") {
        return Err("unknown explicit mode".into());
    }
    let boundaries = state["boundaries"]
        .as_array()
        .ok_or("missing saved boundaries")?;
    for boundary in boundaries {
        for choice in boundary["choices"].as_array().ok_or("missing choices")? {
            let i = &choice["input"];
            let local = &i["legacy"];
            let own = &local["own"];
            let owner = num(own, "id")?;
            let hunger = num(own, "hunger")?;
            let food = num(own, "food")?;
            let now = num(local, "now")?;
            let end = num(local, "end")?;
            let free = local["people"]
                .as_array()
                .ok_or("missing availability")?
                .iter()
                .any(|p| p["id"] == owner && p["available"] == true);
            let cell = &i["raw_retained"];
            let o = &cell["observation"];
            if !cell.is_null() && !i["retained"].is_null() {
                return Err("two source slots".into());
            }
            let mut duration = None;
            if !cell.is_null()
                && num(cell, "owner")? == owner
                && num(o, "id")? == 0
                && num(o, "observed_at")? <= now
                && o["accessible"] == true
                && (num(o, "stock")? - num(cell, "own_takes")?).max(0) >= 2
                && num(o, "extraction_ticks")? > 0
                && num(o, "preparation_ticks")? > 0
                && (
                    num(o, "input_units")?,
                    num(o, "edible_units")?,
                    num(o, "waste_units")?,
                ) == (2, 1, 1)
            {
                duration = Some(num(o, "extraction_ticks")? + num(o, "preparation_ticks")?);
            }
            let eligible = food == 0
                && hunger > 0
                && free
                && duration.is_some_and(|d| d > 0 && d <= 33 && now + d <= end);
            let d = if eligible {
                duration.ok_or("missing duration")?
            } else {
                0
            };
            let base = if !eligible {
                -1
            } else if mode == "Baseline" {
                hunger
            } else {
                (hunger + 5 * (d / 4)).min(100)
            };
            let terms = choice["candidates"].as_array().ok_or("missing terms")?;
            if terms.len() != 5
                || terms
                    .iter()
                    .map(|t| t["action"].as_str())
                    .collect::<Vec<_>>()
                    != vec![
                        Some("Wait"),
                        Some("Consume"),
                        Some("Invite"),
                        Some("Acquire"),
                        Some("Prepare"),
                    ]
            {
                return Err("candidate order".into());
            }
            let term = &terms[4];
            if term["eligible"] != eligible
                || num(term, "base_score")? != base
                || num(term, "work_effort")? != d
                || num(term, "final_score")? != base - d
                || num(term, "meal_relief")? != 0
                || (eligible && terms[1]["eligible"] == true)
            {
                return Err("independent026 local score/mask mismatch".into());
            }
            let mut selected = &terms[0];
            for term in terms.iter().skip(1) {
                if num(term, "final_score")? > num(selected, "final_score")? {
                    selected = term;
                }
            }
            if choice["selected"] != selected["action"] {
                return Err("independent026 stable tie mismatch".into());
            }
        }
        for response in boundary["responses"]
            .as_array()
            .ok_or("missing responses")?
        {
            let candidates = response["candidates"].as_array().ok_or("missing replies")?;
            if candidates.first() != Some(&serde_json::json!({"reply":"Wait","score":0})) {
                return Err("Wait-first reply".into());
            }
            let mut selected = &candidates[0];
            for c in candidates.iter().skip(1) {
                if c["reply"] == "Continue" && num(c, "score")? != 0 {
                    return Err("C1 Continue must be zero".into());
                }
                if num(c, "score")? > num(selected, "score")? {
                    selected = c;
                }
            }
            if response["selected"] != selected["reply"] {
                return Err("C1 response tie mismatch".into());
            }
        }
    }
    let mut pending = boundaries.last().map_or(Ok(vec![]), |b| {
        b["pending"]
            .as_array()
            .cloned()
            .ok_or("missing boundary pending")
    })?;
    if let Some(last) = boundaries.last() {
        for choice in last["choices"].as_array().ok_or("missing last choices")? {
            if choice["selected"] == "Invite" {
                let local = &choice["input"]["legacy"];
                let invitation = &choice["contact"]["Invite"];
                pending.push(serde_json::json!({"source":boundaries.len()-1,"invitation":{"proposer":local["own"]["id"],"recipient":invitation["recipient"],"kind":invitation["kind"],"expires":num(local,"now")?+2},"question":local["query"]["decision"]["selected"],"response":null}));
            }
        }
    }
    if state["pending"] != Value::Array(pending) {
        return Err("independent missing/changed pending protocol state".into());
    }
    Ok(())
}

/// Independently integrate saved primitive need/meal records and episodic IDs.
/// This checks retained state; it never populates or repairs an individual's memory.
pub fn retained_primitives(state: &Value) -> Result<(), String> {
    let agents = state["agents"].as_array().ok_or("missing actors")?;
    let initial = state["initial"]
        .as_array()
        .ok_or("missing initial actors")?;
    let needs = state["needs"].as_array().ok_or("missing need records")?;
    let meals = state["meals"].as_array().ok_or("missing meals")?;
    let events = state["events"].as_array().ok_or("missing actual events")?;
    if agents.len() != 4 || initial.len() != 4 {
        return Err("four actor identities required".into());
    }
    for (owner, actor) in agents.iter().enumerate() {
        if actor["id"] != owner || initial[owner]["id"] != owner {
            return Err("actor ordering".into());
        }
        let mut changes = vec![];
        for n in needs {
            changes.push((
                num(n, "tick")?,
                0,
                n["before"][owner].as_i64().ok_or("need before")?,
                n["after"][owner].as_i64().ok_or("need after")?,
            ));
        }
        for m in meals.iter().filter(|m| m["actor"] == owner) {
            if num(m, "food_before")? - num(m, "food_after")? != 1 || num(m, "consumed")? != 1 {
                return Err("meal accounting".into());
            }
            changes.push((
                num(m, "tick")?,
                1,
                num(m, "hunger_before")?,
                num(m, "hunger_after")?,
            ));
        }
        changes.sort_by_key(|c| (c.0, c.1));
        let mut hunger = num(&initial[owner], "hunger")?;
        for (_, kind, before, after) in changes {
            let expected = if kind == 0 {
                (before + 5).min(100)
            } else {
                (before - 40).max(0)
            };
            if hunger != before || after != expected {
                return Err("independent need/meal integration".into());
            }
            hunger = after;
        }
        if num(actor, "hunger")? != hunger {
            return Err("independent saved hunger mismatch".into());
        }
        let mut expected = events
            .iter()
            .filter(|e| {
                e["participants"]
                    .as_array()
                    .is_some_and(|ps| ps.iter().any(|id| id == owner))
            })
            .map(|e| e["id"].clone())
            .collect::<Vec<_>>();
        if expected.len() > 16 {
            expected.drain(..expected.len() - 16);
        }
        let retained = actor["memories"]
            .as_array()
            .ok_or("missing retained episodes")?
            .iter()
            .map(|m| m["event"].clone())
            .collect::<Vec<_>>();
        if retained != expected {
            return Err("independent unavailable episode restored or membership lost".into());
        }
    }
    Ok(())
}
