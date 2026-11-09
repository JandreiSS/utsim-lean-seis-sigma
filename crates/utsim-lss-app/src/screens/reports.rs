//! The reports screen (stage L5, `docs/plano_lean_seis_sigma.md`, 4.6 and 4.7): PDF made by
//! the program of the charter, the A3, each tollgate, each meeting's minutes, each FMEA and
//! control plan; the project's spreadsheet (`.xlsx`, one sheet per table) and CSV; the
//! company's template of the reports (name, header, footer, colour) kept in the
//! `modelos/` folder next to the projects. The files go to the project's `relatorios/`.

use std::path::PathBuf;

use eframe::egui;
use utsim_lss_core::domain;
use utsim_lss_core::relatorios::{self, Document, Template, pdf};
use utsim_lss_core::tools::{self, Tool};
use utsim_lss_core::{Id, meeting};

use super::super::App;
use crate::reports as build;
use crate::session::Session;
use crate::text::l5::l5;
use crate::text::texts;

/// The template edited on the screen.
#[derive(Default)]
pub struct L5Ui {
    pub template: Option<Template>,
}

/// A file name from a title (no characters the file systems refuse).
pub fn file_name(title: &str, ext: &str) -> String {
    let base: String = crate::app::sanitize_folder_name(title)
        .chars()
        .map(|c| if c == ' ' { '-' } else { c })
        .collect();
    let base = if base.is_empty() {
        "relatorio".to_string()
    } else {
        base
    };
    format!("{base}.{ext}")
}

impl App {
    /// The template of the reports of this project.
    pub(in crate::app) fn report_template(&mut self, s: &Session) -> Template {
        if let Some(t) = &self.l5.template {
            return t.clone();
        }
        let t = s
            .folder()
            .report_template()
            .and_then(|txt| Template::from_json(&txt).ok())
            .unwrap_or_default();
        self.l5.template = Some(t.clone());
        t
    }

    fn footer(&self, s: &Session) -> String {
        (l5().printed_by)(&s.folder().me().name, &s.today().format_br())
    }

    /// Writes a PDF of `doc` into `relatorios/` and opens it.
    pub(in crate::app) fn print(&mut self, s: &Session, doc: &Document) -> Option<PathBuf> {
        let t = self.report_template(s);
        let mut doc = doc.clone();
        build::dress(
            &mut doc,
            &s.folder().me().name,
            &s.today().format_br(),
            &s.folder().title(),
        );
        let bytes = pdf::render(&doc, &t, &self.footer(s));
        self.save_report(s, &file_name(&doc.title, "pdf"), &bytes)
    }

    fn save_report(&mut self, s: &Session, name: &str, bytes: &[u8]) -> Option<PathBuf> {
        let r = s.folder().write_report(name, bytes);
        let p = self.report(r)?;
        let _ = super::model::open_with_system(&p);
        self.set_status((l5().saved_at)(&p.display().to_string()), false);
        Some(p)
    }

