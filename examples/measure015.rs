use std::{hint::black_box, time::Instant};
use world_of_individuals::{encounters as e, experiment015 as f};
fn main() {
    let mut sim = f::setup(42, "information");
    let now = sim.activity_tick();
    sim.encounter_open(24, [now; 3]).unwrap();
    let wave = sim
        .encounter_wave(e::Mode::History, e::Order::Ascending, 0)
        .unwrap();
    let input = &wave.proposals[0].input;
    let response = &wave.responses[2].input;
    println!("kind,mode,repeat,calls,nanoseconds_per_call");
    for repeat in 0..5 {
        for mode in e::MODES {
            let start = Instant::now();
            for _ in 0..20000 {
                black_box(e::policy(black_box(input), black_box(mode)));
            }
            println!(
                "initiation,{mode:?},{repeat},20000,{}",
                start.elapsed().as_nanos() as f64 / 20000.0
            );
        }
        let start = Instant::now();
        for _ in 0..20000 {
            black_box(e::respond(black_box(response)));
        }
        println!(
            "response,Common,{repeat},20000,{}",
            start.elapsed().as_nanos() as f64 / 20000.0
        );
    }
}
