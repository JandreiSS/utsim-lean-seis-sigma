//! O painel do projeto, a primeira tela: a trilha das fases desenhada (uma estação por fase,
//! o losango do portão depois de cada uma), o cartão "Agora faça" da fase escolhida (o
//! próximo passo em destaque, com o botão que leva aonde ele se faz), os passos e os
//! entregáveis da fase, os números do projeto em cartões e, recolhido, o resto. O que
//! sugerir sai de `utsim_lss_core::trilha`.

use eframe::egui::{
    self, Align2, Color32, FontId, Pos2, Rect, RichText, Sense, Shape, Stroke, pos2, vec2,
};
use utsim_lss_core::domain::Column;
use utsim_lss_core::domain::{self, PhaseStatus};
use utsim_lss_core::overview::Overview;
use utsim_lss_core::roteiro::{self, Target};
use utsim_lss_core::tollgate::TollgateStatus;
use utsim_lss_core::tools;
use utsim_lss_core::trilha::{self, Estacao, Proximo};

use super::super::{App, Nav};
use crate::session::Session;
use crate::text::{self, l16::l16, texts};
use crate::theme;

/// O raio de uma estação.
pub const RAIO: f32 = 26.0;
/// A altura da trilha desenhada.
pub const ALTURA: f32 = 118.0;

/// Onde fica cada estação e o losango do portão depois dela, numa faixa de `rect`.
pub fn dispor_trilha(rect: Rect, n: usize) -> Vec<(Pos2, Pos2)> {
    if n == 0 {
        return Vec::new();
    }
    let margem = 50.0;
    let passo = (rect.width() - 2.0 * margem) / n as f32;
    let y = rect.top() + 40.0;
    (0..n)
        .map(|i| {
            let x = rect.left() + margem + passo * (i as f32 + 0.35);
            let estacao = pos2(x, y);
            let losango = pos2(x + passo * 0.5, y);
            (estacao, losango)
        })
        .collect()
}

/// A estação (ou o losango, segundo valor `true`) sob o ponto.
pub fn alvo_na_trilha(pos: &[(Pos2, Pos2)], p: Pos2) -> Option<(usize, bool)> {
    pos.iter().enumerate().find_map(|(i, (e, l))| {
        if e.distance(p) <= RAIO + 4.0 {
            Some((i, false))
        } else if (p.x - l.x).abs() + (p.y - l.y).abs() <= 14.0 {
            Some((i, true))
        } else {
            None
        }
    })
}

fn cor_do_portao(ui: &egui::Ui, s: Option<TollgateStatus>) -> Color32 {
    match s {
        Some(TollgateStatus::Approved) => theme::OK,
        Some(TollgateStatus::Submitted) => theme::WARN,
        Some(TollgateStatus::Redo) => ui.visuals().error_fg_color,
        _ => ui.visuals().weak_text_color(),
    }
}

fn nome_do_portao(s: Option<TollgateStatus>) -> &'static str {
    let t = l16();
    match s {
        None => t.portao_sem,
        Some(TollgateStatus::Open) => t.portao_aberto,
        Some(TollgateStatus::Submitted) => t.portao_enviado,
        Some(TollgateStatus::Approved) => t.portao_aprovado,
        Some(TollgateStatus::Redo) => t.portao_refazer,
        Some(TollgateStatus::Suspended) => t.portao_suspenso,
        Some(TollgateStatus::Cancelled) => t.portao_cancelado,
    }
}

/// Um anel de progresso de 0 a 1 em volta de `c`.
fn anel(p: &egui::Painter, c: Pos2, r: f32, f: f32, cor: Color32, fundo: Color32) {
    p.circle_stroke(c, r, Stroke::new(4.0, fundo));
    if f <= 0.0 {
        return;
    }
    let n = 48;
    let fim = (n as f32 * f.clamp(0.0, 1.0)).ceil() as usize;
    let pts: Vec<Pos2> = (0..=fim)
        .map(|k| {
            let a =
                -std::f32::consts::FRAC_PI_2 + std::f32::consts::TAU * (k as f32 / n as f32).min(f);
            c + vec2(a.cos(), a.sin()) * r
        })
        .collect();
    p.add(Shape::line(pts, Stroke::new(4.0, cor)));
}

