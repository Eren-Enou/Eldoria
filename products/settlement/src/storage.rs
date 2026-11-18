//! Immutable checkpoints. A fresh filename is required for each generation.
use crate::{
    Settlement,
    model::{Checkpoint, RULES, RUNTIME},
    runtime,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::{Component, Path},
};

pub const SCHEMA: &str = "eldoria-settlement-checkpoint/v1";
pub const MAX_BYTES: u64 = 256 * 1024 * 1024;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    schema: String,
    rules: String,
    runtime: String,
    checksum: String,
    payload: Checkpoint,
}
fn checksum(payload: &Checkpoint) -> Result<String, String> {
    let bytes =
        serde_json::to_vec(&(SCHEMA, RULES, RUNTIME, payload)).map_err(|e| e.to_string())?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}
pub fn encode(world: &Settlement) -> Result<Vec<u8>, String> {
    let payload = world.checkpoint();
    runtime::validate(&payload)?;
    let checksum = checksum(&payload)?;
    let bytes = serde_json::to_vec(&Envelope {
        schema: SCHEMA.into(),
        rules: RULES.into(),
        runtime: RUNTIME.into(),
        checksum,
        payload,
    })
    .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err("checkpoint exceeds256MiB bound".into());
    }
    Ok(bytes)
}
pub fn decode(bytes: &[u8]) -> Result<Settlement, String> {
    if bytes.len() as u64 > MAX_BYTES {
        return Err("checkpoint exceeds256MiB bound".into());
    }
    let envelope: Envelope = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    if envelope.schema != SCHEMA || envelope.rules != RULES || envelope.runtime != RUNTIME {
        return Err("unsupported checkpoint schema/rules/runtime".into());
    }
    if envelope.checksum != checksum(&envelope.payload)? {
        return Err("checkpoint checksum mismatch".into());
    }
    Settlement::restore(envelope.payload)
}

/// Refuse archives, traversal, symlinks and Windows junctions before any write.
pub fn check_output(path: &Path) -> Result<(), String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join(path)
    };
    fn forbidden(path: &Path) -> bool {
        path.components().any(|c| match c {
            Component::ParentDir => true,
            Component::Normal(s) => ["experiments", "reinforcement", ".git", "research"]
                .iter()
                .any(|p| s.to_string_lossy().eq_ignore_ascii_case(p)),
            _ => false,
        })
    }
    if forbidden(&absolute) {
        return Err("output must be outside research archives and contain no traversal".into());
    }
    for parent in absolute.ancestors() {
        if let Ok(meta) = fs::symlink_metadata(parent) {
            if meta.file_type().is_symlink() {
                return Err("output links forbidden".into());
            }
            #[cfg(windows)]
            {
                use std::os::windows::fs::MetadataExt;
                if meta.file_attributes() & 0x400 != 0 {
                    return Err("output reparse points forbidden".into());
                }
            }
        }
    }
    let existing = absolute
        .ancestors()
        .find(|p| p.exists())
        .ok_or("no existing output ancestor")?;
    if forbidden(&existing.canonicalize().map_err(|e| e.to_string())?) {
        return Err("resolved output lies in protected archives".into());
    }
    if path.exists() {
        return Err("output already exists; choose a new generation filename".into());
    }
    Ok(())
}

pub fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    check_output(path)?;
    let filename = path
        .file_name()
        .ok_or("output filename required")?
        .to_string_lossy();
    let pending = path.with_file_name(format!("{filename}.pending"));
    check_output(&pending)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&pending)
        .map_err(|e| e.to_string())?;
    file.write_all(bytes)
        .and_then(|_| file.sync_all())
        .map_err(|e| e.to_string())?;
    drop(file);
    if fs::read(&pending).map_err(|e| e.to_string())? != bytes {
        return Err("checkpoint readback mismatch; pending file retained".into());
    }
    // Hard-link publication atomically refuses an existing destination, including
    // races. Unlike rename on Unix, it can never replace an earlier save.
    fs::hard_link(&pending, path).map_err(|e| format!("publish failed; pending retained: {e}"))?;
    fs::remove_file(&pending).map_err(|e| format!("published, but pending cleanup failed: {e}"))?;
    Ok(())
}
pub fn save(world: &Settlement, path: &Path) -> Result<(), String> {
    write_new(path, &encode(world)?)
}
pub fn load(path: &Path) -> Result<Settlement, String> {
    if path
        .extension()
        .is_some_and(|s| s.eq_ignore_ascii_case("pending"))
    {
        return Err("pending file is not a committed checkpoint".into());
    }
    let mut bytes = vec![];
    fs::File::open(path)
        .map_err(|e| e.to_string())?
        .take(MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    decode(&bytes)
}
