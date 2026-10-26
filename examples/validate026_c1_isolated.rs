//! Post-run build isolation;the original frozen C1 verifier stays unchanged.
fn main() {
    assert!(
        std::process::Command::new("python")
            .args(["-B", "examples/support/verify026_c1_isolated.py"])
            .status()
            .unwrap()
            .success()
    );
}
