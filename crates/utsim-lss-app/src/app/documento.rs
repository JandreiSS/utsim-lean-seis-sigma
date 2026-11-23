//! Começar sem pasta e salvar depois, no UTSim LSS: o projeto é o documento. O app abre num
//! projeto em branco numa pasta temporária ([`Session::draft`]) e já deixa trabalhar. A
//! barra de cima à esquerda tem a chave "Salvamento automático", Salvar (Ctrl+S) e Salvar
//! como (Ctrl+Shift+S). Fechar a janela, fechar o projeto ou começar outro com trabalho não
//! salvo pergunta antes ("Salvar / Não salvar / Cancelar"), pela guarda comum de
//! `utsim_ui_tema::documento`.

use std::path::{Path, PathBuf};

use eframe::egui;
use serde_json::Value;
use utsim_lss_core::domain::{Approach, Project, fields};
use utsim_ui_tema::arquivos::{self, Tipo};
use utsim_ui_tema::documento::{self, Clique, Evento, Guarda, Pedido, Situacao};
use utsim_ui_tema::texto::tx as tema;

use super::{App, Screen, sanitize_folder_name};
use crate::session::Session;
use crate::text::{fx, tx};

/// O que espera a resposta de "Salvar / Não salvar / Cancelar".
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Depois {
    /// Fechar o projeto e voltar à abertura.
    Fechar,
    /// Começar outro projeto em branco.
    Novo,
    /// Abrir o projeto desta pasta.
    Abrir(PathBuf),
}

/// A janela "Salvar como": o nome do projeto e onde fica a pasta.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct JanelaSalvar {
    pub nome: String,
    pub pasta: String,
}

impl JanelaSalvar {
    /// A pasta que será criada (`None`: falta o nome ou o lugar).
    pub fn destino(&self) -> Option<PathBuf> {
        let (n, p) = (self.nome.trim(), self.pasta.trim());
        (!n.is_empty() && !p.is_empty()).then(|| {
            utsim_lss_store::project::with_extension(&Path::new(p).join(sanitize_folder_name(n)))
        })
    }
}

#[derive(Default)]
pub struct Documento {
    pub guarda: Guarda<Depois>,
    pub janela: Option<JanelaSalvar>,
    titulo: String,
    fechar: bool,
}

/// Se o projeto aberto é um documento (um projeto, não um programa nem uma carteira).
fn eh_documento(s: &Session) -> bool {
    !s.is_program() && !s.folder().is_portfolio()
}

impl App {
    /// Um projeto novo, sem pasta, já aberto no painel.
    pub fn comecar_sem_pasta(&mut self) -> bool {
        let Some(me) = self.identity() else {
            return false;
        };
        let r = Session::draft(tx("l16-projeto_novo"), Approach::Dmaic, &me, (self.clock)());
        match self.report(r) {
            Some(s) => {
                self.close_project();
                self.session = Some(s);
                self.screen = Screen::Project;
                self.nav = super::Nav::Overview;
                self.status = None;
                true
            }
            None => false,
        }
    }

    /// Há trabalho não salvo no projeto aberto.
    pub fn nao_salvo(&self) -> bool {
        self.session.as_ref().is_some_and(Session::nao_salvo)
    }

    /// Pede uma troca (fechar, novo, abrir): com trabalho não salvo pergunta antes.
    pub fn pedir_troca(&mut self, d: Depois) {
        let sujo = self.nao_salvo();
        if let Some(a) = self.doc.guarda.pedir(sujo, d) {
            self.fazer_troca(Pedido::Acao(a));
        }
    }

    fn fazer_troca(&mut self, p: Pedido<Depois>) {
        match p {
            Pedido::Fechar => {}
            Pedido::Acao(Depois::Fechar) => self.close_project(),
            Pedido::Acao(Depois::Novo) => {
                self.comecar_sem_pasta();
            }
            Pedido::Acao(Depois::Abrir(p)) => {
                self.close_project();
                self.open_project(&p);
            }
        }
    }

