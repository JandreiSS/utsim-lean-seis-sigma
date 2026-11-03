//! Capacity and assignment (8.2): the program's people with level, unit, area, hours per
//! week and project limit; absences; the load of everyone summed over every portfolio,
//! with the overload warnings; and, for an opportunity selected or approved, the belts in
//! order with the reasons of each position. The MBB decides; the decision is recorded.

use eframe::egui::{self, RichText};
use utsim_lss_core::designacao::{self, AssignInput, Need, Overload, Reason};
use utsim_lss_core::domain::{ProjectStatus, enum_value, fields as dfields};
use utsim_lss_core::op::{EntityRef, Kind};
use utsim_lss_core::perfis::Action;
use utsim_lss_core::portfolio;
use utsim_lss_core::programa::{BeltLevel, Member, fields as pf};
use utsim_lss_core::{Edit, Id};

use super::Data;
use crate::app::App;
use crate::app::screens::parse_date;
use crate::session::Session;
use crate::text::l6::{l6, level_name};
use crate::text::texts;
use crate::theme;

impl App {
    pub(in crate::app) fn program_capacity(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
    ) {
        let tl = l6();
        ui.heading(tl.nav_capacity);
        ui.weak(tl.capacity_intro);
        self.people_section(ui, s, d);
        ui.separator();
        self.absences_section(ui, s, d);
        ui.separator();
        ui.strong(tl.loads);
        egui::Grid::new("cargas").striped(true).show(ui, |ui| {
            for h in [
                tl.person,
                tl.level,
                tl.projects,
                tl.units_count,
                tl.hours_committed,
                tl.capacity,
                tl.mentees,
            ] {
                ui.strong(h);
            }
            ui.end_row();
            for l in &d.loads {
                ui.label(d.person(l.person));
                ui.label(l.level.map_or(tl.no_level, level_name));
                let p = format!("{} / {}", l.projects.len(), l.project_limit);
                if l.over_projects() {
                    ui.colored_label(theme::WARN, p);
                } else {
                    ui.label(p);
                }
                ui.label(l.units.len().to_string());
                let h = format!("{:.1} h", l.hours);
                if l.over_hours() {
                    ui.colored_label(theme::WARN, h);
                } else {
                    ui.label(h);
                }
                ui.label(format!("{:.1} h", l.capacity));
                ui.label(l.mentees.to_string());
                ui.end_row();
            }
        });
        for o in designacao::overloads(&d.loads, &d.limits) {
            let text = match o {
                Overload::Projects { person, has, limit } => {
                    (tl.over_projects)(&d.person(person), has, limit)
                }
                Overload::Hours {
                    person,
                    hours,
                    capacity,
                } => (tl.over_hours)(&d.person(person), hours, capacity),
                Overload::Mentees { person, has, limit } => {
                    (tl.over_mentees)(&d.person(person), has, limit)
                }
                Overload::Champion { person, has, limit } => {
                    (tl.over_champion)(&d.person(person), has, limit)
                }
            };
            ui.colored_label(theme::WARN, text);
        }
        ui.small(tl.rule_load);
        ui.separator();
        self.assignment_section(ui, s, d);
    }

