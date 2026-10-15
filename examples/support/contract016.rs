//! Shared protected016 read-only replay and scientific review gate.
use std::process::Command;
use world_of_individuals::experiment016 as e;
fn raw(name: &str) -> Vec<u8> {
    let output=Command::new("python").args(["-B","-c","import gzip,pathlib,sys;sys.stdout.buffer.write(gzip.decompress((pathlib.Path(sys.argv[1])/'experiments/016'/(sys.argv[2]+'.json.gz')).read_bytes()))",env!("CARGO_MANIFEST_DIR"),name]).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}
pub fn validate() {
    let root = env!("CARGO_MANIFEST_DIR");
    let inventory=Command::new("python").args(["-B","-c",r#"import hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1]);expected=json.loads((root/'docs/foundation-001-015-sha256.json').read_bytes())
assert len(expected)==186 and expected==json.loads((root/'experiments/016/frozen-001-015-sha256.json').read_bytes())
current=json.loads((root/'docs/foundation-001-016-sha256.json').read_bytes())
assert len(current)==207 and all(current.get(n)==h for n,h in expected.items())
actual={p.relative_to(root).as_posix() for i in range(1,17) for p in (root/f'experiments/{i:03}').rglob('*') if p.is_file()}
assert actual==set(current)
for name,h in current.items():
 data=(root/name).read_bytes()
 if name.startswith(('experiments/013/','experiments/014/','experiments/015/','experiments/016/')) and not name.endswith('.gz'):data=data.replace(b'\r\n',b'\n')
 assert hashlib.sha256(data).hexdigest()==h,name
for name,h in expected.items():
 data=(root/name).read_bytes()
 if name.startswith(('experiments/013/','experiments/014/','experiments/015/')) and not name.endswith('.gz'):data=data.replace(b'\r\n',b'\n')
 assert hashlib.sha256(data).hexdigest()==h,name
"#,root]).output().unwrap();
    assert!(
        inventory.status.success(),
        "{}",
        String::from_utf8_lossy(&inventory.stderr)
    );
    let mut summaries = vec![];
    for seed in [0, 1, 42, u64::MAX] {
        for held in [false, true] {
            let trials = e::suite(seed, held);
            assert_eq!(trials, e::suite(seed, held));
            if seed == 42 {
                assert_eq!(
                    serde_json::to_vec(&trials).unwrap(),
                    raw(if held { "held-out-seed-42" } else { "seed-42" })
                );
            }
            summaries.extend(trials.iter().map(e::summary));
        }
    }
    assert_eq!(serde_json::to_vec(&summaries).unwrap(), raw("summary"));
    let output = Command::new("python")
        .args([
            "-B",
            concat!(env!("CARGO_MANIFEST_DIR"), "/examples/support/review016.py"),
            "--self-test",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let actual: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let expected: serde_json::Value = serde_json::from_slice(
        &std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/experiments/016/analysis.json"
        ))
        .unwrap(),
    )
    .unwrap();
    assert_eq!(actual, expected);
    assert_eq!(actual["independent_trials"], 250);
    assert_eq!(actual["ordering_contrasts"], 140);
    let mut extracted = vec![];
    for order in [
        world_of_individuals::encounters::Order::Ascending,
        world_of_individuals::encounters::Order::Rotating,
    ] {
        for mode in world_of_individuals::contact_learning::MODES {
            extracted.push(e::trial(
                1,
                mode,
                "decline",
                e::Config {
                    order,
                    ..Default::default()
                },
            ));
        }
    }
    assert_eq!(
        serde_json::to_vec(&extracted).unwrap(),
        raw("seed-1-decline")
    );
    let benefit = Command::new("python")
        .args([
            "-B",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/examples/support/review016_benefit.py"
            ),
        ])
        .output()
        .unwrap();
    assert!(
        benefit.status.success(),
        "{}",
        String::from_utf8_lossy(&benefit.stderr)
    );
    let actual: serde_json::Value = serde_json::from_slice(&benefit.stdout).unwrap();
    let expected: serde_json::Value = serde_json::from_slice(
        &std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/experiments/016/benefit-analysis.json"
        ))
        .unwrap(),
    )
    .unwrap();
    assert_eq!(actual, expected);
    let findings = Command::new("python")
        .args([
            "-B",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/examples/support/review016_findings.py"
            ),
        ])
        .output()
        .unwrap();
    assert!(
        findings.status.success(),
        "{}",
        String::from_utf8_lossy(&findings.stderr)
    );
    let reviewed: serde_json::Value = serde_json::from_slice(&findings.stdout).unwrap();
    assert_eq!(reviewed["histories"], 260);
    assert_eq!(
        reviewed["corruption_categories"].as_array().unwrap().len(),
        9
    );
    println!(
        "016:1000 exact full reruns,250 independently reconstructed histories/500 overlapping within-config comparisons,140 order contrasts,feedback/eligibility/time corruptions, ten supplementary seed1 extractions/independent reconstructions,complementary260-history review/nine corruption categories,source/envelope and207 protected hashes; protected001–016,read-only."
    );
}
