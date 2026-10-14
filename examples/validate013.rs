//! Standalone entry point for the shared read-only 013 foundation gate.
#[path = "support/contract013.rs"]
mod contract013;
fn main() {
    contract013::validate();
}
