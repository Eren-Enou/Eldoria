//! Explicit full001–025 successor; historical source-bound normal gate is unchanged.
use std::process::Command;
pub fn validate() {
    for arguments in [
        vec![
            "run",
            "--release",
            "--locked",
            "-j",
            "1",
            "--example",
            "validate_contract",
        ],
        vec![
            "run",
            "--release",
            "--locked",
            "-j",
            "1",
            "--example",
            "validate025_acceptance",
        ],
    ] {
        assert!(
            Command::new("cargo")
                .args(arguments)
                .status()
                .unwrap()
                .success()
        );
    }
    assert!(
        Command::new("python")
            .args(["-B", "examples/support/foundation025.py"])
            .status()
            .unwrap()
            .success()
    );
}
