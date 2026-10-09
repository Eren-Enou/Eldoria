//! Standalone cache-free read-only 011 foundation validation.
#[path = "support/contract011.rs"]
mod contract011;

fn main() {
    contract011::validate();
}
