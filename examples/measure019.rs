use std::{fs, hint::black_box, time::Instant};
use world_of_individuals::{
    encounters as e, experiment019 as f, opportunities as o, priorities as p, recurring as r,
};
fn main() {
    let output = std::env::args()
        .nth(1)
        .expect("observer-cost JSON output path");
    println!("kind,mode,batch,iterations,nanoseconds,resource_events,queries,need_updates");
    for batch in 0..5 {
        for increment in [0, 5] {
            let mut c = r::Clock {
                epoch: 0,
                last: 0,
                settings: r::Settings {
                    interval: 4,
                    increment,
                },
            };
            let mut h = [20; 3];
            let start = Instant::now();
            for _ in 0..100000 {
                let to = c.last + 1;
                let n = black_box(c).pulses(to);
                h = black_box(h.map(|h| r::pressure(h, n, increment)));
                c.last = to;
            }
            black_box(h);
            println!(
                "pure_need,{increment},{batch},100000,{},0,0,0",
                start.elapsed().as_nanos()
            );
        }
        for mode in p::MODES {
            let mut s = f::setup(42, "ongoing", &Default::default());
            s.recurring_open(mode.legacy(), e::Order::Ascending, 24, 4)
                .unwrap();
            let people = (0..3)
                .map(|id| e::Public {
                    id,
                    available: true,
                })
                .collect::<Vec<_>>();
            let input = s.encounter_input(0, &people, 24).unwrap();
            let start = Instant::now();
            for _ in 0..20000 {
                black_box(p::policy(black_box(&input), mode));
            }
            println!(
                "pure_policy,{mode:?},{batch},20000,{},0,0,0",
                start.elapsed().as_nanos()
            );
            let start = Instant::now();
            let t = f::trial(42, mode, "ongoing", Default::default());
            let elapsed = start.elapsed().as_nanos();
            let summary = f::summary(&t);
            println!(
                "world,{mode:?},{batch},1,{elapsed},{},{},{}",
                t.points[t.periods.last().unwrap().after].state.events - t.initial.state.events,
                summary.queries,
                summary.need_updates
            );
        }
        let mut s = f::setup(42, "information", &Default::default());
        s.recurring_open(p::Mode::Baseline.legacy(), e::Order::Ascending, 24, 4)
            .unwrap();
        let people = (0..3)
            .map(|id| e::Public {
                id,
                available: true,
            })
            .collect::<Vec<_>>();
        let end = s.activity_tick() + 24;
        let start = Instant::now();
        for _ in 0..2000 {
            black_box(s.encounter_input(0, black_box(&people), end).unwrap());
        }
        println!(
            "input_construction,Baseline,{batch},2000,{},0,0,0",
            start.elapsed().as_nanos()
        );
        let start = Instant::now();
        for now in 0..100000 {
            black_box(p::room_after_meal(
                black_box(now),
                black_box(now + 24),
                black_box(15),
            ));
        }
        println!(
            "public_admission_arithmetic,Deadline,{batch},100000,{},0,0,0",
            start.elapsed().as_nanos()
        );
        let mut s = f::setup(42, "ongoing", &Default::default());
        s.recurring_open(p::Mode::Baseline.legacy(), e::Order::Ascending, 128, 4)
            .unwrap();
        let start = Instant::now();
        for _ in 0..100 {
            s.priorities_idle(1).unwrap();
        }
        println!(
            "clock_sync_with_guard_audit,Baseline,{batch},100,{},0,0,100",
            start.elapsed().as_nanos()
        );
        for query in [false, true] {
            let (mut elapsed, mut events, mut queries, mut updates) = (0, 0, 0, 0);
            for _ in 0..20 {
                let mut s = f::setup(
                    42,
                    if query { "information" } else { "ongoing" },
                    &Default::default(),
                );
                s.recurring_open(p::Mode::Baseline.legacy(), e::Order::Ascending, 48, 4)
                    .unwrap();
                s.set_recurring_policy(p::Mode::Baseline.selector())
                    .unwrap();
                let mut points = vec![];
                if !query {
                    s.recurring_step(&mut points).unwrap();
                    s.recurring_step(&mut points).unwrap();
                }
                let before = s.recurring_point();
                let start = Instant::now();
                let step = s.recurring_step(&mut points).unwrap();
                elapsed += start.elapsed().as_nanos();
                let after = s.recurring_point();
                events += after.state.events - before.state.events;
                queries += after.state.queries - before.state.queries;
                updates += step.need_after - step.need_before;
            }
            assert_eq!((events, queries), if query { (0, 20) } else { (20, 0) });
            println!(
                "{},Baseline,{batch},20,{elapsed},{events},{queries},{updates}",
                if query {
                    "query_activity_with_handshake"
                } else {
                    "resource_tick_with_completion"
                }
            );
        }
    }
    let t = f::trial(42, p::Mode::Baseline, "ongoing", Default::default());
    let mut bytes = 0;
    for batch in 0..5 {
        let start = Instant::now();
        let encoded = serde_json::to_vec(&t).unwrap();
        let elapsed = start.elapsed().as_nanos();
        bytes = encoded.len();
        black_box(encoded);
        println!("serialize,Baseline,{batch},1,{elapsed},0,0,0");
    }
    let gap = f::trial(
        42,
        p::Mode::Baseline,
        "recurring",
        f::Config {
            idle_gap: 5,
            ..Default::default()
        },
    );
    let value = serde_json::json!({"seed":42,"case":"ongoing","mode":"Baseline","periods":t.periods.len(),"clock_value_bytes":std::mem::size_of::<r::Clock>(),"commitment_value_bytes":std::mem::size_of::<o::Commitment>(),"pause_value_bytes":std::mem::size_of::<f::Pause>(),"new_cognitive_cells":0,"new_live_simulation_components":0,"steps":t.periods.iter().map(|p|p.steps.len()).sum::<usize>(),"points":t.points.len(),"point_table_json_bytes":serde_json::to_vec(&t.points).unwrap().len(),"need_records":t.needs.len(),"need_records_json_bytes":serde_json::to_vec(&t.needs).unwrap().len(),"trial_json_bytes":bytes,"gap_trial_case":"recurring","gap_trial_pauses":gap.pauses.len(),"gap_records_json_bytes":serde_json::to_vec(&gap.pauses).unwrap().len()});
    fs::write(output, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
}