    pub(in crate::app) fn reports_screen(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = l5();
        ui.heading(t.nav_reports);
        ui.weak(t.intro);
        let st = s.state().clone();
        let p = s.project();
        ui.strong(t.documents);
        ui.horizontal_wrapped(|ui| {
            if ui.button(t.charter).clicked() {
                self.print(s, &build::charter_doc(&st, p));
            }
            if ui.button(t.a3).clicked() {
                self.print(s, &build::a3_doc(&st, p));
            }
        });
        ui.horizontal_wrapped(|ui| {
            ui.label(t.tollgates);
            for ph in domain::phases(&st, p) {
                if ui.button(&ph.name).clicked() {
                    self.print(s, &build::tollgate_doc(&st, p, ph.id));
                }
            }
        });
        let ms = meeting::meetings(&st, p);
        if !ms.is_empty() {
            ui.horizontal_wrapped(|ui| {
                ui.label(t.minutes);
                for m in ms.iter().take(12) {
                    let label = format!(
                        "{} {}",
                        m.date.map(|d| d.format_br()).unwrap_or_default(),
                        m.title
                    );
                    if ui.button(label).clicked() {
                        self.print(s, &build::minutes_doc(&st, p, m.id));
                    }
                }
            });
        }
        let dels = tools::deliverables(&st, p);
        ui.horizontal_wrapped(|ui| {
            for dl in &dels {
                let (doc, label) = match dl.tool {
                    Some(Tool::Fmea) => {
                        (Some(build::fmea_doc as fn(&_, Id, &_) -> Document), "FMEA")
                    }
                    Some(Tool::ControlPlan) => (
                        Some(build::control_doc as fn(&_, Id, &_) -> Document),
                        t.control_plan,
                    ),
                    Some(Tool::Vsm) => (Some(build::vsm_doc as fn(&_, Id, &_) -> Document), "VSM"),
                    _ => (None, ""),
                };
                if let Some(f) = doc
                    && ui.button(format!("{label}: {}", dl.name)).clicked()
                {
                    self.print(s, &f(&st, p, dl));
                }
            }
        });
        let an = utsim_lss_core::analises::analyses(&st, p);
        if !an.is_empty() {
            ui.horizontal_wrapped(|ui| {
                ui.label(crate::text::l10::l10().report_analyses);
                for a in an.iter().rev().take(20) {
                    let label = if a.name.trim().is_empty() {
                        a.recipe.as_ref().map_or("?", |r| r.label()).to_string()
                    } else {
                        a.name.clone()
                    };
                    if ui.button(label).clicked() {
                        self.print(s, &build::analysis_doc(&st, p, a));
                    }
                }
            });
        }
        ui.separator();
        ui.strong(t.spreadsheet);
        ui.horizontal(|ui| {
            let tables = build::project_tables(&st, p);
            if ui.button(t.xlsx).clicked() {
                let refs: Vec<utsim_lss_core::relatorios::TableRef<'_>> = tables
                    .iter()
                    .map(|(n, h, r)| (n.as_str(), h.as_slice(), r.as_slice()))
                    .collect();
                match utsim_lss_store::planilha::xlsx(&refs) {
                    Ok(b) => {
                        let title = st
                            .entity(p)
                            .map(|e| e.text(domain::fields::project::TITLE))
                            .unwrap_or_default();
                        self.save_report(s, &file_name(&title, "xlsx"), &b);
                    }
                    Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                }
            }
            if ui.button(t.csv).clicked() {
                let mut d = Document::new("tabelas", "");
                for (n, h, r) in &tables {
                    let hs: Vec<&str> = h.iter().map(String::as_str).collect();
                    d.table(n, &hs, r.clone(), &[]);
                }
                let c = relatorios::csv(&d);
                self.save_report(s, "tabelas.csv", c.as_bytes());
            }
        });
        ui.separator();
        self.template_section(ui, s);
    }

    fn template_section(&mut self, ui: &mut egui::Ui, s: &Session) {
        let t = l5();
        ui.strong(t.template);
        ui.weak(t.template_intro);
        let mut tp = self.report_template(s);
        let mut changed = false;
        egui::Grid::new("modelo_relatorio")
            .num_columns(2)
            .show(ui, |ui| {
                for (label, v) in [
                    (t.company, &mut tp.company),
                    (t.header, &mut tp.header),
                    (t.footer_label, &mut tp.footer),
                    (t.colour, &mut tp.colour),
                ] {
                    ui.label(label);
                    changed |= ui
                        .add(egui::TextEdit::singleline(v).desired_width(320.0))
                        .changed();
                    ui.end_row();
                }
            });
        if changed {
            self.l5.template = Some(tp.clone());
        }
        ui.horizontal(|ui| {
            if ui.button(t.save_template).clicked() {
                tp.version += 1;
                let r = s.folder().save_report_template(&tp.to_json());
                if let Some(p) = self.report(r) {
                    self.l5.template = Some(tp.clone());
                    self.set_status((t.saved_at)(&p.display().to_string()), false);
                }
            }
            if ui.button(t.preview).clicked() {
                let mut d = Document::new(t.preview_title, &tp.company);
                d.para(t.preview_text);
                self.print(s, &d);
            }
        });
    }
}
