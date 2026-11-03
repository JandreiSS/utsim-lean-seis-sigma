//! What stage L1 adds to the action board (section 1.7): the WIP limits per column, and in
//! the card panel the origin, the entity the card came from and the effectiveness check.

use eframe::egui::{self, RichText};
use utsim_lss_core::board::{self, ColumnLoad};
use utsim_lss_core::domain::{Action, Column};
use utsim_lss_core::op::Kind;

use super::super::App;
use crate::session::Session;
use crate::text::l1::{l1, origin_name};
use crate::text::{self, texts};
use crate::theme;

impl App {
    /// The column header: name, cards and limit (in warning colour when over it).
    pub(in crate::app) fn column_header(&self, ui: &mut egui::Ui, load: &ColumnLoad) {
        let name = text::column_name(load.column);
        let txt = match load.limit {
            Some(l) => format!("{name} ({}/{l})", load.cards),
            None => format!("{name} ({})", load.cards),
        };
        if load.over() {
            ui.label(RichText::new(txt).strong().color(theme::WARN))
                .on_hover_text(l1().wip_over);
        } else {
            ui.strong(txt);
        }
    }

    /// The WIP limits, one box per column.
    pub(in crate::app) fn wip_limits(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let tl = l1();
        ui.collapsing(tl.wip_limits, |ui| {
            ui.weak(tl.wip_intro);
            ui.horizontal(|ui| {
                for c in Column::ALL {
                    ui.label(text::column_name(c));
                    let p = utsim_lss_core::domain::Project::entity_ref(s.project());
                    self.num_field(ui, s, p, &board::wip_field(c), 32.0);
                }
            });
        });
    }

    /// The L1 part of the card panel.
    pub(in crate::app) fn card_extras(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        card: &Action,
    ) {
        let tl = l1();
        let r = Action::entity_ref(card.id);
        ui.add_space(6.0);
        ui.label((tl.card_origin)(origin_name(card.origin)));
        if let Some(link) = board::link_of(s.state(), card.id) {
            let e = s.state().entity(link.id);
            let name = e
                .map(|e| {
                    let t = e.text("titulo");
                    if t.is_empty() { e.text("descricao") } else { t }
                })
                .unwrap_or_default();
            let kind = match link.kind {
                Kind::Risk => texts().nav_risks,
                Kind::Meeting => texts().nav_meetings,
                Kind::TollgateDecision => tl.tollgate_decision,
                other => other.key(),
            };
            ui.weak((tl.card_link)(kind, &name));
        }
        ui.add_space(6.0);
        ui.strong(tl.effectiveness);
        ui.label(tl.effectiveness_criterion);
        self.text_field(ui, s, r, board::fields::CHECK_CRITERION, false, 290.0);
        ui.label(tl.effectiveness_date);
        self.date_field(ui, s, r, board::fields::CHECK_DATE);
        self.bool_field(ui, s, r, board::fields::CHECKED, tl.effectiveness_ok);
    }
}
