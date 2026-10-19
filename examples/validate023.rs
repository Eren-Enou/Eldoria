//! Read-only023 gate, separate from the protected001-022foundation.
use std::{path::Path, process::Command};
fn main() {
    let directory = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "experiments/023".into());
    assert!(
        Command::new("python")
            .args(["-B", "examples/support/validate023.py", &directory])
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
                "prototypes/continuous-world/Cargo.toml",
                "--target-dir",
                "target/continuous-world",
                "--bin",
                "continuous-world",
                "--",
                "replay"
            ])
            .arg(Path::new(&directory).join("trials.jsonl.gz"))
            .status()
            .unwrap()
            .success()
    );
    println!(
        "023 independent reconstruction,19scratch categories and exact replay PASS; no protection/adoption implied"
    );
}
