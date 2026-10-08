//! Read-only009 archive comparison. Decompress archived gzip into target first.
use std::fs;
use world_of_individuals::experiment009 as e;
fn main() {
    for (held, name) in [(false, "seed-42"), (true, "held-out-seed-42")] {
        let archived: Vec<e::Trial> =
            serde_json::from_slice(&fs::read(format!("target/compat009/{name}.json")).unwrap())
                .unwrap();
        for trial in &archived {
            trial.initial.validate().unwrap();
            trial.final_state.validate().unwrap();
        }
        assert_eq!(archived, e::suite(42, held));
    }
    for seed in (0..128).chain([u64::MAX]) {
        for held in [false, true] {
            assert_eq!(e::suite(seed, held), e::suite(seed, held));
        }
    }
    println!(
        "009 archived controlled/held-out trials match;129 seeds exact replay; no archive writes"
    );
}
