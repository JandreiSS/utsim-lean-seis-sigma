//! Textos dos pedidos de outros apps da família: a oportunidade que o UTSim Ambiental manda
//! (`nova_oportunidade`), a janela que a recebe e o texto da ideia criada na carteira.

use crate::text::{fx, tx};
use utsim_idiomas::PorIdioma;

type S = &'static str;

pub struct L17Texts {
    pub janela: S,
    pub intro: S,
    pub carteira: S,
    pub carteira_dica: S,
    pub criar: S,
    pub agora_nao: S,
    pub sem_carteira: S,
    pub nota_ganho: S,
    pub alavancas: S,
    pub diferencas: S,
    pub vsm: S,
    pub testes: S,
    pub m_co2e: S,
    pub m_energia: S,
    pub m_agua: S,
    pub m_conta: S,
    pub m_custo: S,
    pub sem_resumo: S,
    pub app_ambiental: S,
    pub criada: fn(&str) -> String,
    pub erro_acao: fn(&str) -> String,
    pub erro_pedido: fn(&str) -> String,
    pub erro_carteira: fn(&str) -> String,
    pub titulo_ideia: fn(&str) -> String,
    pub problema: fn(&str, &str) -> String,
    pub problema_sem_alavancas: fn(&str) -> String,
    pub cabecalho: fn(&str, &str) -> String,
    pub diferenca_linha: fn(&str, &str, &str, &str) -> String,
    pub investimento: fn(&str) -> String,
    pub vida: fn(&str) -> String,
    pub vsm_linha: fn(&str, &str, &str) -> String,
    pub teste_linha: fn(&str, &str, &str, &str) -> String,
}

pub fn l17() -> &'static L17Texts {
    static CACHE: PorIdioma<L17Texts> = PorIdioma::new();
    CACHE.obter(construir)
}

fn construir() -> L17Texts {
    L17Texts {
        janela: tx("l17-janela"),
        intro: tx("l17-intro"),
        carteira: tx("l17-carteira"),
        carteira_dica: tx("l17-carteira_dica"),
        criar: tx("l17-criar"),
        agora_nao: tx("l17-agora_nao"),
        sem_carteira: tx("l17-sem_carteira"),
        nota_ganho: tx("l17-nota_ganho"),
        alavancas: tx("l17-alavancas"),
        diferencas: tx("l17-diferencas"),
        vsm: tx("l17-vsm"),
        testes: tx("l17-testes"),
        m_co2e: tx("l17-m_co2e"),
        m_energia: tx("l17-m_energia"),
        m_agua: tx("l17-m_agua"),
        m_conta: tx("l17-m_conta"),
        m_custo: tx("l17-m_custo"),
        sem_resumo: tx("l17-sem_resumo"),
        app_ambiental: tx("l17-app_ambiental"),
        criada: |t| fx("l17-criada", &[("titulo", t.into())]),
        erro_acao: |a| fx("l17-erro_acao", &[("acao", a.into())]),
        erro_pedido: |e| fx("l17-erro_pedido", &[("erro", e.into())]),
        erro_carteira: |e| fx("l17-erro_carteira", &[("erro", e.into())]),
        titulo_ideia: |c| fx("l17-titulo_ideia", &[("cenario", c.into())]),
        problema: |c, a| {
            fx(
                "l17-problema",
                &[("cenario", c.into()), ("alavancas", a.into())],
            )
        },
        problema_sem_alavancas: |c| fx("l17-problema_sem_alavancas", &[("cenario", c.into())]),
        cabecalho: |m, c| {
            fx(
                "l17-cabecalho",
                &[("modelo", m.into()), ("cenario", c.into())],
            )
        },
        diferenca_linha: |m, media, de, ate| {
            fx(
                "l17-diferenca_linha",
                &[
                    ("metrica", m.into()),
                    ("media", media.into()),
                    ("de", de.into()),
                    ("ate", ate.into()),
                ],
            )
        },
        investimento: |v| fx("l17-investimento", &[("valor", v.into())]),
        vida: |a| fx("l17-vida", &[("anos", a.into())]),
        vsm_linha: |no, kwh, co2e| {
            fx(
                "l17-vsm_linha",
                &[
                    ("no", no.into()),
                    ("kwh", kwh.into()),
                    ("co2e", co2e.into()),
                ],
            )
        },
        teste_linha: |m, dif, p, origem| {
            fx(
                "l17-teste_linha",
                &[
                    ("metrica", m.into()),
                    ("dif", dif.into()),
                    ("p", p.into()),
                    ("origem", origem.into()),
                ],
            )
        },
    }
}
