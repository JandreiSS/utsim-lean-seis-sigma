//! Screens of stage L1, project management (`docs/plano_lean_seis_sigma.md`, 1.1 to 1.10):
//! portfolio, charter with versions, team and RACI, phases and tollgates, risks, meetings
//! and minutes, schedule with the Gantt, savings, closing and lessons, and the folder
//! picker. A child module of [`crate::app`], so the screens use the same text boxes (with
//! the version seen when typing started) and the same status bar. Every change goes
//! through [`Session::apply`], hence through the journal.

mod actions;
mod charter;
mod closure;
mod meetings;
pub mod picker;
mod portfolio;
mod risks;
mod savings;
mod schedule;
mod team;
mod tollgates;
// Stage L4, the UTSim model of the project.
pub mod model;
// Stage L5, reports.
pub mod reports;
// Stage L6, the implementation program.
pub mod program;

use std::collections::HashMap;

use eframe::egui;
use serde_json::Value;
use utsim_lss_core::{Date, Edit, EntityRef, Id, domain};

use super::{App, Draft};
use crate::session::Session;
use crate::text::l1::l1;

pub(crate) use portfolio::PortfolioUi;

/// What the L1 screens keep between frames (inputs being typed, selections).
#[derive(Default)]
pub struct L1Ui {
    pub(crate) picker: picker::FolderPicker,
    pub(crate) portfolio: PortfolioUi,
    // Charter.
    version_reason: String,
    compare: Option<Id>,
    // Team.
    person_name: String,
    person_email: String,
    role_person: Option<Id>,
    role_kind: domain::RoleKind,
    // Phases.
    phase: Option<Id>,
    decision: Option<utsim_lss_core::tollgate::Decision>,
    reservations: String,
    comment: String,
    item_text: String,
    item_check: String,
    // Risks.
    risk_text: String,
    risk: Option<Id>,
    // Meetings.
    meeting_title: String,
    meeting: Option<Id>,
    minutes_text: String,
    minutes_kind: utsim_lss_core::meeting::ItemKind,
    // Schedule.
    task_name: String,
    holiday_date: String,
    holiday_name: String,
    holiday_year: String,
    monte_carlo: Option<utsim_lss_core::schedule::McSummary>,
    // Savings.
    saving_text: String,
    saving_value: String,
    saving_soft: bool,
    saving_simulation: bool,
    saving: Option<Id>,
    entry_month: String,
    entry_value: String,
    // Lessons.
    lesson_title: String,
    search: String,
    /// Typed text of number and list boxes not bound to the generic text box.
    boxes: HashMap<(Id, String), String>,
}

impl L1Ui {
    /// Called when the project closes.
    pub fn close(&mut self) {
        self.portfolio.close();
        *self = L1Ui {
            picker: std::mem::take(&mut self.picker),
            ..L1Ui::default()
        };
    }
}

/// A number typed in Portuguese (`1.234,5`) or with a point (`1234.5`).
pub fn parse_number(s: &str) -> Option<f64> {
    let t = s.trim().replace(' ', "");
    if t.is_empty() {
        return None;
    }
    let t = if t.contains(',') {
        t.replace('.', "").replace(',', ".")
    } else {
        t
    };
    t.parse::<f64>().ok().filter(|x| x.is_finite())
}

/// A number for the screen, with a decimal comma and no trailing zeros.
pub fn show_number(x: f64) -> String {
    let s = format!("{:.4}", x);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    s.replace('.', ",")
}

/// A probability as a percentage.
pub fn percent(p: f64) -> String {
    format!("{} %", show_number((p * 1000.0).round() / 10.0))
}

impl App {
    /// Commits edits, showing the error in the status bar; whether it worked.
    pub(super) fn commit(&mut self, s: &mut Session, edits: Vec<Edit>) -> bool {
        if edits.is_empty() {
            return true;
        }
        let r = s.apply(edits);
        self.report(r).is_some()
    }

    /// Shows the error of a rule (`Err(e)`) or commits the edits.
    pub(super) fn commit_rule<E: std::fmt::Display>(
        &mut self,
        s: &mut Session,
        r: Result<Vec<Edit>, E>,
    ) -> bool {
        match r {
            Ok(e) => self.commit(s, e),
            Err(e) => {
                self.set_status((crate::text::texts().error)(&e.to_string()), true);
                false
            }
        }
    }

