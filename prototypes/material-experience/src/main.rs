use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};
use woi_material_experience::{continuation_c1 as c, experience as x};
fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("identity") => println!("{}", x::SOURCE_ID),
        Some("parity") => {
            let mut child=Command::new("python").args(["-B","-c","import gzip,sys;[sys.stdout.buffer.write(l) for l in gzip.open(sys.argv[1],'rb')]",&a[2]]).stdout(Stdio::piped()).spawn().unwrap();
            let mut count = 0;
            for line in BufReader::new(child.stdout.take().unwrap()).lines() {
                let old: c::Trial = serde_json::from_str(&line.unwrap()).unwrap();
                assert_eq!(
                    c::trial(old.trial.config.clone()),
                    old,
                    "inherited C1 row{count}"
                );
                if old.trial.config.mode == woi_material_experience::delayed::Mode::ProjectedNeed {
                    let new = x::trial(x::matrix()[2 * old.trial.config.source025].clone());
                    let mut body = old.trial.body;
                    body.schema = "experiment-025-trial/v1".into();
                    body.history.schema = "experiment-025-physical-history/v1".into();
                    assert_eq!(
                        serde_json::to_vec(&new.body).unwrap(),
                        serde_json::to_vec(&body).unwrap(),
                        "027 Fixed exact"
                    );
                }
                count += 1;
            }
            assert!(child.wait().unwrap().success());
            assert_eq!(count, 100);
            println!("100 inherited C1 +50 Fixed complete histories exact");
        }
        Some("run") => {
            assert!(
                Command::new("python")
                    .args(["-B", "examples/support/identity027.py", x::SOURCE_ID])
                    .status()
                    .unwrap()
                    .success(),
                "published027 activation required"
            );
            let parent = std::path::Path::new(&a[2])
                .parent()
                .unwrap()
                .canonicalize()
                .unwrap();
            let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .unwrap();
            assert!(
                !parent.starts_with(root.join("experiments"))
                    && !parent.starts_with(root.join("reinforcement")),
                "scratch output required"
            );
            let mut out = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&a[2])
                .unwrap();
            for cfg in x::matrix() {
                let t = x::trial(cfg);
                serde_json::to_writer(&mut out, &t).unwrap();
                out.write_all(b"\n").unwrap();
                out.flush().unwrap();
            }
        }
        _ => panic!("identity | parity ORIGINAL-GZ | run NEW-OUTPUT"),
    }
}
