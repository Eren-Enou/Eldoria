use crate::{
    Result,
    config::Configuration,
    history::{self, Summary},
    storage::{self, Store},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Read,
    path::Path,
    process::{Command, Stdio},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub revision: String,
    pub dirty: bool,
    pub runner_fingerprint: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub schema: String,
    pub id: String,
    pub configuration: Configuration,
    pub source: Source,
    pub program: String,
    pub arguments: Vec<String>,
    pub cwd: String,
    pub history: String,
    pub started_unix_ms: u128,
    pub finished_unix_ms: Option<u128>,
    pub elapsed_ms: Option<u128>,
    pub status: String,
    pub exit_code: Option<i32>,
    pub diagnostic: Option<String>,
    pub history_sha256: Option<String>,
    pub reproduced_from: Option<String>,
    pub exact_match: Option<bool>,
    pub warnings: Vec<String>,
}
pub fn hash(path: &Path) -> Result<String> {
    storage::no_links(path)?;
    let mut f = File::open(path).map_err(|e| e.to_string())?;
    let mut h = Sha256::new();
    let mut buf = [0; 65536];
    loop {
        let n = f.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(format!("{:x}", h.finalize()))
}
fn git(repo: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .map_err(|e| format!("Git is required for source identity: {e}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}
pub fn source(store: &Store) -> Result<Source> {
    let mut inputs = BTreeMap::new();
    for name in [
        "experiments/023/source-freeze.json",
        "experiments/023/inherited-source-bindings.json",
    ] {
        let locks: BTreeMap<String, String> = storage::read(&store.repo.join(name))?;
        for (path, expected) in locks {
            let p = store.repo.join(&path);
            storage::no_links(&p)?;
            let bytes = fs::read(&p).map_err(|e| format!("Missing runner source {path}: {e}"))?;
            let normalized = String::from_utf8(bytes)
                .map_err(|e| e.to_string())?
                .replace("\r\n", "\n");
            let actual = format!("{:x}", Sha256::digest(normalized.as_bytes()));
            if actual != expected {
                return Err(format!(
                    "023 frozen runner source differs: {path}. Restore the required revision; workbench will not adapt it."
                ));
            }
            inputs.insert(path, actual);
        }
    }
    Ok(Source {
        revision: git(&store.repo, &["rev-parse", "HEAD"])?,
        dirty: !git(&store.repo, &["status", "--porcelain"])?.is_empty(),
        runner_fingerprint: format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&inputs).map_err(|e| e.to_string())?)
        ),
    })
}
pub fn arguments(store: &Store, configuration: &Configuration, history: &Path) -> Vec<String> {
    [
        "run".into(),
        "--release".into(),
        "--locked".into(),
        "-j".into(),
        "1".into(),
        "--manifest-path".into(),
        store
            .repo
            .join("prototypes/continuous-world/Cargo.toml")
            .to_string_lossy()
            .into_owned(),
        "--target-dir".into(),
        store
            .repo
            .join("target/continuous-world")
            .to_string_lossy()
            .into_owned(),
        "--bin".into(),
        "continuous-world".into(),
        "--".into(),
        "run".into(),
        history.to_string_lossy().into_owned(),
        "--case".into(),
        configuration.scenario.clone(),
        "--ticks".into(),
        configuration.ticks.to_string(),
        "--period".into(),
        configuration.period.to_string(),
        "--seed".into(),
        configuration.seed.to_string(),
        "--mode".into(),
        "Continuous".into(),
        "--phase".into(),
        "Epoch".into(),
        "--order".into(),
        "Ascending".into(),
        "--claim".into(),
        "4".into(),
    ]
    .into()
}
pub fn metadata(store: &Store, id: &str) -> Result<Metadata> {
    let m: Metadata = storage::read(&store.run(id)?.join("metadata.json"))?;
    if m.schema != "woi-workbench-run/v1" || m.id != id {
        return Err("Unsupported or mismatched run metadata".into());
    }
    m.configuration.validate()?;
    Ok(m)
}
pub fn inspect(store: &Store, id: &str) -> Result<Summary> {
    let m = metadata(store, id)?;
    if m.status != "Completed" {
        return Err(format!(
            "Run is {}; inspect stdout/stderr in its directory",
            m.status
        ));
    }
    let path = store.run(id)?.join("history.jsonl");
    if m.history_sha256.as_deref() != Some(&hash(&path)?) {
        return Err("History hash differs from completed run metadata".into());
    }
    history::read(&path, &m.configuration)
}
pub fn run(store: &Store, config: Configuration, original: Option<&str>) -> Result<Metadata> {
    execute(store, config, original, None)
}
fn execute(
    store: &Store,
    config: Configuration,
    original: Option<&str>,
    command: Option<Command>,
) -> Result<Metadata> {
    config.validate()?;
    let source = source(store)?;
    storage::no_links(&store.repo.join("target/continuous-world"))?;
    let previous = original.map(|id| metadata(store, id)).transpose()?;
    let (id, path) = store.create()?;
    let args = arguments(store, &config, &path.join("history.jsonl"));
    let mut cmd = command.unwrap_or_else(|| {
        let mut c = Command::new("cargo");
        c.args(&args);
        c
    });
    let mut m = Metadata {
        schema: "woi-workbench-run/v1".into(),
        id,
        configuration: config,
        source,
        program: cmd.get_program().to_string_lossy().into_owned(),
        arguments: cmd
            .get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect(),
        cwd: store.repo.display().to_string(),
        history: "history.jsonl".into(),
        started_unix_ms: storage::now(),
        finished_unix_ms: None,
        elapsed_ms: None,
        status: "Incomplete".into(),
        exit_code: None,
        diagnostic: None,
        history_sha256: None,
        reproduced_from: original.map(str::to_owned),
        exact_match: None,
        warnings: vec![
            "Exploratory execution only; not registered evidence or scientific acceptance".into(),
        ],
    };
    if m.source.dirty {
        m.warnings.push("Working tree has uncommitted changes; frozen023 runner inputs were verified separately".into());
    }
    if let Some(old) = &previous
        && old.source.revision != m.source.revision
    {
        m.warnings
            .push("Source revision changed: exact historical equivalence is not guaranteed".into());
    }
    if let Some(old) = &previous
        && old.source.runner_fingerprint != m.source.runner_fingerprint
    {
        m.warnings.push(
            "Runner input fingerprint changed: this is not an identical source reproduction".into(),
        );
    }
    storage::write_new(&path.join("configuration.json"), &m.configuration)?;
    storage::write_new(&path.join("metadata.json"), &m)?;
    let stdout = File::create(path.join("stdout.log")).map_err(|e| e.to_string())?;
    let stderr = File::create(path.join("stderr.log")).map_err(|e| e.to_string())?;
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }
    let start = std::time::Instant::now();
    let result = cmd
        .current_dir(&store.repo)
        .stdin(Stdio::null())
        .stdout(stdout)
        .stderr(stderr)
        .status();
    m.elapsed_ms = Some(start.elapsed().as_millis());
    m.finished_unix_ms = Some(storage::now());
    match result {
        Err(e) => {
            m.diagnostic = Some(format!(
                "Could not launch Cargo/runner: {e}. Check Rust, Cargo and build-time Python on PATH."
            ))
        }
        Ok(status) => {
            m.exit_code = status.code();
            if status.success() {
                match history::read(&path.join("history.jsonl"), &m.configuration) {
                    Ok(summary) if summary.food_accounting_matches => {
                        m.history_sha256 = Some(hash(&path.join("history.jsonl"))?);
                        m.status = "Completed".into();
                        if let Some(old) = previous {
                            m.exact_match = old.history_sha256.map(|h| Some(h) == m.history_sha256);
                        }
                    }
                    Ok(_) => {
                        m.diagnostic = Some(
                            "Output exists but its observer food ledger does not balance".into(),
                        )
                    }
                    Err(e) => m.diagnostic = Some(e),
                }
            } else {
                m.diagnostic = Some(format!(
                    "Runner failed ({status}); see stderr.log. Output is not complete."
                ));
            }
        }
    }
    storage::write_new(&path.join("metadata.finished.json"), &m)?;
    // Same-directory replacement: interrupted writes leave the initial incomplete record.
    storage::no_links(&path.join("metadata.json"))?;
    fs::rename(
        path.join("metadata.finished.json"),
        path.join("metadata.json"),
    )
    .map_err(|e| e.to_string())?;
    Ok(m)
}
pub fn reproduce(store: &Store, id: &str) -> Result<Metadata> {
    let m = metadata(store, id)?;
    let c: Configuration = storage::read(&store.run(id)?.join("configuration.json"))?;
    if c != m.configuration {
        return Err("Saved configuration differs from original metadata".into());
    }
    run(store, c, Some(id))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn store() -> Store {
        let mut s = Store::open(&crate::repository()).unwrap();
        s.root = s.root.join("acceptance-tests");
        fs::create_dir_all(&s.root).unwrap();
        s
    }
    #[test]
    fn real_failure_and_zero_exit_without_history_are_incomplete() {
        let store = store();
        let missing = execute(
            &store,
            Configuration::default(),
            None,
            Some(Command::new("woi-no-such-runner-executable")),
        )
        .unwrap();
        assert_eq!(missing.status, "Incomplete");
        assert!(missing.diagnostic.unwrap().contains("Could not launch"));
        let mut failure = Command::new("rustc");
        failure.arg("--not-a-real-option");
        let m = execute(&store, Configuration::default(), None, Some(failure)).unwrap();
        assert_eq!(m.status, "Incomplete");
        assert_ne!(m.exit_code, Some(0));
        let mut empty = Command::new("rustc");
        empty.arg("--version");
        let m = execute(&store, Configuration::default(), None, Some(empty)).unwrap();
        assert_eq!(m.status, "Incomplete");
        assert_eq!(m.exit_code, Some(0));
        assert!(m.diagnostic.unwrap().contains("history"));
    }
    #[test]
    #[ignore = "explicit real runner acceptance"]
    fn real_run_and_exact_reproduction() {
        let store = store();
        let m = run(&store, Configuration::default(), None).unwrap();
        assert_eq!(m.status, "Completed", "{:?}", m.diagnostic);
        assert_eq!(
            m.source.revision,
            git(&store.repo, &["rev-parse", "HEAD"]).unwrap()
        );
        let summary = inspect(&store, &m.id).unwrap();
        assert_eq!(summary.final_agents.len(), 4);
        assert_eq!(summary.meals, 10);
        assert_eq!(summary.transferred_units, 34);
        assert_eq!(summary.environmental_grants, 4);
        assert_eq!(summary.resource_events, 182);
        assert_eq!(summary.completed_activities, 480);
        assert_eq!(summary.end, 231);
        let before = hash(&store.run(&m.id).unwrap().join("history.jsonl")).unwrap();
        let command = crate::viewer::command_text(&store, &m.id, 8766).unwrap();
        assert!(command.contains("--continuous") && command.contains("--no-browser"));
        let note = crate::observations::Observation {
            individual: 0,
            tick: 6,
            notes: "Acceptance annotation; external human observation".into(),
            ..Default::default()
        };
        let saved = crate::observations::save(&store, &m.id, &note).unwrap();
        assert!(saved.starts_with(store.run(&m.id).unwrap().join("observations")));
        assert_eq!(
            before,
            hash(&store.run(&m.id).unwrap().join("history.jsonl")).unwrap()
        );
        let invalid = crate::observations::Observation {
            individual: 4,
            ..note
        };
        assert!(crate::observations::save(&store, &m.id, &invalid).is_err());
        // Saved command fields are audit data, never execution instructions.
        let mut edited = m.clone();
        edited.program = "must-never-be-executed".into();
        edited.arguments = vec!["arbitrary saved command".into()];
        edited.source.revision = "historical-other-revision".into();
        fs::write(
            store.run(&m.id).unwrap().join("metadata.json"),
            serde_json::to_vec(&edited).unwrap(),
        )
        .unwrap();
        let next = reproduce(&store, &m.id).unwrap();
        assert_ne!(m.id, next.id);
        assert_eq!(next.exact_match, Some(true));
        assert_eq!(next.program, "cargo");
        assert!(
            next.warnings
                .iter()
                .any(|w| w.contains("Source revision changed"))
        );
    }
}
