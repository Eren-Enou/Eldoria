use std::{env, fs, path::PathBuf};
use world_of_individuals::experiment::{human_report, run_experiment};

fn run() -> Result<(), String> {
    let mut seed = 42_u64;
    let mut scenario = "all".to_owned();
    let mut population = 1000_usize;
    let mut output: Option<PathBuf> = None;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--help" || arg == "-h" {
            println!(
                "world-of-individuals [--seed N] [--scenario A|B|C|D|E|F|G|all] [--population 2..100000] [--output DIR]\nDefault: all scenarios, seed 42, population 1000. Output saves report.json and report.txt."
            );
            return Ok(());
        }
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {arg}"))?;
        match arg.as_str() {
            "--seed" => seed = value.parse().map_err(|_| "invalid seed")?,
            "--scenario" => scenario = value,
            "--population" => population = value.parse().map_err(|_| "invalid population")?,
            "--output" => output = Some(value.into()),
            _ => return Err(format!("unknown argument {arg}")),
        }
    }
    let report = run_experiment(seed, &scenario, population)?;
    let human = human_report(&report);
    if let Some(dir) = output {
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        fs::write(
            dir.join("report.json"),
            serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        fs::write(dir.join("report.txt"), &human).map_err(|e| e.to_string())?;
    }
    print!("{human}");
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(2);
    }
}
