//! Shared read-only001–018 archive/replay and qualified scientific enforcement.
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};
use world_of_individuals::{experiment018 as f, recurring as r};
fn python(code: &str) -> Vec<u8> {
    let out = Command::new("python")
        .args(["-B", "-c", code, env!("CARGO_MANIFEST_DIR")])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    out.stdout
}
fn null_control(
    t: &f::Trial,
    prior: &mut Option<world_of_individuals::experiment013::Snapshot>,
    count: &mut usize,
) {
    if t.case == "no_food" {
        if t.mode == r::Mode::Recurring {
            *prior = Some(t.final_state.clone());
        }
        if t.mode == r::Mode::Serial {
            assert_eq!(prior.as_ref().unwrap(), &t.final_state);
            *count += 1;
        }
    }
}
fn full(name: &str, nulls: &mut usize) -> Vec<f::Summary> {
    let mut child=Command::new("python").args(["-B","-c","import gzip,pathlib,sys,shutil;shutil.copyfileobj(gzip.open(pathlib.Path(sys.argv[1])/'experiments/018'/(sys.argv[2]+'.jsonl.gz'),'rb'),sys.stdout.buffer)",env!("CARGO_MANIFEST_DIR"),name]).stdout(Stdio::piped()).spawn().unwrap();
    let mut summaries = vec![];
    let mut initial = None;
    let mut nofood = None;
    for (index, line) in BufReader::new(child.stdout.take().unwrap())
        .lines()
        .enumerate()
    {
        let line = line.unwrap();
        let expected: f::Trial = serde_json::from_str(&line).unwrap();
        assert_eq!(expected.seed, 42);
        if index % 7 == 0 {
            initial = Some((expected.initial.clone(), expected.profiles.clone()));
        } else {
            assert_eq!(
                initial.as_ref().unwrap(),
                &(expected.initial.clone(), expected.profiles.clone())
            );
        }
        let actual = f::trial(42, expected.mode, &expected.case, expected.config.clone());
        assert!(
            actual == expected,
            "018 archived {} {:?} {:?}",
            expected.case,
            expected.mode,
            expected.config
        );
        assert_eq!(serde_json::to_vec(&actual).unwrap(), line.as_bytes());
        assert!(
            actual == f::trial(42, expected.mode, &expected.case, expected.config.clone()),
            "018 replay"
        );
        null_control(&actual, &mut nofood, nulls);
        summaries.push(f::summary(&actual));
    }
    assert!(child.wait().unwrap().success());
    summaries
}
pub fn validate() {
    python(
        r#"import pathlib,json,hashlib,gzip,sys,re
r=pathlib.Path(sys.argv[1]);d=r/'experiments/018';dev=d/'development/initial-clock-hook'
parent=json.loads((r/'docs/foundation-001-017-sha256.json').read_bytes());frozen=json.loads((r/'docs/foundation-001-018-sha256.json').read_bytes())
assert len(parent)==228 and len(frozen)==269 and all(frozen[n]==h for n,h in parent.items())
assert {p.relative_to(r).as_posix() for p in d.rglob('*') if p.is_file()}==set(frozen)-set(parent)
for n,h in frozen.items():
 b=(r/n).read_bytes()
 if n.startswith(tuple(f'experiments/{i:03}/' for i in range(13,19))) and not n.endswith('.gz'):b=b.replace(b'\r\n',b'\n')
 assert hashlib.sha256(b).hexdigest()==h,n
candidate=json.loads((dev/'candidate-sha256.json').read_bytes());assert {p.name for p in dev.iterdir() if p.is_file()}==set(candidate)|{'candidate-sha256.json'}
for n,h in candidate.items():
 b=(dev/n).read_bytes()
 if n.endswith('.gz'):assert hashlib.sha256(b).hexdigest()==h,n
 else:
  canonical=b.replace(b'\r\n',b'\n')
  assert h in (hashlib.sha256(canonical).hexdigest(),hashlib.sha256(canonical.replace(b'\n',b'\r\n')).hexdigest()),n
m=json.loads((r/'docs/foundation-001-017-sha256.json').read_bytes());assert len(m)==228 and m==json.loads((d/'frozen-001-017-sha256.json').read_bytes())
for n,h in m.items():
 b=(r/n).read_bytes()
 if n.startswith(tuple(f'experiments/{i:03}/' for i in range(13,18))) and not n.endswith('.gz'):b=b.replace(b'\r\n',b'\n')
 assert hashlib.sha256(b).hexdigest()==h,n
for n,h in json.loads((d/'policy-freeze.json').read_bytes()).items():assert hashlib.sha256((r/n).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==h,n
for n,v in json.loads((d/'clock-hooks.json').read_bytes()).items():
 s=(r/n).read_text(encoding='utf-8');assert s.count('self.sync_recurring();')==v['self_hooks'];assert s.count('recurring::sync(world, runtime.tick);')==v['resource_hooks']
for n,v in json.loads((d/'archive-manifest.json').read_bytes()).items():
 b=(d/n).read_bytes();h=hashlib.sha256();size=0
 with gzip.open(d/n,'rb') as f:
  while chunk:=f.read(1048576):h.update(chunk);size+=len(chunk)
 assert (len(b),hashlib.sha256(b).hexdigest(),size,h.hexdigest())==(v['gzip_bytes'],v['gzip_sha256'],v['json_bytes'],v['json_sha256'])
"#,
    );
    let mut summaries = vec![];
    let mut nulls = 0;
    for seed in [0, 1, 42, u64::MAX] {
        let mut nofood = None;
        if seed == 42 {
            summaries.extend(full("seed-42", &mut nulls));
            summaries.extend(full("held-out-seed-42", &mut nulls));
            continue;
        }
        for held in [false, true] {
            for (config, cases) in f::configurations(held) {
                for case in cases {
                    for mode in r::MODES {
                        let t = f::trial(seed, mode, case, config.clone());
                        assert!(
                            t == f::trial(seed, mode, case, config.clone()),
                            "018 replay"
                        );
                        null_control(&t, &mut nofood, &mut nulls);
                        summaries.push(f::summary(&t));
                    }
                }
            }
        }
    }
    assert_eq!(summaries.len(), 1064);
    assert_eq!(nulls, 4);
    let raw = python(
        "import pathlib,gzip,sys;sys.stdout.buffer.write(gzip.decompress((pathlib.Path(sys.argv[1])/'experiments/018/summary.json.gz').read_bytes()))",
    );
    assert_eq!(serde_json::to_vec(&summaries).unwrap(), raw);
    for (reader, expected) in [
        ("review018.py", "analysis.json"),
        ("review018_findings.py", "findings.json"),
    ] {
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/support")
            .join(reader);
        let mut cmd = Command::new("python");
        cmd.args(["-B", script.to_str().unwrap()]);
        if reader == "review018.py" {
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
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("experiments/018")
                    .join(expected),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(actual, expected);
    }
    let out = Command::new("python")
        .args(["-B"])
        .arg(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("examples/support/review018_scope.py"),
        )
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let scope: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(scope["counts"]["histories"], 266);
    println!(
        "018:1064 exact worlds;266 raw histories;three independent readers,physics,locality,consent,resource,cognition,invalid-candidate rejection and qualified causal/null/reconvergence findings;269 frozen artifacts;read-only,protected with qualification,no adoption."
    );
}
