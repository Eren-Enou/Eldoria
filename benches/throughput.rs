use std::time::Instant;
use world_of_individuals::{behavior::scarcity_policy, cognition::EvidenceKind, model::Outcome};
use world_of_individuals::{model::Generator, simulation::Simulation};

fn main() {
    println!("mode,population,rounds,init_ms,run_ms,records,records_per_second");
    // Workloads 6/7 compare identical three-encounter workloads with/without concerns.
    for workload in 1..=9 {
        let mode = if workload >= 6 {
            workload - 2
        } else {
            workload
        };
        for population in [100, 1000] {
            let rounds = 100;
            let mut init_time = std::time::Duration::ZERO;
            let mut run_time = std::time::Duration::ZERO;
            let mut events = 0;
            for seed in 0..rounds {
                let start = Instant::now();
                let mut generator = Generator::new(seed);
                let mut sim =
                    Simulation::new((0..population).map(|id| generator.agent(id)).collect())
                        .unwrap();
                if mode >= 2 {
                    sim.set_policy(scarcity_policy);
                }
                if mode == 3 {
                    sim.enable_intentional().unwrap();
                }
                if mode >= 4 {
                    sim.enable_foresight(
                        seed,
                        world_of_individuals::foresight::Sensor {
                            effort: 5,
                            ..Default::default()
                        },
                    )
                    .unwrap();
                }
                if mode >= 5 {
                    sim.enable_concerns().unwrap();
                }
                if mode >= 6 {
                    sim.enable_inquiry().unwrap();
                }
                if mode == 7 {
                    sim.enable_provenance().unwrap();
                }
                for id in (0..population).step_by(2) {
                    sim.add_scene([id, id + 1], 1, 6).unwrap();
                }
                init_time += start.elapsed();
                let start = Instant::now();
                sim.run();
                if workload >= 6 {
                    for _ in 0..2 {
                        for id in (0..population).step_by(2) {
                            sim.add_scene([id, id + 1], 1, 6).unwrap();
                        }
                        sim.run();
                    }
                }
                if mode == 2 {
                    for event in sim.events() {
                        if event.outcome == Some(Outcome::Refusal) {
                            let speaker = event.decision.actor;
                            let listener = *event
                                .participants
                                .iter()
                                .find(|&&id| id != speaker)
                                .unwrap();
                            sim.communicate(speaker, listener, event.id, EvidenceKind::Disclosure)
                                .unwrap();
                            events += 1;
                        }
                    }
                }
                if mode >= 2 {
                    for id in 0..population {
                        sim.consume(id).unwrap();
                        events += 1;
                    }
                }
                run_time += start.elapsed();
                if mode >= 3 {
                    events += sim.intentional().unwrap().records.len();
                }
                if mode == 5 {
                    events += sim.concerns().unwrap().records.len();
                }
                if mode == 6 {
                    events += sim.inquiry().unwrap().records.len();
                }
                if mode == 7 {
                    events += sim.provenance().unwrap().queries.len();
                }
                events += sim.events().len();
            }
            println!(
                "{},{population},{rounds},{:.3},{:.3},{events},{:.0}",
                format_args!(
                    "experiment00{mode}{}",
                    if workload >= 6 {
                        "_three_encounters"
                    } else {
                        ""
                    }
                ),
                init_time.as_secs_f64() * 1000.,
                run_time.as_secs_f64() * 1000.,
                events as f64 / run_time.as_secs_f64()
            );
        }
    }
}
