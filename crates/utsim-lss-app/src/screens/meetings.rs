//! Meetings and minutes (section 1.8): the list, the agenda generated from the open items,
//! attendance, notes and decisions, the minutes items and the cards created from them.

use eframe::egui::{self, RichText};
use serde_json::Value;
use utsim_lss_core::board;
use utsim_lss_core::domain::{self, fields};
use utsim_lss_core::meeting::{self, AgendaItem, ItemKind, MeetingKind};
use utsim_lss_core::op::{EntityRef, Kind};

use super::super::App;
use super::{combo, parse_date};
use crate::session::Session;
use crate::text::l1::{item_kind_name, l1, meeting_kind_name};
use crate::text::{self, texts};

/// One line of the generated agenda.
pub(super) fn agenda_line(i: &AgendaItem) -> String {
    let tl = l1();
    match i {
        AgendaItem::OverdueAction { title, due, .. } => (tl.ag_overdue)(title, &due.format_br()),
        AgendaItem::ActionDueSoon { title, due, .. } => (tl.ag_due_soon)(title, &due.format_br()),
        AgendaItem::HighRisk {
            description, level, ..
        } => (tl.ag_high_risk)(description, *level),
        AgendaItem::RiskReview { description, .. } => (tl.ag_risk_review)(description),
        AgendaItem::TollgateInReview { phase, .. } => (tl.ag_tollgate)(phase),
        AgendaItem::PendingMinutesAction { text, .. } => (tl.ag_pending_minutes)(text),
        AgendaItem::Conflicts(n) => (tl.ag_conflicts)(*n),
    }
}

