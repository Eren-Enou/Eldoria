//! Separate wall-clock/storage samples, never deterministic experiment outcomes.
use std::time::Instant;
use world_of_individuals::{continuous as c, experiment::concurrent as x};
fn main() {
    for (case, horizon) in [
        ("paired", 64),
        ("paired", 192),
        ("exhaustion", 384),
        ("exhaustion", 768),
    ] {
        for mode in [c::Mode::Windowed, c::Mode::Continuous] {
            let mut config = c::matrix()
                .into_iter()
                .find(|v| v.case == case && v.mode == mode && v.role == "controlled")
                .unwrap();
            config.horizon = horizon;
            for repeat in 0..3 {
                let time = Instant::now();
                let trial = c::trial(config.clone());
                let simulation_ns = time.elapsed().as_nanos();
                let time = Instant::now();
                let encoded = serde_json::to_vec(&trial).unwrap();
                let serialization_ns = time.elapsed().as_nanos();
                let history = &trial.history;
                let mut live = std::collections::BTreeMap::new();
                let mut peak_live = 0;
                let mut peak_commitment_payload = 0;
                for transition in &history.transitions {
                    if transition.completed {
                        live.remove(&transition.commitment.id);
                    } else {
                        live.insert(transition.commitment.id, &transition.commitment);
                    }
                    peak_live = peak_live.max(live.len());
                    let payload: usize = live
                        .values()
                        .map(|c| {
                            std::mem::size_of::<x::Commitment>()
                                + c.participants.len() * std::mem::size_of::<u32>()
                        })
                        .sum();
                    peak_commitment_payload = peak_commitment_payload.max(payload);
                }
                let owned_input_bytes = history
                    .boundaries
                    .iter()
                    .flat_map(|b| &b.choices)
                    .map(|d| serde_json::to_vec(&d.input).unwrap().len())
                    .max()
                    .unwrap_or(0);
                println!(
                    "{}",
                    serde_json::json!({"case":case,"horizon":horizon,"mode":mode,"repeat":repeat,"simulation_ns":simulation_ns,"serialization_ns":serialization_ns,"history_bytes":encoded.len(),"boundaries":history.boundaries.len(),"checkpoints":trial.checkpoints.len(),"peak_live_commitments":peak_live,"peak_commitment_payload_excluding_allocator_and_slack":peak_commitment_payload,"inline_commitment_bytes":std::mem::size_of::<x::Commitment>(),"inline_pending_bytes":std::mem::size_of::<x::Pending>(),"maximum_serialized_owned_input_bytes_not_heap":owned_input_bytes,"original_scenes_including_historical":history.final_state.base.base.base.base.base.scenes.len()})
                );
                std::hint::black_box(trial);
            }
        }
    }
}
