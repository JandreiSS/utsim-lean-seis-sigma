//! Pedidos de outros apps da família, no formato de troca (`utsim-troca`): o app abre com
//! `--utsim-pedido=<caminho>`. Hoje o LSS atende `nova_oportunidade`, do UTSim Ambiental
//! (contrato em `docs/troca_ambiental.md` do Ambiental): o resumo de um cenário com ganho
//! ambiental vira uma ideia no banco de ideias da carteira, com as diferenças de CO₂e,
//! energia, água e custo para o Base, a linha ambiental do mapa de fluxo de valor e os
//! testes da redução, e o arquivo dos cenários anexado (com o hash conferido).
//!
//! O ganho ambiental fica ao lado do financeiro, sem converter um no outro (decisão A25 do
//! Ambiental): a ideia chega sem ganho financeiro estimado, e quem prioriza preenche.

use std::path::{Path, PathBuf};

use serde::Deserialize;
use utsim_lss_core::Id;
use utsim_lss_core::portfolio::Submission;
use utsim_lss_store::user::Identity;
use utsim_troca::Pedido;

use crate::text::l17::l17;

/// A ação do pedido de oportunidade (texto, como as outras ações da família).
pub const NOVA_OPORTUNIDADE: &str = "nova_oportunidade";

/// A origem gravada na ideia (`origem_app`), como `qualidade_8d` para o 8D.
pub const ORIGEM_AMBIENTAL: &str = "ambiental_cenario";

/// O resumo do cenário que o Ambiental manda em `entrada.conteudo`. Campos que faltam ficam
/// no padrão, para um Ambiental mais novo com campos a mais não ser recusado.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct Oportunidade {
    pub cenario: String,
    pub alavancas: Vec<String>,
    /// `(métrica, diferença média, de, até)` para o Base (intervalo de 95 %, pareado).
    pub diferencas: Vec<(String, f64, f64, f64)>,
    pub investimento: Option<f64>,
    pub vida_anos: Option<f64>,
    pub linha_vsm: Vec<LinhaVsm>,
    pub testes: Vec<Teste>,
}

/// Uma máquina na linha ambiental do mapa de fluxo de valor.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct LinhaVsm {
    pub no: String,
    pub etapa: Option<String>,
    pub kwh_por_peca: Option<f64>,
    pub co2e_por_peca: Option<f64>,
}

/// Um teste da redução já gravado no cenário.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct Teste {
    pub metrica: String,
    pub diferenca: f64,
    pub de: f64,
    pub ate: f64,
    pub p_valor: f64,
    pub n: usize,
    pub origem: String,
}

/// Um pedido de oportunidade lido e conferido.
#[derive(Clone, Debug)]
pub struct PedidoOportunidade {
    pub pedido: Pedido,
    pub caminho: PathBuf,
    pub oportunidade: Oportunidade,
}

/// Lê o pedido em `caminho`: a ação tem de ser `nova_oportunidade`, os arquivos de entrada
/// têm de ter o hash do pedido e o resumo tem de ser lido. O erro vem pronto para a pessoa.
pub fn ler(caminho: &Path) -> Result<PedidoOportunidade, String> {
    let t = l17();
    let pedido = Pedido::ler(caminho).map_err(|e| (t.erro_pedido)(&e.to_string()))?;
    if pedido.acao != NOVA_OPORTUNIDADE {
        return Err((t.erro_acao)(&pedido.acao));
    }
    pedido
        .conferir_entrada()
        .map_err(|e| (t.erro_pedido)(&e.to_string()))?;
    let conteudo = pedido
        .entrada
        .conteudo
        .as_deref()
        .filter(|c| !c.trim().is_empty())
        .ok_or_else(|| t.sem_resumo.to_string())?;
    let oportunidade: Oportunidade =
        serde_json::from_str(conteudo).map_err(|e| (t.erro_pedido)(&e.to_string()))?;
    Ok(PedidoOportunidade {
        pedido,
        caminho: caminho.to_path_buf(),
        oportunidade,
    })
}

/// O nome de uma métrica do Ambiental na língua de agora (a chave, se for outra).
pub fn metrica(chave: &str) -> String {
    let t = l17();
    match chave {
        "ambiental.co2e_kg" => t.m_co2e,
        "ambiental.energia_kwh" => t.m_energia,
        "ambiental.agua_m3" => t.m_agua,
        "ambiental.custo_energia" => t.m_conta,
        "ambiental.custo" => t.m_custo,
        outra => return outra.to_string(),
    }
    .to_string()
}

