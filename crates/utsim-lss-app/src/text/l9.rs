//! Texts of the DMAIC roadmap screen, in a catalogue of their own.

type S = &'static str;

pub struct L9Texts {
    pub nav_roadmap: S,
    pub intro: S,
    pub zoom: S,
    pub legend_done: S,
    pub legend_todo: S,
    pub legend_soon: S,
    pub soon: S,
    pub yes: S,
    pub no: S,
}

pub static PT_BR: L9Texts = L9Texts {
    nav_roadmap: "Roteiro DMAIC",
    intro: "As cinco fases com os passos de cada uma e as ferramentas indicadas ao lado. Clique numa ferramenta para abrir a tela dela neste projeto; as cinza ainda não estão no app. O verde marca o que o projeto já tem: um entregável com conteúdo, uma análise guardada ou um item verificado do tollgate.",
    zoom: "Zoom",
    legend_done: "✔ feito no projeto",
    legend_todo: "a fazer",
    legend_soon: "em breve: ainda não está no app",
    soon: "em breve",
    yes: "sim",
    no: "não",
};

pub fn l9() -> &'static L9Texts {
    &PT_BR
}
