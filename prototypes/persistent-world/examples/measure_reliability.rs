//! Bounded engineering observations; never part of deterministic simulation evidence.
use std::{env, fs, path::PathBuf, time::Instant};
use woi_persistent_world::{
    continuous as c,
    persistent::{
        World,
        session::{Session, check_path, reliability},
    },
    storage_codec::Codec,
    valuation as v,
};
fn ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.
}
fn main() -> Result<(), String> {
    let path = PathBuf::from(
        env::args()
            .nth(1)
            .ok_or("supply absolute NEW measurement root")?,
    );
    check_path(&path)?;
    fs::create_dir(&path).map_err(|e| e.to_string())?;
    let mut rows = vec![];
    for horizon in [128, 1024] {
        for (mode_index, mode) in [None, Some(v::Mode::Baseline), Some(v::Mode::MealRelief)]
            .into_iter()
            .enumerate()
        {
            for (codec_index, codec) in [Codec::Raw, Codec::Zlib].into_iter().enumerate() {
                let mut config = c::matrix()
                    .into_iter()
                    .find(|c| c.case == "paired" && c.mode == c::Mode::Continuous)
                    .unwrap();
                config.horizon = horizon;
                config.seed = 42;
                config.period = 11;
                let registered = mode.map(|valuation| v::Config {
                    world: config.clone(),
                    valuation,
                    family: "reliability_measurement".into(),
                    source023: None,
                });
                let world_path = path.join(format!("world-{horizon}-{mode_index}-{codec_index}"));
                let mut s = if let Some(r) = &registered {
                    Session::create_valuation(&world_path, r.clone())?
                } else {
                    Session::create(&world_path, config.clone())?
                };
                let mut save_ms = vec![];
                let start = Instant::now();
                for _ in 0..3 {
                    s.advance((horizon / 4) as usize)?;
                    let before = Instant::now();
                    s.save_with_codec(codec)?;
                    save_ms.push(ms(before));
                }
                let journal_and_save_ms = ms(start);
                let expected = s.world().checkpoint()?;
                drop(s);
                let start = Instant::now();
                let mut s = Session::open(&world_path)?;
                let open_ms = ms(start);
                if s.world().checkpoint()? != expected {
                    return Err("measurement reopen changed state".into());
                }
                let start = Instant::now();
                let inventory = reliability::inspect(&world_path, &Default::default())?;
                let inspection_ms = ms(start);
                // Live writer intentionally makes this first inventory unstable; measure deep validation after close.
                if inventory.stable_snapshot {
                    return Err("inspection missed writer ownership".into());
                }
                s.advance(7)?;
                drop(s);
                let start = Instant::now();
                let inventory = reliability::inspect(&world_path, &Default::default())?;
                let deep_inspection_ms = ms(start);
                let start = Instant::now();
                let independent = reliability::verify_independent(&world_path, 4)?;
                let independent_ms = ms(start);
                let branch_root = path.join(format!("branch-{horizon}-{mode_index}-{codec_index}"));
                fs::create_dir(&branch_root).map_err(|e| e.to_string())?;
                let branch = branch_root.join(world_path.file_name().unwrap());
                let start = Instant::now();
                let recovery = reliability::recover(&world_path, &branch, 4)?;
                let recovery_ms = ms(start);
                let mut resumed = Session::open(&branch)?;
                if resumed.world().checkpoint()? != expected {
                    return Err("measurement branch changed selected state".into());
                }
                resumed.advance(2048)?;
                resumed.save_with_codec(codec)?;
                resumed.finalize()?;
                let mut reference = if let Some(r) = registered {
                    World::new_valuation(r)?
                } else {
                    World::new(config)?
                };
                reference.advance(2048)?;
                if resumed.world().checkpoint()? != reference.checkpoint()?
                    || resumed.world().export()? != reference.export()?
                {
                    return Err("measurement terminal state/history mismatch".into());
                }
                drop(resumed);
                rows.push(serde_json::json!({"horizon":horizon,"mode":format!("{mode:?}"),"codec":format!("{codec:?}"),"saved_generations":inventory.generations.len(),"checkpoint_bytes":inventory.generations.iter().map(|g|g.bytes).collect::<Vec<_>>(),"journal_bytes":inventory.journal_bytes,"total_retained_bytes":inventory.total_retained_bytes,"save_ms":save_ms,"journal_and_save_ms":journal_and_save_ms,"open_ms":open_ms,"busy_inspection_ms":inspection_ms,"deep_inspection_ms":deep_inspection_ms,"independent_ms":independent_ms,"independent_tick":independent.tick,"recovery_ms":recovery_ms,"preserved_tail_bytes":recovery.tail_bytes,"exact_state_and_history":true}));
            }
        }
    }
    let report = serde_json::json!({"schema":"woi-reliability-performance/v1","pid":std::process::id(),"runtime":woi_persistent_world::persistent::IDENTITY,"rows":rows,"limitations":"Local host/cache wall-clock observations; retained file bytes are not heap, allocated blocks, population scaling or power-loss durability. Reference worlds run outside measured save/open/recovery intervals."});
    fs::write(
        path.join("performance.json"),
        serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!("{}", path.join("performance.json").display());
    Ok(())
}
