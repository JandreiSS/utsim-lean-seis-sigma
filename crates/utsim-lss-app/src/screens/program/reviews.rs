//! Tollgate reviews across the program (8.4): the queue with the late ones first, the
//! batch review by day or unit with the minutes of the batch, the standard work of a
//! review (standard and company questions, verified items already checked, the five items
//! of the structured opinion, target time) and the quality of the reviews.

use crate::text::LocalDate;
use eframe::egui::{self, RichText};
use utsim_lss_core::perfis::Action;
use utsim_lss_core::revisao::{self, BatchKey, Criterion, GroupBy, Mark, ReviewInput, ReviewItem};

use super::Data;
use crate::app::App;
use crate::session::Session;
use crate::text::l6::{criterion_name, l6, mark_name};
use crate::theme;

impl App {
    pub(in crate::app) fn program_reviews(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let tl = l6();
        ui.heading(tl.nav_reviews);
        if !self.may_view(ui, s, Action::ViewReviewQueue) {
            return;
        }
        ui.weak(tl.reviews_intro);
        let queue = revisao::queue(&d.records, &d.reviews, d.today);
        ui.horizontal(|ui| {
            ui.strong((tl.queue)(queue.len()));
            ui.checkbox(&mut self.l6.group_by_unit, tl.group_by_unit);
        });
        let by = if self.l6.group_by_unit {
            GroupBy::Unit
        } else {
            GroupBy::Day
        };
        let groups = revisao::batches(&queue, by);
        let mut minutes_for = None;
        for (key, items) in &groups {
            let title = match key {
                BatchKey::Day(Some(day)) => (tl.batch_day)(&day.local()),
                BatchKey::Day(None) => tl.batch_no_date.to_string(),
                BatchKey::Unit(u) => d.unit_name(*u),
            };
            ui.horizontal(|ui| {
                ui.strong(&title);
                if ui.small_button(tl.batch_minutes).clicked() {
                    minutes_for = Some((title.clone(), items.clone()));
                }
            });
            egui::Grid::new(("fila", title.clone()))
                .striped(true)
                .show(ui, |ui| {
                    for h in [
                        tl.project, tl.unit, tl.belt, tl.phase, tl.planned, tl.waiting, tl.late,
                    ] {
                        ui.strong(h);
                    }
                    ui.end_row();
                    for i in items {
                        let sel = self.l6.review_item == Some(i.tollgate);
                        let label = RichText::new(&i.title);
                        let label = if i.overdue() {
                            label.color(theme::WARN)
                        } else {
                            label
                        };
                        if ui.selectable_label(sel, label).clicked() {
                            self.l6.review_item = Some(i.tollgate);
                            self.l6.marks.clear();
                            self.l6.answers.clear();
                        }
                        ui.label(d.unit_name(i.unit));
                        ui.label(
                            i.belts
                                .iter()
                                .map(|b| d.person(*b))
                                .collect::<Vec<_>>()
                                .join(", "),
                        );
                        ui.label(&i.phase);
                        ui.label(i.planned.map_or("-".into(), |x| x.local()));
                        ui.label(format!("{} d", i.days_waiting));
                        if i.overdue() {
                            ui.colored_label(theme::WARN, format!("{} d", i.days_late));
                        } else {
                            ui.label("");
                        }
                        ui.end_row();
                    }
                });
        }
        if queue.is_empty() {
            ui.weak(tl.queue_empty);
        }
        if let Some((title, items)) = minutes_for {
            self.batch_minutes(s, d, &title, &items);
        }
        if let Some(item) = self
            .l6
            .review_item
            .and_then(|t| queue.iter().find(|i| i.tollgate == t))
            .cloned()
        {
            ui.separator();
            self.review_form(ui, s, d, &item);
        }
        ui.separator();
        self.review_quality(ui, d);
    }

    fn batch_minutes(&mut self, s: &mut Session, d: &Data, title: &str, items: &[ReviewItem]) {
        let tl = l6();
        if !self.may_do(s, Action::ReviewTollgate, None) {
            return;
        }
        // The verdict of each item: the latest opinion on its tollgate.
        let lines: Vec<(String, String, Option<Mark>)> = items
            .iter()
            .map(|i| {
                let v = d
                    .reviews
                    .iter()
                    .rfind(|r| r.tollgate == Some(i.tollgate))
                    .and_then(|r| r.verdict());
                (i.title.clone(), i.phase.clone(), v)
            })
            .collect();
        let id = s.new_id();
        let e = revisao::batch_minutes(
            d.program,
            id,
            &(tl.batch_title)(title),
            d.today,
            &lines,
            s.ids(),
        );
        if self.commit(s, e) {
            self.set_status(tl.minutes_written, false);
        }
    }

