use std::time::Instant;
use world_of_individuals::{activity as a, experiment014 as e};
fn main() {
    let (mut sim, _) = e::setup(
        42,
        a::Mode::Local,
        "information_first",
        &e::Config::default(),
    );
    sim.activity_open(0, Some(2), Some(1), 40, 40, 25).unwrap();
    let input = sim.activity_input().unwrap();
    println!("workload,mode,repeat,calls,nanoseconds");
    for mode in a::MODES {
        for repeat in 0..5 {
            let start = Instant::now();
            for _ in 0..20_000 {
                std::hint::black_box(a::policy(std::hint::black_box(&input), mode));
            }
            println!(
                "fixed_selector,{mode:?},{repeat},20000,{}",
                start.elapsed().as_nanos()
            );
        }
    }
    for repeat in 0..5 {
        let start = Instant::now();
        for _ in 0..20_000 {
            std::hint::black_box(sim.activity_input().unwrap());
        }
        println!(
            "fixed_projection,Local,{repeat},20000,{}",
            start.elapsed().as_nanos()
        );
    }
}
