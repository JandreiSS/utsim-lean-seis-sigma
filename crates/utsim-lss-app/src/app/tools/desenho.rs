//! As ferramentas desenhadas do LSS, com os mesmos componentes do UTSim Qualidade e do
//! Segurança (`utsim_visual`): a espinha editável do Ishikawa, a árvore de cada cadeia de
//! porquês, o Pareto vivo que filtra a tabela e a matriz S × O da FMEA. Cada gesto vira uma
//! edição da [`Session`], como os campos.

use eframe::egui;
use serde_json::Value;
use utsim_lss_core::Id as Uid;
use utsim_lss_core::tools::ap::{Ap, ApTable};
use utsim_lss_core::tools::{self, Tool, fmea, ishikawa, item_ref, pareto, whys};
use utsim_visual::{Nivel, arvore, espinha, matriz};

use super::super::App;
use crate::session::Session;
use crate::text::tool_texts;

fn nivel_do_ap(ap: Ap) -> Nivel {
    match ap {
        Ap::High => Nivel::Alto,
        Ap::Medium => Nivel::Medio,
        Ap::Low => Nivel::Baixo,
    }
}

/// Se o nome de uma linha de dados entra na barra escolhida do Pareto: o mesmo nome (sem
/// diferença de maiúsculas) ou, na barra dos outros, um nome sem barra própria.
pub fn na_barra(p: &pareto::Pareto, barra: &str, nome: &str) -> bool {
    let nome = nome.trim().to_lowercase();
    if nome.is_empty() {
        return false;
    }
    if p.rows.iter().any(|r| r.others && r.name == barra) {
        !p.rows
            .iter()
            .any(|r| !r.others && r.name.trim().to_lowercase() == nome)
    } else {
        barra.trim().to_lowercase() == nome
    }
}

/// O id do Pareto vivo de um entregável (a barra escolhida fica na memória da tela).
pub fn id_pareto(chave: &str, del: Uid) -> String {
    format!("lss_pareto_{chave}_{del}")
}

/// O Pareto vivo; devolve a barra escolhida.
pub fn pareto_vivo(ui: &mut egui::Ui, id: &str, p: &pareto::Pareto) -> Option<String> {
    let barras: Vec<utsim_visual::pareto::Barra> = p
        .rows
        .iter()
        .map(|r| utsim_visual::pareto::Barra {
            nome: r.name.clone(),
            valor: r.value,
            pct: r.pct,
            acum: r.cum_pct,
            vital: r.vital,
        })
        .collect();
    utsim_visual::pareto::mostrar(ui, id, &barras, 250.0)
}

/// A linha da FMEA em destaque (escolhida na matriz) e se a grade ainda rola até ela.
pub fn foco_fmea(ctx: &egui::Context, del: Uid) -> (Option<Uid>, bool) {
    ctx.data(|m| m.get_temp::<(Option<Uid>, bool)>(egui::Id::new(format!("lss_fmea_foco_{del}"))))
        .unwrap_or((None, false))
}

/// Marca que a grade já rolou até a linha em foco.
pub fn rolou(ctx: &egui::Context, del: Uid) {
    let (f, _) = foco_fmea(ctx, del);
    ctx.data_mut(|m| m.insert_temp(egui::Id::new(format!("lss_fmea_foco_{del}")), (f, false)));
}

/// A matriz S × O dos modos de falha do entregável, na cor do AP de cada linha.
pub fn fmea_matriz(ui: &mut egui::Ui, del: Uid, rows: &[fmea::Row], table: &ApTable) {
    let (foco, _) = foco_fmea(ui.ctx(), del);
    let pontos: Vec<matriz::Ponto> = rows
        .iter()
        .enumerate()
        .filter_map(|(i, r)| {
            let (s, o, d) = r.sod()?;
            Some(matriz::Ponto {
                linha: i,
                s,
                o,
                nivel: nivel_do_ap(r.ap(table).unwrap_or(Ap::Low)),
                rotulo: format!("{} (D {d}, {})", r.mode, fmea::rpn((s, o, d))),
            })
        })
        .collect();
    if pontos.is_empty() {
        return;
    }
    let escolhida = foco.and_then(|f| rows.iter().position(|r| r.id == f));
    if let Some(i) = matriz::mostrar(ui, format!("lss_matriz_{del}"), &pontos, escolhida)
        && let Some(r) = rows.get(i)
    {
        ui.ctx().data_mut(|m| {
            m.insert_temp(
                egui::Id::new(format!("lss_fmea_foco_{del}")),
                (Some(r.id), true),
            )
        });
    }
}

