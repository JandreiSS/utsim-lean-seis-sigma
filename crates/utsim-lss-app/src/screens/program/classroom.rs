//! Classes (8.5): the curriculum per level with the modules linked to the app's tools, the
//! instructors with their qualification and hours, and for the chosen class its sessions,
//! the attendance and the exercises done right (which feed `estudou` and `praticou`).

use eframe::egui;
use utsim_lss_core::Date;
use utsim_lss_core::formacao::{self, Attendance, curriculum};
use utsim_lss_core::mentoria;
use utsim_lss_core::perfis::Action;

use super::Data;
use super::training::{LEVELS, level_combo};
use crate::app::App;
use crate::session::Session;
use crate::text::l6::{level_name, tool_label};
use crate::text::l7::{attendance_name, l7};
use crate::text::texts;
use crate::theme;

impl App {
    pub(in crate::app) fn program_classroom(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
    ) {
        let t = l7();
        ui.heading(t.nav_classroom);
        ui.weak(t.classroom_intro);
        self.curriculum_section(ui, s, d);
        ui.separator();
        self.instructors_section(ui, s, d);
        ui.separator();
        self.class_picker(ui, d);
        if let Some(k) = d.tr.class(self.l6.l7.class).cloned() {
            self.sessions_section(ui, s, d, &k);
        }
    }

    pub(super) fn class_picker(&mut self, ui: &mut egui::Ui, d: &Data) {
        let t = l7();
        ui.horizontal(|ui| {
            ui.label(t.class);
            egui::ComboBox::from_id_salt("turma_escolhida")
                .selected_text(
                    d.tr.class(self.l6.l7.class)
                        .map_or_else(|| t.choose_class.to_string(), |k| k.name.clone()),
                )
                .show_ui(ui, |ui| {
                    for k in &d.tr.classes {
                        ui.selectable_value(
                            &mut self.l6.l7.class,
                            Some(k.id),
                            format!("{} · {}", k.name, level_name(k.level)),
                        );
                    }
                });
        });
    }

