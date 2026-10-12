//! Standalone entry point for shared read-only 012 foundation protection.
#[path = "support/contract012.rs"]
mod contract012;
fn main() {
    contract012::validate();
}
