//! Closing, lessons learned and the knowledge base (section 1.9).

use eframe::egui::{self, RichText};
use utsim_lss_core::closure;
use utsim_lss_core::domain::Project;
use utsim_lss_core::lessons;
use utsim_lss_core::op::{EntityRef, Kind};

use super::super::App;
use crate::session::Session;
use crate::text::l1::{l1, requirement_name};
use crate::text::texts;
use crate::theme;

impl App {
    pub(in crate::app) fn closure(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let tl = l1();
        let t = texts();
        ui.heading(tl.nav_closure);
        ui.weak(tl.closure_intro);
        let p = Project::entity_ref(s.project());
        egui::Grid::new("encerramento")
            .num_columns(2)
            .spacing([12.0, 6.0])
            .show(ui, |ui| {
                ui.label(tl.c_result);
                self.text_field(ui, s, p, closure::fields::RESULT, true, 460.0);
                ui.end_row();
                ui.label(tl.c_notes);
                self.text_field(ui, s, p, closure::fields::NOTES, true, 460.0);
                ui.end_row();
            });
        let accepted = s
            .state()
            .entity(s.project())
            .is_some_and(|e| e.boolean(closure::fields::OWNER_ACCEPTED));
        if accepted {
            let who = s
                .state()
                .entity(s.project())
                .and_then(|e| e.id(closure::fields::OWNER_ACCEPTED_BY))
                .map(|id| utsim_lss_core::domain::person_name(s.state(), id))
                .unwrap_or_default();
            ui.colored_label(theme::accent(ui), (tl.owner_accepted)(&who));
        } else if ui.button(tl.accept_transfer).clicked() {
            let r = closure::accept(s.state(), s.project(), s.me(), s.today());
            self.commit_rule(s, r);
        }
        ui.add_space(6.0);
        for (req, ok) in closure::check(s.state(), s.project()) {
            let (mark, color) = if ok {
                ("✔", theme::accent(ui))
            } else {
                ("✖", theme::WARN)
            };
            ui.horizontal(|ui| {
                ui.colored_label(color, mark);
                let mut txt = RichText::new(requirement_name(req));
                if req.required() {
                    txt = txt.strong();
                }
                ui.label(txt);
                if !req.required() {
                    ui.weak(tl.advice);
                }
            });
        }
        if ui.button(tl.close_project_now).clicked() {
            let r = closure::close(s.state(), s.project(), s.today());
            if self.commit_rule(s, r) {
                self.set_status(tl.project_closed, false);
            }
        }
        ui.separator();
        // Lessons.
        ui.heading(tl.lessons);
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.l1.lesson_title)
                    .hint_text(tl.new_lesson_hint)
                    .desired_width(360.0),
            );
            if ui.button(t.add).clicked() && !self.l1.lesson_title.trim().is_empty() {
                let e = lessons::new_lesson(s.project(), s.new_id(), &self.l1.lesson_title);
                if self.commit(s, e) {
                    self.l1.lesson_title.clear();
                }
            }
        });
        for l in lessons::lessons(s.state(), s.project()) {
            let r = EntityRef::new(Kind::Lesson, l.id);
            theme::card(ui).show(ui, |ui| {
                ui.set_min_width(520.0);
                egui::Grid::new(("licao", l.id))
                    .num_columns(2)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        for (label, f, multi) in [
                            (tl.l_title, lessons::fields::TITLE, false),
                            (tl.l_worked, lessons::fields::WORKED, true),
                            (tl.l_did_not_work, lessons::fields::DID_NOT_WORK, true),
                            (tl.l_repeat, lessons::fields::REPEAT, true),
                            (tl.l_tags, lessons::fields::TAGS, false),
                        ] {
                            ui.label(label);
                            self.text_field(ui, s, r, f, multi, 420.0);
                            ui.end_row();
                        }
                    });
            });
        }
        ui.separator();
        // Knowledge base.
        ui.heading(tl.knowledge_base);
        ui.weak(tl.knowledge_intro);
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.l1.search)
                    .hint_text(tl.search_hint)
                    .desired_width(360.0),
            );
            if ui.button(tl.refresh_portfolio).clicked() {
                self.l1.portfolio.rescan();
            }
        });
        let mut docs = lessons::docs_of(s.state(), s.project());
        let me = s.folder().me().clone();
        let cache = self.dirs.cache.clone();
        self.l1
            .portfolio
            .ensure_scan(s.folder().root(), &me, &cache);
        docs.extend(
            self.l1
                .portfolio
                .lesson_docs()
                .iter()
                .filter(|d| d.project != s.project())
                .cloned(),
        );
        let hits = lessons::search(&docs, &self.l1.search);
        if !self.l1.search.trim().is_empty() {
            ui.label((tl.hits)(hits.len(), docs.len()));
        }
        for h in hits.iter().take(30) {
            let d = &docs[h.index];
            theme::card(ui).show(ui, |ui| {
                ui.set_min_width(520.0);
                ui.strong(&d.lesson.title);
                ui.weak(format!("{} · {}", d.project_title, d.process));
                if !d.lesson.worked.is_empty() {
                    ui.label(format!("{}: {}", tl.l_worked, d.lesson.worked));
                }
                if !d.lesson.did_not_work.is_empty() {
                    ui.label(format!("{}: {}", tl.l_did_not_work, d.lesson.did_not_work));
                }
                if !d.lesson.repeat.is_empty() {
                    ui.label(format!("{}: {}", tl.l_repeat, d.lesson.repeat));
                }
                if !d.lesson.tags.is_empty() {
                    ui.weak(d.lesson.tags.join(", "));
                }
            });
        }
        // Similar projects (same process or tags in common).
        let process =
            Project::read(s.project(), s.state().entity(s.project()).expect("project")).process;
        let tags: Vec<String> = lessons::lessons(s.state(), s.project())
            .into_iter()
            .flat_map(|l| l.tags)
            .collect();
        let sim = lessons::similar(&docs, s.project(), &process, &tags);
        if !sim.is_empty() {
            ui.add_space(6.0);
            ui.strong(tl.similar_projects);
            for (pid, _) in sim.iter().take(10) {
                if let Some(d) = docs.iter().find(|d| d.project == *pid) {
                    ui.label(format!("▸ {} ({})", d.project_title, d.process));
                }
            }
        }
    }
}
