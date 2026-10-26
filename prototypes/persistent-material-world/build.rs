use std::{env, process::Command};
fn main() {
    for path in [
        "../../src",
        "../persistent-world/src",
        "../continuous-world",
        "../activity-valuation",
        "../material-preparation",
        "../material-preparation-observer2",
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
