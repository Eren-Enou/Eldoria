//! External storage inspection and explicit branches. Journal tails never become cognition.
use super::*;
use std::io::Read;

const JOURNAL_LIMIT: u64 = 256 * 1024 * 1024;
const COMMIT_LIMIT: u64 = 160 * 1024 * 1024;

#[derive(Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Budget {
    pub bytes: Option<u64>,
    pub generations: Option<usize>,
    pub projected_saves: u64,
}
#[derive(Serialize)]
pub struct Generation {
    pub file: String,
    pub generation: Option<u64>,
    pub bytes: u64,
    pub valid: bool,
    pub tick: Option<u64>,
    pub committed_journal_bytes: Option<u64>,
    pub runtime: Option<String>,
    pub error: Option<String>,
}
#[derive(Serialize)]
pub struct Inspection {
    pub schema: &'static str,
    pub world: String,
    pub path: PathBuf,
    pub lock_status: String,
    pub stable_snapshot: bool,
    pub generations: Vec<Generation>,
    pub pending: Vec<String>,
    pub latest_committed: Option<u64>,
    pub latest_valid: Option<u64>,
    pub oldest_valid: Option<u64>,
    pub journal_bytes: Option<u64>,
    pub total_retained_bytes: u64,
    pub local_recovery_branches: usize,
    pub additional_save_estimate_bytes: Option<u64>,
    pub warnings: Vec<String>,
}

fn generation(path: &Path) -> Option<u64> {
    let name = path.file_name()?.to_str()?;
    let n: u64 = name
        .strip_prefix("checkpoint-")?
        .strip_suffix(".cbor")?
        .parse()
        .ok()?;
    (name == format!("checkpoint-{n:08}.cbor") && n != 0).then_some(n)
}
fn bounded_read(path: &Path, limit: u64) -> Result<Vec<u8>, String> {
    check_path(path)?;
    let file = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if file.metadata().map_err(|e| e.to_string())?.len() > limit {
        return Err(format!("{} exceeds load limit{limit}", path.display()));
    }
    let mut bytes = vec![];
    file.take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > limit {
        return Err("file grew beyond load limit".into());
    }
    Ok(bytes)
}

// No create/truncate: inspection must leave even malformed directories unchanged.
fn existing_lock(path: &Path) -> Result<File, String> {
    let lock = path.join("session.lock");
    check_path(&lock)?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&lock)
        .map_err(|e| format!("existing session lock unavailable: {e}"))?;
    file.try_lock()
        .map_err(|e| format!("session writer owns lock or lock unavailable: {e}"))?;
    Ok(file)
}

/// Loads one explicitly selected generation, or the newest numeric generation.
/// No fallback. Tail tolerance is exclusively for read-only checks / recovery.
pub(super) fn load(
    path: &Path,
    selected: Option<u64>,
    allow_tail: bool,
) -> Result<(Commit, Vec<u8>, World), String> {
    check_path(path)?;
    let number = match selected {
        Some(n) => n,
        None => {
            let files = fs::read_dir(path)
                .map_err(|e| e.to_string())?
                .map(|e| e.map(|e| e.path()).map_err(|e| e.to_string()))
                .collect::<Result<Vec<_>, _>>()?;
            if files.iter().any(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("checkpoint-") && n.ends_with(".cbor"))
                    && generation(p).is_none()
            }) {
                return Err(
                    "ambiguous checkpoint filename; inspect and explicitly select a generation"
                        .into(),
                );
            }
            files
                .iter()
                .filter_map(|p| generation(p))
                .max()
                .ok_or("no committed checkpoint; pending files are not saves")?
        }
    };
    let checkpoint = path.join(format!("checkpoint-{number:08}.cbor"));
    let commit: Commit = decode(&bounded_read(&checkpoint, COMMIT_LIMIT)?)?;
    if commit.binding_sha256 != commit.binding()? {
        return Err("checkpoint/journal binding checksum mismatch".into());
    }
    let id = path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("invalid world ID")?;
    if commit.schema != "woi-world-commit/v1"
        || !accepts_runtime(&commit.runtime)
        || commit.world != id
        || commit.generation != number
        || commit.state_sha256 != hash(&commit.state)
    {
        return Err("commit identity/state mismatch".into());
    }
    if commit.journal_bytes > JOURNAL_LIMIT {
        return Err("journal exceeds256MiB load limit".into());
    }
    let journal_path = path.join("journal.jsonl");
    check_path(&journal_path)?;
    let mut file = File::open(&journal_path).map_err(|e| e.to_string())?;
    let actual = file.metadata().map_err(|e| e.to_string())?.len();
    if actual < commit.journal_bytes || (!allow_tail && actual != commit.journal_bytes) {
        return Err("journal missing/truncated or contains unsaved tail; preserve evidence".into());
    }
    let mut journal = vec![0; commit.journal_bytes as usize];
    file.read_exact(&mut journal).map_err(|e| e.to_string())?;
    if hash(&journal) != commit.journal_sha256 {
        return Err("journal/checkpoint mismatch".into());
    }
    if !journal.ends_with(b"\n") {
        return Err("committed journal is not a complete prefix".into());
    }
    let world = World::restore(&commit.state)?;
    if world.tick() != commit.tick {
        return Err("commit clock mismatch".into());
    }
    let records = journal
        .split(|b| *b == b'\n')
        .filter(|r| !r.is_empty())
        .map(serde_json::from_slice::<serde_json::Value>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    let header = records.first().ok_or("missing journal header")?;
    if records.len() != world.boundaries.len() + 1
        || header["world"] != id
        || !header["runtime"].as_str().is_some_and(accepts_runtime)
        || header["schema"] != "woi-world-journal/v1"
        || header["config"] != serde_json::to_value(&world.config).map_err(|e| e.to_string())?
        || header["valuation"]
            != serde_json::to_value(&world.valuation).map_err(|e| e.to_string())?
        || (world.boundaries.is_empty() && header["state_sha256"] != commit.state_sha256)
    {
        return Err("journal sequence/state/configuration binding mismatch".into());
    }
    for (index, record) in records.iter().skip(1).enumerate() {
        if record["schema"] != "woi-world-step/v1"
            || record["world"] != id
            || record["sequence"] != index
            || record["boundary"]
                != serde_json::to_value(&world.boundaries[index]).map_err(|e| e.to_string())?
        {
            return Err("journal boundary identity mismatch".into());
        }
    }
    Ok((commit, journal, world))
}

