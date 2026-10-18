//! Standalone021 read-only research gate; it does not extend the protected foundation.
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};
use world_of_individuals::experiment021 as f;
fn python(code: &str) {
    let out = Command::new("python")
        .args(["-B", "-c", code, env!("CARGO_MANIFEST_DIR")])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    print!("{}", String::from_utf8_lossy(&out.stdout));
}
fn main() {
    python(
        r#"import pathlib,json,hashlib,gzip,sys
r=pathlib.Path(sys.argv[1]);d=r/'experiments/021'
for n,h in json.loads((d/'source-freeze.json').read_bytes()).items():assert hashlib.sha256((r/n).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==h,n
for n,v in json.loads((d/'archive-manifest.json').read_bytes()).items():
 b=(d/n).read_bytes();raw=gzip.decompress(b)
 assert (len(b),hashlib.sha256(b).hexdigest(),len(raw),hashlib.sha256(raw).hexdigest())==(v['gzip_bytes'],v['gzip_sha256'],v['json_bytes'],v['json_sha256']),n
import subprocess
subprocess.run([sys.executable,'-B','examples/support/compare021.py','--analysis','experiments/021/analysis.json.gz','--expected','experiments/021/comparisons.json.gz'],cwd=r,check=True)
subprocess.run([sys.executable,'-B','examples/support/findings021.py'],cwd=r,check=True)
print('021 source/archive hashes and scientific anchors PASS;foundation remains001–020')
"#,
    );
    let mut child=Command::new("python").args(["-B","-c","import gzip,pathlib,sys,shutil;shutil.copyfileobj(gzip.open(pathlib.Path(sys.argv[1])/'experiments/021/trials.jsonl.gz','rb'),sys.stdout.buffer)",env!("CARGO_MANIFEST_DIR")]).stdout(Stdio::piped()).spawn().unwrap();
    let matrix = f::matrix();
    let mut count = 0;
    for (id, line) in BufReader::new(child.stdout.take().unwrap())
        .lines()
        .enumerate()
    {
        let line = line.unwrap();
        let expected: f::Trial = serde_json::from_str(&line).unwrap();
        let row = &matrix[id];
        let (seed, mode, case, config) = (&row.seed, &row.mode, row.case.as_str(), &row.config);
        assert_eq!(
            (
                expected.seed,
                expected.mode,
                expected.case.as_str(),
                &expected.config
            ),
            (*seed, *mode, case, config)
        );
        let actual = f::trial(*seed, row.policy, *mode, case, config.clone());
        assert!(actual == expected, "021 archived trial {id}");
        assert!(
            serde_json::to_vec(&actual).unwrap() == line.as_bytes(),
            "021 byte replay {id}"
        );
        count += 1;
    }
    assert!(child.wait().unwrap().success());
    assert_eq!(count, 214);
    let status = Command::new("python")
        .args([
            "-B",
            "examples/support/review021.py",
            "--corruptions",
            "--expected",
            "experiments/021/analysis.json.gz",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .unwrap();
    assert!(status.success());
    println!("021:214 exact original byte replays;independent reconstruction/corruptions PASS");
}
