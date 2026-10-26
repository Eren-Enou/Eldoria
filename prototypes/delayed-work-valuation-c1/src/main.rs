//! C1 only;original frozen evaluator and its failure remain available.
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
    process::{Command, Stdio},
};
use world_of_individuals::{
    continuation_c1 as c, delayed as n, encounters as e, preparation as m, valuation as v,
};
fn root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn preflight(treatment: bool) {
    assert!(
        Command::new("python")
            .args([
                "-B",
                "examples/support/identity026_c1.py",
                if treatment {
                    "treatment"
                } else {
                    "compatibility"
                },
                c::SOURCE_FREEZE_SHA256
            ])
            .current_dir(root())
            .status()
            .unwrap()
            .success()
    );
}
fn output(path: &str) -> BufWriter<std::fs::File> {
    let p = Path::new(path);
    let parent = p
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
        .canonicalize()
        .unwrap();
    assert!(
        !parent.starts_with(root().join("experiments"))
            && !parent.starts_with(root().join("reinforcement"))
    );
    BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(p)
            .expect("NEW output required"),
    )
}
fn write(out: &mut impl Write, t: &impl serde::Serialize) {
    serde_json::to_writer(&mut *out, t).unwrap();
    out.write_all(b"\n").unwrap();
    out.flush().unwrap();
}
fn read(path: &str) -> std::process::Child {
    Command::new("python").args(["-B","-c","import gzip,sys;p=sys.argv[1];f=gzip.open(p,'rb')if p.endswith('.gz')else open(p,'rb');[sys.stdout.buffer.write(l)for l in f]",path]).stdout(Stdio::piped()).spawn().unwrap()
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.get(1).map(String::as_str).unwrap_or("help") {
        "baseline" => {
            preflight(false);
            let mut child = read(
                args.get(2)
                    .map(String::as_str)
                    .unwrap_or("experiments/025/trials.jsonl.gz"),
            );
            let mut out = args.get(3).map(|s| output(s));
            let mut count = 0;
            for line in BufReader::new(child.stdout.take().unwrap()).lines() {
                let line = line.unwrap();
                let old: m::Trial = serde_json::from_str(&line).unwrap();
                assert!(count < 50);
                let cfg = n::matrix()[2 * count].clone();
                assert_eq!(cfg.world, old.config);
                let new = c::trial(cfg);
                assert_eq!(
                    serde_json::to_string(&c::baseline_projection(&new)).unwrap(),
                    line,
                    "C1 baseline{count}"
                );
                if let Some(out) = &mut out {
                    write(out, &new);
                }
                count += 1;
            }
            assert!(child.wait().unwrap().success());
            assert_eq!(count, 50);
            println!("C1:50 complete original025 Baseline bytes exact;zero extensions");
        }
        "prefix" => {
            preflight(false);
            let mut child = read(
                args.get(2)
                    .map(String::as_str)
                    .unwrap_or("experiments/026/failure/attempt-001.jsonl.gz"),
            );
            let mut out = args.get(3).map(|s| output(s));
            let mut count = 0;
            for line in BufReader::new(child.stdout.take().unwrap()).lines() {
                let line = line.unwrap();
                let old: n::Trial = serde_json::from_str(&line).unwrap();
                assert!(count < 43);
                assert_eq!(old.config, n::matrix()[count]);
                let new = c::trial(old.config);
                assert!(new.response_extensions.is_empty());
                assert_eq!(
                    serde_json::to_string(&new.trial).unwrap(),
                    line,
                    "C1 complete v1 prefix{count}"
                );
                if let Some(out) = &mut out {
                    write(out, &new);
                }
                count += 1;
            }
            assert!(child.wait().unwrap().success());
            assert_eq!(count, 43);
            println!(
                "C1:43 complete v1-prefix bytes exact;negative/mixed cases unchanged;zero extensions"
            );
        }
        "feature-off" => {
            preflight(false);
            let mut child = read(
                args.get(2)
                    .map(String::as_str)
                    .unwrap_or("experiments/024/trials.jsonl.gz"),
            );
            let mut count = 0;
            for line in BufReader::new(child.stdout.take().unwrap()).lines() {
                let old: v::Trial = serde_json::from_str(&line.unwrap()).unwrap();
                let cfg = m::Config {
                    family: old.config.family,
                    pair: None,
                    world: old.config.world,
                    valuation: old.config.valuation,
                    preparation: m::Settings::default(),
                };
                assert_eq!(n::feature_off(cfg.clone()), m::trial(cfg));
                count += 1;
            }
            assert!(child.wait().unwrap().success());
            assert_eq!(count, 56);
            println!("C1:56 feature-off complete histories exact");
        }
        "responses" => {
            preflight(false);
            let mut child = read(args.get(2).expect("old025 or v1-prefix archive"));
            let mut count = 0;
            for line in BufReader::new(child.stdout.take().unwrap()).lines() {
                let t: serde_json::Value = serde_json::from_str(&line.unwrap()).unwrap();
                let h = if t.get("history").is_some() {
                    &t["history"]
                } else {
                    &t["body"]["history"]
                };
                for b in h["boundaries"].as_array().unwrap() {
                    for r in b["responses"].as_array().unwrap() {
                        let r: e::Response = serde_json::from_value(r.clone()).unwrap();
                        assert!(c::defined_response_equal(&r));
                        count += 1;
                    }
                }
            }
            assert!(child.wait().unwrap().success());
            assert!(count > 0);
            println!("C1 Rust defined-response equivalence:{count} exact records");
        }
        "matrix" => {
            preflight(true);
            let mut out = output(args.get(2).expect("NEW C1 output"));
            for (i, cfg) in n::matrix().into_iter().enumerate() {
                eprintln!("026-C1 row{i} source{} {:?}", cfg.source025, cfg.mode);
                write(&mut out, &c::trial(cfg));
            }
        }
        "replay" => {
            preflight(true);
            let mut child = read(args.get(2).expect("C1 archive"));
            let mut count = 0;
            for line in BufReader::new(child.stdout.take().unwrap()).lines() {
                let line = line.unwrap();
                let old: c::Trial = serde_json::from_str(&line).unwrap();
                assert!(count < 100);
                assert_eq!(old.trial.config, n::matrix()[count]);
                assert_eq!(
                    serde_json::to_string(&c::trial(old.trial.config)).unwrap(),
                    line
                );
                count += 1;
            }
            assert!(child.wait().unwrap().success());
            assert_eq!(count, 100);
            println!("C1:100 complete byte replays exact");
        }
        _ => println!(
            "C1 baseline [old025.gz] [NEW]\nC1 prefix [v1-prefix.gz] [NEW]\nC1 feature-off\nC1 responses OLD.gz\nC1 matrix NEW\nC1 replay C1.gz\nNo default execution;unknown treatment requires published C1 gate."
        ),
    }
}
