use std::{fs, path::PathBuf, time::Instant};
use world_of_individuals::experiment013 as e;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let out = args
        .windows(2)
        .find(|w| w[0] == "--output")
        .map_or(PathBuf::from("target/experiment013"), |w| {
            PathBuf::from(&w[1])
        });
    fs::create_dir_all(&out).unwrap();
    let preview = args.iter().any(|a| a == "--preview");
    let mut summaries = vec![];
    let mut perf = String::from("seed,held,case,mode,microseconds,bytes\n");
    for seed in if preview {
        vec![42]
    } else {
        vec![0, 1, 2, 3, 42, u64::MAX]
    } {
        for held in [false, true] {
            if preview && held {
                continue;
            }
            let mut trials = vec![];
            for config in if held {
                e::held_out()
            } else {
                vec![e::Config::default()]
            } {
                for case in e::CASES {
                    for mode in e::MODES {
                        let start = Instant::now();
                        let t = e::trial(seed, mode, case, config.clone());
                        let elapsed = start.elapsed().as_micros();
                        assert_eq!(t, e::trial(seed, mode, case, config.clone()));
                        let bytes = serde_json::to_vec(&t).unwrap().len();
                        perf.push_str(&format!(
                            "{seed},{held},{case},{mode:?},{elapsed},{bytes}\n"
                        ));
                        summaries.push(e::summary(&t, held));
                        trials.push(t);
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
        serde_json::to_vec(&summaries).unwrap(),
    )
    .unwrap();
    fs::write(out.join("performance.csv"), perf).unwrap();
    println!("{} trials rerun exactly", summaries.len());
}
