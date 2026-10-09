//! Read-only archive and full-seed replay gate. Never regenerates historical files.
#[path = "support/contract009.rs"]
mod contract009;
#[path = "support/contract010.rs"]
mod contract010;
#[path = "support/contract011.rs"]
mod contract011;
use serde::Serialize;
use std::{fs, path::Path};
use world_of_individuals::{
    experiment as e1, experiment002 as e2, experiment003 as e3, experiment004 as e4,
    experiment005 as e5, experiment006 as e6, experiment007 as e7, experiment008 as e8,
};

fn archive(value: &impl Serialize, path: &str) {
    let expected: serde_json::Value = serde_json::from_slice(
        &fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).unwrap(),
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(value).unwrap(),
        expected,
        "archive {path}"
    );
}
fn readable(actual: String, path: &str) {
    let expected = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).unwrap();
    assert_eq!(
        actual.replace("\r\n", "\n"),
        expected.replace("\r\n", "\n"),
        "report {path}"
    );
}
fn main() {
    let first = e1::run_experiment(42, "all", 1000).unwrap();
    archive(&first, "experiments/001/results/seed-42/report.json");
    readable(
        e1::human_report(&first),
        "experiments/001/results/seed-42/report.txt",
    );
    macro_rules! check {
        ($module:ident, $number:literal) => {{
            let suite = $module::suite(42);
            archive(&suite, concat!("experiments/", $number, "/seed-42.json"));
            let population = $module::population(42, 1000);
            assert_eq!(population, $module::population(42, 1000));
            archive(
                &population,
                concat!("experiments/", $number, "/population.json"),
            );
            for seed in (0..128).chain(std::iter::once(u64::MAX)) {
                assert_eq!(
                    $module::suite(seed),
                    $module::suite(seed),
                    "replay {} seed {}",
                    $number,
                    seed
                );
            }
            println!(
                "{}: archive, population and 129 complete suite replays passed",
                $number
            );
        }};
    }
    check!(e2, "002");
    check!(e3, "003");
    check!(e4, "004");
    check!(e5, "005");
    check!(e6, "006");
    check!(e7, "007");
    readable(e3::human(&e3::suite(42)), "experiments/003/report.txt");
    readable(e4::human(&e4::suite(42)), "experiments/004/report.txt");
    readable(e5::human(&e5::suite(42)), "experiments/005/report.txt");
    readable(e6::human(&e6::suite(42)), "experiments/006/report.txt");
    readable(e7::human(&e7::suite(42)), "experiments/007/report.txt");
    for seed in (0..128).chain(std::iter::once(u64::MAX)) {
        let actual = e1::run_experiment(seed, "all", 1000).unwrap();
        assert_eq!(actual, e1::run_experiment(seed, "all", 1000).unwrap());
    }
    let suite = e8::suite(42);
    archive(&suite, "experiments/008/seed-42.json");
    readable(e8::human(&suite), "experiments/008/report.txt");
    for seed in (0..128).chain(std::iter::once(u64::MAX)) {
        let suite = e8::suite(seed);
        assert_eq!(suite, e8::suite(seed));
        for trial in suite {
            trial.final_state.validate().unwrap();
        }
    }
    for count in [100, 1000] {
        for mode in [e8::Mode::Legacy, e8::Mode::Max, e8::Mode::Grouped] {
            let population = e8::population(42, count, mode);
            population.validate().unwrap();
            assert_eq!(population, e8::population(42, count, mode));
            archive(
                &population,
                &format!("experiments/008/population-{count}-{mode:?}.json"),
            );
        }
    }
    println!(
        "001–008: exact archived JSON/readable reports, 129 seeded replays per mode, populations and 008 causal validation passed; no files written."
    );
    contract009::validate();
    contract010::validate();
    contract011::validate();
}
