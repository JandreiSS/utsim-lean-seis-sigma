//! A abertura do app (sem projeto aberto), no desenho comum da família
//! ([`utsim_ui_tema::abertura`]): abrir um projeto, criar um projeto ou um programa,
//! importar um `.lssz`, os projetos recentes e onde tudo fica salvo. As pastas e os
//! arquivos são escolhidos no seletor do sistema (o explorador de arquivos no Windows).

use std::path::PathBuf;

use eframe::egui::{self, RichText};
use utsim_lss_core::domain::Approach;
use utsim_lss_store::{lssz, project};
use utsim_ui_tema::abertura::{Abertura, Acao, Escolha, Onde, Recente, Simbolo};
use utsim_ui_tema::arquivos::{self, Filtro, Tipo};

use super::super::{App, modelos_linha};
use crate::text::l14::l14;
use crate::text::{self, texts};

/// A cor do app (a mesma do lançador).
pub const COR: egui::Color32 = egui::Color32::from_rgb(0x4F, 0xD0, 0x6A);
/// O ícone do app (o mesmo do lançador).
pub const ICONE: &[u8] = include_bytes!("../../../../assets/icone/familia/lean-seis-sigma_256.png");

/// O filtro do seletor para os projetos exportados, no idioma da interface.
fn lssz_filtro() -> Filtro {
    Filtro {
        nome: l14().lssz_filter,
        extensoes: &[lssz::EXTENSION],
    }
}

/// A janela aberta por cima da abertura.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dialogo {
    NovoProjeto,
    NovoPrograma,
    Importar,
}

impl App {
    /// O corpo da abertura (o topo e a barra de estado ficam em `start_screen`).
    pub(in crate::app) fn start_body(&mut self, ui: &mut egui::Ui) {
        let t = texts();
        let l = l14();
        let acoes = [
            Acao::nova("abrir", Simbolo::Abrir, l.open_project, l.open_help).principal(),
            Acao::nova("novo", Simbolo::Novo, t.new_project, l.new_help),
            Acao::nova(
                "programa",
                Simbolo::Pessoas,
                text::l6::l6().create_program,
                l.program_help,
            ),
            Acao::nova(
                "importar",
                Simbolo::Importar,
                l.import_project,
                l.import_help,
            ),
        ];
        let recentes: Vec<Recente> = self.config.recent.iter().map(Recente::de).collect();
        let abertura = Abertura {
            nome: t.app_name,
            subtitulo: l.subtitle,
            cor: COR,
            icone: Some(ICONE),
            acoes: &acoes,
            titulo_recentes: l.recent_title,
            recentes: &recentes,
            onde: Onde {
                caminho: None,
                explicacao: l.where_explanation,
            },
            rotulos: l.start,
        };
        let escolha = abertura.mostrar(ui, |_| {}, |_| {});
        match escolha {
            Some(Escolha::Acao("abrir")) => {
                let inicio = self
                    .config
                    .recent
                    .first()
                    .and_then(|p| p.parent())
                    .map(|p| p.display().to_string())
                    .unwrap_or_default();
                if let Some(p) = arquivos::procurar_com(
                    Tipo::Pasta,
                    l.pick_project_folder,
                    &[],
                    &inicio,
                    &l.picker,
                ) {
                    if project::is_project(&p) {
                        self.open_project(&p);
                    } else {
                        self.set_status((l.not_a_project)(&p.display().to_string()), true);
                    }
                }
            }
            Some(Escolha::Acao("novo")) => {
                self.sugerir_pasta();
                self.l1.start_dialog = Some(Dialogo::NovoProjeto);
            }
            Some(Escolha::Acao("programa")) => {
                self.sugerir_pasta();
                self.l1.start_dialog = Some(Dialogo::NovoPrograma);
            }
            Some(Escolha::Acao("importar")) => {
                if let Some(p) = arquivos::procurar_com(
                    Tipo::Abrir,
                    l.pick_exported,
                    &[lssz_filtro()],
                    "",
                    &l.picker,
                ) {
                    self.import_file = p.display().to_string();
                    if self.import_dest.trim().is_empty() {
                        self.import_dest = p
                            .parent()
                            .map(|d| d.display().to_string())
                            .unwrap_or_default();
                    }
                    self.l1.start_dialog = Some(Dialogo::Importar);
                }
            }
            Some(Escolha::Recente(i)) => {
                if let Some(p) = self.config.recent.get(i).cloned() {
                    self.open_project(&p);
                }
            }
            Some(Escolha::Esquecer(i)) => {
                if i < self.config.recent.len() {
                    self.config.recent.remove(i);
                    self.save_config();
                }
            }
            Some(Escolha::MostrarPasta(p)) => {
                if let Err(e) = arquivos::mostrar_no_explorador(&p) {
                    self.set_status((l.explorer_error)(&e.to_string()), true);
                }
            }
            Some(Escolha::Acao(_)) | None => {}
        }
        self.start_dialog(ui.ctx());
    }