impl App {
    pub(in crate::app) fn painel(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = l16();
        let es = trilha::estacoes(s.state(), s.project());
        let Some(o) = Overview::compute(s.state(), s.project(), s.today()) else {
            ui.label(texts().untitled);
            return;
        };
        // O título fica na barra lateral; aqui, a abordagem e a área.
        let abordagem = text::approach_name(o.project.approach);
        ui.weak(if o.project.area.trim().is_empty() {
            abordagem.to_string()
        } else {
            format!("{abordagem} · {}", o.project.area)
        });
        if es.is_empty() {
            return;
        }
        let atual = trilha::indice_atual(s.state(), s.project()).unwrap_or(0);
        let escolhida = self
            .l1
            .painel_fase
            .filter(|i| *i < es.len())
            .unwrap_or(atual);
        ui.add_space(6.0);
        if let Some((i, portao)) = self.trilha_ui(ui, &es, escolhida) {
            if portao {
                self.l1.phase = Some(es[i].fase);
                self.nav = Nav::Phases;
            } else {
                self.l1.painel_fase = Some(i);
            }
        }
        ui.add_space(4.0);
        self.agora_faca(ui, s, &es, escolhida);
        ui.add_space(10.0);
        self.numeros(ui, &o);
        ui.add_space(8.0);
        egui::CollapsingHeader::new(t.detalhes)
            .id_salt("painel_detalhes")
            .default_open(false)
            .show(ui, |ui| self.overview(ui, s));
    }

    /// A trilha desenhada; devolve a estação ou o portão clicado.
    fn trilha_ui(
        &mut self,
        ui: &mut egui::Ui,
        es: &[Estacao],
        escolhida: usize,
    ) -> Option<(usize, bool)> {
        let t = l16();
        let largura = ui.available_width().max(520.0);
        let (rect, resp) = ui.allocate_exact_size(vec2(largura, ALTURA), Sense::click());
        let pos = dispor_trilha(rect, es.len());
        let p = ui.painter_at(rect);
        let fg = ui.visuals().text_color();
        let fraco = ui.visuals().weak_text_color();
        let acento = theme::accent(ui);
        let fundo = ui.visuals().extreme_bg_color;
        // A linha que liga tudo, até a bandeira do fim.
        if let (Some(a), Some(b)) = (pos.first(), pos.last()) {
            let fim = b.1 + vec2(40.0, 0.0);
            p.line_segment([a.0, fim], Stroke::new(3.0, fraco.gamma_multiply(0.5)));
            p.text(
                fim + vec2(4.0, 0.0),
                Align2::LEFT_CENTER,
                "🏁",
                FontId::proportional(16.0),
                fg,
            );
        }
        let hover = resp.hover_pos().and_then(|h| alvo_na_trilha(&pos, h));
        for (i, (e, (c, l))) in es.iter().zip(&pos).enumerate() {
            let aprovada = e.status == PhaseStatus::Approved;
            let cor = if aprovada {
                theme::OK
            } else if e.atual {
                acento
            } else {
                fraco
            };
            let destacada = i == escolhida;
            p.circle_filled(
                *c,
                RAIO,
                if e.atual || aprovada {
                    cor.gamma_multiply(0.22)
                } else {
                    fundo
                },
            );
            anel(&p, *c, RAIO, e.progresso(), cor, fraco.gamma_multiply(0.25));
            if destacada || hover == Some((i, false)) {
                p.circle_stroke(*c, RAIO + 6.0, Stroke::new(2.0, acento));
            }
            let letra = if aprovada {
                "✔".to_string()
            } else {
                e.nome
                    .chars()
                    .next()
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            };
            p.text(
                *c,
                Align2::CENTER_CENTER,
                letra,
                FontId::proportional(20.0),
                cor,
            );
            p.text(
                *c + vec2(0.0, RAIO + 10.0),
                Align2::CENTER_TOP,
                &e.nome,
                FontId::proportional(13.0),
                if destacada { fg } else { fraco },
            );
            p.text(
                *c + vec2(0.0, RAIO + 27.0),
                Align2::CENTER_TOP,
                format!("{}/{}", e.feitos, e.total),
                FontId::proportional(11.0),
                fraco,
            );
            // O portão depois da fase.
            let cp = cor_do_portao(ui, e.portao);
            let r = if hover == Some((i, true)) { 13.0 } else { 11.0 };
            let losango = vec![
                *l + vec2(0.0, -r),
                *l + vec2(r, 0.0),
                *l + vec2(0.0, r),
                *l + vec2(-r, 0.0),
            ];
            let cheio = matches!(
                e.portao,
                Some(TollgateStatus::Approved | TollgateStatus::Submitted | TollgateStatus::Redo)
            );
            p.add(Shape::convex_polygon(
                losango,
                if cheio { cp } else { fundo },
                Stroke::new(2.0, cp),
            ));
        }
        if let Some((i, portao)) = hover {
            let e = &es[i];
            let dica = if portao {
                format!(
                    "{}: {}\n{}",
                    t.portao,
                    nome_do_portao(e.portao),
                    t.dica_portao
                )
            } else {
                format!(
                    "{} · {}\n{}",
                    e.nome,
                    text::phase_status_name(e.status),
                    t.dica_estacao
                )
            };
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            resp.clone().on_hover_text(dica);
        }
        if resp.clicked() {
            return resp
                .interact_pointer_pos()
                .and_then(|q| alvo_na_trilha(&pos, q));
        }
        None
    }

