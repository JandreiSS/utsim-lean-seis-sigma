//! The team, the roles and the RACI matrix (section 1.2).

use eframe::egui;
use utsim_lss_core::domain::{self, Person};
use utsim_lss_core::op::{EntityRef, Kind};
use utsim_lss_core::roles::{self, ALL_ROLES, Letter};

use super::super::App;
use super::{combo, person_combo};
use crate::session::Session;
use crate::text::l1::{l1, role_name};
use crate::theme;

impl App {
    pub(in crate::app) fn team(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let tl = l1();
        ui.heading(tl.nav_team);
        ui.weak(tl.team_intro);
        ui.add_space(6.0);
        // People.
        ui.strong(tl.people);
        let people = domain::people(s.state());
        let rs = roles::roles(s.state(), s.project());
        let mut remove = None;
        egui::Grid::new("pessoas")
            .num_columns(3)
            .striped(true)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                for p in &people {
                    ui.label(&p.name);
                    ui.weak(&p.email);
                    ui.horizontal_wrapped(|ui| {
                        for r in rs.iter().filter(|r| r.person == Some(p.id)) {
                            if ui
                                .small_button(format!("{} ✕", role_name(r.role)))
                                .on_hover_text(tl.remove_role)
                                .clicked()
                            {
                                remove = Some(r.id);
                            }
                        }
                    });
                    ui.end_row();
                }
            });
        if let Some(r) = remove {
            let res = s.delete(EntityRef::new(Kind::Role, r));
            self.report(res);
        }
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.l1.person_name)
                    .hint_text(tl.person_name_hint)
                    .desired_width(200.0),
            );
            ui.add(
                egui::TextEdit::singleline(&mut self.l1.person_email)
                    .hint_text(tl.person_email_hint)
                    .desired_width(220.0),
            );
            if ui.button(tl.add_person).clicked() {
                let (n, e) = (
                    self.l1.person_name.trim().to_string(),
                    self.l1.person_email.trim().to_string(),
                );
                if n.is_empty() {
                    self.set_status(tl.person_needs_name, true);
                } else {
                    let edits = Person::register(s.state(), s.new_id(), &n, &e);
                    if self.commit(s, edits) {
                        self.l1.person_name.clear();
                        self.l1.person_email.clear();
                    }
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label(tl.give_role);
            if let Some(v) = person_combo(ui, s, "papel_pessoa", self.l1.role_person) {
                self.l1.role_person = v;
            }
            if let Some(k) = combo(ui, "papel_tipo", self.l1.role_kind, &ALL_ROLES, role_name) {
                self.l1.role_kind = k;
            }
            if ui.button(tl.add).clicked()
                && let Some(p) = self.l1.role_person
            {
                let e = roles::add_role(s.state(), s.project(), s.new_id(), p, self.l1.role_kind);
                self.commit(s, e);
            }
        });
        ui.add_space(12.0);
        // RACI.
        ui.strong(tl.raci);
        ui.weak(tl.raci_intro);
        let m = roles::raci(s.state(), s.project());
        if m.roles.is_empty() {
            ui.weak(tl.raci_no_roles);
            return;
        }
        let phases = domain::phases(s.state(), s.project());
        let row_name = |r: &roles::Row| match r {
            roles::Row::Charter => tl.row_charter.to_string(),
            roles::Row::Tollgate(p) => (tl.row_tollgate)(
                &phases
                    .iter()
                    .find(|x| x.id == *p)
                    .map(|x| x.name.clone())
                    .unwrap_or_default(),
            ),
            roles::Row::Schedule => tl.row_schedule.to_string(),
            roles::Row::Risks => tl.row_risks.to_string(),
            roles::Row::Savings => tl.row_savings.to_string(),
            roles::Row::ControlPlan => tl.row_control_plan.to_string(),
            roles::Row::Closure => tl.row_closure.to_string(),
        };
        let mut change = None;
        egui::Grid::new("raci")
            .num_columns(m.roles.len() + 1)
            .striped(true)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                ui.label("");
                for k in &m.roles {
                    ui.strong(role_name(*k));
                }
                ui.end_row();
                for row in &m.rows {
                    ui.label(row_name(row));
                    for k in &m.roles {
                        let cur = m.get(row, *k);
                        let edited = m.edited.contains_key(&(row.clone(), *k));
                        let mut v = cur;
                        let text = cur.map_or("·", Letter::symbol);
                        let shown = if edited {
                            egui::RichText::new(text).strong().color(theme::accent(ui))
                        } else {
                            egui::RichText::new(text)
                        };
                        egui::ComboBox::from_id_salt(("raci", row.key(), *k))
                            .width(44.0)
                            .selected_text(shown)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut v, None, "·");
                                for l in Letter::ALL {
                                    ui.selectable_value(&mut v, Some(l), l.symbol());
                                }
                            });
                        if v != cur {
                            change = Some((row.clone(), *k, v));
                        }
                    }
                    ui.end_row();
                }
            });
        if let Some((row, k, v)) = change {
            let e = roles::set_cell(s.state(), s.project(), &row, k, v, s.new_id());
            self.commit(s, e);
        }
        let bad = m.rows_without_single_a();
        if !bad.is_empty() {
            let names: Vec<String> = bad.iter().map(row_name).collect();
            ui.colored_label(theme::WARN, (tl.raci_single_a)(&names.join(", ")));
        }
        self.integrity_section(ui, s);
    }
}
