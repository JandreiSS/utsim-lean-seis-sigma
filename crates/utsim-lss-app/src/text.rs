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
    soon_data: "Importação de CSV e planilhas com detecção de separador e vírgula decimal, conjuntos imutáveis com hash e origem.",
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

/// Data types of the data collection plan: key and name.
pub const DATA_TYPES: [(&str, &str); 3] = [
    ("continuo", "Contínuo"),
    ("atributo", "Atributo"),
    ("contagem", "Contagem"),
];

pub fn tool_texts() -> &'static ToolTexts {
    &PT_BR_TOOLS
}

pub fn tool_name(t: Tool) -> &'static str {
    match t {
        Tool::Voc => "VOC e árvore CTQ",
        Tool::Sipoc => "SIPOC",
        Tool::Pareto => "Pareto",
        Tool::DataCollection => "Plano de coleta de dados",
        Tool::Ishikawa => "Ishikawa",
        Tool::FiveWhys => "5 porquês",
        Tool::CauseEffect => "Matriz causa-efeito",
        Tool::Fmea => "FMEA de processo",
        Tool::ControlPlan => "Plano de controle",
        Tool::ResponsePlan => "Plano de resposta",
        Tool::A3 => "A3",
    }
}

/// One line on how to use each tool.
pub fn tool_help(t: Tool) -> &'static str {
    match t {
        Tool::Voc => {
            "Registre as falas do cliente, agrupe em necessidades e desdobre em direcionadores e CTQs mensuráveis, com unidade e limites."
        }
        Tool::Sipoc => {
            "Fornecedores, entradas, o processo em 5 a 7 etapas, saídas e clientes, da esquerda para a direita."
        }
        Tool::Pareto => {
            "Ordena as categorias, mostra o percentual acumulado e destaca as poucas vitais, que somam até 80 %."
        }
        Tool::DataCollection => {
            "Para cada medida: o que é, como medir (definição operacional), a amostragem e quem coleta."
        }
        Tool::Ishikawa => {
            "O efeito na cabeça e as causas por categoria (6M). Marque cada causa como provável, verificada ou descartada, com a evidência."
        }
        Tool::FiveWhys => {
            "Pergunte \"por quê?\" a partir do problema, com evidência em cada nível; um porquê pode ter mais de um ramo."
        }
        Tool::CauseEffect => {
            "Pese as saídas pelo cliente e dê a cada entrada 0, 1, 3 ou 9 por saída; o total ordena as entradas para a análise."
        }
        Tool::Fmea => {
            "Para cada modo de falha: S, O e D de 1 a 10, o RPN e a ação prioritária. A ação recomendada vira cartão no quadro."
        }
        Tool::ControlPlan => {
            "Como cada característica fica sob controle depois do projeto. Toda característica crítica do FMEA precisa de uma linha."
        }
        Tool::ResponsePlan => {
            "Para cada sinal (regra de carta, limite, alarme): a ação imediata, a contenção e quem acionar. O sinal abre um cartão."
        }
        Tool::A3 => {
            "Uma página com o projeto inteiro, montada a partir dos outros entregáveis; acrescente as notas da equipe."
        }
    }
}

pub fn deliverable_status_name(s: DeliverableStatus) -> &'static str {
    match s {
        DeliverableStatus::Draft => "em elaboração",
        DeliverableStatus::Done => "concluído",
    }
}

pub fn kano_name(k: Kano) -> &'static str {
    match k {
        Kano::MustBe => "Obrigatório",
        Kano::Performance => "Desempenho",
        Kano::Attractive => "Atrativo",
    }
}

pub fn cause_status_name(s: CauseStatus) -> &'static str {
    match s {
        CauseStatus::Open => "aberta",
        CauseStatus::Probable => "provável",
        CauseStatus::Verified => "verificada",
        CauseStatus::RuledOut => "descartada",
    }
}

pub fn ap_name(a: Ap) -> &'static str {
    match a {
        Ap::High => "Alta",
        Ap::Medium => "Média",
        Ap::Low => "Baixa",
    }
}

pub fn origin_name(o: Origin) -> &'static str {
    match o {
        Origin::Manual => "Criado no quadro",
        Origin::Fmea => "Origem: ação recomendada do FMEA",
        Origin::Minutes => "Origem: ata de reunião",
        Origin::Tollgate => "Origem: tollgate",
        Origin::Risk => "Origem: registro de riscos",
        Origin::ResponsePlan => "Origem: sinal do plano de resposta",
        Origin::Kaizen => "Origem: kaizen",
    }
}

pub fn voc_issue(i: Issue) -> &'static str {
    match i {
        Issue::NoUnit(_) => "sem unidade",
        Issue::NoLimit(_) => "sem LIE nem LSE",
        Issue::LimitsReversed(_) => "LIE maior ou igual ao LSE",
        Issue::TargetOutside(_) => "meta fora dos limites",
        Issue::NeedWithoutDriver(_) => "necessidade sem direcionador",
        Issue::DriverWithoutCtq(_) => "direcionador sem CTQ",
        Issue::Ungrouped(_) => "fala sem necessidade",
    }
}

pub fn collection_issue(i: CollectionIssue) -> &'static str {
    match i {
        CollectionIssue::NoName => "nome",
        CollectionIssue::NoDefinition => "definição operacional",
        CollectionIssue::NoUnit => "unidade",
        CollectionIssue::NoSampling => "amostra e frequência",
    }
}

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
pub mod l4;
pub mod l6;
pub mod l7;
