use super::*;
impl World {
    pub(super) fn validate_material(&self) -> Result<(), String> {
        let registered = self.valuation.as_ref().ok_or("missing025 configuration")?;
        if self.material().settings != registered.preparation {
            return Err("material settings differ from025 configuration".into());
        }
        crate::material_audit::validate(
            &serde_json::to_value(self.material()).map_err(|e| e.to_string())?,
            &serde_json::to_value(self.sim.world.resource::<State>()).map_err(|e| e.to_string())?,
            self.tick(),
        )
    }
}

#[cfg(test)]
mod corruption_tests {
    use super::*;
    #[derive(Clone, Serialize, Deserialize)]
    struct RawCommit {
        binding_sha256: String,
        schema: String,
        world: String,
        runtime: String,
        generation: u64,
        tick: u64,
        journal_bytes: u64,
        journal_sha256: String,
        state_sha256: String,
        #[serde(with = "serde_bytes")]
        state: Vec<u8>,
    }
    fn rebind(saved: &Saved) -> Vec<u8> {
        let payload = encode(saved).unwrap();
        encode(&Envelope {
            schema: SCHEMA.into(),
            runtime: IDENTITY.into(),
            checksum: format!("{:x}", Sha256::digest(&payload)),
            payload,
        })
        .unwrap()
    }
    fn reject_saved(s: &Saved, template: &RawCommit, prefix: &[u8]) {
        let mut forged = template.clone();
        forged.state = rebind(s);
        assert!(World::restore(&forged.state).is_err());
        forged.state_sha256 = format!("{:x}", Sha256::digest(&forged.state));
        let binding = serde_json::to_vec(&(
            &forged.schema,
            &forged.world,
            &forged.runtime,
            forged.generation,
            forged.tick,
            forged.journal_bytes,
            &forged.journal_sha256,
            &forged.state_sha256,
        ))
        .unwrap();
        forged.binding_sha256 = format!("{:x}", Sha256::digest(binding));
        assert!(crate::verification::verify(&encode(&forged).unwrap(), prefix).is_err());
    }
    #[test]
    fn recomputed_hashes_do_not_hide_material_corruption() {
        let path = std::env::temp_dir().join(format!(
            "woi-material-independent-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut session = session::Session::create(&path, m::matrix()[1].clone()).unwrap();
        while session
            .world()
            .material()
            .active
            .iter()
            .all(|a| a.escrow != 2)
        {
            session.advance(1).unwrap();
        }
        session.save().unwrap();
        let world = session.world();
        let template: RawCommit =
            decode(&std::fs::read(path.join("checkpoint-00000002.cbor")).unwrap()).unwrap();
        let prefix = std::fs::read(path.join("journal.jsonl")).unwrap();
        let mut checks = 0;
        macro_rules! reject {
            ($change:expr) => {{
                let mut s = world.saved();
                $change(&mut s);
                reject_saved(&s, &template, &prefix);
                checks += 1;
            }};
        }
        reject!(|s: &mut Saved| s.material.raw += 1);
        reject!(|s: &mut Saved| s.material.active[0].escrow = 0);
        reject!(|s: &mut Saved| s.material.active[0].phase = m::Phase::Extracting);
        reject!(|s: &mut Saved| s.material.active[0].process.finish_at += 1);
        reject!(|s: &mut Saved| s.material.active[0].process.owner = 3);
        reject!(|s: &mut Saved| s.material.active.clear());
        reject!(|s: &mut Saved| s.material.operations.push(s.material.operations[0].clone()));
        reject!(|s: &mut Saved| s.material.settings.raw_initial += 1);
        reject!(|s: &mut Saved| s.state.active.push(s.state.active[0].clone()));
        reject!(|s: &mut Saved| s.material.memories.pop().unwrap());
        reject!(|s: &mut Saved| s.agents[0].food += 1);
        assert_eq!(checks, 11);
        session.advance(8).unwrap();
        session.save().unwrap();
        let template: RawCommit =
            decode(&std::fs::read(path.join("checkpoint-00000003.cbor")).unwrap()).unwrap();
        let prefix = std::fs::read(path.join("journal.jsonl")).unwrap();
        let mut s = session.world().saved();
        assert!(s.material.waste > 0);
        s.material.waste -= 1;
        reject_saved(&s, &template, &prefix);
        drop(session);
        std::fs::remove_dir_all(path).unwrap();
    }
    #[test]
    fn archive_cannot_restore_an_overwritten_raw_observation() {
        let path = std::env::temp_dir().join(format!(
            "woi-material-forgetting-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut session = session::Session::create(&path, m::matrix()[14].clone()).unwrap();
        let (owner, forgotten) = loop {
            session.advance(1).unwrap();
            if let Some(memory) = session.world().material().memories.iter().rev().find(|r| {
                r.kind == m::Change::OverwrittenByReady
                    && session.world().material().slots[r.owner as usize].is_none()
            }) {
                break (memory.owner, memory.before.clone().unwrap());
            }
            assert!(
                !session.world().complete(),
                "registered overwrite witness absent"
            );
        };
        session.save().unwrap();
        let template: RawCommit =
            decode(&std::fs::read(path.join("checkpoint-00000002.cbor")).unwrap()).unwrap();
        let prefix = std::fs::read(path.join("journal.jsonl")).unwrap();
        let restored = World::restore(&session.world().checkpoint().unwrap()).unwrap();
        assert!(restored.material().slots[owner as usize].is_none());
        let mut saved = session.world().saved();
        saved.material.slots[owner as usize] = Some(forgotten);
        reject_saved(&saved, &template, &prefix);
        drop(session);
        std::fs::remove_dir_all(path).unwrap();
    }
}
