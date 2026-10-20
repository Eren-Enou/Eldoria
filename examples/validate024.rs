//! Read-only024 gate; does not extend the protected001–023foundation.
use std::{path::Path, process::Command};
fn main() {
    let directory = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "experiments/024".into());
    assert!(
        Command::new("python")
            .args(["-B", "examples/support/validate024.py", &directory])
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
                "prototypes/activity-valuation/Cargo.toml",
                "--target-dir",
                "target/activity-valuation",
                "--bin",
                "activity-valuation",
                "--",
                "replay"
            ])
            .arg(Path::new(&directory).join("trials.jsonl.gz"))
            .status()
            .unwrap()
            .success()
    );
    println!(
        "024 independent reconstructions/corruptions and exact replay PASS;no adoption or foundation extension"
    );
}
