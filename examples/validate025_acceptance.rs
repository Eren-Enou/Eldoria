//! Standalone candidate custody/acceptance gate; protected foundation still ends at024.
use std::process::Command;

fn main() {
    assert!(
        Command::new("python")
            .args(["-B", "examples/support/validate025_acceptance.py"])
            .status()
            .expect("start standalone acceptance checker")
            .success()
    );
}
