//! External human annotations. Nothing here is character state or simulation input.
use eldoria_settlement::{Settlement, model::RUNTIME, storage};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::PathBuf};

const SCHEMA: &str = "eldoria-observer-notebook/v1";
pub const MAX_BOOKMARKS: usize = 128;
pub const MAX_REVISIONS: usize = 512;
pub const MAX_BODY: usize = 4000;
const MAX_FILE: u64 = 32768;
pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn world_key(world: &Settlement) -> String {
    digest(
        &serde_json::to_vec(&(
            RUNTIME,
            &world.state().config,
            world.characters().keys().collect::<Vec<_>>(),
            world.state().initial_food,
        ))
        .expect("serializable world identity"),
    )
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Anchor {
    pub world: String,
    pub record: u64,
    pub tick: u64,
    pub prefix: String,
}
impl Anchor {
    pub fn key(&self) -> String {
        digest(&serde_json::to_vec(self).expect("serializable anchor"))
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bookmark {
    pub anchor: Anchor,
    pub revision: u32,
    pub title: String,
    pub body: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    schema: String,
    checksum: String,
    bookmark: Bookmark,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Availability {
    Available,
    BeyondPrefix,
    DifferentHistory,
}
#[derive(Serialize)]
pub struct Summary {
    pub key: String,
    pub record: u64,
    pub tick: u64,
    pub revision: u32,
    pub title: String,
    pub availability: Availability,
}
#[derive(Serialize)]
pub struct NotebookView {
    pub world: String,
    pub entries: Vec<Summary>,
    pub selected: Option<Bookmark>,
    pub error: Option<String>,
}
pub struct Store {
    root: PathBuf,
}
impl Store {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
    pub fn load(&self, world: &str) -> Result<Vec<Bookmark>, String> {
        if !hex_key(world) {
            return Err("Invalid notebook world identity".into());
        }
        let dir = self.root.join(world);
        if !dir.exists() {
            return Ok(vec![]);
        }
        let mut latest = BTreeMap::<String, Bookmark>::new();
        let mut files = Vec::new();
        for (count, item) in fs::read_dir(&dir).map_err(|e| e.to_string())?.enumerate() {
            if count >= MAX_REVISIONS * 2 {
                return Err("Notebook directory entry limit reached".into());
            }
            let path = item.map_err(|e| e.to_string())?.path();
            if path.extension().is_some_and(|e| e == "json") {
                files.push(path);
            }
        }
        if files.len() > MAX_REVISIONS {
            return Err("Notebook revision limit reached".into());
        }
        files.sort();
        for path in files {
            let mut bytes = vec![];
            fs::File::open(&path)
                .map_err(|e| e.to_string())?
                .take(MAX_FILE + 1)
                .read_to_end(&mut bytes)
                .map_err(|e| e.to_string())?;
            if bytes.len() as u64 > MAX_FILE {
                return Err("Notebook revision exceeds file limit".into());
            }
            let envelope: Envelope = serde_json::from_slice(&bytes).map_err(|e| {
                format!(
                    "Notebook revision invalid ({}), line {}, column {}",
                    path.display(),
                    e.line(),
                    e.column()
                )
            })?;
            let note = envelope.bookmark;
            validate(&note)?;
            let key = note.anchor.key();
            let expected = format!("{key}-r{:04}.json", note.revision);
            if envelope.schema != SCHEMA
                || note.anchor.world != world
                || path
                    .file_name()
                    .is_none_or(|name| name != expected.as_str())
                || envelope.checksum
                    != digest(&serde_json::to_vec(&note).map_err(|e| e.to_string())?)
            {
                return Err(format!(
                    "Notebook revision identity/checksum mismatch: {}",
                    path.display()
                ));
            }
            if latest
                .get(&key)
                .is_none_or(|old| old.revision < note.revision)
            {
                latest.insert(key, note);
            }
        }
        if latest.len() > MAX_BOOKMARKS {
            return Err("Notebook bookmark limit reached".into());
        }
        Ok(latest.into_values().collect())
    }
    pub fn save(
        &self,
        anchor: Anchor,
        expected_revision: u32,
        title: String,
        body: String,
    ) -> Result<Bookmark, String> {
        let entries = self.load(&anchor.world)?;
        let key = anchor.key();
        let previous = entries.iter().find(|b| b.anchor.key() == key);
        if previous.map_or(0, |old| old.revision) != expected_revision {
            return Err(
                "Notebook changed since this draft was opened; reload and review before saving"
                    .into(),
            );
        }
        if let Some(old) = previous
            && old.title == title
            && old.body == body
        {
            return Ok(old.clone());
        }
        if previous.is_none() && entries.len() >= MAX_BOOKMARKS {
            return Err("Notebook bookmark limit reached".into());
        }
        let revision = previous.map_or(1, |old| old.revision + 1);
        let note = Bookmark {
            anchor,
            revision,
            title,
            body,
        };
        validate(&note)?;
        let dir = self.root.join(&note.anchor.world);
        let path = dir.join(format!("{key}-r{revision:04}.json"));
        storage::check_output(&path)?;
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        // Publication rejects same-revision writer races without replacing files.
        let mut count = 0;
        for (seen, entry) in fs::read_dir(&dir).map_err(|e| e.to_string())?.enumerate() {
            if seen >= MAX_REVISIONS * 2 {
                return Err("Notebook directory entry limit reached".into());
            }
            if entry
                .map_err(|e| e.to_string())?
                .path()
                .extension()
                .is_some_and(|x| x == "json")
            {
                count += 1;
            }
        }
        if count >= MAX_REVISIONS {
            return Err("Notebook revision limit reached; existing files retained".into());
        }
        let envelope = Envelope {
            schema: SCHEMA.into(),
            checksum: digest(&serde_json::to_vec(&note).map_err(|e| e.to_string())?),
            bookmark: note.clone(),
        };
        storage::write_new(
            &path,
            &serde_json::to_vec(&envelope).map_err(|e| e.to_string())?,
        )?;
        Ok(note)
    }
}
fn hex_key(key: &str) -> bool {
    key.len() == 64 && key.bytes().all(|c| c.is_ascii_hexdigit())
}
fn validate(note: &Bookmark) -> Result<(), String> {
    if !hex_key(&note.anchor.world)
        || !hex_key(&note.anchor.prefix)
        || note.revision == 0
        || note.revision as usize > MAX_REVISIONS
    {
        return Err("Invalid notebook anchor or revision".into());
    }
    if note.title.trim().is_empty() || note.title.len() > 120 || note.body.len() > MAX_BODY {
        return Err(
            "Bookmark needs a title (at most 120 UTF-8 bytes) and note at most 4000 bytes".into(),
        );
    }
    Ok(())
}
