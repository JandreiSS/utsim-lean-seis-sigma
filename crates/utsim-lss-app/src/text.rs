//! Every text of the interface, in one catalogue. Portuguese (Brazil) first; another
//! language is another `Texts` value (`docs/plano_lean_seis_sigma.md`, 4.8, P8). Screens
//! never write literal text: they read `texts().x`.

use utsim_lss_core::domain::{Approach, Column, Origin, PhaseStatus, SavingStatus};
use utsim_lss_core::tools::ap::Ap;
use utsim_lss_core::tools::ishikawa::CauseStatus;
use utsim_lss_core::tools::plans::CollectionIssue;
use utsim_lss_core::tools::voc::{Issue, Kano};
use utsim_lss_core::tools::{DeliverableStatus, Tool};

pub struct Texts {
    pub app_name: &'static str,

    // First run.
    pub identity_title: &'static str,
    pub identity_intro: &'static str,
    pub identity_name: &'static str,
    pub identity_email: &'static str,
    pub identity_note: &'static str,
    pub continue_: &'static str,

    // Start screen.
    pub projects: &'static str,
    pub recent: &'static str,
    pub no_recent: &'static str,
    pub missing_folder: &'static str,
    pub open_folder: &'static str,
    pub open: &'static str,
    pub path_hint: &'static str,
    pub new_project: &'static str,
    pub title_hint: &'static str,
    pub parent_hint: &'static str,
    pub approach: &'static str,
    /// Project template of a specialised line (`utsim_lss_core::modelos_linha`).
    pub project_model: &'static str,
    pub no_project_model: &'static str,
    pub create: &'static str,
    pub import_lssz: &'static str,
    pub import_hint: &'static str,
    pub import_dest_hint: &'static str,
    pub import: &'static str,
    pub shared_folder_tip: &'static str,
    pub who_am_i: fn(&str, &str) -> String,
    pub change_identity: &'static str,

    // Navigation.
    pub nav_portfolio: &'static str,
    pub nav_project: &'static str,
    pub nav_overview: &'static str,
    pub nav_charter: &'static str,
    pub nav_phases: &'static str,
    pub nav_actions: &'static str,
    pub nav_risks: &'static str,
    pub nav_meetings: &'static str,
    pub nav_schedule: &'static str,
    pub nav_savings: &'static str,
    pub nav_tools: &'static str,
    pub nav_data: &'static str,
    pub nav_conflicts: &'static str,
    pub close_project: &'static str,
    pub coming_in: fn(&str) -> String,

    // Theme.
    pub theme: &'static str,
    pub theme_system: &'static str,
    pub theme_light: &'static str,
    pub theme_dark: &'static str,
    pub language: &'static str,

    // Overview.
    pub current_phase: &'static str,
    pub phases: &'static str,
    pub actions: &'static str,
    pub actions_done: fn(usize, usize) -> String,
    pub overdue: &'static str,
    pub none_overdue: &'static str,
    pub risks_open: &'static str,
    pub risks_high: &'static str,
    pub savings: &'static str,
    pub meetings: &'static str,
    pub deadline: &'static str,
    pub days_left: fn(i64) -> String,
    pub no_deadline: &'static str,
    pub conflicts: &'static str,
    pub folder: &'static str,
    pub operations: fn(usize) -> String,

    // Charter.
    pub charter_intro: &'static str,
    pub f_title: &'static str,
    pub f_process: &'static str,
    pub f_area: &'static str,
    pub f_problem: &'static str,
    pub f_goal: &'static str,
    pub f_baseline: &'static str,
    pub f_deadline: &'static str,
    pub f_scope_in: &'static str,
    pub f_scope_out: &'static str,
    pub f_business_case: &'static str,
    pub date_hint: &'static str,
    pub bad_date: &'static str,

    // Board.
    pub new_action_hint: &'static str,
    pub add: &'static str,
    pub board_tip: &'static str,
    pub a_what: &'static str,
    pub a_why: &'static str,
    pub a_where: &'static str,
    pub a_when: &'static str,
    pub a_who: &'static str,
    pub a_how: &'static str,
    pub a_how_much: &'static str,
    pub a_column: &'static str,
    pub nobody: &'static str,
    pub delete: &'static str,
    pub close: &'static str,
    pub move_left: &'static str,
    pub move_right: &'static str,
    pub move_up: &'static str,
    pub move_down: &'static str,
    pub untitled: &'static str,

    // Presence and conflicts.
    pub here_now: &'static str,
    pub is_on: fn(&str, &str) -> String,
    pub is_editing_this: fn(&str) -> String,
    pub conflicts_banner: fn(usize) -> String,
    pub review: &'static str,
    pub conflicts_intro: &'static str,
    pub no_conflicts: &'static str,
    pub kept: &'static str,
    pub replaced: &'static str,
    pub keep_this: &'static str,
    pub restore_other: &'static str,
    pub by_whom: fn(&str) -> String,
    pub pending_files: fn(usize) -> String,

    // Data.
    pub data_intro: &'static str,
    pub export_lssz: &'static str,
    pub export_dest_hint: &'static str,
    pub exported: fn(&str) -> String,
    pub imported: fn(&str) -> String,
    pub created: fn(&str) -> String,
    pub error: fn(&str) -> String,

    // Screens still to come, with what they will hold.
    pub soon_portfolio: &'static str,
    pub soon_phases: &'static str,
    pub soon_risks: &'static str,
    pub soon_meetings: &'static str,
    pub soon_schedule: &'static str,
    pub soon_savings: &'static str,
    pub soon_data: &'static str,
}

pub static PT_BR: Texts = Texts {
    app_name: "UTSim Seis Sigma",

    identity_title: "Bem-vindo ao UTSim Seis Sigma",
    identity_intro: "Antes de começar, diga quem você é. O seu nome vai em cada alteração que você fizer nos projetos, para a equipe saber quem mudou o quê e quando.",
    identity_name: "Nome",
    identity_email: "E-mail",
    identity_note: "Fica guardado só neste computador, na sua pasta de usuário, e não dentro dos projetos.",
    continue_: "Continuar",

    projects: "Projetos",
    recent: "Recentes",
    no_recent: "Nenhum projeto aberto ainda.",
    missing_folder: "pasta não encontrada",
    open_folder: "Abrir uma pasta de projeto (.lss)",
    open: "Abrir",
    path_hint: "caminho da pasta, por exemplo G:\\Meu Drive\\Melhoria\\Linha 2.lss",
    new_project: "Novo projeto",
    title_hint: "título do projeto",
    parent_hint: "pasta onde criar (a pasta compartilhada da equipe, por exemplo)",
    approach: "Abordagem",
    project_model: "Modelo de projeto",
    no_project_model: "Nenhum (só a abordagem)",
    create: "Criar",
    import_lssz: "Importar um arquivo .lssz",
    import_hint: "caminho do arquivo .lssz",
    import_dest_hint: "pasta onde colocar o projeto",
    import: "Importar",
    shared_folder_tip: "Para trabalhar em equipe, crie o projeto numa pasta compartilhada do Google Drive (Drive para computador). Cada alteração vira um arquivo novo e nada é sobrescrito.",
    who_am_i: |name, email| format!("Você: {name} <{email}>"),
    change_identity: "Trocar",

    nav_portfolio: "Carteira",
    nav_project: "Projeto",
    nav_overview: "Visão geral",
    nav_charter: "Termo de abertura",
    nav_phases: "Fases e tollgates",
    nav_actions: "Ações",
    nav_risks: "Riscos",
    nav_meetings: "Reuniões",
    nav_schedule: "Cronograma",
    nav_savings: "Ganhos",
    nav_tools: "Ferramentas",
    nav_data: "Dados",
    nav_conflicts: "Conflitos",
    close_project: "Fechar projeto",
    coming_in: |phase| format!("Esta tela chega na etapa {phase} do plano."),

    theme: "Tema",
    theme_system: "Sistema",
    theme_light: "Claro",
    theme_dark: "Escuro",
    language: "Idioma",

    current_phase: "Fase atual",
    phases: "Fases",
    actions: "Ações",
    actions_done: |done, total| format!("{done} de {total} concluídas"),
    overdue: "Ações atrasadas",
    none_overdue: "Nenhuma ação atrasada.",
    risks_open: "Riscos abertos",
    risks_high: "riscos altos",
    savings: "Ganhos",
    meetings: "Reuniões",
    deadline: "Prazo",
    days_left: |d| match d {
        0 => "vence hoje".into(),
        1 => "falta 1 dia".into(),
        d if d > 1 => format!("faltam {d} dias"),
        -1 => "atrasado 1 dia".into(),
        d => format!("atrasado {} dias", -d),
    },
    no_deadline: "sem prazo definido (veja o termo de abertura)",
    conflicts: "Conflitos a revisar",
    folder: "Pasta",
    operations: |n| format!("{n} alterações no diário"),

    charter_intro: "Os campos são gravados quando você sai deles. Mudanças de meta ou escopo depois da aprovação vão pedir nova aprovação (etapa L1).",
    f_title: "Título",
    f_process: "Processo",
    f_area: "Área",
    f_problem: "Declaração do problema (o quê, onde, quando, quanto; sem causa nem solução)",
    f_goal: "Meta",
    f_baseline: "Linha de base",
    f_deadline: "Prazo",
    f_scope_in: "Dentro do escopo",
    f_scope_out: "Fora do escopo",
    f_business_case: "Caso de negócio",
    date_hint: "DD/MM/AAAA",
    bad_date: "Data inválida: use DD/MM/AAAA.",

    new_action_hint: "nova ação: o que fazer",
    add: "Adicionar",
    board_tip: "Arraste um cartão pela alça ≡ para outra coluna, ou use as setas no painel do cartão.",
    a_what: "O quê",
    a_why: "Por quê",
    a_where: "Onde",
    a_when: "Quando (prazo)",
    a_who: "Quem",
    a_how: "Como",
    a_how_much: "Quanto",
    a_column: "Coluna",
    nobody: "(ninguém)",
    delete: "Excluir",
    close: "Fechar",
    move_left: "⬅",
    move_right: "➡",
    move_up: "⬆",
    move_down: "⬇",
    untitled: "(sem título)",

    here_now: "Com o projeto aberto:",
    is_on: |name, screen| format!("{name} está em {screen}"),
    is_editing_this: |name| {
        format!("{name} está editando isto agora. Combine com a pessoa antes de mudar.")
    },
    conflicts_banner: |n| {
        if n == 1 {
            "1 conflito a revisar: duas pessoas mudaram o mesmo campo sem ver a mudança uma da outra.".into()
        } else {
            format!(
                "{n} conflitos a revisar: duas pessoas mudaram o mesmo campo sem ver a mudança uma da outra."
            )
        }
    },
    review: "Revisar",
    conflicts_intro: "Quando duas pessoas mudam o mesmo campo sem ver a mudança uma da outra, fica valendo a mais recente e a outra aparece aqui. Mantenha a atual ou restaure a outra.",
    no_conflicts: "Nenhum conflito a revisar.",
    kept: "Valendo agora",
    replaced: "Substituído",
    keep_this: "Manter o atual",
    restore_other: "Restaurar o outro",
    by_whom: |name| format!("por {name}"),
    pending_files: |n| {
        if n == 1 {
            "1 arquivo ainda chegando pela sincronização (ou de uma versão mais nova do programa)."
                .into()
        } else {
            format!(
                "{n} arquivos ainda chegando pela sincronização (ou de uma versão mais nova do programa)."
            )
        }
    },

    data_intro: "Conjuntos de dados (CSV e planilhas, com origem Real, Gêmeo ou Simulado) entram na etapa L3. Aqui já dá para levar o projeto inteiro num arquivo só.",
    export_lssz: "Exportar .lssz",
    export_dest_hint: "arquivo de destino (.lssz)",
    exported: |p| format!("Projeto exportado para {p}"),
    imported: |p| format!("Projeto importado em {p}"),
    created: |p| format!("Projeto criado em {p}"),
    error: |e| format!("Erro: {e}"),

    soon_portfolio: "Banco de ideias, priorização (matriz impacto × esforço, pontuação ponderada, alinhamento estratégico) e a carga dos belts, somando os projetos desta pasta.",
    soon_phases: "Entregáveis de cada fase, checklist do tollgate (itens manuais e verificados), decisões dos aprovadores e congelamento por hash.",
    soon_risks: "Registro de riscos com a matriz probabilidade × impacto e as respostas que viram ações.",
    soon_meetings: "Pauta, participantes, ata, decisões e ações criadas a partir da ata.",
    soon_schedule: "Gantt com linha de base, caminho crítico, folgas e probabilidade de cumprir cada tollgate pelo PERT e pelo Monte Carlo, em dias úteis.",
    soon_savings: "Ganhos hard e soft com os estados estimado, previsto, realizado e validado, e o acompanhamento de 12 meses.",
    soon_data: "Importação de CSV e planilhas com detecção de separador e vírgula decimal, conjuntos imutáveis com hash e origem.",
};

