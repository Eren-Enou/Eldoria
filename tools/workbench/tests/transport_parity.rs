//! Read-only original-producer comparison. Outputs are new temporary engineering
//! files; originals and frozen archives are never regenerated in place.
use std::{fs, path::PathBuf, process::Command};
use woi_workbench::{
    config::{Configuration, Mode, SCENARIOS, Valuation},
    storage,
};
#[test]
#[ignore = "set WOI_PORTABLE_RUNNER to the release runner; build original023/024 producers first"]
fn all_public_cases_match_original_serialized_history_exactly() {
    let runner =
        PathBuf::from(std::env::var_os("WOI_PORTABLE_RUNNER").expect("WOI_PORTABLE_RUNNER"))
            .canonicalize()
            .unwrap();
    let repo = woi_workbench::repository();
    let root = std::env::temp_dir().join(format!(
        "WOI Transport Parity {} {}",
        std::process::id(),
        storage::now()
    ));
    fs::create_dir(&root).unwrap();
    let system = PathBuf::from(std::env::var_os("SYSTEMROOT").unwrap()).join("System32");
    let mut count = 0;
    for (n, case) in SCENARIOS.into_iter().enumerate() {
        for policy in [None, Some(Valuation::Baseline), Some(Valuation::MealRelief)] {
            let c = Configuration {
                schema_version: if policy.is_none() { 1 } else { 2 },
                mode: if policy.is_none() {
                    Mode::ContinuousWorld
                } else {
                    Mode::ActivityValuation
                },
                scenario: case.into(),
                seed: [0, 42, u64::MAX][n % 3],
                ticks: [1, 64, 192, 1024][n % 4],
                period: [1, 11, 128][n % 3],
                valuation: policy,
                preparation: None,
            };
            let dir = root.join(format!("case-{n}-{}", c.policy_label()));
            fs::create_dir(&dir).unwrap();
            let config = dir.join("configuration.json");
            storage::write_new(&config, &c).unwrap();
            let original = dir.join("original.jsonl");
            let history = dir.join("history.jsonl");
            let mut old =
                Command::new(repo.join(format!("target/{0}/release/{0}.exe", c.mode.runner())));
            old.arg("run").arg(&original).args([
                "--case",
                case,
                "--ticks",
                &c.ticks.to_string(),
                "--period",
                &c.period.to_string(),
                "--seed",
                &c.seed.to_string(),
                "--claim",
                "4",
                "--phase",
                "Epoch",
                "--order",
                "Ascending",
            ]);
            if let Some(policy) = policy {
                old.args([
                    "--valuation",
                    match policy {
                        Valuation::Baseline => "Baseline",
                        Valuation::MealRelief => "MealRelief",
                    },
                ]);
            } else {
                old.args(["--mode", "Continuous"]);
            }
            old.current_dir(&root).env("PATH", &system);
            let a = old.output().unwrap();
            assert!(a.status.success(), "{}", String::from_utf8_lossy(&a.stderr));
            let b = Command::new(&runner)
                .arg("run")
                .arg(&config)
                .arg(&history)
                .current_dir(&root)
                .env("PATH", &system)
                .output()
                .unwrap();
            assert!(b.status.success(), "{}", String::from_utf8_lossy(&b.stderr));
            assert!(
                fs::read(&original).unwrap() == fs::read(&history).unwrap(),
                "Byte mismatch: {case} {policy:?}; inspect {}",
                dir.display()
            );
            count += 1;
            fs::remove_file(original).unwrap();
            fs::remove_file(history).unwrap();
        }
    }
    assert_eq!(count, 36);
    println!(
        "36 exact original/portable byte comparisons; engineering configs retained at {}",
        root.display()
    );
}
