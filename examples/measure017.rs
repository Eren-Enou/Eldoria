use std::{hint::black_box, time::Instant};
use world_of_individuals::{experiment017 as f, opportunities as o};
fn resolver_only() {
    println!("kind,batch,iterations,nanoseconds,resource_events");
    for batch in 0..5 {
        let mut worlds = (0..20)
            .map(|_| {
                let mut s = f::setup(42, "competing_eater");
                s.add_scene([1, 0], 1, 6).unwrap();
                s
            })
            .collect::<Vec<_>>();
        let start = Instant::now();
        for s in &mut worlds {
            black_box(s).run();
        }
        let elapsed = start.elapsed().as_nanos();
        let events = worlds.iter().map(|s| s.events().len()).sum::<usize>();
        println!("legacy_resolver_run,{batch},20,{elapsed},{events}");
        let mut worlds = (0..20)
            .map(|_| {
                let mut s = f::setup(42, "competing_eater");
                s.opportunity_open(
                    o::Mode::Active,
                    world_of_individuals::encounters::Order::Ascending,
                    24,
                )
                .unwrap();
                let mut points = vec![];
                s.opportunity_step(&mut points).unwrap();
                (s, points)
            })
            .collect::<Vec<_>>();
        let start = Instant::now();
        for (s, p) in &mut worlds {
            black_box(s.opportunity_step(p).unwrap());
        }
        let elapsed = start.elapsed().as_nanos();
        let events = worlds.iter().map(|(s, _)| s.events().len()).sum::<usize>();
        println!("resource_tick_with_policy_capture,{batch},20,{elapsed},{events}");
    }
}
fn main() {
    if std::env::args().any(|a| a == "--resolver-only") {
        resolver_only();
        return;
    }
    println!("kind,mode,batch,iterations,nanoseconds,bytes");
    let t = f::trial(42, o::Mode::Active, "competing_eater", f::Config::default());
    let input = &t.periods[0].steps[0].decisions[0]
        .as_ref()
        .unwrap()
        .contact
        .input;
    for mode in o::MODES {
        for batch in 0..5 {
            let start = Instant::now();
            for _ in 0..20000 {
                black_box(o::policy(black_box(input), mode));
            }
            println!(
                "policy,{mode:?},{batch},20000,{},0",
                start.elapsed().as_nanos()
            );
            let start = Instant::now();
            black_box(f::trial(42, mode, "competing_eater", f::Config::default()));
            println!("world,{mode:?},{batch},1,{},0", start.elapsed().as_nanos());
        }
    }
    let t = f::trial(42, o::Mode::Active, "longitudinal", f::Config::default());
    for batch in 0..5 {
        let start = Instant::now();
        let bytes = serde_json::to_vec(black_box(&t)).unwrap();
        println!(
            "serialization,Active,{batch},1,{},{}",
            start.elapsed().as_nanos(),
            bytes.len()
        );
    }
    let mut references = 0usize;
    let mut expanded_bytes = 0usize;
    for s in t.periods.iter().flat_map(|p| &p.steps) {
        for id in [
            Some(s.before),
            Some(s.observed),
            s.sent,
            s.admitted,
            Some(s.after),
        ]
        .into_iter()
        .flatten()
        {
            references += 1;
            expanded_bytes += serde_json::to_vec(&t.points[id]).unwrap().len();
        }
    }
    eprintln!(
        "{}",
        serde_json::json!({"case":"longitudinal","mode":"Active","seed":42,"commitment_value_bytes":std::mem::size_of::<o::Commitment>(),"max_active_commitments":1,"new_cognitive_cells":0,"trial_bytes":serde_json::to_vec(&t).unwrap().len(),"points":t.points.len(),"point_table_bytes":serde_json::to_vec(&t.points).unwrap().len(),"point_references":references,"expanded_referenced_point_bytes":expanded_bytes,"steps":t.periods.iter().map(|p|p.steps.len()).sum::<usize>()})
    );
}
