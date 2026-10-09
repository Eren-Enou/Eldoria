use super::*;
use crate::{assessment as a, inquiry as q, self_evaluation as s};

pub(super) struct Pending {
    basis: Vec<s::Basis>,
    assessment_before: usize,
}
impl Simulation {
    pub fn enable_self_evaluation(&mut self, mode: s::Mode) -> Result<(), String> {
        self.require_idle()?;
        if self
            .world
            .get_resource::<a::Assessment>()
            .is_none_or(|a| a.method != a::Method::Grouped)
        {
            return Err("enable Grouped assessment first".into());
        }
        if self
            .world
            .contains_resource::<crate::attention::Attention>()
            || self
                .world
                .contains_resource::<crate::inquiry_value::ValueAudit>()
        {
            return Err("012 cannot combine with 010/011 projections".into());
        }
        if let Some(old) = self.world.get_resource::<s::Learning>() {
            return if old.mode == mode {
                Ok(())
            } else {
                Err("self evaluation mode fixed for run".into())
            };
        }
        let first = self.world.resource::<q::Inquiry>().records.len();
        self.world.insert_resource(s::Learning::new(mode, first));
        self.world.insert_resource(s::Policy(s::policy));
        Ok(())
    }
    pub fn self_evaluation(&self) -> Option<s::Learning> {
        self.world.get_resource::<s::Learning>().cloned()
    }
    pub fn set_self_evaluation_policy(
        &mut self,
        policy: fn(
            crate::provenance::Decision,
            s::Mode,
            &[s::Basis],
            &[s::Cell],
        ) -> crate::provenance::Decision,
    ) -> Result<(), String> {
        self.require_idle()?;
        self.world
            .get_resource_mut::<s::Policy>()
            .ok_or("enable self evaluation first")?
            .0 = policy;
        Ok(())
    }
    pub(super) fn self_evaluation_decide(
        &self,
        decision: crate::provenance::Decision,
    ) -> (crate::provenance::Decision, Option<Pending>) {
        let Some(learning) = self.world.get_resource::<s::Learning>() else {
            return (decision, None);
        };
        let owner = decision.input.base.owner;
        let assessment = self.world.resource::<a::Assessment>();
        let items: Vec<_> = assessment
            .items
            .get(&owner)
            .into_iter()
            .flatten()
            .cloned()
            .collect();
        let basis = s::project(&decision.input.base.concerns, &items);
        let cells: Vec<_> = learning
            .cells
            .get(&owner)
            .into_iter()
            .flatten()
            .cloned()
            .collect();
        let decision =
            (self.world.resource::<s::Policy>().0)(decision, learning.mode, &basis, &cells);
        (
            decision,
            Some(Pending {
                basis,
                assessment_before: assessment.records.len(),
            }),
        )
    }
    pub(super) fn self_evaluation_finish(&mut self, r: s::Experience, pending: Option<Pending>) {
        let Some(pending) = pending else {
            return;
        };
        let owner = r.owner;
        let assessment = self.world.resource::<a::Assessment>();
        let end = assessment.records.len();
        let mut record = s::Record {
            inquiry: r.inquiry,
            owner,
            basis: pending.basis,
            assessment_before: pending.assessment_before,
            assessment_after: end,
            selected: r.selected,
            outcome: None,
            before: None,
            after: None,
            evicted: None,
        };
        if r.valid_learning
            && let q::Action::Ask {
                concern,
                source,
                strategy,
            } = r.selected
        {
            let basis = record.basis.iter().find(|b| b.concern == concern).unwrap();
            let items: Vec<_> = assessment
                .items
                .get(&owner)
                .into_iter()
                .flatten()
                .cloned()
                .collect();
            let outcome = s::Outcome {
                support_before: basis.support,
                support_after: a::grouped(&items, basis.event),
                status_before: r.status_before.unwrap(),
                status_after: r.status_after.unwrap(),
                receipt_acquired: r.receipt_acquired,
                novelty: r.novelty,
                novelty_value: r.novelty_value,
                time: r.time,
            };
            let (before, after, evicted) = self.world.resource_mut::<s::Learning>().learn(
                owner,
                (source, strategy, basis.context),
                outcome.target(),
                r.inquiry,
            );
            record.outcome = Some(outcome);
            record.before = before;
            record.after = Some(after);
            record.evicted = evicted;
        }
        self.world
            .resource_mut::<s::Learning>()
            .records
            .push(record);
    }
}