/// A cor do AP da linha, para o marcador da grade.
pub fn cor_do_ap(ap: Option<Ap>) -> Option<egui::Color32> {
    ap.map(|a| utsim_visual::cores::do_nivel(nivel_do_ap(a)))
}

impl App {
    /// A espinha do Ishikawa, editável: as causas de primeiro nível em cartões.
    pub(in crate::app) fn espinha_lss(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        del: Uid,
        g: &ishikawa::Diagram,
    ) {
        use ishikawa::fields as f;
        let fraco = ui.visuals().weak_text_color();
        let acento = utsim_visual::cores::destaque(ui);
        let raizes: Vec<Vec<&ishikawa::Cause>> = g
            .bones
            .iter()
            .map(|b| b.causes.iter().filter(|c| c.depth == 0).collect())
            .collect();
        let ossos: Vec<espinha::Osso> = g
            .bones
            .iter()
            .zip(&raizes)
            .map(|(b, rs)| espinha::Osso {
                nome: ishikawa::category_name(&b.category),
                cartoes: rs
                    .iter()
                    .map(|c| {
                        let filhas = b.causes.iter().filter(|x| x.parent == Some(c.id)).count();
                        espinha::Cartao {
                            texto: c.text.clone(),
                            cor: Some(match c.status {
                                ishikawa::CauseStatus::Verified => acento,
                                ishikawa::CauseStatus::Probable => crate::theme::WARN,
                                ishikawa::CauseStatus::RuledOut => fraco.gamma_multiply(0.5),
                                ishikawa::CauseStatus::Open => fraco,
                            }),
                            marca: (filhas > 0).then(|| format!("+{filhas}")),
                            dica: (!c.evidence.trim().is_empty())
                                .then(|| format!("{}\n\n{}", c.text, c.evidence)),
                        }
                    })
                    .collect(),
            })
            .collect();
        let mut e = espinha::Espinha::editavel(&g.effect, &ossos);
        // Os ossos do LSS vêm das categorias das causas: não se renomeiam aqui.
        e.ossos_editaveis = false;
        let acoes = espinha::mostrar(ui, format!("lss_espinha_{del}"), &e);
        let causa = |osso: usize, cartao: usize| raizes.get(osso).and_then(|r| r.get(cartao));
        for a in acoes {
            match a {
                espinha::Acao::Mover { osso, cartao, para } => {
                    if let (Some(c), Some(b)) = (causa(osso, cartao), g.bones.get(para)) {
                        let res = s.set_field(
                            item_ref(c.id),
                            f::CATEGORY,
                            Value::from(b.category.clone()),
                            None,
                        );
                        self.report(res);
                    }
                }
                espinha::Acao::Acrescentar { osso, texto } => {
                    if let Some(b) = g.bones.get(osso) {
                        let res = s.add_item(
                            del,
                            ishikawa::ITEM,
                            None,
                            vec![
                                (f::TEXT, Value::from(texto)),
                                (f::CATEGORY, Value::from(b.category.clone())),
                            ],
                        );
                        self.report(res);
                    }
                }
                espinha::Acao::Editar {
                    osso,
                    cartao,
                    texto,
                } => {
                    if let Some(c) = causa(osso, cartao) {
                        let res = s.set_field(item_ref(c.id), f::TEXT, Value::from(texto), None);
                        self.report(res);
                    }
                }
                espinha::Acao::Remover { osso, cartao } => {
                    if let Some(c) = causa(osso, cartao) {
                        let res = s.delete_item(c.id);
                        self.report(res);
                    }
                }
                espinha::Acao::PorQues { osso, cartao } => {
                    if let Some(c) = causa(osso, cartao) {
                        let (id, texto) = (c.id, c.text.clone());
                        self.porques_da_causa(s, id, &texto);
                    }
                }
                espinha::Acao::RenomearOsso { .. } | espinha::Acao::RemoverOsso { .. } => {}
            }
        }
    }

