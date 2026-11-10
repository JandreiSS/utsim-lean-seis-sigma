//! Phases and tollgates (section 1.4): the phases with their state; for the chosen phase,
//! the tollgate checklist (manual and verified items), the approvers and the rule, the
//! submission, the decisions with reservations and the frozen hash.

use eframe::egui::{self, RichText};
use utsim_lss_core::domain::{self, PhaseStatus, ProjectStatus, fields};
use utsim_lss_core::tollgate::{
    self, ApprovalRule, Check, Decision, DecisionInput, Item, ItemType, Outcome, Tollgate,
    TollgateStatus,
};

use super::super::App;
use super::charter::changes_table;
use super::{combo, person_combo};
use crate::session::Session;
use crate::text::l1::{check_name, decision_name, l1, rule_name, tollgate_status_name};
use crate::text::{self, texts};
use crate::theme;

/// Verified checks offered when adding an item by hand.
const CHECKS: [&str; 18] = [
    "termo_completo",
    "termo_aprovado",
    "equipe_definida",
    "cronograma_definido",
    "riscos_avaliados",
    "riscos_altos_tratados",
    "ganho_estimado",
    "ganhos_previstos",
    "ganhos_validados",
    "acoes_da_fase_concluidas",
    "licoes_registradas",
    "aceite_do_dono",
    "entregavel:fmea",
    "entregavel:voc_ctq",
    "entregavel:sipoc",
    "entregavel:ishikawa",
    "entregavel:plano_coleta",
    "entregavel:plano_controle",
];

impl App {
    pub(in crate::app) fn phases_l1(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = texts();
        let tl = l1();
        ui.heading(t.nav_phases);
        ui.weak(tl.phases_intro);
        let project = domain::Project::read(
            s.project(),
            s.state().entity(s.project()).expect("project entity"),
        );
        if matches!(
            project.status,
            ProjectStatus::Suspended | ProjectStatus::Cancelled
        ) {
            ui.colored_label(
                theme::WARN,
                (tl.project_is)(crate::text::l1::project_status_name(project.status)),
            );
        }
        let phases = domain::phases(s.state(), s.project());
        if self
            .l1
            .phase
            .is_none_or(|p| !phases.iter().any(|x| x.id == p))
        {
            self.l1.phase = project.current_phase.or(phases.first().map(|p| p.id));
        }
        ui.horizontal_wrapped(|ui| {
            for (i, ph) in phases.iter().enumerate() {
                let tg = tollgate::of_phase(s.state(), ph.id);
                let mut label = format!(
                    "{}. {} · {}",
                    i + 1,
                    ph.name,
                    text::phase_status_name(ph.status)
                );
                if let Some(tg) = &tg {
                    label.push_str(&format!(" · {}", tollgate_status_name(tg.status)));
                }
                let mut rt = RichText::new(label);
                if project.current_phase == Some(ph.id) {
                    rt = rt.strong().color(theme::accent(ui));
                }
                if ui
                    .selectable_label(self.l1.phase == Some(ph.id), rt)
                    .clicked()
                {
                    self.l1.phase = Some(ph.id);
                }
            }
        });
        ui.separator();
        let Some(phase) = self
            .l1
            .phase
            .and_then(|p| phases.iter().find(|x| x.id == p).cloned())
        else {
            return;
        };
        ui.heading((tl.tollgate_of)(&phase.name));
        let Some(tg) = tollgate::of_phase(s.state(), phase.id) else {
            ui.label(tl.no_tollgate);
            if ui.button(tl.prepare_tollgate).clicked() {
                let e = tollgate::prepare(s.state(), s.project(), phase.id, s.ids());
                self.commit(s, e);
            }
            return;
        };
        self.presence_warning(ui, s, Tollgate::entity_ref(tg.id));
        ui.label((tl.tollgate_state)(
            tollgate_status_name(tg.status),
            tg.round,
        ));
        if let Some(changes) = tollgate::changed_after_approval(s.state(), &tg) {
            if !changes.is_empty() {
                ui.colored_label(theme::WARN, tl.changed_after_approval);
                changes_table(ui, "tg_alterado", &changes);
            } else if !tg.frozen_hash.is_empty() {
                ui.weak((tl.frozen_hash)(
                    &tg.frozen_hash[..12.min(tg.frozen_hash.len())],
                ));
            }
        }
        ui.add_space(6.0);
        self.checklist(ui, s, &tg);
        ui.add_space(8.0);
        self.approvers(ui, s, &tg);
        ui.add_space(8.0);
        match tg.status {
            TollgateStatus::Submitted => self.decision_form(ui, s, &tg),
            TollgateStatus::Suspended => {
                if ui.button(tl.resume).clicked() {
                    let r = tollgate::resume(s.state(), s.project(), tg.id);
                    self.commit_rule(s, r);
                }
            }
            TollgateStatus::Cancelled => {}
            _ => {
                let label = if tg.status == TollgateStatus::Approved {
                    tl.resubmit
                } else {
                    tl.submit
                };
                if ui.button(label).clicked() {
                    let r = tollgate::submit(s.state(), s.project(), tg.id, s.today());
                    if self.commit_rule(s, r) {
                        self.set_status(tl.submitted, false);
                    }
                }
            }
        }
        ui.add_space(8.0);
        self.decision_history(ui, s, &tg);
        if phase.status == PhaseStatus::Approved {
            ui.weak(tl.phase_approved_note);
        }
    }

