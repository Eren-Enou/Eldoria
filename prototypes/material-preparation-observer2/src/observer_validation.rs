//! Observer-only namespace bridge. Never changes live state or exported inputs.
use crate::experiment013::Snapshot;

pub fn validate(snapshot: &Snapshot) -> Result<(), String> {
    let mut observer = snapshot.clone();
    let compact = &mut observer.base.base.base;
    let base = &compact.base.base;
    for record in &mut compact.base.inquiry.records {
        if !record.valid {
            continue;
        }
        let observed = record
            .tick
            .checked_sub(u64::from(record.time_spent))
            .ok_or("025 observer: invalid inquiry interval")?;
        let input = &mut record.decision.input;
        let completed: Vec<_> = base
            .scenes
            .iter()
            .filter(|scene| {
                base.events.iter().any(|event| {
                    event.scene == scene.id && event.outcome.is_some() && event.tick <= observed
                })
            })
            .collect();
        // A concurrent query is admitted only after all existing work settles.
        // A hole here cannot be repaired by namespace translation.
        if completed
            .iter()
            .enumerate()
            .any(|(n, scene)| scene.id != n as u64)
        {
            return Err("025 observer: non-prefix completed scenes at atomic inquiry".into());
        }
        let local_now = completed
            .iter()
            .filter(|scene| scene.participants.contains(&input.owner))
            .count() as u64;
        if input.scene != local_now {
            return Err("025 observer: incorrect actor-local current scene".into());
        }
        let global_now = completed.len() as u64;
        for concern in &mut input.concerns {
            let event = base
                .events
                .get(concern.event as usize)
                .ok_or("025 observer: missing concern event")?;
            if event.id != concern.event
                || concern.owner != input.owner
                || concern.target != event.decision.actor
                || !event.participants.contains(&input.owner)
                || event.tick > observed
            {
                return Err("025 observer: invalid concern ownership or time".into());
            }
            let local_created = base
                .scenes
                .iter()
                .filter(|scene| scene.id < event.scene && scene.participants.contains(&input.owner))
                .count() as u64;
            if concern.created_scene != local_created
                || (concern.created_scene < input.scene) != (event.scene < global_now)
            {
                return Err("025 observer: incorrect actor-local concern scene".into());
            }
            concern.created_scene = event.scene;
        }
        // Both coordinates must be translated together, preserving the existing
        // policy replay's created_scene < scene eligibility. The original input
        // and serialized snapshot are never modified.
        input.scene = global_now;
    }
    observer.validate()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn captured() -> Snapshot {
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../experiments/025/diagnostics/scene-namespace/failed-snapshot.json"
        )))
        .unwrap()
    }

    #[test]
    fn checked_translation_passes_full_validator_and_preserves_original() {
        let snapshot = captured();
        let before = serde_json::to_vec(&snapshot).unwrap();
        assert_eq!(
            snapshot.validate().unwrap_err(),
            "concern event ownership mismatch"
        );
        validate(&snapshot).unwrap();
        assert_eq!(serde_json::to_vec(&snapshot).unwrap(), before);
    }

    #[test]
    fn corrupt_locality_and_counter_cases_are_rejected() {
        for defect in 0..7 {
            let mut snapshot = captured();
            let compact = &mut snapshot.base.base.base;
            let input = &mut compact.base.inquiry.records[0].decision.input;
            match defect {
                0 => input.concerns[0].owner = 0,
                1 => input.concerns[0].target = 0,
                2 => compact.base.base.events[9].tick = 1000,
                3 => input.concerns[0].created_scene = 3,
                4 => input.scene = 7,
                5 => compact.base.base.events[9].participants = [0, 1],
                _ => input.concerns[0].event = 10000,
            }
            assert!(validate(&snapshot).is_err(), "accepted defect{defect}");
        }
    }

    #[test]
    fn unrelated_inherited_validation_remains_active() {
        let mut snapshot = captured();
        snapshot.base.base.base.format = "invalid".into();
        assert!(
            validate(&snapshot)
                .unwrap_err()
                .contains("unsupported audit format")
        );
    }
}
