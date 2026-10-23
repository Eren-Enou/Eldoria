//! Safe, optional bridge to the unchanged local read-only History Explorer.
use crate::{
    Result, execution,
    storage::{self, Store},
};
use std::{
    fs::File,
    net::{TcpListener, TcpStream},
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

pub fn available() -> Result<()> {
    probe("python")
}
fn probe(program: &str) -> Result<()> {
    let mut command = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    let output = command.args(["-B", "--version"]).output().map_err(|e| {
        format!("Python3 is required by the existing viewer and prototype build host: {e}")
    })?;
    let version = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let supported = version
        .strip_prefix("Python 3.")
        .and_then(|s| s.split('.').next())
        .and_then(|s| s.parse::<u32>().ok())
        .is_some_and(|minor| minor >= 10);
    if !output.status.success() || !supported {
        return Err("Python3 is unavailable; native summaries remain available".into());
    }
    Ok(())
}
pub fn arguments(store: &Store, id: &str, port: u16) -> Result<Vec<String>> {
    if store.installed.is_some() {
        return Err("Detailed Python Explorer is a developer/advanced tool and is not included in this portable release. Native inspection remains available.".into());
    }
    execution::inspect(store, id)?;
    let m = execution::metadata(store, id)?;
    let path = store.run(id)?;
    Ok(vec![
        "-B".into(),
        store
            .repo
            .join("tools/workbench/viewer_bridge.py")
            .display()
            .to_string(),
        "--no-browser".into(),
        "--port".into(),
        port.to_string(),
        "--cache".into(),
        store.root.join("viewer-cache").display().to_string(),
        match m.configuration.mode {
            crate::config::Mode::ContinuousWorld => "--continuous",
            crate::config::Mode::ActivityValuation => "--valuation",
            crate::config::Mode::MaterialPreparation => "--preparation",
        }
        .into(),
        path.join("history.jsonl").display().to_string(),
        "--expected-sha256".into(),
        m.history_sha256.ok_or("Missing completed history hash")?,
    ])
}
pub fn command_text(store: &Store, id: &str, port: u16) -> Result<String> {
    // Copyable PowerShell representation; execution always uses the original vector.
    Ok(format!(
        "python {}",
        arguments(store, id, port)?
            .iter()
            .map(|s| format!("'{}'", s.replace('\'', "''")))
            .collect::<Vec<_>>()
            .join(" ")
    ))
}
pub struct Viewer {
    pub child: Child,
    pub url: String,
}
impl Drop for Viewer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
pub fn launch(store: &Store, id: &str) -> Result<Viewer> {
    if store.installed.is_some() {
        return Err("Python Explorer is available only in developer mode; use native inspection in this portable release".into());
    }
    available()?;
    let listener = TcpListener::bind(("127.0.0.1", 0)).map_err(|e| e.to_string())?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    drop(listener);
    let args = arguments(store, id, port)?;
    let path = store.run(id)?;
    storage::no_links(&store.root.join("viewer-cache"))?;
    let (log_id, _) = (storage::now(), id);
    let stdout_path = path.join(format!("viewer-{log_id}.stdout.log"));
    let stdout = File::create(&stdout_path).map_err(|e| e.to_string())?;
    let stderr = File::create(path.join(format!("viewer-{log_id}.stderr.log")))
        .map_err(|e| e.to_string())?;
    let mut cmd = Command::new("python");
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }
    let child = cmd
        .args(args)
        .current_dir(&store.repo)
        .stdin(Stdio::null())
        .stdout(stdout)
        .stderr(stderr)
        .spawn()
        .map_err(|e| e.to_string())?;
    let mut viewer = Viewer {
        child,
        url: format!("http://127.0.0.1:{port}"),
    };
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(180) {
        if let Some(status) = viewer.child.try_wait().map_err(|e| e.to_string())? {
            return Err(format!(
                "Viewer exited ({status}); see viewer logs in the run directory"
            ));
        }
        let announced = std::fs::read_to_string(&stdout_path)
            .unwrap_or_default()
            .contains(&format!("History Explorer: {}", viewer.url));
        if announced
            && TcpStream::connect_timeout(
                &format!("127.0.0.1:{port}")
                    .parse()
                    .map_err(|e| format!("{e}"))?,
                Duration::from_millis(100),
            )
            .is_ok()
        {
            return Ok(viewer);
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    Err(
        "Viewer startup exceeded180s; inspect viewer logs. Native summaries remain available. The copied viewer command can finish a cold catalog without this timeout."
            .into(),
    )
}
#[cfg(test)]
mod tests {
    #[test]
    fn missing_python_has_actionable_error() {
        assert!(
            super::probe("woi-no-such-python-executable")
                .unwrap_err()
                .contains("Python3")
        );
    }
}