    /// O cartão "Agora faça" da fase `i`: o próximo passo, os passos e os entregáveis.
    fn agora_faca(&mut self, ui: &mut egui::Ui, s: &mut Session, es: &[Estacao], i: usize) {
        let t = l16();
        let e = &es[i];
        let passos = trilha::passos(s.state(), s.project(), i);
        let prox = trilha::proximo(s.state(), s.project(), i);
        let acento = theme::accent(ui);
        let mut abrir: Option<Target> = None;
        let mut ir_portao = false;
        let mut seguir: Option<usize> = None;
        theme::card(ui).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.heading(RichText::new(t.agora_faca).color(acento));
                ui.label(RichText::new(format!("· {}", e.nome)).strong());
                ui.weak(text::phase_status_name(e.status));
            });
            if o_dmaic(s)
                && let Some(ph) = roteiro::PHASES.get(i)
            {
                ui.label(RichText::new(ph.question).italics());
            }
            ui.add_space(6.0);
            let vermelho = ui.visuals().error_fg_color;
            // O destaque: o que fazer agora.
            let destaque = |ui: &mut egui::Ui, cor: Color32, add: &mut dyn FnMut(&mut egui::Ui)| {
                egui::Frame::new()
                    .fill(cor.gamma_multiply(0.14))
                    .stroke(Stroke::new(1.5, cor))
                    .corner_radius(8)
                    .inner_margin(egui::Margin::same(12))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        add(ui);
                    });
            };
            match &prox {
                Proximo::Passo(k) => {
                    let p = &passos[*k];
                    destaque(ui, acento, &mut |ui| {
                        ui.label(RichText::new(t.proximo_passo).small().color(acento));
                        ui.label(RichText::new(&p.titulo).size(18.0).strong());
                        if !p.para_que.is_empty() {
                            ui.label(&p.para_que);
                        }
                        ui.horizontal_wrapped(|ui| {
                            for (nome, alvo) in &p.alvos {
                                let rotulo = if nome.is_empty() {
                                    t.abrir.to_string()
                                } else {
                                    format!("{} →", nome)
                                };
                                if ui.add(utsim_ui_tema::widgets::primary(&rotulo)).clicked() {
                                    abrir = Some(*alvo);
                                }
                            }
                        });
                    });
                }
                Proximo::PedirPortao => destaque(ui, theme::OK, &mut |ui| {
                    ui.label(
                        RichText::new(text_fase("l16-pedir_portao", &e.nome))
                            .size(16.0)
                            .strong(),
                    );
                    if ui
                        .add(utsim_ui_tema::widgets::primary(t.ir_ao_portao))
                        .clicked()
                    {
                        ir_portao = true;
                    }
                }),
                Proximo::AguardarDecisao => destaque(ui, theme::WARN, &mut |ui| {
                    ui.label(RichText::new(text_fase("l16-aguardando", &e.nome)).strong());
                    if ui.button(t.ir_ao_portao).clicked() {
                        ir_portao = true;
                    }
                }),
                Proximo::Refazer => destaque(ui, vermelho, &mut |ui| {
                    ui.label(RichText::new(text_fase("l16-refazer", &e.nome)).strong());
                    if ui.button(t.ir_ao_portao).clicked() {
                        ir_portao = true;
                    }
                }),
                Proximo::Seguir(Some(j)) => destaque(ui, theme::OK, &mut |ui| {
                    ui.label(RichText::new(text_fase("l16-fase_aprovada", &e.nome)).strong());
                    let rotulo = text_fase("l16-seguir_para", &es[*j].nome);
                    if ui.add(utsim_ui_tema::widgets::primary(&rotulo)).clicked() {
                        seguir = Some(*j);
                    }
                }),
                Proximo::Seguir(None) => destaque(ui, theme::OK, &mut |ui| {
                    ui.label(RichText::new(t.projeto_concluido).strong());
                }),
            }
            ui.add_space(8.0);
            // Os passos da fase: os do roteiro e, à parte, os do checklist do portão.
            for (titulo, do_portao) in [(t.passos_da_fase, false), (t.do_checklist, true)] {
                let grupo: Vec<&trilha::Passo> =
                    passos.iter().filter(|p| p.do_portao == do_portao).collect();
                if grupo.is_empty() {
                    continue;
                }
                let feitos = grupo.iter().filter(|p| p.feito).count();
                ui.label(RichText::new(format!("{titulo} · {feitos}/{}", grupo.len())).strong());
                for p in grupo {
                    ui.horizontal(|ui| {
                        if p.feito {
                            ui.colored_label(theme::OK, "✔");
                            ui.label(RichText::new(&p.titulo).weak().strikethrough());
                        } else {
                            ui.weak("○");
                            ui.label(&p.titulo);
                            for (nome, alvo) in &p.alvos {
                                let rotulo = if nome.is_empty() {
                                    t.abrir
                                } else {
                                    nome.as_str()
                                };
                                if ui.small_button(rotulo).clicked() {
                                    abrir = Some(*alvo);
                                }
                            }
                        }
                    });
                }
                ui.add_space(4.0);
            }
            // Os entregáveis da fase.
            let ds: Vec<tools::Deliverable> = tools::deliverables(s.state(), s.project())
                .into_iter()
                .filter(|d| d.phase == Some(e.fase))
                .collect();
            ui.label(RichText::new(t.entregaveis).strong());
            if ds.is_empty() {
                ui.weak(t.sem_entregaveis);
            }
            ui.horizontal_wrapped(|ui| {
                for d in &ds {
                    let nome = if d.name.trim().is_empty() {
                        d.tool.map(text::tool_name).unwrap_or_default().to_string()
                    } else {
                        d.name.clone()
                    };
                    let rotulo = format!("{nome} · {}", text::deliverable_status_name(d.status));
                    if ui.button(rotulo).clicked() {
                        self.nav = Nav::Tools;
                        self.selected_deliverable = Some(d.id);
                    }
                }
            });
        });
        if let Some(a) = abrir {
            self.open_target(s, a);
        }
        if ir_portao {
            self.l1.phase = Some(e.fase);
            self.nav = Nav::Phases;
        }
        if let Some(j) = seguir {
            self.l1.painel_fase = Some(j);
        }
    }

    /// Os números do projeto em cartões.
    fn numeros(&mut self, ui: &mut egui::Ui, o: &Overview) {
        let t = texts();
        let feitas = o.actions_by_column.get(&Column::Done).copied().unwrap_or(0);
        let prazo = match (o.project.deadline, o.days_to_deadline) {
            (Some(_), Some(n)) => (t.days_left)(n),
            _ => t.no_deadline.to_string(),
        };
        let ganhos: f64 = o.savings_by_status.values().sum();
        let cartoes = [
            (t.actions, format!("{feitas}/{}", o.actions_total), None),
            (
                t.overdue,
                o.actions_overdue.len().to_string(),
                (!o.actions_overdue.is_empty()).then_some(theme::WARN),
            ),
            (
                t.risks_open,
                format!("{} ({} {})", o.risks_open, o.risks_high.len(), t.risks_high),
                (!o.risks_high.is_empty()).then_some(theme::WARN),
            ),
            (t.savings, text::money(ganhos), None),
            (
                t.deadline,
                prazo,
                o.days_to_deadline.filter(|n| *n < 0).map(|_| theme::WARN),
            ),
        ];
        ui.horizontal_wrapped(|ui| {
            for (rotulo, valor, cor) in cartoes {
                theme::card(ui).show(ui, |ui| {
                    ui.set_min_width(130.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new(rotulo).small().weak());
                        let v = RichText::new(valor).size(18.0).strong();
                        ui.label(match cor {
                            Some(c) => v.color(c),
                            None => v,
                        });
                    });
                });
            }
        });
    }
}

