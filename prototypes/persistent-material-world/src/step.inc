        let registered = self.valuation.as_ref().ok_or("missing025 registration")?;
        let boundary = if registered.preparation.feature_enabled {
            self.sim.preparation_boundary25(self.boundaries.len(), &self.config,
                self.start, self.start+self.config.horizon, registered.valuation, &mut self.valuations)
        } else {
            // The original025 disabled path executes the original024 selector and
            // uses from_legacy only as an observer projection. Preserve that path.
            let mut audit = vec![];
            let boundary = self.sim.valuation_boundary(self.boundaries.len(), &self.config, self.start,
                self.start+self.config.horizon, registered.valuation, &mut audit);
            for record in &mut audit { record.boundary = 0; }
            let mut history = serde_json::to_value(self.material_history()).map_err(|e|e.to_string())?;
            history["schema"] = "experiment-024-physical-history/v1".into();
            history["boundaries"] = serde_json::to_value(vec![boundary]).map_err(|e|e.to_string())?;
            let old = v::Trial {schema:"experiment-024-trial/v1".into(),
                config:v::Config {world:self.config.clone(),valuation:registered.valuation,family:registered.family.clone(),source023:None},
                history:serde_json::from_value(history).map_err(|e|e.to_string())?,checkpoints:vec![],valuations:audit};
            let mut projected = m::from_legacy(registered.clone(), old);
            for mut record in projected.valuations {
                record.boundary = self.boundaries.len();
                self.valuations.push(record);
            }
            projected.history.boundaries.remove(0)
        };
