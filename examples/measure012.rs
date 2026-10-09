//! Wall-clock measurements only, separate from deterministic experimental evidence.
use std::{hint::black_box, time::Instant};
use world_of_individuals::{
    experiment012 as e, inquiry as q, provenance as p, self_evaluation as s,
};
fn main() {
    let t = e::trial(
        42,
        s::Mode::Unchanged,
        "novel_no_progress",
        e::Config::default(),
    );
    let compact = &t.final_state.base.base.base;
    let r = t
        .final_state
        .learning
        .records
        .iter()
        .find(|r| r.owner == 0 && r.inquiry >= t.opportunities[2].before.inquiry_end as u64)
        .unwrap();
    let reference = compact
        .provenance
        .queries
        .iter()
        .find(|x| x.inquiry.0 == r.inquiry)
        .unwrap();
    let base = p::policy(&compact.query(reference.id).unwrap().decision.input);
    let cells: Vec<_> = (1..=16)
        .flat_map(|source| {
            [q::Strategy::Direct, q::Strategy::Evidence].map(move |strategy| s::Cell {
                source,
                strategy,
                context: s::Context::EvidencePresent,
                attempts: 1,
                expected_change: 27,
                last_inquiry: 0,
            })
        })
        .collect();
    println!("repeat,mode,occupancy,candidates,iterations,nanoseconds,cell_json_bytes");
    for repeat in 0..5 {
        for mode in s::MODES {
            for occupancy in [0, 32] {
                let mode_cells: Vec<_> = cells
                    .iter()
                    .cloned()
                    .map(|mut c| {
                        c.context = s::key_context(mode, c.context);
                        c
                    })
                    .collect();
                let live = &mode_cells[..occupancy];
                let iterations = 50_000;
                let start = Instant::now();
                for _ in 0..iterations {
                    black_box(s::policy(
                        black_box(base.clone()),
                        mode,
                        black_box(&r.basis),
                        black_box(live),
                    ));
                }
                println!(
                    "{repeat},{mode:?},{occupancy},{},{iterations},{},{}",
                    base.decision.candidates.len(),
                    start.elapsed().as_nanos(),
                    serde_json::to_vec(live).unwrap().len()
                );
            }
        }
    }
}