pub static EN: Texts = Texts {
    app_name: "UTSim Seis Sigma",

    identity_title: "Welcome to UTSim Seis Sigma",
    identity_intro: "Before you start, tell us who you are. Your name goes on every change you make to the projects, so the team knows who changed what and when.",
    identity_name: "Name",
    identity_email: "E-mail",
    identity_note: "It is kept only on this computer, in your user folder, and not inside the projects.",
    continue_: "Continue",

    projects: "Projects",
    recent: "Recent",
    no_recent: "No project opened yet.",
    missing_folder: "folder not found",
    open_folder: "Open a project folder (.lss)",
    open: "Open",
    path_hint: "folder path, for example G:\\My Drive\\Improvement\\Line 2.lss",
    new_project: "New project",
    title_hint: "project title",
    parent_hint: "folder where to create it (the team's shared folder, for example)",
    approach: "Approach",
    project_model: "Project template",
    no_project_model: "None (approach only)",
    create: "Create",
    import_lssz: "Import an .lssz file",
    import_hint: "path of the .lssz file",
    import_dest_hint: "folder where to put the project",
    import: "Import",
    shared_folder_tip: "To work as a team, create the project in a shared Google Drive folder (Drive for desktop). Each change becomes a new file and nothing is overwritten.",
    who_am_i: |name, email| format!("You: {name} <{email}>"),
    change_identity: "Change",

    nav_portfolio: "Portfolio",
    nav_project: "Project",
    nav_overview: "Overview",
    nav_charter: "Project charter",
    nav_phases: "Phases and tollgates",
    nav_actions: "Actions",
    nav_risks: "Risks",
    nav_meetings: "Meetings",
    nav_schedule: "Schedule",
    nav_savings: "Savings",
    nav_tools: "Tools",
    nav_data: "Data",
    nav_conflicts: "Conflicts",
    close_project: "Close project",
    coming_in: |phase| format!("This screen arrives in stage {phase} of the plan."),

    theme: "Theme",
    theme_system: "System",
    theme_light: "Light",
    theme_dark: "Dark",
    language: "Language",

    current_phase: "Current phase",
    phases: "Phases",
    actions: "Actions",
    actions_done: |done, total| format!("{done} of {total} done"),
    overdue: "Overdue actions",
    none_overdue: "No overdue actions.",
    risks_open: "Open risks",
    risks_high: "high risks",
    savings: "Savings",
    meetings: "Meetings",
    deadline: "Deadline",
    days_left: |d| match d {
        0 => "due today".into(),
        1 => "1 day left".into(),
        d if d > 1 => format!("{d} days left"),
        -1 => "1 day overdue".into(),
        d => format!("{} days overdue", -d),
    },
    no_deadline: "no deadline set (see the project charter)",
    conflicts: "Conflicts to review",
    folder: "Folder",
    operations: |n| {
        if n == 1 {
            "1 change in the journal".into()
        } else {
            format!("{n} changes in the journal")
        }
    },

    charter_intro: "Fields are saved when you leave them. Changes to the goal or scope after approval will ask for a new approval (stage L1).",
    f_title: "Title",
    f_process: "Process",
    f_area: "Area",
    f_problem: "Problem statement (what, where, when, how much; no cause or solution)",
    f_goal: "Goal",
    f_baseline: "Baseline",
    f_deadline: "Deadline",
    f_scope_in: "In scope",
    f_scope_out: "Out of scope",
    f_business_case: "Business case",
    date_hint: "DD/MM/YYYY",
    bad_date: "Invalid date: use DD/MM/YYYY.",

    new_action_hint: "new action: what to do",
    add: "Add",
    board_tip: "Drag a card by the ≡ handle to another column, or use the arrows in the card panel.",
    a_what: "What",
    a_why: "Why",
    a_where: "Where",
    a_when: "When (deadline)",
    a_who: "Who",
    a_how: "How",
    a_how_much: "How much",
    a_column: "Column",
    nobody: "(nobody)",
    delete: "Delete",
    close: "Close",
    move_left: "⬅",
    move_right: "➡",
    move_up: "⬆",
    move_down: "⬇",
    untitled: "(untitled)",

    here_now: "With the project open:",
    is_on: |name, screen| format!("{name} is on {screen}"),
    is_editing_this: |name| {
        format!("{name} is editing this right now. Check with them before changing it.")
    },
    conflicts_banner: |n| {
        if n == 1 {
            "1 conflict to review: two people changed the same field without seeing each other's change.".into()
        } else {
            format!(
                "{n} conflicts to review: two people changed the same field without seeing each other's change."
            )
        }
    },
    review: "Review",
    conflicts_intro: "When two people change the same field without seeing each other's change, the most recent one stands and the other appears here. Keep the current one or restore the other.",
    no_conflicts: "No conflicts to review.",
    kept: "Current",
    replaced: "Replaced",
    keep_this: "Keep the current one",
    restore_other: "Restore the other",
    by_whom: |name| format!("by {name}"),
    pending_files: |n| {
        if n == 1 {
            "1 file still arriving through sync (or from a newer version of the program).".into()
        } else {
            format!(
                "{n} files still arriving through sync (or from a newer version of the program)."
            )
        }
    },

    data_intro: "Data sets (CSV and spreadsheets, with origin Real, Twin or Simulated) arrive in stage L3. Here you can already take the whole project in a single file.",
    export_lssz: "Export .lssz",
    export_dest_hint: "destination file (.lssz)",
    exported: |p| format!("Project exported to {p}"),
    imported: |p| format!("Project imported into {p}"),
    created: |p| format!("Project created in {p}"),
    error: |e| format!("Error: {e}"),

    soon_portfolio: "Idea bank, prioritisation (impact × effort matrix, weighted score, strategic alignment) and the belts' workload, adding up the projects in this folder.",
    soon_phases: "Deliverables of each phase, tollgate checklist (manual and verified items), approvers' decisions and freezing by hash.",
    soon_risks: "Risk register with the probability × impact matrix and the responses that become actions.",
    soon_meetings: "Agenda, attendees, minutes, decisions and actions created from the minutes.",
    soon_schedule: "Gantt with baseline, critical path, float and the probability of meeting each tollgate by PERT and Monte Carlo, in working days.",
    soon_savings: "Hard and soft savings with the states estimated, forecast, realized and validated, and 12-month tracking.",
    soon_data: "Import of CSV and spreadsheets with separator and decimal comma detection, immutable data sets with hash and origin.",
};

