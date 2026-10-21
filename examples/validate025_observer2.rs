//! Read-only025observer-corrected research gate; foundation still ends at024.
use std::{path::Path, process::Command};

fn main() {
    let directory = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "experiments/025".into());
    assert!(
        Command::new("python")
            .args([
                "-B",
                "examples/support/validate025_observer2.py",
                &directory
            ])
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
                "prototypes/material-preparation-observer2/Cargo.toml",
                "--target-dir",
                "target/material-preparation-observer2",
                "--bin",
                "material-preparation-observer2",
                "--",
                "replay"
            ])
            .arg(Path::new(&directory).join("trials.jsonl.gz"))
            .status()
            .unwrap()
            .success()
    );
    println!(
        "025observer2 independent reconstruction and exact replay PASS;no adoption/protection"
    );
}
