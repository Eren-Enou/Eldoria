use std::{env, fs, path::Path};
use woi_persistent_delayed_work_world::{
    continuation_c1 as original, delayed as n,
    persistent::{
        self, World,
        session::{
            Session,
            reliability::{self, Budget},
        },
    },
    storage_codec::Codec,
};
fn print(value: impl serde::Serialize) -> Result<(), String> {
    println!(
        "{}",
        serde_json::to_string(&value).map_err(|e| e.to_string())?
    );
    Ok(())
}
fn config(path: &str) -> Result<n::Config, String> {
    let value: serde_json::Value =
        serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let result: n::Config = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
    if serde_json::to_value(&result).map_err(|e| e.to_string())? != value {
        return Err("configuration contains unknown or noncanonical fields".into());
    }
    woi_persistent_delayed_work_world::validate_registration(&result)?;
    Ok(result)
}
fn status(session: &Session) -> serde_json::Value {
    let w = session.world();
    serde_json::json!({"schema":"woi-delayed-work-world-status/v1","path":session.path(),
        "runtime":persistent::IDENTITY,"config":w.delayed_registration(),"policy_identity":woi_persistent_delayed_work_world::policy_identity(w.delayed_registration().mode),"C1_source":original::SOURCE_FREEZE_SHA256,"tick":w.tick(),"start":w.start(),"end":w.end(),
        "terminal":w.complete(),"active_people":w.commitments(),"pending_social_work":w.pending(),
        "material_work":w.material().active,"raw_stock":w.material().raw,"waste":w.material().waste,
        "raw_escrow":w.material().active.iter().map(|a|a.escrow).sum::<u32>(),
        "ready_cache":w.stock(),"actors":w.agents(),"completed_boundaries":w.boundaries().len(),"unsaved":session.dirty(),
        "latest_committed_generation":session.generation(),"journal_bytes":session.journal_bytes()})
}
fn write_new(path: &str, bytes: &[u8]) -> Result<(), String> {
    use std::io::Write;
    persistent::session::check_path(Path::new(path))?;
    let mut f = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    f.write_all(bytes)
        .and_then(|_| f.sync_all())
        .map_err(|e| e.to_string())
}
fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        ["identity"]=>print(serde_json::json!({"runtime":persistent::IDENTITY,"schema":persistent::SCHEMA,"seams":serde_json::from_str::<serde_json::Value>(include_str!(concat!(env!("OUT_DIR"),"/source-seams.json"))).map_err(|e|e.to_string())?})),
        ["parity",archive]=>{
            use std::io::BufRead;
            let input=std::io::BufReader::new(flate2::read::GzDecoder::new(fs::File::open(archive).map_err(|e|e.to_string())?));
            let matrix=n::matrix();let mut count=0;
            for (index,line) in input.lines().enumerate() {
                let expected:original::Trial=serde_json::from_str(&line.map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
                if matrix.get(index)!=Some(&expected.trial.config) {return Err(format!("registered archive row{index} mismatch"));}
                let mut world=World::new(expected.trial.config.clone())?;world.advance(2048)?;
                if world.trial()?!=expected {return Err(format!("exact original C1 parity row{index} failed"));}count+=1;
            }
            if count!=100 {return Err("exact100 original C1 rows required".into());}
            print(serde_json::json!({"runtime":persistent::IDENTITY,"original_C1_source":original::SOURCE_FREEZE_SHA256,"exact_original_histories":count}))
        }
        ["config",index,mode,output]=>{
            let index:usize=index.parse().map_err(|_|"invalid registered index")?;
            let mode=match *mode {"Baseline"=>n::Mode::Baseline,"ProjectedNeed"=>n::Mode::ProjectedNeed,_=>return Err("explicit Baseline or ProjectedNeed required".into())};
            let config=n::matrix().into_iter().find(|c|c.source025==index && c.mode==mode).ok_or("registered source index outside0..49")?;
            write_new(output,&serde_json::to_vec(&config).map_err(|e|e.to_string())?)
        }
        ["create",directory,configuration]=>{
            let session=Session::create(Path::new(directory),config(configuration)?)?;
            print(status(&session))
        }
        ["advance",directory,steps,options @ ..]=>{
            let codec=match *options {[]=>Codec::Raw,["--codec","zlib"]=>Codec::Zlib,_=>return Err("expected --codec zlib or no options".into())};
            let mut session=Session::open(Path::new(directory))?;
            session.advance(steps.parse().map_err(|_|"invalid bounded step count")?)?;
            session.save_with_codec(codec)?;
            print(status(&session))
        }
        ["save",directory]=>{let mut s=Session::open(Path::new(directory))?;s.save()?;print(status(&s))}
        ["status",directory]=>print(status(&Session::open(Path::new(directory))?)),
        ["events",directory,index]=>print(Session::open(Path::new(directory))?.record(index.parse().map_err(|_|"invalid boundary")?)?),
        ["state",directory,output]=>write_new(output,&Session::open(Path::new(directory))?.world().checkpoint()?),
        ["storage",directory] | ["check",directory]=>{
            let report=reliability::inspect(Path::new(directory),&Budget::default())?;
            let valid=report.stable_snapshot && report.latest_committed.is_some() && report.latest_committed==report.latest_valid && report.generations.iter().all(|g|g.valid);
            print(&report)?;
            if args[0]=="check" && !valid {return Err("invalid or unstable material session".into());} Ok(())
        }
        ["verify",directory,"--generation",generation]=>print(reliability::verify_independent(Path::new(directory),generation.parse().map_err(|_|"explicit numeric generation required")?)?),
        ["recover",source,destination,"--generation",generation]=>print(reliability::recover(Path::new(source),Path::new(destination),generation.parse().map_err(|_|"explicit numeric generation required")?)?),
        ["interrupt",directory,steps]=>{
            let mut s=Session::open(Path::new(directory))?;
            s.advance(steps.parse().map_err(|_|"invalid steps")?)?;
            print(serde_json::json!({"uncommitted_tick":s.world().tick(),"saved":false}))
        }
        ["finalize",directory]=>Session::open(Path::new(directory))?.finalize(),
        ["reference",configuration,output]=>{
            let c=config(configuration)?;
            let mut w=World::new(c.clone())?;w.advance(2048)?;
            let actual=w.trial()?;let original=original::trial(c);
            if actual!=original {return Err("original accepted026 C1 producer mismatch".into());}
            write_new(output,&serde_json::to_vec(&actual).map_err(|e|e.to_string())?)
        }
        ["baseline",configuration,output]=>{
            use sha2::{Digest,Sha256};
            let c=config(configuration)?;
            let mut w=World::new(c.clone())?;
            let mut states=vec![];
            let mut step=0;
            loop {
                states.push(serde_json::json!({"step":step,"tick":w.tick(),
                    "state_sha256":format!("{:x}",Sha256::digest(w.checkpoint()?)),
                    "material_work":w.material().active,"active":w.commitments(),"pending":w.pending(),
                    "raw":w.material().raw,"waste":w.material().waste}));
                if !w.step()? {break;} step+=1;
            }
            let actual=w.trial()?;
            if actual!=original::trial(c) {return Err("baseline original026 C1 parity mismatch".into());}
            write_new(output,&serde_json::to_vec(&serde_json::json!({"runtime":persistent::IDENTITY,"states":states,
                "history_sha256":format!("{:x}",Sha256::digest(w.export()?))})).map_err(|e|e.to_string())?)
        }
        ["measure",configuration,output]=>{
            use std::time::Instant;
            let c=config(configuration)?;
            let mut samples=vec![];
            for _ in 0..3 {
                let start=Instant::now();let mut w=World::new(c.clone())?;w.advance(2048)?;
                let run_seconds=start.elapsed().as_secs_f64();
                let start=Instant::now();let checkpoint=w.checkpoint()?;
                let checkpoint_seconds=start.elapsed().as_secs_f64();
                let start=Instant::now();let restored=World::restore(&checkpoint)?;
                let restore_seconds=start.elapsed().as_secs_f64();
                let start=Instant::now();let original=original::trial(c.clone());
                let original_seconds=start.elapsed().as_secs_f64();
                if restored.trial()?!=original {return Err("measurement original parity mismatch".into());}
                samples.push(serde_json::json!({"run_seconds":run_seconds,"original_seconds":original_seconds,
                    "checkpoint_seconds":checkpoint_seconds,"restore_seconds":restore_seconds,"state_bytes":checkpoint.len(),
                    "boundaries":w.boundaries().len()}));
            }
            write_new(output,&serde_json::to_vec(&serde_json::json!({"schema":"woi-delayed-work-timing/v1",
                "runtime":persistent::IDENTITY,"config":c,"samples":samples,"scope":"Wall-clock samples only; separate from deterministic history"})).map_err(|e|e.to_string())?)
        }
        _=>Err("usage: identity | config SOURCE025 Baseline|ProjectedNeed NEW-OUTPUT | create NEW-DIR CONFIG | advance DIR STEPS [--codec zlib] | save/status/check/storage DIR | events DIR BOUNDARY | state DIR NEW-OUTPUT | verify DIR --generation N | recover SOURCE NEW-BRANCH-ROOT/WORLD-ID --generation N | interrupt DIR STEPS | finalize DIR | reference CONFIG NEW-OUTPUT".into()),
    }
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
