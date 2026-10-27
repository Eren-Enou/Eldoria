//! Additive full001–026 gate. Historical bound entrypoints remain unchanged.
use std::process::Command;
fn main() {
    for example in ["validate025_foundation", "validate026_c1_isolated"] {
        assert!(
            Command::new("cargo")
                .args([
                    "run",
                    "--release",
                    "--locked",
                    "-j",
                    "1",
                    "--example",
                    example
                ])
                .status()
                .unwrap()
                .success()
        );
    }
    assert!(
        Command::new("python")
            .args(["-B", "examples/support/foundation026.py"])
            .status()
            .unwrap()
            .success()
    );
}
