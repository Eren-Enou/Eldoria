//! The desktop is a consumer of the same typed backend as the CLI.
use crate::{
    Result,
    config::{Configuration, Mode, SCENARIOS},
    execution::{self, Metadata},
    history::Summary,
    observations::{self, Category, Observation},
    storage::Store,
    viewer::{self, Viewer},
};
use eframe::egui::{self, Color32, RichText};
use std::sync::mpsc::{self, Receiver, TryRecvError};

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Run,
    Results,
    History,
    About,
}
enum Event {
    Execution(Result<Box<Metadata>>),
    Viewer(Result<Viewer>),
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
    ids: Vec<String>,
    selected: Option<Metadata>,
    summary: Option<Summary>,
    worker: Option<Receiver<Event>>,
    viewer: Option<Viewer>,
    message: String,
    error: bool,
    note: Observation,
    source_text: String,
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
                "Revision {}\nFrozen023 runner fingerprint {}\nWorking tree dirty: {}",
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
            ids: Vec::new(),
            selected: None,
            summary: None,
            worker: None,
            viewer: None,
            message: "Ready. New exploratory runs never overwrite an existing history.".into(),
            error: false,
            note: Observation::default(),
            source_text,
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
            mode: Mode::ContinuousWorld,
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
        Ok(c)
    }
    fn execute(&mut self, config: Configuration, original: Option<String>, ctx: &egui::Context) {
        let (tx, rx) = mpsc::channel();
        let store = self.store.clone();
        let repaint = ctx.clone();
        self.worker = Some(rx);
        self.report("Running existing Rust simulator. A first host build can take several minutes; results and logs are saved locally.",false);
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
    fn poll(&mut self) {
        let result = self.worker.as_ref().map(|rx| rx.try_recv());
        match result {
            Some(Ok(event)) => {
                self.worker = None;
                match event {
                    Event::Execution(Ok(m)) => {
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
                    Event::Execution(Err(e)) | Event::Viewer(Err(e)) => self.report(e, true),
                    Event::Viewer(Ok(v)) => {
                        self.report(format!("Read-only Explorer ready at {}. Select the custom023 run in its catalog.",v.url),false);
                        self.viewer = Some(v);
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
        match execution::metadata(&self.store, id) {
            Ok(m) => {
                self.selected = Some(m);
                match execution::inspect(&self.store, id) {
                    Ok(s) => {
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
        ui.label("Real023 Continuous scheduling. Exploratory configuration only.");
        egui::Grid::new("configuration")
            .spacing([20.0, 16.0])
            .show(ui, |ui| {
                ui.label("Simulation mode");
                egui::ComboBox::from_id_salt("mode")
                    .selected_text("Continuous World023")
                    .show_ui(ui, |ui| {
                        ui.label("Continuous World023 — supported");
                        ui.label("024 Activity valuation — unavailable in V1");
                        ui.label("025 Preparation — research-only, unavailable in V1");
                    });
                ui.end_row();
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
                "No source edits, policy tuning, production, new experiment or canon changes.",
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
                    "{} · ContinuousWorld · {} · seed{}",
                    m.status, m.configuration.scenario, m.configuration.seed
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
                    "{} | {} | {} | seed{}",
                    id, m.status, m.configuration.scenario, m.configuration.seed
                ),
                Err(_) => format!("{id} | Incomplete — missing/corrupt metadata"),
            };
            if ui
                .selectable_label(self.selected.as_ref().is_some_and(|m| m.id == id), label)
                .clicked()
            {
                match execution::metadata(&self.store, &id) {
                    Ok(m) => {
                        self.selected = Some(m);
                        self.summary = None;
                    }
                    Err(e) => self.report(e, true),
                }
            }
        }
    }
    fn history_page(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let Some(m) = self.selected.clone() else {
            ui.heading("Select a completed run in Results, then Inspect");
            return;
        };
        let Some(summary) = self.summary.clone() else {
            ui.heading("No inspected history");
            if ui.button("Inspect selected run").clicked() {
                self.inspect(&m.id);
            }
            return;
        };
        ui.heading("Observer Perspective");
        ui.label(&m.id);
        ui.label("Recorded outcomes are not the individual's knowledge or motives. Counts include recorded bootstrap actions.");
        ui.group(|ui|{
            ui.label(format!("4 individuals · ticks{}–{} · ordinary cutoff{} · final active/pending:0",summary.start,summary.end,summary.nominal_stop));
            ui.label(format!("{} completed activities · {} meals · {} resource events · {} transferred food units",summary.completed_activities,summary.meals,summary.resource_events,summary.transferred_units));
            ui.label(format!("{} successful environmental grants · final cache food{} · food ledger matches:{}",summary.environmental_grants,summary.final_stock,summary.food_accounting_matches));
            ui.horizontal(|ui|{for a in &summary.final_agents {ui.label(format!("Person{}: food{} / hunger{}",a.id,a.food,a.hunger));}});
        });
        ui.horizontal(|ui|{
            if ui.add_enabled(self.worker.is_none(), egui::Button::new("Start local History Explorer")).clicked() {
                let store = self.store.clone();
                let id = m.id.clone();
                let repaint = ctx.clone();
                let (tx, rx) = mpsc::channel();
                self.worker = Some(rx);
                self.report("Starting optional Python viewer on loopback. Its first catalog can take up to3 minutes; source remains read-only.", false);
                std::thread::spawn(move || {
                    let _ = tx.send(Event::Viewer(viewer::launch(&store, &id)));
                    repaint.request_repaint();
                });
            }
            if ui.button("Copy viewer command").clicked(){match viewer::command_text(&self.store,&m.id,8766){Ok(s)=>{ctx.copy_text(s);self.report("PowerShell viewer command copied. Python3.10+ required; opens no browser automatically.",false);},Err(e)=>self.report(e,true)}}
        });
        if let Some(viewer) = &self.viewer {
            ui.hyperlink_to("Open local read-only Explorer", &viewer.url);
            ui.small("Choose the custom023 run in its catalog; this server stops when the workbench closes.");
        }
        egui::CollapsingHeader::new("Captured individual decision progression — bounded sample").default_open(true).show(ui,|ui|{
            ui.small("Up to32 choices from the first32 physical boundaries. Actor-local food/hunger and actual candidate scores only; no inferred thoughts. Full history is available in Explorer.");
            for (index,c) in summary.decisions.iter().enumerate(){
                ui.push_id(index,|ui|{ui.horizontal(|ui|{ui.monospace(format!("tick{:3} | person{} | food{} hunger{} | {}",c.input.legacy.now,c.input.legacy.own.id,c.input.legacy.own.food,c.input.legacy.own.hunger,c.selected));if ui.small_button("Use for observation").clicked(){self.note.individual=c.input.legacy.own.id;self.note.tick=c.input.legacy.now;self.note.local_context=format!("Captured own food{}, hunger{}; decision sample{}",c.input.legacy.own.food,c.input.legacy.own.hunger,index);self.note.selected_activity=c.selected.clone();self.report("Actual sampled decision linked to the observation form below",false);}});ui.small(c.candidates.iter().map(|a|format!("{}: score{}, eligible{}",a.action,a.score,a.eligible)).collect::<Vec<_>>().join(" · "));});
            }
        });
        egui::CollapsingHeader::new("Executed activity records — first32 transitions").show(
            ui,
            |ui| {
                for t in &summary.transitions {
                    ui.monospace(format!(
                        "tick{} | commitment{} | {} | {:?} | {}",
                        t.tick,
                        t.commitment.id,
                        t.commitment.work,
                        t.commitment.participants,
                        if t.completed { "Completed" } else { "Started" }
                    ));
                }
            },
        );
        egui::CollapsingHeader::new("Human observation").default_open(true).show(ui,|ui|{
            ui.small("External human interpretation. Saved separately; never feeds actor cognition or behavior.");
            ui.horizontal(|ui|{ui.label("Individual");ui.add(egui::DragValue::new(&mut self.note.individual).range(0..=3));ui.label("World tick");ui.add(egui::DragValue::new(&mut self.note.tick).range(summary.start..=summary.end));egui::ComboBox::from_id_salt("category").selected_text(format!("{:?}",self.note.category)).show_ui(ui,|ui|{for c in [Category::Plausible,Category::Questionable,Category::Unclear,Category::PossibleBug]{ui.selectable_value(&mut self.note.category,c,format!("{c:?}"));}});});
            for (label,text) in [("Observed local context",&mut self.note.local_context),("Selected activity",&mut self.note.selected_activity),("Recorded outcome",&mut self.note.recorded_outcome)] {ui.label(label);ui.text_edit_singleline(text);}
            ui.label("Human notes");ui.add(egui::TextEdit::multiline(&mut self.note.notes).desired_rows(3).desired_width(f32::INFINITY));
            if ui.button("Save human observation").clicked(){match observations::save(&self.store,&m.id,&self.note){Ok(p)=>self.report(format!("Human observation saved: {}",p.display()),false),Err(e)=>self.report(e,true)}}
        });
    }
}
impl eframe::App for Workbench {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.poll();
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("World of Individuals");
                ui.label(RichText::new("RUST WORKBENCH V1").color(Color32::LIGHT_BLUE));
            });
            ui.horizontal(|ui| {
                for (page, label) in [
                    (Page::Run, "Run"),
                    (Page::Results, "Results"),
                    (Page::History, "History"),
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
            Page::Run=>self.run_page(ui,ctx),Page::Results=>self.results_page(ui,ctx),Page::History=>self.history_page(ui,ctx),Page::About=>{
                ui.heading("A human research workbench");ui.label("The independent Rust simulator produces the history. This application configures, executes, reproduces and reads it.");ui.monospace(&self.source_text);
                ui.label("Protected foundation:001–024. Experiment025: reviewed research, unadopted/unprotected; integrity candidate remains separate. No026.");
                ui.label("Rust/Cargo1.95+ and Git. Existing host generation and optional read-only Explorer require Python3.10+. Native summaries need no viewing server.");
                ui.label("024 Activity valuation: unavailable in V1.025 Preparation: unavailable research mode. No simulation policy, capacity or resource controls were added.");
                ui.label("Support: read docs/development/HUMAN_WORKBENCH_QUICKSTART.md. Keep the checkout in its build location; rebuild after moving it.");
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
        "World of Individuals — Rust Workbench V1",
        options,
        Box::new(move |cc| Ok(Box::new(Workbench::new(store, cc)))),
    )
    .map_err(|e| format!("Native window could not start: {e}. The CLI remains available."))
}
