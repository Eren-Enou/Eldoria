use eldoria_settlement::{Settlement, model::*, observer};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

pub const PAGE_SIZE: usize = 30;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum EventFilter {
    #[default]
    All,
    Consequences,
    Social,
    Failures,
}
impl EventFilter {
    pub fn label(self) -> &'static str {
        match self {
            Self::All => "All records",
            Self::Consequences => "Consequences",
            Self::Social => "Social activity",
            Self::Failures => "Failures & refusals",
        }
    }
    pub fn matches(self, record: &Record) -> bool {
        match self {
            Self::All => true,
            Self::Consequences => {
                matches!(&record.kind, RecordKind::Finished { outcome, .. } if !["replied", "invitation_delivered"].contains(&outcome.as_str()))
                    || matches!(
                        record.kind,
                        RecordKind::Refused { .. } | RecordKind::Rejected { .. }
                    )
            }
            Self::Social => {
                matches!(record.kind, RecordKind::Refused { .. })
                    || matches!(&record.kind, RecordKind::Started { commitment } if matches!(commitment.work, Work::Send { .. } | Work::Transfer { .. } | Work::JointGather))
                    || matches!(&record.kind, RecordKind::Finished { outcome, .. } if ["shared_food", "gathered_together", "transfer_failed"].contains(&outcome.as_str()))
            }
            Self::Failures => {
                matches!(
                    record.kind,
                    RecordKind::Refused { .. } | RecordKind::Rejected { .. }
                ) || matches!(&record.kind, RecordKind::Finished { outcome, .. } if ["depleted", "transfer_failed", "no_food"].contains(&outcome.as_str()))
            }
        }
    }
}
#[derive(Clone, Copy)]
pub enum Direction {
    Earlier,
    Later,
    Latest,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Perspective {
    Observer,
    Character(CharacterId),
}
#[derive(Clone, Debug, Serialize)]
pub struct PersonLabel {
    pub id: CharacterId,
    pub name: String,
}
#[derive(Clone, Debug, Serialize)]
pub struct PlaceLabel {
    pub id: PlaceId,
    pub name: String,
    pub neighbors: Vec<PlaceId>,
}
#[derive(Serialize)]
pub enum View {
    Observer(Box<observer::Snapshot>),
    Character(Box<OwnedView>),
}
#[derive(Clone, Debug, Serialize)]
pub struct Row {
    pub id: u64,
    pub tick: u64,
    pub actor: Option<CharacterId>,
    pub title: String,
}
#[derive(Serialize)]
pub struct Detail {
    pub record: Record,
    pub links: Vec<Row>,
}
#[derive(Serialize)]
pub struct Frame {
    pub perspective: Perspective,
    pub filter: EventFilter,
    pub selected_id: Option<u64>,
    pub selected_anchor: Option<crate::notebook::Anchor>,
    pub notebook: Option<crate::notebook::NotebookView>,
    pub tick: u64,
    pub horizon: u64,
    pub running: bool,
    pub dirty: bool,
    pub last_save: Option<String>,
    /// External follow navigation, not claimed to be an individual's knowledge.
    pub roster: Vec<PersonLabel>,
    pub places: Vec<PlaceLabel>,
    pub activities: Vec<Commitment>,
    pub view: View,
    pub history_total: usize,
    pub page: usize,
    pub rows: Vec<Row>,
    pub detail: Option<Detail>,
    pub detail_error: Option<String>,
}

#[derive(Default)]
pub struct Index {
    scanned: usize,
    children: BTreeMap<u64, Vec<u64>>,
    starts: BTreeMap<u64, u64>,
    categories: BTreeMap<EventFilter, Vec<u64>>,
    by_actor: BTreeMap<CharacterId, Vec<u64>>,
    prefixes: Vec<String>,
    outcomes: BTreeMap<String, usize>,
}
impl Index {
    pub fn update(&mut self, world: &Settlement) {
        for record in world.state().history.iter().skip(self.scanned) {
            for filter in [
                EventFilter::All,
                EventFilter::Consequences,
                EventFilter::Social,
                EventFilter::Failures,
            ] {
                if filter.matches(record) {
                    self.categories.entry(filter).or_default().push(record.id);
                }
            }
            let subjects = match &record.kind {
                RecordKind::Started { commitment } => commitment.participants.clone(),
                _ => record.actor.into_iter().collect(),
            };
            for id in subjects {
                self.by_actor.entry(id).or_default().push(record.id);
            }
            let bytes = serde_json::to_vec(record).expect("serializable history record");
            let mut hash = Sha256::new();
            hash.update(
                self.prefixes
                    .last()
                    .map_or("eldoria-observer-prefix/v1", String::as_str)
                    .as_bytes(),
            );
            hash.update((bytes.len() as u64).to_le_bytes());
            hash.update(bytes);
            self.prefixes.push(format!("{:x}", hash.finalize()));
            let outcome = match &record.kind {
                RecordKind::Finished { outcome, .. } => Some(outcome.clone()),
                RecordKind::Refused { reason, .. } => Some(format!("refused:{reason}")),
                RecordKind::Rejected { reason, .. } => Some(format!("rejected:{reason}")),
                _ => None,
            };
            if let Some(outcome) = outcome {
                *self.outcomes.entry(outcome).or_default() += 1;
            }
            if let Some(cause) = record.cause {
                self.children.entry(cause).or_default().push(record.id);
            }
            if let RecordKind::Started { commitment } = &record.kind {
                self.starts.insert(commitment.id, record.id);
            }
        }
        self.scanned = world.state().history.len();
    }
    pub fn prefix(&self, record: u64) -> Option<&str> {
        self.prefixes
            .get(usize::try_from(record).ok()?)
            .map(String::as_str)
    }
    pub fn visible<'a>(
        &'a self,
        world: &Settlement,
        perspective: Perspective,
        filter: EventFilter,
    ) -> Result<Cow<'a, [u64]>, String> {
        if let Perspective::Character(id) = perspective {
            let receipts = retained(&world.owned_view(id)?);
            let candidates: BTreeSet<_> = self
                .by_actor
                .get(&id)
                .into_iter()
                .flatten()
                .copied()
                .chain(receipts.iter().copied())
                .collect();
            Ok(Cow::Owned(
                candidates
                    .into_iter()
                    .filter(|id| {
                        let r = &world.state().history[*id as usize];
                        filter.matches(r) && allowed(r, perspective, &receipts)
                    })
                    .collect(),
            ))
        } else {
            Ok(Cow::Borrowed(
                self.categories.get(&filter).map_or(&[], Vec::as_slice),
            ))
        }
    }
    pub fn page_for(
        &self,
        world: &Settlement,
        perspective: Perspective,
        filter: EventFilter,
        id: u64,
    ) -> Result<usize, String> {
        let ids = self.visible(world, perspective, filter)?;
        Ok(ids
            .binary_search(&id)
            .map_or(0, |at| (ids.len() - 1 - at) / PAGE_SIZE))
    }
    pub fn navigate(
        &self,
        world: &Settlement,
        perspective: Perspective,
        filter: EventFilter,
        selected: Option<u64>,
        direction: Direction,
    ) -> Result<Option<u64>, String> {
        let filter = if filter == EventFilter::All {
            EventFilter::Consequences
        } else {
            filter
        };
        let ids = self.visible(world, perspective, filter)?;
        Ok(match direction {
            Direction::Latest => ids.last().copied(),
            Direction::Earlier => ids
                .iter()
                .rev()
                .find(|id| selected.is_none_or(|s| **id < s))
                .copied(),
            Direction::Later => ids
                .iter()
                .find(|id| selected.is_none_or(|s| **id > s))
                .copied(),
        })
    }
}
fn retained(view: &OwnedView) -> BTreeSet<u64> {
    view.own
        .memories
        .iter()
        .map(|m| m.receipt)
        .chain(view.own.beliefs.iter().map(|b| b.receipt))
        .chain(view.own.relationships.iter().map(|r| r.receipt))
        .collect()
}
fn allowed(record: &Record, perspective: Perspective, receipts: &BTreeSet<u64>) -> bool {
    match perspective {
        Perspective::Observer => true,
        Perspective::Character(id) => {
            // Foreign decisions always contain private inputs, even if a forged
            // receipt is supplied. Observer-only arbitration is not local feedback.
            match &record.kind {
                RecordKind::Decision { view, .. } => view.own.id == id,
                RecordKind::Started { commitment } => commitment.participants.contains(&id),
                RecordKind::Rejected { .. }
                | RecordKind::Pressure { .. }
                | RecordKind::Observation { .. } => record.actor == Some(id),
                RecordKind::Finished { .. } | RecordKind::Refused { .. } => {
                    record.actor == Some(id) || receipts.contains(&record.id)
                }
            }
        }
    }
}
pub fn action_name(action: &Action) -> String {
    match action {
        Action::Wait => "Wait".into(),
        Action::Eat => "Eat one food".into(),
        Action::Gather => "Gather food".into(),
        Action::Travel { to } => format!("Travel to place {to}"),
        Action::Invite { to, kind } => format!("{} · person {}", invitation_name(*kind), to.0),
        Action::Accept { invitation } => format!("Accept invitation {invitation}"),
        Action::Decline { invitation } => format!("Decline invitation {invitation}"),
    }
}
pub fn invitation_name(kind: InvitationKind) -> &'static str {
    match kind {
        InvitationKind::RequestFood => "Request food",
        InvitationKind::OfferFood => "Offer food",
        InvitationKind::GatherTogether => "Propose joint gathering",
    }
}
pub fn work_name(work: &Work) -> &'static str {
    match work {
        Work::Eat => "Eating",
        Work::Gather => "Gathering",
        Work::Travel { .. } => "Travelling",
        Work::Send { .. } => "Sending invitation",
        Work::Transfer { .. } => "Sharing food",
        Work::JointGather => "Gathering together",
        Work::Reply => "Replying",
    }
}
pub fn row(record: &Record) -> Row {
    let title = match &record.kind {
        RecordKind::Decision { choice, .. } => format!("Chose: {}", action_name(&choice.selected)),
        RecordKind::Started { commitment } => format!(
            "Started {} · commitment {}",
            work_name(&commitment.work).to_lowercase(),
            commitment.id
        ),
        RecordKind::Finished { outcome, units, .. } => {
            format!("{} · {units} food", outcome.replace('_', " "))
        }
        RecordKind::Refused { reason, .. } => format!("Invitation: {}", reason.replace('_', " ")),
        RecordKind::Rejected { reason, .. } => format!("Admission failed: {reason}"),
        RecordKind::Observation { belief } => {
            format!("Saw {} food at place {}", belief.stock, belief.place)
        }
        RecordKind::Pressure { after, .. } => format!("Hunger progressed to {after}"),
    };
    Row {
        id: record.id,
        tick: record.tick,
        actor: record.actor,
        title,
    }
}
pub struct Selection {
    pub perspective: Perspective,
    pub filter: EventFilter,
    pub page: usize,
    pub selected: Option<u64>,
}
pub fn frame(
    world: &Settlement,
    index: &Index,
    selection: Selection,
    running: bool,
    dirty: bool,
    last_save: Option<String>,
) -> Result<Frame, String> {
    let Selection {
        perspective,
        filter,
        page,
        selected,
    } = selection;
    let own = match perspective {
        Perspective::Observer => None,
        Perspective::Character(id) => Some(world.owned_view(id)?),
    };
    let receipts = own.as_ref().map(retained).unwrap_or_default();
    let history = &world.state().history;
    let visible = index.visible(world, perspective, filter)?;
    let page = page.min(visible.len().saturating_sub(1) / PAGE_SIZE);
    let rows = visible
        .iter()
        .rev()
        .skip(page * PAGE_SIZE)
        .take(PAGE_SIZE)
        .map(|id| row(&history[*id as usize]))
        .collect();
    let mut detail_error = None;
    let detail = selected.and_then(|id| {
        let Some(record) = history.get(usize::try_from(id).ok()?).filter(|r| allowed(r, perspective, &receipts)) else {
            detail_error = Some("This record is outside the character's captured perspective. Switch to Observer to inspect its context.".into());
            return None;
        };
        let mut links = BTreeSet::new();
        if let Some(cause) = record.cause { links.insert(cause); }
        if let Some(children) = index.children.get(&id) { links.extend(children); }
        match &record.kind {
            RecordKind::Finished { commitment, .. } => { if let Some(start) = index.starts.get(commitment) { links.insert(*start); } }
            RecordKind::Decision { view, .. } => {
                links.extend(view.incoming.iter().map(|i| i.cause));
                links.extend(view.own.memories.iter().map(|m| m.receipt));
                links.extend(view.own.beliefs.iter().map(|b| b.receipt));
            }
            _ => (),
        }
        let links = links.into_iter().filter_map(|link| history.get(link as usize))
            .filter(|r| allowed(r, perspective, &receipts)).take(24).map(row).collect();
        Some(Detail { record: record.clone(), links })
    });
    let activities = world
        .state()
        .commitments
        .values()
        .filter(|c| match perspective {
            Perspective::Observer => true,
            Perspective::Character(id) => c.participants.contains(&id),
        })
        .cloned()
        .collect();
    let roster = world
        .characters()
        .values()
        .map(|a| PersonLabel {
            id: a.id,
            name: a.name.clone(),
        })
        .collect();
    let places = world
        .state()
        .places
        .values()
        .map(|p| PlaceLabel {
            id: p.id,
            name: p.name.clone(),
            neighbors: p.neighbors.clone(),
        })
        .collect();
    let view = match own {
        Some(view) => View::Character(Box::new(view)),
        None => {
            let characters: Vec<_> = world.characters().into_values().collect();
            let state = world.state();
            let food_balance = observer::FoodBalance {
                initial: state.initial_food,
                consumed: state.consumed,
                sources: state.places.values().map(|p| u64::from(p.food)).sum(),
                owned: characters.iter().map(|a| u64::from(a.food)).sum(),
            };
            View::Observer(Box::new(observer::Snapshot {
                schema: "eldoria-settlement-observer/v1",
                rules: RULES,
                runtime: RUNTIME,
                tick: world.tick(),
                horizon: state.config.horizon,
                complete: world.complete(),
                characters,
                places: state.places.values().cloned().collect(),
                commitments: state.commitments.values().cloned().collect(),
                food_balance,
                history_records: history.len(),
                outcomes: index.outcomes.clone(),
            }))
        }
    };
    Ok(Frame {
        perspective,
        filter,
        selected_id: selected,
        selected_anchor: if perspective == Perspective::Observer {
            selected.and_then(|id| {
                Some(crate::notebook::Anchor {
                    world: crate::notebook::world_key(world),
                    record: id,
                    tick: history.get(id as usize)?.tick,
                    prefix: index.prefix(id)?.to_string(),
                })
            })
        } else {
            None
        },
        notebook: None,
        tick: world.tick(),
        horizon: world.state().config.horizon,
        running,
        dirty,
        last_save,
        roster,
        places,
        activities,
        view,
        history_total: visible.len(),
        page,
        rows,
        detail,
        detail_error,
    })
}
