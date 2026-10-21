use std::{env, process::Command};
fn main() {
    for path in [
        "../../src",
        "../continuous-world",
        "../activity-valuation",
        "../../experiments/024/source-freeze.json",
        "../../experiments/025/registered-matrix.json",
        "generate_host.py",
        "src",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
    assert!(
        Command::new("python")
            .args(["-B", "generate_host.py", &env::var("OUT_DIR").unwrap()])
            .status()
            .unwrap()
            .success(),
        "source-preserving025 host generation"
    );
}
