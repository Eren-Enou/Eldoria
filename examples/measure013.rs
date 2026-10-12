use std::{hint::black_box, time::Instant};
use world_of_individuals::{experiment013 as e, provenance as p, self_evaluation as s};
fn main() {
    let t = e::trial(42, e::Mode::Pulse, "unchanged", e::Config::default());
    let compact = &t.final_state.base.base.base;
    let record = t
        .final_state
        .learning
        .records
        .iter()
        .find(|r| r.owner == 0 && r.before.as_ref().is_some_and(|c| c.attempts >= 2))
        .unwrap();
    let reference = compact
        .provenance
        .queries
        .iter()
        .find(|q| q.inquiry.0 == record.inquiry)
        .unwrap();
    let base = p::policy(&compact.query(reference.id).unwrap().decision.input);
    let mut cells = vec![];
    for r in t
        .final_state
        .learning
        .records
        .iter()
        .take_while(|r| r.inquiry != record.inquiry)
        .filter(|r| r.owner == 0)
    {
        if let Some(c) = &r.after {
            if let Some(i) = cells.iter().position(|old: &s::Cell| {
                old.source == c.source && old.strategy == c.strategy && old.context == c.context
            }) {
                cells[i] = c.clone();
            } else {
                cells.push(c.clone());
            }
        }
    }
    println!("kind,repeat,mode,length,nanoseconds,bytes,cells,audit_bytes");
    for repeat in 0..5 {
        for mode in e::MODES {
            let input_cells = cells
                .iter()
                .cloned()
                .map(|mut c| {
                    c.context = s::key_context(mode.learning(), c.context);
                    c
                })
                .collect::<Vec<_>>();
            let start = Instant::now();
            for _ in 0..50_000 {
                black_box(e::evaluate(
                    black_box(base.clone()),
                    mode,
                    black_box(&record.basis),
                    black_box(&input_cells),
                ));
            }
            println!(
                "policy,{repeat},{mode:?},50000,{},0,{},0",
                start.elapsed().as_nanos(),
                input_cells.len()
            );
        }
    }
    for repeat in 0..3 {
        for length in [16, 32, 64, 96] {
            for mode in [e::Mode::Contextual, e::Mode::Pulse] {
                let start = Instant::now();
                let t = e::trial(
                    42,
                    mode,
                    "longitudinal",
                    e::Config {
                        long_rounds: length,
                        ..Default::default()
                    },
                );
                let elapsed = start.elapsed().as_nanos();
                let bytes = serde_json::to_vec(&t).unwrap().len();
                println!(
                    "history,{repeat},{mode:?},{length},{elapsed},{bytes},{},{}",
                    t.final_state
                        .learning
                        .cells
                        .values()
                        .map(|v| v.len())
                        .sum::<usize>(),
                    serde_json::to_vec(&t.final_state.learning.records)
                        .unwrap()
                        .len()
                );
            }
        }
    }
}