impl App {
    pub(in crate::app) fn meetings(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = texts();
        let tl = l1();
        ui.heading(t.nav_meetings);
        ui.weak(tl.meetings_intro);
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.l1.meeting_title)
                    .hint_text(tl.new_meeting_hint)
                    .desired_width(320.0),
            );
            if ui.button(tl.new_meeting).clicked() {
                let title = if self.l1.meeting_title.trim().is_empty() {
                    (tl.meeting_default_title)(&s.today().format_br())
                } else {
                    self.l1.meeting_title.trim().to_string()
                };
                let id = s.new_id();
                let e = meeting::new_meeting(s.project(), id, &title, s.today(), MeetingKind::Team);
                if self.commit(s, e) {
                    self.l1.meeting_title.clear();
                    self.l1.meeting = Some(id);
                }
            }
        });
        let list = meeting::meetings(s.state(), s.project());
        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                ui.set_width(220.0);
                if list.is_empty() {
                    ui.weak(tl.no_meetings);
                }
                for m in &list {
                    let label = format!(
                        "{} {}",
                        m.date.map(|d| d.format_br()).unwrap_or_default(),
                        m.title
                    );
                    if ui
                        .selectable_label(self.l1.meeting == Some(m.id), label)
                        .clicked()
                    {
                        self.l1.meeting = Some(m.id);
                    }
                }
            });
            ui.separator();
            ui.vertical(|ui| {
                if self
                    .l1
                    .meeting
                    .is_none_or(|id| !list.iter().any(|m| m.id == id))
                {
                    self.l1.meeting = list.first().map(|m| m.id);
                }
                let Some(id) = self.l1.meeting else {
                    ui.weak(tl.pick_a_meeting);
                    return;
                };
                self.meeting_panel(ui, s, id);
            });
        });
    }

    fn meeting_panel(&mut self, ui: &mut egui::Ui, s: &mut Session, id: utsim_lss_core::Id) {
        let tl = l1();
        let t = texts();
        let r = EntityRef::new(Kind::Meeting, id);
        self.presence_warning(ui, s, r);
        use fields::meeting as f;
        egui::Grid::new("reuniao")
            .num_columns(2)
            .spacing([12.0, 6.0])
            .show(ui, |ui| {
                ui.label(tl.m_title);
                self.text_field(ui, s, r, f::TITLE, false, 420.0);
                ui.end_row();
                ui.label(tl.m_date);
                self.date_field(ui, s, r, f::DATE);
                ui.end_row();
                ui.label(tl.m_kind);
                self.enum_field(
                    ui,
                    s,
                    r,
                    meeting::fields::KIND,
                    &[
                        MeetingKind::Team,
                        MeetingKind::Tollgate,
                        MeetingKind::Portfolio,
                    ],
                    meeting_kind_name,
                );
                ui.end_row();
                ui.label(tl.m_present);
                ui.horizontal_wrapped(|ui| {
                    let present = meeting::present(s.state(), id);
                    for p in domain::people(s.state()) {
                        let mut yes = present.contains(&p.id);
                        if ui.checkbox(&mut yes, &p.name).changed()
                            && let Some(e) = meeting::set_present(s.state(), id, p.id, yes)
                        {
                            self.commit(s, vec![e]);
                        }
                    }
                });
                ui.end_row();
                ui.label(tl.m_agenda);
                ui.vertical(|ui| {
                    if ui.button(tl.generate_agenda).clicked() {
                        let day = s
                            .state()
                            .entity(id)
                            .and_then(|e| e.opt_text(f::DATE))
                            .and_then(|d| parse_date(&d))
                            .unwrap_or_else(|| s.today());
                        let items = meeting::agenda(s.state(), s.project(), day);
                        let text = if items.is_empty() {
                            tl.agenda_nothing.to_string()
                        } else {
                            meeting::agenda_text(&items, agenda_line)
                        };
                        let res = s.set_field(r, f::AGENDA, Value::from(text), None);
                        self.report(res);
                    }
                    self.text_field(ui, s, r, f::AGENDA, true, 420.0);
                });
                ui.end_row();
                ui.label(tl.m_notes);
                self.text_field(ui, s, r, f::NOTES, true, 420.0);
                ui.end_row();
                ui.label(tl.m_decisions);
                self.text_field(ui, s, r, f::DECISIONS, true, 420.0);
                ui.end_row();
            });
        ui.add_space(6.0);
        ui.strong(tl.minutes_items);
        let items = meeting::items(s.state(), id);
        egui::Grid::new("itens_ata")
            .num_columns(5)
            .striped(true)
            .spacing([8.0, 4.0])
            .show(ui, |ui| {
                for it in &items {
                    let e = EntityRef::new(Kind::MinutesItem, it.id);
                    ui.label(item_kind_name(it.kind));
                    self.text_field(ui, s, e, meeting::fields::TEXT, false, 260.0);
                    if it.kind == ItemKind::Action {
                        self.person_field(ui, s, e, meeting::fields::WHO);
                        self.date_field(ui, s, e, meeting::fields::DUE);
                        match it.action {
                            Some(c) => {
                                let col = s
                                    .state()
                                    .entity(c)
                                    .and_then(|x| x.typed(fields::action::COLUMN))
                                    .unwrap_or_default();
                                ui.weak((tl.card_in)(text::column_name(col)));
                            }
                            None => {
                                ui.weak(tl.no_card_yet);
                            }
                        }
                    } else {
                        ui.label("");
                        ui.label("");
                        ui.label("");
                    }
                    ui.end_row();
                }
            });
        ui.horizontal(|ui| {
            if let Some(k) = combo(
                ui,
                "tipo_item_ata",
                self.l1.minutes_kind,
                &[ItemKind::Note, ItemKind::Decision, ItemKind::Action],
                item_kind_name,
            ) {
                self.l1.minutes_kind = k;
            }
            let r = ui.add(
                egui::TextEdit::singleline(&mut self.l1.minutes_text)
                    .hint_text(tl.new_minutes_item_hint)
                    .desired_width(320.0),
            );
            let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if (ui.button(t.add).clicked() || enter) && !self.l1.minutes_text.trim().is_empty() {
                let e = meeting::add_item(
                    s.state(),
                    id,
                    s.new_id(),
                    self.l1.minutes_kind,
                    &self.l1.minutes_text,
                );
                if self.commit(s, e) {
                    self.l1.minutes_text.clear();
                }
            }
        });
        let pending = items
            .iter()
            .filter(|i| i.kind == ItemKind::Action && i.action.is_none())
            .count();
        if ui
            .add_enabled(
                pending > 0,
                egui::Button::new((tl.actions_to_cards)(pending)),
            )
            .clicked()
        {
            let e = meeting::actions_to_cards(s.state(), s.project(), id, s.me(), s.ids());
            if self.commit(s, e) {
                self.set_status((tl.cards_created)(pending), false);
            }
        }
        let cards = board::cards_linked_to(s.state(), s.project(), id);
        if !cards.is_empty() {
            ui.weak(RichText::new((tl.linked_cards)(cards.len())));
        }
    }
}
