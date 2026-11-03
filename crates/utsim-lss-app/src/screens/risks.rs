//! The risk register (section 1.7): the 5 × 5 matrix, the list by level and the risk
//! panel with its response turned into a card.

use eframe::egui::{self, Color32, RichText};
use utsim_lss_core::board;
use utsim_lss_core::domain::{RiskResponse, RiskStatus, fields};
use utsim_lss_core::op::{EntityRef, Kind};
use utsim_lss_core::risk::{self, Band};

use super::super::App;
use crate::session::Session;
use crate::text::l1::{l1, response_name, risk_status_name};
use crate::text::{self, texts};

/// Colour of a matrix band.
pub(super) fn band_color(b: Band) -> Color32 {
    match b {
        Band::Low => Color32::from_rgb(70, 150, 90),
        Band::Medium => Color32::from_rgb(210, 170, 50),
        Band::High => Color32::from_rgb(200, 70, 60),
    }
}

impl App {
    pub(in crate::app) fn risks(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = texts();
        let tl = l1();
        ui.heading(t.nav_risks);
        ui.weak(tl.risks_intro);
        ui.add_space(6.0);
        let list = risk::risks(s.state(), s.project());
        ui.horizontal_top(|ui| {
            // Matrix: impact across, probability up.
            ui.vertical(|ui| {
                ui.strong(tl.risk_matrix);
                let m = risk::matrix(&list);
                egui::Grid::new("matriz_riscos")
                    .spacing([2.0, 2.0])
                    .show(ui, |ui| {
                        for p in (1..=5u8).rev() {
                            ui.label(p.to_string());
                            for i in 1..=5u8 {
                                let n = m[usize::from(p - 1)][usize::from(i - 1)];
                                let color = band_color(risk::band(p * i));
                                let txt = if n > 0 { n.to_string() } else { String::new() };
                                egui::Frame::new()
                                    .fill(color.gamma_multiply(if n > 0 { 1.0 } else { 0.35 }))
                                    .inner_margin(4.0)
                                    .show(ui, |ui| {
                                        ui.set_min_size(egui::vec2(26.0, 20.0));
                                        ui.label(RichText::new(txt).strong().color(Color32::WHITE));
                                    });
                            }
                            ui.end_row();
                        }
                        ui.label("");
                        for i in 1..=5u8 {
                            ui.label(i.to_string());
                        }
                        ui.end_row();
                    });
                ui.weak(tl.matrix_axes);
            });
            ui.add_space(16.0);
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.l1.risk_text)
                            .hint_text(tl.new_risk_hint)
                            .desired_width(320.0),
                    );
                    if ui.button(t.add).clicked() && !self.l1.risk_text.trim().is_empty() {
                        let id = s.new_id();
                        let e = risk::new_risk(s.project(), id, &self.l1.risk_text, 3, 3);
                        if self.commit(s, e) {
                            self.l1.risk_text.clear();
                            self.l1.risk = Some(id);
                        }
                    }
                });
                egui::Grid::new("lista_riscos")
                    .num_columns(4)
                    .striped(true)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        for r in &list {
                            let b = risk::band(r.level());
                            ui.colored_label(band_color(b), format!("{:>2}", r.level()));
                            let title = if r.description.is_empty() {
                                t.untitled
                            } else {
                                &r.description
                            };
                            if ui
                                .selectable_label(self.l1.risk == Some(r.id), title)
                                .clicked()
                            {
                                self.l1.risk = Some(r.id);
                            }
                            ui.label(risk_status_name(r.status));
                            ui.weak(
                                r.owner
                                    .map(|o| utsim_lss_core::domain::person_name(s.state(), o))
                                    .unwrap_or_default(),
                            );
                            ui.end_row();
                        }
                    });
            });
        });
        ui.separator();
        if self
            .l1
            .risk
            .is_none_or(|id| !list.iter().any(|r| r.id == id))
        {
            self.l1.risk = list.first().map(|r| r.id);
        }
        let Some(id) = self.l1.risk else {
            ui.weak(tl.pick_a_risk);
            return;
        };
        let r = EntityRef::new(Kind::Risk, id);
        self.presence_warning(ui, s, r);
        use fields::risk as f;
        egui::Grid::new("risco")
            .num_columns(2)
            .spacing([12.0, 6.0])
            .show(ui, |ui| {
                for (label, field, multi) in [
                    (tl.r_description, f::DESCRIPTION, false),
                    (tl.r_cause, f::CAUSE, true),
                    (tl.r_effect, f::EFFECT, true),
                ] {
                    ui.label(label);
                    self.text_field(ui, s, r, field, multi, 460.0);
                    ui.end_row();
                }
                ui.label(tl.r_probability);
                self.scale_field(ui, s, r, f::PROBABILITY, &[1, 2, 3, 4, 5]);
                ui.end_row();
                ui.label(tl.r_impact);
                self.scale_field(ui, s, r, f::IMPACT, &[1, 2, 3, 4, 5]);
                ui.end_row();
                ui.label(tl.r_response);
                self.enum_field(
                    ui,
                    s,
                    r,
                    f::RESPONSE,
                    &[
                        RiskResponse::Avoid,
                        RiskResponse::Mitigate,
                        RiskResponse::Transfer,
                        RiskResponse::Accept,
                    ],
                    response_name,
                );
                ui.end_row();
                ui.label(tl.r_plan);
                self.text_field(ui, s, r, risk::fields::RESPONSE_PLAN, true, 460.0);
                ui.end_row();
                ui.label(tl.r_owner);
                self.person_field(ui, s, r, f::OWNER);
                ui.end_row();
                ui.label(tl.r_review);
                self.date_field(ui, s, r, risk::fields::REVIEW);
                ui.end_row();
                ui.label(tl.r_status);
                self.enum_field(
                    ui,
                    s,
                    r,
                    f::STATUS,
                    &[RiskStatus::Open, RiskStatus::Closed],
                    risk_status_name,
                );
                ui.end_row();
            });
        let cards = board::cards_linked_to(s.state(), s.project(), id);
        ui.horizontal(|ui| {
            if ui.button(tl.response_to_card).clicked()
                && let Some(risk_v) = risk::risks(s.state(), s.project())
                    .into_iter()
                    .find(|x| x.id == id)
            {
                let plan = s
                    .state()
                    .entity(id)
                    .map(|e| e.text(risk::fields::RESPONSE_PLAN))
                    .filter(|p| !p.trim().is_empty())
                    .unwrap_or_else(|| (tl.response_card_title)(&risk_v.description));
                let e = risk::response_card(s.state(), s.project(), &risk_v, s.new_id(), &plan);
                if self.commit(s, e) {
                    self.set_status(tl.card_created, false);
                }
            }
            for c in &cards {
                ui.weak(format!("▸ {} ({})", c.title, text::column_name(c.column)));
            }
        });
        if ui
            .button(RichText::new(t.delete).color(ui.visuals().error_fg_color))
            .clicked()
        {
            let res = s.delete(r);
            self.report(res);
            self.l1.risk = None;
        }
    }
}
