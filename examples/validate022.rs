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
