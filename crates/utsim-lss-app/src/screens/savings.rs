//! Savings (section 1.10, decision G1): hard and soft with the default definitions, the
//! states with the controller's validation, the monthly entries and the follow-up.

use eframe::egui::{self, RichText};
use utsim_lss_core::domain::{SavingCategory, SavingStatus, fields};
use utsim_lss_core::op::{EntityRef, Kind};
use utsim_lss_core::savings::{self, Category, Month, Source};

use super::super::App;
use super::parse_number;
use crate::session::Session;
use crate::text::l1::{category_name, l1, source_name};
use crate::text::{self, texts};
use crate::theme;

const STATES: [SavingStatus; 4] = [
    SavingStatus::Estimated,
    SavingStatus::Forecast,
    SavingStatus::Realized,
    SavingStatus::Validated,
];

impl App {
    pub(in crate::app) fn savings(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = texts();
        let tl = l1();
        ui.heading(t.nav_savings);
        ui.weak(tl.savings_intro);
        ui.collapsing(tl.definitions, |ui| {
            ui.label(RichText::new(tl.hard).strong());
            ui.label(tl.hard_definition);
            ui.label(RichText::new(tl.soft).strong());
            ui.label(tl.soft_definition);
            ui.weak((tl.follow_up_default)(savings::DEFAULT_FOLLOW_UP_MONTHS));
        });
        // Totals.
        let tot = savings::totals(s.state(), s.project());
        egui::Grid::new("totais_ganhos")
            .num_columns(5)
            .striped(true)
            .spacing([14.0, 4.0])
            .show(ui, |ui| {
                ui.label("");
                for st in STATES {
                    ui.strong(text::saving_status_name(st));
                }
                ui.end_row();
                for c in [Category::Hard, Category::Soft] {
                    ui.strong(if c == Category::Hard {
                        tl.hard
                    } else {
                        tl.soft
                    });
                    for st in STATES {
                        ui.label(text::money(tot.get(c, st)));
                    }
                    ui.end_row();
                }
            });
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.l1.saving_text)
                    .hint_text(tl.new_saving_hint)
                    .desired_width(260.0),
            );
            ui.add(
                egui::TextEdit::singleline(&mut self.l1.saving_value)
                    .hint_text(tl.value_hint)
                    .desired_width(100.0),
            );
            ui.checkbox(&mut self.l1.saving_soft, tl.soft);
            ui.checkbox(&mut self.l1.saving_simulation, tl.from_simulation);
            if ui.button(t.add).clicked() && !self.l1.saving_text.trim().is_empty() {
                let value = parse_number(&self.l1.saving_value).unwrap_or(0.0);
                let cat = if self.l1.saving_soft {
                    SavingCategory::Soft
                } else {
                    SavingCategory::Hard
                };
                let src = if self.l1.saving_simulation {
                    Source::Simulation
                } else {
                    Source::Belt
                };
                let id = s.new_id();
                let e = savings::new_saving(s.project(), id, &self.l1.saving_text, cat, value, src);
                if self.commit(s, e) {
                    self.l1.saving_text.clear();
                    self.l1.saving_value.clear();
                    self.l1.saving = Some(id);
                }
            }
        });
        let plans = savings::plans(s.state(), s.project());
        egui::Grid::new("ganhos")
            .num_columns(5)
            .striped(true)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                for p in &plans {
                    let title = if p.saving.description.is_empty() {
                        t.untitled
                    } else {
                        &p.saving.description
                    };
                    if ui
                        .selectable_label(self.l1.saving == Some(p.saving.id), title)
                        .clicked()
                    {
                        self.l1.saving = Some(p.saving.id);
                    }
                    ui.label(category_name(p.saving.category));
                    ui.label(text::saving_status_name(p.saving.status));
                    ui.label(text::money(p.saving.value));
                    ui.weak(source_name(p.source));
                    ui.end_row();
                }
            });
        ui.separator();
        if self
            .l1
            .saving
            .is_none_or(|id| !plans.iter().any(|p| p.saving.id == id))
        {
            self.l1.saving = plans.first().map(|p| p.saving.id);
        }
        let Some(plan) = self
            .l1
            .saving
            .and_then(|id| plans.iter().find(|p| p.saving.id == id).cloned())
        else {
            ui.weak(tl.pick_a_saving);
            return;
        };
        let id = plan.saving.id;
        let r = EntityRef::new(Kind::Saving, id);
        self.presence_warning(ui, s, r);
        use fields::saving as f;
        egui::Grid::new("ganho")
            .num_columns(2)
            .spacing([12.0, 6.0])
            .show(ui, |ui| {
                ui.label(tl.s_description);
                self.text_field(ui, s, r, f::DESCRIPTION, false, 420.0);
                ui.end_row();
                ui.label(tl.s_category);
                self.enum_field(
                    ui,
                    s,
                    r,
                    f::CATEGORY,
                    &[SavingCategory::Hard, SavingCategory::Soft],
                    category_name,
                );
                ui.end_row();
                ui.label(tl.s_value);
                self.number_field(ui, s, r, f::VALUE, 120.0);
                ui.end_row();
                ui.label(tl.s_rationale);
                self.text_field(ui, s, r, f::RATIONALE, true, 420.0);
                ui.end_row();
                ui.label(tl.s_monthly);
                self.number_field(ui, s, r, savings::fields::MONTHLY, 120.0);
                ui.end_row();
                ui.label(tl.s_start);
                self.text_field(ui, s, r, savings::fields::START, false, 90.0);
                ui.end_row();
                ui.label(tl.s_months);
                self.number_field(ui, s, r, savings::fields::MONTHS, 60.0);
                ui.end_row();
            });
        // State moves.
        ui.horizontal(|ui| {
            let pos = STATES
                .iter()
                .position(|x| *x == plan.saving.status)
                .unwrap_or(0);
            if let Some(next) = STATES.get(pos + 1)
                && ui
                    .button((tl.move_saving_to)(text::saving_status_name(*next)))
                    .clicked()
            {
                let res = savings::transition(s.state(), s.project(), id, *next, s.me(), s.today());
                self.commit_rule(s, res);
            }
            if let Some(prev) = pos.checked_sub(1).map(|i| STATES[i])
                && ui
                    .button((tl.back_saving_to)(text::saving_status_name(prev)))
                    .clicked()
            {
                let res = savings::transition(s.state(), s.project(), id, prev, s.me(), s.today());
                self.commit_rule(s, res);
            }
        });
        if let (Some(by), Some(on)) = (plan.forecast_by, plan.forecast_on) {
            ui.weak((tl.forecast_by)(
                &utsim_lss_core::domain::person_name(s.state(), by),
                &on.format_br(),
            ));
        }
        if let (Some(by), Some(on)) = (plan.validated_by, plan.validated_on) {
            ui.weak((tl.validated_by)(
                &utsim_lss_core::domain::person_name(s.state(), by),
                &on.format_br(),
            ));
        }
        ui.add_space(6.0);
        ui.strong(tl.monthly_entries);
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.l1.entry_month)
                    .hint_text(tl.month_hint)
                    .desired_width(90.0),
            );
            ui.add(
                egui::TextEdit::singleline(&mut self.l1.entry_value)
                    .hint_text(tl.value_hint)
                    .desired_width(100.0),
            );
            if ui.button(tl.record_month).clicked() {
                match (
                    self.l1.entry_month.parse::<Month>(),
                    parse_number(&self.l1.entry_value),
                ) {
                    (Ok(m), Some(v)) => {
                        let e = savings::record_month(s.state(), id, m, v, s.new_id());
                        if self.commit(s, e) {
                            self.l1.entry_value.clear();
                            self.l1.entry_month = m.plus(1).format_br();
                        }
                    }
                    _ => self.set_status(tl.bad_month_or_value, true),
                }
            }
        });
        let entries = savings::entries(s.state(), id);
        let rows = savings::follow_up(&plan, &entries);
        if rows.is_empty() {
            ui.weak(tl.no_follow_up);
        } else {
            egui::Grid::new("acompanhamento")
                .num_columns(5)
                .striped(true)
                .spacing([12.0, 3.0])
                .show(ui, |ui| {
                    for h in [
                        tl.month,
                        tl.expected,
                        tl.realized,
                        tl.cumulative_expected,
                        tl.cumulative_realized,
                    ] {
                        ui.strong(h);
                    }
                    ui.end_row();
                    for row in &rows {
                        ui.label(row.month.format_br());
                        ui.label(text::money(row.expected));
                        match row.realized {
                            Some(v) => {
                                let txt = text::money(v);
                                if row.validated {
                                    ui.colored_label(theme::accent(ui), format!("{txt} ✔"));
                                } else {
                                    ui.label(txt);
                                }
                            }
                            None => {
                                ui.weak("-");
                            }
                        }
                        ui.label(text::money(row.cumulative_expected));
                        ui.label(text::money(row.cumulative_realized));
                        ui.end_row();
                    }
                });
        }
    }
}
