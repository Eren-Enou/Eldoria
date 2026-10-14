//! Shared read-only 013 protection; no simulation behavior or evidence changes.
use std::process::Command;
use world_of_individuals::experiment013 as e;
fn inventory() {
    let script = r#"import hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1])
expected=json.loads((root/'docs/foundation-001-013-sha256.json').read_bytes())
previous=json.loads((root/'docs/foundation-001-012-sha256.json').read_bytes())
assert len(previous)==135 and len(expected)==151
assert all(expected.get(p)==h for p,h in previous.items()), 'inherited hashes changed'
actual={p.relative_to(root).as_posix() for i in range(1,14) for p in (root/f'experiments/{i:03}').rglob('*') if p.is_file()}
assert actual==set(expected), 'frozen001–013 inventory changed'
def canonical(name, data):
    return data.replace(b'\r\n',b'\n') if name.startswith('experiments/013/') and not name.endswith('.gz') else data
for name,digest in expected.items():
    data=(root/name).read_bytes()
    assert hashlib.sha256(canonical(name,data)).hexdigest()==digest, f'frozen evidence changed: {name}'
for name,entry in json.loads((root/'experiments/013/archive-manifest.json').read_bytes()).items():
    import gzip
    compressed=(root/f'experiments/013/{name}.json.gz').read_bytes();raw=gzip.decompress(compressed)
    assert (len(compressed),hashlib.sha256(compressed).hexdigest(),len(raw),hashlib.sha256(raw).hexdigest())==(entry['gzip_bytes'],entry['gzip_sha256'],entry['json_bytes'],entry['json_sha256'])
# Narrow integrity self-checks: tolerate text checkout line endings only, not content
# or binary corruption. These in-memory samples never touch historical evidence.
assert canonical('experiments/013/report.txt',b'a\r\nb\r\n')==b'a\nb\n'
assert canonical('experiments/012/report.txt',b'a\r\n')==b'a\r\n'
assert canonical('experiments/013/seed-42.json.gz',b'a\r\n')==b'a\r\n'
assert hashlib.sha256(canonical('experiments/013/report.txt',b'changed\r\n')).digest()!=hashlib.sha256(b'a\n').digest()
"#;
    let result = Command::new("python")
        .args(["-B", "-c", script, env!("CARGO_MANIFEST_DIR")])
        .output()
        .expect("Python standard library is required for frozen 013 verification");
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}
fn raw(name: &str) -> Vec<u8> {
    let result=Command::new("python").args(["-B","-c","import pathlib,gzip,sys; p=pathlib.Path(sys.argv[1])/'experiments/013'; sys.stdout.buffer.write(gzip.decompress((p/(sys.argv[2]+'.json.gz')).read_bytes()))",env!("CARGO_MANIFEST_DIR"),name]).output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    result.stdout
}
pub fn validate() {
    inventory();
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
    let result = Command::new("python")
        .args([
            "-B",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/examples/support/review013_findings.py"
            ),
        ])
        .output()
        .expect("Python standard library is required for reviewed 013 corrections");
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let findings: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(findings["protected_hashes"], 135);
    assert_eq!(findings["trials"], 195);
    assert_eq!(findings["corruptions_rejected"], 3);
    let positive = &findings["positive_decision"];
    assert_eq!(positive["inquiry"], 18);
    assert_eq!(positive["outcome"]["support_before"], 20);
    assert_eq!(positive["outcome"]["support_after"], 60);
    assert_eq!(positive["outcome"]["status_after"], "Resolved");
    assert_eq!(positive["candidates"][0]["score"], 40);
    assert_eq!(positive["candidates"][0]["without_reconsideration"], -8);
    let rows = findings["findings"].as_array().unwrap();
    let row = |case: &str, support: i32, mode: &str| {
        rows.iter()
            .find(|r| r["case"] == case && r["support"] == support && r["mode"] == mode)
            .unwrap()
    };
    let pulse = row("later_useful", 20, "Pulse");
    let contextual = row("later_useful", 20, "Contextual");
    assert_eq!(
        pulse["scenes"][0]["actions"],
        serde_json::json!(["Offer", "Accept"])
    );
    assert_eq!(pulse["scenes"][0]["offer_score"], 5);
    assert_eq!(
        contextual["scenes"][0]["actions"],
        serde_json::json!(["Leave"])
    );
    assert_eq!(contextual["scenes"][0]["offer_score"], -29);
    assert_eq!(
        pulse["scenes"][1]["food"],
        serde_json::json!([1, 6, 4, 4, 4, 4, 4])
    );
    assert_eq!(
        contextual["scenes"][1]["food"],
        serde_json::json!([2, 5, 4, 4, 4, 4, 4])
    );
    assert_eq!(
        row("useful_offer", 20, "Ungated")["scenes"][1]["food"],
        serde_json::json!([3, 4, 4, 4, 4, 4, 4])
    );
    for mode in ["Contextual", "Pulse"] {
        let changes = row("longitudinal", 40, mode)["discoveries"]
            .as_array()
            .unwrap();
        assert!(
            changes.iter().any(
                |r| r["selected"]["Ask"]["concern"] == 2 && r["outcome"]["support_after"] == 64
            )
        );
        assert!(!changes.iter().any(|r| r["selected"]["Ask"]["concern"] == 1));
    }
    let totals = findings["totals"].as_array().unwrap();
    let total = |held: bool, mode: &str| {
        totals
            .iter()
            .find(|r| r["held"] == held && r["mode"] == mode)
            .unwrap()
    };
    assert_eq!(total(false, "Pulse")["raised_no_change"], 111);
    assert_eq!(total(true, "Pulse")["raised_no_change"], 373);
    assert_eq!(total(false, "Pulse")["asks"], 223);
    assert_eq!(total(false, "Contextual")["asks"], 113);
    assert_eq!(total(false, "Ungated")["asks"], 336);
    assert_eq!(total(false, "Pulse")["time"], 3457);
    assert_eq!(total(true, "Pulse")["time"], 8936);
    let diagnostics = findings["diagnostics"].as_array().unwrap();
    assert_eq!(diagnostics.len(), 4);
    for diagnostic in diagnostics {
        let expected = if diagnostic["seed"] == 1 {
            (-28, -8, "Leave")
        } else {
            (-17, 3, "Offer")
        };
        assert_eq!(diagnostic["final"]["trust"], expected.0);
        assert_eq!(diagnostic["final"]["offer_score"], expected.1);
        assert_eq!(diagnostic["final"]["actions"][0], expected.2);
        assert_eq!(diagnostic["final"]["episodic_valence"], 0);
        // The independent reader asserts the old negative episode remains retained.
    }
    println!(
        "013:1170 exact full reruns, all archives/summary,195 independent seed42 reconstructions and corruption/paired-locality checks; both independent readers and reviewed anchors;001–013 frozen evidence; no files written."
    );
}
