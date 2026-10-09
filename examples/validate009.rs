//! Read-only009 archive comparison, directly from frozen gzip/manifest.
#[path = "support/contract009.rs"]
mod contract009;
fn main() {
    contract009::validate();
}