    /// Abre uma cadeia de porquês com a causa, no primeiro 5 porquês do projeto (criado
    /// quando não há), e passa para ele.
    fn porques_da_causa(&mut self, s: &mut Session, causa: Uid, texto: &str) {
        let existente = tools::of_tool(s.state(), s.project(), Tool::FiveWhys)
            .first()
            .map(|d| d.id);
        let del = match existente {
            Some(d) => d,
            None => match self.report(s.add_deliverable(Tool::FiveWhys, texto)) {
                Some(d) => d,
                None => return,
            },
        };
        let ja = whys::chains(s.state(), del)
            .iter()
            .any(|c| c.cause.as_ref().is_some_and(|(id, _)| *id == causa));
        if !ja {
            let res = s.add_item(
                del,
                whys::CHAIN,
                None,
                vec![
                    (whys::fields::TEXT, Value::from(texto)),
                    (whys::fields::CAUSE, Value::from(causa.to_string())),
                ],
            );
            self.report(res);
        }
        self.selected_deliverable = Some(del);
    }

    /// A árvore de uma cadeia: o problema na raiz e cada porquê ligado ao de antes.
    pub(in crate::app) fn arvore_lss(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        del: Uid,
        ch: &whys::Chain,
    ) {
        use whys::fields as f;
        let mut ids: Vec<Uid> = vec![ch.id];
        let mut nos = vec![arvore::No::novo(
            None,
            arvore::Tipo::Raiz,
            ch.problem.clone(),
        )];
        // Os porquês vêm em profundidade: o pai é o último porquê de um nível acima.
        let mut ultimo_do_nivel: Vec<usize> = vec![0];
        for w in &ch.whys {
            let pai = ultimo_do_nivel
                .get(w.level.saturating_sub(1))
                .copied()
                .unwrap_or(0);
            let mut n = arvore::No::novo(Some(pai), arvore::Tipo::Porque, w.text.clone());
            n.marcado = w.root;
            if !w.evidence.trim().is_empty() {
                n.dica = Some(format!("{}\n\n{}", w.text, w.evidence));
            }
            nos.push(n);
            ids.push(w.id);
            ultimo_do_nivel.truncate(w.level);
            ultimo_do_nivel.push(nos.len() - 1);
        }
        let marcar = tool_texts().root_cause;
        let acoes = egui::ScrollArea::horizontal()
            .id_salt(format!("lss_arvore_rolagem_{}", ch.id))
            .show(ui, |ui| {
                arvore::mostrar(
                    ui,
                    format!("lss_arvore_{}", ch.id),
                    &arvore::Arvore {
                        nos: &nos,
                        marcar: Some(marcar),
                    },
                )
            })
            .inner;
        for a in acoes {
            match a {
                arvore::Acao::Editar { no, texto } => {
                    if let Some(&id) = ids.get(no) {
                        let res = s.set_field(item_ref(id), f::TEXT, Value::from(texto), None);
                        self.report(res);
                    }
                }
                arvore::Acao::Filho { no, texto } => {
                    if let Some(&id) = ids.get(no) {
                        let res = s.add_item(
                            del,
                            whys::WHY,
                            Some(id),
                            vec![(f::TEXT, Value::from(texto))],
                        );
                        self.report(res);
                    }
                }
                arvore::Acao::Remover { no } => {
                    if no > 0
                        && let Some(&id) = ids.get(no)
                    {
                        let res = s.delete_item(id);
                        self.report(res);
                    }
                }
                arvore::Acao::Marcar { no } => {
                    if no > 0
                        && let (Some(&id), Some(n)) = (ids.get(no), nos.get(no))
                    {
                        let res = s.set_field(item_ref(id), f::ROOT, Value::Bool(!n.marcado), None);
                        self.report(res);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_barra_escolhida_filtra_pelo_nome_e_a_dos_outros_pelos_que_sobram() {
        let p = pareto::pareto(
            &[
                ("Risco".into(), 50.0),
                ("Mancha".into(), 40.0),
                ("Pó".into(), 1.0),
                ("Fio".into(), 1.0),
            ],
            5.0,
        );
        assert!(na_barra(&p, "Risco", " risco"));
        assert!(!na_barra(&p, "Risco", "Mancha"));
        let outros = p
            .rows
            .iter()
            .find(|r| r.others)
            .expect("outros")
            .name
            .clone();
        assert!(na_barra(&p, &outros, "Pó"));
        assert!(!na_barra(&p, &outros, "Risco"));
        assert!(!na_barra(&p, "Risco", ""));
    }
}
