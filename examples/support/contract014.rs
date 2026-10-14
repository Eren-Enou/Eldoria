//! Shared read-only014 protected replay and scientific gate.
use std::process::Command;
use world_of_individuals::experiment014 as e;

fn inventory() {
    let script = r#"import hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1])
previous=json.loads((root/'docs/foundation-001-013-sha256.json').read_bytes())
expected=json.loads((root/'docs/foundation-001-014-sha256.json').read_bytes())
assert len(previous)==151 and len(expected)==168
assert all(expected.get(p)==h for p,h in previous.items()), 'inherited hashes changed'
actual={p.relative_to(root).as_posix() for i in range(1,15) for p in (root/f'experiments/{i:03}').rglob('*') if p.is_file()}
assert actual==set(expected), 'frozen001–014 inventory changed'
def canonical(name,data):
    return data.replace(b'\r\n',b'\n') if name.startswith(('experiments/013/','experiments/014/')) and not name.endswith('.gz') else data
for name,digest in expected.items():
    assert hashlib.sha256(canonical(name,(root/name).read_bytes())).hexdigest()==digest, 'frozen evidence changed: '+name
assert canonical('experiments/014/results.md',b'a\r\n')==b'a\n'
assert canonical('experiments/012/results.md',b'a\r\n')==b'a\r\n'
assert canonical('experiments/014/seed-42.json.gz',b'a\r\n')==b'a\r\n'
assert hashlib.sha256(canonical('experiments/014/results.md',b'changed\r\n')).digest()!=hashlib.sha256(b'a\n').digest()
"#;
    let result = Command::new("python")
        .args(["-B", "-c", script, env!("CARGO_MANIFEST_DIR")])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

fn raw(name: &str) -> Vec<u8> {
    let out=Command::new("python").args(["-B","-c","import gzip,pathlib,sys;sys.stdout.buffer.write(gzip.decompress((pathlib.Path(sys.argv[1])/'experiments/014'/(sys.argv[2]+'.json.gz')).read_bytes()))",env!("CARGO_MANIFEST_DIR"),name]).output().unwrap();
    assert!(out.status.success());
    out.stdout
}
pub fn validate() {
    inventory();
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
    let findings = Command::new("python")
        .args([
            "-B",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/examples/support/review014_findings.py"
            ),
        ])
        .output()
        .unwrap();
    assert!(
        findings.status.success(),
        "{}",
        String::from_utf8_lossy(&findings.stderr)
    );
    let findings: serde_json::Value = serde_json::from_slice(&findings.stdout).unwrap();
    assert_eq!(findings["verdict"], "ACCEPT WITH QUALIFICATION");
    assert_eq!(findings["primary"], 210);
    assert_eq!(findings["diagnostic"], 5);
    assert_eq!(findings["positive_shared"], true);
    assert_eq!(findings["reservation_counterfactual_unexecuted"], true);
    println!(
        "014:840 exact full replays plus five13-tick diagnostics, archived raw/summary equality,215 independent cases,430 comparisons, corruption and mirrored-locality checks; protected001–014; no files written."
    );
}
