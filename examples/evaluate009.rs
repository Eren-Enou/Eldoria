//! New009 evidence only; all wall-clock values go to a separate CSV.
use std::{collections::BTreeMap, fs, time::Instant};
use world_of_individuals::{experiment009 as e, retention::Method};
fn save(name: &str, value: &impl serde::Serialize) {
    fs::write(
        format!(
            "{}/{name}.json",
            if name.ends_with("seed-42") {
                "target/experiment009"
            } else {
                "experiments/009"
            }
        ),
        serde_json::to_vec_pretty(value).unwrap(),
    )
    .unwrap();
}
fn human(trials: &[e::Trial]) -> String {
    let mut out = String::from(
        "Experiment009: Grouped assessment, capacity32, retention controls; probes are local choices, not transfers.\n",
    );
    for t in trials {
        out.push_str(&format!("\n{} {:?} {:?}\n", t.scenario, t.method, t.config));
        for p in &t.points {
            out.push_str(&format!(
                "  {}: basis{} items{} origins{} belief{:?} concern{:?} trust{} action{:?}\n",
                p.label,
                p.basis_support,
                p.target_items,
                p.target_origins,
                p.belief,
                p.concern,
                p.trust,
                p.action
            ));
        }
    }
    out
}
fn main() {
    let preview = std::env::args().any(|a| a == "--preview");
    let trials = e::suite(42, false);
    if preview {
        print!("{}", human(&trials));
        return;
    }
    fs::create_dir_all("experiments/009").unwrap();
    fs::create_dir_all("target/experiment009").unwrap();
    save("seed-42", &trials);
    fs::write("experiments/009/report.txt", human(&trials)).unwrap();
    let held = e::suite(42, true);
    save("held-out-seed-42", &held);
    fs::write("experiments/009/held-out-report.txt", human(&held)).unwrap();
    let mut outcomes = BTreeMap::new();
    for seed in 0..128 {
        for held in [false, true] {
            let suite = e::suite(seed, held);
            assert_eq!(suite, e::suite(seed, held));
            for t in suite {
                let p = t.points.last().unwrap();
                let key = format!(
                    "held={held}:{}:{:?}:{:?}:{}:{}:{}:{:?}",
                    t.scenario,
                    t.config,
                    t.method,
                    p.basis_support,
                    p.target_origins,
                    p.positive && p.negative,
                    p.action
                );
                *outcomes.entry(key).or_insert(0usize) += 1;
            }
        }
        if seed % 16 == 0 {
            println!("seed {seed}: controlled+held-out exact replay passed");
        }
    }
    let mut timing = String::from(
        "scenario,method,sample,run_snapshot_validation_ms,serialize_ms,bytes,records,live_items\n",
    );
    let mut bytes = BTreeMap::new();
    for method in [Method::Fifo, Method::Quality, Method::Salient] {
        for sample in 0..5 {
            let start = Instant::now();
            let t = e::trial(42, method, "unresolved_concern", e::Config::default());
            let run = start.elapsed();
            let start = Instant::now();
            let data = serde_json::to_vec(&t).unwrap();
            let serialize = start.elapsed();
            timing.push_str(&format!(
                "unresolved_concern,{method:?},{sample},{:.6},{:.6},{},{},{}\n",
                run.as_secs_f64() * 1000.,
                serialize.as_secs_f64() * 1000.,
                data.len(),
                t.final_state.retention.records.len(),
                t.points.last().unwrap().live_items
            ));
            bytes.insert(format!("{method:?}"), data.len());
        }
    }
    fs::write("experiments/009/benchmark.csv", timing).unwrap();
    let mut population_timing = String::from(
        "population,method,sample,build_run_validation_ms,serialize_ms,bytes,audit_bytes,live_assessment_bytes,records,max_items\n",
    );
    let mut populations = vec![];
    for count in [100, 1000] {
        for method in [
            None,
            Some(Method::Fifo),
            Some(Method::Quality),
            Some(Method::Salient),
        ] {
            for sample in 0..3 {
                let start = Instant::now();
                let sim = e::population_sim(42, count, method);
                let (state, retention) = if method.is_some() {
                    let full = e::snapshot(&sim);
                    full.validate().unwrap();
                    (full.base, Some(full.retention))
                } else {
                    let state = world_of_individuals::experiment008::snapshot(&sim);
                    state.validate().unwrap();
                    (state, None)
                };
                let run = start.elapsed();
                let start = Instant::now();
                let data = serde_json::to_vec(&(&state, &retention)).unwrap();
                let serialize = start.elapsed();
                let basis = state.assessment.as_ref().unwrap();
                let live = serde_json::to_vec(&basis.items).unwrap().len();
                let audit = retention
                    .as_ref()
                    .map(|r| serde_json::to_vec(&r.records).unwrap().len())
                    .unwrap_or(0);
                let label = method
                    .map(|m| format!("{m:?}"))
                    .unwrap_or("Frozen008".into());
                population_timing.push_str(&format!(
                    "{count},{label},{sample},{:.6},{:.6},{},{audit},{live},{},{}\n",
                    run.as_secs_f64() * 1000.,
                    serialize.as_secs_f64() * 1000.,
                    data.len(),
                    basis.records.len(),
                    basis.items.values().map(|v| v.len()).max().unwrap_or(0)
                ));
                if sample == 0 {
                    populations.push(serde_json::json!({"population":count,"method":label,"snapshot_bytes":data.len(),"retention_audit_bytes":audit,"live_assessment_bytes":live,"records":basis.records.len()}));
                }
            }
        }
    }
    fs::write(
        "experiments/009/population-benchmark.csv",
        population_timing,
    )
    .unwrap();
    save("populations", &populations);
    save(
        "layout",
        &serde_json::json!({
            "item_size":std::mem::size_of::<world_of_individuals::assessment::Item>(),
            "concern_size":std::mem::size_of::<world_of_individuals::concerns::Concern>(),
            "candidate_size":std::mem::size_of::<world_of_individuals::retention::Candidate>(),
            "retention_resource_size":std::mem::size_of::<world_of_individuals::retention::Retention>(),
            "policy_function_pointer_size":std::mem::size_of::<fn(&world_of_individuals::retention::Input)->world_of_individuals::retention::Decision>(),
            "new_persistent_per_individual_score_slots":0,
            "candidate_budget":33,"concern_budget":8,"existing_item_budget":32,
            "note":"Rust payload sizes; excludes Vec allocations, ECS headers, audit records and process RSS"
        }),
    );
    save(
        "summary",
        &serde_json::json!({"rules":world_of_individuals::retention::RULES,"capacity":world_of_individuals::assessment::CAPACITY,
        "seeds":"0..128","controlled_trials":128*trials.len(),"held_out_trials":128*held.len(),"outcomes":outcomes,
        "serialized_control_bytes":bytes,"controlled_seed42":trials.iter().map(|t|(&t.scenario,t.method,t.points.last().unwrap())).collect::<Vec<_>>(),
        "held_out_seed42":held.iter().map(|t|(&t.scenario,&t.config,t.method,t.points.last().unwrap())).collect::<Vec<_>>() }),
    );
    println!(
        "009: {} controlled and {} held-out trials plus exact replays passed",
        128 * trials.len(),
        128 * held.len()
    );
}
