use std::{env, fs, path::Path};
use woi_persistent_world::{
    continuous,
    persistent::{World, session::Session},
};
fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        ["create",directory,configuration] => {
            let config:continuous::Config=serde_json::from_slice(&fs::read(configuration).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
            let session=Session::create(Path::new(directory),config)?;
            println!("created {} tick{}",session.path().display(),session.world().tick());
        }
        ["advance",directory,steps]=> {
            let mut session=Session::open(Path::new(directory))?;
            session.advance(steps.parse().map_err(|_|"invalid step count")?)?;
            session.save()?;
            println!("saved tick{} complete{}",session.world().tick(),session.world().complete());
        }
        ["status",directory]=> {
            let session=Session::open(Path::new(directory))?;
            println!("tick{} end{} active{} pending{}",session.world().tick(),session.world().end(),session.world().commitments().len(),session.world().pending().len());
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
        _=>return Err("usage: create NEW-DIR CONFIG | advance DIR STEPS | status DIR | finalize DIR | reference CONFIG NEW-OUTPUT".into()),
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
