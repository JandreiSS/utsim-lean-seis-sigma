//! Texts of the start screen (no project open): the actions, the create and import windows,
//! and the labels of the shared start-screen design and of the file picker.

use utsim_ui_tema::abertura::Rotulos;
use utsim_ui_tema::arquivos::Textos;

use crate::text::{Language, language};

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

static PT_BR: L14Texts = L14Texts {
    subtitle: "Projetos Lean Seis Sigma",
    open_project: "Abrir projeto",
    open_help: "Escolha a pasta .lss de um projeto que já existe, sua ou da equipe.",
    new_help: "Comece um projeto DMAIC, Kaizen ou A3, do zero ou de um modelo pronto.",
    program_help: "Junte vários projetos de uma implantação num programa só.",
    import_project: "Importar projeto (.lssz)",
    import_help: "Traga um projeto que alguém exportou e mandou por e-mail ou pendrive.",
    recent_title: "Projetos recentes",
    where_explanation: "Cada projeto é uma pasta (terminada em .lss) com tudo dentro: termo \
        de abertura, equipe, dados, análises e relatórios. Para trabalhar em equipe, crie a \
        pasta numa pasta sincronizada do Google Drive: cada alteração vira um arquivo novo e \
        nada é sobrescrito.",
    lssz_filter: "Projeto exportado (.lssz)",
    pick_project_folder: "Escolha a pasta do projeto (.lss)",
    not_a_project: |p| {
        format!(
            "A pasta {p} não é um projeto. Escolha a pasta terminada em .lss, ou crie um \
             projeto novo."
        )
    },
    explorer_error: |e| format!("não consegui abrir o explorador: {e}"),
    pick_exported: "Escolha o projeto exportado",
    project_title: "Título do projeto:",
    program_name: "Nome do programa:",
    cancel: "Cancelar",
    exported_file: "Arquivo exportado:",
    import_where: "Onde colocar a pasta do projeto:",
    import_where_title: "Onde colocar o projeto",
    create_where: "Onde criar (para a equipe, escolha uma pasta do Google Drive):",
    create_where_title: "Onde criar o projeto",
    will_create: |p| format!("Será criada a pasta: {p}"),
    start: Rotulos::PT,
    picker: Textos::PT,
};

static EN: L14Texts = L14Texts {
    subtitle: "Lean Six Sigma projects",
    open_project: "Open project",
    open_help: "Choose the .lss folder of an existing project, yours or the team's.",
    new_help: "Start a DMAIC, Kaizen or A3 project, from scratch or from a ready-made template.",
    program_help: "Bring several projects of one deployment together in a single program.",
    import_project: "Import project (.lssz)",
    import_help: "Bring in a project someone exported and sent by e-mail or USB drive.",
    recent_title: "Recent projects",
    where_explanation: "Each project is a folder (ending in .lss) with everything inside: \
        charter, team, data, analyses and reports. To work as a team, create the folder inside \
        a synced Google Drive folder: every change becomes a new file and nothing is \
        overwritten.",
    lssz_filter: "Exported project (.lssz)",
    pick_project_folder: "Choose the project folder (.lss)",
    not_a_project: |p| {
        format!(
            "The folder {p} is not a project. Choose the folder ending in .lss, or create a \
             new project."
        )
    },
    explorer_error: |e| format!("could not open the file explorer: {e}"),
    pick_exported: "Choose the exported project",
    project_title: "Project title:",
    program_name: "Program name:",
    cancel: "Cancel",
    exported_file: "Exported file:",
    import_where: "Where to put the project folder:",
    import_where_title: "Where to put the project",
    create_where: "Where to create it (for a team, choose a Google Drive folder):",
    create_where_title: "Where to create the project",
    will_create: |p| format!("This folder will be created: {p}"),
    start: Rotulos {
        o_que_fazer: "What would you like to do?",
        onde_salvo: "Where it is saved",
        nada_aberto: "Nothing opened yet.",
        nada_aberto_texto: "What you open or create shows up here, one click away.",
        aberto_agora: "Open now:",
        mostrar_no_explorador: "Show in file explorer",
        abrir: "Open",
        nao_encontrada: "not found",
        tirar_da_lista: "Remove from list",
    },
    picker: Textos {
        procurar: "Browse…",
        ajuda: "Opens the file explorer to choose.",
        dica_pasta: "folder: type it or click Browse…",
        dica_arquivo: "file: type it or click Browse…",
        todos_os_arquivos: "All files",
    },
};

static ES: L14Texts = L14Texts {
    subtitle: "Proyectos Lean Seis Sigma",
    open_project: "Abrir proyecto",
    open_help: "Elija la carpeta .lss de un proyecto existente, suyo o del equipo.",
    new_help: "Comience un proyecto DMAIC, Kaizen o A3, desde cero o desde un modelo listo.",
    program_help: "Reúna varios proyectos de un despliegue en un solo programa.",
    import_project: "Importar proyecto (.lssz)",
    import_help: "Traiga un proyecto que alguien exportó y envió por correo o memoria USB.",
    recent_title: "Proyectos recientes",
    where_explanation: "Cada proyecto es una carpeta (terminada en .lss) con todo dentro: \
        acta de constitución, equipo, datos, análisis e informes. Para trabajar en equipo, \
        cree la carpeta dentro de una carpeta sincronizada de Google Drive: cada cambio se \
        vuelve un archivo nuevo y nada se sobrescribe.",
    lssz_filter: "Proyecto exportado (.lssz)",
    pick_project_folder: "Elija la carpeta del proyecto (.lss)",
    not_a_project: |p| {
        format!(
            "La carpeta {p} no es un proyecto. Elija la carpeta terminada en .lss, o cree un \
             proyecto nuevo."
        )
    },
    explorer_error: |e| format!("no se pudo abrir el explorador: {e}"),
    pick_exported: "Elija el proyecto exportado",
    project_title: "Título del proyecto:",
    program_name: "Nombre del programa:",
    cancel: "Cancelar",
    exported_file: "Archivo exportado:",
    import_where: "Dónde colocar la carpeta del proyecto:",
    import_where_title: "Dónde colocar el proyecto",
    create_where: "Dónde crearlo (para el equipo, elija una carpeta de Google Drive):",
    create_where_title: "Dónde crear el proyecto",
    will_create: |p| format!("Se creará la carpeta: {p}"),
    start: Rotulos {
        o_que_fazer: "¿Qué desea hacer?",
        onde_salvo: "Dónde se guarda",
        nada_aberto: "Nada abierto todavía.",
        nada_aberto_texto: "Lo que abra o cree aparece aquí, a un clic de distancia.",
        aberto_agora: "Abierto ahora:",
        mostrar_no_explorador: "Mostrar en el explorador",
        abrir: "Abrir",
        nao_encontrada: "no encontrada",
        tirar_da_lista: "Quitar de la lista",
    },
    picker: Textos {
        procurar: "Buscar…",
        ajuda: "Abre el explorador de archivos para elegir.",
        dica_pasta: "carpeta: escríbala o haga clic en Buscar…",
        dica_arquivo: "archivo: escríbalo o haga clic en Buscar…",
        todos_os_arquivos: "Todos los archivos",
    },
};

pub fn l14() -> &'static L14Texts {
    match language() {
        Language::PtBr => &PT_BR,
        Language::En => &EN,
        Language::Es => &ES,
    }
}
