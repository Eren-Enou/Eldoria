//! Controlled provenance opportunities; all observation/share/query choices use policies.
use crate::{
    behavior::scarcity_policy,
    experiment005, experiment006,
    foresight::{Channel, Sensor},
    inquiry,
    intentional::Profile,
    model::Generator,
    provenance as p,
    simulation::Simulation,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub base: experiment006::Snapshot,
    pub provenance: p::Provenance,
}
pub fn snapshot(sim: &Simulation, label: &str) -> Snapshot {
    Snapshot {
        base: experiment006::snapshot(sim, label),
        provenance: sim.provenance().unwrap(),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub name: String,
    pub interventions: Vec<String>,
    pub initial: Snapshot,
    pub checkpoints: Vec<Snapshot>,
    pub final_state: Snapshot,
}
fn sensor(inverted: bool, quality: i32) -> Sensor {
    Sensor {
        reliability: quality,
        effort: 2,
        channel: if inverted {
            Channel::InvertedFixture
        } else {
            Channel::AccurateFixture
        },
        second: None,
    }
}
pub fn setup(seed: u64) -> Simulation {
    let mut g = Generator::new(seed);
    let mut sim = Simulation::new((0..5).map(|id| g.agent(id)).collect()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(seed, sensor(false, 40)).unwrap();
    sim.enable_concerns().unwrap();
    experiment005::refusal_conditions(&mut sim, 60);
    sim.add_scene([0, 1], 1, 6).unwrap();
    sim.run();
    sim.enable_inquiry().unwrap();
    sim.enable_provenance().unwrap();
    present(&mut sim);
    sim
}
fn present(sim: &mut Simulation) {
    for id in 0..5 {
        sim.set_circumstances(id, 4, 30, 0, 80).unwrap();
        sim.set_communication_profile(
            id,
            Profile {
                privacy: 20,
                ..Default::default()
            },
        )
        .unwrap();
    }
}
pub fn event(sim: &Simulation) -> u64 {
    sim.events()
        .into_iter()
        .rev()
        .find(|e| e.outcome == Some(crate::model::Outcome::Refusal))
        .unwrap()
        .id
}
fn inspect(sim: &mut Simulation, event: u64, who: &[(u32, bool, i32)]) {
    sim.inspect_refusal(
        event,
        &who.iter()
            .map(|&(id, inverted, q)| (id, sensor(inverted, q)))
            .collect::<Vec<_>>(),
    )
    .unwrap();
}
fn share(sim: &mut Simulation, source: u32, listener: u32, event: u64, attributed: bool) {
    sim.provenance_exchange(source, listener, event, attributed)
        .unwrap();
}
fn point(sim: &Simulation, points: &mut Vec<Snapshot>, label: &str) {
    points.push(snapshot(sim, label));
}
pub fn trial(seed: u64, name: &str) -> Trial {
    let mut sim = setup(seed);
    let mut target = event(&sim);
    let initial = snapshot(
        &sim,
        "actual unresolved refusal; provenance opt-in; matched present state",
    );
    let mut points = Vec::new();
    let mut interventions=vec!["Actual policy-selected refusal with original requester food0/hunger90 and refuser food1/hunger90/privacy100. Then present agents food4/hunger30, privacy20. Recorded inspection offers open an instrumented historical scarcity record; no knowledge is inserted by the harness. Sensors quality50, effort2, accurate/inverted fixtures as labeled.".into()];
    if name == "credibility_independence" {
        inspect(
            &mut sim,
            target,
            &[(2, false, 50), (3, true, 50), (4, false, 80)],
        );
        share(&mut sim, 2, 0, target, true);
        share(&mut sim, 3, 0, target, true);
        share(&mut sim, 4, 0, target, true);
        point(
            &sim,
            &mut points,
            "fallible independent strong evidence compares earlier claims; directed credibility changes legitimately",
        );
        experiment005::refusal_conditions(&mut sim, 60);
        sim.add_scene([0, 1], 1, 6).unwrap();
        sim.run();
        present(&mut sim);
        target = event(&sim);
        inspect(&mut sim, target, &[(4, false, 50), (3, false, 50)]);
        share(&mut sim, 4, 2, target, true);
        share(&mut sim, 4, 0, target, true);
        share(&mut sim, 2, 0, target, true);
        sim.provenance_offer(3, 0, target, true).unwrap();
        sim.inquiry_meeting(&[0, 2, 3]).unwrap();
        interventions.push("Credibility is learned on the earlier event from quality80 fallible evidence: source2 correct, source3 inverted; no credibility values are assigned. New target: high-credibility2 relays4; lower-credibility3 independently inspects. Both are publicly present.".into());
    } else if name.starts_with("alternative_") {
        inspect(&mut sim, target, &[(2, false, 40)]);
        share(&mut sim, 2, 0, target, true);
        // Deplete both strategy cells through real zero-novelty replies under
        // provenance-disabled valuation, then restore local provenance valuation.
        sim.set_provenance_settings(
            0,
            p::Settings {
                provenance: false,
                speakers: true,
            },
        )
        .unwrap();
        for _ in 0..7 {
            sim.inquiry_meeting(&[0, 2]).unwrap();
        }
        sim.set_provenance_settings(0, p::Settings::default())
            .unwrap();
        if name == "alternative_independent" {
            inspect(&mut sim, target, &[(3, false, 40)]);
        }
        if name == "alternative_relay" {
            share(&mut sim, 2, 3, target, true);
        }
        point(
            &sim,
            &mut points,
            "A's usefulness depleted; B not previously contacted; hidden B acquisition differs",
        );
        sim.inquiry_meeting(&[0, 2, 3]).unwrap();
        interventions.push("Seven actual queries/opportunities with source2 before introducing3; provenance valuation temporarily disabled to measure depletion even with known overlap. Restored before source switch. No pre-query metadata from3, so its private acquisition cannot influence selection.".into());
    } else {
        let independent = matches!(
            name,
            "independent" | "independent_wrong" | "conflict" | "speaker_disabled_independent"
        );
        let attributed = !matches!(name, "hidden" | "revelation");
        if name == "provenance_disabled" {
            sim.set_provenance_settings(
                0,
                p::Settings {
                    provenance: false,
                    speakers: true,
                },
            )
            .unwrap();
        }
        if name == "speaker_disabled_independent" || name == "speaker_disabled_relay" {
            sim.set_provenance_settings(
                0,
                p::Settings {
                    provenance: true,
                    speakers: false,
                },
            )
            .unwrap();
        }
        if independent {
            inspect(
                &mut sim,
                target,
                &[
                    (2, name == "independent_wrong", 50),
                    (3, matches!(name, "conflict" | "independent_wrong"), 50),
                ],
            );
        } else {
            inspect(&mut sim, target, &[(2, false, 50)]);
            share(&mut sim, 2, 3, target, true);
        }
        share(&mut sim, 2, 0, target, true);
        point(&sim, &mut points, "A report received");
        share(&mut sim, 3, 0, target, attributed);
        point(
            &sim,
            &mut points,
            "B report received: independent or relay; attribution as configured",
        );
        if name == "three_speakers" {
            share(&mut sim, 3, 4, target, true);
            share(&mut sim, 4, 0, target, true);
            point(&sim, &mut points, "third communicator, same observation");
        }
        if name == "revelation" {
            share(&mut sim, 3, 0, target, true);
            point(
                &sim,
                &mut points,
                "B voluntarily reveals attribution of its earlier report",
            );
        }
    }
    Trial {
        seed,
        name: name.into(),
        interventions,
        initial,
        checkpoints: points,
        final_state: snapshot(&sim, "final audited state"),
    }
}
pub fn suite(seed: u64) -> Vec<Trial> {
    [
        "independent",
        "independent_wrong",
        "relay",
        "three_speakers",
        "hidden",
        "revelation",
        "conflict",
        "credibility_independence",
        "alternative_independent",
        "alternative_relay",
        "alternative_empty",
        "provenance_disabled",
        "speaker_disabled_independent",
        "speaker_disabled_relay",
    ]
    .into_iter()
    .map(|name| trial(seed, name))
    .collect()
}
pub fn population(seed: u64, count: u32) -> Snapshot {
    let mut g = Generator::new(seed);
    let mut sim = Simulation::new((0..count).map(|id| g.agent(id)).collect()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(seed, Sensor::default()).unwrap();
    sim.enable_concerns().unwrap();
    sim.enable_inquiry().unwrap();
    sim.enable_provenance().unwrap();
    for _ in 0..3 {
        for id in (0..count.saturating_sub(1)).step_by(2) {
            sim.add_scene([id, id + 1], 1, 6).unwrap();
        }
        sim.run();
        for id in 0..count {
            sim.consume(id).unwrap();
        }
    }
    snapshot(
        &sim,
        "three natural pair encounters; no inspection/relay opportunities scheduled",
    )
}
pub fn metrics(s: &Snapshot) -> serde_json::Value {
    let p = &s.provenance;
    let receipts: Vec<_> = p.receipts.iter().filter(|r| r.parent.is_some()).collect();
    let mut last = std::collections::BTreeMap::new();
    let mut switches = 0;
    for query in &p.queries {
        if let inquiry::Action::Ask {
            concern, source, ..
        } = query.decision.decision.selected
            && last
                .insert((query.decision.input.base.owner, concern), source)
                .is_some_and(|old| old != source)
        {
            switches += 1;
        }
    }
    serde_json::json!({"observations":p.roots.len(),"relay_transmissions":receipts.iter().filter(|r|r.depth>1).count(),"testimony_receipts":receipts.len(),"native_cognition_receipts":s.base.base.cognition.information.len(),
        "shared_detections":receipts.iter().filter(|r|r.evaluation.kind==p::ValueKind::Shared).count(),"corroborations":receipts.iter().filter(|r|r.evaluation.kind==p::ValueKind::Corroboration).count(),
        "hidden_relays":receipts.iter().filter(|r|!r.attributed && r.depth>1).count(),"independent_conflicts":receipts.iter().filter(|r|r.evaluation.kind==p::ValueKind::Conflict).count(),"inquiry_source_changes":switches,
        "queries":p.queries.len(),"knowledge_occupancy":p.knowledge.values().map(|v|v.len()).sum::<usize>(),"hint_occupancy":p.hints.values().map(|v|v.len()).sum::<usize>(),"comparison_occupancy":p.comparisons.values().map(|v|v.len()).sum::<usize>(),"maximum_knowledge":p.knowledge.values().map(|v|v.len()).max().unwrap_or(0),"maximum_hints":p.hints.values().map(|v|v.len()).max().unwrap_or(0),"maximum_comparisons":p.comparisons.values().map(|v|v.len()).max().unwrap_or(0),"evictions":p.evictions.len(),
        "unresolved_concerns":s.base.base.concerns.items.values().flatten().filter(|c|c.status.active()).count(),"audit_bytes":serde_json::to_vec(s).unwrap().len(),"provenance_bytes":serde_json::to_vec(p).unwrap().len()})
}
pub fn human(trials: &[Trial]) -> String {
    let mut out = String::new();
    for t in trials {
        out.push_str(&format!("\n{} seed{}\n", t.name, t.seed));
        for concern in t.initial.base.base.concerns.items.values().flatten() {
            out.push_str(&format!(
                "concern{} owner{} event{} importance{} uncertainty{} {:?}\n",
                concern.id,
                concern.owner,
                concern.event,
                concern.importance,
                concern.uncertainty,
                concern.status
            ));
        }
        for r in &t.final_state.provenance.receipts {
            out.push_str(&format!("receipt{} event{} {}->{} root{} parent{:?} known{:?} new_speaker={} independent={} {:?} value{} support{}->{} cognitive{:?} credibility{:?}\n",r.id,r.event,r.communicator,r.listener,r.actual_root,r.parent,r.knowledge.known,r.evaluation.new_speaker,r.evaluation.independent,r.evaluation.kind,r.evaluation.incremental_value,r.evaluation.before,r.evaluation.after,r.cognition_receipt,r.credibility_changes));
        }
        for q in &t.final_state.provenance.queries {
            if q.decision.input.base.owner != 0 {
                continue;
            }
            out.push_str(&format!(
                "query{} {:?} value{} cells{:?}->{:?}\n",
                q.id, q.decision.decision.selected, q.value, q.before, q.after
            ));
            for c in &q.decision.decision.candidates {
                out.push_str(&format!(
                    "  {:?} importance{} gain{} benefit{} cost{} score{}\n",
                    c.action, c.importance, c.expected_gain, c.benefit, c.cost, c.score
                ));
            }
            out.push_str(&format!(
                "  lineage/credibility factors {:?}\n",
                q.decision.factors
            ));
        }
    }
    out
}
