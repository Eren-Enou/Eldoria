use eldoria_settlement::{CharacterId, Config, Settlement, observer, storage};
use serde::Serialize;
use std::{
    io::{self, Write},
    path::Path,
};

const USAGE: &str = "eldoria-settlement init NEW.json [--seed N] [--population N] [--horizon N]\n\
eldoria-settlement run INPUT.json --ticks N --out NEW.json\n\
eldoria-settlement resume INPUT.json --ticks N --out NEW.json\n\
eldoria-settlement inspect INPUT.json [--actor ID]\n\
eldoria-settlement events INPUT.json [--from N] [--limit N]\n\
eldoria-settlement save INPUT.json --out NEW.json\n\
All writes create new files. Each run pauses and commits at a safe boundary.\n";
fn json(value: &impl Serialize) -> Result<(), String> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    serde_json::to_writer_pretty(&mut out, value).map_err(|e| e.to_string())?;
    writeln!(out).map_err(|e| e.to_string())
}
fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.is_empty() || args[0] == "--help" || args[0] == "help" {
        print!("{USAGE}");
        return Ok(());
    }
    let command = args[0].as_str();
    let input = args.get(1).ok_or(USAGE)?;
    let allowed: &[&str] = match command {
        "init" => &["--seed", "--population", "--horizon"],
        "run" | "resume" => &["--ticks", "--out"],
        "inspect" => &["--actor"],
        "events" => &["--from", "--limit"],
        "save" => &["--out"],
        _ => return Err(USAGE.into()),
    };
    let mut options = std::collections::BTreeMap::new();
    let (chunks, remainder) = args[2..].as_chunks::<2>();
    for pair in chunks {
        if !allowed.contains(&pair[0].as_str())
            || options.insert(pair[0].as_str(), pair[1].as_str()).is_some()
        {
            return Err(format!("unknown or duplicate option: {}", pair[0]));
        }
    }
    if !remainder.is_empty() {
        return Err("option value missing".into());
    }
    let number = |key: &str, default: u64| -> Result<u64, String> {
        options.get(key).map_or(Ok(default), |s| {
            s.parse().map_err(|_| format!("invalid number for {key}"))
        })
    };
    if command == "init" {
        let world = Settlement::new(Config {
            seed: number("--seed", 42)?,
            population: usize::try_from(number("--population", 8)?).map_err(|e| e.to_string())?,
            horizon: number("--horizon", 256)?,
        })?;
        storage::save(&world, Path::new(input))?;
        return json(&observer::snapshot(&world));
    }
    let mut world = storage::load(Path::new(input))?;
    match command {
        "run" | "resume" => {
            if !options.contains_key("--ticks") {
                return Err("--ticks required".into());
            }
            let output = options.get("--out").ok_or("--out required")?;
            world.advance(number("--ticks", 0)?)?;
            storage::save(&world, Path::new(output))?;
            json(&observer::snapshot(&world))
        }
        "save" => storage::save(
            &world,
            Path::new(options.get("--out").ok_or("--out required")?),
        ),
        "inspect" => {
            if let Some(actor) = options.get("--actor") {
                let id = CharacterId(actor.parse().map_err(|_| "invalid actor ID")?);
                json(&world.owned_view(id)?)
            } else {
                json(&observer::snapshot(&world))
            }
        }
        "events" => {
            let from = usize::try_from(number("--from", 0)?).map_err(|e| e.to_string())?;
            let limit = usize::try_from(number("--limit", 25)?).map_err(|e| e.to_string())?;
            json(&observer::records(&world, from, limit))
        }
        _ => unreachable!(),
    }
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
