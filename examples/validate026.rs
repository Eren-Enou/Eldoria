//! Separate026 research gate; protected endpoint remains001–025.
fn main() {
    let directory = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "experiments/026".into());
    assert!(
        std::process::Command::new("python")
            .args(["-B", "examples/support/verify026.py", &directory])
            .status()
            .unwrap()
            .success()
    );
}
