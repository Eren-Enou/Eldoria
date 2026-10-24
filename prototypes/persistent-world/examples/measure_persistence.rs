//! Wall-clock/storage observations, kept separate from deterministic histories.
use std::{
    env, fs,
    path::PathBuf,
    time::{Instant, SystemTime, UNIX_EPOCH},
};
use woi_persistent_world::{
    continuous as c,
    persistent::{
        IDENTITY, World,
        session::{Session, check_path},
    },
    valuation as v,
};
fn main() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let root = PathBuf::from(args.first().ok_or("supply absolute new measurement root")?);
    check_path(&root)?;
    fs::create_dir(&root).map_err(|e| e.to_string())?;
    let horizon: u64 = args
        .get(1)
        .map_or(Ok(1024), |s| s.parse())
        .map_err(|_| "invalid horizon")?;
    let mut rows = vec![];
    for mode in [None, Some(v::Mode::Baseline), Some(v::Mode::MealRelief)] {
        let mut config = c::matrix()
            .into_iter()
            .find(|c| c.case == "paired" && c.mode == c::Mode::Continuous)
            .unwrap();
        config.horizon = horizon;
        config.seed = 42;
        config.period = 11;
        config.validate()?;
        let registered = mode.map(|valuation| v::Config {
            world: config.clone(),
            valuation,
            family: "persistence_measurement".into(),
            source023: None,
        });
        let start = Instant::now();
        let mut world = if let Some(r) = &registered {
            World::new_valuation(r.clone())?
        } else {
            World::new(config.clone())?
        };
        let bootstrap_ms = start.elapsed().as_secs_f64() * 1000.0;
        let start = Instant::now();
        world.advance(2048)?;
        let simulation_ms = start.elapsed().as_secs_f64() * 1000.0;
        let start = Instant::now();
        let bytes = world.checkpoint()?;
        let checkpoint_ms = start.elapsed().as_secs_f64() * 1000.0;
        let start = Instant::now();
        let restored = World::restore(&bytes)?;
        let restore_ms = start.elapsed().as_secs_f64() * 1000.0;
        if restored.export()? != world.export()? {
            return Err("measurement restore differs".into());
        }
        let start = Instant::now();
        let history = world.export()?;
        let export_ms = start.elapsed().as_secs_f64() * 1000.0;
        let id = format!(
            "measured-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| e.to_string())?
                .as_nanos()
        );
        let path = root.join(id);
        let start = Instant::now();
        let mut session = if let Some(r) = registered {
            Session::create_valuation(&path, r)?
        } else {
            Session::create(&path, config)?
        };
        let create_ms = start.elapsed().as_secs_f64() * 1000.0;
        let start = Instant::now();
        session.advance(2048)?;
        let journaled_execution_ms = start.elapsed().as_secs_f64() * 1000.0;
        let start = Instant::now();
        session.save()?;
        let save_ms = start.elapsed().as_secs_f64() * 1000.0;
        let start = Instant::now();
        drop(session);
        let opened = Session::open(&path)?;
        let open_ms = start.elapsed().as_secs_f64() * 1000.0;
        if opened.world().export()? != history {
            return Err("journaled execution differs".into());
        }
        let journal_bytes = fs::metadata(path.join("journal.jsonl"))
            .map_err(|e| e.to_string())?
            .len();
        let committed_bytes = fs::metadata(path.join("checkpoint-00000002.cbor"))
            .map_err(|e| e.to_string())?
            .len();
        let row = serde_json::json!({"mode":mode.map(|m|format!("024 {m:?}")).unwrap_or_else(||"023 Continuous".into()),"horizon":horizon,"boundaries":world.boundaries().len(),"bootstrap_ms":bootstrap_ms,"simulation_and_existing_audit_ms":simulation_ms,"checkpoint_encode_and_validate_ms":checkpoint_ms,"restore_ms":restore_ms,"terminal_export_ms":export_ms,"session_create_ms":create_ms,"journaled_execution_ms":journaled_execution_ms,"save_and_readback_ms":save_ms,"open_and_validate_ms":open_ms,"checkpoint_bytes":bytes.len(),"committed_bytes":committed_bytes,"terminal_history_bytes":history.len()+1,"journal_bytes":journal_bytes,"path":path});
        println!("{row}");
        rows.push(row);
    }
    let report = serde_json::json!({"schema":"woi-persistence-performance/v1","runtime":IDENTITY,"pid":std::process::id(),"rows":rows,"limitations":"Wall-clock observations include filesystem/cache and host load; sizes are serialized bytes, not live heap or population scaling."});
    fs::write(
        root.join("performance.json"),
        serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