pub static ES: Texts = Texts {
    app_name: "UTSim Seis Sigma",

    identity_title: "Bienvenido a UTSim Seis Sigma",
    identity_intro: "Antes de empezar, díganos quién es. Su nombre va en cada cambio que haga en los proyectos, para que el equipo sepa quién cambió qué y cuándo.",
    identity_name: "Nombre",
    identity_email: "Correo electrónico",
    identity_note: "Se guarda solo en este equipo, en su carpeta de usuario, y no dentro de los proyectos.",
    continue_: "Continuar",

    projects: "Proyectos",
    recent: "Recientes",
    no_recent: "Ningún proyecto abierto todavía.",
    missing_folder: "carpeta no encontrada",
    open_folder: "Abrir una carpeta de proyecto (.lss)",
    open: "Abrir",
    path_hint: "ruta de la carpeta, por ejemplo G:\\Mi unidad\\Mejora\\Línea 2.lss",
    new_project: "Nuevo proyecto",
    title_hint: "título del proyecto",
    parent_hint: "carpeta donde crearlo (la carpeta compartida del equipo, por ejemplo)",
    approach: "Enfoque",
    project_model: "Plantilla de proyecto",
    no_project_model: "Ninguna (solo el enfoque)",
    create: "Crear",
    import_lssz: "Importar un archivo .lssz",
    import_hint: "ruta del archivo .lssz",
    import_dest_hint: "carpeta donde poner el proyecto",
    import: "Importar",
    shared_folder_tip: "Para trabajar en equipo, cree el proyecto en una carpeta compartida de Google Drive (Drive para escritorio). Cada cambio se convierte en un archivo nuevo y nada se sobrescribe.",
    who_am_i: |name, email| format!("Usted: {name} <{email}>"),
    change_identity: "Cambiar",

    nav_portfolio: "Cartera",
    nav_project: "Proyecto",
    nav_overview: "Visión general",
    nav_charter: "Acta de constitución",
    nav_phases: "Fases y tollgates",
    nav_actions: "Acciones",
    nav_risks: "Riesgos",
    nav_meetings: "Reuniones",
    nav_schedule: "Cronograma",
    nav_savings: "Ahorros",
    nav_tools: "Herramientas",
    nav_data: "Datos",
    nav_conflicts: "Conflictos",
    close_project: "Cerrar proyecto",
    coming_in: |phase| format!("Esta pantalla llega en la etapa {phase} del plan."),

    theme: "Tema",
    theme_system: "Sistema",
    theme_light: "Claro",
    theme_dark: "Oscuro",
    language: "Idioma",

    current_phase: "Fase actual",
    phases: "Fases",
    actions: "Acciones",
    actions_done: |done, total| format!("{done} de {total} concluidas"),
    overdue: "Acciones atrasadas",
    none_overdue: "Ninguna acción atrasada.",
    risks_open: "Riesgos abiertos",
    risks_high: "riesgos altos",
    savings: "Ahorros",
    meetings: "Reuniones",
    deadline: "Plazo",
    days_left: |d| match d {
        0 => "vence hoy".into(),
        1 => "falta 1 día".into(),
        d if d > 1 => format!("faltan {d} días"),
        -1 => "atrasado 1 día".into(),
        d => format!("atrasado {} días", -d),
    },
    no_deadline: "sin plazo definido (vea el acta de constitución)",
    conflicts: "Conflictos por revisar",
    folder: "Carpeta",
    operations: |n| {
        if n == 1 {
            "1 cambio en el diario".into()
        } else {
            format!("{n} cambios en el diario")
        }
    },

    charter_intro: "Los campos se guardan al salir de ellos. Los cambios de meta o alcance después de la aprobación pedirán una nueva aprobación (etapa L1).",
    f_title: "Título",
    f_process: "Proceso",
    f_area: "Área",
    f_problem: "Declaración del problema (qué, dónde, cuándo, cuánto; sin causa ni solución)",
    f_goal: "Meta",
    f_baseline: "Línea base",
    f_deadline: "Plazo",
    f_scope_in: "Dentro del alcance",
    f_scope_out: "Fuera del alcance",
    f_business_case: "Caso de negocio",
    date_hint: "DD/MM/AAAA",
    bad_date: "Fecha no válida: use DD/MM/AAAA.",

    new_action_hint: "nueva acción: qué hacer",
    add: "Añadir",
    board_tip: "Arrastre una tarjeta por el asa ≡ a otra columna, o use las flechas del panel de la tarjeta.",
    a_what: "Qué",
    a_why: "Por qué",
    a_where: "Dónde",
    a_when: "Cuándo (plazo)",
    a_who: "Quién",
    a_how: "Cómo",
    a_how_much: "Cuánto",
    a_column: "Columna",
    nobody: "(nadie)",
    delete: "Eliminar",
    close: "Cerrar",
    move_left: "⬅",
    move_right: "➡",
    move_up: "⬆",
    move_down: "⬇",
    untitled: "(sin título)",

    here_now: "Con el proyecto abierto:",
    is_on: |name, screen| format!("{name} está en {screen}"),
    is_editing_this: |name| {
        format!("{name} está editando esto ahora. Coordine con la persona antes de cambiarlo.")
    },
    conflicts_banner: |n| {
        if n == 1 {
            "1 conflicto por revisar: dos personas cambiaron el mismo campo sin ver el cambio de la otra.".into()
        } else {
            format!(
                "{n} conflictos por revisar: dos personas cambiaron el mismo campo sin ver el cambio de la otra."
            )
        }
    },
    review: "Revisar",
    conflicts_intro: "Cuando dos personas cambian el mismo campo sin ver el cambio de la otra, vale el más reciente y el otro aparece aquí. Mantenga el actual o restaure el otro.",
    no_conflicts: "Ningún conflicto por revisar.",
    kept: "Vigente",
    replaced: "Reemplazado",
    keep_this: "Mantener el actual",
    restore_other: "Restaurar el otro",
    by_whom: |name| format!("por {name}"),
    pending_files: |n| {
        if n == 1 {
            "1 archivo aún llegando por la sincronización (o de una versión más nueva del programa)."
                .into()
        } else {
            format!(
                "{n} archivos aún llegando por la sincronización (o de una versión más nueva del programa)."
            )
        }
    },

    data_intro: "Los conjuntos de datos (CSV y hojas de cálculo, con origen Real, Gemelo o Simulado) llegan en la etapa L3. Aquí ya puede llevar el proyecto entero en un solo archivo.",
    export_lssz: "Exportar .lssz",
    export_dest_hint: "archivo de destino (.lssz)",
    exported: |p| format!("Proyecto exportado a {p}"),
    imported: |p| format!("Proyecto importado en {p}"),
    created: |p| format!("Proyecto creado en {p}"),
    error: |e| format!("Error: {e}"),

    soon_portfolio: "Banco de ideas, priorización (matriz impacto × esfuerzo, puntuación ponderada, alineación estratégica) y la carga de los belts, sumando los proyectos de esta carpeta.",
    soon_phases: "Entregables de cada fase, lista de verificación del tollgate (ítems manuales y verificados), decisiones de los aprobadores y congelación por hash.",
    soon_risks: "Registro de riesgos con la matriz probabilidad × impacto y las respuestas que se convierten en acciones.",
    soon_meetings: "Agenda, participantes, acta, decisiones y acciones creadas a partir del acta.",
    soon_schedule: "Gantt con línea base, ruta crítica, holguras y probabilidad de cumplir cada tollgate por PERT y Monte Carlo, en días hábiles.",
    soon_savings: "Ahorros hard y soft con los estados estimado, previsto, realizado y validado, y el seguimiento de 12 meses.",
    soon_data: "Importación de CSV y hojas de cálculo con detección de separador y coma decimal, conjuntos inmutables con hash y origen.",
};

/// Texts of the "Ferramentas" screen (etapa L2).
pub struct ToolTexts {
    pub intro: &'static str,
    pub new_deliverable: &'static str,
    pub no_deliverables: &'static str,
    pub no_phase: &'static str,
    pub pick_deliverable: &'static str,
    pub unknown_tool: &'static str,
    pub phase: &'static str,
    pub status: &'static str,
    pub bad_number: &'static str,
    pub up: &'static str,
    pub down: &'static str,
    pub remove: &'static str,
    pub no_rows: &'static str,
    pub new_row: &'static str,
    pub add_hint: &'static str,
    pub no_data: &'static str,

    // VOC and CTQ.
    pub voc_statements: &'static str,
    pub voc_statement: &'static str,
    pub voc_statement_hint: &'static str,
    pub source: &'static str,
    pub segment: &'static str,
    pub need: &'static str,
    pub need_hint: &'static str,
    pub ungrouped: &'static str,
    pub statements_n: fn(usize) -> String,
    pub ctq_tree: &'static str,
    pub kano_none: &'static str,
    pub driver: &'static str,
    pub driver_hint: &'static str,
    pub ctq: &'static str,
    pub ctq_hint: &'static str,
    pub definition: &'static str,
    pub unit: &'static str,
    pub lsl: &'static str,
    pub usl: &'static str,
    pub target: &'static str,
    pub orphans: fn(usize) -> String,
    pub tree_ok: &'static str,
    pub tree_issues: fn(usize) -> String,

    // SIPOC.
    pub sipoc_steps: fn(usize) -> String,
    pub sipoc_empty: &'static str,

    // Ishikawa.
    pub effect: &'static str,
    pub effect_missing: &'static str,
    pub cycle_broken: &'static str,
    pub evidence: &'static str,
    pub add_sub: &'static str,
    pub cause_hint: &'static str,
    pub new_category_hint: &'static str,

    // Five whys.
    pub problem: &'static str,
    pub from_cause: &'static str,
    pub no_cause: &'static str,
    pub why_n: fn(usize) -> String,
    pub root_cause: &'static str,
    pub next_why: &'static str,
    pub first_why: &'static str,
    pub roots: fn(&str) -> String,
    pub chain_hint: &'static str,
    pub chain_from_cause: &'static str,

