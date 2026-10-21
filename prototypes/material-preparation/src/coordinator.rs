//!025physical phase authority. No policy reads this observer/physical resource.
use super::*;
use crate::preparation as m;

impl Simulation {
    pub(super) fn material_parts25(&self) -> (u64, u64) {
        self.world
            .get_resource::<m::Material>()
            .map_or((0, 0), |s| {
                assert!(s.active.len() <= 4);
                assert!(s.active.iter().all(|p| p.escrow == 0 || p.escrow == 2));
                for owner in 0..4 {
                    assert!(
                        s.slots[owner].is_none()
                            || self.world.resource::<State>().slots[owner].is_none(),
                        "one logical environmental slot"
                    );
                }
                (u64::from(s.settings.raw_initial), s.remaining_mass())
            })
    }
    fn material_point25(&mut self) {
        let tick = self.activity_tick();
        let mut s = self.world.resource_mut::<m::Material>();
        let point = m::Point {
            tick,
            raw: s.raw,
            waste: s.waste,
            active: s.active.clone(),
        };
        s.points.push(point);
    }
    fn material_memory25(
        &mut self,
        owner: u32,
        kind: m::Change,
        after: Option<m::Cell>,
        physical: Option<u64>,
        cache_record: Option<usize>,
    ) {
        let tick = self.activity_tick();
        let mut s = self.world.resource_mut::<m::Material>();
        let before = s.slots[owner as usize].clone();
        let id = s.memories.len() as u64;
        s.memories.push(m::Memory {
            id,
            owner,
            tick,
            kind,
            before,
            after: after.clone(),
            physical,
            cache_record,
        });
        s.slots[owner as usize] = after;
    }
    /// Called only at the existing cache-write seam. A new ready receipt erases
    /// raw knowledge; observer records cannot supply it to subsequent selection.
    pub(super) fn material_cache25(&mut self, owner: u32, after: &Option<k::Cell>) {
        if after.is_none() || self.world.get_resource::<m::Material>().is_none() {
            return;
        }
        if self.world.resource::<m::Material>().slots[owner as usize].is_some() {
            let cache_record = self.world.resource::<State>().memories.len();
            self.material_memory25(
                owner,
                m::Change::OverwrittenByReady,
                None,
                None,
                Some(cache_record),
            );
        }
    }
    fn material_sense25(&mut self, owner: u32, kind: m::Change, physical: Option<u64>) {
        let now = self.activity_tick();
        let settings = self.world.resource::<m::Material>().settings.clone();
        let stock = self.world.resource::<m::Material>().raw;
        let cache_record = if self.world.resource::<State>().slots[owner as usize].is_some() {
            let index = self.world.resource::<State>().memories.len();
            self.concurrent_memory(owner, k::Change::Discarded, None, None);
            Some(index)
        } else {
            None
        };
        let receipt = self.world.resource::<State>().receipts[owner as usize];
        self.world.resource_mut::<State>().receipts[owner as usize] =
            receipt.checked_add(1).unwrap();
        self.material_memory25(
            owner,
            kind,
            Some(m::Cell {
                owner,
                receipt,
                observation: m::Observation {
                    id: settings.source_id,
                    observed_at: now,
                    stock,
                    accessible: settings.access_enabled,
                    extraction_ticks: settings.extraction_ticks,
                    preparation_ticks: settings.preparation_ticks,
                    input_units: 2,
                    edible_units: 1,
                    waste_units: 1,
                },
                own_takes: 0,
            }),
            physical,
            cache_record,
        );
    }
    pub(super) fn material_glance25(&mut self, owner: u32) {
        let now = self.activity_tick();
        let epoch = self.world.resource::<State>().epoch;
        let plan = self
            .world
            .resource::<m::Material>()
            .settings
            .observation
            .clone();
        let offset = (now - epoch) % plan.period;
        if plan.ready_offsets[owner as usize] == Some(offset) {
            let stock = self.world.resource::<State>().stock;
            self.concurrent_sense(owner, k::Change::Sense, stock, None);
        }
        if plan.raw_offsets[owner as usize] == Some(offset) {
            self.material_sense25(owner, m::Change::Sense, None);
        }
    }
    pub(super) fn material_reserve25(&mut self, owner: u32, source: usize) {
        let now = self.activity_tick();
        let settings = self.world.resource::<m::Material>().settings.clone();
        assert!(settings.feature_enabled && settings.access_enabled);
        assert!(
            !self
                .world
                .resource::<m::Material>()
                .active
                .iter()
                .any(|p| p.process.owner == owner)
        );
        let cell = self.world.resource::<m::Material>().slots[owner as usize]
            .as_ref()
            .unwrap();
        assert_eq!(cell.owner, owner);
        assert_eq!(cell.observation.id, settings.source_id);
        assert_eq!(
            (
                cell.observation.extraction_ticks,
                cell.observation.preparation_ticks
            ),
            (settings.extraction_ticks, settings.preparation_ticks)
        );
        assert!(cell.observation.accessible && cell.upper_bound() >= 2);
        assert!(cell.observation.observed_at <= now);
        let entity = self.world.resource::<Index>().0[&owner];
        assert_eq!(self.world.get::<Agent>(entity).unwrap().food, 0);
        assert!(self.world.get::<Agent>(entity).unwrap().hunger > 0);
        let extract_at = now.checked_add(settings.extraction_ticks).unwrap();
        let finish_at = extract_at.checked_add(settings.preparation_ticks).unwrap();
        assert!(finish_at <= self.world.resource::<State>().end);
        let commitment = self.concurrent_reserve(x::Work::Prepare, vec![owner], finish_at, source);
        let process = m::Process {
            commitment,
            owner,
            source: settings.source_id,
            start: now,
            extract_at,
            finish_at,
        };
        {
            let mut s = self.world.resource_mut::<m::Material>();
            assert!(s.active.len() < 4);
            s.processes.push(process.clone());
            s.active.push(m::Active {
                process,
                phase: m::Phase::Extracting,
                escrow: 0,
            });
        }
        self.material_point25();
        self.concurrent_ledger();
    }
    /// Called after inherited need/resource/cache/meal/protocol resolution, before
    /// any next boundary. Ordinary/global query exclusion sees the same live work.
    pub(super) fn material_tick25(&mut self) {
        if self.world.get_resource::<m::Material>().is_none() {
            return;
        }
        let now = self.activity_tick();
        let order = self.world.resource::<State>().config.order;
        let mut extraction = self
            .world
            .resource::<m::Material>()
            .active
            .iter()
            .filter(|p| p.process.extract_at == now && p.phase == m::Phase::Extracting)
            .map(|p| p.process.clone())
            .collect::<Vec<_>>();
        extraction.sort_by_key(|p| (rank(p.owner, p.start, order), p.commitment));
        for p in extraction {
            let live = self
                .world
                .resource::<State>()
                .active
                .iter()
                .find(|a| a.id == p.commitment)
                .unwrap();
            assert!(matches!(live.work, x::Work::Prepare));
            assert_eq!(live.participants, vec![p.owner]);
            assert_eq!((live.start, live.deadline), (p.start, p.finish_at));
            let entity = self.world.resource::<Index>().0[&p.owner];
            let food = self.world.get::<Agent>(entity).unwrap().food;
            let (raw_before, escrow_before, waste) = {
                let s = self.world.resource::<m::Material>();
                assert_eq!(s.settings.source_id, p.source);
                (
                    s.raw,
                    s.active
                        .iter()
                        .find(|a| a.process.commitment == p.commitment)
                        .unwrap()
                        .escrow,
                    s.waste,
                )
            };
            let (raw_after, escrow_after, outcome) = m::extract(raw_before, escrow_before).unwrap();
            let physical = {
                let mut s = self.world.resource_mut::<m::Material>();
                s.raw = raw_after;
                let active = s
                    .active
                    .iter_mut()
                    .find(|a| a.process.commitment == p.commitment)
                    .unwrap();
                active.escrow = escrow_after;
                active.phase = if outcome == m::Outcome::Extracted {
                    m::Phase::Preparing
                } else {
                    m::Phase::Failed
                };
                let id = s.operations.len() as u64;
                s.operations.push(m::Operation {
                    id,
                    tick: now,
                    commitment: p.commitment,
                    owner: p.owner,
                    source: p.source,
                    outcome,
                    raw_before,
                    raw_after,
                    escrow_before,
                    escrow_after,
                    food_before: food,
                    food_after: food,
                    waste_before: waste,
                    waste_after: waste,
                });
                id
            };
            self.concurrent_ledger();
            match outcome {
                m::Outcome::Extracted => {
                    if let Some(mut cell) =
                        self.world.resource::<m::Material>().slots[p.owner as usize].clone()
                    {
                        assert_eq!(cell.observation.id, p.source);
                        cell.own_takes = cell.own_takes.checked_add(2).unwrap();
                        self.material_memory25(
                            p.owner,
                            m::Change::Taken,
                            Some(cell),
                            Some(physical),
                            None,
                        );
                    }
                }
                m::Outcome::Insufficient => {
                    self.material_sense25(p.owner, m::Change::Insufficient, Some(physical))
                }
                _ => unreachable!(),
            }
            self.material_point25();
        }
        let mut completion = self
            .world
            .resource::<m::Material>()
            .active
            .iter()
            .filter(|p| p.process.finish_at == now)
            .map(|p| p.process.clone())
            .collect::<Vec<_>>();
        completion.sort_by_key(|p| (rank(p.owner, p.start, order), p.commitment));
        for p in completion {
            let entity = self.world.resource::<Index>().0[&p.owner];
            let food_before = self.world.get::<Agent>(entity).unwrap().food;
            let (raw, waste_before, escrow_before, phase) = {
                let s = self.world.resource::<m::Material>();
                let a = s
                    .active
                    .iter()
                    .find(|a| a.process.commitment == p.commitment)
                    .unwrap();
                (s.raw, s.waste, a.escrow, a.phase)
            };
            assert!(matches!(
                (phase, escrow_before),
                (m::Phase::Preparing, 2) | (m::Phase::Failed, 0)
            ));
            let (escrow_after, food_after, waste_after, outcome) =
                m::convert(escrow_before, food_before, waste_before).unwrap();
            self.world.get_mut::<Agent>(entity).unwrap().food = food_after;
            {
                let mut s = self.world.resource_mut::<m::Material>();
                s.waste = waste_after;
                let active = s
                    .active
                    .iter_mut()
                    .find(|a| a.process.commitment == p.commitment)
                    .unwrap();
                active.escrow = escrow_after;
                let id = s.operations.len() as u64;
                s.operations.push(m::Operation {
                    id,
                    tick: now,
                    commitment: p.commitment,
                    owner: p.owner,
                    source: p.source,
                    outcome,
                    raw_before: raw,
                    raw_after: raw,
                    escrow_before,
                    escrow_after,
                    food_before,
                    food_after,
                    waste_before,
                    waste_after,
                });
            }
            self.concurrent_ledger();
            let commitment = self
                .world
                .resource::<State>()
                .active
                .iter()
                .find(|a| a.id == p.commitment)
                .unwrap()
                .clone();
            assert_eq!(commitment.deadline, now);
            self.concurrent_release(&commitment);
            self.world
                .resource_mut::<m::Material>()
                .active
                .retain(|a| a.process.commitment != p.commitment);
            self.material_point25();
            self.concurrent_ledger();
        }
    }
}

