//! Structure of the program (8.1): name and cadence, business units with their champion,
//! sponsor, headcount and portfolio, weighted strategic objectives deployed per unit, the
//! yearly targets of each unit, and what the scan found around the program.

use eframe::egui;
use serde_json::Value;
use utsim_lss_core::designacao;
use utsim_lss_core::op::{EntityRef, Kind};
use utsim_lss_core::perfis::Action;
use utsim_lss_core::programa::{self, BeltLevel, Metric, Unit, fields as pf};
use utsim_lss_core::{privacidade, revisao};

use super::{Data, L6Ui};
use crate::app::App;
use crate::session::Session;
use crate::text::l6::{l6, level_name, metric_name};
use crate::text::texts;
use crate::theme;

impl App {
    pub(in crate::app) fn program_structure(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
    ) {
        let tl = l6();
        ui.heading(tl.nav_structure);
        ui.weak(tl.structure_intro);
        let pr = EntityRef::new(Kind::Program, d.program);
        egui::Grid::new("programa").num_columns(2).show(ui, |ui| {
            ui.label(tl.program_name);
            self.text_field(ui, s, pr, pf::NAME, false, 360.0);
            ui.end_row();
            ui.label(tl.cadence);
            self.num_field(ui, s, pr, pf::CADENCE, 60.0);
            ui.end_row();
            ui.label(tl.limits);
            ui.horizontal(|ui| {
                for (label, f) in [
                    ("Y", designacao::fields::LIMIT_YELLOW),
                    ("G", designacao::fields::LIMIT_GREEN),
                    ("B", designacao::fields::LIMIT_BLACK),
                    (tl.mentees, designacao::fields::LIMIT_MENTEES),
                    (tl.champion_limit, designacao::fields::LIMIT_CHAMPION),
                ] {
                    ui.label(label);
                    self.num_field(ui, s, pr, f, 36.0);
                }
            });
            ui.end_row();
            ui.label(tl.review_target);
            self.num_field(ui, s, pr, revisao::fields::TARGET_MINUTES, 60.0);
            ui.end_row();
            ui.label(tl.retention);
            self.num_field(ui, s, pr, privacidade::RETENTION, 60.0);
            ui.end_row();
        });
        ui.weak((tl.limits_now)(
            d.limits.yellow,
            d.limits.green,
            d.limits.black,
            d.limits.mentees,
            d.limits.champion,
        ));
        ui.separator();
        self.units_section(ui, s, d);
        ui.separator();
        self.objectives_section(ui, s, d);
        ui.separator();
        self.targets_section(ui, s, d);
        ui.separator();
        ui.strong(tl.found_title);
        if d.scan.portfolios.is_empty() && d.records.is_empty() {
            ui.weak(tl.found_none);
        }
        for p in &d.scan.portfolios {
            ui.label((tl.found_portfolio)(&p.name, &d.unit_name(p.unit)));
        }
        for r in &d.records {
            let train = if r.training { tl.training_mark } else { "" };
            ui.label(format!(
                "{} · {} · {}{train}",
                r.title,
                d.unit_name(r.unit),
                crate::text::l1::project_status_name(r.status)
            ));
        }
        for p in &d.scan.unreadable {
            ui.colored_label(theme::WARN, (tl.unreadable)(&p.display().to_string()));
        }
    }

