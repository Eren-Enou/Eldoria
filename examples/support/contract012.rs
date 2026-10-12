//! Shared read-only 012 foundation protection; simulation behavior is unchanged.
use std::process::Command;
use world_of_individuals::experiment012 as e;
fn python(script: &str, name: &str) -> Vec<u8> {
    let result = Command::new("python")
        .args(["-B", "-c", script, env!("CARGO_MANIFEST_DIR"), name])
        .output()
        .expect("Python standard library is required for frozen 012 verification");
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    result.stdout
}
fn raw(name: &str) -> Vec<u8> {
    python(
        r#"import gzip,hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1]); name=sys.argv[2]
expected=json.loads((root/'experiments/012/archive-manifest.json').read_bytes())[name]
compressed=(root/f'experiments/012/{name}.json.gz').read_bytes()
raw=gzip.decompress(compressed)
assert (len(compressed),hashlib.sha256(compressed).hexdigest(),len(raw),hashlib.sha256(raw).hexdigest()) == (expected['gzip_bytes'],expected['gzip_sha256'],expected['json_bytes'],expected['json_sha256'])
sys.stdout.buffer.write(raw)
"#,
        name,
    )
}
pub fn validate() {
    python(
        r#"import hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1])
expected=json.loads((root/'docs/foundation-001-012-sha256.json').read_bytes())
previous=json.loads((root/'docs/foundation-001-011-sha256.json').read_bytes())
assert all(expected.get(p)==h for p,h in previous.items()), 'historical inventory changed'
actual={p.relative_to(root).as_posix() for i in range(1,13) for p in (root/f'experiments/{i:03}').rglob('*') if p.is_file()}
assert actual==set(expected), 'frozen001–012 inventory changed'
for name,digest in expected.items():
    assert hashlib.sha256((root/name).read_bytes()).hexdigest()==digest, f'frozen bytes changed: {name}'
"#,
        "",
    );
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
            "--self-test",
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
    let archived: serde_json::Value = serde_json::from_slice(
        &std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/experiments/012/analysis.json"
        ))
        .unwrap(),
    )
    .unwrap();
    assert_eq!(review, archived, "012 independent/frozen analysis mismatch");
    drop(review);
    drop(archived);
    let findings = Command::new("python")
        .args([
            "-B",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/examples/support/review012_findings.py"
            ),
        ])
        .output()
        .expect("Python standard library is required for reviewed 012 findings");
    assert!(
        findings.status.success(),
        "{}",
        String::from_utf8_lossy(&findings.stderr)
    );
    let findings: serde_json::Value = serde_json::from_slice(&findings.stdout).unwrap();
    assert_eq!(findings["independent_trials"], 180);
    assert_eq!(findings["material_anchors"].as_array().unwrap().len(), 8);
    assert_eq!(findings["maximum_observed_learning_cells"], 7);
    assert_eq!(findings["actual_learning_evictions"], 0);
    println!(
        "001–012 frozen bytes;012 all four gzip/raw manifests,1800 exact full reruns and archived summaries;180 independent reconstructions and qualified scientific anchors passed; no files written."
    );
}
