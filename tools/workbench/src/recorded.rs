//! Streaming, observer-only navigation. Retains compact lanes and one selected
//! decision; it never loads the full history into a JSON value.
use crate::{
    Result,
    history::{Choice, Event, Meal, Transition},
    storage,
};
use serde::{
    Deserialize,
    de::{DeserializeSeed, IgnoredAny, MapAccess, SeqAccess, Visitor},
};
use std::{
    fmt,
    fs::File,
    io::{BufReader, Read},
    marker::PhantomData,
    path::Path,
};

struct Scan<'a, T, F> {
    path: &'a [&'a str],
    callback: &'a mut F,
    marker: PhantomData<T>,
}
impl<'de, T: Deserialize<'de>, F: FnMut(usize, T)> DeserializeSeed<'de> for Scan<'_, T, F> {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> std::result::Result<(), D::Error> {
        if self.path.is_empty() {
            d.deserialize_seq(self)
        } else {
            d.deserialize_map(self)
        }
    }
}
impl<'de, T: Deserialize<'de>, F: FnMut(usize, T)> Visitor<'de> for Scan<'_, T, F> {
    type Value = ();
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("recorded history path")
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> std::result::Result<(), A::Error> {
        let mut found = false;
        while let Some(key) = map.next_key::<String>()? {
            if key == self.path[0] {
                if found {
                    return Err(serde::de::Error::custom("Duplicate history field"));
                }
                found = true;
                map.next_value_seed(Scan::<T, F> {
                    path: &self.path[1..],
                    callback: self.callback,
                    marker: PhantomData,
                })?;
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }
        if !found {
            return Err(serde::de::Error::custom("Missing recorded history path"));
        }
        Ok(())
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> std::result::Result<(), A::Error> {
        let mut n = 0;
        while let Some(item) = seq.next_element::<T>()? {
            if n >= 65536 {
                return Err(serde::de::Error::custom("Record count exceeds65536"));
            }
            (self.callback)(n, item);
            n += 1;
        }
        Ok(())
    }
}
fn scan<T: for<'de> Deserialize<'de>>(
    path: &Path,
    keys: &[&str],
    mut callback: impl FnMut(usize, T),
) -> Result<()> {
    storage::no_links(path)?;
    let file = File::open(path).map_err(|e| e.to_string())?;
    if file.metadata().map_err(|e| e.to_string())?.len() > crate::history::MAX_HISTORY {
        return Err("History exceeds64MiB".into());
    }
    let mut de = serde_json::Deserializer::from_reader(BufReader::new(
        file.take(crate::history::MAX_HISTORY + 1),
    ));
    Scan {
        path: keys,
        callback: &mut callback,
        marker: PhantomData,
    }
    .deserialize(&mut de)
    .map_err(|e| e.to_string())?;
    de.end().map_err(|e| e.to_string())
}
#[derive(Deserialize)]
struct Boundary {
    tick: u64,
    choices: Vec<Choice>,
    #[serde(default)]
    agents: Vec<crate::history::Agent>,
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct Moment {
    pub boundary: usize,
    pub tick: u64,
    pub lanes: [Option<String>; 4],
    pub states: [Option<(u64, u64)>; 4],
    pub executed: [Vec<String>; 4],
    pub consequences: [Vec<String>; 4],
}
pub fn moments(path: &Path) -> Result<Vec<Moment>> {
    let mut rows = Vec::new();
    let mut invalid = false;
    scan::<Boundary>(path, &["history", "boundaries"], |boundary, b| {
        let mut lanes: [Option<String>; 4] = Default::default();
        let mut states = Default::default();
        if b.choices.len() > 4 {
            invalid = true;
        }
        for c in b.choices {
            let actor = c.input.legacy.own.id as usize;
            if actor >= 4 || lanes[actor].is_some() {
                invalid = true;
            } else {
                lanes[actor] = Some(c.selected);
            }
        }
        for a in b.agents {
            if a.id < 4 {
                let all: &mut [Option<(u64, u64)>; 4] = &mut states;
                all[a.id as usize] = Some((a.food, a.hunger));
            } else {
                invalid = true;
            }
        }
        rows.push(Moment {
            boundary,
            tick: b.tick,
            lanes,
            states,
            executed: Default::default(),
            consequences: Default::default(),
        });
    })?;
    if invalid {
        return Err("Invalid four-person boundary".into());
    }
    let ticks: std::collections::BTreeMap<_, _> =
        rows.iter().enumerate().map(|(i, r)| (r.tick, i)).collect();
    if ticks.len() != rows.len() {
        return Err("Duplicate recorded boundary time".into());
    }
    scan::<Transition>(path, &["history", "transitions"], |_, t| {
        if let Some(&index) = ticks.get(&t.tick) {
            for actor in t.commitment.participants {
                if actor < 4 {
                    rows[index].executed[actor as usize].push(format!(
                        "{} commitment{} {}",
                        if t.completed { "Completed" } else { "Started" },
                        t.commitment.id,
                        t.commitment.work
                    ));
                }
            }
        }
    })?;
    scan::<Event>(
        path,
        &[
            "history",
            "final_state",
            "base",
            "base",
            "base",
            "base",
            "base",
            "events",
        ],
        |_, e| {
            if let Some(&index) = ticks.get(&e.tick) {
                for actor in e.participants {
                    if actor < 4 {
                        rows[index].consequences[actor as usize].push(format!(
                            "Event{} scene{}: person{} {} · food{:?}→{:?} · transfer{}",
                            e.id,
                            e.scene,
                            e.decision.actor,
                            e.decision.selected,
                            e.balances_before,
                            e.balances_after,
                            e.transferred
                        ));
                    }
                }
            }
        },
    )?;
    scan::<Meal>(
        path,
        &[
            "history",
            "final_state",
            "base",
            "base",
            "base",
            "base",
            "base",
            "cognition",
            "consumption",
        ],
        |_, m| {
            if let Some(&index) = ticks.get(&m.tick)
                && m.actor < 4
            {
                rows[index].consequences[m.actor as usize].push(format!(
                    "Meal: food{}→{} hunger{}→{}",
                    m.food_before, m.food_after, m.hunger_before, m.hunger_after
                ));
            }
        },
    )?;
    Ok(rows)
}
pub struct Detail {
    pub choice: Option<Choice>,
    pub transitions: Vec<Transition>,
    pub events: Vec<Event>,
    pub meals: Vec<Meal>,
}
pub fn detail(path: &Path, boundary: usize, actor: u32) -> Result<Detail> {
    if actor > 3 {
        return Err("Unknown person".into());
    }
    let mut choice = None;
    let mut tick = None;
    scan::<Boundary>(path, &["history", "boundaries"], |index, b| {
        if index == boundary {
            tick = Some(b.tick);
            choice = b
                .choices
                .into_iter()
                .find(|c| c.input.legacy.own.id == actor);
        }
    })?;
    let tick = tick.ok_or("Unknown recorded moment")?;
    let mut transitions: Vec<Transition> = Vec::new();
    // Starts and completions carry the same immutable boundary source.
    scan::<Transition>(path, &["history", "transitions"], |_, t| {
        if t.commitment.source == Some(boundary) && t.commitment.participants.contains(&actor) {
            transitions.push(t);
        }
    })?;
    let through = transitions
        .iter()
        .map(|t| t.tick.max(t.commitment.deadline))
        .max()
        .unwrap_or(tick);
    let base = [
        "history",
        "final_state",
        "base",
        "base",
        "base",
        "base",
        "base",
    ];
    let mut keys = base.to_vec();
    keys.push("events");
    let mut events = Vec::new();
    scan::<Event>(path, &keys, |_, e| {
        if (tick..=through).contains(&e.tick) && e.participants.contains(&actor) {
            events.push(e);
        }
    })?;
    keys = base.to_vec();
    keys.extend(["cognition", "consumption"]);
    let mut meals = Vec::new();
    scan::<Meal>(path, &keys, |_, m| {
        if (tick..=through).contains(&m.tick) && m.actor == actor {
            meals.push(m);
        }
    })?;
    Ok(Detail {
        choice,
        transitions,
        events,
        meals,
    })
}
