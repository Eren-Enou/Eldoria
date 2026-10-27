use std::{env, process::Command};
fn main() {
    for path in [
        "src",
        "generate_host.py",
        "Cargo.lock",
        "../../experiments/027/registration-bindings.json",
        "../../experiments/027/specification.md",
        "../../experiments/027/design-gate.md",
        "../../experiments/027/registered-matrix.json",
        "../../experiments/027/feasibility.json",
        "../../src",
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
