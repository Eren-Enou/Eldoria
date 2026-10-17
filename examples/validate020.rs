//! Standalone020 read-only gate; it does not extend the protected foundation.
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};
use world_of_individuals::experiment020 as f;
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
r=pathlib.Path(sys.argv[1]);d=r/'experiments/020'
for n,h in json.loads((d/'source-freeze.json').read_bytes()).items():assert hashlib.sha256((r/n).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==h,n
for n,h in json.loads((d/'frozen-001-019-sha256.json').read_bytes()).items():
 b=(r/n).read_bytes()
 if n.startswith(tuple(f'experiments/{i:03}/' for i in range(13,20))) and not n.endswith('.gz'):b=b.replace(b'\r\n',b'\n')
 assert hashlib.sha256(b).hexdigest()==h,n
for n,v in json.loads((d/'archive-manifest.json').read_bytes()).items():
 b=(d/n).read_bytes();raw=gzip.decompress(b)
 assert (len(b),hashlib.sha256(b).hexdigest(),len(raw),hashlib.sha256(raw).hexdigest())==(v['gzip_bytes'],v['gzip_sha256'],v['json_bytes'],v['json_sha256']),n
print('020 source/foundation/archive hashes PASS')
"#,
    );
    let mut child=Command::new("python").args(["-B","-c","import gzip,pathlib,sys,shutil;shutil.copyfileobj(gzip.open(pathlib.Path(sys.argv[1])/'experiments/020/trials.jsonl.gz','rb'),sys.stdout.buffer)",env!("CARGO_MANIFEST_DIR")]).stdout(Stdio::piped()).spawn().unwrap();
    let matrix = f::matrix();
    let mut count = 0;
    for (id, line) in BufReader::new(child.stdout.take().unwrap())
        .lines()
        .enumerate()
    {
        let line = line.unwrap();
        let expected: f::Trial = serde_json::from_str(&line).unwrap();
        let (seed, mode, case, config) = &matrix[id];
        assert_eq!(
            (
                expected.seed,
                expected.mode,
                expected.case.as_str(),
                &expected.config
            ),
            (*seed, *mode, *case, config)
        );
        let actual = f::trial(*seed, *mode, case, config.clone());
        assert!(actual == expected, "020 archived trial {id}");
        assert!(
            serde_json::to_vec(&actual).unwrap() == line.as_bytes(),
            "020 byte replay {id}"
        );
        count += 1;
    }
    assert!(child.wait().unwrap().success());
    assert_eq!(count, 164);
    let status = Command::new("python")
        .args([
            "-B",
            "examples/support/review020.py",
            "--corruptions",
            "--expected",
            "experiments/020/analysis.json.gz",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .unwrap();
    assert!(status.success());
    println!("020:164 exact original byte replays;independent reconstruction/corruptions PASS");
}
