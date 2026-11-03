//! Exams (8.5, decision MB3): the question bank with the keys in its restricted folder,
//! the exam drawn with a recorded seed, the papers published in the class folder without
//! the key, the correction on the instructor's machine (the paper is rebuilt from the seed
//! and the bank) and the grades of the paper exams.

use std::collections::BTreeMap;

use eframe::egui;
use serde_json::Value;
use utsim_lss_core::formacao::exams::{self, ExamInput, Mode, Problem, QuestionKind};
use utsim_lss_core::formacao::{self, curriculum};
use utsim_lss_core::perfis::Action;
use utsim_lss_core::{Edit, Id};
use utsim_lss_store::program::TrainingFolder;

use super::Data;
use crate::app::App;
use crate::session::Session;
use crate::text::l7::{l7, mode_name, recipe_name};
use crate::text::texts;
use crate::theme;

/// The recipes offered in the question form, with default parameters.
pub fn recipes() -> [Problem; 5] {
    [
        Problem::Mean {
            n: 20,
            mean: 50.0,
            sd: 2.0,
        },
        Problem::StdDev {
            n: 20,
            mean: 50.0,
            sd: 2.0,
        },
        Problem::Cpk {
            n: 30,
            mean: 10.0,
            sd: 0.2,
            lsl: 9.4,
            usl: 10.6,
        },
        Problem::Ppk {
            n: 30,
            mean: 10.0,
            sd: 0.2,
            lsl: 9.4,
            usl: 10.6,
        },
        Problem::Dpmo {
            defects: 25,
            units: 500,
            opportunities: 4,
        },
    ]
}

impl App {
    pub(in crate::app) fn program_exams(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let t = l7();
        ui.heading(t.nav_exams);
        ui.weak(t.exams_intro);
        if s.may(Action::Teach, None).is_err() {
            ui.colored_label(theme::WARN, t.staff_only);
            return;
        }
        let bank_state = self.training_state(s, d, TrainingFolder::Questions);
        let bank = bank_state.as_ref().map(exams::bank).unwrap_or_default();
        self.bank_section(ui, s, d, &bank);
        ui.separator();
        self.class_picker(ui, d);
        let Some(k) = d.tr.class(self.l6.l7.class).cloned() else {
            return;
        };
        self.new_exam_section(ui, s, d, &k, &bank);
        ui.separator();
        self.exam_list_section(ui, s, d, &k, &bank);
    }

