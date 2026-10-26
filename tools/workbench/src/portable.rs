//! Installed release identity and constrained output. No runtime developer tools.
use crate::{Result, config::Mode, storage};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    path::{Component, Path, PathBuf},
};

pub const RELEASE_SCHEMA: &str = "woi-portable-release/v1";
pub const RUNNER_FILE: &str = "woi-portable-runner.exe";
pub const APP_FILE: &str = "woi-workbench.exe";
pub fn build_identity() -> BuildIdentity {
    let identity: BuildIdentity = serde_json::from_str(include_str!(concat!(
        env!("OUT_DIR"),
        "/build-identity.json"
    )))
    .expect("verified developer build identity");
    #[cfg(feature = "world-sessions")]
    let identity = BuildIdentity {
        continuation_runtime: Some(persistent_world::persistent::IDENTITY.into()),
        ..identity
    };
    identity
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildIdentity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuation_runtime: Option<String>,
    pub schema: String,
    pub revision: String,
    pub dirty: bool,
    pub source_locks: BTreeMap<String, String>,
    pub engine_fingerprint: String,
    pub application_fingerprint: String,
    pub target: String,
    pub profile: String,
    pub target_features: String,
    pub package_version: String,
    pub compiler: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Binary {
    pub file: String,
    pub sha256: String,
    pub build: BuildIdentity,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseManifest {
    pub schema: String,
    pub version: String,
    pub platform: String,
    pub application: Binary,
    pub runner: Binary,
    pub modes: Vec<Mode>,
    pub history_schemas: Vec<String>,
    pub runtime_requirements: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct InstalledRelease {
    pub root: PathBuf,
    pub manifest: ReleaseManifest,
}
fn sha(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit())
}
impl ReleaseManifest {
    pub fn validate(&self) -> Result<()> {
        if self.schema != RELEASE_SCHEMA
            || self.version.is_empty()
            || self.platform != "windows-x86_64"
            || self.application.file != APP_FILE
            || self.runner.file != RUNNER_FILE
            || self.modes != [Mode::ContinuousWorld, Mode::ActivityValuation]
            || self.history_schemas != ["experiment-023-trial/v1", "experiment-024-trial/v1"]
        {
            return Err("Unsupported portable release/runner/schema contract".into());
        }
        for b in [&self.application, &self.runner] {
            if !sha(&b.sha256)
                || b.build.schema != "woi-build-identity/v1"
                || b.build.revision.len() != 40
                || !b.build.revision.bytes().all(|c| c.is_ascii_hexdigit())
                || !sha(&b.build.engine_fingerprint)
                || !sha(&b.build.application_fingerprint)
                || b.build
                    .continuation_runtime
                    .as_ref()
                    .is_some_and(|value| !sha(value))
                || b.build.target != "x86_64-pc-windows-msvc"
                || b.build.profile != "release"
                || !b.build.compiler.starts_with("rustc ")
                || b.build.source_locks.len() != 3
                || [
                    "experiments/023/inherited-source-bindings.json",
                    "experiments/023/source-freeze.json",
                    "experiments/024/source-freeze.json",
                ]
                .iter()
                .any(|name| {
                    !b.build
                        .source_locks
                        .get(*name)
                        .is_some_and(|value| sha(value))
                })
            {
                return Err("Invalid portable build identity/checksum".into());
            }
        }
        if self.application.build.engine_fingerprint != self.runner.build.engine_fingerprint
            || self.application.build.revision != self.runner.build.revision
            || self.application.build.source_locks != self.runner.build.source_locks
            || self.application.build.application_fingerprint
                != self.runner.build.application_fingerprint
        {
            return Err("App and runner engine/source identities differ".into());
        }
        Ok(())
    }
}
impl InstalledRelease {
    pub fn load(root: &Path) -> Result<Self> {
        storage::no_links(root)?;
        let root = root.canonicalize().map_err(|e| e.to_string())?;
        let manifest: ReleaseManifest = storage::read(&root.join("release.json"))?;
        let installed = Self { root, manifest };
        installed.verify()?;
        Ok(installed)
    }
    pub fn verify(&self) -> Result<()> {
        self.manifest.validate()?;
        for b in [&self.manifest.application, &self.manifest.runner] {
            let path = self.root.join(&b.file);
            if crate::execution::hash(&path)? != b.sha256 {
                return Err(format!("Packaged executable checksum differs: {}", b.file));
            }
        }
        Ok(())
    }
    pub fn supports(&self, mode: Mode) -> Result<()> {
        if self.manifest.modes.contains(&mode) {
            Ok(())
        } else {
            Err("This installed release does not include the requested mode.025 requires its full developer source/preflight environment; no fallback is permitted.".into())
        }
    }
}

/// Stronger portable transport rule: no archive-like paths anywhere, no links,
/// and no writes in the binary installation. Original producer guards stay intact.
pub fn safe_workspace(root: &Path, installation: &Path) -> Result<PathBuf> {
    safe_workspace_mode(root, installation, true)
}
pub fn inspect_workspace(root: &Path, installation: &Path) -> Result<PathBuf> {
    safe_workspace_mode(root, installation, false)
}
fn safe_workspace_mode(root: &Path, installation: &Path, create: bool) -> Result<PathBuf> {
    if !root.is_absolute() {
        return Err("User-data workspace must be absolute".into());
    }
    for component in root.components() {
        match component {
            Component::ParentDir | Component::CurDir => {
                return Err("Workspace traversal is forbidden".into());
            }
            Component::Normal(name)
                if ["experiments", "reinforcement", ".git"]
                    .iter()
                    .any(|n| name.to_string_lossy().eq_ignore_ascii_case(n)) =>
            {
                return Err(
                    "Scientific archives and Git directories are forbidden output locations".into(),
                );
            }
            _ => (),
        }
    }
    storage::no_links(root)?;
    storage::no_links(installation)?;
    let installation = installation.canonicalize().map_err(|e| e.to_string())?;
    // Resolve an existing ancestor before creating any directory. This also
    // handles Windows verbatim paths/case and archive names normalized by the OS.
    let existing = root
        .ancestors()
        .find(|p| p.exists())
        .ok_or("No existing workspace ancestor")?;
    let existing = existing.canonicalize().map_err(|e| e.to_string())?;
    for ancestor in existing.ancestors() {
        if ancestor.join(".git").exists()
            || (ancestor.join("AGENTS.md").is_file() && ancestor.join("experiments").is_dir())
        {
            return Err(
                "Portable user data cannot be placed inside a source/evidence checkout".into(),
            );
        }
        if ancestor.file_name().is_some_and(|s| {
            ["experiments", "reinforcement", ".git"]
                .iter()
                .any(|n| s.to_string_lossy().eq_ignore_ascii_case(n))
        }) {
            return Err("Resolved workspace is inside a scientific archive".into());
        }
    }
    if within(&existing, &installation) {
        return Err("Installation directory is not a user-data workspace".into());
    }
    if create {
        fs::create_dir_all(root).map_err(|e| e.to_string())?;
    } else if !root.exists() {
        return Ok(root.to_path_buf());
    }
    storage::no_links(root)?;
    let canonical = root.canonicalize().map_err(|e| e.to_string())?;
    if within(&canonical, &installation) {
        return Err("Workspace resolves inside installation".into());
    }
    Ok(canonical)
}
fn within(path: &Path, root: &Path) -> bool {
    let a: Vec<_> = path.components().collect();
    let b: Vec<_> = root.components().collect();
    a.len() >= b.len()
        && a.iter().zip(b).all(|(a, b)| {
            if cfg!(windows) {
                a.as_os_str()
                    .to_string_lossy()
                    .eq_ignore_ascii_case(&b.as_os_str().to_string_lossy())
            } else {
                *a == b
            }
        })
}
pub fn default_workspace() -> Result<PathBuf> {
    let local = std::env::var_os("LOCALAPPDATA")
        .ok_or("LOCALAPPDATA is unavailable; supply an explicit absolute --workspace directory")?;
    Ok(PathBuf::from(local).join("WorldOfIndividuals/Workbench"))
}
pub fn new_history_output(path: &Path) -> Result<fs::File> {
    if path.file_name().and_then(|s| s.to_str()) != Some("history.jsonl") {
        return Err("Portable history output must be named history.jsonl".into());
    }
    let parent = path.parent().ok_or("Missing output run directory")?;
    if !parent.is_dir() {
        return Err("Run directory must already exist".into());
    }
    let install = std::env::current_exe().map_err(|e| e.to_string())?;
    safe_workspace(
        parent,
        install.parent().ok_or("Missing installation parent")?,
    )?;
    storage::no_links(path)?;
    fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|e| format!("History must be a NEW file: {e}"))
}
