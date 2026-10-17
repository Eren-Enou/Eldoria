//! Engineering wall-clock measurements, separate from deterministic evidence.
use std::{hint::black_box, time::Instant};
use world_of_individuals::{
    acquisition as a, experiment019 as old, experiment020 as f, priorities,
};
fn main() {
    let t = f::trial(42, f::Mode::Acquire, "contested", f::config("contested"));
    let d = t.periods[0].steps[0].choices[0].as_ref().unwrap();
    let input = a::Input {
        legacy: d.acquiring_base.as_ref().unwrap().contact.input.clone(),
        cache: d.cache.clone(),
        inquiry_lane_free: d.inquiry_lane_free,
    };
    let start = Instant::now();
    for _ in 0..10000 {
        black_box(a::policy(black_box(&input), a::Policy::Need));
    }
    println!(
        "020 candidate evaluations:10000,us:{}",
        start.elapsed().as_micros()
    );
    for mode in [f::Mode::Owned, f::Mode::Need, f::Mode::Acquire] {
        let start = Instant::now();
        for _ in 0..50 {
            black_box(f::trial(42, mode, "ongoing", f::config("ongoing")));
        }
        println!(
            "020 ongoing {mode:?}:50 trials,us:{}",
            start.elapsed().as_micros()
        );
        let t = f::trial(42, mode, "ongoing", f::config("ongoing"));
        let start = Instant::now();
        let bytes = serde_json::to_vec(&t).unwrap();
        println!(
            "020 ongoing {mode:?}:bytes:{},points:{},steps:{},claims:{},serialize_us:{}",
            bytes.len(),
            t.points.len(),
            t.periods.iter().map(|p| p.steps.len()).sum::<usize>(),
            t.records.len(),
            start.elapsed().as_micros()
        );
    }
    let start = Instant::now();
    for _ in 0..50 {
        black_box(old::trial(
            42,
            priorities::Mode::Baseline,
            "ongoing",
            Default::default(),
        ));
    }
    println!(
        "019 ongoing Baseline:50 trials,us:{}",
        start.elapsed().as_micros()
    );
}
