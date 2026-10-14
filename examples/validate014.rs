//! Standalone exploratory014 replay; foundation remains001–013.
use std::process::Command;
use world_of_individuals::experiment014 as e;
fn raw(name: &str) -> Vec<u8> {
    let out=Command::new("python").args(["-B","-c","import gzip,pathlib,sys;sys.stdout.buffer.write(gzip.decompress((pathlib.Path(sys.argv[1])/'experiments/014'/(sys.argv[2]+'.json.gz')).read_bytes()))",env!("CARGO_MANIFEST_DIR"),name]).output().unwrap();
    assert!(out.status.success());
    out.stdout
}
fn main() {
    let mut rows = vec![];
    for seed in [0, 1, 42, u64::MAX] {
        for held in [false, true] {
            let ts = e::suite(seed, held);
            assert_eq!(ts, e::suite(seed, held));
            if seed == 42 {
                assert_eq!(
                    serde_json::to_vec(&ts).unwrap(),
                    raw(if held { "held-out-seed-42" } else { "seed-42" })
                );
            }
            rows.extend(ts.iter().map(|t| e::summary(t, held)));
        }
    }
    assert_eq!(serde_json::to_vec(&rows).unwrap(), raw("summary"));
    drop(rows);
    let diagnostic: Vec<_> = world_of_individuals::activity::MODES
        .into_iter()
        .map(|mode| {
            e::trial(
                42,
                mode,
                "competition",
                e::Config {
                    budget: 13,
                    ..Default::default()
                },
            )
        })
        .collect();
    assert_eq!(
        format!("{}\n", serde_json::to_string(&diagnostic).unwrap()).into_bytes(),
        raw("exclusive-budget")
    );
    let out = Command::new("python")
        .args([
            "-B",
            concat!(env!("CARGO_MANIFEST_DIR"), "/examples/support/review014.py"),
            "--self-test",
        ])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let actual: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let expected: serde_json::Value = serde_json::from_slice(
        &std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/experiments/014/analysis.json"
        ))
        .unwrap(),
    )
    .unwrap();
    assert_eq!(actual, expected);
    assert_eq!(actual["independent_trials"], 215);
    println!(
        "014:840 exact full replays plus five13-tick diagnostics, archived raw/summary equality,215 independent cases,430 comparisons, corruption and mirrored-locality checks; no files written; not protected."
    );
}
