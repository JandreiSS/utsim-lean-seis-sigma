//! Every text of the interface, in one catalogue. Portuguese (Brazil) first; another
//! language is another `Texts` value (`docs/plano_lean_seis_sigma.md`, 4.8, P8). Screens
//! never write literal text: they read `texts().x`.

use utsim_lss_core::domain::{Approach, Column, PhaseStatus, SavingStatus};

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
    pub soon_tools: &'static str,
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
    soon_tools: "VOC e árvore CTQ, SIPOC, Ishikawa, 5 porquês, matriz causa-efeito, FMEA com a tabela de ação prioritária, plano de coleta e de controle, Pareto e A3 (etapa L2); estatística (etapa L3).",
    soon_data: "Importação de CSV e planilhas com detecção de separador e vírgula decimal, conjuntos imutáveis com hash e origem.",
};

/// The active catalogue.
pub fn texts() -> &'static Texts {
    &PT_BR
}

pub fn approach_name(a: Approach) -> &'static str {
    match a {
        Approach::DirectAction => "Ação direta",
        Approach::Kaizen => "Kaizen rápido",
        Approach::A3 => "A3",
        Approach::Pdca => "PDCA",
        Approach::Dmaic => "DMAIC",
        Approach::Dmadv => "DMADV (DFSS)",
    }
}

pub fn column_name(c: Column) -> &'static str {
    match c {
        Column::Todo => "A fazer",
        Column::Doing => "Fazendo",
        Column::Blocked => "Bloqueado",
        Column::Verify => "Verificar",
        Column::Done => "Feito",
    }
}

pub fn phase_status_name(s: PhaseStatus) -> &'static str {
    match s {
        PhaseStatus::NotStarted => "não iniciada",
        PhaseStatus::InProgress => "em andamento",
        PhaseStatus::InTollgate => "em tollgate",
        PhaseStatus::Approved => "aprovada",
    }
}

pub fn saving_status_name(s: SavingStatus) -> &'static str {
    match s {
        SavingStatus::Estimated => "estimado",
        SavingStatus::Forecast => "previsto",
        SavingStatus::Realized => "realizado",
        SavingStatus::Validated => "validado",
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
    }
}