/// Um número com até 4 algarismos significativos, na marca decimal da língua.
pub fn num(v: f64) -> String {
    if !v.is_finite() {
        return "–".into();
    }
    let casas = if v == 0.0 {
        0
    } else {
        (3 - v.abs().log10().floor() as i32).clamp(0, 6) as usize
    };
    let mut s = format!("{v:.casas$}");
    if s.contains('.') {
        s = s.trim_end_matches('0').trim_end_matches('.').to_string();
    }
    if s == "-0" {
        s = "0".into();
    }
    utsim_idiomas::marca_decimal(&s)
}

fn num_ou_traco(v: Option<f64>) -> String {
    v.map_or_else(|| "–".to_string(), num)
}

impl PedidoOportunidade {
    /// O nome do modelo de onde o cenário veio (o arquivo, sem a pasta).
    pub fn modelo(&self) -> String {
        self.pedido
            .contexto
            .modelo
            .as_ref()
            .and_then(|m| m.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    /// O texto dos dados de desperdício e energia, para o campo "dados" da ideia.
    pub fn dados(&self) -> String {
        let t = l17();
        let o = &self.oportunidade;
        let mut linhas = vec![(t.cabecalho)(&self.modelo(), &o.cenario)];
        if !o.alavancas.is_empty() {
            linhas.push(String::new());
            linhas.push(t.alavancas.to_string());
            linhas.extend(o.alavancas.iter().map(|a| format!("- {a}")));
        }
        if !o.diferencas.is_empty() {
            linhas.push(String::new());
            linhas.push(t.diferencas.to_string());
            for (m, media, de, ate) in &o.diferencas {
                linhas.push(format!(
                    "- {}",
                    (t.diferenca_linha)(&metrica(m), &num(*media), &num(*de), &num(*ate))
                ));
            }
        }
        if o.investimento.is_some() || o.vida_anos.is_some() {
            linhas.push(String::new());
            if let Some(v) = o.investimento {
                linhas.push((t.investimento)(&num(v)));
            }
            if let Some(a) = o.vida_anos {
                linhas.push((t.vida)(&num(a)));
            }
        }
        if !o.linha_vsm.is_empty() {
            linhas.push(String::new());
            linhas.push(t.vsm.to_string());
            for l in &o.linha_vsm {
                let no = match l.etapa.as_deref().filter(|e| !e.trim().is_empty()) {
                    Some(e) => format!("{} ({e})", l.no),
                    None => l.no.clone(),
                };
                linhas.push(format!(
                    "- {}",
                    (t.vsm_linha)(
                        &no,
                        &num_ou_traco(l.kwh_por_peca),
                        &num_ou_traco(l.co2e_por_peca)
                    )
                ));
            }
        }
        if !o.testes.is_empty() {
            linhas.push(String::new());
            linhas.push(t.testes.to_string());
            for x in &o.testes {
                linhas.push(format!(
                    "- {}",
                    (t.teste_linha)(
                        &metrica(&x.metrica),
                        &num(x.diferenca),
                        &num(x.p_valor),
                        &x.origem
                    )
                ));
            }
        }
        linhas.push(String::new());
        linhas.push(t.nota_ganho.to_string());
        linhas.join("\n")
    }

    /// A ideia que entra na carteira.
    pub fn submissao(&self) -> Submission {
        let t = l17();
        let o = &self.oportunidade;
        let cenario = if o.cenario.trim().is_empty() {
            self.modelo()
        } else {
            o.cenario.trim().to_string()
        };
        let problema = if o.alavancas.is_empty() {
            (t.problema_sem_alavancas)(&cenario)
        } else {
            (t.problema)(&cenario, &o.alavancas.join("; "))
        };
        Submission {
            title: (t.titulo_ideia)(&cenario),
            process: self.modelo(),
            problem: problema,
            data: self.dados(),
            rough_gain: 0.0,
            source: ORIGEM_AMBIENTAL.into(),
            reference: cenario,
        }
    }

    /// Cria a ideia na carteira em `carteira`, como `me`, com os arquivos do pedido anexados
    /// (o hash é conferido de novo: o arquivo pode ter mudado desde a leitura). Devolve o id
    /// da ideia; o erro vem pronto para a pessoa.
    pub fn enviar(
        &self,
        carteira: &Path,
        me: &Identity,
        clock: &dyn utsim_lss_store::Clock,
    ) -> Result<Id, String> {
        let t = l17();
        self.pedido
            .conferir_entrada()
            .map_err(|e| (t.erro_pedido)(&e.to_string()))?;
        let anexos: Vec<PathBuf> = self
            .pedido
            .entrada
            .arquivos
            .iter()
            .map(|a| a.caminho.clone())
            .collect();
        utsim_lss_store::intake::submit_idea(carteira, me, clock, &self.submissao(), &anexos)
            .map_err(|e| (t.erro_carteira)(&e.to_string()))
    }
}
