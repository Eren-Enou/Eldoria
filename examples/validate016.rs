//! Read-only protected016 replay, shared with normal foundation validation.
#[path = "support/contract016.rs"]
mod contract016;
fn main() {
    contract016::validate();
}