    // C&E matrix.
    pub ce_outputs: &'static str,
    pub output: &'static str,
    pub output_hint: &'static str,
    pub weight: &'static str,
    pub ce_matrix: &'static str,
    pub input: &'static str,
    pub input_hint: &'static str,
    pub total: &'static str,
    pub rank: &'static str,
    pub column_sum: &'static str,
    pub ce_pareto: &'static str,

    // Pareto.
    pub pareto_counts: &'static str,
    pub pareto_table: &'static str,
    pub pareto_table_tip: &'static str,
    pub others_below: &'static str,
    pub by_cost: &'static str,
    pub category: &'static str,
    pub category_hint: &'static str,
    pub count: &'static str,
    pub unit_cost: &'static str,
    pub value: &'static str,
    pub percentage: &'static str,
    pub cumulative: &'static str,
    pub vital: &'static str,

    // Plans.
    pub c_role: &'static str,
    pub c_name: &'static str,
    pub c_data_type: &'static str,
    pub c_stratify: &'static str,
    pub c_sample: &'static str,
    pub c_frequency: &'static str,
    pub c_subgroup: &'static str,
    pub c_who: &'static str,
    pub c_where: &'static str,
    pub c_form: &'static str,
    pub c_missing: &'static str,
    pub c_process: &'static str,
    pub c_characteristic: &'static str,
    pub c_char_kind: &'static str,
    pub c_spec: &'static str,
    pub c_method: &'static str,
    pub c_control: &'static str,
    pub c_owner: &'static str,
    pub c_reaction: &'static str,
    pub c_fmea_link: &'static str,
    pub critical: &'static str,
    pub no_gaps: &'static str,
    pub gaps: fn(usize) -> String,
    pub add_control_line: &'static str,
    pub c_signal: &'static str,
    pub c_immediate: &'static str,
    pub c_containment: &'static str,
    pub c_call: &'static str,
    pub c_deadline: &'static str,
    pub c_card: &'static str,
    pub open_card: &'static str,
    pub make_card: &'static str,
    pub fire_signal: &'static str,
    pub card_made: &'static str,

    // FMEA.
    pub ap_table: &'static str,
    pub ap_builtin: &'static str,
    pub ap_company: fn(&str) -> String,
    pub ap_use_builtin: &'static str,
    pub ap_counts: fn(usize, usize, usize) -> String,
    pub ap_non_monotone: fn(usize) -> String,
    pub ap_csv_hint: &'static str,
    pub ap_name_hint: &'static str,
    pub ap_import: &'static str,
    pub ap_export: &'static str,
    pub ap_need_path: &'static str,
    pub ap_exported: fn(&str) -> String,
    pub ap_imported: &'static str,
    pub ap_rejected: &'static str,
    pub ap_tip: &'static str,
    pub scales: &'static str,
    pub scales_tip: &'static str,
    pub level: &'static str,
    pub scale_s: &'static str,
    pub scale_o: &'static str,
    pub scale_d: &'static str,
    pub critical_from: &'static str,
    pub critical_default: fn(u8) -> String,
    pub fmea_headers: &'static [&'static str],

    // A3.
    pub a3_titles: [&'static str; 7],
    pub a3_empty: &'static str,
    pub a3_note: &'static str,
    pub a3_pareto: &'static str,
    pub a3_process: &'static str,
    pub a3_verified: &'static str,
    pub a3_root: &'static str,
    pub a3_fmea_high: &'static str,
    pub a3_control: &'static str,
    pub a3_signals: fn(usize) -> String,
}

pub static PT_BR_TOOLS: ToolTexts = ToolTexts {
    intro: "Cada ferramenta é um entregável de uma fase do projeto. Crie, escolha na lista e edite: cada campo é gravado quando você sai dele e a equipe vê pela pasta compartilhada.",
    new_deliverable: "Novo entregável:",
    no_deliverables: "Nenhum entregável ainda. Escolha a ferramenta e clique em \"Criar\".",
    no_phase: "Sem fase",
    pick_deliverable: "Escolha um entregável acima.",
    unknown_tool: "Ferramenta de uma versão mais nova do programa: atualize pelo launcher.",
    phase: "Fase",
    status: "Estado",
    bad_number: "Número inválido: use, por exemplo, 12,5.",
    up: "Subir",
    down: "Descer",
    remove: "✖",
    no_rows: "Nenhuma linha ainda.",
    new_row: "Nova linha",
    add_hint: "novo item",
    no_data: "Sem dados para o gráfico.",

    voc_statements: "Voz do cliente",
    voc_statement: "O que o cliente disse",
    voc_statement_hint: "fala do cliente, nas palavras dele",
    source: "Fonte",
    segment: "Segmento",
    need: "Necessidade",
    need_hint: "nova necessidade (agrupa falas)",
    ungrouped: "(sem agrupar)",
    statements_n: |n| match n {
        0 => "nenhuma fala".into(),
        1 => "1 fala".into(),
        n => format!("{n} falas"),
    },
    ctq_tree: "Árvore CTQ: necessidade → direcionador → CTQ",
    kano_none: "(Kano)",
    driver: "Direcionador",
    driver_hint: "novo direcionador",
    ctq: "CTQ",
    ctq_hint: "novo CTQ (característica mensurável)",
    definition: "Definição operacional",
    unit: "Unidade",
    lsl: "LIE",
    usl: "LSE",
    target: "Meta",
    orphans: |n| format!("{n} itens sem pai na árvore (o pai foi excluído)."),
    tree_ok: "Árvore completa: cada CTQ com unidade e limite.",
    tree_issues: |n| {
        if n == 1 {
            "1 pendência na árvore.".into()
        } else {
            format!("{n} pendências na árvore.")
        }
    },

    sipoc_steps: |n| format!("O processo tem {n} etapas; o usual num SIPOC é de 5 a 7."),
    sipoc_empty: "Há colunas vazias.",

    effect: "Efeito (problema)",
    effect_missing: "(escreva o efeito)",
    cycle_broken: "Duas pessoas moveram causas ao mesmo tempo e uma ficou embaixo da outra; o diagrama mostra a primeira como causa principal.",
    evidence: "Evidência",
    add_sub: "+ subcausa",
    cause_hint: "nova causa",
    new_category_hint: "nova categoria (além dos 6M)",

    problem: "Problema",
    from_cause: "Causa do Ishikawa",
    no_cause: "(nenhuma)",
    why_n: |n| format!("Por quê? {n}"),
    root_cause: "causa raiz",
    next_why: "+ por quê",
    first_why: "+ por quê (novo ramo)",
    roots: |r| format!("Causas raiz candidatas: {r}"),
    chain_hint: "nova cadeia: o problema",
    chain_from_cause: "Nova cadeia a partir de uma causa do Ishikawa",

    ce_outputs: "Saídas (Y) com o peso para o cliente (1 a 10)",
    output: "Saída",
    output_hint: "nova saída (Y)",
    weight: "Peso",
    ce_matrix: "Entradas (X) × saídas: correlação 0, 1, 3 ou 9",
    input: "Entrada (X)",
    input_hint: "nova entrada (X)",
    total: "Total",
    rank: "Ordem",
    column_sum: "Soma das notas",
    ce_pareto: "Pareto dos totais",

    pareto_counts: "Contagens por categoria",
    pareto_table: "Tabela de ocorrências",
    pareto_table_tip: "Uma ocorrência por linha; para várias de uma vez, termine a linha com ;quantidade (por exemplo, \"Rebarba;12\").",
    others_below: "Juntar em \"Outros\" abaixo de",
    by_cost: "Ponderar pelo custo unitário",
    category: "Categoria",
    category_hint: "nova categoria",
    count: "Contagem",
    unit_cost: "Custo unitário",
    value: "Valor",
    percentage: "%",
    cumulative: "Acumulado",
    vital: "poucos vitais",

    c_role: "Y/X",
    c_name: "Medida",
    c_data_type: "Tipo de dado",
    c_stratify: "Estratificação",
    c_sample: "Amostra (n)",
    c_frequency: "Frequência",
    c_subgroup: "Subgrupo",
    c_who: "Quem coleta",
    c_where: "Onde",
    c_form: "Formulário",
    c_missing: "Falta",
    c_process: "Processo / etapa",
    c_characteristic: "Característica",
    c_char_kind: "Tipo",
    c_spec: "Especificação",
    c_method: "Método de medição",
    c_control: "Método de controle",
    c_owner: "Responsável",
    c_reaction: "Plano de reação",
    c_fmea_link: "Linha do FMEA",
    critical: "característica crítica",
    no_gaps: "Todas as características críticas do FMEA (S alto) têm linha no plano de controle.",
    gaps: |n| {
        if n == 1 {
            "1 característica crítica do FMEA (S alto) ainda sem linha no plano de controle.".into()
        } else {
            format!(
                "{n} características críticas do FMEA (S alto) ainda sem linha no plano de controle."
            )
        }
    },
    add_control_line: "Criar a linha",
    c_signal: "Sinal",
    c_immediate: "Ação imediata",
    c_containment: "Contenção",
    c_call: "Quem acionar",
    c_deadline: "Prazo",
    c_card: "Cartão",
    open_card: "Abrir cartão",
    make_card: "Criar cartão",
    fire_signal: "Sinal ocorreu: abrir cartão",
    card_made: "Cartão criado em \"A fazer\" no quadro de ações.",

    ap_table: "Tabela de ação prioritária (AP):",
    ap_builtin: "tabela própria do UTSim (regras no manual)",
    ap_company: |n| {
        if n.is_empty() {
            "tabela da empresa".into()
        } else {
            format!("tabela da empresa: {n}")
        }
    },
    ap_use_builtin: "Voltar à tabela do UTSim",
    ap_counts: |h, m, l| format!("{h} combinações altas, {m} médias, {l} baixas"),
    ap_non_monotone: |n| {
        format!("Atenção: em {n} combinações a prioridade cai quando S, O ou D sobe.")
    },
    ap_csv_hint: "arquivo CSV com S,O,D,AP (as 1000 combinações)",
    ap_name_hint: "nome da tabela",
    ap_import: "Importar tabela da empresa",
    ap_export: "Salvar a tabela atual em CSV",
    ap_need_path: "Escreva o caminho do arquivo CSV.",
    ap_exported: |p| format!("Tabela salva em {p}"),
    ap_imported: "Tabela da empresa importada: vale para todo o projeto.",
    ap_rejected: "Tabela não importada: veja os problemas abaixo.",
    ap_tip: "Importe só uma tabela que a empresa tem direito de usar. Ela vale para todos os FMEAs do projeto e para toda a equipe.",
    scales: "Escalas da empresa (S, O, D de 1 a 10)",
    scales_tip: "Descreva o que cada nota significa na sua empresa; a descrição fica no projeto para toda a equipe.",
    level: "Nota",
    scale_s: "Severidade (S)",
    scale_o: "Ocorrência (O)",
    scale_d: "Detecção (D)",
    critical_from: "Característica crítica a partir de S =",
    critical_default: |s| format!("(padrão {s})"),
    fmea_headers: &[
        "",
        "Etapa / função",
        "Modo de falha",
        "Efeito",
        "S",
        "Causa",
        "O",
        "Controle de prevenção",
        "Controle de detecção",
        "D",
        "RPN",
        "AP",
        "Ação recomendada",
        "Cartão",
        "Ação tomada",
        "S'",
        "O'",
        "D'",
        "RPN antes → depois",
        "AP depois",
        "",
    ],

    a3_titles: [
        "Contexto",
        "Condição atual",
        "Objetivo",
        "Análise das causas",
        "Contramedidas",
        "Plano de ação",
        "Acompanhamento",
    ],
    a3_empty: "—",
    a3_note: "Nota da equipe",
    a3_pareto: "Pareto",
    a3_process: "Processo",
    a3_verified: "Causa verificada",
    a3_root: "Causa raiz",
    a3_fmea_high: "FMEA, AP alta",
    a3_control: "Plano de controle",
    a3_signals: |n| match n {
        0 => "Nenhum sinal no plano de resposta.".into(),
        1 => "1 sinal no plano de resposta.".into(),
        n => format!("{n} sinais no plano de resposta."),
    },
};

