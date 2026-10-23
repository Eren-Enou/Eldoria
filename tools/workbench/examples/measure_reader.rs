//! Wall-clock observer cost, kept separate from deterministic simulation output.
use std::time::Instant;
use woi_workbench::{execution, storage::Store};
fn main() -> Result<(), String> {
    let mut store = Store::open(&woi_workbench::repository())?;
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.first().is_some_and(|s| s == "--root") {
        if args.len() < 3 {
            return Err("Use --root READ_ONLY_RUN_ROOT RUN-ID...".into());
        }
        let root = std::path::PathBuf::from(args.remove(1));
        args.remove(0);
        woi_workbench::storage::no_links(&root)?;
        store.root = root.canonicalize().map_err(|e| e.to_string())?;
    }
    for id in args {
        let path = store.run(&id)?.join("history.jsonl");
        let mut times = Vec::new();
        let mut projection_bytes = 0;
        for _ in 0..5 {
            let start = Instant::now();
            let summary = execution::inspect(&store, &id)?;
            times.push(start.elapsed().as_micros());
            projection_bytes = serde_json::to_vec(&summary)
                .map_err(|e| e.to_string())?
                .len();
        }
        println!(
            "{}",
            serde_json::json!({"run":id,"history_bytes":std::fs::metadata(&path).map_err(|e|e.to_string())?.len(),"hash_and_native_inspect_microseconds":times,"serialized_native_projection_bytes":projection_bytes,"scope":"Five wall-clock inspections in one process; filesystem caches uncontrolled; includes hash checking and parsing, excludes simulation/build/GUI rendering"})
        );
        let start = Instant::now();
        let rows = woi_workbench::recorded::moments(&path)?;
        let index_us = start.elapsed().as_micros();
        let selected = rows.iter().rev().find_map(|r| {
            r.lanes
                .iter()
                .position(Option::is_some)
                .map(|actor| (r.boundary, actor as u32))
        });
        if let Some((index, actor)) = selected {
            let start = Instant::now();
            let d = woi_workbench::recorded::detail(&path, index, actor)?;
            println!(
                "{}",
                serde_json::json!({"run":id,"moment_count":rows.len(),"serialized_moment_index_bytes":serde_json::to_vec(&rows).map_err(|e|e.to_string())?.len(),"index_microseconds":index_us,"late_detail_microseconds":start.elapsed().as_micros(),"selected_boundary":index,"selected_actor":actor,"linked_transitions":d.transitions.len(),"scope":"One four-pass streamed index and five-pass late detail lookup; cached filesystem, excludes GUI rendering; serialization bytes are not allocator/RSS measurements; no scientific outcome or throughput claim"})
            );
        }
    }
    Ok(())
}