#[cfg(test)]
mod physical_controls {
    use super::*;

    fn fixture() -> Simulation {
        let config = crate::continuous::matrix()
            .into_iter()
            .find(|c| c.case == "scarcity" && c.mode == crate::continuous::Mode::Continuous)
            .unwrap();
        let (mut sim, start, _, _, _, _, _, _) = Simulation::continuous_bootstrap(config.physics());
        assert_eq!(start, 0);
        sim.world.resource_mut::<State>().end = start + 33;
        sim.world.insert_resource(m::Material::new(
            m::Settings {
                feature_enabled: true,
                access_enabled: true,
                raw_initial: 2,
                ..Default::default()
            },
            start,
        ));
        sim
    }
    #[test]
    fn synthetic_phases_keep_escrow_failed_effort_and_concurrent_meal_real() {
        // Resolver unit fixture only: no registered025world or voluntary outcome.
        let mut sim = fixture();
        for owner in 0..2 {
            sim.material_sense25(owner, m::Change::Sense, None);
            sim.material_reserve25(owner, 0);
        }
        sim.concurrent_reserve(x::Work::Claim, vec![2], 1, 0);
        sim.concurrent_tick();
        sim.concurrent_reserve(x::Work::Meal, vec![2], 2, 0);
        sim.concurrent_tick();
        let mat = sim.world.resource::<m::Material>();
        assert_eq!(mat.raw, 0);
        assert_eq!(mat.waste, 0);
        assert_eq!(
            mat.active.iter().map(|p| p.escrow).collect::<Vec<_>>(),
            vec![2, 0]
        );
        assert_eq!(
            mat.operations.iter().map(|p| p.outcome).collect::<Vec<_>>(),
            vec![m::Outcome::Extracted, m::Outcome::Insufficient]
        );
        assert_eq!(
            sim.agents().iter().map(|a| a.food).collect::<Vec<_>>(),
            vec![0, 0, 0, 0]
        );
        assert_eq!(sim.cognition().consumption.len(), 1);
        while sim.activity_tick() < 5 {
            sim.concurrent_tick();
        }
        assert_eq!(sim.world.resource::<State>().active.len(), 2);
        assert_eq!(sim.world.resource::<Runtime>().busy.len(), 2);
        assert_eq!(
            sim.agents().iter().map(|a| a.hunger).collect::<Vec<_>>(),
            vec![85, 85, 45, 85]
        );
        assert!(sim.agents().iter().all(|a| a.food == 0));
        let point = sim
            .world
            .resource::<m::Material>()
            .points
            .iter()
            .rfind(|p| p.tick <= 3)
            .unwrap();
        assert_eq!(point.active.iter().map(|p| p.escrow).sum::<u32>(), 2);
        sim.concurrent_tick();
        assert!(sim.world.resource::<m::Material>().active.is_empty());
        assert!(sim.world.resource::<State>().active.is_empty());
        assert!(sim.world.resource::<Runtime>().busy.is_empty());
        assert_eq!(sim.world.resource::<m::Material>().waste, 1);
        assert_eq!(
            sim.agents().iter().map(|a| a.food).collect::<Vec<_>>(),
            vec![1, 0, 0, 0]
        );
        let releases = sim
            .world
            .resource::<State>()
            .transitions
            .iter()
            .filter(|r| r.completed && matches!(r.commitment.work, x::Work::Prepare))
            .collect::<Vec<_>>();
        assert_eq!(releases.len(), 2);
        assert!(releases.iter().all(|r| r.tick == 6));
        sim.concurrent_tick();
        assert_eq!(
            sim.agents()[0].food,
            1,
            "conversion must not automatically consume"
        );
        sim.concurrent_ledger();
    }
    #[test]
    fn synthetic_query_admission_cannot_skip_live_preparation_or_make_receipts() {
        let mut sim = fixture();
        sim.material_sense25(0, m::Change::Sense, None);
        sim.material_reserve25(0, 0);
        let before = sim.world.resource::<q::Inquiry>().records.len();
        // Isolate admission's global exclusivity barrier. This synthetic pending
        // protocol is not claimed as an executed research inquiry opportunity.
        sim.world.resource_mut::<State>().pending.push(x::Pending {
            source: 0,
            invitation: e::Invitation {
                proposer: 2,
                recipient: 3,
                kind: e::Kind::Inquiry,
                expires: 0,
            },
            question: q::Action::Ask {
                concern: 0,
                source: 3,
                strategy: q::Strategy::Direct,
            },
            response: Some(x::ReplyRef {
                boundary: 0,
                response: 0,
                selected: e::Reply::Accept(2),
            }),
        });
        sim.concurrent_admit();
        assert_eq!(sim.activity_tick(), 0);
        assert_eq!(sim.world.resource::<q::Inquiry>().records.len(), before);
        assert_eq!(
            sim.world
                .resource::<State>()
                .admissions
                .last()
                .unwrap()
                .reason,
            "AtomicBlocked"
        );
        assert_eq!(sim.world.resource::<State>().active.len(), 1);
    }

