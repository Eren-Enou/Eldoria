use std::{env, process::Command};
fn main() {
    for path in [
        "../../src",
        "../../experiments/026",
        "../persistent-world/src",
        "../persistent-material-world",
        "../continuous-world",
        "../activity-valuation",
        "../material-preparation",
        "../material-preparation-observer2",
        "../delayed-work-valuation",
        "../delayed-work-valuation-c1",
        "generate_host.py",
        "src",
        "Cargo.toml",
        "Cargo.lock",
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
