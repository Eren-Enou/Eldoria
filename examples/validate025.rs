//! Read-only025research gate. Normal foundation intentionally ends at024.
use std::{path::Path, process::Command};
fn main() {
    let directory = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "experiments/025".into());
    assert!(
        Command::new("python")
            .args(["-B", "examples/support/validate025.py", &directory])
            .status()
            .unwrap()
            .success()
    );
    assert!(
        Command::new("cargo")
            .args([
                "run",
                "--release",
                "--locked",
                "-j",
                "1",
                "--manifest-path",
                "prototypes/material-preparation/Cargo.toml",
                "--target-dir",
                "target/material-preparation",
                "--bin",
                "material-preparation",
                "--",
                "replay"
            ])
            .arg(Path::new(&directory).join("trials.jsonl.gz"))
            .status()
            .unwrap()
            .success()
    );
    println!(
        "025independent reconstruction and exact replay PASS;no foundation extension or default adoption"
    );
}
