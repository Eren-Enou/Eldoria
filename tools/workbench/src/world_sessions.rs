//! User-owned persistent sessions. Completed exploratory runs retain their old store.
use crate::{
    Result,
    config::{Configuration, Mode},
    storage::{self, Store},
};
pub use persistent_world::persistent::{self, session::Session};
use std::{fs, path::PathBuf};

pub struct Sessions {
    root: PathBuf,
}
impl Sessions {
    pub fn new(store: &Store) -> Result<Self> {
        if let Some(installed) = &store.installed {
            installed.verify()?;
            if installed
                .manifest
                .application
                .build
                .continuation_runtime
                .as_deref()
                != Some(persistent::IDENTITY)
            {
                return Err("installed release has no matching continuation runtime; no fallback is permitted".into());
            }
        }
        let root = store.root.join("world-sessions");
        storage::no_links(&root)?;
        fs::create_dir_all(&root).map_err(|e| e.to_string())?;
        storage::no_links(&root)?;
        Ok(Self { root })
    }
    pub fn path(&self, id: &str) -> Result<PathBuf> {
        if !id.starts_with("world-")
            || id.len() > 96
            || !id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return Err("invalid world session ID".into());
        }
        let path = self.root.join(id);
        storage::no_links(&path)?;
        Ok(path)
    }
    pub fn list(&self) -> Result<Vec<String>> {
        let mut ids = vec![];
        for entry in fs::read_dir(&self.root).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let id = entry.file_name().to_string_lossy().to_string();
            if id.starts_with("world-") && self.path(&id)?.is_dir() {
                ids.push(id);
            }
        }
        ids.sort();
        ids.reverse();
        Ok(ids)
    }
    pub fn create(&self, config: &Configuration) -> Result<Session> {
        config.validate()?;
        if config.mode == Mode::MaterialPreparation {
            return Err(
                "025 persistence is deferred; original developer preflight remains required".into(),
            );
        }
        let world: persistent_world::continuous::Config =
            serde_json::from_value(config.world()).map_err(|e| e.to_string())?;
        for index in 0..1000 {
            let id = format!("world-{}-{}-{index}", storage::now(), std::process::id());
            let path = self.path(&id)?;
            if !path.exists() {
                return if config.mode == Mode::ContinuousWorld {
                    Session::create(&path, world)
                } else {
                    Session::create_valuation(
                        &path,
                        serde_json::from_value(config.runner_config())
                            .map_err(|e| e.to_string())?,
                    )
                };
            }
        }
        Err("could not allocate a new world ID".into())
    }
    pub fn open(&self, id: &str) -> Result<Session> {
        Session::open(&self.path(id)?)
    }
}

pub fn status(session: &Session) -> serde_json::Value {
    let world = session.world();
    serde_json::json!({"schema":"woi-world-session-status/v1","id":session.path().file_name().unwrap().to_string_lossy(),"path":session.path(),"runtime":persistent::IDENTITY,"config":world.configuration(),"valuation":world.valuation(),"execution_phase":world.execution_phase(),"tick":world.tick(),"start":world.start(),"terminal":world.end(),"complete":world.complete(),"dirty":session.dirty(),"stock":world.stock(),"agents":world.agents(),"active":world.commitments(),"pending":world.pending(),"executed_boundaries":world.boundaries().len()})
}
