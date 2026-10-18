//! Wall-clock engineering measurements, excluded from deterministic histories.
use std::{hint::black_box, time::Instant};
use world_of_individuals::{
    acquisition as a, environmental_knowledge as k, experiment020 as old, experiment021 as f,
};
fn main() {
    let t = f::trial(
        42,
        a::Policy::Need,
        k::Mode::LastSeen,
        "contested",
        f::config("contested"),
    );
    let d = t.periods[0].steps[0].choices[0].as_ref().unwrap();
    let input = k::Input {
        legacy: d.acquiring_base.as_ref().unwrap().contact.input.clone(),
        retained: d.retained.clone(),
        duration: d.duration,
        inquiry_lane_free: d.inquiry_lane_free,
    };
    let start = Instant::now();
    for _ in 0..10000 {
        black_box(k::policy(black_box(&input), a::Policy::Need));
    }
    println!(
        "021 candidate evaluations:10000,us:{}",
        start.elapsed().as_micros()
    );
    for mode in k::MODES {
        let start = Instant::now();
        for _ in 0..20 {
            black_box(f::trial(
                42,
                a::Policy::Need,
                mode,
                "ongoing",
                f::config("ongoing"),
            ));
        }
        let execution = start.elapsed().as_micros();
        let t = f::trial(42, a::Policy::Need, mode, "ongoing", f::config("ongoing"));
        let start = Instant::now();
        let bytes = serde_json::to_vec(&t).unwrap();
        println!(
            "021 ongoing {mode:?}:20 trials,us:{execution},bytes:{},memory_records:{},memory_audit_bytes:{},steps:{},serialize_us:{}",
            bytes.len(),
            t.memory_records.len(),
            serde_json::to_vec(&t.memory_records).unwrap().len(),
            t.periods.iter().map(|p| p.steps.len()).sum::<usize>(),
            start.elapsed().as_micros()
        );
    }
    let start = Instant::now();
    for _ in 0..20 {
        black_box(old::trial(
            42,
            old::Mode::Need,
            "ongoing",
            old::config("ongoing"),
        ));
    }
    let execution = start.elapsed().as_micros();
    let t = old::trial(42, old::Mode::Need, "ongoing", old::config("ongoing"));
    let bytes = serde_json::to_vec(&t).unwrap();
    println!(
        "020 matched fresh ongoing Need:20 trials,us:{execution},bytes:{}",
        bytes.len()
    );
}
