//! Post-evaluation extraction of already preregistered seed1 worlds, no new rules.
use world_of_individuals::{contact_learning as c, encounters as e, experiment016 as f};
fn main() {
    let output = std::env::args()
        .nth(1)
        .expect("explicit isolated JSON output path");
    let mut ts = vec![];
    for order in [e::Order::Ascending, e::Order::Rotating] {
        for mode in c::MODES {
            ts.push(f::trial(
                1,
                mode,
                "decline",
                f::Config {
                    order,
                    ..Default::default()
                },
            ));
        }
    }
    assert_eq!(ts.len(), 10);
    std::fs::write(output, serde_json::to_vec(&ts).unwrap()).unwrap();
}
