use sha2::{Digest, Sha256};
fn main() {
    let mut hash = Sha256::new();
    for path in [
        "build.rs",
        "Cargo.toml",
        "Cargo.lock",
        "src/lib.rs",
        "src/model.rs",
        "src/policy.rs",
        "src/runtime.rs",
        "src/storage.rs",
        "src/observer.rs",
        "src/main.rs",
    ] {
        println!("cargo:rerun-if-changed={path}");
        hash.update(path.as_bytes());
        hash.update([0]);
        let text = std::fs::read_to_string(path).unwrap().replace("\r\n", "\n");
        hash.update(text.len().to_le_bytes());
        hash.update(text.as_bytes());
    }
    println!("cargo:rustc-env=ELDORIA_RUNTIME={:x}", hash.finalize());
}
