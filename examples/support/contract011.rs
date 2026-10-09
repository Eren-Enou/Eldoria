//! Read-only 011 protection: exact current output and qualified reconstruction.
use std::process::Command;
use world_of_individuals::experiment011 as e;

#[path = "summary011.rs"]
mod summary011;

fn python(script: &str, name: &str) -> Vec<u8> {
    let result = Command::new("python")
        .args(["-B", "-c", script, env!("CARGO_MANIFEST_DIR"), name])
        .output()
        .expect("Python standard library is required for frozen 011 verification");
    assert!(
        result.status.success(),
        "011 archive verification failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    result.stdout
}

fn archive(name: &str) -> Vec<u8> {
    python(
        r#"import gzip,hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1]); name=sys.argv[2]
expected=json.loads((root/'experiments/011/archive-manifest.json').read_text())[name]
compressed=(root/f'experiments/011/{name}.json.gz').read_bytes()
assert len(compressed)==expected['gzip_bytes'] and hashlib.sha256(compressed).hexdigest()==expected['gzip_sha256']
raw=gzip.decompress(compressed)
assert len(raw)==expected['json_bytes'] and hashlib.sha256(raw).hexdigest()==expected['json_sha256']
sys.stdout.buffer.write(raw)
"#,
        name,
    )
}

pub fn validate() {
    python(
        r#"import hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1])
expected=json.loads((root/'docs/foundation-001-011-sha256.json').read_text())
previous=json.loads((root/'docs/foundation-001-010-sha256.json').read_text())
assert all(expected.get(p)==h for p,h in previous.items()), 'historical inventory changed'
actual={p.relative_to(root).as_posix() for i in range(1,12) for p in (root/f'experiments/{i:03}').rglob('*') if p.is_file()}
assert actual==set(expected), 'frozen001–011 inventory changed'
for name,digest in expected.items():
    assert hashlib.sha256((root/name).read_bytes()).hexdigest()==digest, f'frozen bytes changed: {name}'
"#,
        "",
    );
    let mut rows = Vec::new();
    for seed in (0..16).chain([42, u64::MAX]) {
        for held in [false, true] {
            // Each constructed trial validates its full causal snapshot.
            let trials = e::suite(seed, held);
            assert_eq!(trials, e::suite(seed, held));
            if seed == 42 {
                let name = if held { "held-out-seed-42" } else { "seed-42" };
                assert!(
                    serde_json::to_vec(&trials).unwrap() == archive(name),
                    "011 exact current/archive mismatch: {name}"
                );
            }
            rows.extend(trials.iter().map(|t| summary011::summary(t, held)));
        }
    }
    assert!(
        serde_json::to_vec(&rows).unwrap() == archive("summary"),
        "011 exact all-seed summary mismatch"
    );
    drop(rows);
    // Keep the independent reconstruction separate from runtime validation.
    let result = Command::new("python")
        .args([
            "-B",
            concat!(env!("CARGO_MANIFEST_DIR"), "/examples/support/review011.py"),
            "--verify-finalized",
        ])
        .output()
        .expect("Python standard library is required for 011 reconstruction");
    assert!(
        result.status.success(),
        "011 scientific reconstruction failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let review: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(review["finalized_consistency_verified"], true);
    assert_eq!(review["reconstructed_trials"], 240);
    assert_eq!(review["archived_trial_rows"], 4320);
    println!(
        "001–011 frozen bytes;011 all three exact archives,18-seed replay/causal validation and qualified negative/positive reconstruction passed; no files written."
    );
}
