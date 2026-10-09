//! Independent 012 replay gate, deliberately outside protected001–011 foundation.
use std::process::Command;
use world_of_individuals::experiment012 as e;
fn raw(name: &str) -> Vec<u8> {
    let result=Command::new("python").args(["-B","-c",
        "import pathlib,gzip,sys; p=pathlib.Path(sys.argv[1])/'experiments/012'; sys.stdout.buffer.write(gzip.decompress((p/(sys.argv[2]+'.json.gz')).read_bytes()))",
        env!("CARGO_MANIFEST_DIR"),name]).output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    result.stdout
}
fn main() {
    let mut summary = vec![];
    for seed in (0..8).chain([42, u64::MAX]) {
        for held in [false, true] {
            let trials = e::suite(seed, held);
            assert_eq!(trials, e::suite(seed, held));
            if seed == 42 {
                assert!(
                    serde_json::to_vec(&trials).unwrap()
                        == raw(if held { "held-out-seed-42" } else { "seed-42" }),
                    "exact012 full archive mismatch"
                );
            }
            summary.extend(trials.iter().map(|t| e::summary(t, held)));
        }
    }
    assert!(
        serde_json::to_vec(&summary).unwrap() == raw("summary"),
        "exact012 summary mismatch"
    );
    drop(summary);
    let result = Command::new("python")
        .args([
            "-B",
            concat!(env!("CARGO_MANIFEST_DIR"), "/examples/support/review012.py"),
        ])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let review: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(review["independent_trials"], 180);
    println!(
        "012:1800 exact full reruns and archived summaries;180 independently reconstructed controlled/held-out trials; no files written; foundation not extended."
    );
}
