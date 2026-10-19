//! Cache-free, read-only 022 source/archive/primitive/corruption/exact-byte gate.
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};
use world_of_individuals::experiment::concurrent as x;
fn main() {
    let directory = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "experiments/022".into());
    assert!(
        Command::new("python")
            .args(["-B", "examples/support/validate022.py", &directory])
            .status()
            .unwrap()
            .success()
    );
    let path = std::path::Path::new(&directory).join("trials.jsonl.gz");
    let mut child=Command::new("python").args(["-B","-c","import gzip,sys;f=gzip.open(sys.argv[1],'rb');o=sys.stdout.buffer;[(o.write(l)) for l in f]"]).arg(path).stdout(Stdio::piped()).spawn().unwrap();
    let stream = BufReader::new(child.stdout.take().unwrap());
    let matrix = x::matrix();
    let mut count = 0;
    for (n, line) in stream.lines().enumerate() {
        let line = line.unwrap();
        let archived: x::Trial = serde_json::from_str(&line).unwrap();
        assert_eq!(archived.config, matrix[n]);
        let replay = x::trial(archived.config.clone());
        assert_eq!(
            serde_json::to_string(&replay).unwrap(),
            line,
            "022 byte replay {n}"
        );
        count += 1;
    }
    assert!(child.wait().unwrap().success());
    assert_eq!(count, matrix.len());
    println!(
        "022: {count} original worlds exact byte replay; independent primitive/corruption checks passed"
    );
}
// FOUNDATION022 EXTENSION: original standalone above remains byte-equivalent text.
pub fn validate() {
    main();
    let code = r#"import pathlib,json,hashlib,sys
r=pathlib.Path(sys.argv[1]);m=json.loads((r/'docs/foundation-001-022-sha256.json').read_bytes());old=json.loads((r/'docs/foundation-001-021-sha256.json').read_bytes())
assert len(old)==341 and len(m)==361 and all(m[k]==v for k,v in old.items())
assert {n for n in m if n.startswith('experiments/022/')}=={p.relative_to(r).as_posix() for p in (r/'experiments/022').iterdir() if p.is_file()}
for n,h in m.items():
 b=(r/n).read_bytes()
 if n.startswith(tuple(f'experiments/{i:03}/' for i in range(13,23))) and not n.endswith('.gz'):b=b.replace(b'\r\n',b'\n')
 assert hashlib.sha256(b).hexdigest()==h,n
original=(r/'examples/validate022.rs').read_text(encoding='utf-8').rstrip()
shared=(r/'examples/support/contract022.rs').read_text(encoding='utf-8').split('\n// FOUNDATION022 EXTENSION:')[0]
assert original==shared,'022 frozen standalone/shared replay logic drift'
assert hashlib.sha256((r/'examples/support/review022_findings.py').read_bytes().replace(b'\r\n',b'\n')).hexdigest()=='22957e6c8e937f0888e7de717f28698315078871a6956aba4d3ebef3c7ef9519','022 independent review reader changed'
assert hashlib.sha256((r/'docs/development/experiment-022-review-findings.json').read_bytes().replace(b'\r\n',b'\n')).hexdigest()=='e377ea9f801803d3dd8e6f6f96c9fe9b28f0c86f2eee192427eb41b4a668c105','022 independently checked anchors changed'
print('001-022:361 original artifacts;341 inherited hashes;standalone/shared parity;review source identities PASS')
"#;
    let status = Command::new("python")
        .args(["-B", "-c", code, env!("CARGO_MANIFEST_DIR")])
        .status()
        .unwrap();
    assert!(status.success(), "022 foundation inventory/parity");
    for args in [
        vec![
            "examples/support/review022_findings.py",
            "--expected",
            "docs/development/experiment-022-review-findings.json",
        ],
        vec!["examples/support/review022_coverage.py", "--verify"],
    ] {
        let status = Command::new("python")
            .arg("-B")
            .args(args)
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .unwrap();
        assert!(status.success(), "022 complementary independent review");
    }
}
