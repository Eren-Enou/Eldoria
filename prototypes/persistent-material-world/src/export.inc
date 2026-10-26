    pub fn material(&self) -> &m::Material {
        self.sim.world.resource::<m::Material>()
    }
    pub fn registration(&self) -> &m::Config {
        self.valuation.as_ref().expect("validated025 registration")
    }
    pub fn trial(&self) -> Result<m::Trial, String> {
        if !self.complete() { return Err("unfinished world is not a completed Trial".into()); }
        self.validate()?;
        let history = self.material_history();
        crate::preparation_observer2::validate(&history.final_state)?;
        let material = self.material().history();
        Ok(m::Trial {schema:"experiment-025-trial/v1".into(),config:self.registration().clone(),
            checkpoints:m::checkpoints(&history,&self.config,&material.points),
            history,valuations:self.valuations.clone(),material})
    }
    pub fn export(&self) -> Result<Vec<u8>, String> {
        serde_json::to_vec(&self.trial()?).map_err(|e|e.to_string())
    }
    fn material_history(&self) -> m::History {
        let sim = &self.sim;
        let s = sim.world.resource::<State>();
        m::History {
            schema:"experiment-025-physical-history/v1".into(),config:self.config.physics(),
            start:self.start,end:self.end,initial:self.initial.clone(),initial_stock:self.stock,
            initial_total:self.total+u64::from(self.material().settings.raw_initial),
            boundaries:self.boundaries.clone(),transitions:s.transitions.clone(),
            admissions:s.admissions.clone(),needs:s.needs.clone(),grants:s.grants.clone(),
            memories:s.memories.clone(),final_memory:s.slots.to_vec(),
            profiles:(0..4).map(|id|sim.intentional().unwrap().profiles[&id].clone()).collect(),
            bootstrap_events:self.bootstrap_events,initial_contexts:self.initial_contexts.clone(),
            final_contexts:sim.world.resource::<x::LocalContexts>().clone(),
            final_agents:sim.agents(),final_stock:s.stock,
            final_state:crate::experiment013::Snapshot {
                base:crate::experiment009::snapshot(sim),learning:sim.self_evaluation().unwrap(),
                mode:crate::experiment013::Mode::Contextual,
            },
        }
    }
