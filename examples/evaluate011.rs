use std::{fs, time::Instant};
use world_of_individuals::{experiment011 as e, inquiry_value as v};
#[path = "support/summary011.rs"]
mod summary011;
use summary011::summary;
fn main() {
    let preview = std::env::args().any(|a| a == "--preview");
    fs::create_dir_all("target/experiment011").unwrap();
    let mut rows = vec![];
    let mut timings = String::from("seed,held,run_replay_validate_ms,bytes,serialize_ms\n");
    let seeds: Vec<_> = if preview {
        vec![42]
    } else {
        (0..16).chain([42, u64::MAX]).collect()
    };
    for seed in seeds {
        for held in if preview {
            vec![false]
        } else {
            vec![false, true]
        } {
            let start = Instant::now();
            let trials = e::suite(seed, held);
            assert_eq!(trials, e::suite(seed, held));
            let elapsed = start.elapsed().as_secs_f64() * 1000.;
            for group in trials.as_chunks::<3>().0 {
                for t in group {
                    assert_eq!(t.before_prelude, group[0].before_prelude);
                    assert_eq!(t.opportunities[0].before, group[0].opportunities[0].before);
                    assert_eq!(
                        e::queries(t)[0].decision.input,
                        e::queries(&group[0])[0].decision.input
                    );
                }
            }
            for config in if held {
                e::held_out()
            } else {
                vec![e::Config::default()]
            } {
                for mode in v::MODES {
                    let a = trials
                        .iter()
                        .find(|t| {
                            t.case == "uncertain_minor" && t.config == config && t.mode == mode
                        })
                        .unwrap();
                    let b = trials
                        .iter()
                        .find(|t| {
                            t.case == "mirrored_future" && t.config == config && t.mode == mode
                        })
                        .unwrap();
                    assert_eq!(e::queries(a)[0].decision, e::queries(b)[0].decision);
                    assert_eq!(a.opportunities, b.opportunities);
                }
            }
            let start = Instant::now();
            let bytes = serde_json::to_vec(&trials).unwrap();
            timings.push_str(&format!(
                "{seed},{held},{elapsed:.3},{},{:.3}\n",
                bytes.len(),
                start.elapsed().as_secs_f64() * 1000.
            ));
            if seed == 42 {
                fs::write(
                    format!(
                        "target/experiment011/{}.json",
                        if held { "held-out-seed-42" } else { "seed-42" }
                    ),
                    bytes,
                )
                .unwrap();
            }
            for t in &trials {
                let row = summary(t, held);
                if preview {
                    println!(
                        "{} {:?}: {:?} -> {:?}",
                        t.case,
                        t.mode,
                        e::queries(t)
                            .iter()
                            .map(|r| r.decision.selected)
                            .collect::<Vec<_>>(),
                        row["actions"]
                    );
                }
                rows.push(row);
            }
            eprintln!(
                "011 seed {seed} held {held}: exact replay, matched inputs and mirrored futures passed"
            );
        }
    }
    fs::write(
        "target/experiment011/summary.json",
        serde_json::to_vec(&rows).unwrap(),
    )
    .unwrap();
    fs::write("target/experiment011/benchmark.csv", timings).unwrap();
    if !preview {
        let mut timing = String::from(
            "mode,sample,run_validate_ms,serialize_ms,bytes,audit_bytes,audit_records\n",
        );
        for sample in 0..7 {
            for mode in v::MODES {
                let start = Instant::now();
                let t = e::trial(42, mode, "uncertain_minor", e::Config::default());
                let run = start.elapsed().as_secs_f64() * 1000.;
                let start = Instant::now();
                let bytes = serde_json::to_vec(&t).unwrap();
                let serialize = start.elapsed().as_secs_f64() * 1000.;
                timing.push_str(&format!(
                    "{mode:?},{sample},{run:.3},{serialize:.3},{},{},{}\n",
                    bytes.len(),
                    serde_json::to_vec(&t.final_state.value).unwrap().len(),
                    t.final_state.value.records.len()
                ));
            }
        }
        fs::write("target/experiment011/performance.csv", timing).unwrap();
        // Identical bounded inputs for every mode; includes identical decision-clone cost.
        let t = e::trial(
            42,
            v::Mode::Unchanged,
            "uncertain_minor",
            e::Config::default(),
        );
        let query = t
            .final_state
            .base
            .base
            .base
            .query(world_of_individuals::audit::QueryId(0))
            .unwrap();
        let base = world_of_individuals::provenance::policy(&query.decision.input);
        let basis = &t.opportunities[0].before.basis;
        let mut scoring = String::from("mode,sample,iterations,candidates,basis,clone_score_ns\n");
        for sample in 0..7 {
            for mode in v::MODES {
                let start = Instant::now();
                for _ in 0..10000 {
                    std::hint::black_box(v::compare(
                        mode,
                        std::hint::black_box(base.clone()),
                        std::hint::black_box(basis),
                    ));
                }
                scoring.push_str(&format!(
                    "{mode:?},{sample},10000,{},{},{:.3}\n",
                    base.decision.candidates.len(),
                    basis.len(),
                    start.elapsed().as_nanos() as f64 / 10000.
                ));
            }
        }
        fs::write("target/experiment011/scoring.csv", scoring).unwrap();
    }
}
