use world_of_individuals::{
    audit::*,
    behavior::scarcity_policy,
    cognition::EvidenceKind,
    experiment007 as e,
    foresight::{Channel, Sensor},
    model::Generator,
    provenance as p,
    simulation::Simulation,
};
fn sensor() -> Sensor {
    Sensor {
        reliability: 50,
        channel: Channel::AccurateFixture,
        ..Default::default()
    }
}
fn prepared() -> Simulation {
    let mut s = e::setup(42);
    let event = e::event(&s);
    s.inspect_refusal(event, &[(2, sensor()), (3, sensor())])
        .unwrap();
    s.provenance_exchange(2, 0, event, true).unwrap();
    s.inquiry_meeting(&[0, 2, 3]).unwrap();
    s
}
#[test]
fn compact_roundtrip_all_controlled_traces_and_exact_archive() {
    let trials = e::suite(42);
    let archived: serde_json::Value =
        serde_json::from_str(include_str!("../experiments/007/seed-42.json")).unwrap();
    assert_eq!(serde_json::to_value(&trials).unwrap(), archived);
    for trial in &trials {
        for snapshot in std::iter::once(&trial.initial)
            .chain(&trial.checkpoints)
            .chain(std::iter::once(&trial.final_state))
        {
            let compact = CompactSnapshot::from_legacy(snapshot).unwrap();
            let data = serde_json::to_vec(&compact).unwrap();
            let loaded: CompactSnapshot = serde_json::from_slice(&data).unwrap();
            assert_eq!(loaded.expand().unwrap(), *snapshot);
            assert_eq!(data, serde_json::to_vec(&loaded).unwrap());
        }
    }
}
#[test]
fn runtime_references_match_migration_and_do_not_mutate_past_contexts() {
    let mut sim = prepared();
    sim.validate_history_indexes().unwrap();
    let before = sim.compact_snapshot("test").unwrap();
    assert_eq!(
        before,
        CompactSnapshot::from_legacy(&e::snapshot(&sim, "test")).unwrap()
    );
    let retained = before.contexts.values().to_vec();
    let event = e::event(&sim);
    sim.provenance_exchange(3, 0, event, false).unwrap();
    sim.inquiry_meeting(&[0, 2, 3]).unwrap();
    let later = sim.compact_snapshot("test").unwrap();
    later.validate().unwrap();
    assert_eq!(&later.contexts.values()[..retained.len()], retained);
    assert_eq!(
        before.expand().unwrap().provenance.queries.len(),
        before.provenance.queries.len()
    );
    sim.validate_history_indexes().unwrap();
}
#[test]
fn corrupted_references_fail_explicitly() {
    let original = prepared().compact_snapshot("test").unwrap();
    original.validate().unwrap();
    let mut bad = original.clone();
    bad.provenance.queries[0].context = ContextId(u64::MAX);
    assert!(bad.expand().is_err());
    let mut bad = original.clone();
    bad.provenance.queries[0].inquiry = InquiryId(u64::MAX);
    assert!(bad.expand().is_err());
    let mut bad = original.clone();
    bad.provenance.queries[0].owner = 99;
    assert!(bad.expand().is_err());
    let mut bad = original.clone();
    bad.provenance.queries[0].observed_at = u64::MAX;
    assert!(bad.expand().is_err());
    let mut bad = original.clone();
    bad.provenance.receipts[2].parent = Some(2);
    assert!(bad.expand().is_err());
    let mut bad = original.clone();
    bad.provenance.receipts[2].communicator = 3;
    assert!(bad.expand().is_err());
    let mut bad = original.clone();
    bad.provenance.roots[0].tick = u64::MAX;
    assert!(bad.expand().is_err());
    let mut bad = original.clone();
    bad.format = "future".into();
    assert!(bad.expand().is_err());
    let mut legacy = original.expand().unwrap();
    legacy.provenance.queries[0].value += 1;
    assert!(CompactSnapshot::from_legacy(&legacy).is_err());
    assert!(original.query(QueryId(u64::MAX)).is_err());
}
#[test]
fn invalid_policy_output_is_retained_without_normalizing_it_away() {
    fn invalid(input: &p::Input) -> p::Decision {
        let mut d = p::policy(input);
        d.input.base.owner = 999;
        d
    }
    let mut sim = e::setup(42);
    sim.set_provenance_policy(invalid).unwrap();
    sim.inquiry_meeting(&[0, 2]).unwrap();
    let compact = sim.compact_snapshot("invalid").unwrap();
    assert!(
        compact
            .provenance
            .queries
            .iter()
            .all(|q| q.attempted_base.is_some())
    );
    let legacy = compact.expand().unwrap();
    assert_eq!(legacy, e::snapshot(&sim, "invalid"));
    assert!(
        legacy
            .provenance
            .queries
            .iter()
            .all(|q| !q.valid && q.decision.input.base.owner == 999)
    );
}
#[test]
fn context_interning_is_stable_across_serialization_and_versions() {
    let mut store = ContextStore::default();
    let first = store.intern(vec![1, 2]);
    let second = store.intern(vec![2, 1]);
    assert_ne!(first, second);
    assert_eq!(first, store.intern(vec![1, 2]));
    let mut loaded: ContextStore<Vec<i32>> =
        serde_json::from_slice(&serde_json::to_vec(&store).unwrap()).unwrap();
    assert_eq!(first, loaded.intern(vec![1, 2]));
    assert_eq!(store, loaded);
    assert_eq!(loaded.intern(vec![3]), ContextId(2));
    assert_eq!(loaded.get(first).unwrap(), &vec![1, 2]);
}
#[test]
fn long_history_indexes_rebuild_and_saturation_matches_authoritative_scan() {
    let mut g = Generator::new(42);
    let mut sim = Simulation::new((0..4).map(|i| g.agent(i)).collect()).unwrap();
    sim.set_policy(scarcity_policy);
    for round in 0..100 {
        for owner in [0, 2] {
            sim.set_circumstances(owner, 0, 90, 20, 20).unwrap();
            sim.set_circumstances(owner + 1, 1, 90, 0, 90).unwrap();
        }
        for owner in [0, 2] {
            sim.add_scene([owner, owner + 1], 1, 6).unwrap();
        }
        sim.validate_history_indexes().unwrap();
        sim.run();
        for owner in [0, 2] {
            let event = sim
                .events()
                .iter()
                .rev()
                .find(|e| e.decision.actor == owner + 1)
                .unwrap()
                .id;
            if round % 3 == 0 {
                sim.communicate(
                    owner + 1,
                    owner,
                    event,
                    EvidenceKind::Fallible {
                        scarce: true,
                        reliability: 80,
                        source: 0,
                    },
                )
                .unwrap();
                sim.communicate(
                    owner + 1,
                    owner,
                    event,
                    EvidenceKind::Fallible {
                        scarce: false,
                        reliability: 80,
                        source: 1,
                    },
                )
                .unwrap();
            } else {
                sim.communicate(owner + 1, owner, event, EvidenceKind::Disclosure)
                    .unwrap();
            }
            let cognition = sim.cognition();
            let mut replacements = std::collections::BTreeMap::new();
            for info in cognition.information.iter().filter(|i| i.listener == owner) {
                if let Some(r) = &info.revision {
                    replacements.insert(info.event, r.after.valence);
                }
            }
            let trust = sim
                .events()
                .iter()
                .filter(|e| e.participants.contains(&owner))
                .fold(0i32, |t, e| {
                    let index = e.participants.iter().position(|&id| id == owner).unwrap();
                    (t + replacements
                        .get(&e.id)
                        .copied()
                        .unwrap_or(e.interpretations[index].valence))
                    .clamp(-100, 100)
                });
            assert_eq!(sim.agents()[owner as usize].trust[&(owner + 1)], trust);
        }
        sim.validate_history_indexes().unwrap();
    }
}

