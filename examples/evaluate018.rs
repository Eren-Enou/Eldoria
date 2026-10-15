use std::{
    fs,
    io::{BufWriter, Write},
    path::PathBuf,
    time::Instant,
};
use world_of_individuals::{experiment018 as f, recurring as r};
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let out = args
        .windows(2)
        .find(|w| w[0] == "--output")
        .map_or(PathBuf::from("target/experiment018"), |w| {
            PathBuf::from(&w[1])
        });
    fs::create_dir_all(&out).unwrap();
    let mut summaries = vec![];
    let mut timing = String::from(
        "seed,held,worlds,execution_microseconds,replay_microseconds,serialization_microseconds\n",
    );
    for seed in [0, 1, 42, u64::MAX] {
        for held in [false, true] {
            let mut file = (seed == 42).then(|| {
                BufWriter::new(
                    fs::File::create(out.join(if held {
                        "held-out-seed-42.jsonl"
                    } else {
                        "seed-42.jsonl"
                    }))
                    .unwrap(),
                )
            });
            let (mut count, mut execution, mut replay, mut serialization) = (0, 0, 0, 0);
            for (config, cases) in f::configurations(held) {
                for case in cases {
                    let mut initial = None;
                    for mode in r::MODES {
                        let start = Instant::now();
                        let t = f::trial(seed, mode, case, config.clone());
                        execution += start.elapsed().as_micros();
                        let opening = (t.initial.clone(), t.profiles.clone());
                        if let Some(prior) = &initial {
                            assert_eq!(prior, &opening);
                        } else {
                            initial = Some(opening);
                        }
                        let start = Instant::now();
                        let repeat = f::trial(seed, mode, case, config.clone());
                        replay += start.elapsed().as_micros();
                        assert!(t == repeat, "018 replay {seed}/{case}/{mode:?}");
                        drop(repeat);
                        let start = Instant::now();
                        summaries.push(f::summary(&t));
                        if let Some(file) = &mut file {
                            serde_json::to_writer(&mut *file, &t).unwrap();
                            file.write_all(b"\n").unwrap();
                        }
                        serialization += start.elapsed().as_micros();
                        count += 1;
                    }
                }
            }
            if let Some(mut file) = file {
                file.flush().unwrap();
            }
            assert_eq!(count, if held { 154 } else { 112 });
            timing.push_str(&format!(
                "{seed},{held},{count},{execution},{replay},{serialization}\n"
            ));
            println!("018 seed={seed} held={held}: {count} exact trials");
        }
    }
    assert_eq!(summaries.len(), 1064);
    fs::write(
        out.join("summary.json"),
        serde_json::to_vec(&summaries).unwrap(),
    )
    .unwrap();
    fs::write(out.join("performance.csv"), timing).unwrap();
}
