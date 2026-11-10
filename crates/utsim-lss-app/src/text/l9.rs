//! Texts of the DMAIC roadmap screen, in a catalogue of their own.

use crate::text::{Language, language};

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

pub static EN: L9Texts = L9Texts {
    nav_roadmap: "DMAIC roadmap",
    intro: "The five phases with the steps of each one and the recommended tools beside them. Click a tool to open its screen in this project; the grey ones are not in the app yet. Green marks what the project already has: a deliverable with content, a saved analysis or a verified tollgate item.",
    zoom: "Zoom",
    legend_done: "✔ done in the project",
    legend_todo: "to do",
    legend_soon: "coming soon: not in the app yet",
    soon: "coming soon",
    yes: "yes",
    no: "no",
};

pub static ES: L9Texts = L9Texts {
    nav_roadmap: "Hoja de ruta DMAIC",
    intro: "Las cinco fases con los pasos de cada una y las herramientas indicadas al lado. Haga clic en una herramienta para abrir su pantalla en este proyecto; las grises aún no están en la app. El verde marca lo que el proyecto ya tiene: un entregable con contenido, un análisis guardado o un ítem verificado del tollgate.",
    zoom: "Zoom",
    legend_done: "✔ hecho en el proyecto",
    legend_todo: "por hacer",
    legend_soon: "próximamente: aún no está en la app",
    soon: "próximamente",
    yes: "sí",
    no: "no",
};

pub fn l9() -> &'static L9Texts {
    match language() {
        Language::PtBr => &PT_BR,
        Language::En => &EN,
        Language::Es => &ES,
    }
}
