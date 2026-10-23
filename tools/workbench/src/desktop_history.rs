//! Observer-only pages sharing the desktop's typed backend and selection state.
use super::*;

fn number(value: Option<i64>) -> String {
    value
        .map(|n| n.to_string())
        .unwrap_or_else(|| "Not captured".into())
}

fn metrics(s: &Summary) -> Vec<(&'static str, String)> {
    vec![
        ("Experiment", s.experiment.clone()),
        ("Completed activities", s.completed_activities.to_string()),
        ("Meals (bootstrap included)", s.meals.to_string()),
        ("Resource events", s.resource_events.to_string()),
        (
            "Completed resource encounters",
            s.completed_resource_encounters.to_string(),
        ),
        ("Transferred edible units", s.transferred_units.to_string()),
        (
            "Successful environmental grants",
            s.environmental_grants.to_string(),
        ),
        ("Final cache food", s.final_stock.to_string()),
        (
            "Final id / food / hunger",
            format!(
                "{:?}",
                s.final_agents
                    .iter()
                    .map(|a| (a.id, a.food, a.hunger))
                    .collect::<Vec<_>>()
            ),
        ),
        (
            "Ledger basis",
            if s.material.is_some() {
                "025 common mass"
            } else {
                "Edible food"
            }
            .into(),
        ),
        ("Ledger balances", s.food_accounting_matches.to_string()),
        (
            "Prepared / failed conversions",
            s.material
                .as_ref()
                .map(|m| format!("{} / {}", m.prepared, m.failed))
                .unwrap_or("Unknown / not this mode".into()),
        ),
        (
            "Final raw / waste / escrow",
            s.material
                .as_ref()
                .map(|m| format!("{} / {} / {}", m.final_raw, m.final_waste, m.final_escrow))
                .unwrap_or("Unknown / not this mode".into()),
        ),
    ]
}
impl Workbench {
    pub(super) fn compare_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Compare completed exploratory histories");
        let Some(left) = self.selected.clone() else {
            ui.label("Select a run in Results first.");
            return;
        };
        ui.label(format!("Left: {}", left.id));
        let previous = self.compare_id.clone();
        egui::ComboBox::from_id_salt("compare-right")
            .width(700.0)
            .selected_text(if self.compare_id.is_empty() {
                "Choose a second run"
            } else {
                &self.compare_id
            })
            .show_ui(ui, |ui| {
                for id in &self.ids {
                    if let Ok(m) = execution::metadata(&self.store, id)
                        && m.status == "Completed"
                    {
                        ui.selectable_value(
                            &mut self.compare_id,
                            id.clone(),
                            format!(
                                "{} · {} / {} · {} · seed{}",
                                id,
                                m.configuration.mode.experiment(),
                                m.configuration.policy_label(),
                                m.configuration.scenario,
                                m.configuration.seed
                            ),
                        );
                    }
                }
            });
        if previous != self.compare_id {
            self.comparison = None;
        }
        if ui
            .add_enabled(
                !self.compare_id.is_empty(),
                egui::Button::new("Compare verified histories"),
            )
            .clicked()
        {
            match crate::comparison::compare(&self.store, &left.id, &self.compare_id) {
                Ok(c) => self.comparison = Some(c),
                Err(e) => self.report(e, true),
            }
        }
        let Some(c) = &self.comparison else {
            return;
        };
        ui.strong(&c.classification);
        ui.label(format!("Verified pair: {} ↔ {}", c.left_id, c.right_id));
        ui.label("Recorded difference ≠ registered causal conclusion. More meals or transfers is not a universal benefit. No adoption follows from an exploratory pair.");
        egui::CollapsingHeader::new("Actual configurations side by side")
            .default_open(true)
            .show(ui, |ui| {
                ui.columns(2, |cols| {
                    cols[0].monospace(serde_json::to_string_pretty(&c.left_configuration).unwrap());
                    cols[1]
                        .monospace(serde_json::to_string_pretty(&c.right_configuration).unwrap());
                });
            });
        ui.label("Actual configuration differences");
        if c.differences.is_empty() {
            ui.label("Identical configurations (reproduction/control).");
        }
        for d in &c.differences {
            ui.monospace(format!("{}: {} → {}", d.field, d.left, d.right));
        }
        egui::Grid::new("comparison-quantities")
            .striped(true)
            .show(ui, |ui| {
                ui.strong("Observer quantity");
                ui.strong("Left");
                ui.strong("Right");
                ui.end_row();
                for ((label, left), (_, right)) in
                    metrics(&c.left).into_iter().zip(metrics(&c.right))
                {
                    ui.label(label);
                    ui.label(left);
                    ui.label(right);
                    ui.end_row();
                }
                ui.label("Initial total (same units only)");
                for s in [&c.left, &c.right] {
                    ui.label(if c.left.experiment == c.right.experiment {
                        s.initial_total.to_string()
                    } else {
                        "Not comparable".into()
                    });
                }
                ui.end_row();
            });
        ui.small("Unknown means not captured in this projection. Exact source hashes/configurations remain in each run. Detailed history provides unsampled causal records.");
    }

    pub(super) fn history_page(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let Some(m) = self.selected.clone() else {
            ui.heading("Select a completed run in Results, then Inspect");
            return;
        };
        let Some(s) = self.summary.clone() else {
            ui.heading("No inspected history");
            if ui.button("Inspect selected run").clicked() {
                self.inspect(&m.id);
            }
            return;
        };
        ui.heading("Observer Perspective");
        ui.label(format!(
            "{} · experiment{} · {}",
            m.id,
            s.experiment,
            m.configuration.policy_label()
        ));
        ui.small(format!(
            "SHA256 {}",
            m.history_sha256.as_deref().unwrap_or("Unknown")
        ));
        ui.label("Recorded outcomes are distinct from actor knowledge and motives. Counts include bootstrap actions.");
        ui.group(|ui| {
            ui.label(format!(
                "Four individuals · ticks{}–{} · ordinary cutoff{} · final active/pending:0",
                s.start, s.end, s.nominal_stop
            ));
            ui.label(format!(
                "{} completed activities · {} meals · {} resource events · {} transferred units",
                s.completed_activities, s.meals, s.resource_events, s.transferred_units
            ));
            ui.label(format!(
                "{} successful grants · cache food{} · {} ledger matches:{}",
                s.environmental_grants,
                s.final_stock,
                if s.material.is_some() {
                    "common-mass"
                } else {
                    "food"
                },
                s.food_accounting_matches
            ));
            ui.horizontal_wrapped(|ui| {
                for a in &s.final_agents {
                    ui.label(format!(
                        "Person{}: food{} / hunger{}",
                        a.id, a.food, a.hunger
                    ));
                }
            });
        });
        if let Some(material) = &s.material {
            ui.group(|ui| {
                ui.strong("025 common-mass ledger — observer record");
                ui.label(format!("Initial raw{} → final raw{}, waste{}, escrow{}; {} prepared / {} failed conversions",material.initial_raw,material.final_raw,material.final_waste,material.final_escrow,material.prepared,material.failed));
                ui.small("Owned food + cache + consumed + raw + waste + escrow = initial common mass. Food is fungible; later meals/transfers cannot identify a prepared unit.");
                egui::CollapsingHeader::new("Extraction and conversion — first32 operations").show(ui,|ui| {
                    for op in &material.operations {ui.small(format!("operation{} tick{} commitment{} person{} {} | raw{}→{} escrow{}→{} food{}→{} waste{}→{}",op.id,op.tick,op.commitment,op.owner,op.outcome,op.raw_before,op.raw_after,op.escrow_before,op.escrow_after,op.food_before,op.food_after,op.waste_before,op.waste_after));}
                });
            });
        }
        ui.horizontal(|ui| {
            if ui.add_enabled(self.worker.is_none()&&self.store.installed.is_none(),egui::Button::new("Inspect detailed history (developer)")).clicked() {
                let store=self.store.clone();let id=m.id.clone();let repaint=ctx.clone();let(tx,rx)=mpsc::channel();
                self.worker=Some(rx);
                self.report("Verifying selected history for read-only Explorer on loopback. The selected run will open when ready.",false);
                std::thread::spawn(move||{let result=viewer::launch(&store,&id);let _=tx.send(Event::Viewer(id,result));repaint.request_repaint();});
            }
            if ui.add_enabled(self.store.installed.is_none(),egui::Button::new("Copy viewer command")).clicked() {
                match viewer::command_text(&self.store,&m.id,8766) {
                    Ok(command)=>{ctx.copy_text(command);self.report("PowerShell command copied. Python3.10+ required; selected source only.",false);},
                    Err(e)=>self.report(e,true)
                }
            }
        });
        if let Some(v) = &self.viewer {
            ui.hyperlink_to("Open local read-only Explorer", &v.url);
            ui.small("Only the selected verified run opens automatically. Server stops when workbench closes. Playback never reruns the simulation.");
        }
        self.recorded_moments(ui, &m.id, ctx);
        self.individual_history(ui, &s);
        egui::CollapsingHeader::new("Executed activities — first32 transitions").show(ui, |ui| {
            for t in &s.transitions {
                ui.monospace(format!(
                    "tick{} commitment{} {} {:?}: {}",
                    t.tick,
                    t.commitment.id,
                    t.commitment.work,
                    t.commitment.participants,
                    if t.completed { "Completed" } else { "Started" }
                ));
            }
        });
        egui::CollapsingHeader::new("Executed resource and meal consequences — first32 records each").show(ui,|ui| {
            ui.small("Actual core.events[id] and cognition.consumption[index]. Bootstrap included. Consequences are recorded; prepared food is not unit-attributed.");
            for e in &s.resource_records {
                ui.monospace(format!("event{} tick{} scene{} {:?} actor{} {} | food{:?}→{:?} transfer{} outcome{:?}",e.id,e.tick,e.scene,e.participants,e.decision.actor,e.decision.selected,e.balances_before,e.balances_after,e.transferred,e.outcome));
                ui.small(format!("Recorded interpretations: {:?}",e.interpretations));
            }
            for (index,m) in s.meal_records.iter().enumerate(){ui.monospace(format!("meal[{index}] tick{} person{} consumed{} | food{}→{} hunger{}→{}",m.tick,m.actor,m.consumed,m.food_before,m.food_after,m.hunger_before,m.hunger_after));}
        });
        self.human_observation(ui, &m.id, &s);
    }

    fn recorded_moments(&mut self, ui: &mut egui::Ui, id: &str, ctx: &egui::Context) {
        if self.detail_attempt != Some((self.moment, self.actor)) {
            self.detail = None;
        }
        ui.heading("Recorded moments — four-person lanes");
        ui.small("Read-only playback. Moving the selection never advances or reruns the simulation. A blank lane means no recorded activity choice at that boundary.");
        let before = (self.moment, self.actor);
        ui.horizontal(|ui| {
            if ui.button("Previous moment").clicked() {
                self.moment = self.moment.saturating_sub(1);
            }
            if ui.button("Next moment").clicked() {
                self.moment = (self.moment + 1).min(self.moments.len().saturating_sub(1));
            }
            if !self.moments.is_empty() {
                ui.add(
                    egui::Slider::new(&mut self.moment, 0..=self.moments.len() - 1)
                        .text("Recorded boundary"),
                );
            }
        });
        if let Some(row) = self.moments.get(self.moment) {
            ui.label(format!(
                "Boundary{} · recorded tick{}",
                row.boundary, row.tick
            ));
            ui.columns(4, |cols| {
                for (actor, col) in cols.iter_mut().enumerate() {
                    col.selectable_value(&mut self.actor, actor as u32, format!("Person{actor}"));
                    col.label(row.lanes[actor].as_deref().unwrap_or("No choice recorded"));
                    if let Some((food, hunger)) = row.states[actor] {
                        col.small(format!(
                            "Recorded boundary state: food{food}, hunger{hunger}"
                        ));
                    }
                    for text in &row.executed[actor] {
                        col.small(text);
                    }
                    for text in &row.consequences[actor] {
                        col.small(text);
                    }
                }
            });
            if before != (self.moment, self.actor) {
                self.detail = None;
            }
            if self.detail.is_none()
                && self.worker.is_none()
                && self.detail_attempt != Some((self.moment, self.actor))
            {
                self.detail_attempt = Some((self.moment, self.actor));
                let store = self.store.clone();
                let id = id.to_owned();
                let index = row.boundary;
                let actor = self.actor;
                let repaint = ctx.clone();
                let (tx, rx) = mpsc::channel();
                self.worker = Some(rx);
                std::thread::spawn(move || {
                    let result = (|| {
                        let m = execution::metadata(&store, &id)?;
                        let path = store.run(&id)?.join("history.jsonl");
                        if m.status != "Completed"
                            || m.history_sha256.as_deref() != Some(&execution::hash(&path)?)
                        {
                            return Err("Selected history identity changed".into());
                        }
                        crate::recorded::detail(&path, index, actor)
                    })();
                    let _ = tx.send(Event::Detail(id, index, actor, result.map(Box::new)));
                    repaint.request_repaint();
                });
            }
        }
        if let Some(d) = &self.detail {
            if let Some(c) = &d.choice {
                ui.strong(format!(
                    "Person{} selected {} at tick{}",
                    self.actor, c.selected, c.input.legacy.now
                ));
                ui.label(format!("Recorded local input: food{}, hunger{}, trust{:?}; observed people{}; retained receipt{}",c.input.legacy.own.food,c.input.legacy.own.hunger,c.input.legacy.own.trust,c.input.legacy.people,c.input.retained));
                ui.label(format!("Recorded contact choice: {}", c.contact));
                for candidate in &c.candidates {
                    ui.small(format!(
                        "{} · eligible{} · recorded score{}",
                        candidate.action,
                        candidate.eligible,
                        number(candidate.final_score.or(candidate.score))
                    ));
                }
                if ui.button("Annotate selected decision").clicked() {
                    self.note.individual = self.actor;
                    self.note.tick = c.input.legacy.now;
                    self.note.selected_activity = c.selected.clone();
                    self.note.local_context = format!(
                        "Recorded boundary{}; food{}, hunger{}",
                        self.moment, c.input.legacy.own.food, c.input.legacy.own.hunger
                    );
                }
            } else {
                ui.label("This person has no recorded choice at this moment.");
            }
            egui::CollapsingHeader::new("Execution and recorded consequences").default_open(true).show(ui,|ui|{
                ui.small("Commitments join by immutable boundary source and participant. Resource/meal records below are same-person temporal context through the linked deadline, not a claim that the activity caused every event.");
                for t in &d.transitions{ui.label(format!("Commitment{} {} · tick{} · {} · participants{:?}",t.commitment.id,t.commitment.work,t.tick,if t.completed{"Completed"}else{"Started"},t.commitment.participants));}
                for e in &d.events{ui.label(format!("Event{} scene{} tick{}: person{} {} · balances{:?}→{:?} · transfer{} · {:?}",e.id,e.scene,e.tick,e.decision.actor,e.decision.selected,e.balances_before,e.balances_after,e.transferred,e.outcome));}
                for m in &d.meals{ui.label(format!("Meal tick{}: food{}→{}, hunger{}→{}",m.tick,m.food_before,m.food_after,m.hunger_before,m.hunger_after));}
                if d.transitions.is_empty()&&d.events.is_empty()&&d.meals.is_empty(){ui.label("No execution link or nearby resource consequence captured for this selection. Selection alone is not execution.");}
            });
        } else if self.worker.is_some() {
            ui.label("Loading verified recorded detail…");
        }
    }
    fn individual_history(&mut self, ui: &mut egui::Ui, s: &Summary) {
        egui::CollapsingHeader::new("Captured individual decision progression — bounded sample").default_open(false).show(ui,|ui| {
            ui.horizontal(|ui|{ui.label("Individual overview");for actor in 0..4 {ui.selectable_value(&mut self.actor,actor,format!("Person{actor}"));}});
            for (label,agents) in [("Initial",&s.initial_agents),("Final",&s.final_agents)] {
                if let Some(a)=agents.iter().find(|a|a.id==self.actor){ui.label(format!("{label} recorded state: food{}, hunger{}, directed trust{:?}",a.food,a.hunger,a.trust));}
            }
            ui.small("Up to32 choices from first32 physical boundaries. Separate first32 valuation/admission/transition samples; missing links are Unknown, never proof of nonexecution. Full records are in detailed history.");
            for p in s.progressions.iter().filter(|p|s.decisions[p.decision_index].input.legacy.own.id==self.actor) {
                let c=&s.decisions[p.decision_index];
                ui.horizontal(|ui| {
                    ui.monospace(format!("tick{} food{} hunger{} → {}",c.input.legacy.now,c.input.legacy.own.food,c.input.legacy.own.hunger,c.selected));
                    if ui.small_button("Use for observation").clicked() {
                        self.note.individual=self.actor;self.note.tick=c.input.legacy.now;
                        self.note.local_context=format!("Recorded boundary{} choice{}: food{}, hunger{}",p.boundary,p.choice_index,c.input.legacy.own.food,c.input.legacy.own.hunger);
                        self.note.selected_activity=c.selected.clone();
                    }
                });
                ui.small(c.candidates.iter().map(|a|format!("{} score{} eligible{}",a.action,number(a.final_score.or(a.score)),a.eligible)).collect::<Vec<_>>().join(" · "));
                egui::CollapsingHeader::new(format!("Explain boundary{} choice{}",p.boundary,p.choice_index)).id_salt((p.boundary,p.choice_index)).show(ui,|ui| {
                    ui.label(format!("Captured actor-local trust{:?}; public people{}; ready receipt{}; raw receipt{}; duration{:?}; inquiry lane{:?}",c.input.legacy.own.trust,c.input.legacy.people,c.input.retained,c.input.raw_retained,c.input.duration,c.input.inquiry_lane_free));
                    if let Some(v)=&p.valuation {
                        ui.label(format!("Recorded {} valuation selected{}. Strict greater-than comparison preserves first eligible tie.",v.policy,v.selection.selected));
                        for t in &v.selection.terms {ui.monospace(format!("{} eligible{} base{} meal relief{} work effort{} final{}",t.action,t.eligible,number(t.base_score),number(t.meal_relief),number(t.work_effort),number(t.final_score)));}
                    } else {ui.label("Separate valuation record not captured; actual candidates shown above.");}
                    ui.label(format!("Selected contact{}",c.contact));
                    for a in &p.admissions {ui.label(format!("Admission tick{} source{} {} commitment{:?}",a.tick,a.source,a.reason,a.commitment));}
                    for (label,records) in [("Started",&p.started),("Completed",&p.completed)] {
                        for t in records {ui.label(format!("{label}: tick{} commitment{} {} {:?}, reserved{}–{}",t.tick,t.commitment.id,t.commitment.work,t.commitment.participants,t.commitment.start,t.commitment.deadline));}
                    }
                    ui.small("Source: history.boundaries[boundary].choices[choice], valuations[boundary/choice], admissions[source], transitions[commitment.id]. Missing records are Unknown. Selection, recipient consent, admission and execution remain distinct.");
                });
            }
        });
    }

    fn human_observation(&mut self, ui: &mut egui::Ui, id: &str, s: &Summary) {
        egui::CollapsingHeader::new(format!(
            "Saved human observations ({})",
            self.saved_notes.len()
        ))
        .show(ui, |ui| {
            for (index, saved) in self.saved_notes.iter().enumerate() {
                let n = &saved.observation;
                ui.label(format!(
                    "Note{} · person{} tick{} · {:?}: {}",
                    index + 1,
                    n.individual,
                    n.tick,
                    n.category,
                    n.notes
                ));
                if ui
                    .button(format!("Load note{} into form", index + 1))
                    .clicked()
                {
                    self.note = n.clone();
                }
            }
        });
        egui::CollapsingHeader::new("Human observation").default_open(true).show(ui,|ui| {
            ui.small("External human interpretation. Saved separately; never feeds actor cognition or behavior.");
            ui.horizontal(|ui| {
                ui.label("Individual");ui.add(egui::DragValue::new(&mut self.note.individual).range(0..=3));
                ui.label("World tick");ui.add(egui::DragValue::new(&mut self.note.tick).range(s.start..=s.end));
                egui::ComboBox::from_id_salt("category").selected_text(format!("{:?}",self.note.category)).show_ui(ui,|ui|{for c in [Category::Plausible,Category::Questionable,Category::Unclear,Category::PossibleBug]{ui.selectable_value(&mut self.note.category,c,format!("{c:?}"));}});
            });
            for (label,text) in [("Observed local context",&mut self.note.local_context),("Selected activity",&mut self.note.selected_activity),("Recorded outcome",&mut self.note.recorded_outcome),("Suggested improvement (human interpretation)",&mut self.note.suggested_improvement)] {ui.label(label);ui.text_edit_singleline(text);}
            ui.label("Human notes");ui.add(egui::TextEdit::multiline(&mut self.note.notes).desired_rows(3).desired_width(f32::INFINITY));
            if ui.button("Save human observation").clicked(){match observations::save(&self.store,id,&self.note){Ok(p)=>{self.report(format!("Human observation saved: {}",p.display()),false);match observations::load(&self.store,id){Ok(notes)=>self.saved_notes=notes,Err(e)=>self.report(e,true)}},Err(e)=>self.report(e,true)}}
        });
    }
}