    /// A pasta onde criar, se ainda vazia: a do último projeto, senão Documentos.
    fn sugerir_pasta(&mut self) {
        if !self.new_parent.trim().is_empty() {
            return;
        }
        self.new_parent = self
            .config
            .recent
            .first()
            .and_then(|p| p.parent())
            .map(PathBuf::from)
            .or_else(arquivos::documentos)
            .map(|p| p.display().to_string())
            .unwrap_or_default();
    }

    fn start_dialog(&mut self, ctx: &egui::Context) {
        let Some(d) = self.l1.start_dialog else {
            return;
        };
        let t = texts();
        let l = l14();
        let mut fechar = false;
        egui::Modal::new(egui::Id::new("abertura_lss")).show(ctx, |ui| {
            ui.set_width(600.0);
            match d {
                Dialogo::NovoProjeto => {
                    ui.heading(t.new_project);
                    ui.add_space(8.0);
                    ui.label(l.project_title);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_title)
                            .hint_text(t.title_hint)
                            .desired_width(f32::INFINITY),
                    );
                    ui.add_space(6.0);
                    egui::Grid::new("novo_projeto")
                        .num_columns(2)
                        .show(ui, |ui| {
                            ui.label(t.project_model);
                            let chosen = self.new_model.and_then(modelos_linha::find);
                            egui::ComboBox::from_id_salt("modelo_projeto")
                                .selected_text(chosen.map_or(t.no_project_model, |m| m.name))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.new_model,
                                        None,
                                        t.no_project_model,
                                    );
                                    for m in &modelos_linha::MODELS {
                                        ui.selectable_value(
                                            &mut self.new_model,
                                            Some(m.id),
                                            m.name,
                                        )
                                        .on_hover_text(m.description);
                                    }
                                });
                            ui.end_row();
                            // A template brings its approach.
                            if let Some(m) = self.new_model.and_then(modelos_linha::find) {
                                self.new_approach = m.approach;
                            }
                            ui.label(t.approach);
                            let before = self.new_approach;
                            egui::ComboBox::from_id_salt("abordagem")
                                .selected_text(text::approach_name(self.new_approach))
                                .show_ui(ui, |ui| {
                                    for a in Approach::ALL {
                                        ui.selectable_value(
                                            &mut self.new_approach,
                                            a,
                                            text::approach_name(a),
                                        );
                                    }
                                });
                            if self.new_approach != before {
                                // Another approach leaves the template.
                                self.new_model = None;
                            }
                            ui.end_row();
                        });
                    onde_criar(ui, &mut self.new_parent, &self.new_title, ".lss");
                    ui.add_space(4.0);
                    ui.label(RichText::new(t.shared_folder_tip).small().weak());
                    ui.add_space(10.0);
                    let pronto =
                        !self.new_title.trim().is_empty() && !self.new_parent.trim().is_empty();
                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled(pronto, utsim_ui_tema::widgets::primary(t.create))
                            .clicked()
                        {
                            let (parent, title) = (
                                PathBuf::from(self.new_parent.trim()),
                                self.new_title.clone(),
                            );
                            let created = match self.new_model.and_then(modelos_linha::find) {
                                Some(m) => self.create_line_project(&parent, &title, m),
                                None => self.create_project(&parent, &title, self.new_approach),
                            };
                            if created {
                                self.new_title.clear();
                                self.new_model = None;
                                fechar = true;
                            }
                        }
                        if ui.button(l.cancel).clicked() {
                            fechar = true;
                        }
                    });
                }
                Dialogo::NovoPrograma => {
                    let t6 = text::l6::l6();
                    ui.heading(t6.create_program);
                    ui.add_space(8.0);
                    ui.label(l.program_name);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_program)
                            .hint_text(t6.program_name_hint)
                            .desired_width(f32::INFINITY),
                    );
                    onde_criar(ui, &mut self.new_parent, &self.new_program, ".lss");
                    ui.add_space(4.0);
                    ui.label(RichText::new(t6.program_tip).small().weak());
                    ui.add_space(10.0);
                    let pronto =
                        !self.new_program.trim().is_empty() && !self.new_parent.trim().is_empty();
                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled(pronto, utsim_ui_tema::widgets::primary(t.create))
                            .clicked()
                        {
                            let (parent, name) = (
                                PathBuf::from(self.new_parent.trim()),
                                self.new_program.clone(),
                            );
                            if self.create_program(&parent, &name) {
                                self.new_program.clear();
                                fechar = true;
                            }
                        }
                        if ui.button(l.cancel).clicked() {
                            fechar = true;
                        }
                    });
                }
                Dialogo::Importar => {
                    ui.heading(l.import_project);
                    ui.add_space(8.0);
                    ui.label(l.exported_file);
                    arquivos::campo_com(
                        ui,
                        &mut self.import_file,
                        Tipo::Abrir,
                        l.pick_exported,
                        &[lssz_filtro()],
                        440.0,
                        &l.picker,
                    );
                    ui.add_space(6.0);
                    ui.label(l.import_where);
                    arquivos::campo_com(
                        ui,
                        &mut self.import_dest,
                        Tipo::Pasta,
                        l.import_where_title,
                        &[],
                        440.0,
                        &l.picker,
                    );
                    ui.add_space(10.0);
                    let pronto =
                        !self.import_file.trim().is_empty() && !self.import_dest.trim().is_empty();
                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled(pronto, utsim_ui_tema::widgets::primary(t.import))
                            .clicked()
                        {
                            let (file, dest) = (
                                PathBuf::from(self.import_file.trim()),
                                PathBuf::from(self.import_dest.trim()),
                            );
                            let r = lssz::import(&file, &dest);
                            if let Some(root) = self.report(r) {
                                self.set_status((t.imported)(&root.display().to_string()), false);
                                self.open_project(&root);
                                fechar = true;
                            }
                        }
                        if ui.button(l.cancel).clicked() {
                            fechar = true;
                        }
                    });
                }
            }
            if let Some((msg, true)) = &self.status {
                ui.colored_label(ui.visuals().error_fg_color, msg);
            }
        });
        if fechar || self.session.is_some() {
            self.l1.start_dialog = None;
        }
    }
}

/// "Onde criar": a pasta (com "Procurar…") e a pasta que vai ser criada.
fn onde_criar(ui: &mut egui::Ui, pasta: &mut String, nome: &str, ext: &str) {
    ui.add_space(8.0);
    let l = l14();
    ui.label(l.create_where);
    arquivos::campo_com(
        ui,
        pasta,
        Tipo::Pasta,
        l.create_where_title,
        &[],
        440.0,
        &l.picker,
    );
    if !nome.trim().is_empty() && !pasta.trim().is_empty() {
        let p = PathBuf::from(pasta.trim()).join(format!(
            "{}{ext}",
            super::super::sanitize_folder_name(nome).trim_end_matches(ext)
        ));
        ui.label(
            RichText::new((l.will_create)(&p.display().to_string()))
                .color(ui.visuals().weak_text_color()),
        );
    }
}
