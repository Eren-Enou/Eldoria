use super::*;
use crate::notebook::{Anchor, Availability, Bookmark};

#[derive(Clone)]
pub(super) struct Draft {
    anchor: Anchor,
    title: String,
    body: String,
    revision: u32,
    original: Option<(String, String)>,
}
impl Draft {
    fn dirty(&self) -> bool {
        self.original
            .as_ref()
            .is_none_or(|(title, body)| *title != self.title || *body != self.body)
    }
}
#[derive(Default)]
pub(super) struct Editor {
    draft: Option<Draft>,
}
impl Editor {
    pub(super) fn dirty(&self) -> bool {
        self.draft.as_ref().is_some_and(Draft::dirty)
    }
    pub(super) fn clear(&mut self) {
        self.draft = None;
    }
    pub(super) fn sync(&mut self, frame: &Frame) {
        if self.dirty() {
            return;
        }
        let Some(notebook) = &frame.notebook else {
            return;
        };
        if self
            .draft
            .as_ref()
            .is_some_and(|d| d.anchor.world != notebook.world)
        {
            self.clear();
        }
        if let Some(note) = &notebook.selected {
            let replace = self
                .draft
                .as_ref()
                .is_none_or(|d| d.anchor != note.anchor || d.revision != note.revision);
            if replace {
                self.load(note);
            }
        }
    }
    fn load(&mut self, note: &Bookmark) {
        self.draft = Some(Draft {
            anchor: note.anchor.clone(),
            title: note.title.clone(),
            body: note.body.clone(),
            revision: note.revision,
            original: Some((note.title.clone(), note.body.clone())),
        });
    }
    pub(super) fn saved(&mut self, submitted: &Draft) {
        if let Some(draft) = &mut self.draft
            && draft.anchor == submitted.anchor
        {
            draft.original = Some((submitted.title.clone(), submitted.body.clone()));
            draft.revision = submitted.revision + 1;
        }
    }
}

