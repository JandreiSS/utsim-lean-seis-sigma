//! My training (8.5, the trainee's view): the nominations and their state, the classes
//! with the attendance, the exams in **exam mode** (the paper of the class folder, without
//! the key; the answers are the trainee's own operations, graded on the instructor's
//! machine), the grades and the certifications with their validity.

use eframe::egui;
use utsim_lss_core::formacao::{self, exams, exams::QuestionKind};
use utsim_lss_core::{Id, certificacao};
use utsim_lss_store::program::TrainingFolder;

use super::Data;
use super::exams::answers_value;
use crate::app::App;
use crate::session::Session;
use crate::text::l6::level_name;
use crate::text::l7::{l7, status_name, validity_text};
use crate::theme;

impl App {
    pub(in crate::app) fn program_my_training(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
    ) {
        let t = l7();
        if let Some(ex) = self.l6.l7.exam_mode_on {
            self.exam_mode(ui, s, d, ex);
            return;
        }
        ui.heading(t.nav_my_training);
        ui.weak(t.my_training_intro);
        let mine: Vec<&formacao::Nomination> =
            d.tr.noms
                .iter()
                .filter(|n| n.person == Some(d.me))
                .collect();
        if mine.is_empty() {
            ui.label(t.no_nomination);
        }
        for n in &mine {
            ui.label((t.my_nomination)(
                level_name(n.level),
                &d.tr.wave_name(n.wave),
                status_name(n.status),
            ));
            let Some(k) = d.tr.class(n.class).cloned() else {
                continue;
            };
            if n.status != formacao::NominationStatus::Selected {
                continue;
            }
            let c = formacao::completion(&k, &d.tr.sessions, d.me);
            ui.label((t.my_attendance)(&k.name, c.rate * 100.0, c.min * 100.0));
            let folder = TrainingFolder::Class(k.id);
            let Some(cs) = self.training_state(s, d, folder) else {
                ui.weak(t.class_folder_missing);
                continue;
            };
            for p in exams::published(&cs) {
                let Some(paper) = p.papers.get(&d.me) else {
                    continue;
                };
                let handed = exams::all_answers(&cs).into_iter().any(|a| {
                    a.exam == Some(p.exam) && a.person == Some(d.me) && a.submitted_on.is_some()
                });
                ui.horizontal(|ui| {
                    ui.label((t.paper_line)(&paper.name, paper.items.len()));
                    if handed {
                        ui.label(t.handed);
                    } else if ui.button(t.start_exam).clicked() {
                        self.l6.l7.exam_mode_on = Some(p.exam);
                        self.l6.l7.answers.clear();
                    }
                });
            }
            for r in exams::results(&cs)
                .into_iter()
                .filter(|r| r.person == Some(d.me))
            {
                let name =
                    d.tr.exams
                        .iter()
                        .find(|x| Some(x.id) == r.exam)
                        .map_or_else(String::new, |x| x.name.clone());
                ui.label((t.my_grade)(
                    &name,
                    r.score * 100.0,
                    if r.passed { t.passed } else { t.failed },
                ));
            }
            self.reaction_form(ui, s, d, k.id, &cs);
        }
        ui.separator();
        ui.strong(t.my_certifications);
        for c in
            d.tr.certs
                .iter()
                .filter(|c| c.person == Some(d.me) && c.number.is_some())
        {
            let v = certificacao::validity(c, &d.tr.certs, d.today);
            ui.label(format!(
                "{:04} · {} · {} · {}",
                c.number.unwrap_or(0),
                level_name(c.level),
                c.valid_until.map(|x| x.format_br()).unwrap_or_default(),
                validity_text(v)
            ));
        }
    }

