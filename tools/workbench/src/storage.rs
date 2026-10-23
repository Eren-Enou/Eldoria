//! All application writes are confined to new directories below the approved root.
use crate::Result;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

pub fn no_links(path: &Path) -> Result<()> {
    for ancestor in path.ancestors() {
        if let Ok(m) = fs::symlink_metadata(ancestor) {
            if m.file_type().is_symlink() {
                return Err(format!("Links are not allowed: {}", ancestor.display()));
            }
            #[cfg(windows)]
            {
                use std::os::windows::fs::MetadataExt;
                if m.file_attributes() & 0x400 != 0 {
                    return Err(format!(
                        "Reparse points are not allowed: {}",
                        ancestor.display()
                    ));
                }
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct Store {
    pub repo: PathBuf,
    pub root: PathBuf,
    pub installed: Option<crate::portable::InstalledRelease>,
}
impl Store {
    pub fn open(repo: &Path) -> Result<Self> {
        let repo = repo
            .canonicalize()
            .map_err(|e| format!("Repository unavailable: {e}"))?;
        if !repo
            .join("prototypes/continuous-world/Cargo.toml")
            .is_file()
            || !repo.join("AGENTS.md").is_file()
        {
            return Err(
                "Select the Eldoria repository containing the continuous-world runner".into(),
            );
        }
        let root = repo.join("target/workbench-runs");
        no_links(&root)?;
        fs::create_dir_all(&root).map_err(|e| e.to_string())?;
        no_links(&root)?;
        if !root
            .canonicalize()
            .map_err(|e| e.to_string())?
            .starts_with(&repo)
        {
            return Err("Output root escapes repository".into());
        }
        Ok(Self {
            repo,
            root,
            installed: None,
        })
    }
    pub fn detect(workspace: Option<&Path>) -> Result<Self> {
        let executable = std::env::current_exe().map_err(|e| e.to_string())?;
        let installation = executable
            .parent()
            .ok_or("No executable installation directory")?;
        if installation.join("release.json").exists() || cfg!(feature = "portable") {
            return Self::portable(installation, workspace);
        }
        if workspace.is_some() {
            return Err("--workspace is for installed portable mode; development storage stays inside its approved root".into());
        }
        #[cfg(not(feature = "portable"))]
        {
            Self::open(&crate::repository())
        }
        #[cfg(feature = "portable")]
        {
            Err("Portable release manifest is missing; no source-build fallback is allowed".into())
        }
    }
    pub fn portable(installation: &Path, workspace: Option<&Path>) -> Result<Self> {
        let installed = crate::portable::InstalledRelease::load(installation)?;
        #[cfg(feature = "portable")]
        if installed.manifest.application.build != crate::portable::build_identity() {
            return Err(
                "Manifest application identity differs from this executable's embedded identity"
                    .into(),
            );
        }
        let workspace = match workspace {
            Some(path) => path.to_path_buf(),
            None => crate::portable::default_workspace()?,
        };
        let data = crate::portable::safe_workspace(&workspace, &installed.root)?;
        let root = data.join("runs");
        no_links(&root)?;
        fs::create_dir_all(&root).map_err(|e| e.to_string())?;
        no_links(&root)?;
        Ok(Self {
            repo: installed.root.clone(),
            root,
            installed: Some(installed),
        })
    }
    pub fn supports(&self, mode: crate::config::Mode) -> Result<()> {
        self.installed
            .as_ref()
            .map_or(Ok(()), |installed| installed.supports(mode))
    }
    pub fn run(&self, id: &str) -> Result<PathBuf> {
        if id.is_empty()
            || !id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
            || Path::new(id)
                .components()
                .any(|c| !matches!(c, Component::Normal(_)))
        {
            return Err("Invalid run identity; paths and separators are forbidden".into());
        }
        let path = self.root.join(id);
        no_links(&path)?;
        Ok(path)
    }
    pub fn create(&self) -> Result<(String, PathBuf)> {
        for index in 0..1000 {
            let id = format!("run-{}-{}-{index}", now(), std::process::id());
            let path = self.run(&id)?;
            match fs::create_dir(&path) {
                Ok(()) => return Ok((id, path)),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e.to_string()),
            }
        }
        Err("Could not reserve a unique run directory".into())
    }
    pub fn list(&self) -> Result<Vec<String>> {
        let mut ids = Vec::new();
        for entry in fs::read_dir(&self.root).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let id = entry.file_name().to_string_lossy().into_owned();
            if id.starts_with("run-")
                && let Ok(path) = self.run(&id)
                && path.is_dir()
            {
                ids.push(id);
            }
        }
        ids.sort();
        ids.reverse();
        Ok(ids)
    }
}
pub fn write_new<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    no_links(path)?;
    let bytes = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    f.write_all(&bytes)
        .and_then(|()| f.sync_all())
        .map_err(|e| e.to_string())
}
pub fn read<T: DeserializeOwned>(path: &Path) -> Result<T> {
    no_links(path)?;
    let mut f = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if f.metadata().map_err(|e| e.to_string())?.len() > 1024 * 1024 {
        return Err("Metadata/configuration exceeds 1 MiB".into());
    }
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}
