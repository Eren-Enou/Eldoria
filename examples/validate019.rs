//! Read-only019 research replay; does not extend protected001–018.
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};
use world_of_individuals::{experiment019 as f, priorities as r};
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
fn full(name: &str) -> Vec<f::Summary> {
    let mut child=Command::new("python").args(["-B","-c","import gzip,pathlib,sys,shutil;shutil.copyfileobj(gzip.open(pathlib.Path(sys.argv[1])/'experiments/019'/(sys.argv[2]+'.jsonl.gz'),'rb'),sys.stdout.buffer)",env!("CARGO_MANIFEST_DIR"),name]).stdout(Stdio::piped()).spawn().unwrap();
    let mut summaries = vec![];
    let mut initial = None;

    for (index, line) in BufReader::new(child.stdout.take().unwrap())
        .lines()
        .enumerate()
    {
        let line = line.unwrap();
        let expected: f::Trial = serde_json::from_str(&line).unwrap();
        assert_eq!(expected.seed, if name == "seed-1-focus" { 1 } else { 42 });
        if index % 6 == 0 {
            initial = Some((expected.initial.clone(), expected.profiles.clone()));
        } else {
            assert_eq!(
                initial.as_ref().unwrap(),
                &(expected.initial.clone(), expected.profiles.clone())
            );
        }
        let actual = f::trial(
            expected.seed,
            expected.mode,
            &expected.case,
            expected.config.clone(),
        );
        assert!(
            actual == expected,
            "019 archived {} {:?} {:?}",
            expected.case,
            expected.mode,
            expected.config
        );
        assert_eq!(serde_json::to_vec(&actual).unwrap(), line.as_bytes());
        assert!(
            actual
                == f::trial(
                    expected.seed,
                    expected.mode,
                    &expected.case,
                    expected.config.clone()
                ),
            "019 replay"
        );

        summaries.push(f::summary(&actual));
    }
    assert!(child.wait().unwrap().success());
    summaries
}
fn main() {
    python(
        r#"import pathlib,json,hashlib,gzip,sys
r=pathlib.Path(sys.argv[1]);d=r/'experiments/019';m=json.loads((r/'docs/foundation-001-018-sha256.json').read_bytes())
assert len(m)==269 and m==json.loads((d/'frozen-001-018-sha256.json').read_bytes())
for n,h in m.items():
 b=(r/n).read_bytes()
 if n.startswith(tuple(f'experiments/{i:03}/' for i in range(13,19))) and not n.endswith('.gz'):b=b.replace(b'\r\n',b'\n')
 assert hashlib.sha256(b).hexdigest()==h,n
for n,h in json.loads((d/'policy-freeze.json').read_bytes()).items():assert hashlib.sha256((r/n).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==h,n
for n,v in json.loads((d/'archive-manifest.json').read_bytes()).items():
 p=d/n;b=p.read_bytes();h=hashlib.sha256();size=0
 with gzip.open(p,'rb') as f:
  while chunk:=f.read(1048576):h.update(chunk);size+=len(chunk)
 assert (len(b),hashlib.sha256(b).hexdigest(),size,h.hexdigest())==(v['gzip_bytes'],v['gzip_sha256'],v['json_bytes'],v['json_sha256'])
"#,
    );
    let focus = full("seed-1-focus");
    assert_eq!(focus.len(), 12);
    let mut summaries = vec![];
    let mut checked_focus = 0;
    for seed in [0, 1, 42, u64::MAX] {
        if seed == 42 {
            summaries.extend(full("seed-42"));
            summaries.extend(full("held-out-seed-42"));
            continue;
        }
        for held in [false, true] {
            for (config, cases) in f::configurations(held) {
                for case in cases {
                    for mode in r::MODES {
                        let t = f::trial(seed, mode, case, config.clone());
                        assert!(
                            t == f::trial(seed, mode, case, config.clone()),
                            "019 exact replay"
                        );
                        let row = f::summary(&t);
                        if seed == 1 && !held && matches!(case, "history" | "useful_contact") {
                            assert_eq!(&row, &focus[checked_focus]);
                            checked_focus += 1;
                        }
                        summaries.push(row);
                    }
                }
            }
        }
    }
    assert_eq!(summaries.len(), 1296);
    assert_eq!(checked_focus, 12);
    let raw = python(
        "import pathlib,gzip,sys;sys.stdout.buffer.write(gzip.decompress((pathlib.Path(sys.argv[1])/'experiments/019/summary.json.gz').read_bytes()))",
    );
    assert_eq!(serde_json::to_vec(&summaries).unwrap(), raw);
    for (reader, record) in [
        ("review019.py", "analysis.json"),
        ("review019_findings.py", "findings.json"),
    ] {
        let mut cmd = Command::new("python");
        cmd.args(["-B"]).arg(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("examples/support")
                .join(reader),
        );
        if reader == "review019.py" {
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
                    .join("experiments/019")
                    .join(record),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(actual, expected);
    }
    println!(
        "019:1296 exact worlds,336 raw full histories,independent priorities/physics/admission/consent/resource/cognition/gap/pressure-time reconstruction,269 frozen historical hashes;read-only research,no adoption or protection."
    );
}
