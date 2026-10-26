//! Corrected026-C1 acceptance;original v1 failure remains protected evidence.
fn main() {
    assert!(
        std::process::Command::new("python")
            .args(["-B", "examples/support/verify026_c1.py"])
            .status()
            .unwrap()
            .success()
    );
}
