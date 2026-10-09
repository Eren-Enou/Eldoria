//! Standalone read-only 010 foundation validation, without generated caches.
#[path = "support/contract010.rs"]
mod contract010;
fn main() {
    contract010::validate();
}
