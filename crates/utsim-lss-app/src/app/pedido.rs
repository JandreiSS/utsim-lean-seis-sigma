//! A oportunidade que outro app da família manda (`--utsim-pedido`, [`crate::troca`]): uma
//! janela com o resumo e a carteira onde a ideia entra. Só aparece depois da identidade (a
//! ideia vai com o nome de quem cria).

use std::path::Path;

use eframe::egui;

use super::{App, Screen};
use crate::text::l17::l17;
use crate::troca::{self, PedidoOportunidade};

/// O pedido à espera e a carteira escolhida.
#[derive(Default)]
pub struct PedidoUi {
    pub pedido: Option<PedidoOportunidade>,
    pub carteira: String,
}

impl App {
    /// Lê o pedido em `caminho` (o app abriu com `--utsim-pedido`): um pedido válido fica à
    /// espera da carteira; um inválido vira mensagem de erro na barra de baixo.
    pub fn receber_pedido(&mut self, caminho: &Path) -> bool {
        match troca::ler(caminho) {
            Ok(p) => {
                self.pedido_ui.pedido = Some(p);
                true
            }
            Err(e) => {
                self.set_status(e, true);
                false
            }
        }
    }

    /// O pedido à espera, se houver.
    pub fn pedido(&self) -> Option<&PedidoOportunidade> {
        self.pedido_ui.pedido.as_ref()
    }

    /// Cria a ideia do pedido na carteira em `carteira` e abre a carteira nela.
    pub fn enviar_pedido(&mut self, carteira: &Path) -> bool {
        let (Some(p), Some(me)) = (self.pedido_ui.pedido.as_ref(), self.identity()) else {
            return false;
        };
        let clock = (self.clock)();
        match p.enviar(carteira, &me, clock.as_ref()) {
            Ok(idea) => {
                let titulo = p.submissao().title;
                self.pedido_ui = PedidoUi::default();
                self.close_project();
                if self.open_project(carteira) {
                    self.select_idea(idea);
                }
                self.set_status((l17().criada)(&titulo), false);
                true
            }
            Err(e) => {
                self.set_status(e, true);
                false
            }
        }
    }

    /// A janela do pedido, por cima das telas.
    pub(super) fn janela_pedido(&mut self, ctx: &egui::Context) {
        if self.pedido_ui.pedido.is_none() || self.screen == Screen::Identity {
            return;
        }
        // A carteira aberta é a sugestão.
        if self.pedido_ui.carteira.is_empty()
            && let Some(s) = self.session.as_ref().filter(|s| s.folder().is_portfolio())
        {
            self.pedido_ui.carteira = s.folder().root().display().to_string();
        }
        let t = l17();
        let mut criar = false;
        let mut fechar = false;
        let Some(p) = self.pedido_ui.pedido.as_ref() else {
            return;
        };
        let titulo = p.submissao().title;
        let dados = p.dados();
        // Um quadro modal, como o de Salvar como e os da abertura.
        let modal = egui::Modal::new(egui::Id::new("pedido_lss")).show(ctx, |ui| {
            ui.set_width(600.0);
            ui.heading(t.janela);
            ui.add_space(8.0);
            ui.label(t.intro);
            ui.add_space(8.0);
            ui.strong(&titulo);
            ui.add_space(4.0);
            egui::ScrollArea::vertical()
                .max_height(280.0)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.label(&dados);
                });
            ui.add_space(8.0);
            ui.label(t.carteira);
            utsim_ui_tema::arquivos::campo(
                ui,
                &mut self.pedido_ui.carteira,
                utsim_ui_tema::arquivos::Tipo::Pasta,
                t.carteira,
                &[],
                420.0,
            )
            .on_hover_text(t.carteira_dica);
            let tem = !self.pedido_ui.carteira.trim().is_empty();
            if !tem {
                ui.add_space(8.0);
                ui.colored_label(ui.visuals().weak_text_color(), t.sem_carteira);
            }
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                criar = ui
                    .add_enabled(tem, utsim_ui_tema::widgets::primary(t.criar))
                    .clicked();
                fechar = ui.button(t.agora_nao).clicked();
            });
        });
        fechar |= modal.should_close();
        if criar {
            let c = std::path::PathBuf::from(self.pedido_ui.carteira.trim());
            self.enviar_pedido(&c);
        } else if fechar {
            self.pedido_ui = PedidoUi::default();
        }
    }
}