impl ObserverApp {
    pub(super) fn save_note(&mut self, after: Option<Intent>) {
        if self.perspective != Perspective::Observer || !self.after_note.is_empty() {
            return;
        }
        let Some(draft) = self.notebook.draft.clone() else {
            return;
        };
        if let Some(id) = self.send(Command::SaveBookmark {
            anchor: draft.anchor.clone(),
            expected_revision: draft.revision,
            title: draft.title.clone(),
            body: draft.body.clone(),
        }) {
            self.after_note.insert(id, (draft, after));
        }
    }
    pub(super) fn notebook_list(&mut self, ui: &mut egui::Ui, frame: &Frame) {
        let Some(notebook) = &frame.notebook else {
            return;
        };
        ui.separator();
        egui::CollapsingHeader::new(format!(
            "Observer notebook · {} bookmarks",
            notebook.entries.len()
        ))
        .default_open(true)
        .show(ui, |ui| {
            ui.label(
                RichText::new("External human observations · never character memory")
                    .small()
                    .color(MUTED),
            );
            if ui.small_button("Reload notebook").clicked() {
                self.send(Command::ReloadNotebook);
            }
            if let Some(error) = &notebook.error {
                ui.colored_label(Color32::LIGHT_RED, error);
            }
            egui::ScrollArea::vertical()
                .id_salt("bookmarks")
                .max_height(110.0)
                .show(ui, |ui| {
                    if notebook.entries.is_empty() {
                        ui.label("Select a recorded event and choose Bookmark this record.");
                    }
                    for bookmark in &notebook.entries {
                        let availability = match bookmark.availability {
                            Availability::Available => "",
                            Availability::BeyondPrefix => " · outside this checkpoint prefix",
                            Availability::DifferentHistory => " · different history",
                        };
                        if ui
                            .add_enabled(
                                bookmark.availability == Availability::Available,
                                egui::Button::new(format!(
                                    "t{} · {}{availability}",
                                    bookmark.tick, bookmark.title
                                )),
                            )
                            .clicked()
                        {
                            self.filter = EventFilter::All;
                            self.frame = None;
                            self.send(Command::JumpBookmark(bookmark.key.clone()));
                        }
                    }
                });
        });
    }
    pub(super) fn note_editor(&mut self, ui: &mut egui::Ui, frame: &Frame) {
        if frame.perspective != Perspective::Observer {
            return;
        }
        let Some(anchor) = &frame.selected_anchor else {
            return;
        };
        ui.add_space(8.0);
        let same = self
            .notebook
            .draft
            .as_ref()
            .is_some_and(|d| d.anchor == *anchor);
        if !same
            && ui
                .add_enabled(
                    !self.notebook.dirty(),
                    egui::Button::new("Bookmark this record"),
                )
                .clicked()
        {
            if let Some(existing) = frame.notebook.as_ref().and_then(|n| n.selected.as_ref()) {
                self.notebook.load(existing);
            } else {
                let title = frame
                    .detail
                    .as_ref()
                    .map(|d| projection::row(&d.record).title)
                    .unwrap_or_else(|| "Observation".into());
                let title: String = title.chars().take(80).collect();
                self.notebook.draft = Some(Draft {
                    anchor: anchor.clone(),
                    title,
                    body: String::new(),
                    revision: 0,
                    original: None,
                });
            }
        }
        let mut save = false;
        let mut jump = None;
        if let Some(draft) = &mut self.notebook.draft {
            ui.label(
                RichText::new(format!(
                    "Observer note · record {} · tick {}",
                    draft.anchor.record, draft.anchor.tick
                ))
                .color(GOLD),
            );
            ui.label(
                RichText::new("Saved separately from world checkpoints")
                    .small()
                    .color(MUTED),
            );
            if draft.anchor != *anchor {
                ui.label("This draft stays pinned to its original event.");
                if ui.small_button("Show draft event").clicked() {
                    jump = Some(draft.anchor.record);
                }
            }
            if let Some(saved) = frame.notebook.as_ref().and_then(|n| n.selected.as_ref())
                && saved.anchor == draft.anchor
                && saved.revision > draft.revision
            {
                ui.colored_label(
                    GOLD,
                    "A newer saved revision exists. Your draft is retained.",
                );
                egui::CollapsingHeader::new("Review latest saved text").show(ui, |ui| {
                    ui.label(&saved.title);
                    ui.label(&saved.body);
                });
                if ui
                    .button("Use latest revision as base; keep draft")
                    .clicked()
                {
                    draft.revision = saved.revision;
                    draft.original = Some((saved.title.clone(), saved.body.clone()));
                }
            }
            ui.label("Bookmark title");
            ui.add(egui::TextEdit::singleline(&mut draft.title).desired_width(f32::INFINITY));
            ui.label("Observer note");
            ui.add(
                egui::TextEdit::multiline(&mut draft.body)
                    .desired_rows(4)
                    .desired_width(f32::INFINITY),
            );
            ui.label(
                RichText::new(format!("{} /4000 UTF-8 bytes", draft.body.len()))
                    .small()
                    .color(MUTED),
            );
            save = ui
                .add_enabled(
                    draft.dirty() && self.after_note.is_empty(),
                    egui::Button::new("Save bookmark & note"),
                )
                .clicked();
        }
        if let Some(id) = jump {
            self.select(id);
        }
        if save {
            self.save_note(None);
        }
        ui.separator();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acknowledgement_preserves_edits_made_while_save_is_pending() {
        let mut editor = Editor::default();
        let note = Bookmark {
            anchor: Anchor {
                world: "a".repeat(64),
                record: 7,
                tick: 2,
                prefix: "b".repeat(64),
            },
            revision: 1,
            title: "Original".into(),
            body: "Old text".into(),
        };
        editor.load(&note);
        editor.draft.as_mut().unwrap().body = "Submitted".into();
        let submitted = editor.draft.clone().unwrap();
        editor.draft.as_mut().unwrap().body = "Typed during save".into();
        editor.saved(&submitted);
        assert!(editor.dirty());
        let draft = editor.draft.as_ref().unwrap();
        assert_eq!(draft.revision, 2);
        assert_eq!(draft.body, "Typed during save");
        assert_eq!(draft.original.as_ref().unwrap().1, "Submitted");
        let next = draft.clone();
        editor.saved(&next);
        assert!(!editor.dirty());
    }
}
