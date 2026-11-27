//! Textos do fluxo do projeto: o painel com a trilha das fases e o "agora faça", a barra
//! com "Mais", o termo e o portão com o essencial à vista.

use crate::text::tx;
use utsim_idiomas::PorIdioma;

type S = &'static str;

pub struct L16Texts {
    pub detalhes: S,
    pub portao: S,
    pub dica_portao: S,
    pub dica_estacao: S,
    pub portao_sem: S,
    pub portao_aberto: S,
    pub portao_enviado: S,
    pub portao_aprovado: S,
    pub portao_refazer: S,
    pub portao_suspenso: S,
    pub portao_cancelado: S,
    pub agora_faca: S,
    pub proximo_passo: S,
    pub abrir: S,
    pub ir_ao_portao: S,
    pub projeto_concluido: S,
    pub passos_da_fase: S,
    pub do_checklist: S,
    pub entregaveis: S,
    pub sem_entregaveis: S,
    pub mais: S,
    pub mais_telas: S,
    pub menos_telas: S,
    pub termo_mais_campos: S,
    pub termo_versoes: S,
    pub termo_pedir_aprovacao: S,
    pub termo_pronto: S,
    pub portao_avancado: S,
    pub painel_tabelas: S,
    pub desfazer: S,
    pub refazer: S,
    pub desfazer_ajuda: S,
    pub refazer_ajuda: S,
    pub desfeito: S,
    pub refeito: S,
    pub nada_para_desfazer: S,
    pub nada_para_refazer: S,
}

pub fn l16() -> &'static L16Texts {
    static CACHE: PorIdioma<L16Texts> = PorIdioma::new();
    CACHE.obter(construir)
}

fn construir() -> L16Texts {
    L16Texts {
        detalhes: tx("l16-detalhes"),
        portao: tx("l16-portao"),
        dica_portao: tx("l16-dica_portao"),
        dica_estacao: tx("l16-dica_estacao"),
        portao_sem: tx("l16-portao_sem"),
        portao_aberto: tx("l16-portao_aberto"),
        portao_enviado: tx("l16-portao_enviado"),
        portao_aprovado: tx("l16-portao_aprovado"),
        portao_refazer: tx("l16-portao_refazer"),
        portao_suspenso: tx("l16-portao_suspenso"),
        portao_cancelado: tx("l16-portao_cancelado"),
        agora_faca: tx("l16-agora_faca"),
        proximo_passo: tx("l16-proximo_passo"),
        abrir: tx("l16-abrir"),
        ir_ao_portao: tx("l16-ir_ao_portao"),
        projeto_concluido: tx("l16-projeto_concluido"),
        passos_da_fase: tx("l16-passos_da_fase"),
        do_checklist: tx("l16-do_checklist"),
        entregaveis: tx("l16-entregaveis"),
        sem_entregaveis: tx("l16-sem_entregaveis"),
        mais: tx("l16-mais"),
        mais_telas: tx("l16-mais_telas"),
        menos_telas: tx("l16-menos_telas"),
        termo_mais_campos: tx("l16-termo_mais_campos"),
        termo_versoes: tx("l16-termo_versoes"),
        termo_pedir_aprovacao: tx("l16-termo_pedir_aprovacao"),
        termo_pronto: tx("l16-termo_pronto"),
        portao_avancado: tx("l16-portao_avancado"),
        painel_tabelas: tx("l16-painel_tabelas"),
        desfazer: tx("l16-desfazer"),
        refazer: tx("l16-refazer_botao"),
        desfazer_ajuda: tx("l16-desfazer_ajuda"),
        refazer_ajuda: tx("l16-refazer_ajuda"),
        desfeito: tx("l16-desfeito"),
        refeito: tx("l16-refeito"),
        nada_para_desfazer: tx("l16-nada_para_desfazer"),
        nada_para_refazer: tx("l16-nada_para_refazer"),
    }
}
