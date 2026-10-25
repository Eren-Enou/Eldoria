//! Append-only completed prefixes and immutable checkpoint generations.
use super::*;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Component, Path, PathBuf},
};

#[path = "reliability.rs"]
pub mod reliability;

/// Explicit, build-guarded v1 read compatibility; never accepts arbitrary identities.
pub fn accepts_runtime(identity: &str) -> bool {
    identity == IDENTITY
        || include_str!(concat!(env!("OUT_DIR"), "/compatible_v1.txt"))
            .lines()
            .any(|old| old == identity)
}
fn decode_commit(bytes: &[u8]) -> Result<Commit, String> {
    decode(crate::storage_codec::decode(bytes)?.as_ref())
}

pub fn check_path(path: &Path) -> Result<(), String> {
    if !path.is_absolute()
        || path.components().any(|c| match c {
            Component::ParentDir => true,
            Component::Normal(n) => ["experiments", "reinforcement", ".git"]
                .iter()
                .any(|s| n.to_string_lossy().eq_ignore_ascii_case(s)),
            _ => false,
        })
    {
        return Err("session paths must be absolute and outside scientific archives".into());
    }
    for parent in path.ancestors() {
        if let Ok(metadata) = fs::symlink_metadata(parent) {
            if metadata.file_type().is_symlink() {
                return Err("session links forbidden".into());
            }
            #[cfg(windows)]
            {
                use std::os::windows::fs::MetadataExt;
                if metadata.file_attributes() & 0x400 != 0 {
                    return Err("session reparse points forbidden".into());
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod binding_tests {
    use super::*;
    #[test]
    fn commit_metadata_and_journal_corruption_are_rejected() {
        let directory = std::env::temp_dir().join(format!(
            "woi-binding-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut config = c::matrix()
            .into_iter()
            .find(|c| c.case == "paired" && c.mode == c::Mode::Continuous)
            .unwrap();
        config.horizon = 16;
        let session = Session::create(&directory, config).unwrap();
        let checkpoint = directory.join("checkpoint-00000001.cbor");
        let original = fs::read(&checkpoint).unwrap();
        drop(session);
        let mut commit: Commit = decode(&original).unwrap();
        commit.tick += 1;
        fs::write(&checkpoint, encode(&commit).unwrap()).unwrap();
        assert!(
            Session::open(&directory)
                .err()
                .unwrap()
                .contains("binding checksum")
        );
        fs::write(&checkpoint, &original).unwrap();
        let opened = Session::open(&directory).unwrap();
        drop(opened);
        let journal = directory.join("journal.jsonl");
        let mut bytes = fs::read(&journal).unwrap();
        let index = bytes.len() / 2;
        bytes[index] ^= 1;
        fs::write(&journal, bytes).unwrap();
        assert!(
            Session::open(&directory)
                .err()
                .unwrap()
                .contains("journal/checkpoint mismatch")
        );
    }
}

fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    file.write_all(bytes)
        .and_then(|_| file.sync_all())
        .map_err(|e| e.to_string())
}
#[derive(Serialize, Deserialize)]
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
impl Commit {
    fn binding(&self) -> Result<String, String> {
        serde_json::to_vec(&(
            &self.schema,
            &self.world,
            &self.runtime,
            self.generation,
            self.tick,
            self.journal_bytes,
            &self.journal_sha256,
            &self.state_sha256,
        ))
        .map(|bytes| hash(&bytes))
        .map_err(|e| e.to_string())
    }
}
pub struct Session {
    _writer_lock: File,
    world: World,
    path: PathBuf,
    id: String,
    generation: u64,
    journal_bytes: Vec<u8>,
    dirty: bool,
    failed: bool,
}
// Borrow the already-owned audit records; a step needs lengths, not two additional
// complete world snapshots. This changes transport cost, never actor state.
struct JournalView<'a> {
    runtime: &'a Runtime,
    state: &'a State,
    cognition: &'a Cognition,
    inquiry: &'a q::Inquiry,
    assessment: &'a crate::assessment::Assessment,
    concerns: &'a Concerns,
    boundaries: &'a [x::Boundary],
    valuations: &'a [v::Record],
    provenance: &'a crate::audit::RuntimeProvenance,
    contexts: &'a crate::audit::Contexts,
    foresight: &'a Foresight,
    intentional: &'a Intentional,
    retention: &'a crate::retention::Retention,
    learning: &'a crate::self_evaluation::Learning,
}
impl<'a> JournalView<'a> {
    fn new(world: &'a World) -> Self {
        let data = &world.sim.world;
        Self {
            runtime: data.resource::<Runtime>(),
            state: data.resource::<State>(),
            cognition: data.resource::<Cognition>(),
            inquiry: data.resource::<q::Inquiry>(),
            assessment: data.resource::<crate::assessment::Assessment>(),
            concerns: data.resource::<Concerns>(),
            boundaries: &world.boundaries,
            valuations: &world.valuations,
            provenance: data.resource::<crate::audit::RuntimeProvenance>(),
            contexts: data.resource::<crate::audit::Contexts>(),
            foresight: data.resource::<Foresight>(),
            intentional: data.resource::<Intentional>(),
            retention: data.resource::<crate::retention::Retention>(),
            learning: data.resource::<crate::self_evaluation::Learning>(),
        }
    }
}
struct Cuts {
    tick: u64,
    events: usize,
    transitions: usize,
    admissions: usize,
    needs: usize,
    grants: usize,
    memories: usize,
    meals: usize,
    information: usize,
    queries: usize,
    assessment: usize,
    concerns: usize,
    valuations: usize,
    roots: usize,
    receipts: usize,
    windows: usize,
    exchanges: usize,
    provenance_queries: usize,
    provenance_evictions: usize,
    contexts: usize,
    forecasts: usize,
    errors: usize,
    readings: usize,
    talk: usize,
    conversations: usize,
    retention: usize,
    learning: usize,
}
impl Cuts {
    fn new(world: &World) -> Self {
        let s = JournalView::new(world);
        Self {
            tick: world.tick(),
            events: s.runtime.events.len(),
            transitions: s.state.transitions.len(),
            admissions: s.state.admissions.len(),
            needs: s.state.needs.len(),
            grants: s.state.grants.len(),
            memories: s.state.memories.len(),
            meals: s.cognition.consumption.len(),
            information: s.cognition.information.len(),
            queries: s.inquiry.records.len(),
            assessment: s.assessment.records.len(),
            concerns: s.concerns.transitions.len(),
            valuations: s.valuations.len(),
            roots: s.provenance.roots.len(),
            receipts: s.provenance.receipts.len(),
            windows: s.provenance.windows.len(),
            exchanges: s.provenance.exchanges.len(),
            provenance_queries: s.provenance.queries.len(),
            provenance_evictions: s.provenance.evictions.len(),
            contexts: s.contexts.0.values().len(),
            forecasts: s.foresight.forecasts.len(),
            errors: s.foresight.errors.len(),
            readings: s.foresight.readings.len(),
            talk: s.intentional.records.len(),
            conversations: s.intentional.conversations.len(),
            retention: s.retention.records.len(),
            learning: s.learning.records.len(),
        }
    }
}
impl Session {
    fn writer_lock(path: &Path) -> Result<File, String> {
        let path = path.join("session.lock");
        check_path(&path)?;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(|e| e.to_string())?;
        file.try_lock()
            .map_err(|e| format!("world is already open for writing or cannot be locked: {e}"))?;
        Ok(file)
    }
    pub fn world(&self) -> &World {
        &self.world
    }
    pub fn create(path: &Path, config: c::Config) -> Result<Self, String> {
        Self::create_world(path, World::new(config)?)
    }
    pub fn create_valuation(path: &Path, config: v::Config) -> Result<Self, String> {
        Self::create_world(path, World::new_valuation(config)?)
    }
    fn create_world(path: &Path, world: World) -> Result<Self, String> {
        check_path(path)?;
        let id = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or("invalid world directory")?
            .to_owned();
        if id.is_empty()
            || id.len() > 96
            || !id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return Err("world ID requires bounded ASCII letters/digits/hyphens".into());
        }
        fs::create_dir(path).map_err(|e| e.to_string())?;
        let writer_lock = Self::writer_lock(path)?;
        let initial = JournalView::new(&world);
        let mut journal_bytes=serde_json::to_vec(&serde_json::json!({"schema":"woi-world-journal/v1","world":id,"runtime":IDENTITY,"start":world.start,"config":world.config,"valuation":world.valuation,"bootstrap_events":world.sim.events(),"bootstrap_audit":{"cognition":initial.cognition,"foresight":initial.foresight,"intentional":initial.intentional,"inquiry":initial.inquiry,"provenance":initial.provenance,"contexts":initial.contexts,"concerns":initial.concerns,"assessment":initial.assessment,"retention":initial.retention,"learning":initial.learning},"initial":world.initial,"state_sha256":hash(&world.checkpoint()?)})).map_err(|e|e.to_string())?;
        journal_bytes.push(b'\n');
        write_new(&path.join("journal.jsonl"), &journal_bytes)?;
        let mut result = Self {
            _writer_lock: writer_lock,
            world,
            path: path.to_owned(),
            id,
            generation: 0,
            journal_bytes,
            dirty: true,
            failed: false,
        };
        result.save()?;
        Ok(result)
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    /// Completed observer record only. Never used for restoration or policy input.
    pub fn record(&self, boundary: usize) -> Result<serde_json::Value, String> {
        if boundary >= self.world.boundaries.len() {
            return Err("no executed record at this boundary".into());
        }
        let bytes = self
            .journal_bytes
            .split(|b| *b == b'\n')
            .nth(boundary + 1)
            .ok_or("missing journal record")?;
        serde_json::from_slice(bytes).map_err(|e| e.to_string())
    }
    pub fn advance(&mut self, steps: usize) -> Result<(), String> {
        if self.failed {
            return Err("session has a failed journal write; preserve diagnostics and reopen last saved generation".into());
        }
        if steps > 2048 {
            return Err("bounded advance requires at most2048 steps".into());
        }
        for _ in 0..steps {
            if self.world.complete() {
                break;
            }
            let prior = Cuts::new(&self.world);
            if let Err(error) = self.world.step() {
                self.failed = true;
                return Err(format!(
                    "execution failed before a complete journal boundary: {error}"
                ));
            }
            self.dirty = true;
            let saved = JournalView::new(&self.world);
            let mut bytes=serde_json::to_vec(&serde_json::json!({
                "schema":"woi-world-step/v1","world":self.id,"sequence":saved.boundaries.len()-1,
                "from":prior.tick,"tick":saved.runtime.tick,"boundary":saved.boundaries.last(),
                "events":&saved.runtime.events[prior.events..],
                "transitions":&saved.state.transitions[prior.transitions..],
                "admissions":&saved.state.admissions[prior.admissions..],
                "needs":&saved.state.needs[prior.needs..],
                "grants":&saved.state.grants[prior.grants..],
                "environmental_memory":&saved.state.memories[prior.memories..],
                "meals":&saved.cognition.consumption[prior.meals..],
                "information":&saved.cognition.information[prior.information..],
                "queries":&saved.inquiry.records[prior.queries..],
                "assessment":&saved.assessment.records[prior.assessment..],
                "valuations":&saved.valuations[prior.valuations..],
                "concerns":&saved.concerns.transitions[prior.concerns..],
                "roots":&saved.provenance.roots[prior.roots..],"receipts":&saved.provenance.receipts[prior.receipts..],
                "windows":&saved.provenance.windows[prior.windows..],"exchanges":&saved.provenance.exchanges[prior.exchanges..],
                "provenance_queries":&saved.provenance.queries[prior.provenance_queries..],"provenance_evictions":&saved.provenance.evictions[prior.provenance_evictions..],
                "contexts":&saved.contexts.0.values()[prior.contexts..],"forecasts":&saved.foresight.forecasts[prior.forecasts..],
                "forecast_errors":&saved.foresight.errors[prior.errors..],"readings":&saved.foresight.readings[prior.readings..],
                "talk":&saved.intentional.records[prior.talk..],"conversations":&saved.intentional.conversations[prior.conversations..],
                "retention":&saved.retention.records[prior.retention..],"learning":&saved.learning.records[prior.learning..],
            })).map_err(|e|e.to_string())?;
            bytes.push(b'\n');
            let append = (|| {
                check_path(&self.path.join("journal.jsonl"))?;
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(self.path.join("journal.jsonl"))
                    .map_err(|e| e.to_string())?;
                file.write_all(&bytes)
                    .and_then(|_| file.sync_data())
                    .map_err(|e| e.to_string())
            })();
            if let Err(error) = append {
                self.failed = true;
                return Err(format!("world advanced but journal write failed: {error}"));
            }
            self.journal_bytes.extend(bytes);
        }
        Ok(())
    }
    pub fn save(&mut self) -> Result<(), String> {
        self.save_with_codec(crate::storage_codec::Codec::Raw)
    }
    pub fn save_with_codec(&mut self, codec: crate::storage_codec::Codec) -> Result<(), String> {
        check_path(&self.path)?;
        if self.failed {
            return Err("cannot save after journal failure".into());
        }
        if !self.dirty {
            return Ok(());
        }
        let state = self.world.checkpoint()?;
        let next = self
            .generation
            .checked_add(1)
            .ok_or("checkpoint generation exhausted")?;
        let generation = (next..=next.saturating_add(999))
            .take(1000)
            .find(|generation| {
                !self
                    .path
                    .join(format!("checkpoint-{generation:08}.pending"))
                    .exists()
                    && !self
                        .path
                        .join(format!("checkpoint-{generation:08}.cbor"))
                        .exists()
            })
            .ok_or("no unused checkpoint generation within retry bound")?;
        let mut commit = Commit {
            binding_sha256: String::new(),
            schema: "woi-world-commit/v1".into(),
            world: self.id.clone(),
            runtime: IDENTITY.into(),
            generation,
            tick: self.world.tick(),
            journal_bytes: self.journal_bytes.len() as u64,
            journal_sha256: hash(&self.journal_bytes),
            state_sha256: hash(&state),
            state,
        };
        commit.binding_sha256 = commit.binding()?;
        let bytes = crate::storage_codec::encode(&encode(&commit)?, codec)?;
        let temp = self
            .path
            .join(format!("checkpoint-{generation:08}.pending"));
        let destination = self.path.join(format!("checkpoint-{generation:08}.cbor"));
        check_path(&temp)?;
        check_path(&destination)?;
        if destination.exists() {
            return Err("checkpoint generation already published".into());
        }
        write_new(&temp, &bytes)?;
        let persisted = fs::read(&temp).map_err(|e| e.to_string())?;
        let verified: Commit = decode_commit(&persisted)?;
        World::restore(&verified.state)?;
        if bytes != persisted {
            return Err("checkpoint readback mismatch".into());
        }
        fs::rename(&temp, &destination).map_err(|e| e.to_string())?;
        self.generation = generation;
        self.dirty = false;
        Ok(())
    }
    pub fn open(path: &Path) -> Result<Self, String> {
        check_path(path)?;
        let id = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("invalid world directory ID")?;
        if id.is_empty()
            || id.len() > 96
            || !id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return Err("world ID requires bounded ASCII letters/digits/hyphens".into());
        }
        let writer_lock = Self::writer_lock(path)?;
        let (commit, journal_bytes, world) = reliability::load(path, None, false)?;
        Ok(Self {
            _writer_lock: writer_lock,
            world,
            path: path.to_owned(),
            id: commit.world,
            generation: commit.generation,
            journal_bytes,
            dirty: false,
            failed: false,
        })
    }
    pub fn finalize(&mut self) -> Result<(), String> {
        if !self.world.complete() {
            return Err("finalization requires actual terminal execution".into());
        }
        self.save()?;
        let mut bytes = self.world.export()?;
        bytes.push(b'\n');
        let destination = self.path.join("history.jsonl");
        check_path(&destination)?;
        if destination.exists() {
            if fs::read(destination).map_err(|e| e.to_string())? == bytes {
                return Ok(());
            }
            return Err("existing terminal export mismatch".into());
        }
        write_new(&destination, &bytes)
    }
}
