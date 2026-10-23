#[path = "../workbench/build_support/identity.rs"]
mod identity;
fn main() {
    identity::generate(
        &std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("../.."),
    );
}
