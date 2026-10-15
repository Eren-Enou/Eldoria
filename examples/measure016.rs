use std::{hint::black_box, time::Instant};
use world_of_individuals::{contact_learning as c, encounters as e, experiment016 as f};
fn main() {
    let mut sim = f::setup(42, "information");
    let tick = sim.activity_tick();
    sim.encounter_open(24, [tick; 3]).unwrap();
    sim.encounter_prepare().unwrap();
    let own = sim
        .encounter_input(0, &sim.encounter_people(), tick + 24)
        .unwrap();
    let signal = c::Signal {
        owner: 0,
        partner: 2,
        kind: e::Kind::Inquiry,
        sent: tick - 2,
        received: tick,
        outcome: c::Outcome::Declined,
    };
    println!("operation,mode,repeat,calls,mean_nanoseconds");
    for mode in c::MODES {
        let cells = c::update(&[], mode, signal.clone()).after;
        let input = c::Input {
            own: own.clone(),
            mode,
            retained: cells.clone(),
        };
        for repeat in 0..5 {
            let now = Instant::now();
            for _ in 0..20000 {
                black_box(c::policy(black_box(&input)));
            }
            println!(
                "policy,{mode:?},{repeat},20000,{}",
                now.elapsed().as_nanos() as f64 / 20000.0
            );
            let now = Instant::now();
            for _ in 0..20000 {
                black_box(c::update(
                    black_box(&cells),
                    mode,
                    black_box(signal.clone()),
                ));
            }
            println!(
                "update,{mode:?},{repeat},20000,{}",
                now.elapsed().as_nanos() as f64 / 20000.0
            );
        }
    }
    let trial = f::trial(42, c::Mode::Typed, "longitudinal", f::Config::default());
    for repeat in 0..5 {
        let now = Instant::now();
        black_box(serde_json::to_vec(black_box(&trial)).unwrap());
        println!("serialize,Typed,{repeat},1,{}", now.elapsed().as_nanos());
    }
    let waves = trial
        .periods
        .iter()
        .flat_map(|p| &p.waves)
        .collect::<Vec<_>>();
    let wrapped: usize = waves
        .iter()
        .map(|w| serde_json::to_vec(w).unwrap().len())
        .sum();
    let base: usize = waves
        .iter()
        .map(|w| serde_json::to_vec(&w.wave).unwrap().len())
        .sum();
    let storage = serde_json::json!({"case":"longitudinal","mode":"Typed","seed":42,"cell_value_bytes":std::mem::size_of::<c::Cell>(),"logical_cap_per_actor":2,"wave_count":waves.len(),"wrapped_wave_bytes":wrapped,"base_wave_bytes":base,"feedback_wrapper_bytes":wrapped-base,"trial_bytes":serde_json::to_vec(&trial).unwrap().len()});
    let output = std::env::args()
        .nth(1)
        .expect("explicit storage JSON output path");
    std::fs::write(output, serde_json::to_vec_pretty(&storage).unwrap()).unwrap();
}
