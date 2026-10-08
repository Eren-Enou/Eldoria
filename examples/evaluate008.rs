use std::{collections::BTreeMap, fs, time::Instant};
use world_of_individuals::{
    experiment, experiment002, experiment003, experiment004, experiment005, experiment006,
    experiment007, experiment008 as e,
};
fn save(name: &str, value: &impl serde::Serialize) {
    fs::write(
        format!("experiments/008/{name}.json"),
        serde_json::to_vec_pretty(value).unwrap(),
    )
    .unwrap();
}
fn main() {
    fs::create_dir_all("experiments/008").unwrap();
    let trials = e::suite(42);
    save("seed-42", &trials);
    fs::write("experiments/008/report.txt", e::human(&trials)).unwrap();
    if std::env::args().any(|a| a == "--preview") {
        print!("{}", e::human(&trials));
        return;
    }
    let mut outcomes = BTreeMap::new();
    for seed in 0..128 {
        let suite = e::suite(seed);
        assert_eq!(suite, e::suite(seed));
        for t in &suite {
            let supports: Vec<_> = t.points.iter().map(|p| p.support).collect();
            *outcomes
                .entry(format!(
                    "{:?}:{}:{supports:?}:{:?}",
                    t.mode,
                    t.scenario,
                    e::choices(t)
                ))
                .or_insert(0u32) += 1;
        }
    }
    let read =
        |path: &str| serde_json::from_slice::<serde_json::Value>(&fs::read(path).unwrap()).unwrap();
    let c = serde_json::json!({
        "001":serde_json::to_value(experiment::run_experiment(42,"all",1000).unwrap()).unwrap()==read("experiments/001/results/seed-42/report.json"),
        "002":serde_json::to_value(experiment002::suite(42)).unwrap()==read("experiments/002/seed-42.json"),
        "003":serde_json::to_value(experiment003::suite(42)).unwrap()==read("experiments/003/seed-42.json"),
        "004":serde_json::to_value(experiment004::suite(42)).unwrap()==read("experiments/004/seed-42.json"),
        "005":serde_json::to_value(experiment005::suite(42)).unwrap()==read("experiments/005/seed-42.json"),
        "006":serde_json::to_value(experiment006::suite(42)).unwrap()==read("experiments/006/seed-42.json"),
        "007":serde_json::to_value(experiment007::suite(42)).unwrap()==read("experiments/007/seed-42.json")});
    assert!(c.as_object().unwrap().values().all(|v| v == true));
    save("compatibility", &c);
    let mut timing = String::from(
        "mode,population,sample,build_run_snapshot_ms,serialize_ms,bytes,assessment_records\n",
    );
    for count in [100, 1000] {
        for mode in [e::Mode::Legacy, e::Mode::Max, e::Mode::Grouped] {
            for sample in 0..3 {
                let start = Instant::now();
                let snapshot = e::population(42, count, mode);
                let elapsed = start.elapsed();
                let start = Instant::now();
                let bytes = serde_json::to_vec(&snapshot).unwrap();
                let serialize = start.elapsed();
                assert_eq!(snapshot, e::population(42, count, mode));
                timing.push_str(&format!(
                    "{mode:?},{count},{sample},{:.6},{:.6},{},{}\n",
                    elapsed.as_secs_f64() * 1000.,
                    serialize.as_secs_f64() * 1000.,
                    bytes.len(),
                    snapshot.assessment.as_ref().map_or(0, |a| a.records.len())
                ));
                if sample == 0 {
                    save(&format!("population-{count}-{mode:?}"), &snapshot);
                }
            }
        }
    }
    fs::write("experiments/008/population-benchmark.csv", timing).unwrap();
    save(
        "summary",
        &serde_json::json!({"rules":world_of_individuals::assessment::RULES,"seeds":"0..128","trials":128*trials.len(),"replayed_suites":128,"outcomes":outcomes,"compatibility":c}),
    );
    println!(
        "Experiment 008: {} controlled trials plus replays, archive equality and population replay passed.",
        128 * trials.len()
    );
}
