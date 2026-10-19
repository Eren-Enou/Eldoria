//! Wall-clock evidence only; never part of deterministic scientific histories.
use std::time::Instant;
use world_of_individuals::experiment::concurrent as x;
fn main() {
    for case in ["paired", "acquisition", "inquiry_quiet", "ongoing"] {
        for capacity in [1, 2] {
            let config = x::matrix()
                .into_iter()
                .find(|c| c.case == case && c.capacity == capacity)
                .unwrap();
            for repeat in 0..3 {
                let start = Instant::now();
                let trial = x::trial(config.clone());
                let simulation = start.elapsed().as_nanos();
                let start = Instant::now();
                let bytes = serde_json::to_vec(&trial).unwrap();
                let serialization = start.elapsed().as_nanos();
                println!(
                    "{}",
                    serde_json::json!({"case":case,"capacity":capacity,"repeat":repeat,"simulation_ns":simulation,"serialization_ns":serialization,"bytes":bytes.len(),"boundaries":trial.boundaries.len(),"choices":trial.boundaries.iter().map(|b|b.choices.len()).sum::<usize>(),"transitions":trial.transitions.len(),"world_ticks":trial.end-trial.start})
                );
                std::hint::black_box(trial);
            }
        }
    }
}