fn o_dmaic(s: &Session) -> bool {
    s.state()
        .entity(s.project())
        .map(|e| domain::Project::read(s.project(), e).approach)
        == Some(domain::Approach::Dmaic)
}

/// O texto de uma chave com o nome da fase.
fn text_fase(chave: &str, fase: &str) -> String {
    crate::text::fx(chave, &[("fase", fase.to_string().into())])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clicar_na_estacao_escolhe_a_fase_e_no_losango_abre_o_portao() {
        let r = Rect::from_min_size(Pos2::ZERO, vec2(900.0, ALTURA));
        let pos = dispor_trilha(r, 5);
        assert_eq!(pos.len(), 5);
        for (i, (e, l)) in pos.iter().enumerate() {
            assert_eq!(alvo_na_trilha(&pos, *e), Some((i, false)));
            assert_eq!(alvo_na_trilha(&pos, *l), Some((i, true)));
            assert!(l.x > e.x + RAIO, "o portão vem depois da estação");
            assert!(r.contains(*e));
        }
        assert!(
            pos[1].0.x > pos[0].1.x,
            "a próxima estação vem depois do portão"
        );
        assert_eq!(alvo_na_trilha(&pos, pos2(1.0, 110.0)), None);
        assert!(dispor_trilha(r, 0).is_empty());
    }
}