    fn bank_section(&mut self, ui: &mut egui::Ui, s: &Session, d: &Data, bank: &[exams::Question]) {
        let t = l7();
        ui.strong((t.bank_title)(bank.len()));
        ui.weak(t.bank_intro);
        egui::Grid::new("banco").striped(true).show(ui, |ui| {
            for h in [t.module, t.kind, t.difficulty, t.statement] {
                ui.strong(h);
            }
            ui.end_row();
            for q in bank {
                ui.label(q.module.map(|m| d.tr.module_name(m)).unwrap_or_default());
                ui.label(q.kind.label());
                ui.label(q.difficulty.to_string());
                ui.label(&q.statement);
                ui.end_row();
            }
        });
        let q = &mut self.l6.l7.question;
        ui.collapsing(t.new_question, |ui| {
            egui::Grid::new("nova_questao")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label(t.module);
                    egui::ComboBox::from_id_salt("modulo_questao")
                        .selected_text(q.module.map(|m| d.tr.module_name(m)).unwrap_or_default())
                        .show_ui(ui, |ui| {
                            for m in &d.tr.modules {
                                ui.selectable_value(&mut q.module, Some(m.id), &m.name);
                            }
                        });
                    ui.end_row();
                    ui.label(t.kind);
                    egui::ComboBox::from_id_salt("tipo_questao")
                        .selected_text(q.kind.label())
                        .show_ui(ui, |ui| {
                            for k in QuestionKind::ALL {
                                ui.selectable_value(&mut q.kind, k, k.label());
                            }
                        });
                    ui.end_row();
                    ui.label(t.difficulty);
                    ui.add(egui::Slider::new(&mut q.difficulty, 1..=3));
                    ui.end_row();
                    ui.label(t.statement);
                    ui.add(egui::TextEdit::multiline(&mut q.statement).desired_width(420.0));
                    ui.end_row();
                    match q.kind {
                        QuestionKind::MultipleChoice => {
                            ui.label(t.options);
                            let mut text = q.options.join("\n");
                            if ui
                                .add(egui::TextEdit::multiline(&mut text).hint_text(t.options_hint))
                                .changed()
                            {
                                q.options = text.lines().map(String::from).collect();
                            }
                            ui.end_row();
                            ui.label(t.right_option);
                            let mut i = q.right_option.unwrap_or(0) + 1;
                            ui.add(egui::DragValue::new(&mut i).range(1..=q.options.len().max(1)));
                            q.right_option = Some(i - 1);
                            ui.end_row();
                        }
                        QuestionKind::TrueFalse => {
                            ui.label(t.key);
                            ui.checkbox(&mut q.true_false, t.true_);
                            ui.end_row();
                        }
                        QuestionKind::Numeric => {
                            ui.label(t.key);
                            let mut x = q.number.unwrap_or(0.0);
                            ui.add(egui::DragValue::new(&mut x).speed(0.1));
                            q.number = Some(x);
                            ui.end_row();
                            ui.label(t.tolerance);
                            ui.add(
                                egui::DragValue::new(&mut q.tolerance)
                                    .speed(0.01)
                                    .range(0.0..=1e9),
                            );
                            ui.end_row();
                        }
                        QuestionKind::Practical => {
                            ui.label(t.recipe);
                            let rs = recipes();
                            let r = &mut self.l6.l7.recipe;
                            egui::ComboBox::from_id_salt("receita")
                                .selected_text(recipe_name(&rs[(*r).min(4)]))
                                .show_ui(ui, |ui| {
                                    for (i, p) in rs.iter().enumerate() {
                                        ui.selectable_value(r, i, recipe_name(p));
                                    }
                                });
                            if q.problem
                                .as_ref()
                                .is_none_or(|p| recipe_name(p) != recipe_name(&rs[*r]))
                            {
                                q.problem = Some(rs[*r].clone());
                            }
                            ui.end_row();
                            ui.label(t.parameters);
                            if let Some(p) = q.problem.as_mut() {
                                problem_params(ui, p);
                            }
                            ui.end_row();
                            ui.label(t.tolerance);
                            ui.add(
                                egui::DragValue::new(&mut q.tolerance)
                                    .speed(0.01)
                                    .range(0.0..=1e9),
                            );
                            ui.end_row();
                        }
                    }
                });
        });
        let mut add: Option<Vec<Edit>> = None;
        ui.horizontal(|ui| {
            if ui.button(t.add_question).clicked() {
                match exams::add_question(s.new_id(), &self.l6.l7.question) {
                    Ok(e) => add = Some(e),
                    Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                }
            }
            if ui.button(t.sample_questions).clicked() {
                let mut e = Vec::new();
                for m in &d.tr.modules {
                    for qi in exams::sample_questions(m.id, &m.tools) {
                        if let Ok(x) = exams::add_question(s.new_id(), &qi) {
                            e.extend(x);
                        }
                    }
                }
                add = Some(e);
            }
        });
        if let Some(e) = add
            && self.write_training(s, d, TrainingFolder::Questions, e)
        {
            self.l6.l7.question.statement.clear();
            self.set_status(t.question_saved, false);
        }
    }

    fn new_exam_section(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
        k: &formacao::Class,
        bank: &[exams::Question],
    ) {
        let t = l7();
        ui.strong((t.new_exam_of)(&k.name));
        let mods = curriculum::of_level(&d.tr.modules, k.level);
        ui.horizontal_wrapped(|ui| {
            for m in &mods {
                let mut on = self.l6.l7.exam_modules.contains(&m.id);
                if ui.checkbox(&mut on, &m.name).changed() {
                    if on {
                        self.l6.l7.exam_modules.push(m.id);
                    } else {
                        self.l6.l7.exam_modules.retain(|x| *x != m.id);
                    }
                }
            }
        });
        ui.horizontal(|ui| {
            for (i, label) in [t.easy, t.medium, t.hard].into_iter().enumerate() {
                ui.label(label);
                ui.add(egui::DragValue::new(&mut self.l6.l7.exam_counts[i]).range(0..=200));
            }
            egui::ComboBox::from_id_salt("aplicacao")
                .selected_text(mode_name(self.l6.l7.exam_mode))
                .show_ui(ui, |ui| {
                    for m in [Mode::App, Mode::Paper] {
                        ui.selectable_value(&mut self.l6.l7.exam_mode, m, mode_name(m));
                    }
                });
        });
        let exams_k = exams::exams_of(&d.tr.exams, k.id);
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(self.l6.input("nome_prova"))
                    .hint_text(t.exam_name_hint)
                    .desired_width(220.0),
            );
            egui::ComboBox::from_id_salt("reaplicacao")
                .selected_text(
                    self.l6
                        .l7
                        .exam_retake
                        .and_then(|x| exams_k.iter().find(|e| e.id == x))
                        .map_or_else(
                            || t.first_application.to_string(),
                            |e| (t.retake_of)(&e.name),
                        ),
                )
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.l6.l7.exam_retake, None, t.first_application);
                    for e in exams_k.iter().filter(|e| e.retake_of.is_none()) {
                        ui.selectable_value(
                            &mut self.l6.l7.exam_retake,
                            Some(e.id),
                            (t.retake_of)(&e.name),
                        );
                    }
                });
            if ui.button(t.draw_exam).clicked() && self.may_do(s, Action::Teach, None) {
                // The seed comes from the clock's random bits and is recorded.
                let seed =
                    u64::from_le_bytes(s.new_id().as_bytes()[8..16].try_into().unwrap_or([0; 8]));
                let input = ExamInput {
                    class: k.id,
                    name: {
                        let n = self.l6.take_input("nome_prova");
                        if n.trim().is_empty() {
                            t.default_exam_name.to_string()
                        } else {
                            n
                        }
                    },
                    modules: self.l6.l7.exam_modules.clone(),
                    counts: self.l6.l7.exam_counts,
                    seed,
                    mode: self.l6.l7.exam_mode,
                    min_grade: k.min_grade,
                    date: Some(d.today),
                    retake_of: self.l6.l7.exam_retake,
                    counts_for_cert: true,
                };
                match exams::new_exam(d.program, s.new_id(), &input, bank) {
                    Ok(e) => {
                        let r = s.apply(e);
                        self.report(r);
                    }
                    Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                }
            }
        });
        ui.weak(t.exam_consult_note);
    }

    fn exam_list_section(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
        k: &formacao::Class,
        bank: &[exams::Question],
    ) {
        let t = l7();
        let students = formacao::students(&d.tr.noms, k.id);
        let class_folder = TrainingFolder::Class(k.id);
        let cstate = self.training_state(s, d, class_folder);
        let published = cstate.as_ref().map(exams::published).unwrap_or_default();
        let answers = cstate.as_ref().map(exams::all_answers).unwrap_or_default();
        let gstate = self.training_state(s, d, TrainingFolder::Grades);
        let results = gstate.as_ref().map(exams::results).unwrap_or_default();
        for ex in exams::exams_of(&d.tr.exams, k.id) {
            let ex = ex.clone();
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.strong(&ex.name);
                    ui.label(mode_name(ex.mode));
                    ui.weak((t.exam_facts)(
                        ex.questions.len(),
                        ex.seed,
                        ex.min_grade * 100.0,
                    ));
                    if ex.retake_of.is_some() {
                        ui.label(t.retake);
                    }
                    let mut counts = ex.counts_for_cert;
                    if ui.checkbox(&mut counts, t.counts_for_cert).changed()
                        && self.may_do(s, Action::Teach, None)
                    {
                        let r = s.set_field(
                            exams::Exam::entity_ref(ex.id),
                            exams::fields::COUNTS_FOR_CERT,
                            Value::from(counts),
                            None,
                        );
                        self.report(r);
                    }
                });
                let is_pub = published.iter().any(|p| p.exam == ex.id);
                if ex.mode == Mode::App {
                    ui.horizontal(|ui| {
                        if ui
                            .button(if is_pub { t.publish_again } else { t.publish })
                            .clicked()
                        {
                            match exams::publish(&ex, bank, &students) {
                                Ok(e) => {
                                    if self.write_training(s, d, class_folder, e) {
                                        self.set_status((t.published)(students.len()), false);
                                    }
                                }
                                Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                            }
                        }
                        let handed: Vec<&exams::Answers> = answers
                            .iter()
                            .filter(|a| a.exam == Some(ex.id) && a.submitted_on.is_some())
                            .collect();
                        ui.label((t.handed_in)(handed.len(), students.len()));
                        if !handed.is_empty() && ui.button(t.grade_all).clicked() {
                            self.grade_answers(s, d, &ex, bank, &handed, &results);
                        }
                    });
                }
                egui::Grid::new(("notas", ex.id))
                    .striped(true)
                    .show(ui, |ui| {
                        for h in [t.person, t.score, t.result, ""] {
                            ui.strong(h);
                        }
                        ui.end_row();
                        for p in &students {
                            ui.label(d.person(*p));
                            let r = results
                                .iter()
                                .rev()
                                .find(|r| r.exam == Some(ex.id) && r.person == Some(*p));
                            ui.label(
                                r.map_or_else(String::new, |r| format!("{:.0} %", r.score * 100.0)),
                            );
                            ui.label(r.map_or(t.not_graded, |r| {
                                if r.passed { t.passed } else { t.failed }
                            }));
                            if ex.mode == Mode::Paper {
                                let key = format!("nota_{}_{}", ex.id, p);
                                ui.horizontal(|ui| {
                                    ui.add(
                                        egui::TextEdit::singleline(self.l6.input(&key))
                                            .hint_text(t.paper_grade_hint)
                                            .desired_width(60.0),
                                    );
                                    if ui.small_button(t.record_grade).clicked() {
                                        match exams::parse_number(&self.l6.take_input(&key)) {
                                            Some(x) => {
                                                let score = if x > 1.0 { x / 100.0 } else { x };
                                                let id = r.map_or_else(|| s.new_id(), |r| r.id);
                                                self.save_result(s, d, &ex, *p, id, score, &[]);
                                            }
                                            None => self.set_status(t.bad_grade, true),
                                        }
                                    }
                                });
                            } else {
                                ui.label("");
                            }
                            ui.end_row();
                        }
                    });
            });
        }
    }

    fn grade_answers(
        &mut self,
        s: &Session,
        d: &Data,
        ex: &exams::Exam,
        bank: &[exams::Question],
        handed: &[&exams::Answers],
        results: &[exams::ExamResult],
    ) {
        let mut n = 0;
        for a in handed {
            let Some(p) = a.person else { continue };
            match exams::grade(ex, bank, p, &a.answers) {
                Ok(g) => {
                    let id = results
                        .iter()
                        .find(|r| r.exam == Some(ex.id) && r.person == Some(p))
                        .map_or_else(|| s.new_id(), |r| r.id);
                    if self.save_result(s, d, ex, p, id, g.score, &g.items) {
                        n += 1;
                    }
                }
                Err(e) => self.set_status((texts().error)(&e.to_string()), true),
            }
        }
        self.set_status((l7().graded)(n), false);
    }

    /// Records a result in the grades folder and the trainee's own copy (score and
    /// result, without the key) in the class folder, where the trainee sees it.
    #[allow(clippy::too_many_arguments)]
    fn save_result(
        &mut self,
        s: &Session,
        d: &Data,
        ex: &exams::Exam,
        person: Id,
        id: Id,
        score: f64,
        detail: &[exams::ItemGrade],
    ) -> bool {
        let e = exams::record_result(id, ex, person, score, detail, d.me, d.today);
        if !self.write_training(s, d, TrainingFolder::Grades, e) {
            return false;
        }
        let Some(class) = ex.class else { return true };
        let copy = exams::record_result(id, ex, person, score, &[], d.me, d.today);
        self.write_training(s, d, TrainingFolder::Class(class), copy)
    }
}