    fn review_form(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data, item: &ReviewItem) {
        let tl = l6();
        ui.strong((tl.review_of)(&item.title, &item.phase, item.round));
        ui.weak((tl.target_time)(revisao::target_minutes(
            s.state(),
            d.program,
        )));
        if !item.verified.is_empty() {
            ui.label(tl.verified_items);
            for (text, ok) in &item.verified {
                ui.label(format!("{} {text}", if *ok { "☑" } else { "☐" }));
            }
        }
        ui.label(tl.standard_questions);
        let mut questions: Vec<String> = revisao::standard_questions(&item.phase)
            .iter()
            .map(|q| q.to_string())
            .collect();
        questions.extend(
            revisao::company_questions(s.state(), &item.phase)
                .into_iter()
                .map(|(_, t)| t),
        );
        for (n, q) in questions.iter().enumerate() {
            let mut v = self.l6.answers.get(&n).copied().unwrap_or(false);
            if ui.checkbox(&mut v, q).changed() {
                self.l6.answers.insert(n, v);
            }
        }
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(self.l6.input("pergunta"))
                    .hint_text(tl.question_hint)
                    .desired_width(300.0),
            );
            if ui.button(tl.add_question).clicked() {
                let text = self.l6.take_input("pergunta");
                if !text.trim().is_empty() {
                    let e = revisao::add_question(d.program, s.new_id(), &item.phase, &text);
                    self.commit(s, e);
                }
            }
        });
        ui.label(tl.opinion);
        egui::Grid::new("parecer").show(ui, |ui| {
            for c in Criterion::ALL {
                ui.label(criterion_name(c));
                let (mut m, mut text) = self
                    .l6
                    .marks
                    .get(&c)
                    .cloned()
                    .unwrap_or((Mark::Ok, String::new()));
                let had = self.l6.marks.contains_key(&c);
                let mut chosen = had;
                egui::ComboBox::from_id_salt(("marca", c))
                    .selected_text(if had { mark_name(m) } else { "-" })
                    .show_ui(ui, |ui| {
                        for x in Mark::ALL {
                            if ui.selectable_value(&mut m, x, mark_name(x)).clicked() {
                                chosen = true;
                            }
                        }
                    });
                ui.add(
                    egui::TextEdit::singleline(&mut text)
                        .hint_text(tl.comment_hint)
                        .desired_width(320.0),
                );
                if chosen {
                    self.l6.marks.insert(c, (m, text));
                }
                ui.end_row();
            }
        });
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(self.l6.input("revisao_min"))
                    .hint_text(tl.minutes_hint)
                    .desired_width(80.0),
            );
            ui.add(
                egui::TextEdit::singleline(self.l6.input("revisao_conclusao"))
                    .hint_text(tl.summary_hint)
                    .desired_width(300.0),
            );
            if ui.button(tl.record_review).clicked() {
                if !self.may_do(s, Action::ReviewTollgate, None) {
                    return;
                }
                let input = ReviewInput {
                    marks: self.l6.marks.clone(),
                    answers: self.l6.answers.clone(),
                    summary: self.l6.input("revisao_conclusao").clone(),
                    minutes: super::super::parse_number(self.l6.input("revisao_min"))
                        .unwrap_or(0.0),
                };
                let r = revisao::record(d.program, s.new_id(), item, s.me(), d.today, &input);
                if self.commit_rule(s, r) {
                    self.l6.marks.clear();
                    self.l6.answers.clear();
                    self.l6.take_input("revisao_min");
                    self.l6.take_input("revisao_conclusao");
                    self.l6.review_item = None;
                    self.set_status(tl.review_recorded, false);
                }
            }
        });
    }

    fn review_quality(&mut self, ui: &mut egui::Ui, d: &Data) {
        let tl = l6();
        ui.strong(tl.review_quality);
        let q = revisao::quality(&d.reviews);
        if q.all.reviews == 0 {
            ui.weak(tl.no_reviews);
            return;
        }
        let row = |ui: &mut egui::Ui, name: String, st: &revisao::Stats| {
            ui.label(name);
            ui.label(st.reviews.to_string());
            ui.label(format!("{:.0} %", st.redo_rate() * 100.0));
            ui.label(st.reservations.to_string());
            ui.label(st.median_lead().map_or("-".into(), |m| format!("{m:.1} d")));
            ui.end_row();
        };
        egui::Grid::new("qualidade_revisoes")
            .striped(true)
            .show(ui, |ui| {
                for h in [
                    tl.group,
                    tl.reviews_count,
                    tl.redo_rate,
                    tl.reservations,
                    tl.median_lead,
                ] {
                    ui.strong(h);
                }
                ui.end_row();
                row(ui, tl.all.to_string(), &q.all);
                for (p, st) in &q.by_phase {
                    row(ui, (tl.phase_is)(p), st);
                }
                for (b, st) in &q.by_belt {
                    row(ui, d.person(*b), st);
                }
            });
        for (b, c, n) in &q.recurring {
            ui.colored_label(
                theme::WARN,
                (tl.recurring)(&d.person(*b), criterion_name(*c), *n),
            );
        }
        ui.small(tl.rule_quality);
    }
}
