//! Complete protected020 gate, retaining the frozen original standalone verifier.
#[path = "support/contract020.rs"]
mod contract020;
fn main() {
    contract020::validate();
}
