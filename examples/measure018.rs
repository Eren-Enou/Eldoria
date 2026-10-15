use std::{fs, hint::black_box, time::Instant};
use world_of_individuals::{
    encounters as e, experiment018 as f, opportunities as o, recurring as r,
};
fn resolver_measurements() {
    println!("kind,mode,batch,iterations,nanoseconds,resource_events,queries,need_updates");
    for batch in 0..5 {
        for mode in [r::Mode::Fixed, r::Mode::Recurring] {
            for query in [false, true] {
                let (mut elapsed, mut events, mut queries, mut updates) = (0, 0, 0, 0);
                for _ in 0..20 {
                    let mut sim = f::setup(
                        42,
                        if query { "information" } else { "ongoing" },
                        &Default::default(),
                    );
                    sim.recurring_open(mode, e::Order::Ascending, 48, 4)
                        .unwrap();
                    let mut points = vec![];
                    if !query {
                        sim.recurring_step(&mut points).unwrap();
                        sim.recurring_step(&mut points).unwrap();
                    }
                    let before = sim.recurring_point();
                    let start = Instant::now();
                    let step = sim.recurring_step(&mut points).unwrap();
                    elapsed += start.elapsed().as_nanos();
                    let after = sim.recurring_point();
                    events += after.state.events - before.state.events;
                    queries += after.state.queries - before.state.queries;
                    updates += step.need_after - step.need_before;
                }
                assert_eq!((events, queries), if query { (0, 20) } else { (20, 0) });
                println!(
                    "{},{mode:?},{batch},20,{elapsed},{events},{queries},{updates}",
                    if query {
                        "query_activity_with_handshake"
                    } else {
                        "resource_tick_with_completion"
                    }
                );
            }
        }
    }
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.iter().any(|a| a == "--resolver-only") {
        resolver_measurements();
        return;
    }
    let output = args.get(1).expect("observer-cost JSON output path");
    println!("kind,mode,batch,iterations,nanoseconds,resource_events,queries,need_updates");
    for batch in 0..5 {
        for increment in [0, 5] {
            let mut clock = r::Clock {
                epoch: 0,
                last: 0,
                settings: r::Settings {
                    interval: 4,
                    increment,
                },
            };
            let mut hunger = [20; 3];
            let start = Instant::now();
            for _ in 0..100000 {
                let to = clock.last + 1;
                let pulses = black_box(clock).pulses(to);
                hunger = black_box(hunger.map(|h| r::pressure(h, pulses, increment)));
                clock.last = to;
            }
            black_box(hunger);
            println!(
                "pure_need,{increment},{batch},100000,{},0,0,0",
                start.elapsed().as_nanos()
            );
        }
        for mode in r::MODES {
            let mut s = f::setup(42, "ongoing", &Default::default());
            s.recurring_open(mode, e::Order::Ascending, 24, 4).unwrap();
            let people = (0..3)
                .map(|id| e::Public {
                    id,
                    available: true,
                })
                .collect::<Vec<_>>();
            let input = s.encounter_input(0, &people, 24).unwrap();
            let start = Instant::now();
            for _ in 0..20000 {
                black_box(r::policy(black_box(&input), mode));
            }
            println!(
                "pure_policy,{mode:?},{batch},20000,{},0,0,0",
                start.elapsed().as_nanos()
            );
            let start = Instant::now();
            let t = f::trial(
                42,
                mode,
                "ongoing",
                f::Config {
                    long_periods: 6,
                    ..Default::default()
                },
            );
            let elapsed = start.elapsed().as_nanos();
            let summary = f::summary(&t);
            println!(
                "world,{mode:?},{batch},1,{elapsed},{},{},{}",
                t.points[t.periods.last().unwrap().after].state.events - t.initial.state.events,
                summary.queries,
                summary.need_updates
            );
        }
    }
    let t = f::trial(42, r::Mode::Recurring, "ongoing", f::Config::default());
    let mut bytes = 0;
    for batch in 0..5 {
        let start = Instant::now();
        let encoded = serde_json::to_vec(&t).unwrap();
        let elapsed = start.elapsed().as_nanos();
        bytes = encoded.len();
        black_box(encoded);
        println!("serialize,Recurring,{batch},1,{elapsed},0,0,0");
    }
    let steps = t.periods.iter().map(|p| p.steps.len()).sum::<usize>();
    let points = serde_json::to_vec(&t.points).unwrap().len();
    let need_bytes = serde_json::to_vec(&t.needs).unwrap().len();
    let policy_bytes = t
        .periods
        .iter()
        .flat_map(|p| &p.steps)
        .flat_map(|s| s.activity.decisions.iter().flatten())
        .map(|d| serde_json::to_vec(d).unwrap().len())
        .sum::<usize>();
    let value = serde_json::json!({"seed":42,"case":"ongoing","mode":"Recurring","periods":24,"clock_value_bytes":std::mem::size_of::<r::Clock>(),"need_record_value_bytes":std::mem::size_of::<r::Record>(),"commitment_value_bytes":std::mem::size_of::<o::Commitment>(),"max_commitments":1,"new_cognitive_cells":0,"steps":steps,"points":t.points.len(),"point_table_json_bytes":points,"need_records":t.needs.len(),"need_records_json_bytes":need_bytes,"policy_decisions_json_bytes":policy_bytes,"trial_json_bytes":bytes});
    fs::write(output, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
}
