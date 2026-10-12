//! Standalone013 read-only replay; deliberately outside protected001–012.
use std::process::Command;
use world_of_individuals::experiment013 as e;
fn raw(name: &str) -> Vec<u8> {
    let result=Command::new("python").args(["-B","-c","import pathlib,gzip,sys; p=pathlib.Path(sys.argv[1])/'experiments/013'; sys.stdout.buffer.write(gzip.decompress((p/(sys.argv[2]+'.json.gz')).read_bytes()))",env!("CARGO_MANIFEST_DIR"),name]).output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    result.stdout
}
fn main() {
    let mut rows = vec![];
    for seed in [0, 1, 2, 3, 42, u64::MAX] {
        for held in [false, true] {
            let ts = e::suite(seed, held);
            assert_eq!(ts, e::suite(seed, held));
            if seed == 42 {
                assert!(
                    serde_json::to_vec(&ts).unwrap()
                        == raw(if held { "held-out-seed-42" } else { "seed-42" })
                );
            }
            rows.extend(ts.iter().map(|t| e::summary(t, held)));
        }
    }
    assert!(serde_json::to_vec(&rows).unwrap() == raw("summary"));
    drop(rows);
    let diagnostic: Vec<_> = [1, 42]
        .into_iter()
        .flat_map(|seed| {
            [e::Mode::Unchanged, e::Mode::Pulse]
                .into_iter()
                .map(move |mode| e::trial(seed, mode, "longitudinal", e::held_out()[1].clone()))
        })
        .collect();
    assert!(
        format!("{}\n", serde_json::to_string(&diagnostic).unwrap()).into_bytes()
            == raw("seed-diagnostic")
    );
    let result = Command::new("python")
        .args([
            "-B",
            concat!(env!("CARGO_MANIFEST_DIR"), "/examples/support/review013.py"),
            "--self-test",
        ])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let actual: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let expected: serde_json::Value = serde_json::from_slice(
        &std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/experiments/013/analysis.json"
        ))
        .unwrap(),
    )
    .unwrap();
    assert_eq!(actual, expected);
    assert_eq!(actual["independent_trials"], 195);
    println!(
        "013:1170 exact full reruns, all archives/summary,195 independent seed42 reconstructions and corruption/paired-locality checks; no files written; foundation not extended."
    );
}
