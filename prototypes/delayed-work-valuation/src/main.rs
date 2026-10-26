//! Headless026 evaluator. No arguments executes no world.
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
    process::{Command, Stdio},
};
use world_of_individuals::{delayed as n, preparation as m, valuation as v};
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
                "examples/support/identity026.py",
                if treatment { "treatment" } else { "baseline" },
                n::SOURCE_FREEZE_SHA256
            ])
            .current_dir(root())
            .status()
            .unwrap()
            .success()
    );
}
fn output(name: &str) -> BufWriter<std::fs::File> {
    let path = Path::new(name);
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
        .canonicalize()
        .unwrap();
    assert!(
        !parent.starts_with(root().join("experiments"))
            && !parent.starts_with(root().join("reinforcement")),
        "archive output forbidden"
    );
    BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .expect("NEW output required"),
    )
}
fn write(out: &mut impl Write, trial: &impl serde::Serialize) {
    serde_json::to_writer(&mut *out, trial).unwrap();
    out.write_all(b"\n").unwrap();
    out.flush().unwrap();
}
fn read(name: &str) -> std::process::Child {
    Command::new("python").args(["-B","-c","import gzip,sys;p=sys.argv[1];f=gzip.open(p,'rb') if p.endswith('.gz') else open(p,'rb');[sys.stdout.buffer.write(l) for l in f]",name]).stdout(Stdio::piped()).spawn().unwrap()
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.get(1).map(String::as_str).unwrap_or("help") {
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
                let original = m::trial(cfg.clone());
                let new = n::feature_off(cfg);
                assert_eq!(
                    new, original,
                    "026 disabled physical feature full state row{count}"
                );
                assert_eq!(
                    serde_json::to_string(&new).unwrap(),
                    serde_json::to_string(&original).unwrap()
                );
                count += 1;
            }
            assert!(child.wait().unwrap().success());
            assert_eq!(count, 56);
            println!(
                "026 feature-off:56 exact inherited025 compatibility bodies PASS;both024 valuation modes preserved"
            );
        }
        "baseline" => {
            preflight(false);
            let mut child = read(
                args.get(2)
                    .map(String::as_str)
                    .unwrap_or("experiments/025/trials.jsonl.gz"),
            );
            let mut out = args.get(3).map(|p| output(p));
            let mut count = 0;
            for line in BufReader::new(child.stdout.take().unwrap()).lines() {
                let line = line.unwrap();
                let original: m::Trial = serde_json::from_str(&line).unwrap();
                assert!(count < 50);
                let cfg = n::matrix()[2 * count].clone();
                assert_eq!(cfg.world, original.config);
                let actual = n::trial(cfg);
                assert_eq!(
                    serde_json::to_string(&n::baseline_projection(&actual)).unwrap(),
                    line,
                    "026 disabled original025 row{count}"
                );
                if let Some(o) = &mut out {
                    write(o, &actual);
                }
                count += 1;
            }
            assert!(child.wait().unwrap().success());
            assert_eq!(count, 50);
            println!("026 Baseline:50 exact original025 complete byte histories PASS");
        }
        "matrix" => {
            preflight(true);
            let mut out = output(args.get(2).expect("NEW output"));
            for (i, cfg) in n::matrix().into_iter().enumerate() {
                eprintln!("026 row{i} source{} {:?}", cfg.source025, cfg.mode);
                write(&mut out, &n::trial(cfg));
            }
        }
        "replay" => {
            preflight(true);
            let mut child = read(args.get(2).expect("026 archive"));
            let mut count = 0;
            for line in BufReader::new(child.stdout.take().unwrap()).lines() {
                let line = line.unwrap();
                let original: n::Trial = serde_json::from_str(&line).unwrap();
                assert!(count < 100);
                assert_eq!(original.config, n::matrix()[count]);
                assert_eq!(
                    serde_json::to_string(&n::trial(original.config)).unwrap(),
                    line,
                    "026 exact row{count}"
                );
                count += 1;
            }
            assert!(child.wait().unwrap().success());
            assert_eq!(count, 100);
            println!("026:100 exact complete byte replays PASS");
        }
        _ => println!(
            "delayed-work-valuation baseline [ORIGINAL025.gz] [NEW.jsonl]\ndelayed-work-valuation matrix NEW.jsonl\ndelayed-work-valuation replay ORIGINAL026.gz\nExplicit research only;matrix/replay require published runtime gate. No default execution."
        ),
    }
}
