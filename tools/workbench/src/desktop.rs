//! The desktop is a consumer of the same typed backend as the CLI.
use crate::{
    Result,
    config::{Configuration, Effort, Mode, Preparation, PreparationPreset, SCENARIOS, Valuation},
    execution::{self, Metadata},
    history::Summary,
    observations::{self, Category, Observation},
    storage::Store,
    viewer::{self, Viewer},
};
use eframe::egui::{self, Color32, RichText};
use std::sync::mpsc::{self, Receiver, TryRecvError};
#[path = "desktop_history.rs"]
mod history_ui;

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Run,
    Results,
    History,
    About,
    Compare,
}
enum Event {
    Execution(Result<Box<Metadata>>),
    Viewer(String, Result<Viewer>),
    Detail(String, usize, u32, Result<Box<crate::recorded::Detail>>),
}
fn completion_time(ms: Option<u128>) -> String {
    ms.and_then(|n| i64::try_from(n).ok())
        .and_then(chrono::DateTime::<chrono::Utc>::from_timestamp_millis)
        .map(|d| d.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
        .unwrap_or_else(|| "Not finished".into())
}
pub struct Workbench {
    store: Store,
    page: Page,
    scenario: String,
    seed: String,
    ticks: String,
    period: String,
    mode: Mode,
    valuation: Valuation,
    preparation: Preparation,
    actor: u32,
    compare_id: String,
    comparison: Option<crate::comparison::Comparison>,
    ids: Vec<String>,
    selected: Option<Metadata>,
    summary: Option<Summary>,
    worker: Option<Receiver<Event>>,
    viewer: Option<Viewer>,
    message: String,
    error: bool,
    note: Observation,
    source_text: String,
    moments: Vec<crate::recorded::Moment>,
    moment: usize,
    detail: Option<crate::recorded::Detail>,
    detail_attempt: Option<(usize, u32)>,
    saved_notes: Vec<observations::Saved>,
}
impl Workbench {
    pub fn new(store: Store, cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::ThemePreference::Dark);
        cc.egui_ctx.style_mut(|s| {
            s.spacing.item_spacing = egui::vec2(10.0, 9.0);
            s.spacing.button_padding = egui::vec2(12.0, 7.0);
        });
        let source_text = match execution::source(&store) {
            Ok(s) => format!(
                "Revision {}\nVerified engine fingerprint {}\nWorking tree dirty: {}",
                s.revision, s.runner_fingerprint, s.dirty
            ),
            Err(e) => format!("Source identity unavailable: {e}"),
        };
        let mut app = Self {
            store,
            page: Page::Run,
            scenario: "paired".into(),
            seed: "42".into(),
            ticks: "192".into(),
            period: "11".into(),
            mode: Mode::ContinuousWorld,
            valuation: Valuation::Baseline,
            preparation: Preparation::default(),
            actor: 0,
            compare_id: String::new(),
            comparison: None,
            ids: Vec::new(),
            selected: None,
            summary: None,
            worker: None,
            viewer: None,
            message: "Ready. New exploratory runs never overwrite an existing history.".into(),
            error: false,
            note: Observation::default(),
            source_text,
            moments: Vec::new(),
            moment: 0,
            detail: None,
            detail_attempt: None,
            saved_notes: Vec::new(),
        };
        app.refresh();
        app
    }
    fn report(&mut self, text: impl Into<String>, error: bool) {
        self.message = text.into();
        self.error = error;
    }
    fn refresh(&mut self) {
        match self.store.list() {
            Ok(ids) => self.ids = ids,
            Err(e) => self.report(e, true),
        }
    }
    fn config(&self) -> Result<Configuration> {
        let c = Configuration {
            schema_version: if self.mode == Mode::ContinuousWorld {
                1
            } else {
                2
            },
            mode: self.mode,
            valuation: (self.mode != Mode::ContinuousWorld).then_some(self.valuation),
            preparation: (self.mode == Mode::MaterialPreparation)
                .then_some(self.preparation.clone()),
            scenario: self.scenario.clone(),
            seed: self
                .seed
                .parse()
                .map_err(|_| "Seed must be an unsigned64-bit integer")?,
            ticks: self
                .ticks
                .parse()
                .map_err(|_| "Duration must be an integer")?,
            period: self
                .period
                .parse()
                .map_err(|_| "Reporting period must be an integer")?,
        };
        c.validate()?;
        self.store.supports(c.mode)?;
        Ok(c)
    }
    fn execute(&mut self, config: Configuration, original: Option<String>, ctx: &egui::Context) {
        let (tx, rx) = mpsc::channel();
        let store = self.store.clone();
        let repaint = ctx.clone();
        self.worker = Some(rx);
        self.report(if self.store.installed.is_some(){"Running the packaged Rust simulator. Results and logs are saved in your user-data workspace."}else{"Running existing Rust simulator. A first host build can take several minutes; results and logs are saved locally."},false);
        std::thread::spawn(move || {
            let result = if let Some(id) = original {
                execution::reproduce(&store, &id)
            } else {
                execution::run(&store, config, None)
            };
            let _ = tx.send(Event::Execution(result.map(Box::new)));
            repaint.request_repaint();
        });
    }
    fn poll(&mut self, ctx: &egui::Context) {
        let result = self.worker.as_ref().map(|rx| rx.try_recv());
        match result {
            Some(Ok(event)) => {
                self.worker = None;
                match event {
                    Event::Detail(id, index, actor, result) => {
                        if self.selected.as_ref().is_some_and(|m| m.id == id)
                            && self.moment == index
                            && self.actor == actor
                        {
                            match result {
                                Ok(d) => self.detail = Some(*d),
                                Err(e) => self.report(e, true),
                            }
                        }
                    }
                    Event::Execution(Ok(m)) => {
                        self.viewer = None;
                        self.comparison = None;
                        self.report(
                            format!(
                                "{}: {}. Reproduction hash match: {:?}. {}",
                                m.status,
                                m.id,
                                m.exact_match,
                                m.diagnostic.as_deref().unwrap_or(
                                    "Execution completion is not scientific acceptance."
                                )
                            ),
                            m.status != "Completed",
                        );
                        self.selected = Some(*m);
                        self.summary = None;
                        self.page = Page::Results;
                        self.refresh();
                    }
                    Event::Execution(Err(e)) => self.report(e, true),
                    Event::Viewer(id, result) => {
                        if self.selected.as_ref().is_some_and(|m| m.id == id) {
                            match result {
                                Ok(v) => {
                                    ctx.open_url(egui::OpenUrl::new_tab(&v.url));
                                    self.report(format!("Opening the verified selected history at {}. Use the link below if your browser did not open.",v.url),false);
                                    self.viewer = Some(v);
                                }
                                Err(e) => self.report(e, true),
                            }
                        } else {
                            self.report("Selection changed during viewer startup; the earlier viewer was closed. Inspect the current run to open its history.",false);
                        }
                    }
                }
            }
            Some(Err(TryRecvError::Disconnected)) => {
                self.worker = None;
                self.refresh();
                self.report("Worker stopped unexpectedly. Saved initial metadata remains Incomplete; inspect run diagnostics.",true);
            }
            _ => {}
        }
    }
    fn inspect(&mut self, id: &str) {
        self.viewer = None;
        self.comparison = None;
        match execution::metadata(&self.store, id) {
            Ok(m) => {
                self.selected = Some(m);
                match execution::inspect(&self.store, id) {
                    Ok(s) => {
                        self.moments.clear();
                        self.detail = None;
                        self.detail_attempt = None;
                        self.moment = 0;
                        self.saved_notes.clear();
                        match self
                            .store
                            .run(id)
                            .and_then(|p| crate::recorded::moments(&p.join("history.jsonl")))
                        {
                            Ok(rows) => self.moments = rows,
                            Err(e) => self.report(e, true),
                        }
                        match observations::load(&self.store, id) {
                            Ok(notes) => self.saved_notes = notes,
                            Err(e) => self.report(e, true),
                        }
                        self.note = Observation {
                            tick: s.start,
                            ..Default::default()
                        };
                        self.summary = Some(s);
                        self.page = Page::History;
                    }
                    Err(e) => {
                        self.summary = None;
                        self.report(e, true);
                    }
                }
            }
            Err(e) => self.report(e, true),
        }
    }
    fn run_page(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Configure a four-person world");
        ui.label("Existing four-person Rust runners. Exploratory configuration only.");
        egui::Grid::new("configuration")
            .spacing([20.0, 16.0])
            .show(ui, |ui| {
                ui.label("Simulation mode");
                egui::ComboBox::from_id_salt("mode")
                    .selected_text(self.mode.label())
                    .show_ui(ui, |ui| {
                        for (mode, label) in [
                            (Mode::ContinuousWorld, "023 Continuous World"),
                            (Mode::ActivityValuation, "024 Activity valuation"),
                            (
                                Mode::MaterialPreparation,
                                "025 Corrected preparation — research",
                            ),
                        ] {
                            if self.store.supports(mode).is_ok() {
                                ui.selectable_value(&mut self.mode, mode, label);
                            } else {
                                ui.add_enabled(
                                    false,
                                    egui::Label::new(format!(
                                        "{label} — unavailable in portable release"
                                    )),
                                );
                            }
                        }
                    });
                ui.end_row();
                if self.mode != Mode::ContinuousWorld {
                    ui.label("Activity valuation");
                    egui::ComboBox::from_id_salt("valuation")
                        .selected_text(format!("{:?}", self.valuation))
                        .show_ui(ui, |ui| {
                            for v in [Valuation::Baseline, Valuation::MealRelief] {
                                ui.selectable_value(&mut self.valuation, v, format!("{v:?}"));
                            }
                        });
                    ui.end_row();
                }
                if self.mode == Mode::MaterialPreparation {
                    ui.label("Material observation preset");
                    egui::ComboBox::from_id_salt("preparation")
                        .selected_text(format!("{:?}", self.preparation.preset))
                        .show_ui(ui, |ui| {
                            for p in [
                                PreparationPreset::KnownRaw,
                                PreparationPreset::UnobservedRaw,
                                PreparationPreset::NoAccess,
                            ] {
                                ui.selectable_value(
                                    &mut self.preparation.preset,
                                    p,
                                    format!("{p:?}"),
                                );
                            }
                        });
                    ui.end_row();
                    ui.label("Initial raw units (0–16)");
                    ui.add(egui::DragValue::new(&mut self.preparation.raw_initial).range(0..=16));
                    ui.end_row();
                    ui.label("Extraction + preparation ticks");
                    egui::ComboBox::from_id_salt("effort")
                        .selected_text(format!("{:?}", self.preparation.effort))
                        .show_ui(ui, |ui| {
                            for (e, label) in [
                                (Effort::Short, "Short:2+4"),
                                (Effort::Medium, "Medium:4+8"),
                                (Effort::Long, "Long:8+24"),
                            ] {
                                ui.selectable_value(&mut self.preparation.effort, e, label);
                            }
                        });
                    ui.end_row();
                }
                ui.label("Scenario");
                egui::ComboBox::from_id_salt("scenario")
                    .selected_text(&self.scenario)
                    .show_ui(ui, |ui| {
                        for scenario in SCENARIOS {
                            ui.selectable_value(&mut self.scenario, scenario.to_owned(), scenario);
                        }
                    });
                ui.end_row();
                ui.label("Deterministic seed");
                ui.add(egui::TextEdit::singleline(&mut self.seed).desired_width(220.0))
                    .labelled_by(ui.label("Unsigned64-bit integer").id);
                ui.end_row();
                ui.label("Ordinary duration");
                ui.add(egui::TextEdit::singleline(&mut self.ticks).desired_width(220.0))
                    .labelled_by(ui.label("1–1024 ticks, followed by33-tick drain").id);
                ui.end_row();
                ui.label("Reporting period");
                ui.add(egui::TextEdit::singleline(&mut self.period).desired_width(220.0))
                    .labelled_by(ui.label("1–128 ticks; observer annotations").id);
                ui.end_row();
            });
        ui.add_space(12.0);
        if self.mode != Mode::ContinuousWorld {
            ui.label("Baseline and MealRelief are research policies, not globally adopted. MealRelief adds min(H,40) to eligible Consume's H score.");
        }
        if self.mode == Mode::MaterialPreparation {
            ui.label("025 is unadopted/unprotected research. Two raw → one edible food + one waste; failed attempts consume full effort. KnownRaw senses raw at phase0 and ready at1 of8. UnobservedRaw senses only ready. NoAccess permits observation but forbids preparation. Later meals and transfers cannot identify a manufactured unit.");
        }
        let config = self.config();
        if let Err(e) = &config {
            ui.colored_label(Color32::LIGHT_RED, e);
        } else {
            ui.colored_label(Color32::LIGHT_GREEN, "Configuration valid");
        }
        if ui
            .add_enabled(
                self.worker.is_none() && config.is_ok(),
                egui::Button::new("Run world").min_size(egui::vec2(180.0, 40.0)),
            )
            .clicked()
        {
            self.execute(config.unwrap(), None, ctx);
        }
        ui.add_space(15.0);
        ui.group(|ui| {
            ui.label("Fixed scientific boundary");
            ui.label(
                "Four individuals · claim4 · cadence8 · capacity2 · Epoch phase · Ascending order",
            );
            ui.label(
                "No policy tuning, new experiment or canon changes. Only existing025 research enables preparation.",
            );
        });
    }
    fn results_page(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.heading("Saved exploratory runs");
            if ui.button("Refresh runs").clicked() {
                self.refresh();
            }
        });
        if let Some(m) = self.selected.clone() {
            ui.group(|ui| {
                ui.strong(&m.id);
                ui.label(format!(
                    "{} · {} / {} · {} · seed{}",
                    m.status,
                    m.configuration.mode.label(),
                    m.configuration.policy_label(),
                    m.configuration.scenario,
                    m.configuration.seed
                ));
                ui.label(format!("Source {}", m.source.revision));
                ui.label(format!(
                    "Completed: {} · Process elapsed: {}",
                    completion_time(m.finished_unix_ms),
                    m.elapsed_ms
                        .map(|n| format!("{n}ms"))
                        .unwrap_or_else(|| "Not finished".into())
                ));
                if let Ok(path) = self.store.run(&m.id) {
                    ui.label(path.display().to_string());
                    if ui.button("Copy run path").clicked() {
                        ctx.copy_text(path.display().to_string());
                        self.report("Run path copied", false);
                    }
                }
                if let Some(e) = &m.diagnostic {
                    ui.colored_label(Color32::LIGHT_RED, e);
                }
                for warning in &m.warnings {
                    ui.small(warning);
                }
                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(m.status == "Completed", egui::Button::new("Inspect"))
                        .clicked()
                    {
                        self.inspect(&m.id);
                    }
                    if ui.button("Compare with another run").clicked() {
                        self.page = Page::Compare;
                        self.comparison = None;
                    }
                    if ui
                        .add_enabled(
                            self.worker.is_none(),
                            egui::Button::new("Reproduce in new directory"),
                        )
                        .clicked()
                    {
                        self.execute(m.configuration.clone(), Some(m.id.clone()), ctx);
                    }
                });
            });
        }
        ui.separator();
        if self.ids.is_empty() {
            ui.label("No runs yet. Configure a world in Run.");
        }
        for id in self.ids.clone() {
            let label = match execution::metadata(&self.store, &id) {
                Ok(m) => format!(
                    "{} | {} | {} / {} | {} | seed{}",
                    id,
                    m.status,
                    m.configuration.mode.experiment(),
                    m.configuration.policy_label(),
                    m.configuration.scenario,
                    m.configuration.seed
                ),
                Err(_) => format!("{id} | Incomplete — missing/corrupt metadata"),
            };
            if ui
                .selectable_label(self.selected.as_ref().is_some_and(|m| m.id == id), label)
                .clicked()
            {
                match execution::metadata(&self.store, &id) {
                    Ok(m) => {
                        self.viewer = None;
                        self.comparison = None;
                        self.selected = Some(m);
                        self.summary = None;
                    }
                    Err(e) => self.report(e, true),
                }
            }
        }
    }
}
impl eframe::App for Workbench {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.poll(ctx);
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("World of Individuals");
                ui.label(
                    RichText::new(if self.store.installed.is_some() {
                        "PORTABLE WORKBENCH 0.2"
                    } else {
                        "RUST WORKBENCH · DEVELOPMENT"
                    })
                    .color(Color32::LIGHT_BLUE),
                );
            });
            ui.horizontal(|ui| {
                for (page, label) in [
                    (Page::Run, "Run"),
                    (Page::Results, "Results"),
                    (Page::History, "History"),
                    (Page::Compare, "Compare"),
                    (Page::About, "About"),
                ] {
                    ui.selectable_value(&mut self.page, page, label);
                }
            });
        });
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            if self.worker.is_some() {
                ui.spinner();
                ctx.request_repaint_after(std::time::Duration::from_millis(200));
            }
            ui.colored_label(
                if self.error {
                    Color32::LIGHT_RED
                } else {
                    Color32::LIGHT_BLUE
                },
                &self.message,
            );
        });
        egui::CentralPanel::default().show(ctx,|ui|{egui::ScrollArea::vertical().show(ui,|ui|{match self.page{
            Page::Run=>self.run_page(ui,ctx),Page::Results=>self.results_page(ui,ctx),Page::History=>self.history_page(ui,ctx),Page::Compare=>self.compare_page(ui),Page::About=>{
                ui.heading("A human research workbench");ui.label("The independent Rust simulator produces the history. This application configures, executes, reproduces and reads it.");ui.monospace(&self.source_text);
                ui.label("Protected foundation:001–024. Experiment025: reviewed research, unadopted/unprotected; integrity candidate remains separate. No026.");
                if let Some(installed)=&self.store.installed{
                    ui.label(format!("Installed release {}. Normal operation needs no Cargo, Rust, Git, Python, checkout or network.025 and Python Explorer are developer-only.",installed.manifest.version));
                    ui.monospace(format!("User-data runs: {}",self.store.root.display()));
                    ui.label("Release checksums identify packaged bytes; they are not publisher signatures. See QUICKSTART.md beside the application.");
                }else{ui.label("Development mode needs Rust/Cargo1.95+ and Git. Existing host generation and optional read-only Explorer require Python3.10+. Native inspection needs no viewing server.");}
                ui.label("023 Continuous World;024 Baseline/MealRelief;025 corrected observer2 preparation.025 remains research, unadopted/unprotected. Strict versioned configuration. No policy, physics or cognitive capacity changes.");
                ui.label("The existing optional Python Explorer is opened through a narrow single-source bridge. Configuration, process execution, native reading, comparison and observations remain Rust.");
                if self.store.installed.is_none(){ui.label("Support: read docs/development/HUMAN_WORKBENCH_QUICKSTART.md. Keep the developer checkout in its build location; rebuild after moving it.");}
                ui.label("Closing during execution can leave an Incomplete record. Cancellation/resume and scientific acceptance are not provided.");
            }
        }});});
        if self.worker.is_some() && ctx.input(|i| i.viewport().close_requested()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.report("An operation is still running. Wait for its completion before closing; see run logs for build progress.",true);
        }
    }
}

pub fn launch(store: Store) -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1140.0, 780.0])
            .with_min_inner_size([850.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "World of Individuals — Research Workbench",
        options,
        Box::new(move |cc| Ok(Box::new(Workbench::new(store, cc)))),
    )
    .map_err(|e| format!("Native window could not start: {e}. The CLI remains available."))
}
