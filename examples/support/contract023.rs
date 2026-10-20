//! Read-only023 gate, separate from the protected001-022foundation.
use std::{path::Path, process::Command};
fn main() {
    let directory = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "experiments/023".into());
    assert!(
        Command::new("python")
            .args(["-B", "examples/support/validate023.py", &directory])
            .status()
            .unwrap()
            .success()
    );
    assert!(
        Command::new("cargo")
            .args([
                "run",
                "--release",
                "--locked",
                "-j",
                "1",
                "--manifest-path",
                "prototypes/continuous-world/Cargo.toml",
                "--target-dir",
                "target/continuous-world",
                "--bin",
                "continuous-world",
                "--",
                "replay"
            ])
            .arg(Path::new(&directory).join("trials.jsonl.gz"))
            .status()
            .unwrap()
            .success()
    );
    println!(
        "023 independent reconstruction,19scratch categories and exact replay PASS; no protection/adoption implied"
    );
}
// FOUNDATION023 EXTENSION: original standalone above remains byte-equivalent text.
pub fn validate() {
    main();
    let code = r#"import pathlib,json,hashlib,sys
r=pathlib.Path(sys.argv[1]);m=json.loads((r/'docs/foundation-001-023-sha256.json').read_bytes());old=json.loads((r/'docs/foundation-001-022-sha256.json').read_bytes())
assert len(old)==361 and len(m)==386 and all(m[k]==v for k,v in old.items())
assert {n for n in m if n.startswith('experiments/023/')}=={p.relative_to(r).as_posix() for p in (r/'experiments/023').iterdir() if p.is_file()}
for n,h in m.items():
 b=(r/n).read_bytes()
 if n.startswith(tuple(f'experiments/{i:03}/' for i in range(13,24))) and not n.endswith('.gz'):b=b.replace(b'\r\n',b'\n')
 assert hashlib.sha256(b).hexdigest()==h,n
original=(r/'examples/validate023.rs').read_text(encoding='utf-8').rstrip()
shared=(r/'examples/support/contract023.rs').read_text(encoding='utf-8').split('\n// FOUNDATION023 EXTENSION:')[0]
assert original==shared,'023 frozen standalone/shared replay logic drift'
raw=json.loads((r/'docs/development/activity-valuation-original-working-sha256.json').read_bytes())
assert len(raw)==403
for n,h in raw.items():assert hashlib.sha256((r/n).read_bytes()).hexdigest()==h,n
analysis=json.loads((r/'experiments/023/analysis-manifest.json').read_bytes())
for section in ('helpers_sha256','derived_sha256'):
 for n,h in analysis[section].items():assert hashlib.sha256((r/n).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==h,n
assert hashlib.sha256((r/'examples/support/review023_findings.py').read_bytes().replace(b'\r\n',b'\n')).hexdigest()=='b30a133766e97fdb36d35be78c445d039a3230a4b00e70e1294aac667f88e457','023 independent review identity'
assert hashlib.sha256((r/'docs/development/experiment-023-review-findings.json').read_bytes().replace(b'\r\n',b'\n')).hexdigest()=='5d272cb2f519ef779f6d3568f2aef9144979e752771e775642b13ed370128667','023 independent review identity'
print('001-023:386 original artifacts;361 inherited entries;403 raw originals;standalone/shared and analysis/review identity PASS')
"#;
    let status = Command::new("python")
        .args(["-B", "-c", code, env!("CARGO_MANIFEST_DIR")])
        .status()
        .unwrap();
    assert!(status.success(), "023 foundation inventory/parity");
    for args in [
        vec![
            "examples/support/review023_findings.py",
            "--expected",
            "docs/development/experiment-023-review-findings.json",
        ],
        vec!["examples/support/slices023.py", "--verify"],
        vec!["examples/support/coverage023.py", "--verify"],
        vec!["examples/support/review023_endpoints.py", "--verify"],
        vec!["examples/support/review023_inquiry.py", "--verify"],
    ] {
        let status = Command::new("python")
            .arg("-B")
            .args(args)
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .unwrap();
        assert!(status.success(), "023 independent review/causal controls");
    }
}