    /// A barra do documento, num painel de cima antes dos outros.
    pub(super) fn barra_documento(&mut self, ui: &mut egui::Ui) {
        let Some(s) = self.session.as_ref().filter(|s| eh_documento(s)) else {
            return;
        };
        let situacao = if s.is_draft() || s.nao_salvo() {
            Situacao::NaoSalvo
        } else {
            Situacao::Salvo
        };
        let (rascunho, auto) = (s.is_draft(), s.automatico());
        match documento::barra(ui, auto, situacao) {
            Some(Clique::Salvar) => self.salvar_doc(),
            Some(Clique::SalvarComo) => self.salvar_como_doc(),
            Some(Clique::Automatico(ligado)) => {
                if rascunho {
                    if ligado {
                        self.salvar_como_doc();
                    }
                } else {
                    self.definir_automatico(ligado);
                }
            }
            None => {}
        }
    }

    fn definir_automatico(&mut self, ligado: bool) {
        let Some(s) = self.session.as_mut() else {
            return;
        };
        let raiz = s.folder().root().to_path_buf();
        let r = s.set_automatico(ligado);
        self.config.sem_automatico.retain(|p| *p != raiz);
        if !ligado {
            self.config.sem_automatico.push(raiz);
        }
        self.save_config();
        if let Some(n) = self.report(r)
            && n > 0
        {
            self.set_status(fx("l16-salvos", &[("n", (n as i64).into())]), false);
        }
    }

    /// Ctrl+S salva e Ctrl+Shift+S salva como.
    pub(super) fn atalhos_documento(&mut self, ctx: &egui::Context) {
        if self.doc.janela.is_some() || !self.session.as_ref().is_some_and(eh_documento) {
            return;
        }
        use egui::{Key, Modifiers};
        let (como, salvar) = ctx.input_mut(|i| {
            let como = i.consume_key(Modifiers::COMMAND | Modifiers::SHIFT, Key::S);
            (como, !como && i.consume_key(Modifiers::COMMAND, Key::S))
        });
        if como {
            self.salvar_como_doc();
        } else if salvar {
            self.salvar_doc();
        }
    }

    /// Salvar: com pasta, grava o que esperava; sem pasta, abre o "Salvar como".
    pub fn salvar_doc(&mut self) {
        let Some(s) = self.session.as_mut() else {
            return;
        };
        if s.is_draft() {
            self.salvar_como_doc();
            return;
        }
        let r = s.salvar();
        match self.report(r) {
            Some(n) => {
                self.set_status(fx("l16-salvos", &[("n", (n as i64).into())]), false);
                self.depois_de_salvar();
            }
            None => self.doc.guarda.cancelar(),
        }
    }

    /// Abre a janela "Salvar como" com o título do projeto e a pasta de agora (ou Documentos).
    pub fn salvar_como_doc(&mut self) {
        let Some(s) = self.session.as_ref() else {
            return;
        };
        let pasta = if s.is_draft() {
            arquivos::documentos()
        } else {
            s.folder().root().parent().map(Path::to_path_buf)
        };
        let nome = s
            .state()
            .entity(s.project())
            .map(|e| Project::read(s.project(), e).title)
            .unwrap_or_default();
        self.doc.janela = Some(JanelaSalvar {
            nome,
            pasta: pasta.map(|p| p.display().to_string()).unwrap_or_default(),
        });
    }

    /// Salva na pasta da janela; o nome vira o título do projeto. Devolve se salvou.
    pub fn salvar_em(&mut self, j: &JanelaSalvar) -> bool {
        let Some(destino) = j.destino() else {
            return false;
        };
        let Some(s) = self.session.as_mut() else {
            return false;
        };
        let titulo = j.nome.trim().to_string();
        let mudou_titulo = s
            .state()
            .entity(s.project())
            .is_some_and(|e| Project::read(s.project(), e).title != titulo);
        if mudou_titulo {
            let p = Project::entity_ref(s.project());
            let r = s.set_field(p, fields::project::TITLE, Value::from(titulo), None);
            if self.report(r).is_none() {
                return false;
            }
        }
        let Some(s) = self.session.as_mut() else {
            return false;
        };
        let r = s.save_as(&destino);
        if self.report(r).is_none() {
            return false;
        }
        // A escolha de salvar sozinho é por projeto; o padrão é ligado.
        let desligado = self.config.sem_automatico.contains(&destino);
        if let Some(s) = self.session.as_mut() {
            let _ = s.set_automatico(!desligado);
        }
        self.config.touch_recent(&destino);
        self.save_config();
        self.set_status(
            fx(
                "l16-salvo_em",
                &[("pasta", destino.display().to_string().into())],
            ),
            false,
        );
        self.doc.janela = None;
        self.depois_de_salvar();
        true
    }

