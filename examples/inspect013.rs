//! Targeted diagnostic export to stdout; never writes experiment archives.
use world_of_individuals::experiment013 as e;
fn main() {
    let seeds = [1, 42];
    let config = e::held_out()[1].clone();
    let trials: Vec<_> = seeds
        .into_iter()
        .flat_map(|seed| {
            [e::Mode::Unchanged, e::Mode::Pulse].into_iter().map({
                let config = config.clone();
                move |mode| e::trial(seed, mode, "longitudinal", config.clone())
            })
        })
        .collect();
    println!("{}", serde_json::to_string(&trials).unwrap());
}
