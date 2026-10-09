use std::{fs, path::PathBuf, time::Instant};
use world_of_individuals::experiment012 as e;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let out = args.windows(2).find(|w| w[0] == "--output").map_or_else(
        || PathBuf::from("target/experiment012"),
        |w| PathBuf::from(&w[1]),
    );
    fs::create_dir_all(&out).unwrap();
    let preview = args.iter().any(|a| a == "--preview");
    let mut summary = vec![];
    let mut performance = String::from(
        "seed,held,case,mode,microseconds,serialized_bytes,learning_bytes,cells,audit_records\n",
    );
    for seed in if preview {
        vec![42]
    } else {
        (0..8).chain([42, u64::MAX]).collect()
    } {
        for held in [false, true] {
            if preview && held {
                continue;
            }
            let configs = if held {
                e::held_out()
            } else {
                vec![e::Config::default()]
            };
            let mut trials = vec![];
            for config in configs {
                for case in e::CASES {
                    for mode in world_of_individuals::self_evaluation::MODES {
                        let start = Instant::now();
                        let trial = e::trial(seed, mode, case, config.clone());
                        let elapsed = start.elapsed().as_micros();
                        assert_eq!(trial, e::trial(seed, mode, case, config.clone()));
                        let bytes = serde_json::to_vec(&trial).unwrap().len();
                        let learning = &trial.final_state.learning;
                        performance.push_str(&format!(
                            "{seed},{held},{case},{mode:?},{elapsed},{bytes},{},{},{}\n",
                            serde_json::to_vec(learning).unwrap().len(),
                            learning.cells.values().map(|v| v.len()).sum::<usize>(),
                            learning.records.len()
                        ));
                        summary.push(e::summary(&trial, held));
                        trials.push(trial);
                    }
                }
            }
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
        serde_json::to_vec(&summary).unwrap(),
    )
    .unwrap();
    fs::write(out.join("performance.csv"), performance).unwrap();
    println!(
        "{} trials replay exactly; output {}",
        summary.len(),
        out.display()
    );
}
