use super::*;
use crate::{
    assessment as a,
    inquiry_value::{self as v, Mode, ValueAudit},
};
impl Simulation {
    pub fn enable_inquiry_value(&mut self, mode: Mode) -> Result<(), String> {
        self.require_idle()?;
        if self
            .world
            .contains_resource::<crate::self_evaluation::Learning>()
        {
            return Err("012 cannot combine with 010/011 projections".into());
        }
        if self
            .world
            .get_resource::<a::Assessment>()
            .is_none_or(|s| s.method != a::Method::Grouped)
        {
            return Err("enable Grouped assessment first".into());
        }
        if self
            .world
            .contains_resource::<crate::attention::Attention>()
        {
            return Err("011 and 010 projections cannot be combined".into());
        }
        if let Some(s) = self.world.get_resource::<ValueAudit>() {
            return if s.mode == mode {
                Ok(())
            } else {
                Err("inquiry value mode fixed for run".into())
            };
        }
        let first_query = self
            .world
            .resource::<crate::audit::RuntimeProvenance>()
            .queries
            .len();
        self.world.insert_resource(ValueAudit {
            mode,
            first_query,
            records: vec![],
        });
        self.world.insert_resource(v::Policy(v::policy));
        Ok(())
    }
    pub fn inquiry_value(&self) -> Option<ValueAudit> {
        self.world.get_resource::<ValueAudit>().cloned()
    }
    /// Replace only the experimental StatusValue scorer; controls stay exact.
    pub fn set_inquiry_value_policy(
        &mut self,
        policy: fn(
            crate::provenance::Decision,
            &[crate::attention::Basis],
        ) -> crate::provenance::Decision,
    ) -> Result<(), String> {
        self.require_idle()?;
        self.world
            .get_resource_mut::<v::Policy>()
            .ok_or("enable inquiry value first")?
            .0 = policy;
        Ok(())
    }
}
