//! Texts of the start screen (no project open): the actions, the create and import windows,
//! and the labels of the shared start-screen design and of the file picker.

use utsim_ui_tema::abertura::Rotulos;
use utsim_ui_tema::arquivos::Textos;

use crate::text::{tx, fx};
use utsim_idiomas::PorIdioma;

type S = &'static str;
type F1 = fn(&str) -> String;

pub struct L14Texts {
    pub subtitle: S,
    pub open_project: S,
    pub open_help: S,
    pub new_help: S,
    pub program_help: S,
    pub import_project: S,
    pub import_help: S,
    pub recent_title: S,
    pub where_explanation: S,
    pub lssz_filter: S,
    pub pick_project_folder: S,
    pub not_a_project: F1,
    pub explorer_error: F1,
    pub pick_exported: S,
    pub project_title: S,
    pub program_name: S,
    pub cancel: S,
    pub exported_file: S,
    pub import_where: S,
    pub import_where_title: S,
    pub create_where: S,
    pub create_where_title: S,
    pub will_create: F1,
    pub start: Rotulos<'static>,
    pub picker: Textos<'static>,
}







pub fn l14() -> &'static L14Texts {
    static CACHE: PorIdioma<L14Texts> = PorIdioma::new();
    CACHE.obter(construir_l14_texts)
}

fn construir_l14_texts() -> L14Texts {
    L14Texts {
        subtitle: tx("l14-subtitle"),
        open_project: tx("l14-open_project"),
        open_help: tx("l14-open_help"),
        new_help: tx("l14-new_help"),
        program_help: tx("l14-program_help"),
        import_project: tx("l14-import_project"),
        import_help: tx("l14-import_help"),
        recent_title: tx("l14-recent_title"),
        where_explanation: tx("l14-where_explanation"),
        lssz_filter: tx("l14-lssz_filter"),
        pick_project_folder: tx("l14-pick_project_folder"),
        not_a_project: |p| fx("l14-not_a_project", &[("p", p.into())]),
        explorer_error: |e| fx("l14-explorer_error", &[("e", e.into())]),
        pick_exported: tx("l14-pick_exported"),
        project_title: tx("l14-project_title"),
        program_name: tx("l14-program_name"),
        cancel: tx("l14-cancel"),
        exported_file: tx("l14-exported_file"),
        import_where: tx("l14-import_where"),
        import_where_title: tx("l14-import_where_title"),
        create_where: tx("l14-create_where"),
        create_where_title: tx("l14-create_where_title"),
        will_create: |p| fx("l14-will_create", &[("p", p.into())]),
        start: Rotulos { o_que_fazer: tx("l14-start-o_que_fazer"), onde_salvo: tx("l14-start-onde_salvo"), nada_aberto: tx("l14-start-nada_aberto"), nada_aberto_texto: tx("l14-start-nada_aberto_texto"), aberto_agora: tx("l14-start-aberto_agora"), mostrar_no_explorador: tx("l14-start-mostrar_no_explorador"), abrir: tx("l14-start-abrir"), nao_encontrada: tx("l14-start-nao_encontrada"), tirar_da_lista: tx("l14-start-tirar_da_lista") },
        picker: Textos { procurar: tx("l14-picker-procurar"), ajuda: tx("l14-picker-ajuda"), dica_pasta: tx("l14-picker-dica_pasta"), dica_arquivo: tx("l14-picker-dica_arquivo"), todos_os_arquivos: tx("l14-picker-todos_os_arquivos") },
    }
}
