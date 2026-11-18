use eldoria_settlement::{Config, Settlement, observer, storage};
use std::time::Instant;

fn main() {
    let mut samples = vec![];
    for population in [8, 12] {
        for ticks in [64, 256, 512] {
            let mut world = Settlement::new(Config {
                seed: 42,
                population,
                horizon: 512,
            })
            .unwrap();
            let start = Instant::now();
            world.advance(ticks).unwrap();
            let advance_us = start.elapsed().as_micros();
            let start = Instant::now();
            let bytes = storage::encode(&world).unwrap();
            let encode_us = start.elapsed().as_micros();
            let start = Instant::now();
            let restored = storage::decode(&bytes).unwrap();
            let restore_us = start.elapsed().as_micros();
            assert_eq!(world.checkpoint(), restored.checkpoint());
            let path = std::env::temp_dir().join(format!(
                "eldoria-benchmark-{}-{population}-{ticks}.json",
                std::process::id()
            ));
            let start = Instant::now();
            storage::write_new(&path, &bytes).unwrap();
            let publish_us = start.elapsed().as_micros();
            let start = Instant::now();
            let loaded = storage::load(&path).unwrap();
            let load_restore_us = start.elapsed().as_micros();
            assert_eq!(world.checkpoint(), loaded.checkpoint());
            std::fs::remove_file(&path).unwrap();
            samples.push(serde_json::json!({ "population":population,"ticks":ticks,"advance_us":advance_us,
                "encode_validate_us":encode_us,"restore_validate_us":restore_us,"publish_us":publish_us,
                "load_restore_us":load_restore_us,"checkpoint_bytes":bytes.len(),
                "history_records":world.state().history.len(),"outcomes":observer::snapshot(&world).outcomes }));
        }
    }
    println!(
        "{}",
        serde_json::json!({"scope":"one release-mode process, seed42, includes first scheduler initialization; no repeated samples", "runtime":eldoria_settlement::model::RUNTIME, "samples":samples})
    );
}
