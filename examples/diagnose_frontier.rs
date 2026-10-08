//! Research probes only: existing public behavior and the exact history index source.
//! No new policy, evidence rule or runtime path is installed.
use std::{collections::BTreeMap, fs, hint::black_box, time::Instant};
use world_of_individuals::{
    behavior::scarcity_policy,
    experiment007 as e,
    foresight::{Channel, Sensor},
    intentional::Profile,
    simulation::Simulation,
};
// Compile the unchanged index implementation for synthetic stress measurements.
pub use world_of_individuals::{cognition, model};
#[allow(dead_code)]
#[path = "../src/history.rs"]
mod measured_history;

fn main() {
    fs::create_dir_all("research/next-target").unwrap();
    let mut mixed = vec![];
    for provenance_first in [false, true] {
        let mut sim = e::setup(42);
        let event = e::event(&sim);
        sim.inspect_refusal(
            event,
            &[(
                2,
                Sensor {
                    reliability: 40,
                    effort: 2,
                    channel: Channel::InvertedFixture,
                    second: None,
                },
            )],
        )
        .unwrap();
        let mut steps = vec![];
        for provenance in if provenance_first {
            [true, false, true]
        } else {
            [false, true, false]
        } {
            if provenance {
                sim.provenance_exchange(2, 0, event, true).unwrap();
            } else {
                sim.communicate(
                    1,
                    0,
                    event,
                    cognition::EvidenceKind::Fallible {
                        scarce: true,
                        reliability: 90,
                        source: 0,
                    },
                )
                .unwrap();
            }
            let state = sim.cognition();
            let receipt = state.information.last().unwrap();
            // Matched-present local policy probe; no resource encounter is executed.
            let mut own = sim.agents()[0].clone();
            own.food = 4;
            own.hunger = 30;
            own.generosity = 60;
            own.caution = 80;
            let probe = scarcity_policy(
                &own,
                &model::Observation {
                    partner: 1,
                    last_signal: None,
                    amount: 1,
                    turns_left: 6,
                },
            );
            steps.push(
                serde_json::json!({"channel":if provenance {"provenance"} else {"native"},
                "support":receipt.after.support,"revision":receipt.revision,
                "trust":receipt.trust_after,"concerns":sim.concerns().unwrap().items,
                "receipt":receipt.id,"matched_present_policy_probe":probe}),
            );
        }
        mixed.push(
            serde_json::json!({"provenance_first":provenance_first,"event":event,"steps":steps}),
        );
        sim.validate_history_indexes().unwrap();
        sim.compact_snapshot("mixed diagnostic")
            .unwrap()
            .validate()
            .unwrap();
    }
    fs::write(
        "research/next-target/mixed-evidence.json",
        serde_json::to_vec_pretty(&mixed).unwrap(),
    )
    .unwrap();
    let mut csv = String::from("probe,size,repetitions,elapsed_ms,records_or_result\n");
    for partners in [10, 100, 1000, 10000] {
        let mut g = model::Generator::new(42);
        let mut agent = g.agent(0);
        agent.trust = (1..=partners).map(|id| (id, 0)).collect();
        let start = Instant::now();
        for _ in 0..1000 {
            black_box(agent.clone());
        }
        csv.push_str(&format!(
            "agent_clone_partners,{partners},1000,{:.6},{}\n",
            start.elapsed().as_secs_f64() * 1000.,
            serde_json::to_vec(&agent).unwrap().len()
        ));
    }
    for population in [100, 1000] {
        let mut run = std::time::Duration::ZERO;
        let mut append = std::time::Duration::ZERO;
        let mut count = 0;
        for seed in 0..100 {
            let mut g = model::Generator::new(seed);
            let agents: Vec<_> = (0..population).map(|id| g.agent(id)).collect();
            let initial = agents.iter().map(|a| (a.id, a.trust.clone())).collect();
            let mut sim = Simulation::new(agents).unwrap();
            for id in (0..population).step_by(2) {
                sim.add_scene([id, id + 1], 1, 6).unwrap();
            }
            let start = Instant::now();
            sim.run();
            run += start.elapsed();
            let events = sim.events();
            count += events.len();
            let mut index = measured_history::HistoryIndex::default();
            let start = Instant::now();
            for event in &events {
                index.event(event, &initial);
            }
            append += start.elapsed();
            black_box(index);
        }
        csv.push_str(&format!(
            "001_run,{population},100,{:.6},{count}\n",
            run.as_secs_f64() * 1000.
        ));
        csv.push_str(&format!(
            "001_index_replay,{population},100,{:.6},{count}\n",
            append.as_secs_f64() * 1000.
        ));
    }
    let mut g = model::Generator::new(42);
    let mut template_sim = Simulation::new(vec![g.agent(0), g.agent(1)]).unwrap();
    template_sim.add_scene([0, 1], 1, 6).unwrap();
    template_sim.run();
    let mut template = template_sim.events()[0].clone();
    let initial = BTreeMap::from([(0, BTreeMap::new()), (1, BTreeMap::new())]);
    for count in [1000usize, 10000, 100000] {
        let mut index = measured_history::HistoryIndex::default();
        // Zero contributions prevent saturation/convergence: stress the true suffix bound.
        for m in &mut template.interpretations {
            m.valence = 0;
        }
        let start = Instant::now();
        for id in 0..count {
            template.id = id as u64;
            index.event(&template, &initial);
        }
        csv.push_str(&format!(
            "index_append,{count},1,{:.6},{}\n",
            start.elapsed().as_secs_f64() * 1000.,
            count * 2
        ));
        let owner = template.participants[0];
        let partner = template.interpretations[0].partner;
        for (label, id) in [("suffix_first", 0), ("suffix_last", count as u64 - 1)] {
            let start = Instant::now();
            for i in 0..200 {
                black_box(index.revise(owner, partner, id, i % 2));
            }
            csv.push_str(&format!(
                "{label},{count},200,{:.6},{}\n",
                start.elapsed().as_secs_f64() * 1000.,
                index.revise(owner, partner, id, 0)
            ));
        }
    }
    // Actual repeated conversations; no concern/inquiry mode, so no extra follow-up paths.
    for count in [100usize, 1000, 4000] {
        let mut g = model::Generator::new(42);
        let mut sim = Simulation::new(vec![g.agent(0), g.agent(1)]).unwrap();
        sim.set_policy(scarcity_policy);
        sim.enable_foresight(42, Sensor::default()).unwrap();
        sim.set_communication_profile(0, Profile::default())
            .unwrap();
        sim.set_communication_profile(
            1,
            Profile {
                privacy: 100,
                ..Default::default()
            },
        )
        .unwrap();
        let start = Instant::now();
        for _ in 0..count {
            sim.set_circumstances(0, 0, 90, 20, 20).unwrap();
            sim.set_circumstances(1, 1, 90, 0, 90).unwrap();
            sim.add_scene([0, 1], 1, 6).unwrap();
            sim.run();
        }
        let elapsed = start.elapsed();
        let f = sim.foresight().unwrap();
        csv.push_str(&format!(
            "conversations,{count},1,{:.6},{}\n",
            elapsed.as_secs_f64() * 1000.,
            f.forecasts.len()
        ));
        let last = f.forecasts.last().unwrap().talk_record;
        let start = Instant::now();
        for _ in 0..10000 {
            black_box(
                f.forecasts
                    .iter()
                    .find(|x| x.talk_record == black_box(last))
                    .unwrap(),
            );
        }
        csv.push_str(&format!(
            "forecast_last_lookup,{},10000,{:.6},{}\n",
            f.forecasts.len(),
            start.elapsed().as_secs_f64() * 1000.,
            last
        ));
    }
    let mut storage = vec![];
    for rounds in [3, 30, 100] {
        let mut g = model::Generator::new(42);
        let mut sim = Simulation::new((0..100).map(|id| g.agent(id)).collect()).unwrap();
        sim.set_policy(scarcity_policy);
        sim.enable_foresight(42, Sensor::default()).unwrap();
        sim.enable_concerns().unwrap();
        sim.enable_inquiry().unwrap();
        sim.enable_provenance().unwrap();
        for _ in 0..rounds {
            for id in (0..100).step_by(2) {
                sim.add_scene([id, id + 1], 1, 6).unwrap();
            }
            sim.run();
            for id in 0..100 {
                sim.consume(id).unwrap();
            }
        }
        let start = Instant::now();
        let snapshot = sim.compact_snapshot("depth diagnostic").unwrap();
        let clone_time = start.elapsed();
        let start = Instant::now();
        let bytes = serde_json::to_vec(&snapshot).unwrap();
        let serialize_time = start.elapsed();
        let input_bytes: usize = snapshot
            .base
            .inquiry
            .records
            .iter()
            .map(|r| serde_json::to_vec(&r.decision.input).unwrap().len())
            .sum();
        storage.push(
            serde_json::json!({"agents":100,"encounters":rounds,"compact_bytes":bytes.len(),
            "native_input_bytes":input_bytes,"inquiries":snapshot.base.inquiry.records.len(),
            "events":snapshot.base.base.events.len()}),
        );
        csv.push_str(&format!(
            "compact_clone_depth,{rounds},1,{:.6},{}\n",
            clone_time.as_secs_f64() * 1000.,
            bytes.len()
        ));
        csv.push_str(&format!(
            "compact_serialize_depth,{rounds},1,{:.6},{}\n",
            serialize_time.as_secs_f64() * 1000.,
            bytes.len()
        ));
        snapshot.validate().unwrap();
    }
    let paths = [
        "/base/base/events",
        "/base/base/cognition/information",
        "/base/base/intentional/records",
        "/base/base/foresight/forecasts",
        "/base/inquiry/records",
        "/provenance/roots",
        "/provenance/receipts",
        "/provenance/queries",
    ];
    let mut checkpoints = vec![];
    for trial in e::suite(42) {
        let snapshots: Vec<_> = std::iter::once(&trial.initial)
            .chain(&trial.checkpoints)
            .chain(std::iter::once(&trial.final_state))
            .map(|s| serde_json::to_value(s).unwrap())
            .collect();
        let mut sums = BTreeMap::new();
        for path in paths {
            let final_array = snapshots
                .last()
                .unwrap()
                .pointer(path)
                .unwrap()
                .as_array()
                .unwrap();
            for snapshot in &snapshots {
                let prefix = snapshot.pointer(path).unwrap().as_array().unwrap();
                assert_eq!(prefix, &final_array[..prefix.len()]);
            }
            let total: usize = snapshots
                .iter()
                .map(|s| serde_json::to_vec(s.pointer(path).unwrap()).unwrap().len())
                .sum();
            let final_bytes = serde_json::to_vec(snapshots.last().unwrap().pointer(path).unwrap())
                .unwrap()
                .len();
            sums.insert(path,serde_json::json!({"all_checkpoints_bytes":total,"final_bytes":final_bytes,"repeated_prefix_bytes":total-final_bytes}));
        }
        checkpoints.push(
            serde_json::json!({"trial":trial.name,"snapshots":snapshots.len(),"archives":sums}),
        );
    }
    fs::write(
        "research/next-target/storage.json",
        serde_json::to_vec_pretty(
            &serde_json::json!({"depth":storage,"checkpoint_prefixes":checkpoints}),
        )
        .unwrap(),
    )
    .unwrap();
    fs::write("research/next-target/diagnostic-timings.csv", &csv).unwrap();
    print!("{csv}");
}
