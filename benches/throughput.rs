use std::time::Instant;
use world_of_individuals::{model::Generator, simulation::Simulation};

fn main() {
    println!("population,rounds,init_ms,run_ms,events,events_per_second");
    for population in [100, 1000] {
        let rounds = 100;
        let mut init_time = std::time::Duration::ZERO;
        let mut run_time = std::time::Duration::ZERO;
        let mut events = 0;
        for seed in 0..rounds {
            let start = Instant::now();
            let mut generator = Generator::new(seed);
            let mut sim =
                Simulation::new((0..population).map(|id| generator.agent(id)).collect()).unwrap();
            for id in (0..population).step_by(2) {
                sim.add_scene([id, id + 1], 1, 6).unwrap();
            }
            init_time += start.elapsed();
            let start = Instant::now();
            sim.run();
            run_time += start.elapsed();
            events += sim.events().len();
        }
        println!(
            "{population},{rounds},{:.3},{:.3},{events},{:.0}",
            init_time.as_secs_f64() * 1000.,
            run_time.as_secs_f64() * 1000.,
            events as f64 / run_time.as_secs_f64()
        );
    }
}
