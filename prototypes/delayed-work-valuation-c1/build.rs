use std::{env, process::Command};
fn main() {
    for path in [
        "../../src",
        "../delayed-work-valuation",
        "../../experiments/026/continuation-c1/source-bindings.json",
        "../../experiments/026/continuation-c1/prospective-validation.json",
        "../../experiments/026/continuation-c1/source-freeze.json",
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
