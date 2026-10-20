//! Opt-in024 bounded evaluator/prototype. Outputs must be new and outside archives.
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
    process::{Command, Stdio},
};
use world_of_individuals::{continuous as c, valuation as v};
fn output(name: &str) -> BufWriter<std::fs::File> {
    let path = Path::new(name);
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
        .canonicalize()
        .expect("output parent must already exist");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    assert!(
        !parent.starts_with(root.join("experiments"))
            && !parent.starts_with(root.join("reinforcement")),
        "scientific archives are not runtime output locations"
    );
    BufWriter::new(
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .expect("output must be NEW"),
    )
}
fn write(out: &mut impl Write, trial: &impl serde::Serialize) {
    serde_json::to_writer(&mut *out, trial).unwrap();
    out.write_all(b"\n").unwrap();
    out.flush().unwrap();
}
fn read(path: &str) -> std::process::Child {
    Command::new("python").args(["-B","-c","import gzip,sys;p=sys.argv[1];f=gzip.open(p,'rb') if p.endswith('.gz') else open(p,'rb');[sys.stdout.buffer.write(line) for line in f]",path]).stdout(Stdio::piped()).spawn().unwrap()
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1).map(String::as_str).unwrap_or("help") {
        "baseline-parity" => {
            let path = args
                .get(2)
                .map(String::as_str)
                .unwrap_or("experiments/023/trials.jsonl.gz");
            let mut child = read(path);
            let mut old_count = 0;
            let mut new_count = 0;
            for line in BufReader::new(child.stdout.take().unwrap()).lines() {
                let line = line.unwrap();
                let old: c::Trial = serde_json::from_str(&line).unwrap();
                let legacy = c::trial(old.config.clone());
                assert_eq!(
                    serde_json::to_string(&legacy).unwrap(),
                    line,
                    "inherited023row{old_count}"
                );
                if old.config.mode == c::Mode::Continuous {
                    let mut new = v::trial(v::Config {
                        world: old.config.clone(),
                        valuation: v::Mode::Baseline,
                        family: "pre_treatment_parity".into(),
                        source023: Some(old_count),
                    });
                    assert_eq!(new.history.schema, "experiment-024-physical-history/v1");
                    new.history.schema = old.history.schema.clone();
                    assert_eq!(
                        serde_json::to_string(&new.history).unwrap(),
                        serde_json::to_string(&old.history).unwrap(),
                        "024 disabled-path body{old_count}"
                    );
                    assert_eq!(new.checkpoints, old.checkpoints);
                    assert!(new.valuations.iter().all(|r| {
                        r.selection
                            .terms
                            .iter()
                            .all(|t| t.meal_relief == 0 && t.base_score == t.final_score)
                    }));
                    new_count += 1;
                }
                old_count += 1;
            }
            assert!(child.wait().unwrap().success());
            assert_eq!((old_count, new_count), (42, 22));
            println!(
                "024 pre-treatment gate PASS:42original023exact byte reruns;22Continuous Baseline bodies/checkpoints exact except declared physical schema;all zero-bonus audit terms"
            );
        }
        "matrix" => {
            let mut out = output(args.get(2).expect("new output file"));
            for (n, cfg) in v::matrix().into_iter().enumerate() {
                eprintln!(
                    "024row{n}:{} {:?} seed{} H{}",
                    cfg.world.case, cfg.valuation, cfg.world.seed, cfg.world.horizon
                );
                write(&mut out, &v::trial(cfg));
            }
        }
        "replay" => {
            let mut child = read(args.get(2).expect("original JSONL/gzip"));
            let mut count = 0;
            for line in BufReader::new(child.stdout.take().unwrap()).lines() {
                let line = line.unwrap();
                let old: v::Trial = serde_json::from_str(&line).unwrap();
                let new = v::trial(old.config.clone());
                assert_eq!(
                    serde_json::to_string(&new).unwrap(),
                    line,
                    "024 byte replay{count}"
                );
                count += 1;
            }
            assert!(child.wait().unwrap().success());
            println!("024 {count} exact byte replays PASS");
        }
        "run" => {
            let mut cfg = v::matrix()[0].clone();
            assert_eq!(args.len() % 2, 1, "run OUTPUT [--option value] pairs");
            for pair in args[3..].as_chunks::<2>().0 {
                match pair[0].as_str() {
                    "--case" => cfg.world.case = pair[1].clone(),
                    "--ticks" => cfg.world.horizon = pair[1].parse().unwrap(),
                    "--period" => cfg.world.period = pair[1].parse().unwrap(),
                    "--seed" => cfg.world.seed = pair[1].parse().unwrap(),
                    "--claim" => cfg.world.duration = pair[1].parse().unwrap(),
                    "--valuation" => {
                        cfg.valuation =
                            serde_json::from_value(serde_json::Value::String(pair[1].clone()))
                                .unwrap()
                    }
                    "--phase" => {
                        cfg.world.phase =
                            serde_json::from_value(serde_json::Value::String(pair[1].clone()))
                                .unwrap()
                    }
                    "--order" => {
                        cfg.world.order =
                            serde_json::from_value(serde_json::Value::String(pair[1].clone()))
                                .unwrap()
                    }
                    _ => panic!("unknown bounded024option"),
                }
            }
            cfg.world.role = "demo".into();
            cfg.family = "custom_demo".into();
            cfg.source023 = None;
            cfg.validate().unwrap();
            let mut out = output(args.get(2).unwrap());
            write(&mut out, &v::trial(cfg));
        }
        _ => println!(
            "activity-valuation baseline-parity ORIGINAL023.gz\nactivity-valuation matrix NEW.jsonl\nactivity-valuation replay ORIGINAL024.gz\nactivity-valuation run NEW.jsonl [--valuation Baseline|MealRelief --case paired --ticks 192 --period 11 --seed 42 --claim 4 --phase Epoch --order Ascending]"
        ),
    }
}
