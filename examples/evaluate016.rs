use std::{fs, path::PathBuf, time::Instant};
use world_of_individuals::experiment016 as e;
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let output = args
        .windows(2)
        .find(|w| w[0] == "--output")
        .map_or(PathBuf::from("target/experiment016"), |w| {
            PathBuf::from(&w[1])
        });
    fs::create_dir_all(&output).unwrap();
    let preview = args.iter().any(|a| a == "--preview");
    let mut rows = vec![];
    let mut performance = String::from("seed,held,worlds,suite_microseconds\n");
    for seed in if preview {
        vec![42]
    } else {
        vec![0, 1, 42, u64::MAX]
    } {
        for held in [false, true] {
            if preview && held {
                continue;
            }
            let start = Instant::now();
            let ts = e::suite(seed, held);
            let duration = start.elapsed().as_micros();
            assert_eq!(ts, e::suite(seed, held));
            performance.push_str(&format!("{seed},{held},{},{duration}\n", ts.len()));
            rows.extend(ts.iter().map(e::summary));
            if seed == 42 {
                fs::write(
                    output.join(if held {
                        "held-out-seed-42.json"
                    } else {
                        "seed-42.json"
                    }),
                    serde_json::to_vec(&ts).unwrap(),
                )
                .unwrap();
            }
        }
    }
    fs::write(
        output.join("summary.json"),
        serde_json::to_vec(&rows).unwrap(),
    )
    .unwrap();
    fs::write(output.join("performance.csv"), performance).unwrap();
    println!(
        "{} Experiment016 worlds, each exactly replayed; output {}",
        rows.len(),
        output.display()
    );
}
