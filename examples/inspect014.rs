//! Additional13-tick coverage diagnostic; does not replace the preregistered matrix.
use world_of_individuals::{activity, experiment014 as e};
fn main() {
    let c = e::Config {
        budget: 13,
        ..Default::default()
    };
    let trials: Vec<_> = activity::MODES
        .into_iter()
        .map(|mode| e::trial(42, mode, "competition", c.clone()))
        .collect();
    println!("{}", serde_json::to_string(&trials).unwrap());
}
