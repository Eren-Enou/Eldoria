use std::{env, process::Command};
fn main() {
    for path in [
        "../../src",
        "../material-preparation",
        "../material-preparation-observer2",
        "../activity-valuation",
        "../../experiments/026/registered-matrix.json",
        "../../experiments/026/prospective-validation.json",
        "../../experiments/026/inherited-source-bindings.json",
        "../../experiments/026/source-freeze.json",
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
            .success()
    );
}
