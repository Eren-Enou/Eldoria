use std::{env, process::Command};
fn main() {
    println!("cargo:rerun-if-changed=../../src");
    println!("cargo:rerun-if-changed=../../experiments/023/inherited-source-bindings.json");
    println!("cargo:rerun-if-changed=generate_host.py");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=../continuous-world");
    println!("cargo:rerun-if-changed=../../experiments/024/registered-matrix.json");
    let status = Command::new("python")
        .args(["-B", "generate_host.py", &env::var("OUT_DIR").unwrap()])
        .status()
        .unwrap();
    assert!(status.success(), "immutable source host generation failed");
}
