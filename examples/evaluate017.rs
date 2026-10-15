use std::{fs, path::PathBuf, time::Instant};
use world_of_individuals::experiment017 as e;
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let out = args
        .windows(2)
        .find(|w| w[0] == "--output")
        .map_or(PathBuf::from("target/experiment017"), |w| {
            PathBuf::from(&w[1])
        });
    fs::create_dir_all(&out).unwrap();
    let mut summaries = vec![];
    let mut timings = String::from("seed,held,worlds,suite_microseconds\n");
    for seed in [0, 1, 42, u64::MAX] {
        for held in [false, true] {
            let start = Instant::now();
            let trials = e::suite(seed, held);
            let us = start.elapsed().as_micros();
            assert_eq!(trials, e::suite(seed, held));
            timings.push_str(&format!("{seed},{held},{},{us}\n", trials.len()));
            summaries.extend(trials.iter().map(e::summary));
            if seed == 42 {
                fs::write(
                    out.join(if held {
                        "held-out-seed-42.json"
                    } else {
                        "seed-42.json"
                    }),
                    serde_json::to_vec(&trials).unwrap(),
                )
                .unwrap();
            }
        }
    }
    fs::write(
        out.join("summary.json"),
        serde_json::to_vec(&summaries).unwrap(),
    )
    .unwrap();
    fs::write(out.join("performance.csv"), timings).unwrap();
    println!(
        "017:{} worlds each exact replayed; {}",
        summaries.len(),
        out.display()
    );
}
