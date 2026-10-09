use std::{fs, time::Instant};
use world_of_individuals::{experiment011 as e, inquiry_value as v, model::Observation};
fn summary(t: &e::Trial, held: bool) -> serde_json::Value {
    let view = Observation {
        partner: t.resource_partner,
        last_signal: None,
        amount: t.config.amount,
        turns_left: 6,
    };
    let probe = |p: &e::Point| world_of_individuals::behavior::scarcity_policy(&p.agents[0], &view);
    let queries = e::queries(t);
    let steps: Vec<_>=queries.iter().zip(&t.opportunities).map(|(r,o)| {
        serde_json::json!({"inquiry":r.id,"selected":r.decision.selected,"concerns_before":o.before.concerns,
            "basis_before":o.before.basis,"public_sources":r.decision.input.sources,"cells_before":r.decision.input.cells,
            "candidates":r.decision.candidates,"status_components":r.decision.candidates.iter().map(|c|v::components(c,&o.before.basis)).collect::<Vec<_>>(),
            "response":r.response,"realized_value":r.realized_value,"novelty":r.novelty,
            "cell_after":r.cell_after,"time":r.time_spent,"concerns_after":o.after.concerns,"basis_after":o.after.basis,
            "assessment_before":o.before.assessment_end,"assessment_after":o.after.assessment_end,"remaining":o.remaining,
            "readiness_before":probe(&o.before),"readiness_after":probe(&o.after)})
    }).collect();
    let compact = &t.final_state.base.base.base;
    let start = t.opportunities.last().unwrap().after.event_end;
    let events = &compact.base.base.events[start..];
    serde_json::json!({"seed":t.seed,"held":held,"case":t.case,"config":t.config,"mode":t.mode,
        "targets":t.targets,"resource_partner":t.resource_partner,"steps":steps,
        "actions":events.iter().map(|e|e.decision.selected).collect::<Vec<_>>(),
        "event_ids":events.iter().map(|e|e.id).collect::<Vec<_>>(),
        "transferred":events.iter().map(|e|e.transferred).sum::<u32>(),
        "pre_readiness":probe(&t.opportunities[0].before),"post_readiness":probe(&t.opportunities.last().unwrap().after),
        "food_after_action":t.after_action.agents.iter().map(|a|a.food).collect::<Vec<_>>(),
        "persistent_food":t.persistent.agents.iter().map(|a|a.food).collect::<Vec<_>>(),"consumption":t.consumption,
        "unresolved_before_resource":t.opportunities.last().unwrap().after.concerns.iter().filter(|c|c.status.active()).count(),
        "unresolved_after_resource":t.persistent.concerns.iter().filter(|c|c.status.active()).count(),
        "trial_bytes":serde_json::to_vec(t).unwrap().len(),"audit_bytes":serde_json::to_vec(&t.final_state.value).unwrap().len(),
        "audit_records":t.final_state.value.records.len()})
}
fn main() {
    let preview = std::env::args().any(|a| a == "--preview");
    fs::create_dir_all("target/experiment011").unwrap();
    let mut rows = vec![];
    let mut timings = String::from("seed,held,run_replay_validate_ms,bytes,serialize_ms\n");
    let seeds: Vec<_> = if preview {
        vec![42]
    } else {
        (0..16).chain([42, u64::MAX]).collect()
    };
    for seed in seeds {
        for held in if preview {
            vec![false]
        } else {
            vec![false, true]
        } {
            let start = Instant::now();
            let trials = e::suite(seed, held);
            assert_eq!(trials, e::suite(seed, held));
            let elapsed = start.elapsed().as_secs_f64() * 1000.;
            for group in trials.as_chunks::<3>().0 {
                for t in group {
                    assert_eq!(t.before_prelude, group[0].before_prelude);
                    assert_eq!(t.opportunities[0].before, group[0].opportunities[0].before);
                    assert_eq!(
                        e::queries(t)[0].decision.input,
                        e::queries(&group[0])[0].decision.input
                    );
                }
            }
            for config in if held {
                e::held_out()
            } else {
                vec![e::Config::default()]
            } {
                for mode in v::MODES {
                    let a = trials
                        .iter()
                        .find(|t| {
                            t.case == "uncertain_minor" && t.config == config && t.mode == mode
                        })
                        .unwrap();
                    let b = trials
                        .iter()
                        .find(|t| {
                            t.case == "mirrored_future" && t.config == config && t.mode == mode
                        })
                        .unwrap();
                    assert_eq!(e::queries(a)[0].decision, e::queries(b)[0].decision);
                    assert_eq!(a.opportunities, b.opportunities);
                }
            }
            let start = Instant::now();
            let bytes = serde_json::to_vec(&trials).unwrap();
            timings.push_str(&format!(
                "{seed},{held},{elapsed:.3},{},{:.3}\n",
                bytes.len(),
                start.elapsed().as_secs_f64() * 1000.
            ));
            if seed == 42 {
                fs::write(
                    format!(
                        "target/experiment011/{}.json",
                        if held { "held-out-seed-42" } else { "seed-42" }
                    ),
                    bytes,
                )
                .unwrap();
            }
            for t in &trials {
                let row = summary(t, held);
                if preview {
                    println!(
                        "{} {:?}: {:?} -> {:?}",
                        t.case,
                        t.mode,
                        e::queries(t)
                            .iter()
                            .map(|r| r.decision.selected)
                            .collect::<Vec<_>>(),
                        row["actions"]
                    );
                }
                rows.push(row);
            }
            eprintln!(
                "011 seed {seed} held {held}: exact replay, matched inputs and mirrored futures passed"
            );
        }
    }
    fs::write(
        "target/experiment011/summary.json",
        serde_json::to_vec(&rows).unwrap(),
    )
    .unwrap();
    fs::write("target/experiment011/benchmark.csv", timings).unwrap();
    if !preview {
        let mut timing = String::from(
            "mode,sample,run_validate_ms,serialize_ms,bytes,audit_bytes,audit_records\n",
        );
        for sample in 0..7 {
            for mode in v::MODES {
                let start = Instant::now();
                let t = e::trial(42, mode, "uncertain_minor", e::Config::default());
                let run = start.elapsed().as_secs_f64() * 1000.;
                let start = Instant::now();
                let bytes = serde_json::to_vec(&t).unwrap();
                let serialize = start.elapsed().as_secs_f64() * 1000.;
                timing.push_str(&format!(
                    "{mode:?},{sample},{run:.3},{serialize:.3},{},{},{}\n",
                    bytes.len(),
                    serde_json::to_vec(&t.final_state.value).unwrap().len(),
                    t.final_state.value.records.len()
                ));
            }
        }
        fs::write("target/experiment011/performance.csv", timing).unwrap();
        // Identical bounded inputs for every mode; includes identical decision-clone cost.
        let t = e::trial(
            42,
            v::Mode::Unchanged,
            "uncertain_minor",
            e::Config::default(),
        );
        let query = t
            .final_state
            .base
            .base
            .base
            .query(world_of_individuals::audit::QueryId(0))
            .unwrap();
        let base = world_of_individuals::provenance::policy(&query.decision.input);
        let basis = &t.opportunities[0].before.basis;
        let mut scoring = String::from("mode,sample,iterations,candidates,basis,clone_score_ns\n");
        for sample in 0..7 {
            for mode in v::MODES {
                let start = Instant::now();
                for _ in 0..10000 {
                    std::hint::black_box(v::compare(
                        mode,
                        std::hint::black_box(base.clone()),
                        std::hint::black_box(basis),
                    ));
                }
                scoring.push_str(&format!(
                    "{mode:?},{sample},10000,{},{},{:.3}\n",
                    base.decision.candidates.len(),
                    basis.len(),
                    start.elapsed().as_nanos() as f64 / 10000.
                ));
            }
        }
        fs::write("target/experiment011/scoring.csv", scoring).unwrap();
    }
}