    fn people_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let tl = l6();
        ui.strong(tl.people);
        egui::Grid::new("pessoas_programa")
            .striped(true)
            .show(ui, |ui| {
                for h in [
                    tl.person,
                    tl.level,
                    tl.unit,
                    tl.area,
                    tl.hours_week,
                    tl.project_limit,
                    tl.needs_cert,
                    tl.trained_year,
                    tl.certified_on,
                ] {
                    ui.strong(h);
                }
                ui.end_row();
                for m in d.members.iter().filter(|m| !m.anonymised) {
                    let r = Member::entity_ref(m.id);
                    ui.label(&m.name);
                    let mut lv = m.level;
                    egui::ComboBox::from_id_salt(("nivel", m.id))
                        .selected_text(lv.map_or(tl.no_level, level_name))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut lv, None, tl.no_level);
                            for l in BeltLevel::ALL {
                                ui.selectable_value(&mut lv, Some(l), level_name(l));
                            }
                        });
                    if lv != m.level {
                        let v = lv.map_or(serde_json::Value::Null, enum_value);
                        let res = s.set_field(r, pf::LEVEL, v, None);
                        self.report(res);
                    }
                    if let Some(u) =
                        Self::unit_combo(ui, d, ("unidade_pessoa", m.id), m.unit, tl.no_unit)
                    {
                        let v = u.map_or(serde_json::Value::from(""), |u| u.to_string().into());
                        let res = s.set_field(r, pf::UNIT, v, None);
                        self.report(res);
                    }
                    self.text_field(ui, s, r, pf::AREA, false, 120.0);
                    self.num_field(ui, s, r, pf::HOURS_WEEK, 44.0);
                    self.num_field(ui, s, r, pf::PROJECT_LIMIT, 36.0);
                    self.bool_field(ui, s, r, pf::NEEDS_CERTIFICATION, "");
                    self.num_field(ui, s, r, pf::TRAINED_YEAR, 52.0);
                    self.date_field(ui, s, r, pf::CERTIFIED_ON);
                    ui.end_row();
                }
            });
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(self.l6.input("pessoa_nome"))
                    .hint_text(tl.person_name_hint)
                    .desired_width(200.0),
            );
            ui.add(
                egui::TextEdit::singleline(self.l6.input("pessoa_email"))
                    .hint_text(tl.person_email_hint)
                    .desired_width(200.0),
            );
            if ui.button(texts().add).clicked() {
                let name = self.l6.input("pessoa_nome").trim().to_string();
                let email = self.l6.input("pessoa_email").trim().to_string();
                if !name.is_empty() {
                    let id = s.new_id();
                    let r = Member::entity_ref(id);
                    let e = vec![
                        Edit::new(r, dfields::person::NAME, name),
                        Edit::new(r, dfields::person::EMAIL, email),
                    ];
                    if self.commit(s, e) {
                        self.l6.take_input("pessoa_nome");
                        self.l6.take_input("pessoa_email");
                    }
                }
            }
        });
    }

    fn absences_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let tl = l6();
        ui.strong(tl.absences);
        for a in &d.absences {
            ui.horizontal(|ui| {
                ui.label(format!(
                    "{}: {} a {}",
                    a.person.map_or(String::new(), |p| d.person(p)),
                    a.start.map_or(String::new(), |x| x.format_br()),
                    a.end.map_or(String::new(), |x| x.format_br()),
                ));
                ui.weak(&a.reason);
                if ui.small_button(texts().delete).clicked() {
                    let r = s.delete(EntityRef::new(Kind::Absence, a.id));
                    self.report(r);
                }
            });
        }
        ui.horizontal(|ui| {
            let cur = self.l6.absence_person.or(Some(d.me));
            if let Some(p) = Self::member_combo(ui, d, "ausencia_pessoa", cur, false) {
                self.l6.absence_person = p;
            }
            for (k, hint) in [
                ("aus_inicio", texts().date_hint),
                ("aus_fim", texts().date_hint),
                ("aus_motivo", tl.reason_hint),
            ] {
                ui.add(
                    egui::TextEdit::singleline(self.l6.input(k))
                        .hint_text(hint)
                        .desired_width(if k == "aus_motivo" { 180.0 } else { 100.0 }),
                );
            }
            if ui.button(tl.add_absence).clicked() {
                let person = self.l6.absence_person.unwrap_or(d.me);
                match (
                    parse_date(self.l6.input("aus_inicio")),
                    parse_date(self.l6.input("aus_fim")),
                ) {
                    (Some(a), Some(b)) => {
                        let reason = self.l6.input("aus_motivo").clone();
                        let e = designacao::add_absence(s.new_id(), person, a, b, &reason);
                        if self.commit(s, e) {
                            for k in ["aus_inicio", "aus_fim", "aus_motivo"] {
                                self.l6.take_input(k);
                            }
                        }
                    }
                    _ => self.set_status(texts().bad_date, true),
                }
            }
        });
    }

    fn assignment_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let tl = l6();
        ui.strong(tl.assignment);
        ui.weak(tl.assignment_intro);
        let candidates: Vec<&utsim_lss_core::programa::IdeaRecord> = d
            .ideas
            .iter()
            .filter(|i| {
                matches!(i.status, ProjectStatus::Approved | ProjectStatus::Screening)
                    && (i.status == ProjectStatus::Approved
                        || utsim_lss_core::programa::opportunity_of(&d.opps, i.id)
                            .is_some_and(|o| o.open()))
            })
            .collect();
        if candidates.is_empty() {
            ui.weak(tl.nothing_to_assign);
            return;
        }
        ui.horizontal_wrapped(|ui| {
            for i in &candidates {
                if ui
                    .selectable_label(self.l6.assign_idea == Some(i.id), &i.title)
                    .clicked()
                {
                    self.l6.assign_idea = Some(i.id);
                }
            }
        });
        let Some(idea) = self
            .l6
            .assign_idea
            .and_then(|x| candidates.iter().find(|i| i.id == x))
            .map(|i| (*i).clone())
        else {
            return;
        };
        let mut need = Need::of_idea(&idea);
        let tools_text = self.l6.input("ferramentas_previstas").clone();
        need.tools = tools_text
            .split(',')
            .map(|t| t.trim().to_lowercase())
            .filter(|t| !t.is_empty())
            .collect();
        ui.horizontal(|ui| {
            ui.label((tl.need)(level_name(need.level), need.hours_week));
            ui.add(
                egui::TextEdit::singleline(self.l6.input("ferramentas_previstas"))
                    .hint_text(tl.planned_tools_hint)
                    .desired_width(260.0),
            );
        });
        let ranked = designacao::rank(&need, &d.members, &d.loads, &d.map);
        let mut chosen = None;
        egui::Grid::new("ranking_belts")
            .striped(true)
            .show(ui, |ui| {
                ui.strong("#");
                ui.strong(tl.person);
                ui.strong(tl.reasons);
                ui.strong("");
                ui.end_row();
                for (pos, c) in ranked.iter().enumerate() {
                    ui.label((pos + 1).to_string());
                    let name = RichText::new(&c.name);
                    ui.label(if c.eligible {
                        name.strong()
                    } else {
                        name.weak()
                    });
                    ui.vertical(|ui| {
                        for r in &c.reasons {
                            let text = reason_text(r);
                            if r.favourable() {
                                ui.small(format!("+ {text}"));
                            } else {
                                ui.small(RichText::new(format!("− {text}")).color(theme::WARN));
                            }
                        }
                    });
                    if ui.button(tl.assign).clicked() {
                        chosen = Some((pos, c.person));
                    }
                    ui.end_row();
                }
            });
        ui.small(tl.rule_ranking);
        if let Some((pos, person)) = chosen {
            self.assign(s, d, &idea, &need, pos, person);
        }
    }

    fn assign(
        &mut self,
        s: &mut Session,
        d: &Data,
        idea: &utsim_lss_core::programa::IdeaRecord,
        need: &Need,
        pos: usize,
        person: Id,
    ) {
        let tl = l6();
        if !self.may_do(s, Action::Assign, None) {
            return;
        }
        let input = AssignInput {
            opportunity: utsim_lss_core::programa::opportunity_of(&d.opps, idea.id).map(|o| o.id),
            idea: Some(idea.id),
            person,
            hours_week: need.hours_week,
            mentor: None,
            position: Some(pos),
            reason: (tl.assigned_reason)(pos + 1),
            by: s.me(),
            date: d.today,
        };
        if !self.commit(s, designacao::assign(d.program, s.new_id(), &input)) {
            return;
        }
        // The belt of the idea in its portfolio.
        if let Some(path) = d
            .scan
            .portfolio_path(idea.portfolio)
            .map(|p| p.to_path_buf())
        {
            let e = vec![Edit::new(
                portfolio::Idea::entity_ref(idea.id),
                portfolio::fields::BELT,
                person.to_string(),
            )];
            self.write_folder(s, &path, e);
        }
        self.set_status((tl.assigned)(&d.person(person), &idea.title), false);
    }
}

fn reason_text(r: &Reason) -> String {
    let tl = l6();
    match r {
        Reason::Level { has, needs, .. } => (tl.r_level)(level_name(*has), level_name(*needs)),
        Reason::Slots { used, limit, .. } => (tl.r_slots)(*used, *limit),
        Reason::Hours { free, needs, .. } => (tl.r_hours)(*free, *needs),
        Reason::Away { days } => (tl.r_away)(*days),
        Reason::NeedsCertification => tl.r_cert.to_string(),
        Reason::SameUnit(true) => tl.r_same_unit.to_string(),
        Reason::SameUnit(false) => tl.r_other_unit.to_string(),
        Reason::SameArea => tl.r_same_area.to_string(),
        Reason::Experience { tools, of } => (tl.r_experience)(*tools, *of),
    }
}
