//! One bounded developer entry point; never opens a GUI or writes scientific archives.
use serde_json::json;
use std::{
    env, fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    time::{Instant, SystemTime, UNIX_EPOCH},
};
fn main() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.iter().any(|a| a != "--foundation") {
        return Err("validate_reliability [--foundation]".into());
    }
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .map_err(|e| e.to_string())?;
    let out = repo.join("target/reliability-suite").join(format!(
        "run-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos()
    ));
    fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let p = "prototypes/persistent-world/Cargo.toml";
    let w = "tools/workbench/Cargo.toml";
    let mut steps: Vec<(&str, &str, Vec<&str>)> = vec![
        (
            "persistent-fmt",
            "cargo",
            vec!["fmt", "--manifest-path", p, "--check"],
        ),
        (
            "private-fmt",
            "rustfmt",
            vec![
                "--edition",
                "2024",
                "--check",
                "prototypes/persistent-world/src/runtime.rs",
                "prototypes/persistent-world/src/session.rs",
                "prototypes/persistent-world/src/reliability.rs",
            ],
        ),
        (
            "persistent-clippy",
            "cargo",
            vec![
                "clippy",
                "--manifest-path",
                p,
                "--locked",
                "--all-targets",
                "--features",
                "failure-probes",
                "-j",
                "1",
                "--",
                "-D",
                "warnings",
            ],
        ),
        (
            "persistent-tests",
            "cargo",
            vec![
                "test",
                "--lib",
                "--bins",
                "--tests",
                "--manifest-path",
                p,
                "--release",
                "--locked",
                "--features",
                "failure-probes",
                "-j",
                "1",
            ],
        ),
        (
            "workbench-fmt",
            "cargo",
            vec!["fmt", "--manifest-path", w, "--check"],
        ),
        (
            "workbench-clippy",
            "cargo",
            vec![
                "clippy",
                "--manifest-path",
                w,
                "--locked",
                "--all-targets",
                "--features",
                "world-sessions",
                "-j",
                "1",
                "--",
                "-D",
                "warnings",
            ],
        ),
        (
            "workbench-tests",
            "cargo",
            vec![
                "test",
                "--manifest-path",
                w,
                "--release",
                "--locked",
                "--all-targets",
                "--features",
                "world-sessions",
                "-j",
                "1",
            ],
        ),
    ];
    // These existing regressions execute CLI/HTTP readers only; no GUI/browser.
    for (name, test) in [
        ("workbench-v2", "v2"),
        ("workbench-readers", "portable"),
        ("workbench-runner", "real_runner"),
    ] {
        steps.push((
            name,
            "cargo",
            vec![
                "test",
                "--manifest-path",
                w,
                "--release",
                "--locked",
                "--features",
                "world-sessions",
                "-j",
                "1",
                "--test",
                test,
                "--",
                "--ignored",
            ],
        ));
    }
    if args.iter().any(|a| a == "--foundation") {
        steps.extend([
            ("root-fmt", "cargo", vec!["fmt", "--check"]),
            (
                "root-clippy",
                "cargo",
                vec![
                    "clippy",
                    "--locked",
                    "--all-targets",
                    "-j",
                    "1",
                    "--",
                    "-D",
                    "warnings",
                ],
            ),
            ("root-tests", "cargo", vec!["test", "--locked", "-j", "1"]),
            (
                "foundation",
                "cargo",
                vec![
                    "run",
                    "--release",
                    "--locked",
                    "-j",
                    "1",
                    "--example",
                    "validate_contract",
                ],
            ),
            (
                "025-custody",
                "cargo",
                vec![
                    "run",
                    "--release",
                    "--locked",
                    "-j",
                    "1",
                    "--example",
                    "validate025_acceptance",
                ],
            ),
        ]);
    }
    let mut results = vec![];
    for (name, program, arguments) in steps {
        println!("{name}: running");
        let started = Instant::now();
        let output = Command::new(program)
            .args(&arguments)
            .current_dir(&repo)
            .stdin(Stdio::null())
            .output()
            .map_err(|e| e.to_string())?;
        let log = out.join(format!("{name}.log"));
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&log)
            .map_err(|e| e.to_string())?;
        file.write_all(&output.stdout)
            .and_then(|_| file.write_all(&output.stderr))
            .map_err(|e| e.to_string())?;
        results.push(json!({"step":name,"command":program,"arguments":arguments,"success":output.status.success(),"exit_code":output.status.code(),"wall_ms":started.elapsed().as_secs_f64()*1000.,"log":log}));
        fs::write(out.join("summary.json"),serde_json::to_vec_pretty(&json!({"schema":"woi-reliability-suite/v1","results":results,"platform":env::consts::OS,"gui":"DEFERRED; no desktop control authorized"})).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
        if !output.status.success() {
            return Err(format!(
                "{name} failed; exact diagnostics: {}",
                log.display()
            ));
        }
        println!("{name}: PASS");
    }
    println!("PASS: {}", out.join("summary.json").display());
    Ok(())
}
