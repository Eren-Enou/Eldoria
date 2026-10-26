    pub fn new_valuation(registered: m::Config) -> Result<Self, String> {
        Self::new(registered)
    }
    pub fn new(registered: m::Config) -> Result<Self, String> {
        registered.validate()?;
        let config = registered.world.clone();
        let (mut sim, start, end, initial, bootstrap_events, initial_contexts, stock, total) =
            Simulation::continuous_bootstrap(config.physics());
        if config.phase == c::Phase::Absolute {
            sim.world.resource_mut::<State>().epoch = 0;
        }
        sim.world.insert_resource(m::Material::new(registered.preparation.clone(), start));
        sim.concurrent_ledger();
        Ok(Self {
            valuation: Some(registered), valuations: vec![], sim, config, start, end,
            initial, bootstrap_events, initial_contexts, stock, total, boundaries: vec![],
        })
    }
