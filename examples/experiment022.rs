//! Deterministic registered runner. Never writes historical experiments.
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};
use world_of_individuals::experiment::concurrent as x;
fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("provide a NEW isolated JSONL output path");
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    let mut out = BufWriter::new(file);
    for (ordinal, config) in x::matrix().into_iter().enumerate() {
        eprintln!(
            "022 trial {ordinal}: {} cap{}",
            config.case, config.capacity
        );
        let trial = x::trial(config);
        serde_json::to_writer(&mut out, &trial).unwrap();
        out.write_all(b"\n").unwrap();
        out.flush().unwrap();
    }
}
