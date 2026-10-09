use super::*;
use crate::attention::{Attention, Basis, Mode, Policy};
use crate::{attention::policy, provenance as p};
impl Simulation {
    pub fn enable_attention(&mut self, mode: Mode) -> Result<(), String> {
        self.require_idle()?;
        // Only the new 011 configuration is excluded; historical paths are identical.
        if self
            .world
            .contains_resource::<crate::inquiry_value::ValueAudit>()
        {
            return Err("011 and 010 projections cannot be combined".into());
        }
        if self
            .world
            .get_resource::<crate::assessment::Assessment>()
            .is_none_or(|s| s.method != crate::assessment::Method::Grouped)
        {
            return Err("enable Grouped assessment first".into());
        }
        if let Some(state) = self.world.get_resource::<Attention>() {
            return if state.mode == mode {
                Ok(())
            } else {
                Err("attention mode fixed for run".into())
            };
        }
        let first_query = self
            .world
            .resource::<crate::audit::RuntimeProvenance>()
            .queries
            .len();
        self.world.insert_resource(Attention {
            mode,
            first_query,
            records: vec![],
        });
        self.world.insert_resource(Policy(policy));
        Ok(())
    }
    pub fn attention(&self) -> Option<Attention> {
        self.world.get_resource::<Attention>().cloned()
    }
    pub fn set_attention_policy(
        &mut self,
        policy: fn(p::Decision, &[Basis]) -> p::Decision,
    ) -> Result<(), String> {
        self.require_idle()?;
        self.world
            .get_resource_mut::<Policy>()
            .ok_or("enable attention first")?
            .0 = policy;
        Ok(())
    }
}
