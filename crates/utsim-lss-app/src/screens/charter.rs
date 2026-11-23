//! The charter with its versions (section 1.3): the fields, where it stands (approved,
//! needs a new approval), the versions with who approved them and the differences between
//! two versions or a version and the current text.

use crate::text::LocalDate;
use eframe::egui::{self, RichText};
use utsim_lss_core::charter::{self, CharterStatus, VersionStatus};
use utsim_lss_core::digest::{self, Change};
use utsim_lss_core::domain::{self, Project, fields};
use utsim_lss_core::schedule;

use super::super::App;
use crate::session::Session;
use crate::text::l1::{field_label, l1};
use crate::text::texts;
use crate::theme;

impl App {
    pub(in crate::app) fn charter_l1(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = texts();
        let tl = l1();
        let p = Project::entity_ref(s.project());
        ui.heading(t.nav_charter);
        ui.weak(tl.charter_intro);
        self.presence_warning(ui, s, p);
        ui.add_space(6.0);
        match charter::status(s.state(), s.project()) {
            CharterStatus::NoVersion => {
                ui.colored_label(theme::WARN, tl.charter_no_version);
            }
            CharterStatus::NotApproved => {
                ui.colored_label(theme::WARN, tl.charter_not_approved);
            }
            CharterStatus::Approved { minor_changes } => {
                ui.colored_label(theme::accent(ui), tl.charter_approved);
                if !minor_changes.is_empty() {
                    ui.weak((tl.charter_minor_changes)(minor_changes.len()));
                }
            }
            CharterStatus::NeedsReapproval { changes } => {
                ui.colored_label(theme::WARN, tl.charter_needs_reapproval);
                changes_table(ui, "reaprovar", &changes);
            }
        }
        ui.add_space(8.0);
        let w = 520.0;
        let t16 = crate::text::l16::l16();
        // O essencial à vista (os campos que o portão confere), com quantos já estão escritos.
        let faltam = charter::missing_fields(s.state(), s.project()).len();
        let total = charter::REQUIRED_FIELDS.len();
        ui.add(
            egui::ProgressBar::new((total - faltam) as f32 / total as f32)
                .desired_width(w)
                .text(crate::text::fx(
                    "l16-termo_essenciais",
                    &[
                        ("n", ((total - faltam) as i64).into()),
                        ("total", (total as i64).into()),
                    ],
                )),
        );
        ui.add_space(4.0);
        let mut campos = |ui: &mut egui::Ui, lista: &[&'static str], salt: &str| {
            egui::Grid::new(salt)
                .num_columns(2)
                .spacing([16.0, 8.0])
                .show(ui, |ui| {
                    for &f in lista {
                        ui.label(field_label(f));
                        if f == fields::project::DEADLINE || f == schedule::fields::START {
                            self.date_field(ui, s, p, f);
                        } else {
                            let multi = matches!(
                                f,
                                fields::project::PROBLEM
                                    | fields::project::SCOPE_IN
                                    | fields::project::SCOPE_OUT
                                    | fields::project::BUSINESS_CASE
                                    | charter::fields::ASSUMPTIONS
                                    | charter::fields::CONSTRAINTS
                                    | charter::fields::INITIAL_RISKS
                            );
                            self.text_field(ui, s, p, f, multi, w);
                        }
                        ui.end_row();
                    }
                });
        };
        campos(ui, &charter::REQUIRED_FIELDS, "termo_essencial");
        let outros: Vec<&'static str> = charter::CHARTER_FIELDS
            .iter()
            .copied()
            .filter(|f| !charter::REQUIRED_FIELDS.contains(f))
            .collect();
        egui::CollapsingHeader::new(t16.termo_mais_campos)
            .id_salt("termo_mais")
            .default_open(false)
            .show(ui, |ui| campos(ui, &outros, "termo_l1"));
        let versions = charter::versions(s.state(), s.project());
        // Com o essencial escrito e nenhuma versão, o próximo passo fica à vista.
        if faltam == 0 && versions.is_empty() {
            ui.add_space(8.0);
            ui.colored_label(theme::accent(ui), t16.termo_pronto);
            if ui
                .add(utsim_ui_tema::widgets::primary(t16.termo_pedir_aprovacao))
                .clicked()
            {
                let reason = std::mem::take(&mut self.l1.version_reason);
                let e = charter::new_version(s.state(), s.project(), s.new_id(), &reason);
                if self.commit(s, e) {
                    self.set_status(tl.version_recorded, false);
                }
            }
        }
        ui.add_space(8.0);
        let mut approve = None;
        let abrir_versoes = !versions.is_empty()
            && !matches!(
                charter::status(s.state(), s.project()),
                CharterStatus::Approved { .. }
            );
        egui::CollapsingHeader::new(t16.termo_versoes)
            .id_salt("termo_versoes")
            .default_open(abrir_versoes)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.l1.version_reason)
                            .hint_text(tl.version_reason_hint)
                            .desired_width(360.0),
                    );
                    if ui.button(tl.record_version).clicked() {
                        let reason = std::mem::take(&mut self.l1.version_reason);
                        let e = charter::new_version(s.state(), s.project(), s.new_id(), &reason);
                        if self.commit(s, e) {
                            self.set_status(tl.version_recorded, false);
                        }
                    }
                });
                if versions.is_empty() {
                    ui.weak(tl.no_versions);
                }
                let current = charter::current(s.state(), s.project());
                egui::Grid::new("versoes")
                    .num_columns(5)
                    .striped(true)
                    .spacing([12.0, 4.0])
                    .show(ui, |ui| {
                        for v in versions.iter().rev() {
                            ui.label(format!("v{}", v.number));
                            match v.status {
                                VersionStatus::Approved => ui.colored_label(
                                    theme::accent(ui),
                                    (tl.approved_by)(
                                        &v.approved_by
                                            .map(|p| domain::person_name(s.state(), p))
                                            .unwrap_or_default(),
                                        &v.approved_on.map(|d| d.local()).unwrap_or_default(),
                                    ),
                                ),
                                VersionStatus::Recorded => ui.label(tl.version_recorded_state),
                            };
                            ui.label(&v.reason);
                            if ui
                                .selectable_label(self.l1.compare == Some(v.id), tl.compare)
                                .clicked()
                            {
                                self.l1.compare = if self.l1.compare == Some(v.id) {
                                    None
                                } else {
                                    Some(v.id)
                                };
                            }
                            let latest = versions.last().map(|l| l.id) == Some(v.id);
                            if latest
                                && v.status == VersionStatus::Recorded
                                && ui.button(tl.approve_version).clicked()
                            {
                                approve = Some(v.id);
                            }
                            ui.end_row();
                        }
                    });
                if let Some(id) = self.l1.compare
                    && let Some(i) = versions.iter().position(|v| v.id == id)
                {
                    let v = &versions[i];
                    ui.add_space(6.0);
                    if i > 0 {
                        ui.strong((tl.diff_between)(versions[i - 1].number, v.number));
                        changes_table(
                            ui,
                            "dif_ant",
                            &charter::diff(&versions[i - 1].content, &v.content),
                        );
                    }
                    ui.strong((tl.diff_to_current)(v.number));
                    changes_table(ui, "dif_atual", &charter::diff(&v.content, &current));
                }
            });
        if let Some(v) = approve {
            let r = charter::approve(s.state(), s.project(), v, s.me(), s.today());
            if self.commit_rule(s, r) {
                self.set_status(tl.version_approved, false);
            }
        }
    }
}

/// The differences, field by field.
pub(super) fn changes_table(ui: &mut egui::Ui, salt: &str, changes: &[Change]) {
    let tl = l1();
    if changes.is_empty() {
        ui.weak(tl.no_differences);
        return;
    }
    egui::Grid::new(("mudancas", salt))
        .num_columns(3)
        .striped(true)
        .spacing([12.0, 4.0])
        .show(ui, |ui| {
            ui.strong(tl.field);
            ui.strong(tl.before);
            ui.strong(tl.after);
            ui.end_row();
            for c in changes {
                ui.label(field_label(&c.key));
                let show = |v: &Option<serde_json::Value>| {
                    v.as_ref().map(digest::show).unwrap_or_default()
                };
                ui.label(RichText::new(show(&c.before)).weak());
                ui.label(show(&c.after));
                ui.end_row();
            }
        });
}
