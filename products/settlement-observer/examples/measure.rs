use eldoria_observer::{
    host::{Command, Host},
    projection::Perspective,
};
use eldoria_settlement::Config;
use std::time::Instant;
fn main() {
    let mut points = vec![];
    for population in [8, 12] {
        let mut host = Host::new(Config {
            population,
            seed: 42,
            horizon: 512,
        })
        .unwrap();
        for tick in [64, 512] {
            while host.world.tick() < tick {
                host.apply(Command::Step).unwrap();
            }
            let actor = *host.world.characters().keys().next().unwrap();
            for perspective in [Perspective::Observer, Perspective::Character(actor)] {
                host.apply(Command::View {
                    perspective,
                    page: 0,
                    selected: None,
                })
                .unwrap();
                let start = Instant::now();
                let frame = host.frame().unwrap();
                let build_us = start.elapsed().as_micros();
                let start = Instant::now();
                let bytes = serde_json::to_vec(&frame).unwrap();
                let encode_us = start.elapsed().as_micros();
                assert!(frame.rows.len() <= 30);
                points.push(serde_json::json!({"population":population,"tick":tick,"perspective":perspective,"build_us":build_us,
                    "encode_us":encode_us,"frame_bytes":bytes.len(),"history_records":host.world.state().history.len(),"page_records":frame.rows.len()}));
            }
        }
    }
    println!(
        "{}",
        serde_json::json!({"engine_runtime":eldoria_settlement::model::RUNTIME,"scope":"One release-mode sample per point; projection build and serialization only, no GPU/render timing; background research gate active","samples":points})
    );
}