fn problem_params(ui: &mut egui::Ui, p: &mut Problem) {
    let t = l7();
    ui.horizontal(|ui| match p {
        Problem::Mean { n, mean, sd } | Problem::StdDev { n, mean, sd } => {
            ui.label("n");
            ui.add(egui::DragValue::new(n).range(2..=500));
            ui.label(t.mean);
            ui.add(egui::DragValue::new(mean).speed(0.1));
            ui.label(t.sd);
            ui.add(egui::DragValue::new(sd).speed(0.01).range(0.001..=1e9));
        }
        Problem::Cpk {
            n,
            mean,
            sd,
            lsl,
            usl,
        }
        | Problem::Ppk {
            n,
            mean,
            sd,
            lsl,
            usl,
        } => {
            ui.label("n");
            ui.add(egui::DragValue::new(n).range(5..=500));
            ui.label(t.mean);
            ui.add(egui::DragValue::new(mean).speed(0.1));
            ui.label(t.sd);
            ui.add(egui::DragValue::new(sd).speed(0.01).range(0.001..=1e9));
            ui.label(t.lsl);
            ui.add(egui::DragValue::new(lsl).speed(0.1));
            ui.label(t.usl);
            ui.add(egui::DragValue::new(usl).speed(0.1));
        }
        Problem::Dpmo {
            defects,
            units,
            opportunities,
        } => {
            ui.label(t.defects);
            ui.add(egui::DragValue::new(defects).range(0..=1_000_000));
            ui.label(t.units);
            ui.add(egui::DragValue::new(units).range(1..=1_000_000));
            ui.label(t.opportunities);
            ui.add(egui::DragValue::new(opportunities).range(1..=1000));
        }
    });
}

/// The answers typed by a trainee, as values for the grading.
pub fn answers_value(typed: &BTreeMap<Id, String>, paper: &exams::Paper) -> BTreeMap<Id, Value> {
    let mut out = BTreeMap::new();
    for it in &paper.items {
        let Some(txt) = typed.get(&it.question) else {
            continue;
        };
        let v = match it.kind {
            QuestionKind::MultipleChoice => {
                txt.parse::<u64>().map(Value::from).unwrap_or(Value::Null)
            }
            QuestionKind::TrueFalse => match txt.as_str() {
                "true" => Value::from(true),
                "false" => Value::from(false),
                _ => Value::Null,
            },
            _ => Value::from(txt.clone()),
        };
        out.insert(it.question, v);
    }
    out
}
