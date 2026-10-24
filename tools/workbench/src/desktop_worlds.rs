//! External execution controls; none of this state enters an individual's inputs.
use crate::{
    Result,
    config::{Configuration, Mode, SCENARIOS, Valuation},
    storage::Store,
    world_sessions::{Session, Sessions},
};
use eframe::egui::{self, Color32};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc::{self, Receiver, TryRecvError},
};

pub struct Worlds {
    session: Option<Session>,
    worker: Option<Receiver<(Session, Result<()>)>>,
    stop: Arc<AtomicBool>,
    scenario: String,
    seed: String,
    horizon: String,
    period: String,
    mode: Mode,
    valuation: Valuation,
    ids: Vec<String>,
    selected: String,
    initialized: bool,
    message: String,
    error: bool,
    moment: usize,
    actor: u32,
    journal_record: Option<(usize, String)>,
}
impl Default for Worlds {
    fn default() -> Self {
        Self {
            session: None,
            worker: None,
            stop: Arc::new(AtomicBool::new(false)),
            scenario: "paired".into(),
            seed: "42".into(),
            horizon: "1024".into(),
            period: "11".into(),
            mode: Mode::ContinuousWorld,
            valuation: Valuation::Baseline,
            ids: vec![],
            selected: String::new(),
            initialized: false,
            message: "Create an unfinished world or reopen a committed checkpoint.".into(),
            error: false,
            moment: 0,
            actor: 0,
            journal_record: None,
        }
    }
}
impl Worlds {
    fn report(&mut self, result: Result<()>, success: &str) {
        self.error = result.is_err();
        self.message = result.err().unwrap_or_else(|| success.into());
    }
    fn save(&mut self) -> Result<()> {
        if let Some(session) = &mut self.session {
            session.save()?;
        }
        Ok(())
    }
    fn refresh(&mut self, store: &Store) -> Result<()> {
        self.ids = Sessions::new(store)?.list()?;
        if self.selected.is_empty() {
            self.selected = self.ids.first().cloned().unwrap_or_default();
        }
        self.initialized = true;
        Ok(())
    }
    pub fn poll(&mut self, ctx: &egui::Context) {
        let Some(receiver) = &self.worker else {
            return;
        };
        match receiver.try_recv() {
            Ok((session, result)) => {
                let tick = session.world().tick();
                self.session = Some(session);
                self.worker = None;
                self.report(result,&format!("Paused at safe boundary, actual tick{tick}. Save checkpoint to commit this progress."));
            }
            Err(TryRecvError::Empty) => {
                ctx.request_repaint_after(std::time::Duration::from_millis(40))
            }
            Err(TryRecvError::Disconnected) => {
                self.worker = None;
                self.report(Err("Execution worker failed. Last committed files and any journal tail remain on disk; no restart from seed was attempted.".into()),"");
            }
        }
    }
    fn advance(&mut self, steps: usize) {
        let Some(mut session) = self.session.take() else {
            return;
        };
        self.stop.store(false, Ordering::Relaxed);
        let stop = self.stop.clone();
        let (sender, receiver) = mpsc::channel();
        self.worker = Some(receiver);
        self.message = format!(
            "Executing up to{steps} scheduler boundaries. Pause waits for the current atomic operation."
        );
        self.error = false;
        std::thread::spawn(move || {
            let result = (|| {
                for _ in 0..steps {
                    if stop.load(Ordering::Relaxed) || session.world().complete() {
                        break;
                    }
                    session.advance(1)?;
                }
                Ok(())
            })();
            let _ = sender.send((session, result));
        });
    }
    pub fn before_close(&mut self, ctx: &egui::Context) -> bool {
        if self.worker.is_some() {
            self.stop.store(true, Ordering::Relaxed);
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.message="Pause requested. Close again after the current atomic boundary; closing then saves the world.".into();
            return false;
        }
        match self.save() {
            Ok(()) => true,
            Err(error) => {
                self.report(
                    Err(format!(
                        "Closing blocked because checkpoint save failed: {error}"
                    )),
                    "",
                );
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                false
            }
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, store: &Store) {
        ui.heading("World Sessions");
        ui.label("Opt-in023/024 continuation research · four individuals · fixed horizon · finite food · no026");
        ui.label("Current state and completed history are distinct. Future events have not been executed. Closing never advances physical time.");
        if !self.initialized {
            let result = self.refresh(store);
            self.report(result, "Saved world list loaded.");
        }
        let busy = self.worker.is_some();
        ui.label("New world settings");
        ui.add_enabled_ui(!busy,|ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode,Mode::ContinuousWorld,"023 Continuous");
                ui.selectable_value(&mut self.mode,Mode::ActivityValuation,"024 Valuation research");
                if self.mode==Mode::ActivityValuation {ui.selectable_value(&mut self.valuation,Valuation::Baseline,"Baseline");ui.selectable_value(&mut self.valuation,Valuation::MealRelief,"MealRelief");}
            });
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_salt("session-scenario").selected_text(&self.scenario).show_ui(ui,|ui| {for scenario in SCENARIOS {ui.selectable_value(&mut self.scenario,scenario.to_string(),scenario);}});
                ui.label("Seed");ui.add(egui::TextEdit::singleline(&mut self.seed).desired_width(130.0));
                ui.label("Nominal ticks1–1024");ui.add(egui::TextEdit::singleline(&mut self.horizon).desired_width(65.0));
                ui.label("Report period");ui.add(egui::TextEdit::singleline(&mut self.period).desired_width(45.0));
            });
            if ui.button("Create world").clicked() {
                let result=(|| {self.save()?;let config=Configuration {schema_version:if self.mode==Mode::ActivityValuation {2}else{1},mode:self.mode,valuation:if self.mode==Mode::ActivityValuation {Some(self.valuation)}else{None},scenario:self.scenario.clone(),seed:self.seed.parse().map_err(|_|"invalid seed")?,ticks:self.horizon.parse().map_err(|_|"invalid horizon")?,period:self.period.parse().map_err(|_|"invalid period")?,..Default::default()};self.session=Some(Sessions::new(store)?.create(&config)?);self.selected=self.session.as_ref().unwrap().path().file_name().unwrap().to_string_lossy().to_string();self.moment=0;self.journal_record=None;self.refresh(store)})();
                self.report(result,"World created and initial checkpoint saved. Ordinary execution has not started.");
            }
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_salt("saved-world").width(330.0).selected_text(if self.selected.is_empty(){"No saved worlds"}else{&self.selected}).show_ui(ui,|ui| {for id in &self.ids {ui.selectable_value(&mut self.selected,id.clone(),id);}});
                if ui.button("Refresh worlds").clicked() {let result=self.refresh(store);self.report(result,"World list refreshed.");}
                let already_open=self.session.as_ref().is_some_and(|s|s.path().file_name().is_some_and(|id|id.to_string_lossy()==self.selected));
                if ui.add_enabled(!self.selected.is_empty()&&!already_open,egui::Button::new("Reopen world")).clicked() {
                    let result=(|| {self.save()?;let restored=Sessions::new(store)?.open(&self.selected)?;self.session=Some(restored);self.moment=0;self.journal_record=None;Ok(())})();
                    self.report(result,"Restored actual saved state. No bootstrap, prior history replay or wall-clock progression occurred.");
                }
            });
        });
        ui.label("Advance counts scheduler boundaries; an atomic inquiry can span multiple physical ticks.");
        ui.horizontal(|ui| {
            let can_advance = self.session.as_ref().is_some_and(|s| !s.world().complete());
            for (steps, label) in [
                (1, "Start / Advance 1"),
                (16, "Advance 16"),
                (64, "Advance 64"),
                (256, "Advance 256"),
            ] {
                if ui
                    .add_enabled(!busy && can_advance, egui::Button::new(label))
                    .clicked()
                {
                    self.advance(steps);
                    ctx.request_repaint();
                }
            }
            if ui.add_enabled(busy, egui::Button::new("Pause")).clicked() {
                self.stop.store(true, Ordering::Relaxed);
                self.message = "Pause requested; waiting for the current atomic operation.".into();
            }
            if ui
                .add_enabled(
                    !busy && self.session.is_some(),
                    egui::Button::new("Save checkpoint"),
                )
                .clicked()
            {
                let result = self.save();
                self.report(
                    result,
                    "Checkpoint committed and read back. Prior generation retained.",
                );
            }
            if ui.add_enabled(!busy&&self.session.is_some(),egui::Button::new("Close saved world")).clicked() {
                let result=self.save();if result.is_ok(){self.session=None;self.journal_record=None;}
                self.report(result,"World saved and closed. Its writer lock is released; no physical execution occurred.");
            }
            let terminal = self.session.as_ref().is_some_and(|s| s.world().complete());
            if ui
                .add_enabled(
                    !busy && terminal,
                    egui::Button::new("Finalize terminal world"),
                )
                .clicked()
            {
                let result = self.session.as_mut().unwrap().finalize();
                self.report(
                    result,
                    "Terminal export saved. No extra execution or settlement was introduced.",
                );
            }
        });
        ui.colored_label(
            if self.error {
                Color32::LIGHT_RED
            } else {
                Color32::LIGHT_BLUE
            },
            &self.message,
        );
        if busy {
            ui.spinner();
            ui.label("Execution owns the world while this bounded batch runs. Display updates after its next acknowledged pause.");
            return;
        }
        let Some(session) = &self.session else {
            return;
        };
        let world = session.world();
        let config = world.configuration();
        ui.label(format!(
            "Saved scenario {} · seed{} · fixed horizon{} · report period{}",
            config.case, config.seed, config.horizon, config.period
        ));
        ui.label(format!(
            "Actual saved mode: {}",
            world
                .valuation()
                .map(|m| format!("024 {m:?}"))
                .unwrap_or_else(|| "023 Continuous".into())
        ));
        ui.separator();
        ui.heading("Current world state · objective observer");
        ui.label(format!(
            "Tick{} · elapsed{} · nominal cutoff{} · terminal{} · {} · remaining cache{}",
            world.tick(),
            world.tick() - world.start(),
            world.start() + config.horizon,
            world.end(),
            if world.complete() {
                "terminal/settled"
            } else if world.tick() >= world.start() + config.horizon {
                "charged drain"
            } else {
                "paused/unfinished"
            },
            world.stock()
        ));
        ui.label(if session.dirty(){"Unsaved progress: journal advanced; save before reopening. Closing attempts a checkpoint save."}else{"Current state matches a committed checkpoint."});
        ui.monospace(session.path().display().to_string());
        let active = world.commitments();
        let pending = world.pending();
        egui::Grid::new("live-four-individuals")
            .striped(true)
            .show(ui, |ui| {
                for heading in [
                    "Individual",
                    "Food",
                    "Hunger",
                    "Current physical activity",
                    "Pending contact",
                ] {
                    ui.strong(heading);
                }
                ui.end_row();
                for agent in world.agents() {
                    ui.label(format!("Individual{}", agent.id));
                    ui.label(agent.food.to_string());
                    ui.label(agent.hunger.to_string());
                    ui.label(
                        active
                            .iter()
                            .find(|c| c.participants.contains(&agent.id))
                            .map(|c| format!("{:?} #{} until{}", c.work, c.id, c.deadline))
                            .unwrap_or_else(|| "Idle".into()),
                    );
                    ui.label(
                        pending
                            .iter()
                            .filter(|p| {
                                p.invitation.proposer == agent.id
                                    || p.invitation.recipient == agent.id
                            })
                            .map(|p| {
                                format!(
                                    "{}→{} expires{}",
                                    p.invitation.proposer,
                                    p.invitation.recipient,
                                    p.invitation.expires
                                )
                            })
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                    ui.end_row();
                }
            });
        ui.separator();
        ui.heading("Completed history · recorded actor-local perspective");
        let boundaries = world.boundaries();
        if boundaries.is_empty() {
            ui.label("No ordinary scheduler boundary has executed. The declared fixture bootstrap is preserved in the checkpoint/journal.");
            return;
        }
        self.moment = self.moment.min(boundaries.len() - 1);
        if ui
            .add(
                egui::Slider::new(&mut self.moment, 0..=boundaries.len() - 1)
                    .text("Recorded boundary"),
            )
            .changed()
        {
            self.journal_record = None;
        }
        ui.horizontal(|ui| {
            for actor in 0..4 {
                ui.selectable_value(&mut self.actor, actor, format!("Individual{actor}"));
            }
        });
        let boundary = &boundaries[self.moment];
        ui.label(format!("Recorded boundary{} at actual tick{}. Later observer records do not become this individual's knowledge.",self.moment,boundary.tick));
        if let Some(choice) = boundary
            .choices
            .iter()
            .find(|c| c.input.legacy.own.id == self.actor)
        {
            ui.label(format!("Selected {:?}; contact intention {:?}. Selection is distinct from admission and execution.",choice.selected,choice.contact));
            egui::CollapsingHeader::new("Captured local inputs and alternatives").show(ui, |ui| {
                ui.monospace(
                    serde_json::to_string_pretty(choice).unwrap_or_else(|e| e.to_string()),
                );
            });
        } else {
            ui.label("No ordinary choice was recorded for this individual at this boundary.");
        }
        egui::CollapsingHeader::new("Actual executed transitions linked to this boundary").show(
            ui,
            |ui| {
                for transition in world.transitions().iter().filter(|t| {
                    t.commitment.source == self.moment
                        && t.commitment.participants.contains(&self.actor)
                }) {
                    ui.label(format!(
                        "Tick{} · {:?} #{} · {}",
                        transition.tick,
                        transition.commitment.work,
                        transition.commitment.id,
                        if transition.completed {
                            "completed/released"
                        } else {
                            "admitted/reserved"
                        }
                    ));
                }
            },
        );
        if ui.button("Inspect completed step record").clicked() {
            self.journal_record = Some((
                self.moment,
                session
                    .record(self.moment)
                    .and_then(|r| serde_json::to_string_pretty(&r).map_err(|e| e.to_string()))
                    .unwrap_or_else(|e| e),
            ));
        }
        if let Some((moment, record)) = &self.journal_record {
            ui.label(format!("Observer execution record for boundary{moment}. This is temporal/causal audit, including actual responses, transfers and information delivery; it is not additional actor knowledge."));
            egui::CollapsingHeader::new("Observer step audit")
                .default_open(true)
                .show(ui, |ui| {
                    ui.monospace(record);
                });
        }
    }
}
