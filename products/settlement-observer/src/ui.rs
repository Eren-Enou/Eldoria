use crate::{
    host::{Client, Command},
    projection::{self, Direction, EventFilter, Frame, Perspective, View},
};
use eframe::egui::{self, Color32, RichText, Stroke};
use eldoria_settlement::{Config, model::*};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const GREEN: Color32 = Color32::from_rgb(124, 200, 164);
const GOLD: Color32 = Color32::from_rgb(222, 189, 124);
const MUTED: Color32 = Color32::from_rgb(151, 166, 177);
#[path = "notebook_ui.rs"]
mod notebook_ui;

#[derive(Clone)]
enum Intent {
    New(Config),
    Open(PathBuf),
    Close,
}
enum Dialog {
    New,
    Open,
    Save(Option<Intent>),
    Unsaved(Intent),
}
pub struct ObserverApp {
    client: Client,
    frame: Option<Frame>,
    perspective: Perspective,
    filter: EventFilter,
    follow: CharacterId,
    page: usize,
    selected: Option<u64>,
    speed: u32,
    status: String,
    error: bool,
    dialog: Option<Dialog>,
    seed: String,
    population: usize,
    horizon: u64,
    file: String,
    workspace: PathBuf,
    after_save: BTreeMap<u64, Option<Intent>>,
    after_note: BTreeMap<u64, (notebook_ui::Draft, Option<Intent>)>,
    notebook: notebook_ui::Editor,
    resume_intent: Option<Intent>,
    replacements: BTreeSet<u64>,
    view_revision: u64,
    closing: bool,
    pending: usize,
}
impl ObserverApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        config: Config,
        workspace: PathBuf,
        open: Option<PathBuf>,
    ) -> Result<Self, String> {
        cc.egui_ctx.set_theme(egui::ThemePreference::Dark);
        cc.egui_ctx.style_mut(|style| {
            style.spacing.item_spacing = egui::vec2(10.0, 8.0);
            style.spacing.button_padding = egui::vec2(12.0, 7.0);
            style.visuals.panel_fill = Color32::from_rgb(22, 30, 38);
            style.visuals.window_fill = Color32::from_rgb(29, 39, 48);
            style.visuals.selection.bg_fill = Color32::from_rgb(39, 87, 73);
        });
        let mut client = Client::spawn_with_notebook(config.clone(), workspace.join("notebooks"))?;
        let mut pending = 0;
        if let Some(path) = open {
            client.send(Command::Open(path))?;
            pending += 1;
        }
        Ok(Self {
            client,
            frame: None,
            perspective: Perspective::Observer,
            filter: EventFilter::All,
            follow: CharacterId(1001),
            page: 0,
            selected: None,
            speed: 8,
            status: "A finite world. Follow a person and see what unfolds.".into(),
            error: false,
            dialog: None,
            seed: config.seed.to_string(),
            population: config.population,
            horizon: config.horizon,
            file: String::new(),
            workspace,
            after_save: BTreeMap::new(),
            after_note: BTreeMap::new(),
            notebook: notebook_ui::Editor::default(),
            resume_intent: None,
            replacements: BTreeSet::new(),
            view_revision: 0,
            closing: false,
            pending,
        })
    }
    fn send(&mut self, command: Command) -> Option<u64> {
        match self.client.send(command) {
            Ok(id) => {
                self.pending += 1;
                Some(id)
            }
            Err(error) => {
                self.status = error;
                self.error = true;
                None
            }
        }
    }
    fn refresh_view(&mut self) {
        self.view_revision += 1;
        self.frame = None;
        self.send(Command::View {
            perspective: self.perspective,
            filter: self.filter,
            page: self.page,
            selected: self.selected,
        });
    }
    fn select(&mut self, id: u64) {
        self.filter = EventFilter::All;
        self.selected = Some(id);
        self.refresh_view();
    }
    fn set_perspective(&mut self, perspective: Perspective) {
        if self.perspective != perspective {
            self.perspective = perspective;
            self.page = 0;
            self.selected = None;
            self.refresh_view();
        }
    }
    fn save_name(&self) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        self.workspace
            .join(format!("settlement-{now}.json"))
            .display()
            .to_string()
    }
    fn request_save(&mut self, after: Option<Intent>) {
        self.send(Command::Running(false));
        self.file = self.save_name();
        self.dialog = Some(Dialog::Save(after));
    }
    fn request(&mut self, intent: Intent, ctx: &egui::Context) {
        if self.notebook.dirty()
            || self.pending > 0
            || self
                .frame
                .as_ref()
                .is_none_or(|f| (f.dirty && f.tick > 0) || f.running)
        {
            self.send(Command::Running(false));
            self.dialog = Some(Dialog::Unsaved(intent));
        } else {
            self.perform(intent, ctx);
        }
    }
    fn perform(&mut self, intent: Intent, ctx: &egui::Context) {
        self.notebook.clear();
        match intent {
            Intent::Close => {
                self.closing = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            Intent::New(config) => {
                self.frame = None;
                if let Some(id) = self.send(Command::New(config)) {
                    self.replacements.insert(id);
                }
            }
            Intent::Open(path) => {
                self.frame = None;
                if let Some(id) = self.send(Command::Open(path)) {
                    self.replacements.insert(id);
                }
            }
        }
    }
    fn poll(&mut self, ctx: &egui::Context) {
        loop {
            match self.client.poll() {
                Ok(Some(reply)) => {
                    self.pending = self.pending.saturating_sub(1);
                    let after = self.after_save.remove(&reply.id);
                    let note_after = self.after_note.remove(&reply.id);
                    let replacement = self.replacements.remove(&reply.id);
                    match reply.result {
                        Ok(message) => {
                            if replacement {
                                self.perspective = Perspective::Observer;
                                self.page = 0;
                                self.selected = None;
                            }
                            if message != "View updated" {
                                self.status = message;
                                self.error = false;
                            }
                            if let Some(Some(intent)) = after {
                                self.resume_intent = Some(intent);
                            }
                            if let Some((submitted, intent)) = note_after {
                                self.notebook.saved(&submitted);
                                if let Some(intent) = intent {
                                    self.resume_intent = Some(intent);
                                }
                            }
                        }
                        Err(error) => {
                            self.status = error;
                            self.error = true;
                            if let Some(intent) = after {
                                self.dialog = Some(Dialog::Save(intent));
                            }
                        }
                    }
                }
                Ok(None) => break,
                Err(error) => {
                    self.status = error;
                    self.error = true;
                    break;
                }
            }
        }
        if self.pending == 0
            && self.replacements.is_empty()
            && let Some(frame) = self.client.take_frame()
            && frame.perspective == self.perspective
            && frame.filter == self.filter
        {
            if !frame.roster.iter().any(|p| p.id == self.follow)
                && let Some(first) = frame.roster.first()
            {
                self.follow = first.id;
            }
            self.page = frame.page;
            self.selected = frame.selected_id;
            self.notebook.sync(&frame);
            self.frame = Some(frame);
        }
        if self.pending == 0
            && self.frame.is_some()
            && let Some(intent) = self.resume_intent.take()
        {
            self.request(intent, ctx);
        }
    }
    fn header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("ELDORIA").size(25.0).strong().color(GOLD));
            ui.label(RichText::new("SETTLEMENT OBSERVER").size(12.0).color(MUTED));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Save checkpoint").clicked() {
                    self.request_save(None);
                }
                if ui.button("Open").clicked() {
                    self.file.clear();
                    self.dialog = Some(Dialog::Open);
                }
                if ui.button("New settlement").clicked() {
                    self.dialog = Some(Dialog::New);
                }
            });
        });
        ui.separator();
        ui.horizontal(|ui| {
            let running = self.frame.as_ref().is_some_and(|f| f.running);
            let complete = self.frame.as_ref().is_some_and(|f| f.tick == f.horizon);
            ui.add_enabled_ui(self.pending == 0 && !complete, |ui| {
                if ui.button(if running { "Pause" } else { "Run" }).clicked() {
                    self.send(Command::Running(!running));
                }
                if ui.button("Step").clicked() {
                    self.send(Command::Step);
                }
            });
            ui.label(if complete {
                RichText::new("SCENARIO COMPLETE").color(GOLD)
            } else if running {
                RichText::new("RUNNING").color(GREEN)
            } else {
                RichText::new("PAUSED").color(MUTED)
            });
            if let Some(frame) = &self.frame {
                ui.label(format!("Tick {} / {}", frame.tick, frame.horizon));
                if frame.dirty {
                    ui.label(RichText::new("Unsaved progress").small().color(GOLD));
                }
            }
            if self.notebook.dirty() {
                ui.label(RichText::new("Unsaved observer note").small().color(GOLD));
            }
            ui.separator();
            ui.label("Pace");
            if ui
                .add(egui::Slider::new(&mut self.speed, 1..=64).suffix(" ticks/s"))
                .changed()
            {
                self.send(Command::Rate(self.speed));
            }
            ui.label(
                RichText::new("Pace changes playback speed, never the rules.")
                    .small()
                    .color(MUTED),
            );
        });
    }
    fn people(&mut self, ui: &mut egui::Ui, frame: &Frame) {
        ui.heading("People");
        ui.label(RichText::new("Follow a character").small().color(MUTED));
        let previous = self.follow;
        egui::ComboBox::from_id_salt("follow")
            .selected_text(person(frame, self.follow))
            .width(205.0)
            .show_ui(ui, |ui| {
                for p in &frame.roster {
                    ui.selectable_value(&mut self.follow, p.id, &p.name);
                }
            });
        if previous != self.follow && matches!(self.perspective, Perspective::Character(_)) {
            self.set_perspective(Perspective::Character(self.follow));
        }
        ui.add_space(8.0);
        ui.label(RichText::new("Perspective").strong());
        if ui
            .selectable_label(
                self.perspective == Perspective::Observer,
                "Observer · whole settlement",
            )
            .clicked()
        {
            self.set_perspective(Perspective::Observer);
        }
        if ui
            .selectable_label(
                matches!(self.perspective, Perspective::Character(_)),
                "Character · local knowledge",
            )
            .clicked()
        {
            self.set_perspective(Perspective::Character(self.follow));
        }
        ui.separator();
        if frame.perspective != self.perspective {
            return;
        }
        match &frame.view {
            View::Observer(snapshot) => {
                ui.label(
                    RichText::new("All individuals · observer information")
                        .small()
                        .color(MUTED),
                );
                for character in &snapshot.characters {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        if ui
                            .selectable_label(character.id == self.follow, &character.name)
                            .clicked()
                        {
                            self.follow = character.id;
                        }
                        ui.label(format!(
                            "{} food   |   hunger {}",
                            character.food, character.hunger
                        ));
                        ui.label(
                            RichText::new(activity(frame, character.id))
                                .small()
                                .color(GREEN),
                        );
                    });
                }
            }
            View::Character(view) => {
                ui.label(RichText::new("Co-present people").strong());
                if view.people.is_empty() {
                    ui.label("No one currently in view.");
                }
                for p in &view.people {
                    ui.label(person(frame, p.id));
                    ui.label(
                        RichText::new(if p.available { "Available" } else { "Occupied" })
                            .small()
                            .color(MUTED),
                    );
                    if p.asking_food {
                        ui.label(RichText::new("Publicly asked for food").small().color(GOLD));
                    }
                }
                ui.add_space(12.0);
                ui.label(
                    RichText::new("Other people's needs, inventories and beliefs are private.")
                        .small()
                        .color(MUTED),
                );
            }
        }
    }
    fn map(&mut self, ui: &mut egui::Ui, frame: &Frame) {
        if frame.perspective != self.perspective {
            return;
        }
        ui.heading("A small world, shared consequences");
        ui.label(RichText::new(match frame.perspective {
            Perspective::Observer => "Current state · finite food sources · no scripted encounters",
            Perspective::Character(_) => "Current local perspective · distant stocks are retained observations, not live truth",
        }).color(MUTED));
        let (response, painter) = ui.allocate_painter(
            egui::vec2(ui.available_width(), 235.0),
            egui::Sense::hover(),
        );
        let rect = response.rect;
        painter.rect_filled(rect.shrink(2.0), 12.0, Color32::from_rgb(16, 24, 29));
        let positions: BTreeMap<_, _> = frame
            .places
            .iter()
            .enumerate()
            .map(|(i, p)| {
                (
                    p.id,
                    egui::pos2(
                        rect.left() + rect.width() * (0.17 + i as f32 * 0.33),
                        rect.top() + 84.0,
                    ),
                )
            })
            .collect();
        for p in &frame.places {
            for neighbor in p.neighbors.iter().filter(|n| **n > p.id) {
                painter.line_segment(
                    [positions[&p.id], positions[neighbor]],
                    Stroke::new(3.0_f32, Color32::from_rgb(65, 80, 78)),
                );
            }
        }
        for p in &frame.places {
            let at = positions[&p.id];
            painter.circle_filled(
                at,
                24.0,
                if p.id == 0 {
                    Color32::from_rgb(88, 77, 55)
                } else {
                    Color32::from_rgb(35, 83, 62)
                },
            );
            painter.circle_stroke(at, 25.0, Stroke::new(1.5_f32, GREEN));
            painter.text(
                at,
                egui::Align2::CENTER_CENTER,
                p.id.to_string(),
                egui::FontId::proportional(16.0),
                GOLD,
            );
            painter.text(
                at + egui::vec2(0.0, -42.0),
                egui::Align2::CENTER_CENTER,
                &p.name,
                egui::FontId::proportional(15.0),
                Color32::WHITE,
            );
            let stock = match &frame.view {
                View::Observer(s) => s
                    .places
                    .iter()
                    .find(|place| place.id == p.id)
                    .map(|p| format!("{} food remaining", p.food))
                    .unwrap_or_default(),
                View::Character(v) => v
                    .own
                    .beliefs
                    .iter()
                    .find(|b| b.place == p.id)
                    .map(|b| format!("Last saw {} food · tick {}", b.stock, b.observed_at))
                    .unwrap_or("Stock unknown".into()),
            };
            painter.text(
                at + egui::vec2(0.0, 42.0),
                egui::Align2::CENTER_CENTER,
                stock,
                egui::FontId::proportional(12.0),
                MUTED,
            );
            let visible: Vec<_> = match &frame.view {
                View::Observer(s) => s
                    .characters
                    .iter()
                    .filter(|a| a.place == p.id)
                    .map(|a| a.id)
                    .collect(),
                View::Character(v) if v.own.place == p.id => std::iter::once(v.own.id)
                    .chain(v.people.iter().map(|p| p.id))
                    .collect(),
                _ => vec![],
            };
            for (i, id) in visible.iter().take(12).enumerate() {
                let dot =
                    at + egui::vec2((i % 6) as f32 * 17.0 - 42.0, 71.0 + (i / 6) as f32 * 20.0);
                let click = ui.interact(
                    egui::Rect::from_center_size(dot, egui::vec2(16.0, 18.0)),
                    egui::Id::new(("map-person", id.0)),
                    egui::Sense::click(),
                );
                painter.circle_filled(
                    dot,
                    if *id == self.follow { 6.5 } else { 4.5 },
                    if *id == self.follow { GOLD } else { GREEN },
                );
                if click
                    .clone()
                    .on_hover_text(format!("{} · {}", person(frame, *id), activity(frame, *id)))
                    .clicked()
                {
                    self.follow = *id;
                    if matches!(self.perspective, Perspective::Character(_)) {
                        self.set_perspective(Perspective::Character(*id));
                    }
                }
            }
        }
        if let View::Observer(s) = &frame.view {
            ui.horizontal_wrapped(|ui| {
                ui.label(format!(
                    "Food: {} in sources + {} owned + {} consumed = {} initial",
                    s.food_balance.sources,
                    s.food_balance.owned,
                    s.food_balance.consumed,
                    s.food_balance.initial
                ));
                ui.label(
                    RichText::new(format!("{} active commitments", frame.activities.len()))
                        .color(GREEN),
                );
            });
        }
        ui.add_space(8.0);
        egui::CollapsingHeader::new("Current commitments")
            .default_open(true)
            .show(ui, |ui| {
                if frame.activities.is_empty() {
                    ui.label("No work currently in progress.");
                }
                for job in frame.activities.iter().take(12) {
                    let names = job
                        .participants
                        .iter()
                        .map(|id| person(frame, *id))
                        .collect::<Vec<_>>()
                        .join(" + ");
                    ui.horizontal_wrapped(|ui| {
                        ui.label(format!("{names}: {}", projection::work_name(&job.work)));
                        ui.label(
                            RichText::new(format!("tick {} → {}", job.start, job.due))
                                .small()
                                .color(MUTED),
                        );
                        if ui.small_button("Trace decision").clicked() {
                            self.select(job.cause);
                        }
                    });
                }
            });
    }
    fn history(&mut self, ui: &mut egui::Ui, frame: &Frame) {
        if frame.perspective != self.perspective {
            return;
        }
        ui.separator();
        ui.horizontal(|ui| {
            ui.heading("Recorded history");
            ui.label(
                RichText::new(format!("{} visible records", frame.history_total))
                    .small()
                    .color(MUTED),
            );
            if ui
                .add_enabled(frame.page > 0, egui::Button::new("Newer"))
                .clicked()
            {
                self.page = frame.page - 1;
                self.refresh_view();
            }
            if ui
                .add_enabled(
                    (frame.page + 1) * projection::PAGE_SIZE < frame.history_total,
                    egui::Button::new("Older"),
                )
                .clicked()
            {
                self.page = frame.page + 1;
                self.refresh_view();
            }
        });
        ui.horizontal_wrapped(|ui| {
            let old_filter = self.filter;
            egui::ComboBox::from_id_salt("event-filter")
                .selected_text(self.filter.label())
                .show_ui(ui, |ui| {
                    for filter in [
                        EventFilter::All,
                        EventFilter::Consequences,
                        EventFilter::Social,
                        EventFilter::Failures,
                    ] {
                        ui.selectable_value(&mut self.filter, filter, filter.label());
                    }
                });
            if self.filter != old_filter {
                self.page = 0;
                self.selected = None;
                self.refresh_view();
            }
            for (label, direction) in [
                ("Earlier event", Direction::Earlier),
                ("Later event", Direction::Later),
                ("Latest event", Direction::Latest),
            ] {
                if ui.button(label).clicked() {
                    self.frame = None;
                    self.send(Command::Navigate(direction));
                }
            }
        });
        ui.label(RichText::new("Event navigation highlights recorded consequences and failures; it is not a wellbeing ranking.").small().color(MUTED));
        ui.label(RichText::new("Select a record to inspect its causes. History selection never rewinds the live world.").small().color(MUTED));
        egui::ScrollArea::vertical()
            .id_salt("history-list")
            .max_height(260.0)
            .show(ui, |ui| {
                for row in &frame.rows {
                    let who = row.actor.map(|id| person(frame, id)).unwrap_or_default();
                    let label = format!("{:>4}  ·  {}  ·  {}", row.tick, who, row.title);
                    if ui
                        .selectable_label(self.selected == Some(row.id), label)
                        .clicked()
                    {
                        self.selected = Some(row.id);
                        self.refresh_view();
                    }
                }
                if frame.rows.is_empty() {
                    ui.label("Step or run to begin this settlement's history.");
                }
            });
        self.notebook_list(ui, frame);
    }
    fn inspector(&mut self, ui: &mut egui::Ui, frame: &Frame) {
        if frame.perspective != self.perspective {
            return;
        }
        ui.heading("Individual");
        let character = match &frame.view {
            View::Observer(s) => s.characters.iter().find(|a| a.id == self.follow),
            View::Character(v) => Some(&v.own),
        };
        if let Some(a) = character {
            ui.label(RichText::new(&a.name).size(20.0).color(GOLD));
            ui.label(format!("Current state · tick {}", frame.tick));
            ui.label(format!(
                "{} · {} food · reserve target {}",
                place(frame, a.place),
                a.food,
                a.reserve_goal
            ));
            ui.add(
                egui::ProgressBar::new(a.hunger as f32 / 100.0)
                    .text(format!("Hunger {} / 100", a.hunger))
                    .fill(if a.hunger > 70 {
                        Color32::from_rgb(176, 104, 82)
                    } else {
                        GREEN
                    }),
            );
            ui.label(RichText::new(activity(frame, a.id)).color(GREEN));
            egui::CollapsingHeader::new("Disposition and retained knowledge").show(ui, |ui| {
                ui.label(format!(
                    "Generosity {} · caution {}",
                    a.generosity, a.caution
                ));
                for belief in &a.beliefs {
                    ui.label(format!(
                        "{}: saw {} food at tick {}",
                        place(frame, belief.place),
                        belief.stock,
                        belief.observed_at
                    ));
                }
                ui.label(RichText::new("Directed trust").strong());
                for relation in &a.relationships {
                    ui.label(format!(
                        "{}: {:+}",
                        person(frame, relation.other),
                        relation.trust
                    ));
                }
                if a.relationships.is_empty() {
                    ui.label("No relationship experience yet.");
                }
            });
            egui::CollapsingHeader::new("Retained experiences").show(ui, |ui| {
                for memory in a.memories.iter().rev().take(8) {
                    if ui
                        .small_button(format!(
                            "t{} · {}",
                            memory.tick,
                            memory.kind.replace('_', " ")
                        ))
                        .clicked()
                    {
                        self.select(memory.receipt);
                    }
                }
            });
        }
        ui.separator();
        ui.heading("Decision & consequence");
        if let Some(error) = &frame.detail_error {
            ui.colored_label(GOLD, error);
        }
        let Some(detail) = &frame.detail else {
            ui.label(
                RichText::new("Select history or trace a commitment to follow a causal chain.")
                    .color(MUTED),
            );
            return;
        };
        let record = &detail.record;
        if let Some(actor) = record.actor {
            ui.label(
                RichText::new(format!("Recorded person: {}", person(frame, actor))).color(GOLD),
            );
        }
        ui.label(format!(
            "Recorded tick {} · record {}",
            record.tick, record.id
        ));
        ui.label(RichText::new(projection::row(record).title).strong());
        self.note_editor(ui, frame);
        match &record.kind {
            RecordKind::Decision { view, choice, .. } => {
                ui.label(
                    RichText::new("Captured local input · separate from current memory")
                        .small()
                        .color(GOLD),
                );
                ui.label(format!(
                    "{} food · hunger {} · {} retained episodes",
                    view.own.food,
                    view.own.hunger,
                    view.own.memories.len()
                ));
                for c in &choice.candidates {
                    egui::CollapsingHeader::new(format!(
                        "{}  {}  ({})",
                        if c.action == choice.selected {
                            "Selected:"
                        } else {
                            "Alternative:"
                        },
                        projection::action_name(&c.action),
                        c.score
                    ))
                    .id_salt(("candidate", record.id, format!("{:?}", c.action)))
                    .show(ui, |ui| {
                        for (term, score) in &c.terms {
                            ui.label(format!("{}: {score:+}", term.replace('_', " ")));
                        }
                    });
                }
            }
            RecordKind::Started { commitment } => {
                ui.label(format!(
                    "{} · tick {} to {}",
                    projection::work_name(&commitment.work),
                    commitment.start,
                    commitment.due
                ));
                ui.label(
                    commitment
                        .participants
                        .iter()
                        .map(|id| person(frame, *id))
                        .collect::<Vec<_>>()
                        .join(" + "),
                );
            }
            RecordKind::Finished { outcome, units, .. } => {
                ui.label(format!(
                    "Actual outcome: {} · {units} food",
                    outcome.replace('_', " ")
                ));
            }
            RecordKind::Observation { belief } => {
                ui.label(format!(
                    "{}: {} food observed at tick {}",
                    place(frame, belief.place),
                    belief.stock,
                    belief.observed_at
                ));
            }
            RecordKind::Pressure { before, after } => {
                ui.label(format!("Own hunger: {before} → {after}"));
            }
            RecordKind::Refused { reason, .. } | RecordKind::Rejected { reason, .. } => {
                ui.label(reason.replace('_', " "));
            }
        }
        ui.add_space(8.0);
        ui.label(RichText::new("Related causes and consequences").strong());
        for link in &detail.links {
            if ui
                .small_button(format!("t{} · {}", link.tick, link.title))
                .clicked()
            {
                self.select(link.id);
            }
        }
        if matches!(frame.perspective, Perspective::Character(_)) {
            ui.label(RichText::new("Only captured local records are shown. Foreign decision inputs require Observer perspective.").small().color(MUTED));
        }
    }
    fn dialogs(&mut self, ctx: &egui::Context) {
        let Some(dialog) = self.dialog.take() else {
            return;
        };
        let mut keep = true;
        egui::Modal::new(egui::Id::new("settlement-dialog")).show(ctx, |ui| {
            ui.set_width(560.0);
            match &dialog {
                Dialog::New => {
                    ui.heading("New settlement"); ui.label("A new group, finite food, no scripted encounters.");
                    ui.horizontal(|ui| { ui.label("Seed"); ui.text_edit_singleline(&mut self.seed); });
                    ui.horizontal(|ui| {
                        ui.label("People");
                        ui.selectable_value(&mut self.population, 8, "8 people"); ui.selectable_value(&mut self.population, 12, "12 people");
                    });
                    ui.horizontal(|ui| { ui.label("Finite horizon"); egui::ComboBox::from_id_salt("horizon").selected_text(format!("{} ticks", self.horizon)).show_ui(ui, |ui| {
                        for ticks in [64, 256, 512, 1024] { ui.selectable_value(&mut self.horizon, ticks, format!("{ticks} ticks")); }
                    }); });
                    if ui.button("Create settlement").clicked() {
                        match self.seed.parse() {
                            Ok(seed) => { keep = false; self.request(Intent::New(Config { seed, population: self.population, horizon: self.horizon }), ctx); }
                            Err(_) => { self.status = "Seed must be an unsigned integer".into(); self.error = true; }
                        }
                    }
                }
                Dialog::Open => {
                    ui.heading("Open checkpoint"); ui.label("Opens complete saved state, paused at its actual boundary.");
                    ui.add(egui::TextEdit::singleline(&mut self.file).desired_width(540.0).hint_text("Checkpoint .json file"));
                    if ui.add_enabled(!self.file.trim().is_empty(), egui::Button::new("Open checkpoint")).clicked() {
                        keep = false; self.request(Intent::Open(PathBuf::from(self.file.trim())), ctx);
                    }
                }
                Dialog::Save(after) => {
                    ui.heading("Save checkpoint"); ui.label("Time is paused. Choose a fresh filename; existing saves stay intact.");
                    ui.add(egui::TextEdit::singleline(&mut self.file).desired_width(540.0));
                    if ui.add_enabled(!self.file.trim().is_empty(), egui::Button::new("Save now")).clicked()
                        && let Some(id) = self.send(Command::Save(PathBuf::from(self.file.trim()))) {
                        self.after_save.insert(id, after.clone()); keep = false;
                    }
                }
                Dialog::Unsaved(intent) => {
                    ui.heading("Keep this settlement's progress?");
                    ui.label(if self.notebook.dirty() { "An external observer note draft is unsaved. Save it first; world progress is handled separately." } else { "Time is paused. Save a checkpoint before leaving, or explicitly discard unsaved progress." });
                    ui.horizontal(|ui| {
                        if ui.add_enabled(self.after_note.is_empty(), egui::Button::new("Save first")).clicked() {
                            keep = false;
                            if self.notebook.dirty() { self.set_perspective(Perspective::Observer); self.save_note(Some(intent.clone())); }
                            else { self.request_save(Some(intent.clone())); }
                        }
                        if ui.button("Discard unsaved progress").clicked() { keep = false; self.perform(intent.clone(), ctx); }
                    });
                }
            }
            if self.error { ui.colored_label(Color32::LIGHT_RED, &self.status); }
            if ui.button("Cancel").clicked() { keep = false; }
        });
        if keep && self.dialog.is_none() {
            self.dialog = Some(dialog);
        }
    }
}
impl eframe::App for ObserverApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll(ctx);
        if ctx.input(|i| i.viewport().close_requested()) && !self.closing {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            if self.dialog.is_none() {
                self.request(Intent::Close, ctx);
            }
        }
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            self.header(ui);
        });
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.colored_label(
                    if self.error {
                        Color32::LIGHT_RED
                    } else {
                        MUTED
                    },
                    &self.status,
                );
                if let Some(frame) = &self.frame
                    && let Some(path) = &frame.last_save
                {
                    ui.label(
                        RichText::new(format!(
                            "Last checkpoint: {}",
                            PathBuf::from(path)
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                        ))
                        .small(),
                    );
                }
            });
        });
        let view_revision = self.view_revision;
        if let Some(frame) = self.frame.take() {
            egui::SidePanel::left("people")
                .exact_width(240.0)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| self.people(ui, &frame));
                });
            egui::SidePanel::right("inspector")
                .exact_width(345.0)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| self.inspector(ui, &frame));
                });
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.map(ui, &frame);
                    self.history(ui, &frame);
                });
            });
            // A perspective switch deliberately cleared the old frame. Never put
            // an omniscient frame back after the UI has selected a local view.
            if frame.perspective == self.perspective && self.view_revision == view_revision {
                self.frame = Some(frame);
            }
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.spinner();
                ui.label("Preparing settlement view…");
            });
        }
        self.dialogs(ctx);
        ctx.request_repaint_after(Duration::from_millis(40));
    }
}
fn person(frame: &Frame, id: CharacterId) -> String {
    frame
        .roster
        .iter()
        .find(|p| p.id == id)
        .map(|p| p.name.clone())
        .unwrap_or_else(|| format!("Person {}", id.0))
}
fn place(frame: &Frame, id: PlaceId) -> String {
    frame
        .places
        .iter()
        .find(|p| p.id == id)
        .map(|p| p.name.clone())
        .unwrap_or_else(|| format!("Place {id}"))
}
fn activity(frame: &Frame, id: CharacterId) -> String {
    if let Some(job) = frame
        .activities
        .iter()
        .find(|c| c.participants.contains(&id))
    {
        return match job.work {
            Work::Travel { to } => format!("Travelling to {}", place(frame, to)),
            Work::Send { to, .. } => format!("Inviting {}", person(frame, to)),
            _ => projection::work_name(&job.work).into(),
        };
    }
    if let View::Character(view) = &frame.view
        && view.people.iter().any(|p| p.id == id && !p.available)
    {
        return "Occupied · activity private".into();
    }
    "Available / waiting".into()
}
