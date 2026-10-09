//! Shared read-only009 gate; Python standard library reads gzip/SHA-256 only.
use std::{collections::BTreeMap, fs, path::Path, process::Command};
use world_of_individuals::{assessment, experiment009 as e, retention};

fn python(script: &str, name: &str) -> Vec<u8> {
    let output = Command::new("python")
        .args(["-c", script, env!("CARGO_MANIFEST_DIR"), name])
        .output()
        .expect("Python standard library is required to verify frozen gzip archives");
    assert!(
        output.status.success(),
        "read-only archive verification failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

pub fn validate() {
    python(
        r#"import hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1])
expected=json.loads((root/'docs/foundation-001-009-sha256.json').read_text())
actual={p.relative_to(root).as_posix() for i in range(1,10) for p in (root/f'experiments/{i:03}').rglob('*') if p.is_file()}
assert actual==set(expected), 'frozen001–009 file inventory changed'
for name,digest in expected.items():
    assert hashlib.sha256((root/name).read_bytes()).hexdigest()==digest, f'frozen bytes changed: {name}'
"#,
        "",
    );
    for (held, name) in [(false, "seed-42"), (true, "held-out-seed-42")] {
        let raw = python(
            r#"import gzip,hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1]); name=sys.argv[2]
expected=json.loads((root/'experiments/009/archive-manifest.json').read_text())[name]
compressed=(root/f'experiments/009/{name}.json.gz').read_bytes()
assert len(compressed)==expected['gzip_bytes'] and hashlib.sha256(compressed).hexdigest()==expected['gzip_sha256'], 'gzip integrity mismatch'
raw=gzip.decompress(compressed)
assert len(raw)==expected['json_bytes'] and hashlib.sha256(raw).hexdigest()==expected['json_sha256'], 'decompressed integrity mismatch'
sys.stdout.buffer.write(raw)
"#,
            name,
        );
        let actual = e::suite(42, held);
        for trial in &actual {
            trial.initial.validate().unwrap();
            trial.final_state.validate().unwrap();
        }
        // Preserve exact frozen fields/serialization, without lossy typed archive
        // deserialization or two enormous serde_json::Value trees.
        assert!(
            serde_json::to_vec_pretty(&actual).unwrap() == raw,
            "009 exact archived JSON mismatch: {name}"
        );
    }
    let mut outcomes = BTreeMap::<String, usize>::new();
    for seed in (0..128).chain([u64::MAX]) {
        for held in [false, true] {
            let trials = e::suite(seed, held);
            assert_eq!(trials, e::suite(seed, held));
            for trial in trials {
                trial.final_state.validate().unwrap();
                if seed != u64::MAX {
                    let p = trial.points.last().unwrap();
                    let key = format!(
                        "held={held}:{}:{:?}:{:?}:{}:{}:{}:{:?}",
                        trial.scenario,
                        trial.config,
                        trial.method,
                        p.basis_support,
                        p.target_origins,
                        p.positive && p.negative,
                        p.action
                    );
                    *outcomes.entry(key).or_default() += 1;
                }
            }
        }
    }
    let summary: serde_json::Value = serde_json::from_slice(
        &fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("experiments/009/summary.json"))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(summary["rules"], retention::RULES);
    assert_eq!(summary["capacity"], assessment::CAPACITY);
    assert_eq!(serde_json::to_value(outcomes).unwrap(), summary["outcomes"]);
    println!(
        "001–009 frozen bytes;009 exact gzip/raw archives and all archived outcome counts;129 seeds replay/causal validation passed. No files written;010 excluded."
    );
}