    fn exam_mode(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
        exam: utsim_lss_core::Id,
    ) {
        let t = l7();
        let Some(class) =
            d.tr.exams
                .iter()
                .find(|x| x.id == exam)
                .and_then(|x| x.class)
        else {
            self.l6.l7.exam_mode_on = None;
            return;
        };
        let folder = TrainingFolder::Class(class);
        let Some(cs) = self.training_state(s, d, folder) else {
            self.l6.l7.exam_mode_on = None;
            return;
        };
        let Some(paper) = exams::published(&cs)
            .into_iter()
            .find(|p| p.exam == exam)
            .and_then(|p| p.papers.get(&d.me).cloned())
        else {
            self.l6.l7.exam_mode_on = None;
            return;
        };
        ui.heading((t.exam_mode_title)(&paper.name));
        ui.colored_label(theme::WARN, t.exam_mode_note);
        for (i, it) in paper.items.iter().enumerate() {
            ui.group(|ui| {
                ui.strong(format!("{}.", i + 1));
                ui.label(&it.statement);
                if !it.data.is_empty() {
                    ui.monospace(
                        it.data
                            .iter()
                            .map(|x| crate::app::screens::show_number(*x))
                            .collect::<Vec<_>>()
                            .join("  "),
                    );
                }
                let a = self.l6.l7.answers.entry(it.question).or_default();
                match it.kind {
                    QuestionKind::MultipleChoice => {
                        for (j, o) in it.options.iter().enumerate() {
                            ui.radio_value(a, j.to_string(), o);
                        }
                    }
                    QuestionKind::TrueFalse => {
                        ui.horizontal(|ui| {
                            ui.radio_value(a, "true".to_string(), t.true_);
                            ui.radio_value(a, "false".to_string(), t.false_);
                        });
                    }
                    _ => {
                        ui.add(
                            egui::TextEdit::singleline(a)
                                .hint_text(t.answer_hint)
                                .desired_width(160.0),
                        );
                    }
                }
            });
        }
        ui.horizontal(|ui| {
            let mine = exams::all_answers(&cs)
                .into_iter()
                .find(|a| a.exam == Some(exam) && a.person == Some(d.me));
            let id = mine.as_ref().map_or_else(|| s.new_id(), |a| a.id);
            let values = answers_value(&self.l6.l7.answers, &paper);
            if ui.button(t.save_answers).clicked() {
                let e = exams::answer(id, exam, d.me, &values, None);
                if self.write_training(s, d, folder, e) {
                    self.set_status(t.answers_saved, false);
                }
            }
            if ui.button(t.hand_in).clicked() {
                let e = exams::answer(id, exam, d.me, &values, Some(d.today));
                if self.write_training(s, d, folder, e) {
                    self.l6.l7.exam_mode_on = None;
                    self.set_status(t.handed_in_ok, false);
                }
            }
            if ui.button(t.cancel).clicked() {
                self.l6.l7.exam_mode_on = None;
            }
        });
    }
}

impl App {
    /// The trainee's reaction evaluation of a class (once; nothing after sending).
    fn reaction_form(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
        class: Id,
        cs: &utsim_lss_core::State,
    ) {
        use utsim_lss_core::formacao::reacao;
        let t = crate::text::l10::l10();
        let sent = reacao::reactions(cs)
            .iter()
            .any(|r| r.class == Some(class) && r.person == Some(d.me) && r.sent);
        egui::CollapsingHeader::new(t.reaction_title)
            .id_salt(("reacao", class))
            .show(ui, |ui| {
                if sent {
                    ui.label(t.reaction_sent);
                    return;
                }
                ui.weak(t.reaction_intro);
                let entry = self.l6.l7.reaction.entry(class).or_default();
                for (i, st) in reacao::STATEMENTS.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(*st);
                        for v in 1..=5u8 {
                            ui.selectable_value(&mut entry.0[i], v, v.to_string());
                        }
                    });
                }
                ui.add(
                    egui::TextEdit::multiline(&mut entry.1)
                        .hint_text(t.reaction_comment)
                        .desired_rows(2),
                );
            });
        if sent {
            return;
        }
        if ui.small_button(t.reaction_send).clicked() {
            let (scores, comment) = self.l6.l7.reaction.get(&class).cloned().unwrap_or_default();
            self.send_reaction_in(s, d, class, scores, &comment);
        }
    }

    pub(in crate::app) fn send_reaction_in(
        &mut self,
        s: &mut Session,
        d: &Data,
        class: Id,
        scores: [u8; 5],
        comment: &str,
    ) -> bool {
        use utsim_lss_core::formacao::reacao;
        let sc: [Option<u8>; 5] = scores.map(|v| (v > 0).then_some(v));
        let e = reacao::answer(class, s.new_id(), d.me, &sc, comment, d.today);
        let ok = self.write_training(s, d, TrainingFolder::Class(class), e);
        if ok {
            self.l6.l7.reaction.remove(&class);
            self.set_status(crate::text::l10::l10().reaction_thanks, false);
        }
        ok
    }
}