    fn curriculum_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let t = l7();
        ui.strong(t.curriculum);
        ui.weak(t.curriculum_intro);
        for level in LEVELS {
            let mods = curriculum::of_level(&d.tr.modules, level);
            ui.collapsing(
                (t.curriculum_of)(
                    level_name(level),
                    curriculum::hours_of(&d.tr.modules, level),
                ),
                |ui| {
                    if mods.is_empty()
                        && ui.button(t.install_curriculum).clicked()
                        && self.may_do(s, Action::ManageTraining, None)
                    {
                        let e = curriculum::install_default(d.program, level, s.ids());
                        let r = s.apply(e);
                        self.report(r);
                    }
                    for m in mods {
                        let r = curriculum::Module::entity_ref(m.id);
                        ui.horizontal(|ui| {
                            ui.label(format!("{}.", m.order));
                            self.text_field(ui, s, r, formacao::fields::NAME, false, 220.0);
                            ui.label(t.hours);
                            self.num_field(ui, s, r, formacao::fields::HOURS, 40.0);
                        });
                        self.text_field(ui, s, r, curriculum::fields::OBJECTIVES, true, 520.0);
                        if !m.prerequisites.is_empty() {
                            ui.weak((t.requires)(
                                &m.prerequisites
                                    .iter()
                                    .map(|p| d.tr.module_name(*p))
                                    .collect::<Vec<_>>()
                                    .join(", "),
                            ));
                        }
                        ui.horizontal_wrapped(|ui| {
                            ui.label(t.tools);
                            let mut tools = m.tools.clone();
                            let mut changed = false;
                            for key in mentoria::all_tools() {
                                let mut on = tools.contains(&key);
                                if ui.checkbox(&mut on, tool_label(&key)).changed() {
                                    changed = true;
                                    if on {
                                        tools.push(key.clone());
                                    } else {
                                        tools.retain(|x| *x != key);
                                    }
                                }
                            }
                            if changed && self.may_do(s, Action::ManageTraining, None) {
                                let r = s.apply(vec![curriculum::set_tools(m.id, &tools)]);
                                self.report(r);
                            }
                        });
                        ui.add_space(4.0);
                    }
                },
            );
        }
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(self.l6.input("modulo"))
                    .hint_text(t.new_module_hint)
                    .desired_width(240.0),
            );
            level_combo(ui, "nivel_modulo", &mut self.l6.l7.level);
            if ui.button(texts().add).clicked() && self.may_do(s, Action::ManageTraining, None) {
                let name = self.l6.take_input("modulo");
                let level = self.l6.l7.level;
                let order = curriculum::of_level(&d.tr.modules, level).len() as i64 + 1;
                if !name.trim().is_empty() {
                    let e = curriculum::add_module(
                        d.program,
                        s.new_id(),
                        level,
                        order,
                        &name,
                        4.0,
                        "",
                        &[],
                        &[],
                    );
                    let r = s.apply(e);
                    self.report(r);
                }
            }
        });
    }

    fn instructors_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let t = l7();
        ui.strong(t.instructors);
        let year_start = Date::new(d.today.year, 1, 1).unwrap_or(d.today);
        egui::Grid::new("instrutores").striped(true).show(ui, |ui| {
            for h in [t.person, t.qualified_for, t.hours_given, t.agenda] {
                ui.strong(h);
            }
            ui.end_row();
            for i in &d.tr.instructors {
                let Some(p) = i.person else { continue };
                ui.label(d.person(p));
                ui.label(
                    i.modules
                        .iter()
                        .map(|m| d.tr.module_name(*m))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                ui.label(format!(
                    "{:.1}",
                    formacao::hours_given(&d.tr.sessions, p, year_start, d.today)
                ));
                ui.label(formacao::agenda_of(&d.tr.sessions, p).len().to_string());
                ui.end_row();
            }
        });
        ui.horizontal_wrapped(|ui| {
            if let Some(v) = Self::member_combo(ui, d, "novo_instrutor", self.l6.l7.person, false) {
                self.l6.l7.person = v;
            }
            for m in &d.tr.modules {
                let mut on = self.l6.l7.instructor_modules.contains(&m.id);
                if ui.checkbox(&mut on, &m.name).changed() {
                    if on {
                        self.l6.l7.instructor_modules.push(m.id);
                    } else {
                        self.l6.l7.instructor_modules.retain(|x| *x != m.id);
                    }
                }
            }
            if ui.button(t.add_instructor).clicked()
                && self.may_do(s, Action::ManageTraining, None)
                && let Some(p) = self.l6.l7.person
            {
                let mods = std::mem::take(&mut self.l6.l7.instructor_modules);
                let e = formacao::add_instructor(d.program, s.new_id(), p, &mods);
                let r = s.apply(e);
                self.report(r);
            }
        });
        for (session, module) in formacao::unqualified(&d.tr.sessions, &d.tr.instructors) {
            let when =
                d.tr.sessions
                    .iter()
                    .find(|x| x.id == session)
                    .and_then(|x| x.date)
                    .map(Date::format_br)
                    .unwrap_or_default();
            ui.colored_label(
                theme::WARN,
                (t.unqualified)(&when, &d.tr.module_name(module)),
            );
        }
    }

    fn sessions_section(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
        k: &formacao::Class,
    ) {
        let t = l7();
        ui.horizontal(|ui| {
            ui.label(t.class_instructors);
            let mut chosen = k.instructors.clone();
            let mut changed = false;
            for p in d.tr.instructor_people() {
                let mut on = chosen.contains(&p);
                if ui.checkbox(&mut on, d.person(p)).changed() {
                    changed = true;
                    if on {
                        chosen.push(p);
                    } else {
                        chosen.retain(|x| *x != p);
                    }
                }
            }
            if changed && self.may_do(s, Action::ManageTraining, None) {
                let r = s.apply(vec![formacao::set_instructors(k.id, &chosen)]);
                self.report(r);
            }
        });
        let students = formacao::students(&d.tr.noms, k.id);
        let sessions = formacao::sessions_of(&d.tr.sessions, k.id);
        let mods = curriculum::of_level(&d.tr.modules, k.level);
        for (m, p) in curriculum::order_problems(
            &mods.iter().map(|m| (*m).clone()).collect::<Vec<_>>(),
            &sessions,
        ) {
            ui.colored_label(
                theme::WARN,
                (t.order_problem)(&d.tr.module_name(m), &d.tr.module_name(p)),
            );
        }
        ui.strong((t.sessions_of)(&k.name));
        egui::Grid::new("presenca").striped(true).show(ui, |ui| {
            ui.strong(t.person);
            for x in &sessions {
                ui.strong(x.date.map(Date::format_br).unwrap_or_default());
            }
            ui.strong(t.attendance);
            ui.end_row();
            ui.label("");
            for x in &sessions {
                ui.weak(
                    x.modules
                        .iter()
                        .map(|m| d.tr.module_name(*m))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
            }
            ui.label("");
            ui.end_row();
            for p in &students {
                ui.label(d.person(*p));
                for x in &sessions {
                    ui.horizontal(|ui| {
                        let cur = x.attendance.get(p).copied();
                        let mut v = cur;
                        egui::ComboBox::from_id_salt(("presenca", x.id, *p))
                            .width(90.0)
                            .selected_text(v.map_or("—", attendance_name))
                            .show_ui(ui, |ui| {
                                for a in Attendance::ALL {
                                    ui.selectable_value(&mut v, Some(a), attendance_name(a));
                                }
                            });
                        if v != cur
                            && self.may_do(s, Action::Teach, None)
                            && let Some(a) = v
                        {
                            let r = s.apply(vec![formacao::mark(x.id, *p, a)]);
                            self.report(r);
                        }
                        let mut ex = x.exercises.contains(p);
                        if ui
                            .checkbox(&mut ex, t.exercise_short)
                            .on_hover_text(t.exercise_tip)
                            .changed()
                            && self.may_do(s, Action::Teach, None)
                        {
                            let r = s.apply(vec![formacao::mark_exercise(x.id, *p, ex)]);
                            self.report(r);
                        }
                    });
                }
                let c = formacao::completion(k, &d.tr.sessions, *p);
                let line = format!("{:.0} %", c.rate * 100.0);
                if c.ok {
                    ui.label(line);
                } else {
                    ui.colored_label(theme::WARN, line);
                }
                ui.end_row();
            }
        });
        ui.horizontal_wrapped(|ui| {
            ui.add(
                egui::TextEdit::singleline(self.l6.input("data_sessao"))
                    .hint_text(texts().date_hint)
                    .desired_width(100.0),
            );
            ui.add(
                egui::TextEdit::singleline(self.l6.input("horas_sessao"))
                    .hint_text(t.hours)
                    .desired_width(50.0),
            );
            for m in &mods {
                let mut on = self.l6.l7.session_modules.contains(&m.id);
                if ui.checkbox(&mut on, &m.name).changed() {
                    if on {
                        self.l6.l7.session_modules.push(m.id);
                    } else {
                        self.l6.l7.session_modules.retain(|x| *x != m.id);
                    }
                }
            }
            if ui.button(t.add_session).clicked() && self.may_do(s, Action::Teach, None) {
                let date = self.l6.input("data_sessao").trim().parse::<Date>().ok();
                let hours =
                    crate::app::screens::parse_number(self.l6.input("horas_sessao")).unwrap_or(4.0);
                match date {
                    Some(dt) => {
                        let mods = std::mem::take(&mut self.l6.l7.session_modules);
                        let instr = k.instructors.first().copied().or(Some(d.me));
                        let e = formacao::new_session(k.id, s.new_id(), dt, hours, &mods, instr);
                        let r = s.apply(e);
                        if self.report(r).is_some() {
                            self.l6.take_input("data_sessao");
                        }
                    }
                    None => self.set_status(t.bad_date, true),
                }
            }
        });
    }
}