    #[test]
    fn legitimate_overwrite_cannot_restore_raw_knowledge_from_its_audit() {
        let mut sim = fixture();
        sim.world.resource_mut::<m::Material>().settings.observation = m::ObservationPlan {
            period: 16,
            raw_offsets: [Some(0); 4],
            ready_offsets: [Some(8); 4],
        };
        sim.material_glance25(0);
        assert_eq!(
            sim.world.resource::<m::Material>().slots[0]
                .as_ref()
                .unwrap()
                .receipt,
            0
        );
        while sim.activity_tick() < 8 {
            sim.concurrent_tick();
        }
        sim.material_glance25(0);
        assert!(sim.world.resource::<m::Material>().slots[0].is_none());
        assert!(sim.world.resource::<State>().slots[0].is_some());
        assert!(!sim.world.resource::<m::Material>().memories.is_empty());
        let inherited = k::Input {
            legacy: sim
                .concurrent_input(0, &sim.concurrent_people(), 41)
                .unwrap(),
            retained: sim.world.resource::<State>().slots[0].clone(),
            duration: 4,
            inquiry_lane_free: true,
        };
        let old = k::policy(&inherited, a::Policy::Need);
        let terms =
            (crate::valuation::Mode::Baseline.selector())(&inherited, &old.candidates).terms;
        let local = m::Input {
            inherited,
            raw_retained: sim.world.resource::<m::Material>().slots[0].clone(),
        };
        assert!(!m::rank(&local, &terms).terms[4].eligible);
        while sim.activity_tick() < 16 {
            sim.concurrent_tick();
        }
        sim.material_glance25(0);
        let cell = sim.world.resource::<m::Material>().slots[0]
            .as_ref()
            .unwrap();
        assert_eq!((cell.receipt, cell.observation.observed_at), (2, 16));
        assert!(sim.world.resource::<State>().slots[0].is_none());
        sim.concurrent_ledger();
    }
}
