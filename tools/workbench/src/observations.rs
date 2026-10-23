//! Human annotations are external observations, never simulation inputs.
use crate::{
    Result, execution,
    storage::{self, Store},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    Plausible,
    Questionable,
    #[default]
    Unclear,
    PossibleBug,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub individual: u32,
    pub tick: u64,
    pub category: Category,
    pub local_context: String,
    pub selected_activity: String,
    pub recorded_outcome: String,
    pub notes: String,
    #[serde(default)]
    pub suggested_improvement: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Saved {
    schema: String,
    run: String,
    source_revision: String,
    history_sha256: String,
    scope: String,
    pub observation: Observation,
}
pub fn load(store: &Store, id: &str) -> Result<Vec<Saved>> {
    let m = execution::metadata(store, id)?;
    let directory = store.run(id)?.join("observations");
    storage::no_links(&directory)?;
    if !directory.exists() {
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    for entry in std::fs::read_dir(directory).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().is_some_and(|e| e == "json") {
            files.push(path);
        }
        if files.len() > 4096 {
            return Err("Saved observation count exceeds4096".into());
        }
    }
    files.sort();
    files
        .into_iter()
        .map(|path| {
            let saved: Saved = storage::read(&path)?;
            if saved.schema != "woi-human-observation/v1"
                || saved.run != id
                || Some(&saved.history_sha256) != m.history_sha256.as_ref()
            {
                return Err("Saved observation belongs to a different run/history".into());
            }
            Ok(saved)
        })
        .collect()
}
pub fn save(store: &Store, id: &str, note: &Observation) -> Result<PathBuf> {
    let summary = execution::inspect(store, id)?;
    if note.individual > 3 || !(summary.start..=summary.end).contains(&note.tick) {
        return Err(
            "Observation needs an actual individual0–3 and a tick inside this recorded run".into(),
        );
    }
    if note.notes.trim().is_empty()
        || [
            &note.notes,
            &note.local_context,
            &note.selected_activity,
            &note.recorded_outcome,
            &note.suggested_improvement,
        ]
        .iter()
        .any(|s| s.len() > 16384)
    {
        return Err("Add human notes; each field is limited to16KiB".into());
    }
    let m = execution::metadata(store, id)?;
    let saved = Saved {
        schema: "woi-human-observation/v1".into(),
        run: id.into(),
        source_revision: m.source.revision,
        history_sha256: m.history_sha256.ok_or("Missing history identity")?,
        scope: "Human interpretation; not actor cognition or registered scientific evidence".into(),
        observation: note.clone(),
    };
    let directory = store.run(id)?.join("observations");
    storage::no_links(&directory)?;
    std::fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
    for index in 0..1000 {
        let path = directory.join(format!(
            "note-{}-{}-{index}.json",
            storage::now(),
            std::process::id()
        ));
        match storage::write_new(&path, &saved) {
            Ok(()) => return Ok(path),
            Err(e) if path.exists() => {
                let _ = e;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    Err("Could not reserve observation file".into())
}