    fn depois_de_salvar(&mut self) {
        match self.doc.guarda.salvou() {
            Some(Pedido::Fechar) => self.doc.fechar = true,
            Some(p) => self.fazer_troca(p),
            None => {}
        }
    }

    /// Uma vez por quadro, no fim: a pergunta, a janela "Salvar como" e o título da janela.
    pub(super) fn quadro_documento(&mut self, ctx: &egui::Context) {
        if let Some(j) = self.doc.janela.clone() {
            self.janela_salvar(ctx, j);
        }
        let sujo = self.nao_salvo();
        match self.doc.guarda.quadro(ctx, sujo) {
            Some(Evento::Salvar) => self.salvar_doc(),
            Some(Evento::Seguir(p)) => self.fazer_troca(p),
            None => {}
        }
        if std::mem::take(&mut self.doc.fechar) {
            self.doc.guarda.fechar_agora(ctx);
        }
        let app = crate::text::texts().app_name;
        let (nome, sujo) = match self.session.as_ref() {
            Some(s) => (Some(s.folder().title()), s.is_draft() || s.nao_salvo()),
            None => (None, false),
        };
        let titulo = documento::titulo_da_janela(app, nome.as_deref(), sujo);
        if titulo != self.doc.titulo {
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(titulo.clone()));
            self.doc.titulo = titulo;
        }
    }

    fn janela_salvar(&mut self, ctx: &egui::Context, mut j: JanelaSalvar) {
        let mut salvar = false;
        let mut cancelar = false;
        let destino = j.destino();
        let erro = self
            .status
            .as_ref()
            .filter(|(_, e)| *e)
            .map(|(m, _)| m.clone());
        let modal = egui::Modal::new(egui::Id::new("salvar_projeto_lss")).show(ctx, |ui| {
            ui.set_width(560.0);
            ui.heading(tema("documento-salvar_como"));
            ui.add_space(8.0);
            ui.label(tx("l16-nome_do_projeto"));
            ui.add(egui::TextEdit::singleline(&mut j.nome).desired_width(f32::INFINITY));
            ui.add_space(8.0);
            ui.label(tx("l16-onde_salvar"));
            arquivos::campo(
                ui,
                &mut j.pasta,
                Tipo::Pasta,
                tx("l16-onde_salvar"),
                &[],
                420.0,
            );
            ui.add_space(8.0);
            let fraco = ui.visuals().weak_text_color();
            match &destino {
                Some(d) => ui.colored_label(
                    fraco,
                    fx(
                        "l16-sera_criada",
                        &[("pasta", d.display().to_string().into())],
                    ),
                ),
                None => ui.colored_label(fraco, tx("l16-escreva_nome_e_pasta")),
            };
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                salvar = ui
                    .add_enabled(
                        destino.is_some(),
                        utsim_ui_tema::widgets::primary(tema("documento-salvar")),
                    )
                    .clicked();
                cancelar = ui.button(tema("documento-cancelar")).clicked();
            });
            if let Some(e) = &erro {
                ui.colored_label(ui.visuals().error_fg_color, e);
            }
        });
        if salvar {
            self.doc.janela = Some(j.clone());
            self.salvar_em(&j);
        } else if cancelar || modal.should_close() {
            self.doc.janela = None;
            self.doc.guarda.cancelar();
        } else {
            self.doc.janela = Some(j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn o_destino_ganha_a_extensao_e_pede_nome_e_pasta() {
        let j = JanelaSalvar {
            nome: "Refugo da tampa".into(),
            pasta: "C:/projetos".into(),
        };
        let d = j.destino().expect("destino");
        assert!(d.to_string_lossy().ends_with(".lss"));
        assert!(d.starts_with("C:/projetos"));
        assert_eq!(
            JanelaSalvar {
                nome: " ".into(),
                pasta: "x".into()
            }
            .destino(),
            None
        );
    }
}