pub static EN_TOOLS: ToolTexts = ToolTexts {
    intro: "Each tool is a deliverable of a project phase. Create one, pick it from the list and edit it: each field is saved when you leave it and the team sees it through the shared folder.",
    new_deliverable: "New deliverable:",
    no_deliverables: "No deliverables yet. Choose the tool and click \"Create\".",
    no_phase: "No phase",
    pick_deliverable: "Pick a deliverable above.",
    unknown_tool: "Tool from a newer version of the program: update through the launcher.",
    phase: "Phase",
    status: "Status",
    bad_number: "Invalid number: use, for example, 12.5.",
    up: "Up",
    down: "Down",
    remove: "✖",
    no_rows: "No rows yet.",
    new_row: "New row",
    add_hint: "new item",
    no_data: "No data for the chart.",

    voc_statements: "Voice of the customer",
    voc_statement: "What the customer said",
    voc_statement_hint: "customer statement, in their own words",
    source: "Source",
    segment: "Segment",
    need: "Need",
    need_hint: "new need (groups statements)",
    ungrouped: "(ungrouped)",
    statements_n: |n| match n {
        0 => "no statements".into(),
        1 => "1 statement".into(),
        n => format!("{n} statements"),
    },
    ctq_tree: "CTQ tree: need → driver → CTQ",
    kano_none: "(Kano)",
    driver: "Driver",
    driver_hint: "new driver",
    ctq: "CTQ",
    ctq_hint: "new CTQ (measurable characteristic)",
    definition: "Operational definition",
    unit: "Unit",
    lsl: "LSL",
    usl: "USL",
    target: "Target",
    orphans: |n| format!("{n} items without a parent in the tree (the parent was deleted)."),
    tree_ok: "Complete tree: every CTQ has a unit and a limit.",
    tree_issues: |n| {
        if n == 1 {
            "1 open issue in the tree.".into()
        } else {
            format!("{n} open issues in the tree.")
        }
    },

    sipoc_steps: |n| format!("The process has {n} steps; the usual in a SIPOC is 5 to 7."),
    sipoc_empty: "There are empty columns.",

    effect: "Effect (problem)",
    effect_missing: "(write the effect)",
    cycle_broken: "Two people moved causes at the same time and one ended up under the other; the diagram shows the first as the main cause.",
    evidence: "Evidence",
    add_sub: "+ sub-cause",
    cause_hint: "new cause",
    new_category_hint: "new category (besides the 6Ms)",

    problem: "Problem",
    from_cause: "Ishikawa cause",
    no_cause: "(none)",
    why_n: |n| format!("Why? {n}"),
    root_cause: "root cause",
    next_why: "+ why",
    first_why: "+ why (new branch)",
    roots: |r| format!("Candidate root causes: {r}"),
    chain_hint: "new chain: the problem",
    chain_from_cause: "New chain from an Ishikawa cause",

    ce_outputs: "Outputs (Y) with the weight for the customer (1 to 10)",
    output: "Output",
    output_hint: "new output (Y)",
    weight: "Weight",
    ce_matrix: "Inputs (X) × outputs: correlation 0, 1, 3 or 9",
    input: "Input (X)",
    input_hint: "new input (X)",
    total: "Total",
    rank: "Rank",
    column_sum: "Sum of scores",
    ce_pareto: "Pareto of the totals",

    pareto_counts: "Counts by category",
    pareto_table: "Occurrence table",
    pareto_table_tip: "One occurrence per line; for several at once, end the line with ;quantity (for example, \"Burr;12\").",
    others_below: "Group into \"Others\" below",
    by_cost: "Weight by unit cost",
    category: "Category",
    category_hint: "new category",
    count: "Count",
    unit_cost: "Unit cost",
    value: "Value",
    percentage: "%",
    cumulative: "Cumulative",
    vital: "vital few",

    c_role: "Y/X",
    c_name: "Measure",
    c_data_type: "Data type",
    c_stratify: "Stratification",
    c_sample: "Sample (n)",
    c_frequency: "Frequency",
    c_subgroup: "Subgroup",
    c_who: "Who collects",
    c_where: "Where",
    c_form: "Form",
    c_missing: "Missing",
    c_process: "Process / step",
    c_characteristic: "Characteristic",
    c_char_kind: "Type",
    c_spec: "Specification",
    c_method: "Measurement method",
    c_control: "Control method",
    c_owner: "Owner",
    c_reaction: "Reaction plan",
    c_fmea_link: "FMEA line",
    critical: "critical characteristic",
    no_gaps: "Every critical characteristic of the FMEA (high S) has a line in the control plan.",
    gaps: |n| {
        if n == 1 {
            "1 critical characteristic of the FMEA (high S) still without a line in the control plan.".into()
        } else {
            format!(
                "{n} critical characteristics of the FMEA (high S) still without a line in the control plan."
            )
        }
    },
    add_control_line: "Create the line",
    c_signal: "Signal",
    c_immediate: "Immediate action",
    c_containment: "Containment",
    c_call: "Who to call",
    c_deadline: "Deadline",
    c_card: "Card",
    open_card: "Open card",
    make_card: "Create card",
    fire_signal: "Signal occurred: open card",
    card_made: "Card created in \"To do\" on the action board.",

    ap_table: "Action priority (AP) table:",
    ap_builtin: "UTSim's own table (rules in the manual)",
    ap_company: |n| {
        if n.is_empty() {
            "company table".into()
        } else {
            format!("company table: {n}")
        }
    },
    ap_use_builtin: "Back to the UTSim table",
    ap_counts: |h, m, l| format!("{h} high combinations, {m} medium, {l} low"),
    ap_non_monotone: |n| {
        format!("Warning: in {n} combinations the priority drops when S, O or D goes up.")
    },
    ap_csv_hint: "CSV file with S,O,D,AP (the 1000 combinations)",
    ap_name_hint: "table name",
    ap_import: "Import company table",
    ap_export: "Save the current table as CSV",
    ap_need_path: "Write the path of the CSV file.",
    ap_exported: |p| format!("Table saved to {p}"),
    ap_imported: "Company table imported: it applies to the whole project.",
    ap_rejected: "Table not imported: see the problems below.",
    ap_tip: "Import only a table that the company has the right to use. It applies to all FMEAs in the project and to the whole team.",
    scales: "Company scales (S, O, D from 1 to 10)",
    scales_tip: "Describe what each score means in your company; the description stays in the project for the whole team.",
    level: "Score",
    scale_s: "Severity (S)",
    scale_o: "Occurrence (O)",
    scale_d: "Detection (D)",
    critical_from: "Critical characteristic from S =",
    critical_default: |s| format!("(default {s})"),
    fmea_headers: &[
        "",
        "Step / function",
        "Failure mode",
        "Effect",
        "S",
        "Cause",
        "O",
        "Prevention control",
        "Detection control",
        "D",
        "RPN",
        "AP",
        "Recommended action",
        "Card",
        "Action taken",
        "S'",
        "O'",
        "D'",
        "RPN before → after",
        "AP after",
        "",
    ],

    a3_titles: [
        "Background",
        "Current condition",
        "Goal",
        "Root cause analysis",
        "Countermeasures",
        "Action plan",
        "Follow-up",
    ],
    a3_empty: "—",
    a3_note: "Team note",
    a3_pareto: "Pareto",
    a3_process: "Process",
    a3_verified: "Verified cause",
    a3_root: "Root cause",
    a3_fmea_high: "FMEA, high AP",
    a3_control: "Control plan",
    a3_signals: |n| match n {
        0 => "No signal in the response plan.".into(),
        1 => "1 signal in the response plan.".into(),
        n => format!("{n} signals in the response plan."),
    },
};