    /// A number box bound to a field (empty: no value). Committed when the focus leaves.
    pub(super) fn num_field(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        entity: EntityRef,
        field: &str,
        width: f32,
    ) {
        let current = s.state().entity(entity.id).and_then(|e| e.number(field));
        let shown = current.map(show_number).unwrap_or_default();
        let key = (entity.id, field.to_string());
        let (mut buf, seen) = match self.drafts.get(&key) {
            Some(d) => (d.text.clone(), d.seen),
            None => (
                shown.clone(),
                s.state().entity(entity.id).and_then(|e| e.version(field)),
            ),
        };
        let r = ui.add(egui::TextEdit::singleline(&mut buf).desired_width(width));
        if r.has_focus() {
            self.drafts.insert(
                key.clone(),
                Draft {
                    text: buf.clone(),
                    seen,
                },
            );
            self.editing = Some(entity);
        }
        if r.lost_focus() {
            self.drafts.remove(&key);
            if buf.trim() != shown {
                let v = if buf.trim().is_empty() {
                    Some(Value::Null)
                } else {
                    parse_number(&buf).map(Value::from)
                };
                match v {
                    Some(v) => {
                        let res = s.set_field(entity, field, v, Some(seen));
                        self.report(res);
                    }
                    None => self.set_status(l1().bad_number, true),
                }
            }
        }
    }

    /// A check box bound to a boolean field.
    pub(super) fn bool_field(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        entity: EntityRef,
        field: &str,
        label: &str,
    ) {
        let mut v = s
            .state()
            .entity(entity.id)
            .is_some_and(|e| e.boolean(field));
        if ui.checkbox(&mut v, label).changed() {
            let res = s.set_field(entity, field, Value::from(v), None);
            self.report(res);
        }
    }

    /// A list of people to choose one (or nobody); writes the id into `field`.
    pub(super) fn person_field(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        entity: EntityRef,
        field: &str,
    ) {
        let current = s.state().entity(entity.id).and_then(|e| e.id(field));
        if let Some(v) = person_combo(ui, s, (entity.id, field), current) {
            let value = v.map_or(Value::from(""), |p| Value::from(p.to_string()));
            let res = s.set_field(entity, field, value, None);
            self.report(res);
        }
    }

    /// A choice among the values of a serde enum stored in `field`.
    pub(super) fn enum_field<T>(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        entity: EntityRef,
        field: &str,
        options: &[T],
        name: impl Fn(T) -> &'static str,
    ) where
        T: Copy + PartialEq + Default + serde::Serialize + serde::de::DeserializeOwned,
    {
        let current: T = s
            .state()
            .entity(entity.id)
            .and_then(|e| e.typed(field))
            .unwrap_or_default();
        if let Some(v) = combo(ui, (entity.id, field), current, options, name) {
            let res = s.set_field(entity, field, domain::enum_value(v), None);
            self.report(res);
        }
    }

    /// A choice of 1 to 5 stored as an integer.
    pub(super) fn scale_field(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        entity: EntityRef,
        field: &str,
        values: &[i64],
    ) {
        let current = s.state().entity(entity.id).and_then(|e| e.integer(field));
        let mut v = current;
        egui::ComboBox::from_id_salt((entity.id, field))
            .width(52.0)
            .selected_text(v.map_or("-".to_string(), |x| x.to_string()))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut v, None, "-");
                for x in values {
                    ui.selectable_value(&mut v, Some(*x), x.to_string());
                }
            });
        if v != current {
            let value = v.map_or(Value::Null, Value::from);
            let res = s.set_field(entity, field, value, None);
            self.report(res);
        }
    }
}

/// A combo box over enum values; the new value when it changed.
pub fn combo<T: Copy + PartialEq>(
    ui: &mut egui::Ui,
    salt: impl std::hash::Hash + std::fmt::Debug,
    current: T,
    options: &[T],
    name: impl Fn(T) -> &'static str,
) -> Option<T> {
    let mut v = current;
    egui::ComboBox::from_id_salt(salt)
        .selected_text(name(v))
        .show_ui(ui, |ui| {
            for o in options {
                ui.selectable_value(&mut v, *o, name(*o));
            }
        });
    (v != current).then_some(v)
}

/// A combo box over the people of the project; the new choice when it changed.
pub fn person_combo(
    ui: &mut egui::Ui,
    s: &Session,
    salt: impl std::hash::Hash + std::fmt::Debug,
    current: Option<Id>,
) -> Option<Option<Id>> {
    let t = crate::text::texts();
    let people = domain::people(s.state());
    let mut v = current;
    let shown = v.map_or(t.nobody.to_string(), |p| domain::person_name(s.state(), p));
    egui::ComboBox::from_id_salt(salt)
        .selected_text(shown)
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut v, None, t.nobody);
            for p in &people {
                ui.selectable_value(&mut v, Some(p.id), &p.name);
            }
        });
    (v != current).then_some(v)
}

/// A date typed as `DD/MM/AAAA` (or `AAAA-MM-DD`).
pub fn parse_date(s: &str) -> Option<Date> {
    s.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_in_portuguese() {
        assert_eq!(parse_number("1.234,5"), Some(1234.5));
        assert_eq!(parse_number("1234.5"), Some(1234.5));
        assert_eq!(parse_number(" 12 "), Some(12.0));
        assert_eq!(parse_number(""), None);
        assert_eq!(parse_number("x"), None);
        assert_eq!(show_number(2.5), "2,5");
        assert_eq!(show_number(3.0), "3");
        assert_eq!(percent(0.8413), "84,1 %");
    }
}
