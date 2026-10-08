//! Controlled assimilation and inquiry probes. No policy choices are scheduled.
use crate::{
    assessment::{Assessment, Method, Origin, Receipt},
    audit::CompactSnapshot,
    behavior::scarcity_policy,
    cognition::EvidenceKind,
    experiment007 as e,
    foresight::{Channel, Sensor},
    model::{Action, Decision, Generator, Observation},
    simulation::Simulation,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Legacy,
    Max,
    Grouped,
}
impl Mode {
    pub fn enable(self, sim: &mut Simulation) {
        match self {
            Self::Legacy => {}
            Self::Max => sim.enable_assessment(Method::Max).unwrap(),
            Self::Grouped => sim.enable_assessment(Method::Grouped).unwrap(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub base: CompactSnapshot,
    pub assessment: Option<Assessment>,
}
impl Snapshot {
    pub fn validate(&self) -> Result<(), String> {
        self.base.validate()?;
        if let Some(a) = &self.assessment {
            a.validate()?;
            let cognition = &self.base.base.base.cognition;
            let mut last_tick = 0;
            for r in &a.records {
                let i = &r.incoming;
                let info = match i.receipt {
                    Receipt::Native(id) => {
                        let info = cognition
                            .information
                            .get(id as usize)
                            .ok_or("missing native assessment receipt")?;
                        let origin = match info.kind {
                            EvidenceKind::Testimony { .. } => Origin::Claim(info.speaker),
                            EvidenceKind::Disclosure => Origin::Disclosure(info.speaker),
                            EvidenceKind::Fallible { source, .. } => Origin::NativeReading {
                                speaker: info.speaker,
                                channel: source,
                            },
                        };
                        if i.origin != origin
                            || i.communicator != info.speaker
                            || i.message.is_some()
                            || i.quality != info.reliability
                            || i.claim != info.scarce
                        {
                            return Err("native assessment input mismatch".into());
                        }
                        info
                    }
                    Receipt::Provenance(id) => {
                        let receipt = self
                            .base
                            .provenance
                            .receipts
                            .get(id as usize)
                            .ok_or("missing provenance assessment receipt")?;
                        let k = &receipt.knowledge;
                        if receipt.listener != r.owner
                            || i.origin
                                != k.known
                                    .map_or(Origin::Unknown(k.communicator), Origin::Known)
                            || i.message != Some(k.message)
                            || i.communicator != k.communicator
                            || i.claim != k.claim
                            || i.quality != k.quality
                        {
                            return Err(
                                "private or inconsistent provenance assessment input".into()
                            );
                        }
                        cognition
                            .information
                            .get(receipt.cognition_receipt.ok_or("missing bridge")? as usize)
                            .ok_or("missing bridge receipt")?
                    }
                };
                if info.listener != r.owner
                    || info.event != i.event
                    || info.after.support != r.support
                    || info.before.as_ref().map(|b| b.support) != r.prior_belief
                    || info.tick <= last_tick
                {
                    return Err("assessment consequence/ownership/time mismatch".into());
                }
                last_tick = info.tick;
            }
        }
        Ok(())
    }
}
pub fn snapshot(sim: &Simulation) -> Snapshot {
    Snapshot {
        base: sim.compact_snapshot("experiment008").unwrap(),
        assessment: sim.assessment(),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub label: String,
    pub support: Option<i32>,
    pub trust: i32,
    pub concern: Option<crate::concerns::Status>,
    pub probe: Decision,
    pub information: usize,
    pub assessment_records: usize,
}
pub fn point(sim: &Simulation, event: u64, label: &str) -> Point {
    let mut own = sim.agents()[0].clone();
    own.food = 4;
    own.hunger = 30;
    own.generosity = 60;
    own.caution = 80;
    Point {
        label: label.into(),
        support: sim
            .cognition()
            .beliefs
            .get(&0)
            .and_then(|v| v.iter().find(|b| b.event == event))
            .map(|b| b.support),
        trust: *own.trust.get(&1).unwrap_or(&0),
        concern: sim
            .concerns()
            .unwrap()
            .items
            .get(&0)
            .and_then(|v| v.iter().find(|c| c.event == event))
            .map(|c| c.status),
        probe: scarcity_policy(
            &own,
            &Observation {
                partner: 1,
                last_signal: None,
                amount: 1,
                turns_left: 6,
            },
        ),
        information: sim.cognition().information.len(),
        assessment_records: sim.assessment().map_or(0, |a| a.records.len()),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trial {
    pub seed: u64,
    pub mode: Mode,
    pub scenario: String,
    pub event: u64,
    pub points: Vec<Point>,
    pub final_state: Snapshot,
}
fn inspect(sim: &mut Simulation, event: u64, source: u32, positive: bool, quality: i32) {
    sim.inspect_refusal(
        event,
        &[(
            source,
            Sensor {
                reliability: quality,
                effort: 2,
                channel: if positive {
                    Channel::AccurateFixture
                } else {
                    Channel::InvertedFixture
                },
                second: None,
            },
        )],
    )
    .unwrap();
}
fn native(sim: &mut Simulation, event: u64, positive: bool, quality: i32) {
    sim.communicate(
        1,
        0,
        event,
        EvidenceKind::Fallible {
            scarce: positive,
            reliability: quality,
            source: 0,
        },
    )
    .unwrap();
}
fn share(sim: &mut Simulation, event: u64, source: u32, known: bool) {
    sim.provenance_exchange(source, 0, event, known).unwrap();
}
pub const SCENARIOS: &[&str] = &[
    "mixed_np",
    "mixed_pn",
    "corroboration_np",
    "corroboration_pn",
    "conflict_np",
    "conflict_pn",
    "known_relay",
    "hidden_relay",
    "revelation",
    "wrong",
    "claim_then_evidence",
    "evidence_then_claim",
    "eviction",
    "evicted_memory",
    "inquiry",
];
pub fn trial(seed: u64, mode: Mode, scenario: &str) -> Trial {
    let mut sim = e::setup(seed);
    mode.enable(&mut sim);
    let event = e::event(&sim);
    let mut points = vec![point(&sim, event, "initial")];
    match scenario {
        "mixed_np" | "mixed_pn" | "corroboration_np" | "corroboration_pn" | "conflict_np"
        | "conflict_pn" | "wrong" | "inquiry" => {
            let corroboration = scenario.starts_with("corroboration");
            let mixed = scenario.starts_with("mixed") || scenario == "inquiry";
            inspect(
                &mut sim,
                event,
                2,
                corroboration,
                if mixed { 40 } else { 50 },
            );
            let order = if scenario.ends_with("pn") {
                [true, false, true]
            } else {
                [false, true, false]
            };
            for provenance in order {
                if provenance {
                    share(&mut sim, event, 2, true);
                } else {
                    native(
                        &mut sim,
                        event,
                        scenario != "wrong",
                        if mixed { 90 } else { 50 },
                    );
                }
                points.push(point(
                    &sim,
                    event,
                    if provenance { "provenance" } else { "native" },
                ));
            }
            if scenario == "inquiry" {
                inspect(&mut sim, event, 3, true, 50);
                sim.provenance_offer(3, 0, event, true).unwrap();
                sim.inquiry_meeting(&[0, 3]).unwrap();
                points.push(point(&sim, event, "voluntary follow-up opportunity"));
            }
        }
        "known_relay" | "hidden_relay" | "revelation" => {
            inspect(&mut sim, event, 2, true, 50);
            sim.provenance_exchange(2, 3, event, true).unwrap();
            share(&mut sim, event, 2, true);
            points.push(point(&sim, event, "first"));
            share(&mut sim, event, 3, scenario == "known_relay");
            points.push(point(&sim, event, "relay"));
            if scenario == "revelation" {
                share(&mut sim, event, 3, true);
                points.push(point(&sim, event, "later attribution"));
            }
        }
        "claim_then_evidence" | "evidence_then_claim" => {
            inspect(&mut sim, event, 2, false, 40);
            for evidence in if scenario == "claim_then_evidence" {
                [false, true, false]
            } else {
                [true, false, true]
            } {
                if evidence {
                    share(&mut sim, event, 2, true);
                } else {
                    sim.communicate(1, 0, event, EvidenceKind::Testimony { scarce: true })
                        .unwrap();
                }
                points.push(point(
                    &sim,
                    event,
                    if evidence { "evidence" } else { "claim" },
                ));
            }
        }
        "eviction" => {
            inspect(&mut sim, event, 2, false, 40);
            native(&mut sim, event, true, 90);
            points.push(point(&sim, event, "native"));
            share(&mut sim, event, 2, true);
            points.push(point(&sim, event, "mixed"));
            for _ in 0..32 {
                share(&mut sim, event, 2, true);
            }
            points.push(point(&sim, event, "native acquisition evicted"));
            native(&mut sim, event, true, 90);
            points.push(point(&sim, event, "native explicitly received again"));
        }
        "evicted_memory" => {
            for _ in 0..20 {
                sim.set_circumstances(0, 4, 0, 0, 100).unwrap();
                sim.set_circumstances(1, 4, 0, 0, 100).unwrap();
                sim.add_scene([0, 1], 1, 6).unwrap();
                sim.run();
            }
            points.push(point(&sim, event, "episode no longer retained"));
            native(&mut sim, event, true, 90);
            points.push(point(&sim, event, "later evidence"));
        }
        _ => panic!("unknown scenario"),
    }
    sim.validate_history_indexes().unwrap();
    let final_state = snapshot(&sim);
    final_state.validate().unwrap();
    Trial {
        seed,
        mode,
        scenario: scenario.into(),
        event,
        points,
        final_state,
    }
}
pub fn suite(seed: u64) -> Vec<Trial> {
    [Mode::Legacy, Mode::Max, Mode::Grouped]
        .into_iter()
        .flat_map(|mode| {
            SCENARIOS
                .iter()
                .map(move |scenario| trial(seed, mode, scenario))
        })
        .collect()
}
pub fn population(seed: u64, count: u32, mode: Mode) -> Snapshot {
    let mut g = Generator::new(seed);
    let mut sim = Simulation::new((0..count).map(|id| g.agent(id)).collect()).unwrap();
    sim.set_policy(scarcity_policy);
    sim.enable_foresight(seed, Sensor::default()).unwrap();
    sim.enable_concerns().unwrap();
    sim.enable_inquiry().unwrap();
    sim.enable_provenance().unwrap();
    mode.enable(&mut sim);
    for _ in 0..3 {
        for id in (0..count.saturating_sub(1)).step_by(2) {
            sim.add_scene([id, id + 1], 1, 6).unwrap();
        }
        sim.run();
        for id in 0..count {
            sim.consume(id).unwrap();
        }
    }
    sim.validate_history_indexes().unwrap();
    let out = snapshot(&sim);
    out.validate().unwrap();
    out
}
pub fn human(trials: &[Trial]) -> String {
    let mut out = String::from(
        "Experiment 008: actual local receipt consequences; probe actions are not executed transfers.\n",
    );
    for t in trials {
        out.push_str(&format!(
            "\n{:?} {} seed={} event={}\n",
            t.mode, t.scenario, t.seed, t.event
        ));
        for p in &t.points {
            out.push_str(&format!(
                "{}: support={:?} trust={} concern={:?} probe={:?} information={} assessment={}\n",
                p.label,
                p.support,
                p.trust,
                p.concern,
                p.probe.selected,
                p.information,
                p.assessment_records
            ));
        }
    }
    out
}
pub fn choices(t: &Trial) -> Vec<Action> {
    t.points.iter().skip(1).map(|p| p.probe.selected).collect()
}