    fn units_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let tl = l6();
        ui.strong(tl.units);
        egui::Grid::new("unidades").striped(true).show(ui, |ui| {
            for h in [tl.unit, tl.champion, tl.sponsor, tl.headcount, tl.portfolio] {
                ui.strong(h);
            }
            ui.end_row();
            for u in &d.units {
                let r = Unit::entity_ref(u.id);
                self.text_field(ui, s, r, pf::NAME, false, 160.0);
                self.person_field(ui, s, r, pf::CHAMPION);
                self.person_field(ui, s, r, pf::SPONSOR);
                self.num_field(ui, s, r, pf::HEADCOUNT, 60.0);
                let current = u.portfolio;
                let mut v = current;
                egui::ComboBox::from_id_salt(("carteira_unidade", u.id))
                    .selected_text(
                        v.and_then(|c| d.scan.portfolios.iter().find(|p| p.id == c))
                            .map_or_else(|| tl.no_portfolio.to_string(), |p| p.name.clone()),
                    )
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut v, None, tl.no_portfolio);
                        for p in &d.scan.portfolios {
                            ui.selectable_value(&mut v, Some(p.id), &p.name);
                        }
                    });
                if v != current {
                    let value = v.map_or(Value::from(""), |c| Value::from(c.to_string()));
                    let res = s.set_field(r, pf::PORTFOLIO, value, None);
                    if self.report(res).is_some() {
                        self.l6.rescan();
                    }
                }
                ui.end_row();
            }
        });
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(self.l6.input("unidade"))
                    .hint_text(tl.new_unit_hint)
                    .desired_width(260.0),
            );
            if ui.button(texts().add).clicked() {
                let name = self.l6.take_input("unidade");
                if !name.trim().is_empty() {
                    let e = programa::add_unit(d.program, s.new_id(), &name);
                    self.commit(s, e);
                }
            }
        });
    }

    fn objectives_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let tl = l6();
        ui.strong(tl.objectives);
        ui.weak(tl.objectives_tip);
        let os = programa::objectives(s.state(), d.program);
        egui::Grid::new("objetivos").striped(true).show(ui, |ui| {
            ui.strong(tl.objective);
            ui.strong(tl.weight);
            for u in &d.units {
                ui.strong(&u.name);
            }
            ui.end_row();
            for o in &os {
                let r = EntityRef::new(Kind::StrategicObjective, o.id);
                self.text_field(ui, s, r, pf::NAME, false, 200.0);
                self.num_field(ui, s, r, pf::WEIGHT, 44.0);
                for u in &d.units {
                    self.num_field(ui, s, r, &programa::deploy_field(u.id), 44.0);
                }
                ui.end_row();
            }
        });
        for u in &d.units {
            let w = programa::unit_objectives(&os, u.id);
            if w.is_empty() {
                continue;
            }
            let parts: Vec<String> = w
                .iter()
                .map(|(id, x)| {
                    let name = os
                        .iter()
                        .find(|o| o.id == *id)
                        .map_or("", |o| o.name.as_str());
                    format!("{name} {:.0} %", x * 100.0)
                })
                .collect();
            ui.weak(format!("{}: {}", u.name, parts.join(" · ")));
        }
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(self.l6.input("objetivo"))
                    .hint_text(tl.new_objective_hint)
                    .desired_width(260.0),
            );
            if ui.button(texts().add).clicked() {
                let name = self.l6.take_input("objetivo");
                if !name.trim().is_empty() {
                    let e = programa::add_objective(d.program, s.new_id(), &name, 1.0);
                    self.commit(s, e);
                }
            }
        });
    }

    fn targets_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let tl = l6();
        let year = d.year(&self.l6);
        ui.horizontal(|ui| {
            ui.strong((tl.targets_of)(year));
            let mut y = year;
            if ui
                .add(egui::DragValue::new(&mut y).range(2000..=2100))
                .changed()
            {
                self.l6.year = Some(y);
            }
        });
        let ts: Vec<programa::Target> = programa::targets(s.state(), d.program)
            .into_iter()
            .filter(|t| t.year == year)
            .collect();
        if ts.is_empty() {
            ui.weak(tl.no_targets);
        }
        egui::Grid::new("metas").striped(true).show(ui, |ui| {
            for t in &ts {
                ui.label(d.unit_name(t.unit));
                ui.label(metric_name(t.metric));
                ui.label(t.level.map_or("", level_name));
                self.num_field(
                    ui,
                    s,
                    EntityRef::new(Kind::ProgramTarget, t.id),
                    pf::VALUE,
                    90.0,
                );
                ui.end_row();
            }
        });
        ui.horizontal(|ui| {
            if let Some(u) =
                Self::unit_combo(ui, d, "meta_unidade", self.l6.target_unit, tl.choose_unit)
            {
                self.l6.target_unit = u;
            }
            let ui_state: &mut L6Ui = &mut self.l6;
            egui::ComboBox::from_id_salt("meta_indicador")
                .selected_text(metric_name(ui_state.target_metric))
                .show_ui(ui, |ui| {
                    for m in Metric::ALL {
                        ui.selectable_value(&mut ui_state.target_metric, m, metric_name(m));
                    }
                });
            if ui_state.target_metric.per_level() {
                egui::ComboBox::from_id_salt("meta_nivel")
                    .selected_text(ui_state.target_level.map_or(tl.choose_level, level_name))
                    .show_ui(ui, |ui| {
                        for l in BeltLevel::ALL {
                            ui.selectable_value(&mut ui_state.target_level, Some(l), level_name(l));
                        }
                    });
            }
            ui.add(
                egui::TextEdit::singleline(ui_state.input("meta_valor"))
                    .hint_text(tl.value_hint)
                    .desired_width(100.0),
            );
            if ui.button(tl.set_target).clicked() {
                self.set_target(s, d, year);
            }
        });
    }

    fn set_target(&mut self, s: &mut Session, d: &Data, year: i32) {
        let tl = l6();
        let Some(unit) = self.l6.target_unit else {
            self.set_status(tl.choose_unit, true);
            return;
        };
        if !self.may_do(s, Action::SetTargets, None) {
            return;
        }
        let Some(v) = super::super::parse_number(self.l6.input("meta_valor")) else {
            self.set_status(crate::text::l1::l1().bad_number, true);
            return;
        };
        let r = programa::set_target(
            s.state(),
            d.program,
            s.new_id(),
            unit,
            year,
            self.l6.target_metric,
            self.l6.target_level,
            v,
        );
        if self.commit_rule(s, r) {
            self.l6.take_input("meta_valor");
        }
    }
}
