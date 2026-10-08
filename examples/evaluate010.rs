use std::{fs, time::Instant};
use world_of_individuals::experiment010 as e;
fn main() {
    let preview = std::env::args().any(|a| a == "--preview");
    fs::create_dir_all("target/experiment010").unwrap();
    let mut rows = vec![];
    let mut timings =
        String::from("seed,held,run_replay_validate_ms,serialized_bytes,serialize_ms\n");
    let seeds: Vec<_> = if preview {
        vec![42]
    } else {
        (0..32).chain([42, u64::MAX]).collect()
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
            let serialization = Instant::now();
            let bytes = serde_json::to_vec(&trials).unwrap();
            timings.push_str(&format!(
                "{seed},{held},{elapsed:.3},{},{:.3}\n",
                bytes.len(),
                serialization.elapsed().as_secs_f64() * 1000.
            ));
            if seed == 42 {
                fs::write(
                    format!(
                        "target/experiment010/{}.json",
                        if held { "held-out-seed-42" } else { "seed-42" }
                    ),
                    &bytes,
                )
                .unwrap();
            }
            for trial in trials {
                let compact = &trial.final_state.base.base.base;
                let events = &compact.base.base.events;
                let start = trial.points[5].event_end;
                let actions: Vec<_> = events[start..]
                    .iter()
                    .map(|event| event.decision.selected)
                    .collect();
                let inquiry = e::selected(&trial);
                let first_inquiry = compact.base.inquiry.records[trial.points[3].inquiry_end..]
                    .iter()
                    .find(|r| r.decision.input.owner == 0)
                    .unwrap();
                let view = world_of_individuals::model::Observation {
                    partner: trial.resource_partner,
                    last_signal: None,
                    amount: trial.config.amount,
                    turns_left: 6,
                };
                let diagnostic_probes: Vec<_> = [3, 4, 5]
                    .into_iter()
                    .map(|stage| {
                        world_of_individuals::behavior::scarcity_policy(
                            &trial.points[stage].agents[0],
                            &view,
                        )
                    })
                    .collect();
                let row = serde_json::json!({"seed":seed,"held":held,"case":trial.case,"config":trial.config,
                "mode":trial.mode,"retention":trial.method,"basis":trial.points[3].basis,"inquiry":inquiry,
                "after_inquiry":trial.points[4].basis,"resource_partner":trial.resource_partner,"actions":actions,
                "after_action_food":trial.points[6].agents.iter().map(|a|a.food).collect::<Vec<_>>(),
                "persistent_food":trial.points[7].agents.iter().map(|a|a.food).collect::<Vec<_>>(),
                "after_inquiry_concerns":trial.points[4].concerns,"consumption":trial.consumption,
                "diagnostic_probes":diagnostic_probes,"inquiry_id":first_inquiry.id,
                "inquiry_candidates":first_inquiry.decision.candidates,
                "response":first_inquiry.response.as_ref().map(|r|serde_json::json!({
                    "valid":r.valid,"public_action":r.public_action,"receipts":r.receipts,
                    "credibility_before":r.credibility_before,"credibility_after":r.credibility_after,
                    "failure":r.failure})),
                "inquiry_time":first_inquiry.time_spent,"realized_value":first_inquiry.realized_value,
                "attention_records":trial.final_state.attention.records.len(),
                "attention_bytes":serde_json::to_vec(&trial.final_state.attention).unwrap().len(),
                "trial_bytes":serde_json::to_vec(&trial).unwrap().len()});
                if preview {
                    println!("{}", serde_json::to_string(&row).unwrap());
                }
                rows.push(row);
            }
            eprintln!("seed {seed} held {held}: exact replay passed");
        }
    }
    fs::write(
        "target/experiment010/summary.json",
        serde_json::to_vec(&rows).unwrap(),
    )
    .unwrap();
    fs::write("target/experiment010/benchmark.csv", timings).unwrap();
    if !preview {
        let mut performance = String::from(
            "mode,retention,sample,run_validate_ms,serialize_ms,trial_bytes,attention_bytes,attention_records\n",
        );
        for mode in [
            world_of_individuals::attention::Mode::Unchanged,
            world_of_individuals::attention::Mode::CurrentNeed,
        ] {
            for method in [
                world_of_individuals::retention::Method::Fifo,
                world_of_individuals::retention::Method::Quality,
                world_of_individuals::retention::Method::Salient,
            ] {
                for sample in 0..5 {
                    let start = Instant::now();
                    let t = e::trial(42, method, mode, "protected", e::Config::default());
                    let run = start.elapsed().as_secs_f64() * 1000.;
                    let start = Instant::now();
                    let bytes = serde_json::to_vec(&t).unwrap();
                    let serialize = start.elapsed().as_secs_f64() * 1000.;
                    performance.push_str(&format!(
                        "{mode:?},{method:?},{sample},{run:.3},{serialize:.3},{},{},{}\n",
                        bytes.len(),
                        serde_json::to_vec(&t.final_state.attention).unwrap().len(),
                        t.final_state.attention.records.len()
                    ));
                }
            }
        }
        fs::write("target/experiment010/performance.csv", performance).unwrap();
    }
}