#[test]
fn empty_single_odd_and_thousand_agent_compact_snapshots_replay() {
    for count in [0, 1, 11, 1000] {
        let original = e::population(42, count);
        let compact = CompactSnapshot::from_legacy(&original).unwrap();
        let replay = CompactSnapshot::from_legacy(&e::population(42, count)).unwrap();
        assert_eq!(
            serde_json::to_vec(&compact).unwrap(),
            serde_json::to_vec(&replay).unwrap()
        );
        assert_eq!(compact.expand().unwrap(), original);
        if count == 1000 {
            assert!(
                serde_json::to_vec(&compact).unwrap().len()
                    < serde_json::to_vec(&original).unwrap().len()
            );
        }
    }
}

#[test]
fn observer_truth_cannot_be_substituted_for_undisclosed_local_attribution() {
    let mut s = e::setup(42);
    let event = e::event(&s);
    s.inspect_refusal(event, &[(2, sensor())]).unwrap();
    s.provenance_exchange(2, 0, event, false).unwrap();
    s.inquiry_meeting(&[0, 2]).unwrap();
    let compact = s.compact_snapshot("hidden").unwrap();
    compact.validate().unwrap();
    let id = compact.provenance.queries[0].context.0 as usize;
    assert_eq!(compact.contexts.values()[id].knowledge[0].known, None);
    let mut data = serde_json::to_value(&compact).unwrap();
    data["contexts"]["values"][id]["knowledge"][0]["known"] = serde_json::json!(0);
    let corrupted: CompactSnapshot = serde_json::from_value(data).unwrap();
    assert!(
        corrupted
            .validate()
            .unwrap_err()
            .contains("not locally disclosed")
    );
}