pub static ES_TOOLS: ToolTexts = ToolTexts {
    intro: "Cada herramienta es un entregable de una fase del proyecto. Cree uno, elíjalo en la lista y edítelo: cada campo se guarda al salir de él y el equipo lo ve por la carpeta compartida.",
    new_deliverable: "Nuevo entregable:",
    no_deliverables: "Ningún entregable todavía. Elija la herramienta y haga clic en \"Crear\".",
    no_phase: "Sin fase",
    pick_deliverable: "Elija un entregable arriba.",
    unknown_tool: "Herramienta de una versión más nueva del programa: actualiza con el launcher.",
    phase: "Fase",
    status: "Estado",
    bad_number: "Número no válido: use, por ejemplo, 12,5.",
    up: "Subir",
    down: "Bajar",
    remove: "✖",
    no_rows: "Ninguna fila todavía.",
    new_row: "Nueva fila",
    add_hint: "nuevo ítem",
    no_data: "Sin datos para el gráfico.",

    voc_statements: "Voz del cliente",
    voc_statement: "Lo que dijo el cliente",
    voc_statement_hint: "frase del cliente, con sus palabras",
    source: "Fuente",
    segment: "Segmento",
    need: "Necesidad",
    need_hint: "nueva necesidad (agrupa frases)",
    ungrouped: "(sin agrupar)",
    statements_n: |n| match n {
        0 => "ninguna frase".into(),
        1 => "1 frase".into(),
        n => format!("{n} frases"),
    },
    ctq_tree: "Árbol CTQ: necesidad → impulsor → CTQ",
    kano_none: "(Kano)",
    driver: "Impulsor",
    driver_hint: "nuevo impulsor",
    ctq: "CTQ",
    ctq_hint: "nuevo CTQ (característica medible)",
    definition: "Definición operacional",
    unit: "Unidad",
    lsl: "LIE",
    usl: "LSE",
    target: "Meta",
    orphans: |n| format!("{n} ítems sin padre en el árbol (el padre fue eliminado)."),
    tree_ok: "Árbol completo: cada CTQ con unidad y límite.",
    tree_issues: |n| {
        if n == 1 {
            "1 pendiente en el árbol.".into()
        } else {
            format!("{n} pendientes en el árbol.")
        }
    },

    sipoc_steps: |n| format!("El proceso tiene {n} etapas; lo habitual en un SIPOC es de 5 a 7."),
    sipoc_empty: "Hay columnas vacías.",

    effect: "Efecto (problema)",
    effect_missing: "(escriba el efecto)",
    cycle_broken: "Dos personas movieron causas al mismo tiempo y una quedó debajo de la otra; el diagrama muestra la primera como causa principal.",
    evidence: "Evidencia",
    add_sub: "+ subcausa",
    cause_hint: "nueva causa",
    new_category_hint: "nueva categoría (además de las 6M)",

    problem: "Problema",
    from_cause: "Causa del Ishikawa",
    no_cause: "(ninguna)",
    why_n: |n| format!("¿Por qué? {n}"),
    root_cause: "causa raíz",
    next_why: "+ por qué",
    first_why: "+ por qué (nueva rama)",
    roots: |r| format!("Causas raíz candidatas: {r}"),
    chain_hint: "nueva cadena: el problema",
    chain_from_cause: "Nueva cadena a partir de una causa del Ishikawa",

    ce_outputs: "Salidas (Y) con el peso para el cliente (1 a 10)",
    output: "Salida",
    output_hint: "nueva salida (Y)",
    weight: "Peso",
    ce_matrix: "Entradas (X) × salidas: correlación 0, 1, 3 o 9",
    input: "Entrada (X)",
    input_hint: "nueva entrada (X)",
    total: "Total",
    rank: "Orden",
    column_sum: "Suma de las notas",
    ce_pareto: "Pareto de los totales",

    pareto_counts: "Conteos por categoría",
    pareto_table: "Tabla de ocurrencias",
    pareto_table_tip: "Una ocurrencia por línea; para varias a la vez, termina la línea con ;cantidad (por ejemplo, \"Rebaba;12\").",
    others_below: "Agrupar en \"Otros\" por debajo de",
    by_cost: "Ponderar por el costo unitario",
    category: "Categoría",
    category_hint: "nueva categoría",
    count: "Conteo",
    unit_cost: "Costo unitario",
    value: "Valor",
    percentage: "%",
    cumulative: "Acumulado",
    vital: "pocos vitales",

    c_role: "Y/X",
    c_name: "Medida",
    c_data_type: "Tipo de dato",
    c_stratify: "Estratificación",
    c_sample: "Muestra (n)",
    c_frequency: "Frecuencia",
    c_subgroup: "Subgrupo",
    c_who: "Quién recoge",
    c_where: "Dónde",
    c_form: "Formulario",
    c_missing: "Falta",
    c_process: "Proceso / etapa",
    c_characteristic: "Característica",
    c_char_kind: "Tipo",
    c_spec: "Especificación",
    c_method: "Método de medición",
    c_control: "Método de control",
    c_owner: "Responsable",
    c_reaction: "Plan de reacción",
    c_fmea_link: "Línea del AMEF",
    critical: "característica crítica",
    no_gaps: "Todas las características críticas del AMEF (S alto) tienen línea en el plan de control.",
    gaps: |n| {
        if n == 1 {
            "1 característica crítica del AMEF (S alto) aún sin línea en el plan de control.".into()
        } else {
            format!(
                "{n} características críticas del AMEF (S alto) aún sin línea en el plan de control."
            )
        }
    },
    add_control_line: "Crear la línea",
    c_signal: "Señal",
    c_immediate: "Acción inmediata",
    c_containment: "Contención",
    c_call: "A quién avisar",
    c_deadline: "Plazo",
    c_card: "Tarjeta",
    open_card: "Abrir tarjeta",
    make_card: "Crear tarjeta",
    fire_signal: "La señal ocurrió: abrir tarjeta",
    card_made: "Tarjeta creada en \"Por hacer\" en el tablero de acciones.",

    ap_table: "Tabla de acción prioritaria (AP):",
    ap_builtin: "tabla propia de UTSim (reglas en el manual)",
    ap_company: |n| {
        if n.is_empty() {
            "tabla de la empresa".into()
        } else {
            format!("tabla de la empresa: {n}")
        }
    },
    ap_use_builtin: "Volver a la tabla de UTSim",
    ap_counts: |h, m, l| format!("{h} combinaciones altas, {m} medias, {l} bajas"),
    ap_non_monotone: |n| {
        format!("Atención: en {n} combinaciones la prioridad baja cuando S, O o D sube.")
    },
    ap_csv_hint: "archivo CSV con S,O,D,AP (las 1000 combinaciones)",
    ap_name_hint: "nombre de la tabla",
    ap_import: "Importar tabla de la empresa",
    ap_export: "Guardar la tabla actual en CSV",
    ap_need_path: "Escriba la ruta del archivo CSV.",
    ap_exported: |p| format!("Tabla guardada en {p}"),
    ap_imported: "Tabla de la empresa importada: vale para todo el proyecto.",
    ap_rejected: "Tabla no importada: vea los problemas abajo.",
    ap_tip: "Importe solo una tabla que la empresa tenga derecho a usar. Vale para todos los AMEF del proyecto y para todo el equipo.",
    scales: "Escalas de la empresa (S, O, D de 1 a 10)",
    scales_tip: "Describa qué significa cada nota en su empresa; la descripción queda en el proyecto para todo el equipo.",
    level: "Nota",
    scale_s: "Severidad (S)",
    scale_o: "Ocurrencia (O)",
    scale_d: "Detección (D)",
    critical_from: "Característica crítica a partir de S =",
    critical_default: |s| format!("(por defecto {s})"),
    fmea_headers: &[
        "",
        "Etapa / función",
        "Modo de falla",
        "Efecto",
        "S",
        "Causa",
        "O",
        "Control de prevención",
        "Control de detección",
        "D",
        "RPN",
        "AP",
        "Acción recomendada",
        "Tarjeta",
        "Acción tomada",
        "S'",
        "O'",
        "D'",
        "RPN antes → después",
        "AP después",
        "",
    ],

    a3_titles: [
        "Contexto",
        "Condición actual",
        "Objetivo",
        "Análisis de las causas",
        "Contramedidas",
        "Plan de acción",
        "Seguimiento",
    ],
    a3_empty: "—",
    a3_note: "Nota del equipo",
    a3_pareto: "Pareto",
    a3_process: "Proceso",
    a3_verified: "Causa verificada",
    a3_root: "Causa raíz",
    a3_fmea_high: "AMEF, AP alta",
    a3_control: "Plan de control",
    a3_signals: |n| match n {
        0 => "Ninguna señal en el plan de respuesta.".into(),
        1 => "1 señal en el plan de respuesta.".into(),
        n => format!("{n} señales en el plan de respuesta."),
    },
};

/// Data types of the data collection plan: key and name (Portuguese; see `data_types`).
pub const DATA_TYPES: [(&str, &str); 3] = [
    ("continuo", "Contínuo"),
    ("atributo", "Atributo"),
    ("contagem", "Contagem"),
];

const DATA_TYPES_EN: [(&str, &str); 3] = [
    ("continuo", "Continuous"),
    ("atributo", "Attribute"),
    ("contagem", "Count"),
];

const DATA_TYPES_ES: [(&str, &str); 3] = [
    ("continuo", "Continuo"),
    ("atributo", "Atributo"),
    ("contagem", "Conteo"),
];

