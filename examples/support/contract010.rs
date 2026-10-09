//! Read-only protected 010 gate: current execution plus frozen causal reconstruction.
use std::process::Command;
use world_of_individuals::experiment010 as e;

fn python(script: &str, name: &str) -> Vec<u8> {
    let result = Command::new("python")
        .args(["-c", script, env!("CARGO_MANIFEST_DIR"), name])
        .output()
        .expect("Python standard library is required for frozen archive verification");
    assert!(
        result.status.success(),
        "010 archive verification failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    result.stdout
}

pub fn validate() {
    python(
        r#"import hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1])
expected=json.loads((root/'docs/foundation-001-010-sha256.json').read_text())
actual={p.relative_to(root).as_posix() for i in range(1,11) for p in (root/f'experiments/{i:03}').rglob('*') if p.is_file()}
assert actual==set(expected), 'frozen001–010 file inventory changed'
for name,digest in expected.items():
    assert hashlib.sha256((root/name).read_bytes()).hexdigest()==digest, f'frozen bytes changed: {name}'
"#,
        "",
    );
    for (held, name) in [(false, "seed-42"), (true, "held-out-seed-42")] {
        let raw = python(
            r#"import gzip,hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1]); name=sys.argv[2]
expected=json.loads((root/'experiments/010/archive-manifest.json').read_text())[name]
compressed=(root/f'experiments/010/{name}.json.gz').read_bytes()
assert len(compressed)==expected['gzip_bytes'] and hashlib.sha256(compressed).hexdigest()==expected['gzip_sha256']
raw=gzip.decompress(compressed)
assert len(raw)==expected['json_bytes'] and hashlib.sha256(raw).hexdigest()==expected['json_sha256']
sys.stdout.buffer.write(raw)
"#,
            name,
        );
        assert!(
            serde_json::to_vec(&e::suite(42, held)).unwrap() == raw,
            "010 exact current/archive mismatch: {name}"
        );
    }
    for seed in (0..32).chain([42, u64::MAX]) {
        for held in [false, true] {
            // Every trial validates its complete causal snapshot on construction.
            assert_eq!(e::suite(seed, held), e::suite(seed, held));
        }
    }
    let result = Command::new("python")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/examples/support/review010.py"
        ))
        .arg("--archive-only")
        .output()
        .expect("Python standard library is required for 010 causal reconstruction");
    assert!(
        result.status.success(),
        "010 scientific reconstruction failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let review: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(review["archive_integrity_verified"], true);
    assert_eq!(review["archived_trial_rows"], 12240);
    assert_eq!(review["seed42_groups_independently_reconstructed"], 120);
    println!(
        "001–010 frozen bytes;010 exact archives,34-seed replay/causal validation, unchanged-path E_Q and distinct E_A/F reconstruction passed; no files written."
    );
}
