//! Isolated, bounded research CLI. All output files must be new.
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
    process::{Command, Stdio},
};
use world_of_individuals::{continuous as c, experiment::concurrent as x};
fn output(path: &str) -> BufWriter<std::fs::File> {
    let path = Path::new(path);
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent).unwrap();
    let parent = parent.canonicalize().unwrap();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    if let Ok(relative) = parent.strip_prefix(root.join("experiments")) {
        let n = relative
            .components()
            .next()
            .and_then(|c| c.as_os_str().to_str())
            .and_then(|c| c.parse::<u32>().ok());
        assert!(
            !n.is_some_and(|n| n <= 22),
            "protected historical output directory"
        );
    }
    assert!(
        !parent.starts_with(root.join("reinforcement")),
        "protected reinforcement evidence"
    );
    BufWriter::new(
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .expect("output must be a NEW file"),
    )
}
fn write<T: serde::Serialize>(out: &mut impl Write, trial: &T) {
    serde_json::to_writer(&mut *out, trial).unwrap();
    out.write_all(b"\n").unwrap();
    out.flush().unwrap();
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("help");
    match command {
        "legacy022" => {
            let mut out = output(args.get(2).expect("new output path"));
            for config in x::matrix() {
                write(&mut out, &x::trial(config));
            }
        }
        "matrix" => {
            let mut out = output(args.get(2).expect("new output path"));
            for (n, config) in c::matrix().into_iter().enumerate() {
                eprintln!(
                    "023 {n}: {} {:?} H{} P{} {:?}",
                    config.case, config.mode, config.horizon, config.period, config.phase
                );
                write(&mut out, &c::trial(config));
            }
        }
        "run" => {
            let mut config = c::matrix()
                .into_iter()
                .find(|c| c.mode == c::Mode::Continuous && c.case == "paired")
                .unwrap();
            assert_eq!(args.len() % 2, 1, "run OUTPUT [--option value] pairs");
            for pair in args[3..].as_chunks::<2>().0 {
                match pair[0].as_str() {
                    "--case" => config.case = pair[1].clone(),
                    "--ticks" => config.horizon = pair[1].parse().unwrap(),
                    "--period" => config.period = pair[1].parse().unwrap(),
                    "--seed" => config.seed = pair[1].parse().unwrap(),
                    "--claim" => config.duration = pair[1].parse().unwrap(),
                    "--mode" => {
                        config.mode =
                            serde_json::from_value(serde_json::Value::String(pair[1].clone()))
                                .unwrap()
                    }
                    "--phase" => {
                        config.phase =
                            serde_json::from_value(serde_json::Value::String(pair[1].clone()))
                                .unwrap()
                    }
                    "--order" => {
                        config.order =
                            serde_json::from_value(serde_json::Value::String(pair[1].clone()))
                                .unwrap()
                    }
                    _ => panic!("unknown bounded run option"),
                }
            }
            config.role = "demo".into();
            config.validate().unwrap();
            let mut out = output(args.get(2).expect("new output path"));
            write(&mut out, &c::trial(config));
        }
        "replay" => {
            let path = args.get(2).expect("original archive or isolated JSONL");
            let mut child=Command::new("python").args(["-B","-c","import gzip,sys; p=sys.argv[1]; f=gzip.open(p,'rb') if p.endswith('.gz') else open(p,'rb'); [sys.stdout.buffer.write(line) for line in f]",path]).stdout(Stdio::piped()).spawn().unwrap();
            let mut count = 0;
            for line in BufReader::new(child.stdout.take().unwrap()).lines() {
                let line = line.unwrap();
                let archived: c::Trial = serde_json::from_str(&line).unwrap();
                let replay = c::trial(archived.config.clone());
                assert_eq!(
                    serde_json::to_string(&replay).unwrap(),
                    line,
                    "023 exact replay row{count}"
                );
                count += 1;
            }
            assert!(child.wait().unwrap().success());
            println!("023: {count} exact byte replays PASS");
        }
        _ => println!(
            "continuous-world run NEW.jsonl [--case paired --ticks 192 --period 11 --mode Continuous --phase Epoch --seed 42 --claim 4 --order Ascending]\ncontinuous-world matrix NEW.jsonl\ncontinuous-world replay ORIGINAL.jsonl.gz\ncontinuous-world legacy022 NEW.jsonl"
        ),
    }
}