/// Data types in the active language.
pub fn data_types() -> &'static [(&'static str, &'static str); 3] {
    match language() {
        Language::PtBr => &DATA_TYPES,
        Language::En => &DATA_TYPES_EN,
        Language::Es => &DATA_TYPES_ES,
    }
}

/// Picks the text of the active language: `[Portuguese, English, Spanish]`.
fn pick(t: [&'static str; 3]) -> &'static str {
    match language() {
        Language::PtBr => t[0],
        Language::En => t[1],
        Language::Es => t[2],
    }
}

pub fn tool_texts() -> &'static ToolTexts {
    match language() {
        Language::PtBr => &PT_BR_TOOLS,
        Language::En => &EN_TOOLS,
        Language::Es => &ES_TOOLS,
    }
}

pub fn tool_name(t: Tool) -> &'static str {
    match t {
        Tool::Voc => pick(["VOC e árvore CTQ", "VOC and CTQ tree", "VOC y árbol CTQ"]),
        Tool::Sipoc => "SIPOC",
        Tool::Pareto => "Pareto",
        Tool::DataCollection => pick([
            "Plano de coleta de dados",
            "Data collection plan",
            "Plan de recolección de datos",
        ]),
        Tool::Ishikawa => "Ishikawa",
        Tool::FiveWhys => pick(["5 porquês", "5 whys", "5 porqués"]),
        Tool::CauseEffect => pick([
            "Matriz causa-efeito",
            "Cause-and-effect matrix",
            "Matriz causa-efecto",
        ]),
        Tool::Fmea => pick(["FMEA de processo", "Process FMEA", "AMEF de proceso"]),
        Tool::ControlPlan => pick(["Plano de controle", "Control plan", "Plan de control"]),
        Tool::ResponsePlan => pick(["Plano de resposta", "Response plan", "Plan de respuesta"]),
        Tool::A3 => "A3",
        Tool::Vsm => pick([
            "Mapa do fluxo de valor",
            "Value stream map",
            "Mapa del flujo de valor",
        ]),
        Tool::Selection => pick([
            "Seleção de soluções",
            "Solution selection",
            "Selección de soluciones",
        ]),
        Tool::FiveS => pick(["Auditoria 5S", "5S audit", "Auditoría 5S"]),
        Tool::Smed => pick([
            "SMED (troca rápida)",
            "SMED (quick changeover)",
            "SMED (cambio rápido)",
        ]),
        Tool::StandardWork => pick([
            "Trabalho padronizado",
            "Standard work",
            "Trabajo estandarizado",
        ]),
        Tool::KaizenEvent => pick(["Evento kaizen", "Kaizen event", "Evento kaizen"]),
        Tool::Pdca => pick(["Ciclos PDCA", "PDCA cycles", "Ciclos PDCA"]),
    }
}

/// One line on how to use each tool.
pub fn tool_help(t: Tool) -> &'static str {
    match t {
        Tool::Voc => pick([
            "Registre as falas do cliente, agrupe em necessidades e desdobre em direcionadores e CTQs mensuráveis, com unidade e limites.",
            "Record the customer's statements, group them into needs and break them down into drivers and measurable CTQs, with unit and limits.",
            "Registre las frases del cliente, agrúpelas en necesidades y desglóselas en impulsores y CTQs medibles, con unidad y límites.",
        ]),
        Tool::Sipoc => pick([
            "Fornecedores, entradas, o processo em 5 a 7 etapas, saídas e clientes, da esquerda para a direita.",
            "Suppliers, inputs, the process in 5 to 7 steps, outputs and customers, from left to right.",
            "Proveedores, entradas, el proceso en 5 a 7 etapas, salidas y clientes, de izquierda a derecha.",
        ]),
        Tool::Pareto => pick([
            "Ordena as categorias, mostra o percentual acumulado e destaca as poucas vitais, que somam até 80 %.",
            "Sorts the categories, shows the cumulative percentage and highlights the vital few, which add up to 80 %.",
            "Ordena las categorías, muestra el porcentaje acumulado y destaca las pocas vitales, que suman hasta el 80 %.",
        ]),
        Tool::DataCollection => pick([
            "Para cada medida: o que é, como medir (definição operacional), a amostragem e quem coleta.",
            "For each measure: what it is, how to measure it (operational definition), the sampling and who collects.",
            "Para cada medida: qué es, cómo medirla (definición operacional), el muestreo y quién recoge los datos.",
        ]),
        Tool::Ishikawa => pick([
            "O efeito na cabeça e as causas por categoria (6M). Marque cada causa como provável, verificada ou descartada, com a evidência.",
            "The effect at the head and the causes by category (6Ms). Mark each cause as probable, verified or ruled out, with the evidence.",
            "El efecto en la cabeza y las causas por categoría (6M). Marque cada causa como probable, verificada o descartada, con la evidencia.",
        ]),
        Tool::FiveWhys => pick([
            "Pergunte \"por quê?\" a partir do problema, com evidência em cada nível; um porquê pode ter mais de um ramo.",
            "Ask \"why?\" starting from the problem, with evidence at each level; a why can have more than one branch.",
            "Pregunte \"¿por qué?\" a partir del problema, con evidencia en cada nivel; un porqué puede tener más de una rama.",
        ]),
        Tool::CauseEffect => pick([
            "Pese as saídas pelo cliente e dê a cada entrada 0, 1, 3 ou 9 por saída; o total ordena as entradas para a análise.",
            "Weigh the outputs by the customer and give each input 0, 1, 3 or 9 per output; the total ranks the inputs for the analysis.",
            "Pondere las salidas según el cliente y dé a cada entrada 0, 1, 3 o 9 por salida; el total ordena las entradas para el análisis.",
        ]),
        Tool::Fmea => pick([
            "Para cada modo de falha: S, O e D de 1 a 10, o RPN e a ação prioritária. A ação recomendada vira cartão no quadro.",
            "For each failure mode: S, O and D from 1 to 10, the RPN and the action priority. The recommended action becomes a card on the board.",
            "Para cada modo de falla: S, O y D de 1 a 10, el RPN y la acción prioritaria. La acción recomendada se convierte en tarjeta en el tablero.",
        ]),
        Tool::ControlPlan => pick([
            "Como cada característica fica sob controle depois do projeto. Toda característica crítica do FMEA precisa de uma linha.",
            "How each characteristic stays under control after the project. Every critical characteristic of the FMEA needs a line.",
            "Cómo cada característica queda bajo control después del proyecto. Toda característica crítica del AMEF necesita una línea.",
        ]),
        Tool::ResponsePlan => pick([
            "Para cada sinal (regra de carta, limite, alarme): a ação imediata, a contenção e quem acionar. O sinal abre um cartão.",
            "For each signal (chart rule, limit, alarm): the immediate action, the containment and who to call. The signal opens a card.",
            "Para cada señal (regla de carta, límite, alarma): la acción inmediata, la contención y a quién avisar. La señal abre una tarjeta.",
        ]),
        Tool::A3 => pick([
            "Uma página com o projeto inteiro, montada a partir dos outros entregáveis; acrescente as notas da equipe.",
            "One page with the whole project, built from the other deliverables; add the team's notes.",
            "Una página con el proyecto entero, armada a partir de los otros entregables; añada las notas del equipo.",
        ]),
        Tool::Vsm => pick([
            "Cada processo com o tempo de ciclo, a disponibilidade e o estoque antes dele; com a demanda, o app calcula o takt, a escada do lead time e a eficiência do ciclo. Do estado atual sai o futuro.",
            "Each process with its cycle time, availability and the inventory before it; with the demand, the app calculates the takt, the lead time ladder and the cycle efficiency. The future state comes out of the current state.",
            "Cada proceso con su tiempo de ciclo, disponibilidad y el inventario antes de él; con la demanda, la app calcula el takt, la escalera del lead time y la eficiencia del ciclo. Del estado actual sale el futuro.",
        ]),
        Tool::Selection => pick([
            "Compare soluções ou conceitos por critérios com peso: na matriz ponderada cada opção recebe de 1 a 5; na de Pugh, melhor (+), igual (S) ou pior (−) que a referência. Impacto e esforço montam a grade de prioridade.",
            "Compare solutions or concepts by weighted criteria: in the weighted matrix each option gets 1 to 5; in the Pugh matrix, better (+), same (S) or worse (−) than the reference. Impact and effort build the priority grid.",
            "Compara soluciones o conceptos por criterios con peso: en la matriz ponderada cada opción recibe de 1 a 5; en la de Pugh, mejor (+), igual (S) o peor (−) que la referencia. Impacto y esfuerzo arman la cuadrícula de prioridad.",
        ]),
        Tool::FiveS => pick([
            "A lista de verificação por senso e as auditorias da área, cada pergunta com nota de 0 a 4. O app soma por senso, mostra o radar da última auditoria e a evolução.",
            "The checklist by sense and the area audits, each question scored from 0 to 4. The app adds up by sense, shows the radar of the last audit and the trend.",
            "La lista de verificación por S y las auditorías del área, cada pregunta con nota de 0 a 4. La app suma por S, muestra el radar de la última auditoría y la evolución.",
        ]),
        Tool::Smed => pick([
            "Cada atividade da troca com o tempo e a classe: interna (máquina parada) ou externa (máquina rodando). Diga o que cada uma vira depois do estudo; o tempo de setup é a soma das internas.",
            "Each changeover activity with its time and class: internal (machine stopped) or external (machine running). Say what each one becomes after the study; the setup time is the sum of the internal ones.",
            "Cada actividad del cambio con su tiempo y clase: interna (máquina parada) o externa (máquina en marcha). Indique en qué se convierte cada una después del estudio; el tiempo de setup es la suma de las internas.",
        ]),
        Tool::StandardWork => pick([
            "Os elementos de cada operador com os tempos manual, automático e de deslocamento. Com o takt, o app monta a tabela de combinação, o balanceamento e o número mínimo de operadores.",
            "Each operator's elements with manual, automatic and walking times. With the takt, the app builds the combination table, the balancing and the minimum number of operators.",
            "Los elementos de cada operador con los tiempos manual, automático y de desplazamiento. Con el takt, la app arma la tabla de combinación, el balanceo y el número mínimo de operadores.",
        ]),
        Tool::KaizenEvent => pick([
            "O preparo, a pauta dos 3 a 5 dias, as pendências (que vão ao quadro de ações) e o acompanhamento em 30, 60 e 90 dias.",
            "The preparation, the agenda for the 3 to 5 days, the open items (which go to the action board) and the follow-up at 30, 60 and 90 days.",
            "La preparación, la agenda de los 3 a 5 días, los pendientes (que van al tablero de acciones) y el seguimiento a 30, 60 y 90 días.",
        ]),
        Tool::Pdca => pick([
            "Cada ciclo registra a hipótese, o que foi feito, o que se verificou e o que se padronizou ou ajustou. O ciclo seguinte parte do que o anterior aprendeu.",
            "Each cycle records the hypothesis, what was done, what was checked and what was standardised or adjusted. The next cycle starts from what the previous one learned.",
            "Cada ciclo registra la hipótesis, lo que se hizo, lo que se verificó y lo que se estandarizó o ajustó. El ciclo siguiente parte de lo que aprendió el anterior.",
        ]),
    }
}

