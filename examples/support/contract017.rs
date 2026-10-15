//! Shared protected017 read-only replay and scientific review gate.
use std::process::Command;
use world_of_individuals::experiment017 as e;
fn raw(name: &str) -> Vec<u8> {
    let out=Command::new("python").args(["-B","-c","import gzip,pathlib,sys;sys.stdout.buffer.write(gzip.decompress((pathlib.Path(sys.argv[1])/'experiments/017'/(sys.argv[2]+'.json.gz')).read_bytes()))",env!("CARGO_MANIFEST_DIR"),name]).output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    out.stdout
}
pub fn validate() {
    let root = env!("CARGO_MANIFEST_DIR");
    let out=Command::new("python").args(["-B","-c",r#"import pathlib,json,hashlib,gzip,sys
r=pathlib.Path(sys.argv[1]);d=r/'experiments/017';m=json.loads((r/'docs/foundation-001-016-sha256.json').read_bytes())
assert len(m)==207 and m==json.loads((d/'frozen-001-016-sha256.json').read_bytes())
current=json.loads((r/'docs/foundation-001-017-sha256.json').read_bytes())
assert len(current)==228 and all(current.get(n)==h for n,h in m.items())
actual={p.relative_to(r).as_posix() for i in range(1,18) for p in (r/f'experiments/{i:03}').rglob('*') if p.is_file()}
assert actual==set(current)
for n,h in current.items():
 b=(r/n).read_bytes()
 if n.startswith(tuple(f'experiments/{i:03}/' for i in range(13,18))) and not n.endswith('.gz'):b=b.replace(b'\r\n',b'\n')
 assert hashlib.sha256(b).hexdigest()==h,n

for n,h in m.items():
 b=(r/n).read_bytes()
 if n.startswith(tuple(f'experiments/{i:03}/' for i in range(13,17))) and not n.endswith('.gz'):b=b.replace(b'\r\n',b'\n')
 assert hashlib.sha256(b).hexdigest()==h,n
for n,h in json.loads((d/'policy-freeze.json').read_bytes()).items():assert hashlib.sha256((r/n).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==h,n
for n,m in json.loads((d/'archive-manifest.json').read_bytes()).items():
 b=(d/(n+'.json.gz')).read_bytes();v=gzip.decompress(b)
 assert (len(b),hashlib.sha256(b).hexdigest(),len(v),hashlib.sha256(v).hexdigest())==(m['gzip_bytes'],m['gzip_sha256'],m['json_bytes'],m['json_sha256'])
"#,root]).output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let mut summaries = vec![];
    let mut null_pairs = 0;
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
            for group in trials.as_chunks::<5>().0 {
                assert!(
                    group
                        .iter()
                        .all(|t| t.initial == group[0].initial && t.profiles == group[0].profiles)
                );
                assert_eq!(group[0].final_state, group[1].final_state);
                null_pairs += 1;
            }
            summaries.extend(trials.iter().map(e::summary));
        }
    }
    assert_eq!(null_pairs, 188);
    assert_eq!(serde_json::to_vec(&summaries).unwrap(), raw("summary"));
    for (reader, expected) in [
        ("review017.py", "analysis.json"),
        ("review017_findings.py", "findings.json"),
    ] {
        let script = std::path::Path::new(root)
            .join("examples/support")
            .join(reader);
        let mut cmd = Command::new("python");
        cmd.args(["-B", script.to_str().unwrap()]);
        if reader == "review017.py" {
            cmd.arg("--self-test");
        }
        let out = cmd.output().unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let actual: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
        let expected: serde_json::Value = serde_json::from_slice(
            &std::fs::read(
                std::path::Path::new(root)
                    .join("experiments/017")
                    .join(expected),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(actual, expected);
    }
    let scope = Command::new("python")
        .args([
            "-B",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/examples/support/review017_scope.py"
            ),
        ])
        .output()
        .unwrap();
    assert!(
        scope.status.success(),
        "{}",
        String::from_utf8_lossy(&scope.stderr)
    );
    let scope: serde_json::Value = serde_json::from_slice(&scope.stdout).unwrap();
    assert_eq!(scope["summary_worlds"], 940);
    assert_eq!(scope["active_nohistory_material_changes"], 34);
    println!(
        "017:940 exact full reruns/raw archive equality,188 full-state Serial/Committed null controls,235 independent histories/47 matched groups/470 overlapping pairs/130 order controls,seven corruptions,four independent causal slices,complementary scope/order/private-twin checks,source/envelope/228 protected hashes;read-only,protected with qualification,no adoption."
    );
}
