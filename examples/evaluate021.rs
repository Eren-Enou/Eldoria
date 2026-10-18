use std::{
    fs,
    io::{BufWriter, Write},
    path::PathBuf,
    time::Instant,
};
use world_of_individuals::experiment021 as f;
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let out = args
        .windows(2)
        .find(|w| w[0] == "--output")
        .map_or(PathBuf::from("target/experiment021"), |w| {
            PathBuf::from(&w[1])
        });
    fs::create_dir_all(out.parent().unwrap()).unwrap();
    fs::create_dir(&out).expect("021 output directory must not already exist");
    assert!(
        !out.join("trials.jsonl").exists(),
        "021 output must be fresh"
    );
    let mut file = BufWriter::new(fs::File::create(out.join("trials.jsonl")).unwrap());
    let mut timing = String::from("trial,execution_us,replay_us,serialization_us,bytes\n");
    for (id, row) in f::matrix().into_iter().enumerate() {
        let (seed, policy, mode, case, config) =
            (row.seed, row.policy, row.mode, row.case, row.config);
        let start = Instant::now();
        let trial = f::trial(seed, policy, mode, &case, config.clone());
        let execution = start.elapsed().as_micros();
        let start = Instant::now();
        let replay = f::trial(seed, policy, mode, &case, config);
        assert_eq!(trial, replay);
        let replay_us = start.elapsed().as_micros();
        drop(replay);
        let start = Instant::now();
        let bytes = serde_json::to_vec(&trial).unwrap();
        file.write_all(&bytes).unwrap();
        file.write_all(b"\n").unwrap();
        timing.push_str(&format!(
            "{id},{execution},{replay_us},{},{bytes_len}\n",
            start.elapsed().as_micros(),
            bytes_len = bytes.len()
        ));
        println!(
            "021 {id} {seed}/{case}/{mode:?}: exact replay, {} claims",
            trial.records.len()
        );
    }
    file.flush().unwrap();
    fs::write(out.join("performance.csv"), timing).unwrap();
}