pub fn deliverable_status_name(s: DeliverableStatus) -> &'static str {
    match s {
        DeliverableStatus::Draft => pick(["em elaboração", "in progress", "en elaboración"]),
        DeliverableStatus::Done => pick(["concluído", "done", "concluido"]),
    }
}

pub fn kano_name(k: Kano) -> &'static str {
    match k {
        Kano::MustBe => pick(["Obrigatório", "Must-be", "Obligatorio"]),
        Kano::Performance => pick(["Desempenho", "Performance", "Desempeño"]),
        Kano::Attractive => pick(["Atrativo", "Attractive", "Atractivo"]),
    }
}

pub fn cause_status_name(s: CauseStatus) -> &'static str {
    match s {
        CauseStatus::Open => pick(["aberta", "open", "abierta"]),
        CauseStatus::Probable => pick(["provável", "probable", "probable"]),
        CauseStatus::Verified => pick(["verificada", "verified", "verificada"]),
        CauseStatus::RuledOut => pick(["descartada", "ruled out", "descartada"]),
    }
}

pub fn ap_name(a: Ap) -> &'static str {
    match a {
        Ap::High => pick(["Alta", "High", "Alta"]),
        Ap::Medium => pick(["Média", "Medium", "Media"]),
        Ap::Low => pick(["Baixa", "Low", "Baja"]),
    }
}

pub fn origin_name(o: Origin) -> &'static str {
    match o {
        Origin::Manual => pick([
            "Criado no quadro",
            "Created on the board",
            "Creado en el tablero",
        ]),
        Origin::Fmea => pick([
            "Origem: ação recomendada do FMEA",
            "Origin: FMEA recommended action",
            "Origen: acción recomendada del AMEF",
        ]),
        Origin::Minutes => pick([
            "Origem: ata de reunião",
            "Origin: meeting minutes",
            "Origen: acta de reunión",
        ]),
        Origin::Tollgate => pick(["Origem: tollgate", "Origin: tollgate", "Origen: tollgate"]),
        Origin::Risk => pick([
            "Origem: registro de riscos",
            "Origin: risk register",
            "Origen: registro de riesgos",
        ]),
        Origin::ResponsePlan => pick([
            "Origem: sinal do plano de resposta",
            "Origin: response plan signal",
            "Origen: señal del plan de respuesta",
        ]),
        Origin::Kaizen => pick(["Origem: kaizen", "Origin: kaizen", "Origen: kaizen"]),
    }
}

pub fn voc_issue(i: Issue) -> &'static str {
    match i {
        Issue::NoUnit(_) => pick(["sem unidade", "no unit", "sin unidad"]),
        Issue::NoLimit(_) => pick(["sem LIE nem LSE", "no LSL or USL", "sin LIE ni LSE"]),
        Issue::LimitsReversed(_) => pick([
            "LIE maior ou igual ao LSE",
            "LSL greater than or equal to USL",
            "LIE mayor o igual al LSE",
        ]),
        Issue::TargetOutside(_) => pick([
            "meta fora dos limites",
            "target outside the limits",
            "meta fuera de los límites",
        ]),
        Issue::NeedWithoutDriver(_) => pick([
            "necessidade sem direcionador",
            "need without a driver",
            "necesidad sin impulsor",
        ]),
        Issue::DriverWithoutCtq(_) => pick([
            "direcionador sem CTQ",
            "driver without a CTQ",
            "impulsor sin CTQ",
        ]),
        Issue::Ungrouped(_) => pick([
            "fala sem necessidade",
            "statement without a need",
            "frase sin necesidad",
        ]),
    }
}

pub fn collection_issue(i: CollectionIssue) -> &'static str {
    match i {
        CollectionIssue::NoName => pick(["nome", "name", "nombre"]),
        CollectionIssue::NoDefinition => pick([
            "definição operacional",
            "operational definition",
            "definición operacional",
        ]),
        CollectionIssue::NoUnit => pick(["unidade", "unit", "unidad"]),
        CollectionIssue::NoSampling => pick([
            "amostra e frequência",
            "sample and frequency",
            "muestra y frecuencia",
        ]),
    }
}

pub use utsim_lss_store::user::Language;

thread_local! {
    static LANGUAGE: std::cell::Cell<Language> = const { std::cell::Cell::new(Language::PtBr) };
}

/// Sets the language of the interface (of this thread: the window's, or one test's).
pub fn set_language(l: Language) {
    LANGUAGE.with(|c| c.set(l));
}

/// The language of the interface.
pub fn language() -> Language {
    LANGUAGE.with(|c| c.get())
}

/// The active catalogue.
pub fn texts() -> &'static Texts {
    match language() {
        Language::PtBr => &PT_BR,
        Language::En => &EN,
        Language::Es => &ES,
    }
}

pub fn approach_name(a: Approach) -> &'static str {
    match a {
        Approach::DirectAction => pick(["Ação direta", "Direct action", "Acción directa"]),
        Approach::Kaizen => pick(["Kaizen rápido", "Quick kaizen", "Kaizen rápido"]),
        Approach::A3 => "A3",
        Approach::Pdca => "PDCA",
        Approach::Dmaic => "DMAIC",
        Approach::Dmadv => "DMADV (DFSS)",
    }
}

pub fn column_name(c: Column) -> &'static str {
    match c {
        Column::Todo => pick(["A fazer", "To do", "Por hacer"]),
        Column::Doing => pick(["Fazendo", "Doing", "En curso"]),
        Column::Blocked => pick(["Bloqueado", "Blocked", "Bloqueado"]),
        Column::Verify => pick(["Verificar", "Verify", "Verificar"]),
        Column::Done => pick(["Feito", "Done", "Hecho"]),
    }
}

pub fn phase_status_name(s: PhaseStatus) -> &'static str {
    match s {
        PhaseStatus::NotStarted => pick(["não iniciada", "not started", "no iniciada"]),
        PhaseStatus::InProgress => pick(["em andamento", "in progress", "en curso"]),
        PhaseStatus::InTollgate => pick(["em tollgate", "in tollgate", "en tollgate"]),
        PhaseStatus::Approved => pick(["aprovada", "approved", "aprobada"]),
    }
}

pub fn saving_status_name(s: SavingStatus) -> &'static str {
    match s {
        SavingStatus::Estimated => pick(["estimado", "estimated", "estimado"]),
        SavingStatus::Forecast => pick(["previsto", "forecast", "previsto"]),
        SavingStatus::Realized => pick(["realizado", "realized", "realizado"]),
        SavingStatus::Validated => pick(["validado", "validated", "validado"]),
    }
}

/// Number in the Brazilian format: `1.234,56`.
pub fn money(v: f64) -> String {
    let neg = v < 0.0;
    let cents = (v.abs() * 100.0).round() as u64;
    let (int, frac) = (cents / 100, cents % 100);
    let digits = int.to_string();
    let mut grouped = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            grouped.push('.');
        }
        grouped.push(c);
    }
    format!("{}R$ {grouped},{frac:02}", if neg { "−" } else { "" })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatted_texts() {
        let t = texts();
        assert_eq!((t.days_left)(0), "vence hoje");
        assert_eq!((t.days_left)(5), "faltam 5 dias");
        assert_eq!((t.days_left)(-3), "atrasado 3 dias");
        assert_eq!((t.is_on)("Ana", "Ações"), "Ana está em Ações");
        assert!((t.conflicts_banner)(2).starts_with("2 conflitos"));
        assert_eq!(money(1234567.891), "R$ 1.234.567,89");
        assert_eq!(money(-12.5), "−R$ 12,50");
        assert_eq!(money(0.0), "R$ 0,00");
        for a in Approach::ALL {
            assert!(!approach_name(a).is_empty());
        }
        let tt = tool_texts();
        for tool in Tool::ALL {
            assert!(!tool_name(tool).is_empty() && !tool_help(tool).is_empty());
        }
        assert_eq!(tt.fmea_headers.len(), 21, "one per column of the FMEA grid");
        assert_eq!((tt.why_n)(3), "Por quê? 3");
        assert!((tt.gaps)(2).starts_with("2 características"));
    }
}

/// Texts of the stage L1 screens.
pub mod l1;
pub mod l10;
pub mod l11;
pub mod l12;
pub mod l13;
pub mod l14;
pub mod l3;
pub mod l4;
pub mod l5;
pub mod l6;
pub mod l7;
pub mod l9;
