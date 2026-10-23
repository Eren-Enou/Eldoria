//! Additive transport over the unchanged source-preserving023/024 library.
use std::{
    io::{BufWriter, Write},
    path::Path,
};
use woi_workbench::{
    config::{Configuration, Mode},
    portable, storage,
};
fn main() {
    if let Err(e) = run() {
        eprintln!("Portable runner: {e}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("identity") if args.len() == 1 => println!(
            "{}",
            include_str!(concat!(env!("OUT_DIR"), "/build-identity.json"))
        ),
        Some("run") if args.len() == 3 => {
            let config: Configuration = storage::read(Path::new(&args[1]))?;
            config.validate()?;
            if config.mode == Mode::MaterialPreparation {
                return Err("025 is not available under this portable contract; its original preflight is preserved".into());
            }
            let value = config.runner_config();
            let mut out = BufWriter::new(portable::new_history_output(Path::new(&args[2]))?);
            match config.mode {
                Mode::ContinuousWorld => {
                    let c: engine::continuous::Config =
                        serde_json::from_value(value).map_err(|e| e.to_string())?;
                    c.validate()?;
                    serde_json::to_writer(&mut out, &engine::continuous::trial(c))
                        .map_err(|e| e.to_string())?;
                }
                Mode::ActivityValuation => {
                    let c: engine::valuation::Config =
                        serde_json::from_value(value).map_err(|e| e.to_string())?;
                    c.validate()?;
                    serde_json::to_writer(&mut out, &engine::valuation::trial(c))
                        .map_err(|e| e.to_string())?;
                }
                Mode::MaterialPreparation => unreachable!(),
            }
            out.write_all(b"\n")
                .and_then(|()| out.flush())
                .map_err(|e| e.to_string())?;
        }
        _ => return Err("Use identity or run CONFIGURATION.json NEW-history.jsonl".into()),
    }
    Ok(())
}