    fn checklist(&mut self, ui: &mut egui::Ui, s: &mut Session, tg: &Tollgate) {
        let tl = l1();
        ui.strong(tl.checklist);
        let results = tollgate::evaluate(s.state(), s.project(), tg);
        let mut remove = None;
        egui::Grid::new("checklist")
            .num_columns(4)
            .striped(true)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                for r in &results {
                    let it: &Item = &r.item;
                    let e = Item::entity_ref(it.id);
                    match it.item_type {
                        ItemType::Manual => {
                            self.bool_field(ui, s, e, fields::checklist_item::DONE, "");
                        }
                        ItemType::Verified => {
                            let (mark, color) = if r.ok {
                                ("✔", theme::accent(ui))
                            } else {
                                ("✖", theme::WARN)
                            };
                            ui.colored_label(color, mark).on_hover_text(
                                it.check.as_ref().map(check_name).unwrap_or_default(),
                            );
                        }
                    }
                    let mut label = RichText::new(&it.text);
                    if it.required {
                        label = label.strong();
                    }
                    ui.label(label).on_hover_text(match it.item_type {
                        ItemType::Manual => tl.item_manual,
                        ItemType::Verified => tl.item_verified,
                    });
                    if it.item_type == ItemType::Manual {
                        self.text_field(ui, s, e, tollgate::fields::EVIDENCE, false, 260.0);
                    } else {
                        ui.weak(it.check.as_ref().map(check_name).unwrap_or_default());
                    }
                    if ui.small_button("✖").on_hover_text(tl.remove_item).clicked() {
                        remove = Some(e);
                    }
                    ui.end_row();
                }
            });
        if let Some(e) = remove {
            let r = s.delete(e);
            self.report(r);
        }
        let missing = results.iter().filter(|r| r.item.required && !r.ok).count();
        if missing > 0 {
            ui.colored_label(theme::WARN, (tl.required_missing)(missing));
        }
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.l1.item_text)
                    .hint_text(tl.new_item_hint)
                    .desired_width(280.0),
            );
            let shown = Check::from_key(&self.l1.item_check)
                .map(|c| check_name(&c))
                .unwrap_or(tl.item_manual_choice.to_string());
            egui::ComboBox::from_id_salt("novo_item_tipo")
                .selected_text(shown)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.l1.item_check,
                        String::new(),
                        tl.item_manual_choice,
                    );
                    for k in CHECKS {
                        let c = Check::from_key(k).expect("known check");
                        ui.selectable_value(&mut self.l1.item_check, k.to_string(), check_name(&c));
                    }
                });
            if ui.button(tl.add).clicked() && !self.l1.item_text.trim().is_empty() {
                let order = results.iter().map(|r| r.item.order).fold(0.0, f64::max) + 1.0;
                let e = tollgate::item_edits(
                    tg.id,
                    s.new_id(),
                    self.l1.item_text.trim(),
                    true,
                    Check::from_key(&self.l1.item_check),
                    order,
                );
                if self.commit(s, e) {
                    self.l1.item_text.clear();
                }
            }
        });
    }

    fn approvers(&mut self, ui: &mut egui::Ui, s: &mut Session, tg: &Tollgate) {
        let tl = l1();
        ui.horizontal_wrapped(|ui| {
            ui.strong(tl.approvers);
            let mut list = tg.approvers.clone();
            let mut changed = false;
            for a in tg.approvers.iter() {
                if ui
                    .small_button(format!("{} ✕", domain::person_name(s.state(), *a)))
                    .clicked()
                {
                    list.retain(|x| x != a);
                    changed = true;
                }
            }
            if tg.approvers.is_empty() {
                ui.weak(tl.anyone_decides);
            }
            if let Some(Some(p)) = person_combo(ui, s, ("aprovador_novo", tg.id), None) {
                list.push(p);
                changed = true;
            }
            if changed {
                let e = tollgate::set_approvers(tg.id, &list);
                self.commit(s, vec![e]);
            }
        });
        ui.horizontal(|ui| {
            ui.label(tl.rule);
            if let Some(r) = combo(ui, ("regra", tg.id), tg.rule, &ApprovalRule::ALL, rule_name) {
                let res = s.set_field(
                    Tollgate::entity_ref(tg.id),
                    tollgate::fields::RULE,
                    domain::enum_value(r),
                    None,
                );
                self.report(res);
            }
            ui.label(tl.planned_date);
            self.date_field(
                ui,
                s,
                Tollgate::entity_ref(tg.id),
                tollgate::fields::PLANNED,
            );
        });
    }

    fn decision_form(&mut self, ui: &mut egui::Ui, s: &mut Session, tg: &Tollgate) {
        let tl = l1();
        if let Outcome::Pending { approvals, voters } =
            tollgate::outcome(s.state(), s.project(), tg)
        {
            ui.label((tl.pending_outcome)(approvals, voters));
        }
        theme::card(ui).show(ui, |ui| {
            ui.strong(tl.your_decision);
            let current = self.l1.decision.unwrap_or(Decision::Approve);
            if let Some(d) = combo(ui, "decisao", current, &Decision::ALL, decision_name) {
                self.l1.decision = Some(d);
            }
            let d = self.l1.decision.unwrap_or(Decision::Approve);
            if d == Decision::ApproveWithReservations {
                ui.label(tl.reservations_hint);
                ui.add(
                    egui::TextEdit::multiline(&mut self.l1.reservations)
                        .desired_rows(3)
                        .desired_width(420.0),
                );
            }
            ui.add(
                egui::TextEdit::singleline(&mut self.l1.comment)
                    .hint_text(tl.comment_hint)
                    .desired_width(420.0),
            );
            if ui.button(tl.record_decision).clicked() {
                let input = DecisionInput {
                    approver: s.me(),
                    decision: d,
                    reservations: self.l1.reservations.lines().map(String::from).collect(),
                    comment: self.l1.comment.clone(),
                    date: s.today(),
                };
                let r = tollgate::decide(s.state(), s.project(), tg.id, &input, s.ids());
                if self.commit_rule(s, r) {
                    self.l1.reservations.clear();
                    self.l1.comment.clear();
                    self.set_status(tl.decision_recorded, false);
                }
            }
        });
    }

    fn decision_history(&mut self, ui: &mut egui::Ui, s: &mut Session, tg: &Tollgate) {
        let tl = l1();
        let ds = tollgate::decisions(s.state(), tg.id);
        if ds.is_empty() {
            return;
        }
        ui.strong(tl.decisions);
        let project = s.project();
        egui::Grid::new("decisoes")
            .num_columns(6)
            .striped(true)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                for d in ds.iter().rev() {
                    ui.label(d.date.map(|x| x.format_br()).unwrap_or_default());
                    ui.label(
                        d.approver
                            .map(|p| domain::person_name(s.state(), p))
                            .unwrap_or_default(),
                    );
                    ui.label(d.decision.map(decision_name).unwrap_or("?"));
                    ui.weak(format!("{} {}", tl.round, d.round));
                    let mut txt = d.comment.clone();
                    if !d.reservations.is_empty() {
                        txt = format!("{txt} [{}]", d.reservations.join("; "));
                    }
                    ui.label(txt).on_hover_text((tl.frozen_hash)(&d.hash));
                    ui.horizontal(|ui| {
                        let a = utsim_lss_store::assinatura::tollgate_approval(project, d);
                        self.approval_seal(ui, s, a);
                    });
                    ui.end_row();
                }
            });
    }
}
