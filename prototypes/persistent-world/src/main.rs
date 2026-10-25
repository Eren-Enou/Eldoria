use std::{env, fs, path::Path};
use woi_persistent_world::{
    continuous,
    persistent::{
        World,
        session::{
            Session,
            reliability::{self, Budget},
        },
    },
};
fn budget(options: &[&str]) -> Result<Budget, String> {
    if !options.len().is_multiple_of(2) {
        return Err("storage options require --name NUMBER pairs".into());
    }
    let mut result = Budget::default();
    let mut seen = std::collections::BTreeSet::new();
    for pair in options.as_chunks::<2>().0 {
        if !seen.insert(pair[0]) {
            return Err(format!("duplicate storage option{}", pair[0]));
        }
        match pair[0] {
            "--budget-bytes" => {
                result.bytes = Some(pair[1].parse().map_err(|_| "invalid byte budget")?)
            }
            "--max-generations" => {
                result.generations =
                    Some(pair[1].parse().map_err(|_| "invalid generation warning")?)
            }
            "--project-saves" => {
                result.projected_saves = pair[1].parse().map_err(|_| "invalid projection")?
            }
            unknown => return Err(format!("unknown storage option{unknown}")),
        }
    }
    Ok(result)
}
fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        ["verify", directory, "--generation", generation] => {
            let selected=generation.parse().map_err(|_|"explicit numeric generation required")?;
            println!("{}",serde_json::to_string_pretty(&reliability::verify_independent(Path::new(directory),selected)?).map_err(|e|e.to_string())?);
        }
        ["check", directory] => {
            let report=reliability::inspect(Path::new(directory),&Budget::default())?;
            let checked=report.stable_snapshot && report.latest_committed.is_some() && report.latest_committed==report.latest_valid && report.generations.iter().all(|g|g.valid);
            println!("{}",serde_json::to_string_pretty(&report).map_err(|e|e.to_string())?);
            if !checked {return Err("verification incomplete or invalid: inspect JSON errors and select recovery generation explicitly".into());}
        }
        ["storage", directory, options @ ..] => {
            let budget=budget(options)?;
            println!("{}",serde_json::to_string_pretty(&reliability::inspect(Path::new(directory), &budget)?).map_err(|e|e.to_string())?);
        }
        ["recover", source, destination, "--generation", generation] => {
            let selected=generation.parse().map_err(|_|"explicit numeric generation required")?;
            println!("{}",serde_json::to_string_pretty(&reliability::recover(Path::new(source), Path::new(destination), selected)?).map_err(|e|e.to_string())?);
        }
        ["interrupt", directory, steps] => {
            let mut session=Session::open(Path::new(directory))?;
            session.advance(steps.parse().map_err(|_|"invalid step count")?)?;
            println!("{}",serde_json::json!({"schema":"woi-uncommitted-stop/v1","tick":session.world().tick(),"checkpoint_saved":false,"warning":"completed journal tail is evidence; resume requires explicit branch recovery"}));
        }
        ["create",directory,configuration] => {
            let config:continuous::Config=serde_json::from_slice(&fs::read(configuration).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
            let session=Session::create(Path::new(directory),config)?;
            println!("{}",serde_json::json!({"schema":"woi-world-status/v1","path":session.path(),"tick":session.world().tick(),"complete":session.world().complete(),"saved":true}));
        }
        ["advance",directory,steps]=> {
            let mut session=Session::open(Path::new(directory))?;
            session.advance(steps.parse().map_err(|_|"invalid step count")?)?;
            session.save()?;
            println!("{}",serde_json::json!({"schema":"woi-world-status/v1","path":session.path(),"tick":session.world().tick(),"complete":session.world().complete(),"saved":true}));
        }
        ["status",directory]=> {
            let session=Session::open(Path::new(directory))?;
            println!("{}",serde_json::json!({"schema":"woi-world-status/v1","path":session.path(),"tick":session.world().tick(),"end":session.world().end(),"complete":session.world().complete(),"active":session.world().commitments(),"pending":session.world().pending(),"config":session.world().configuration()}));
        }
        ["finalize",directory]=> { Session::open(Path::new(directory))?.finalize()?; }
        ["reference",configuration,output]=> {
            woi_persistent_world::persistent::session::check_path(Path::new(output))?;
            let config:continuous::Config=serde_json::from_slice(&fs::read(configuration).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
            let mut world=World::new(config.clone())?;world.advance(2048)?;
            let trial=world.trial()?;
            if trial!=continuous::trial(config) {return Err("original coordinator mismatch".into());}
            use std::io::Write;
            let mut file=fs::OpenOptions::new().write(true).create_new(true).open(output).map_err(|e|e.to_string())?;
            serde_json::to_writer(&mut file,&trial).map_err(|e|e.to_string())?;
            file.write_all(b"\n").and_then(|_|file.sync_all()).map_err(|e|e.to_string())?;
        }
        _=>return Err("usage: create NEW-DIR CONFIG | advance DIR STEPS | status DIR | finalize DIR | check DIR | storage DIR [--budget-bytes N --max-generations N --project-saves N] | recover SOURCE NEW-BRANCH-ROOT/WORLD-ID --generation N | interrupt DIR STEPS | reference CONFIG NEW-OUTPUT".into()),
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
