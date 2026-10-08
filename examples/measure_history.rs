//! Foundation measurements; deliberately no policy or rule changes.
use serde::Serialize;
use std::{fs, hint::black_box, time::Instant};
use world_of_individuals::{
    behavior::scarcity_policy, cognition::EvidenceKind, experiment007, foresight::Sensor,
    model::Generator, simulation::Simulation,
};
fn bytes<T: Serialize>(v: &T) -> usize {
    serde_json::to_vec(v).unwrap().len()
}
fn main() {
    let label = std::env::args().nth(1).unwrap_or_else(|| "baseline".into());
    assert!(matches!(label.as_str(), "baseline" | "after"));
    fs::create_dir_all("reinforcement").unwrap();
    let mut timings = String::from(
        "workload,population,encounters,init_ms,run_ms,snapshot_ms,serialize_ms,bytes\n",
    );
    let mut metrics = vec![];
    for population in [100, 1000] {
        for sample in 0..5 {
            let start = Instant::now();
            let mut g = Generator::new(42);
            let mut sim = Simulation::new((0..population).map(|i| g.agent(i)).collect()).unwrap();
            sim.set_policy(scarcity_policy);
            sim.enable_foresight(42, Sensor::default()).unwrap();
            sim.enable_concerns().unwrap();
            sim.enable_inquiry().unwrap();
            sim.enable_provenance().unwrap();
            let init = start.elapsed();
            let start = Instant::now();
            for _ in 0..3 {
                for i in (0..population).step_by(2) {
                    sim.add_scene([i, i + 1], 1, 6).unwrap();
                }
                sim.run();
                for i in 0..population {
                    sim.consume(i).unwrap();
                }
            }
            let run = start.elapsed();
            let start = Instant::now();
            let snapshot = experiment007::snapshot(&sim, "foundation measurement");
            let construction = start.elapsed();
            let start = Instant::now();
            let data = serde_json::to_vec(&snapshot).unwrap();
            black_box(&data);
            let serialization = start.elapsed();
            timings.push_str(&format!(
                "population,{population},3,{:.6},{:.6},{:.6},{:.6},{}\n",
                init.as_secs_f64() * 1000.,
                run.as_secs_f64() * 1000.,
                construction.as_secs_f64() * 1000.,
                serialization.as_secs_f64() * 1000.,
                data.len()
            ));
            if sample == 0 {
                let q = &snapshot.base.inquiry;
                let p = &snapshot.provenance;
                metrics.push(serde_json::json!({"population":population,"legacy_bytes":data.len(),"provenance_bytes":bytes(p),
                    "query_bytes":bytes(&p.queries),"inquiry_bytes":bytes(q),"inquiry_record_bytes":bytes(&q.records),
                    "duplicated_base_inputs_bytes":p.queries.iter().map(|r|bytes(&r.decision.input.base)).sum::<usize>(),
                    "duplicated_decisions_bytes":p.queries.iter().map(|r|bytes(&r.decision.decision)).sum::<usize>(),
                    "duplicated_responses_bytes":p.queries.iter().map(|r|bytes(&r.legacy_response)).sum::<usize>(),
                    "events":snapshot.base.base.events.len(),"information":snapshot.base.base.cognition.information.len(),
                    "queries":p.queries.len(),"roots":p.roots.len(),"provenance_receipts":p.receipts.len(),
                    "live_knowledge":p.knowledge.values().map(|v|v.len()).sum::<usize>(),
                    "live_concerns":snapshot.base.base.concerns.items.values().map(|v|v.len()).sum::<usize>(),
                    "live_cells":q.cells.values().map(|v|v.len()).sum::<usize>(),"live_signatures":q.seen.values().map(|v|v.len()).sum::<usize>()}));
            }
        }
    }
    // Long-history receipt/revision workload, fixed pair; only legitimate new
    // refusals and disclosures. Snapshotting stays outside execution measurement.
    for count in [100, 1000, 4000] {
        let start = Instant::now();
        let mut g = Generator::new(42);
        let mut sim = Simulation::new(vec![g.agent(0), g.agent(1)]).unwrap();
        sim.set_policy(scarcity_policy);
        let init = start.elapsed();
        let start = Instant::now();
        for n in 0..count {
            sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
            sim.set_circumstances(1, 1, 90, 0, 90).unwrap();
            sim.add_scene([0, 1], 1, 6).unwrap();
            sim.run();
            sim.communicate(1, 0, 2 * n + 1, EvidenceKind::Disclosure)
                .unwrap();
        }
        let run = start.elapsed();
        let start = Instant::now();
        let data = (sim.events(), sim.cognition(), sim.agents());
        let construction = start.elapsed();
        let start = Instant::now();
        let serialized = serde_json::to_vec(&data).unwrap();
        let serialization = start.elapsed();
        timings.push_str(&format!(
            "revision,2,{count},{:.6},{:.6},{:.6},{:.6},{}\n",
            init.as_secs_f64() * 1000.,
            run.as_secs_f64() * 1000.,
            construction.as_secs_f64() * 1000.,
            serialization.as_secs_f64() * 1000.,
            serialized.len()
        ));
    }
    fs::write(format!("reinforcement/{label}-measurements.csv"), &timings).unwrap();
    fs::write(
        format!("reinforcement/{label}-structure.json"),
        serde_json::to_vec_pretty(&metrics).unwrap(),
    )
    .unwrap();
    print!("{timings}");
}
