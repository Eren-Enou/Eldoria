//! Cross-mode contracts: persistence is distinct from episodic retention.
use world_of_individuals::{
    assessment::{self as a, Method, Origin, Receipt},
    behavior::scarcity_policy,
    cognition::EvidenceKind,
    experiment008,
    model::{Generator, MEMORY_CAPACITY},
    simulation::Simulation,
};

#[test]
fn finalized_experiment008_reproduces_its_archive_exactly() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("experiments/008/seed-42.json");
    let archived: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    assert_eq!(
        serde_json::to_value(experiment008::suite(42)).unwrap(),
        archived
    );
}

#[test]
fn legacy_evidence_consequences_survive_eviction_without_restoring_episodes() {
    let mut g = Generator::new(42);
    let mut sim = Simulation::new(vec![g.agent(0), g.agent(1)]).unwrap();
    sim.set_policy(scarcity_policy);
    for _ in 0..=MEMORY_CAPACITY {
        sim.set_circumstances(0, 0, 90, 0, 0).unwrap();
        sim.set_circumstances(1, 1, 90, 0, 90).unwrap();
        sim.add_scene([0, 1], 1, 6).unwrap();
        sim.run();
        let event = sim.events().last().unwrap().id;
        sim.communicate(
            1,
            0,
            event,
            EvidenceKind::Fallible {
                scarce: true,
                reliability: 90,
                source: 0,
            },
        )
        .unwrap();
    }
    let original = sim.events()[1].clone();
    assert!(
        !sim.cognition().beliefs[&0]
            .iter()
            .any(|b| b.event == original.id)
    );
    assert!(
        !sim.agents()[0]
            .memories
            .iter()
            .any(|m| m.event == original.id)
    );
    let memories = sim.agents()[0].memories.clone();
    let trust = sim.agents()[0].trust.clone();
    let receipt = sim
        .communicate(
            1,
            0,
            original.id,
            EvidenceKind::Fallible {
                scarce: false,
                reliability: 40,
                source: 1,
            },
        )
        .unwrap();
    assert_eq!(receipt.before, None);
    // Legacy's separately persistent received-evidence maximum is intentional.
    assert_eq!(receipt.after.support, 50);
    assert_eq!(receipt.revision, None);
    assert_eq!(sim.agents()[0].memories, memories);
    assert_eq!(sim.agents()[0].trust, trust);
    assert_eq!(sim.events()[1], original);
    sim.validate_history_indexes().unwrap();
    // Observer inspection and detached export mutation cannot restore live state.
    let mut detached = sim.cognition();
    detached.beliefs.get_mut(&0).unwrap().clear();
    assert_eq!(sim.cognition().beliefs[&0].len(), MEMORY_CAPACITY);
    assert_eq!(sim.agents()[0].memories, memories);
}

#[test]
fn grouped_assessment_is_event_local_and_permutation_invariant_with_unequal_weights() {
    let items: Vec<_> = [
        (Origin::Known(2), true, 37),
        (Origin::Known(0), true, 61),
        (Origin::Unknown(3), false, 43),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (origin, claim, quality))| a::Item {
        receipt: Receipt::Provenance(id as u64),
        event: 1,
        communicator: id as u32,
        message: Some(id as u64),
        origin,
        claim,
        quality,
    })
    .collect();
    let expected = a::grouped(&items, 1);
    assert_eq!(expected, 32); // stable origin order: 61 + 39*37/100 - 43
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let mut delivered: Vec<_> = order.iter().map(|&i| items[i].clone()).collect();
        delivered.extend(items.clone());
        delivered.push(a::Item {
            event: 99,
            quality: 100,
            ..items[2].clone()
        });
        assert_eq!(Method::Grouped.rule()(&delivered, 1), expected);
    }
}
