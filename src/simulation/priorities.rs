//! Explicit019 quiescent observation timing; protected018 logic is reused unchanged.
use super::*;
impl Simulation {
    pub fn priorities_need_count(&self) -> usize {
        self.world.resource::<crate::recurring::History>().0.len()
    }
    /// External sampling interval, not an invented actor decision. The019 controller
    /// calls this only after all three actually chose Wait with no active work.
    pub fn priorities_idle(&mut self, extra: u64) -> Result<(), String> {
        self.require_idle()?;
        let env = self
            .world
            .get_resource::<crate::recurring::Environment>()
            .ok_or("open recurring protocol first")?;
        let now = self.activity_tick();
        if extra == 0 || env.active.is_some() || now.checked_add(extra).is_none_or(|t| t > env.end)
        {
            return Err("invalid019 observation interval".into());
        }
        self.world.resource_mut::<Runtime>().tick += extra;
        self.sync_recurring();
        Ok(())
    }
}
