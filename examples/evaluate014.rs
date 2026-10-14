use std::{fs, path::PathBuf, time::Instant};
use world_of_individuals::experiment014 as e;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let out = args
        .windows(2)
        .find(|w| w[0] == "--output")
        .map_or(PathBuf::from("target/experiment014"), |w| {
            PathBuf::from(&w[1])
        });
    fs::create_dir_all(&out).unwrap();
    let preview = args.iter().any(|a| a == "--preview");
    let mut summary = vec![];
    let mut perf = String::from("seed,held,worlds,suite_microseconds\n");
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
            perf.push_str(&format!("{seed},{held},{},{duration}\n", ts.len()));
            assert_eq!(ts, e::suite(seed, held));
            for t in &ts {
                summary.push(e::summary(t, held));
            }
            if seed == 42 {
                fs::write(
                    out.join(if held {
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
        out.join("summary.json"),
        serde_json::to_vec(&summary).unwrap(),
    )
    .unwrap();
    fs::write(out.join("performance.csv"), perf).unwrap();
    println!("{} exact replayed worlds", summary.len());
}
