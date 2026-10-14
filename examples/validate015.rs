//! Read-only exploratory015 replay; protected foundation remains001–014.
use std::process::Command;
use world_of_individuals::experiment015 as e;
fn raw(name: &str) -> Vec<u8> {
    let output=Command::new("python").args(["-B","-c","import gzip,pathlib,sys;sys.stdout.buffer.write(gzip.decompress((pathlib.Path(sys.argv[1])/'experiments/015'/(sys.argv[2]+'.json.gz')).read_bytes()))",env!("CARGO_MANIFEST_DIR"),name]).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}
fn main() {
    let root = env!("CARGO_MANIFEST_DIR");
    let inventory=Command::new("python").args(["-B","-c",r#"import hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1]);expected=json.loads((root/'docs/foundation-001-014-sha256.json').read_bytes())
assert len(expected)==168
assert expected==json.loads((root/'experiments/015/frozen-001-014-sha256.json').read_bytes())
for name,h in expected.items():
    data=(root/name).read_bytes()
    if name.startswith(('experiments/013/','experiments/014/')) and not name.endswith('.gz'):data=data.replace(b'\r\n',b'\n')
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
            concat!(env!("CARGO_MANIFEST_DIR"), "/examples/support/review015.py"),
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
            "/experiments/015/analysis.json"
        ))
        .unwrap(),
    )
    .unwrap();
    assert_eq!(actual, expected);
    assert_eq!(actual["independent_trials"], 195);
    println!(
        "015:780 exact full reruns,195 independent reconstructions/195 overlapping comparisons, archive envelopes, three source hashes,168 inherited hashes and corruption/locality/accounting checks pass; read-only, no foundation extension."
    );
}
