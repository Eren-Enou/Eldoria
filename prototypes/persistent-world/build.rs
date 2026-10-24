use std::{env, process::Command};
fn main() {
    println!("cargo:rerun-if-changed=../../src");
    println!("cargo:rerun-if-changed=../continuous-world");
    println!("cargo:rerun-if-changed=../activity-valuation");
    println!("cargo:rerun-if-changed=generate_host.py");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=Cargo.lock");
    if env::var_os("CARGO_FEATURE_WORKBENCH_HOST").is_some() {
        println!("cargo:rerun-if-changed=../../tools/workbench/Cargo.lock");
    }
    assert!(
        Command::new("python")
            .args(["-B", "generate_host.py", &env::var("OUT_DIR").unwrap()])
            .status()
            .unwrap()
            .success()
    );
}