fn retained_size(path: &Path, depth: usize) -> Result<u64, String> {
    if depth > 8 {
        return Err("storage inventory nesting exceeds8".into());
    }
    check_path(path)?;
    let meta = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if meta.is_file() {
        return Ok(meta.len());
    }
    let mut sum = 0u64;
    for e in fs::read_dir(path).map_err(|e| e.to_string())? {
        let e = e.map_err(|e| e.to_string())?;
        sum = sum
            .checked_add(retained_size(&e.path(), depth + 1)?)
            .ok_or("storage byte count overflow")?;
    }
    Ok(sum)
}

/// Read-only. A busy/absent lock makes the observed snapshot explicitly unstable.
pub fn inspect(path: &Path, budget: &Budget) -> Result<Inspection, String> {
    check_path(path)?;
    let lock = existing_lock(path);
    let stable = lock.is_ok();
    let lock_status = lock.as_ref().map_or_else(
        |e| e.clone(),
        |_| "available; exclusive inspection guard held".into(),
    );
    let mut paths = fs::read_dir(path)
        .map_err(|e| e.to_string())?
        .map(|e| e.map(|e| e.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    if paths.len() > 4096 {
        return Err("inventory exceeds4096 entries; select a smaller world directory".into());
    }
    paths.sort();
    let mut generations = vec![];
    let mut pending = vec![];
    let mut local_recovery_branches = usize::from(path.join("recovery.json").is_file());
    for file in paths {
        check_path(&file)?;
        let name = file.file_name().unwrap().to_string_lossy().to_string();
        if name.starts_with("checkpoint-") && name.ends_with(".pending") {
            pending.push(name);
            continue;
        }
        if file.is_dir() && file.join("recovery.json").is_file() {
            local_recovery_branches += 1;
        }
        if !name.starts_with("checkpoint-") || !name.ends_with(".cbor") {
            continue;
        }
        let n = generation(&file);
        let bytes = fs::metadata(&file).map_err(|e| e.to_string())?.len();
        let loaded = if stable {
            n.ok_or_else(|| "noncanonical generation filename".into())
                .and_then(|n| load(path, Some(n), true))
        } else {
            Err("writer/lock unavailable; verification not asserted".into())
        };
        let (tick, prefix, runtime, error) = match loaded {
            Ok((commit, _, _)) => (
                Some(commit.tick),
                Some(commit.journal_bytes),
                Some(commit.runtime),
                None,
            ),
            Err(error) => (None, None, None, Some(error)),
        };
        generations.push(Generation {
            file: name,
            generation: n,
            bytes,
            valid: error.is_none(),
            tick,
            committed_journal_bytes: prefix,
            runtime,
            error,
        });
    }
    generations.sort_by_key(|g| g.generation);
    let latest_committed = generations.iter().filter_map(|g| g.generation).max();
    let latest_valid = generations
        .iter()
        .filter(|g| g.valid)
        .filter_map(|g| g.generation)
        .max();
    let oldest_valid = generations
        .iter()
        .filter(|g| g.valid)
        .filter_map(|g| g.generation)
        .min();
    let estimate = generations
        .iter()
        .find(|g| g.generation == latest_valid && g.valid)
        .and_then(|g| g.bytes.checked_mul(budget.projected_saves));
    let total = retained_size(path, 0)?;
    let mut warnings = vec![];
    if budget.bytes.is_some_and(|b| total > b) {
        warnings
            .push("retained bytes exceed configured storage budget; simulation unchanged".into());
    }
    if budget.generations.is_some_and(|n| generations.len() > n) {
        warnings.push("checkpoint count exceeds configured generation warning".into());
    }
    if let (Some(projected), Some(limit)) = (estimate, budget.bytes)
        && total.checked_add(projected).is_none_or(|n| n > limit)
    {
        warnings.push(
            "projected unchanged-size saves exceed storage budget; future growth is unknown".into(),
        );
    }
    if latest_committed != latest_valid {
        warnings.push(
            "latest generation is not verified; explicit selection required, no fallback".into(),
        );
    }
    if !pending.is_empty() {
        warnings.push("pending files are unpublished diagnostics, not saves".into());
    }
    let journal_bytes = fs::metadata(path.join("journal.jsonl"))
        .ok()
        .map(|m| m.len());
    if let Some(g) = generations
        .iter()
        .find(|g| g.generation == latest_valid && g.valid)
        && journal_bytes > g.committed_journal_bytes
    {
        warnings.push("uncommitted tail exists; only a new explicit branch may resume the selected committed state".into());
    }
    Ok(Inspection {
        schema: "woi-world-inspection/v1",
        world: path.file_name().unwrap().to_string_lossy().into(),
        path: path.into(),
        lock_status,
        stable_snapshot: stable,
        generations,
        pending,
        latest_committed,
        latest_valid,
        oldest_valid,
        journal_bytes,
        total_retained_bytes: total,
        local_recovery_branches,
        additional_save_estimate_bytes: estimate,
        warnings,
    })
}

#[derive(Debug, Serialize)]
pub struct Recovery {
    pub schema: &'static str,
    pub branch: PathBuf,
    pub parent: PathBuf,
    pub world: String,
    pub generation: u64,
    pub tick: u64,
    pub parent_checkpoint_sha256: String,
    pub state_sha256: String,
    pub prefix_bytes: u64,
    pub prefix_sha256: String,
    pub tail_bytes: u64,
    pub tail_sha256: String,
    pub original_journal_sha256: String,
    pub divergence: &'static str,
}

/// Explicit branch, never modifies parent bytes or infers live state from its tail.
pub fn recover(source: &Path, destination: &Path, selected: u64) -> Result<Recovery, String> {
    check_path(source)?;
    check_path(destination)?;
    if destination.exists() {
        return Err("recovery destination already exists; choose a fresh branch".into());
    }
    if source.file_name() != destination.file_name() {
        return Err(
            "destination basename must preserve world ID; use a fresh branch root/WORLD-ID".into(),
        );
    }
    let canonical_source = source.canonicalize().map_err(|e| e.to_string())?;
    let parent = destination
        .parent()
        .ok_or("branch needs an existing parent directory")?
        .canonicalize()
        .map_err(|e| e.to_string())?;
    if parent
        .ancestors()
        .any(|ancestor| ancestor == canonical_source)
    {
        return Err("recovery branch must be outside the original world directory".into());
    }
    let _source_lock = existing_lock(source)?;
    let (commit, prefix, world) = load(source, Some(selected), true)?;
    let checkpoint = bounded_read(
        &source.join(format!("checkpoint-{selected:08}.cbor")),
        COMMIT_LIMIT,
    )?;
    let journal = bounded_read(&source.join("journal.jsonl"), JOURNAL_LIMIT)?;
    if !journal.starts_with(&prefix) {
        return Err("source journal changed during recovery; no branch published".into());
    }
    let tail = &journal[prefix.len()..];
    let result = Recovery {
        schema: "woi-world-recovery/v1",
        branch: destination.into(),
        parent: source.into(),
        world: commit.world.clone(),
        generation: selected,
        tick: world.tick(),
        parent_checkpoint_sha256: hash(&checkpoint),
        state_sha256: commit.state_sha256,
        prefix_bytes: prefix.len() as u64,
        prefix_sha256: hash(&prefix),
        tail_bytes: tail.len() as u64,
        tail_sha256: hash(tail),
        original_journal_sha256: hash(&journal),
        divergence: "new execution branch starts at the selected committed tick; tail events occurred but their private end state is unavailable",
    };
    fs::create_dir(destination).map_err(|e| e.to_string())?;
    let _destination_lock = Session::writer_lock(destination)?;
    // Publish checkpoint last. A partial branch stays diagnosable and never replaces parent.
    write_new(&destination.join("journal.jsonl"), &prefix)?;
    write_new(&destination.join("uncommitted-tail.bin"), tail)?;
    write_new(
        &destination.join("recovery.json"),
        &serde_json::to_vec_pretty(&result).map_err(|e| e.to_string())?,
    )?;
    write_new(
        &destination.join(format!("checkpoint-{selected:08}.cbor")),
        &checkpoint,
    )?;
    let (_, _, restored) = load(destination, Some(selected), false)?;
    if restored.checkpoint()? != world.checkpoint()? {
        return Err("recovered state differs from selected checkpoint".into());
    }
    Ok(result)
}
