//! Texts of the stage L1 screens (portfolio, charter versions, team and RACI, tollgates,
//! risks, meetings, schedule, savings, closing and lessons, folder picker), in a catalogue of
//! their own next to the main one, so the stages grow without touching the same lines.

use crate::text::{Language, language};
use utsim_lss_core::closure::Requirement;
use utsim_lss_core::domain::{
    Origin, ProjectStatus, RiskResponse, RiskStatus, RoleKind, SavingCategory, fields,
};
use utsim_lss_core::meeting::{ItemKind, MeetingKind};
use utsim_lss_core::portfolio::{Quadrant, Question, Scale};
use utsim_lss_core::savings::Source;
use utsim_lss_core::tollgate::{ApprovalRule, Check, Decision, TollgateStatus};
use utsim_lss_core::{charter, closure, schedule};

type S = &'static str;

pub struct L1Texts {
    pub add: S,
    pub bad_number: S,
    pub browse: S,
    pub nav_team: S,
    pub nav_closure: S,

    // Folder picker.
    pub picker_title: S,
    pub picker_up: S,
    pub picker_empty: S,
    pub picker_choose: S,
    pub picker_cancel: S,
    pub picker_tip: S,
    pub picker_file_tip: S,

    // Charter.
    pub charter_intro: S,
    pub charter_no_version: S,
    pub charter_not_approved: S,
    pub charter_approved: S,
    pub charter_minor_changes: fn(usize) -> String,
    pub charter_needs_reapproval: S,
    pub versions: S,
    pub version_reason_hint: S,
    pub record_version: S,
    pub version_recorded: S,
    pub no_versions: S,
    pub approved_by: fn(&str, &str) -> String,
    pub version_recorded_state: S,
    pub compare: S,
    pub approve_version: S,
    pub version_approved: S,
    pub diff_between: fn(i64, i64) -> String,
    pub diff_to_current: fn(i64) -> String,
    pub no_differences: S,
    pub field: S,
    pub before: S,
    pub after: S,

    // Team.
    pub team_intro: S,
    pub people: S,
    pub remove_role: S,
    pub person_name_hint: S,
    pub person_email_hint: S,
    pub add_person: S,
    pub person_needs_name: S,
    pub give_role: S,
    pub raci: S,
    pub raci_intro: S,
    pub raci_no_roles: S,
    pub row_charter: S,
    pub row_tollgate: fn(&str) -> String,
    pub row_schedule: S,
    pub row_risks: S,
    pub row_savings: S,
    pub row_control_plan: S,
    pub row_closure: S,
    pub raci_single_a: fn(&str) -> String,

    // Phases and tollgates.
    pub phases_intro: S,
    pub project_is: fn(&str) -> String,
    pub tollgate_of: fn(&str) -> String,
    pub no_tollgate: S,
    pub prepare_tollgate: S,
    pub tollgate_state: fn(&str, i64) -> String,
    pub changed_after_approval: S,
    pub frozen_hash: fn(&str) -> String,
    pub resume: S,
    pub resubmit: S,
    pub submit: S,
    pub submitted: S,
    pub phase_approved_note: S,
    pub checklist: S,
    pub item_manual: S,
    pub item_verified: S,
    pub remove_item: S,
    pub required_missing: fn(usize) -> String,
    pub new_item_hint: S,
    pub item_manual_choice: S,
    pub approvers: S,
    pub anyone_decides: S,
    pub rule: S,
    pub planned_date: S,
    pub pending_outcome: fn(usize, usize) -> String,
    pub your_decision: S,
    pub reservations_hint: S,
    pub comment_hint: S,
    pub record_decision: S,
    pub decision_recorded: S,
    pub decisions: S,
    pub round: S,

    // Board.
    pub wip_over: S,
    pub wip_limits: S,
    pub wip_intro: S,
    pub card_origin: fn(&str) -> String,
    pub tollgate_decision: S,
    pub card_link: fn(&str, &str) -> String,
    pub effectiveness: S,
    pub effectiveness_criterion: S,
    pub effectiveness_date: S,
    pub effectiveness_ok: S,

    // Risks.
    pub risks_intro: S,
    pub risk_matrix: S,
    pub matrix_axes: S,
    pub new_risk_hint: S,
    pub pick_a_risk: S,
    pub r_description: S,
    pub r_cause: S,
    pub r_effect: S,
    pub r_probability: S,
    pub r_impact: S,
    pub r_response: S,
    pub r_plan: S,
    pub r_owner: S,
    pub r_review: S,
    pub r_status: S,
    pub response_to_card: S,
    pub response_card_title: fn(&str) -> String,
    pub card_created: S,

    // Meetings.
    pub meetings_intro: S,
    pub new_meeting_hint: S,
    pub new_meeting: S,
    pub meeting_default_title: fn(&str) -> String,
    pub no_meetings: S,
    pub pick_a_meeting: S,
    pub m_title: S,
    pub m_date: S,
    pub m_kind: S,
    pub m_present: S,
    pub m_agenda: S,
    pub generate_agenda: S,
    pub agenda_nothing: S,
    pub m_notes: S,
    pub m_decisions: S,
    pub minutes_items: S,
    pub card_in: fn(&str) -> String,
    pub no_card_yet: S,
    pub new_minutes_item_hint: S,
    pub actions_to_cards: fn(usize) -> String,
    pub cards_created: fn(usize) -> String,
    pub linked_cards: fn(usize) -> String,
    pub ag_overdue: fn(&str, &str) -> String,
    pub ag_due_soon: fn(&str, &str) -> String,
    pub ag_high_risk: fn(&str, u8) -> String,
    pub ag_risk_review: fn(&str) -> String,
    pub ag_tollgate: fn(&str) -> String,
    pub ag_pending_minutes: fn(&str) -> String,
    pub ag_conflicts: fn(usize) -> String,

    // Schedule.
    pub schedule_intro: S,
    pub schedule_start: S,
    pub workdays: S,
    pub new_task_hint: S,
    pub add_milestones: S,
    pub freeze_baseline: S,
    pub baseline_frozen: S,
    pub t_name: S,
    pub t_duration: S,
    pub t_three_points: S,
    pub t_preds: S,
    pub t_who: S,
    pub t_percent: S,
    pub t_target: S,
    pub milestone: S,
    pub preds_tip: S,
    pub holidays: fn(usize) -> String,
    pub holiday_name_hint: S,
    pub add_national_holidays: S,
    pub schedule_finish: fn(&str, &str) -> String,
    pub schedule_sd: fn(&str) -> String,
    pub deadline_probability: fn(&str, &str) -> String,
    pub run_monte_carlo: S,
    pub monte_carlo_result: fn(usize, &str) -> String,
    pub monte_carlo_deadline: fn(&str) -> String,
    pub w_negative_lag: fn(&str) -> String,
    pub w_unknown_pred: fn(&str) -> String,
    pub w_bad_three_point: fn(&str) -> String,
    pub milestone_probability: fn(&str, &str, &str) -> String,
    pub no_tasks: S,
    pub legend_critical: S,
    pub legend_baseline: S,
    pub legend_today: S,

    // Savings.
    pub savings_intro: S,
    pub definitions: S,
    pub hard: S,
    pub hard_definition: S,
    pub soft: S,
    pub soft_definition: S,
    pub follow_up_default: fn(u32) -> String,
    pub new_saving_hint: S,
    pub value_hint: S,
    pub from_simulation: S,
    pub pick_a_saving: S,
    pub s_description: S,
    pub s_category: S,
    pub s_value: S,
    pub s_rationale: S,
    pub s_monthly: S,
    pub s_start: S,
    pub s_months: S,
    pub move_saving_to: fn(&str) -> String,
    pub back_saving_to: fn(&str) -> String,
    pub forecast_by: fn(&str, &str) -> String,
    pub validated_by: fn(&str, &str) -> String,
    pub monthly_entries: S,
    pub month_hint: S,
    pub record_month: S,
    pub bad_month_or_value: S,
    pub no_follow_up: S,
    pub month: S,
    pub expected: S,
    pub realized: S,
    pub cumulative_expected: S,
    pub cumulative_realized: S,

    // Closing and lessons.
    pub closure_intro: S,
    pub c_result: S,
    pub c_notes: S,
    pub owner_accepted: fn(&str) -> String,
    pub accept_transfer: S,
    pub advice: S,
    pub close_project_now: S,
    pub project_closed: S,
    pub lessons: S,
    pub new_lesson_hint: S,
    pub l_title: S,
    pub l_worked: S,
    pub l_did_not_work: S,
    pub l_repeat: S,
    pub l_tags: S,
    pub knowledge_base: S,
    pub knowledge_intro: S,
    pub search_hint: S,
    pub refresh_portfolio: S,
    pub hits: fn(usize, usize) -> String,
    pub similar_projects: S,

    // Portfolio.
    pub portfolio_intro: S,
    pub no_portfolio: S,
    pub no_portfolio_tip: S,
    pub portfolio_name_hint: S,
    pub unit_name_hint: S,
    pub program_name_hint: S,
    pub create_portfolio: S,
    pub default_portfolio_name: S,
    pub default_unit_name: S,
    pub default_program_name: S,
    pub portfolio_header: fn(&str, &str, &str) -> String,
    pub new_idea_hint: S,
    pub closed_ideas: fn(usize) -> String,
    pub belt_overload: fn(&str, usize) -> String,
    pub i_title: S,
    pub i_process: S,
    pub i_problem: S,
    pub i_data: S,
    pub i_rough_gain: S,
    pub i_impact_effort: S,
    pub i_belt: S,
    pub screening: S,
    pub suggested_approach: fn(&str, &str) -> String,
    pub default_reason: S,
    pub chosen_approach: S,
    pub to_choose: S,
    pub scores: S,
    pub alignment_with: fn(&str) -> String,
    pub idea_score: fn(f64, f64) -> String,
    pub idea_state: fn(&str) -> String,
    pub move_idea_to: fn(&str) -> String,
    pub open_project_from_idea: S,
    pub idea_project: fn(&str, &str) -> String,
    pub choose_approach_first: S,
    pub default_project_title: S,
    pub project_opened_from_idea: fn(&str) -> String,
    pub prioritisation: S,
    pub matrix_title: S,
    pub matrix_tip: S,
    pub ranking: S,
    pub completeness: fn(f64) -> String,
    pub criteria_and_objectives: S,
    pub criterion: S,
    pub weight: S,
    pub scale: S,
    pub inverse: S,
    pub from_alignment: S,
    pub new_criterion_hint: S,
    pub default_criteria: S,
    pub objectives: S,
    pub new_objective_hint: S,
    pub projects_here: S,
    pub no_projects_here: S,
    pub validated_savings: S,
}

pub static PT_BR: L1Texts = L1Texts {
    add: "Adicionar",
    bad_number: "Número inválido: use, por exemplo, 1.234,5.",
    browse: "Procurar…",
    nav_team: "Equipe e RACI",
    nav_closure: "Encerramento e lições",

    picker_title: "Escolher pasta",
    picker_up: "⬆ Subir",
    picker_empty: "(pasta vazia)",
    picker_choose: "Escolher esta pasta",
    picker_cancel: "Cancelar",
    picker_tip: "Clique numa pasta para entrar nela. ▣ marca uma pasta de projeto (duplo clique abre).",
    picker_file_tip: "Clique no arquivo para escolhê-lo.",

    charter_intro: "Os campos são gravados quando você sai deles. Registre uma versão para pedir a aprovação do patrocinador; depois de aprovada, mudar a meta, o prazo ou o escopo pede uma versão nova.",
    charter_no_version: "Nenhuma versão registrada: o termo ainda não foi submetido ao patrocinador.",
    charter_not_approved: "Há versão registrada aguardando a aprovação do patrocinador.",
    charter_approved: "Termo aprovado.",
    charter_minor_changes: |n| {
        format!("{n} campo(s) mudaram desde a aprovação, sem afetar meta, prazo ou escopo.")
    },
    charter_needs_reapproval: "Meta, prazo ou escopo mudaram depois da aprovação: registre uma versão nova e peça nova aprovação.",
    versions: "Versões do termo",
    version_reason_hint: "motivo da versão (por exemplo, meta revista no Medir)",
    record_version: "Registrar versão",
    version_recorded: "Versão registrada.",
    no_versions: "Nenhuma versão ainda.",
    approved_by: |who, when| format!("aprovada por {who} em {when}"),
    version_recorded_state: "aguardando aprovação",
    compare: "Comparar",
    approve_version: "Aprovar (patrocinador)",
    version_approved: "Versão aprovada; a linha de base do cronograma foi congelada.",
    diff_between: |a, b| format!("Diferenças da v{a} para a v{b}"),
    diff_to_current: |a| format!("Diferenças da v{a} para o termo atual"),
    no_differences: "Sem diferenças.",
    field: "Campo",
    before: "Antes",
    after: "Depois",

    team_intro: "Pessoas do projeto e seus papéis (uma pessoa pode ter papéis diferentes em cada projeto). A matriz RACI nasce dos papéis e pode ser ajustada célula a célula.",
    people: "Pessoas",
    remove_role: "Tirar este papel",
    person_name_hint: "nome",
    person_email_hint: "e-mail",
    add_person: "Adicionar pessoa",
    person_needs_name: "Escreva o nome da pessoa.",
    give_role: "Dar o papel",
    raci: "Matriz RACI",
    raci_intro: "R executa, A responde (uma por linha), C é consultado, I é informado. Em destaque, as células mudadas à mão.",
    raci_no_roles: "Defina os papéis acima para gerar a matriz.",
    row_charter: "Termo de abertura",
    row_tollgate: |phase| format!("Tollgate {phase}"),
    row_schedule: "Cronograma",
    row_risks: "Riscos",
    row_savings: "Ganhos",
    row_control_plan: "Plano de controle",
    row_closure: "Encerramento",
    raci_single_a: |rows| format!("Linhas sem exatamente um A: {rows}."),

    phases_intro: "Escolha a fase para ver o checklist do tollgate. Itens verificados são conferidos pelo programa no estado do projeto; itens manuais são marcados por você, com a evidência.",
    project_is: |st| format!("O projeto está {st}."),
    tollgate_of: |p| format!("Tollgate da fase {p}"),
    no_tollgate: "Esta fase ainda não tem tollgate.",
    prepare_tollgate: "Preparar o tollgate (checklist padrão)",
    tollgate_state: |st, round| format!("Situação: {st} (rodada {round})"),
    changed_after_approval: "Alterado depois da aprovação: os entregáveis mudaram desde o congelamento. Submeta de novo para aprovar a versão atual.",
    frozen_hash: |h| format!("Hash congelado: {h}"),
    resume: "Retomar o projeto",
    resubmit: "Submeter de novo",
    submit: "Submeter ao tollgate",
    submitted: "Tollgate submetido aos aprovadores.",
    phase_approved_note: "Fase aprovada.",
    checklist: "Checklist",
    item_manual: "Item manual: marque e escreva a evidência.",
    item_verified: "Item verificado pelo programa.",
    remove_item: "Tirar o item",
    required_missing: |n| format!("{n} item(ns) obrigatório(s) pendente(s)."),
    new_item_hint: "novo item do checklist",
    item_manual_choice: "manual",
    approvers: "Aprovadores:",
    anyone_decides: "(qualquer pessoa)",
    rule: "Regra",
    planned_date: "Data planejada",
    pending_outcome: |a, n| format!("Em revisão: {a} aprovação(ões) de {n} aprovador(es)."),
    your_decision: "Sua decisão",
    reservations_hint: "Ressalvas, uma por linha (cada uma vira uma ação no quadro):",
    comment_hint: "comentário",
    record_decision: "Registrar decisão",
    decision_recorded: "Decisão registrada.",
    decisions: "Decisões",
    round: "rodada",

    wip_over: "Mais cartões que o limite (movimentos simultâneos).",
    wip_limits: "Limites de cartões em andamento por coluna",
    wip_intro: "Vazio ou 0: sem limite. Uma coluna cheia não recebe cartões de outra.",
    card_origin: |o| format!("Origem: {o}"),
    tollgate_decision: "Decisão de tollgate",
    card_link: |kind, name| format!("Ligado a {kind}: {name}"),
    effectiveness: "Verificação de eficácia",
    effectiveness_criterion: "Critério (vazio: não pede verificação)",
    effectiveness_date: "Data da verificação",
    effectiveness_ok: "Eficácia confirmada",

    risks_intro: "Probabilidade e impacto de 1 a 5; o nível é o produto (15 ou mais: alto). A resposta pode virar uma ação no quadro.",
    risk_matrix: "Matriz probabilidade × impacto (riscos abertos)",
    matrix_axes: "Linhas: probabilidade; colunas: impacto.",
    new_risk_hint: "novo risco",
    pick_a_risk: "Escolha um risco na lista.",
    r_description: "Descrição",
    r_cause: "Causa",
    r_effect: "Efeito",
    r_probability: "Probabilidade",
    r_impact: "Impacto",
    r_response: "Resposta",
    r_plan: "Plano de resposta",
    r_owner: "Dono",
    r_review: "Revisão",
    r_status: "Situação",
    response_to_card: "Criar ação da resposta",
    response_card_title: |r| format!("Resposta ao risco: {r}"),
    card_created: "Cartão criado no quadro de ações.",

    meetings_intro: "Pauta gerada com os itens abertos do projeto, presença, anotações, decisões e as ações da ata, que viram cartões com um clique.",
    new_meeting_hint: "título da reunião",
    new_meeting: "Nova reunião",
    meeting_default_title: |d| format!("Reunião de {d}"),
    no_meetings: "Nenhuma reunião ainda.",
    pick_a_meeting: "Escolha uma reunião.",
    m_title: "Título",
    m_date: "Data",
    m_kind: "Tipo",
    m_present: "Presentes",
    m_agenda: "Pauta",
    generate_agenda: "Gerar pauta com os itens abertos",
    agenda_nothing: "Nada pendente no projeto.",
    m_notes: "Anotações",
    m_decisions: "Decisões",
    minutes_items: "Itens da ata",
    card_in: |col| format!("cartão em {col}"),
    no_card_yet: "sem cartão",
    new_minutes_item_hint: "novo item da ata",
    actions_to_cards: |n| format!("Criar cartões das ações da ata ({n})"),
    cards_created: |n| format!("{n} cartão(ões) criado(s) no quadro."),
    linked_cards: |n| format!("{n} cartão(ões) no quadro vieram desta reunião."),
    ag_overdue: |t, d| format!("Ação atrasada: {t} (prazo {d})"),
    ag_due_soon: |t, d| format!("Ação a vencer: {t} (prazo {d})"),
    ag_high_risk: |t, n| format!("Risco alto: {t} (nível {n})"),
    ag_risk_review: |t| format!("Revisar o risco: {t}"),
    ag_tollgate: |p| format!("Tollgate em revisão: {p}"),
    ag_pending_minutes: |t| format!("Ação de ata anterior sem cartão: {t}"),
    ag_conflicts: |n| format!("{n} conflito(s) a revisar"),

    schedule_intro: "Durações em dias úteis (ou três pontos: otimista, mais provável, pessimista). Datas, folgas, caminho crítico e probabilidades calculados pelo PERT/CPM do UTSim.",
    schedule_start: "Início",
    workdays: "Dias úteis",
    new_task_hint: "nova tarefa",
    add_milestones: "Marcos dos tollgates",
    freeze_baseline: "Congelar linha de base",
    baseline_frozen: "Linha de base congelada com as datas atuais.",
    t_name: "Tarefa",
    t_duration: "Dias",
    t_three_points: "O / M / P",
    t_preds: "Depende de",
    t_who: "Responsável",
    t_percent: "%",
    t_target: "Data-alvo",
    milestone: "marco",
    preds_tip: "\"Depende de\": números das linhas, separados por ponto e vírgula; +N dá N dias úteis de defasagem (por exemplo 2; 3+1).",
    holidays: |n| format!("Feriados ({n})"),
    holiday_name_hint: "nome do feriado",
    add_national_holidays: "Incluir os feriados nacionais do ano",
    schedule_finish: |d, n| format!("Término previsto: {d} ({n} dias úteis)."),
    schedule_sd: |sd| format!("Desvio padrão no caminho crítico: {sd} dias."),
    deadline_probability: |d, p| format!("Probabilidade de cumprir o prazo de {d}: {p}."),
    run_monte_carlo: "Rodar Monte Carlo",
    monte_carlo_result: |n, ps| format!("Monte Carlo ({n} sorteios): {ps}"),
    monte_carlo_deadline: |p| format!("Probabilidade de cumprir o prazo no Monte Carlo: {p}."),
    w_negative_lag: |t| format!("{t}: defasagem negativa tratada como zero."),
    w_unknown_pred: |t| format!("{t}: depende de uma tarefa que não existe mais."),
    w_bad_three_point: |t| {
        format!(
            "{t}: os três pontos precisam de otimista ≤ provável ≤ pessimista; usada a duração simples."
        )
    },
    milestone_probability: |m, d, p| format!("{m}: probabilidade de chegar até {d}: {p}."),
    no_tasks: "Nenhuma tarefa ainda.",
    legend_critical: "caminho crítico",
    legend_baseline: "linha de base",
    legend_today: "hoje e datas-alvo",

    savings_intro: "Estados: estimado (belt), previsto (validado pela controladoria), realizado (lançamentos mensais) e validado (controladoria confirma). Ganho estimado por simulação só é validado com lançamentos reais.",
    definitions: "Definições padrão (decisão G1)",
    hard: "Hard",
    hard_definition: "Redução de custo com efeito no resultado (menos material, energia, horas extras, refugo, retrabalho, frete) ou aumento de receita com a mesma estrutura.",
    soft: "Soft",
    soft_definition: "Custo evitado, capacidade liberada, ganho de tempo sem redução de quadro, melhoria de segurança, qualidade percebida ou satisfação, sem efeito direto no resultado.",
    follow_up_default: |m| format!("Acompanhamento padrão: {m} meses depois do Controlar."),
    new_saving_hint: "novo ganho",
    value_hint: "valor (R$)",
    from_simulation: "estimado por simulação",
    pick_a_saving: "Escolha um ganho na lista.",
    s_description: "Descrição",
    s_category: "Categoria",
    s_value: "Valor anual (R$)",
    s_rationale: "Memória de cálculo (fórmula, premissas, fonte)",
    s_monthly: "Valor mensal esperado (R$)",
    s_start: "Início do acompanhamento (MM/AAAA)",
    s_months: "Meses de acompanhamento",
    move_saving_to: |s| format!("Passar para {s}"),
    back_saving_to: |s| format!("Voltar para {s}"),
    forecast_by: |w, d| format!("Previsto validado por {w} em {d}."),
    validated_by: |w, d| format!("Realizado validado por {w} em {d}."),
    monthly_entries: "Lançamentos mensais",
    month_hint: "MM/AAAA",
    record_month: "Lançar",
    bad_month_or_value: "Use o mês como MM/AAAA e um valor numérico.",
    no_follow_up: "Defina o início do acompanhamento ou faça o primeiro lançamento.",
    month: "Mês",
    expected: "Esperado",
    realized: "Realizado",
    cumulative_expected: "Acumulado esperado",
    cumulative_realized: "Acumulado realizado",

    closure_intro: "Resultado do Y contra a meta, aceite do dono do processo, lições aprendidas e as condições para encerrar.",
    c_result: "Resultado do Y contra a meta",
    c_notes: "Observações do encerramento",
    owner_accepted: |w| format!("Transferência aceita pelo dono do processo ({w})."),
    accept_transfer: "Aceitar a transferência (dono do processo)",
    advice: "(recomendado)",
    close_project_now: "Encerrar o projeto",
    project_closed: "Projeto encerrado.",
    lessons: "Lições aprendidas",
    new_lesson_hint: "título da lição",
    l_title: "Título",
    l_worked: "O que funcionou",
    l_did_not_work: "O que não funcionou",
    l_repeat: "O que repetir",
    l_tags: "Etiquetas (processo, área, problema, causa, solução, ferramentas; separadas por vírgula)",
    knowledge_base: "Base de conhecimento",
    knowledge_intro: "Busca nas lições deste projeto e dos outros projetos da mesma pasta (acentos e maiúsculas não importam; todas as palavras precisam aparecer).",
    search_hint: "buscar nas lições",
    refresh_portfolio: "Atualizar a carteira",
    hits: |n, total| format!("{n} lição(ões) encontrada(s) entre {total}."),
    similar_projects: "Projetos parecidos (mesmo processo ou etiquetas em comum)",

    portfolio_intro: "A carteira é a pasta com as pastas dos projetos. O banco de ideias, a triagem, a priorização e a seleção ficam numa pasta de carteira ao lado dos projetos, com o mesmo diário.",
    no_portfolio: "Ainda não há carteira nesta pasta.",
    no_portfolio_tip: "Crie a carteira: ela fica numa pasta própria ao lado dos projetos, ligada à unidade de negócio e ao programa de implantação.",
    portfolio_name_hint: "nome da carteira",
    unit_name_hint: "unidade de negócio (planta, área ou diretoria)",
    program_name_hint: "programa de implantação",
    create_portfolio: "Criar a carteira",
    default_portfolio_name: "Carteira",
    default_unit_name: "Unidade",
    default_program_name: "Programa Lean Seis Sigma",
    portfolio_header: |c, u, p| format!("{c} · {u} · {p}"),
    new_idea_hint: "nova ideia: oportunidade observada",
    closed_ideas: |n| format!("{n} ideia(s) encerrada(s), cancelada(s) ou arquivada(s)."),
    belt_overload: |who, n| format!("{who} está com {n} projetos, acima do limite da carteira."),
    i_title: "Título",
    i_process: "Processo",
    i_problem: "Problema observado",
    i_data: "Dados disponíveis",
    i_rough_gain: "Ganho estimado grosseiro (R$)",
    i_impact_effort: "Impacto × esforço (1 a 5)",
    i_belt: "Belt candidato",
    screening: "Triagem: roteiro de perguntas",
    suggested_approach: |a, why| format!("Abordagem sugerida: {a} ({why})."),
    default_reason: "processo existente, causa desconhecida",
    chosen_approach: "Abordagem escolhida (champion)",
    to_choose: "(a escolher)",
    scores: "Notas por critério",
    alignment_with: |o| format!("Alinhamento: {o} (0, 1, 3, 9)"),
    idea_score: |s, c| format!("Pontuação: {s:.0} de 100 ({c:.0} % dos critérios com nota)."),
    idea_state: |s| format!("Situação: {s}."),
    move_idea_to: |s| format!("→ {s}"),
    open_project_from_idea: "Abrir o projeto (cria a pasta ao lado da carteira)",
    idea_project: |f, id| format!("Projeto aberto: {f} ({id})"),
    choose_approach_first: "Escolha a abordagem antes de abrir o projeto.",
    default_project_title: "Projeto",
    project_opened_from_idea: |p| format!("Projeto criado em {p}"),
    prioritisation: "Priorização",
    matrix_title: "Matriz impacto × esforço",
    matrix_tip: "Impacto e esforço a partir de 3 contam como altos.",
    ranking: "Classificação pela pontuação ponderada",
    completeness: |c| format!("{c:.0} % com nota"),
    criteria_and_objectives: "Critérios e objetivos estratégicos",
    criterion: "Critério",
    weight: "Peso",
    scale: "Escala",
    inverse: "Menor é melhor",
    from_alignment: "vem do alinhamento",
    new_criterion_hint: "novo critério",
    default_criteria: "Usar os critérios padrão",
    objectives: "Objetivos estratégicos",
    new_objective_hint: "novo objetivo estratégico",
    projects_here: "Projetos nesta pasta",
    no_projects_here: "Nenhum projeto ao lado deste.",
    validated_savings: "ganhos validados",
};

pub static EN: L1Texts = L1Texts {
    add: "Add",
    bad_number: "Invalid number: use, for example, 1,234.5.",
    browse: "Browse…",
    nav_team: "Team and RACI",
    nav_closure: "Closure and lessons",

    picker_title: "Choose folder",
    picker_up: "⬆ Up",
    picker_empty: "(empty folder)",
    picker_choose: "Choose this folder",
    picker_cancel: "Cancel",
    picker_tip: "Click a folder to open it. ▣ marks a project folder (double-click opens it).",
    picker_file_tip: "Click the file to choose it.",

    charter_intro: "Fields are saved when you leave them. Record a version to ask for the sponsor's approval; once approved, changing the goal, the deadline or the scope requires a new version.",
    charter_no_version: "No version recorded: the charter has not been submitted to the sponsor yet.",
    charter_not_approved: "A recorded version is waiting for the sponsor's approval.",
    charter_approved: "Charter approved.",
    charter_minor_changes: |n| {
        if n == 1 {
            "1 field changed since the approval, without affecting the goal, deadline or scope."
                .to_string()
        } else {
            format!(
                "{n} fields changed since the approval, without affecting the goal, deadline or scope."
            )
        }
    },
    charter_needs_reapproval: "The goal, deadline or scope changed after the approval: record a new version and ask for a new approval.",
    versions: "Charter versions",
    version_reason_hint: "reason for the version (for example, goal revised in Measure)",
    record_version: "Record version",
    version_recorded: "Version recorded.",
    no_versions: "No versions yet.",
    approved_by: |who, when| format!("approved by {who} on {when}"),
    version_recorded_state: "awaiting approval",
    compare: "Compare",
    approve_version: "Approve (sponsor)",
    version_approved: "Version approved; the schedule baseline was frozen.",
    diff_between: |a, b| format!("Differences from v{a} to v{b}"),
    diff_to_current: |a| format!("Differences from v{a} to the current charter"),
    no_differences: "No differences.",
    field: "Field",
    before: "Before",
    after: "After",

    team_intro: "Project people and their roles (a person can have different roles in each project). The RACI matrix comes from the roles and can be adjusted cell by cell.",
    people: "People",
    remove_role: "Remove this role",
    person_name_hint: "name",
    person_email_hint: "email",
    add_person: "Add person",
    person_needs_name: "Write the person's name.",
    give_role: "Give the role",
    raci: "RACI matrix",
    raci_intro: "R does the work, A is accountable (one per row), C is consulted, I is informed. Cells changed by hand are highlighted.",
    raci_no_roles: "Define the roles above to generate the matrix.",
    row_charter: "Project charter",
    row_tollgate: |phase| format!("Tollgate {phase}"),
    row_schedule: "Schedule",
    row_risks: "Risks",
    row_savings: "Savings",
    row_control_plan: "Control plan",
    row_closure: "Closure",
    raci_single_a: |rows| format!("Rows without exactly one A: {rows}."),

    phases_intro: "Choose the phase to see the tollgate checklist. Verified items are checked by the program against the project state; manual items are ticked by you, with the evidence.",
    project_is: |st| format!("The project is {st}."),
    tollgate_of: |p| format!("Tollgate of the {p} phase"),
    no_tollgate: "This phase has no tollgate yet.",
    prepare_tollgate: "Prepare the tollgate (default checklist)",
    tollgate_state: |st, round| format!("Status: {st} (round {round})"),
    changed_after_approval: "Changed after approval: the deliverables changed since the freeze. Submit again to approve the current version.",
    frozen_hash: |h| format!("Frozen hash: {h}"),
    resume: "Resume the project",
    resubmit: "Submit again",
    submit: "Submit to the tollgate",
    submitted: "Tollgate submitted to the approvers.",
    phase_approved_note: "Phase approved.",
    checklist: "Checklist",
    item_manual: "Manual item: tick it and write the evidence.",
    item_verified: "Item verified by the program.",
    remove_item: "Remove the item",
    required_missing: |n| {
        if n == 1 {
            "1 required item pending.".to_string()
        } else {
            format!("{n} required items pending.")
        }
    },
    new_item_hint: "new checklist item",
    item_manual_choice: "manual",
    approvers: "Approvers:",
    anyone_decides: "(anyone)",
    rule: "Rule",
    planned_date: "Planned date",
    pending_outcome: |a, n| {
        format!(
            "Under review: {a} {} out of {n} {}.",
            if a == 1 { "approval" } else { "approvals" },
            if n == 1 { "approver" } else { "approvers" }
        )
    },
    your_decision: "Your decision",
    reservations_hint: "Reservations, one per line (each becomes an action on the board):",
    comment_hint: "comment",
    record_decision: "Record decision",
    decision_recorded: "Decision recorded.",
    decisions: "Decisions",
    round: "round",

    wip_over: "More cards than the limit (simultaneous moves).",
    wip_limits: "Limits of cards in progress per column",
    wip_intro: "Empty or 0: no limit. A full column does not receive cards from another.",
    card_origin: |o| format!("Origin: {o}"),
    tollgate_decision: "Tollgate decision",
    card_link: |kind, name| format!("Linked to {kind}: {name}"),
    effectiveness: "Effectiveness check",
    effectiveness_criterion: "Criterion (empty: no check required)",
    effectiveness_date: "Check date",
    effectiveness_ok: "Effectiveness confirmed",

    risks_intro: "Probability and impact from 1 to 5; the level is the product (15 or more: high). The response can become an action on the board.",
    risk_matrix: "Probability × impact matrix (open risks)",
    matrix_axes: "Rows: probability; columns: impact.",
    new_risk_hint: "new risk",
    pick_a_risk: "Choose a risk from the list.",
    r_description: "Description",
    r_cause: "Cause",
    r_effect: "Effect",
    r_probability: "Probability",
    r_impact: "Impact",
    r_response: "Response",
    r_plan: "Response plan",
    r_owner: "Owner",
    r_review: "Review",
    r_status: "Status",
    response_to_card: "Create action from the response",
    response_card_title: |r| format!("Risk response: {r}"),
    card_created: "Card created on the actions board.",

    meetings_intro: "Agenda generated from the project's open items, attendance, notes, decisions and the minutes' actions, which become cards with one click.",
    new_meeting_hint: "meeting title",
    new_meeting: "New meeting",
    meeting_default_title: |d| format!("Meeting of {d}"),
    no_meetings: "No meetings yet.",
    pick_a_meeting: "Choose a meeting.",
    m_title: "Title",
    m_date: "Date",
    m_kind: "Type",
    m_present: "Attendees",
    m_agenda: "Agenda",
    generate_agenda: "Generate agenda from the open items",
    agenda_nothing: "Nothing pending in the project.",
    m_notes: "Notes",
    m_decisions: "Decisions",
    minutes_items: "Minutes items",
    card_in: |col| format!("card in {col}"),
    no_card_yet: "no card",
    new_minutes_item_hint: "new minutes item",
    actions_to_cards: |n| format!("Create cards from the minutes' actions ({n})"),
    cards_created: |n| {
        if n == 1 {
            "1 card created on the board.".to_string()
        } else {
            format!("{n} cards created on the board.")
        }
    },
    linked_cards: |n| {
        if n == 1 {
            "1 card on the board came from this meeting.".to_string()
        } else {
            format!("{n} cards on the board came from this meeting.")
        }
    },
    ag_overdue: |t, d| format!("Overdue action: {t} (due {d})"),
    ag_due_soon: |t, d| format!("Action due soon: {t} (due {d})"),
    ag_high_risk: |t, n| format!("High risk: {t} (level {n})"),
    ag_risk_review: |t| format!("Review the risk: {t}"),
    ag_tollgate: |p| format!("Tollgate under review: {p}"),
    ag_pending_minutes: |t| format!("Action from earlier minutes without a card: {t}"),
    ag_conflicts: |n| {
        if n == 1 {
            "1 conflict to review".to_string()
        } else {
            format!("{n} conflicts to review")
        }
    },

    schedule_intro: "Durations in workdays (or three points: optimistic, most likely, pessimistic). Dates, float, critical path and probabilities calculated by UTSim's PERT/CPM.",
    schedule_start: "Start",
    workdays: "Workdays",
    new_task_hint: "new task",
    add_milestones: "Tollgate milestones",
    freeze_baseline: "Freeze baseline",
    baseline_frozen: "Baseline frozen with the current dates.",
    t_name: "Task",
    t_duration: "Days",
    t_three_points: "O / M / P",
    t_preds: "Depends on",
    t_who: "Owner",
    t_percent: "%",
    t_target: "Target date",
    milestone: "milestone",
    preds_tip: "\"Depends on\": row numbers, separated by semicolons; +N adds N workdays of lag (for example 2; 3+1).",
    holidays: |n| format!("Holidays ({n})"),
    holiday_name_hint: "holiday name",
    add_national_holidays: "Add the year's national holidays",
    schedule_finish: |d, n| format!("Expected finish: {d} ({n} workdays)."),
    schedule_sd: |sd| format!("Standard deviation on the critical path: {sd} days."),
    deadline_probability: |d, p| format!("Probability of meeting the {d} deadline: {p}."),
    run_monte_carlo: "Run Monte Carlo",
    monte_carlo_result: |n, ps| format!("Monte Carlo ({n} draws): {ps}"),
    monte_carlo_deadline: |p| format!("Probability of meeting the deadline in Monte Carlo: {p}."),
    w_negative_lag: |t| format!("{t}: negative lag treated as zero."),
    w_unknown_pred: |t| format!("{t}: depends on a task that no longer exists."),
    w_bad_three_point: |t| {
        format!(
            "{t}: the three points need optimistic ≤ most likely ≤ pessimistic; the simple duration was used."
        )
    },
    milestone_probability: |m, d, p| format!("{m}: probability of arriving by {d}: {p}."),
    no_tasks: "No tasks yet.",
    legend_critical: "critical path",
    legend_baseline: "baseline",
    legend_today: "today and target dates",

    savings_intro: "States: estimated (belt), forecast (validated by the controller), realized (monthly entries) and validated (controller confirms). A saving estimated by simulation is only validated with real entries.",
    definitions: "Default definitions (decision G1)",
    hard: "Hard",
    hard_definition: "Cost reduction with an effect on the bottom line (less material, energy, overtime, scrap, rework, freight) or revenue increase with the same structure.",
    soft: "Soft",
    soft_definition: "Avoided cost, freed capacity, time saved without headcount reduction, safety improvement, perceived quality or satisfaction, with no direct effect on the bottom line.",
    follow_up_default: |m| format!("Default follow-up: {m} months after Control."),
    new_saving_hint: "new saving",
    value_hint: "value (R$)",
    from_simulation: "estimated by simulation",
    pick_a_saving: "Choose a saving from the list.",
    s_description: "Description",
    s_category: "Category",
    s_value: "Annual value (R$)",
    s_rationale: "Calculation basis (formula, assumptions, source)",
    s_monthly: "Expected monthly value (R$)",
    s_start: "Follow-up start (MM/YYYY)",
    s_months: "Follow-up months",
    move_saving_to: |s| format!("Move to {s}"),
    back_saving_to: |s| format!("Go back to {s}"),
    forecast_by: |w, d| format!("Forecast validated by {w} on {d}."),
    validated_by: |w, d| format!("Realized value validated by {w} on {d}."),
    monthly_entries: "Monthly entries",
    month_hint: "MM/YYYY",
    record_month: "Record",
    bad_month_or_value: "Use the month as MM/YYYY and a numeric value.",
    no_follow_up: "Set the follow-up start or make the first entry.",
    month: "Month",
    expected: "Expected",
    realized: "Realized",
    cumulative_expected: "Cumulative expected",
    cumulative_realized: "Cumulative realized",

    closure_intro: "Result of Y against the goal, process owner acceptance, lessons learned and the conditions to close.",
    c_result: "Result of Y against the goal",
    c_notes: "Closure notes",
    owner_accepted: |w| format!("Transfer accepted by the process owner ({w})."),
    accept_transfer: "Accept the transfer (process owner)",
    advice: "(recommended)",
    close_project_now: "Close the project",
    project_closed: "Project closed.",
    lessons: "Lessons learned",
    new_lesson_hint: "lesson title",
    l_title: "Title",
    l_worked: "What worked",
    l_did_not_work: "What did not work",
    l_repeat: "What to repeat",
    l_tags: "Tags (process, area, problem, cause, solution, tools; separated by commas)",
    knowledge_base: "Knowledge base",
    knowledge_intro: "Searches the lessons of this project and of the other projects in the same folder (accents and capitals do not matter; all words must appear).",
    search_hint: "search the lessons",
    refresh_portfolio: "Refresh the portfolio",
    hits: |n, total| {
        if n == 1 {
            format!("1 lesson found among {total}.")
        } else {
            format!("{n} lessons found among {total}.")
        }
    },
    similar_projects: "Similar projects (same process or shared tags)",

    portfolio_intro: "The portfolio is the folder that holds the project folders. The idea bank, screening, prioritisation and selection live in a portfolio folder next to the projects, with the same journal.",
    no_portfolio: "There is no portfolio in this folder yet.",
    no_portfolio_tip: "Create the portfolio: it lives in a folder of its own next to the projects, linked to the business unit and to the deployment program.",
    portfolio_name_hint: "portfolio name",
    unit_name_hint: "business unit (plant, area or division)",
    program_name_hint: "deployment program",
    create_portfolio: "Create the portfolio",
    default_portfolio_name: "Portfolio",
    default_unit_name: "Unit",
    default_program_name: "Lean Six Sigma Program",
    portfolio_header: |c, u, p| format!("{c} · {u} · {p}"),
    new_idea_hint: "new idea: observed opportunity",
    closed_ideas: |n| {
        if n == 1 {
            "1 idea closed, cancelled or archived.".to_string()
        } else {
            format!("{n} ideas closed, cancelled or archived.")
        }
    },
    belt_overload: |who, n| format!("{who} has {n} projects, above the portfolio limit."),
    i_title: "Title",
    i_process: "Process",
    i_problem: "Observed problem",
    i_data: "Available data",
    i_rough_gain: "Rough estimated saving (R$)",
    i_impact_effort: "Impact × effort (1 to 5)",
    i_belt: "Candidate belt",
    screening: "Screening: question guide",
    suggested_approach: |a, why| format!("Suggested approach: {a} ({why})."),
    default_reason: "existing process, unknown cause",
    chosen_approach: "Chosen approach (champion)",
    to_choose: "(to be chosen)",
    scores: "Scores by criterion",
    alignment_with: |o| format!("Alignment: {o} (0, 1, 3, 9)"),
    idea_score: |s, c| format!("Score: {s:.0} out of 100 ({c:.0}% of the criteria scored)."),
    idea_state: |s| format!("Status: {s}."),
    move_idea_to: |s| format!("→ {s}"),
    open_project_from_idea: "Open the project (creates the folder next to the portfolio)",
    idea_project: |f, id| format!("Project opened: {f} ({id})"),
    choose_approach_first: "Choose the approach before opening the project.",
    default_project_title: "Project",
    project_opened_from_idea: |p| format!("Project created in {p}"),
    prioritisation: "Prioritisation",
    matrix_title: "Impact × effort matrix",
    matrix_tip: "Impact and effort of 3 or more count as high.",
    ranking: "Ranking by weighted score",
    completeness: |c| format!("{c:.0}% scored"),
    criteria_and_objectives: "Criteria and strategic objectives",
    criterion: "Criterion",
    weight: "Weight",
    scale: "Scale",
    inverse: "Lower is better",
    from_alignment: "comes from alignment",
    new_criterion_hint: "new criterion",
    default_criteria: "Use the default criteria",
    objectives: "Strategic objectives",
    new_objective_hint: "new strategic objective",
    projects_here: "Projects in this folder",
    no_projects_here: "No projects next to this one.",
    validated_savings: "validated savings",
};

pub static ES: L1Texts = L1Texts {
    add: "Agregar",
    bad_number: "Número no válido: use, por ejemplo, 1.234,5.",
    browse: "Examinar…",
    nav_team: "Equipo y RACI",
    nav_closure: "Cierre y lecciones",

    picker_title: "Elegir carpeta",
    picker_up: "⬆ Subir",
    picker_empty: "(carpeta vacía)",
    picker_choose: "Elegir esta carpeta",
    picker_cancel: "Cancelar",
    picker_tip: "Haga clic en una carpeta para entrar en ella. ▣ marca una carpeta de proyecto (doble clic la abre).",
    picker_file_tip: "Haga clic en el archivo para elegirlo.",

    charter_intro: "Los campos se guardan al salir de ellos. Registre una versión para pedir la aprobación del patrocinador; una vez aprobada, cambiar la meta, el plazo o el alcance exige una versión nueva.",
    charter_no_version: "Ninguna versión registrada: el acta de constitución aún no fue enviada al patrocinador.",
    charter_not_approved: "Hay una versión registrada a la espera de la aprobación del patrocinador.",
    charter_approved: "Acta de constitución aprobada.",
    charter_minor_changes: |n| {
        if n == 1 {
            "1 campo cambió desde la aprobación, sin afectar la meta, el plazo ni el alcance."
                .to_string()
        } else {
            format!(
                "{n} campos cambiaron desde la aprobación, sin afectar la meta, el plazo ni el alcance."
            )
        }
    },
    charter_needs_reapproval: "La meta, el plazo o el alcance cambiaron después de la aprobación: registre una versión nueva y pida una nueva aprobación.",
    versions: "Versiones del acta de constitución",
    version_reason_hint: "motivo de la versión (por ejemplo, meta revisada en Medir)",
    record_version: "Registrar versión",
    version_recorded: "Versión registrada.",
    no_versions: "Ninguna versión todavía.",
    approved_by: |who, when| format!("aprobada por {who} el {when}"),
    version_recorded_state: "a la espera de aprobación",
    compare: "Comparar",
    approve_version: "Aprobar (patrocinador)",
    version_approved: "Versión aprobada; la línea base del cronograma fue congelada.",
    diff_between: |a, b| format!("Diferencias de la v{a} a la v{b}"),
    diff_to_current: |a| format!("Diferencias de la v{a} al acta de constitución actual"),
    no_differences: "Sin diferencias.",
    field: "Campo",
    before: "Antes",
    after: "Después",

    team_intro: "Personas del proyecto y sus roles (una persona puede tener roles distintos en cada proyecto). La matriz RACI nace de los roles y se puede ajustar celda por celda.",
    people: "Personas",
    remove_role: "Quitar este rol",
    person_name_hint: "nombre",
    person_email_hint: "correo electrónico",
    add_person: "Agregar persona",
    person_needs_name: "Escriba el nombre de la persona.",
    give_role: "Dar el rol",
    raci: "Matriz RACI",
    raci_intro: "R ejecuta, A responde (una por fila), C es consultado, I es informado. Resaltadas, las celdas cambiadas a mano.",
    raci_no_roles: "Defina los roles arriba para generar la matriz.",
    row_charter: "Acta de constitución",
    row_tollgate: |phase| format!("Tollgate {phase}"),
    row_schedule: "Cronograma",
    row_risks: "Riesgos",
    row_savings: "Ahorros",
    row_control_plan: "Plan de control",
    row_closure: "Cierre",
    raci_single_a: |rows| format!("Filas sin exactamente una A: {rows}."),

    phases_intro: "Elija la fase para ver la lista de verificación del tollgate. Los ítems verificados los comprueba el programa en el estado del proyecto; los ítems manuales los marca usted, con la evidencia.",
    project_is: |st| format!("El proyecto está {st}."),
    tollgate_of: |p| format!("Tollgate de la fase {p}"),
    no_tollgate: "Esta fase aún no tiene tollgate.",
    prepare_tollgate: "Preparar el tollgate (lista de verificación estándar)",
    tollgate_state: |st, round| format!("Situación: {st} (ronda {round})"),
    changed_after_approval: "Modificado después de la aprobación: los entregables cambiaron desde el congelamiento. Envíe de nuevo para aprobar la versión actual.",
    frozen_hash: |h| format!("Hash congelado: {h}"),
    resume: "Retomar el proyecto",
    resubmit: "Enviar de nuevo",
    submit: "Enviar al tollgate",
    submitted: "Tollgate enviado a los aprobadores.",
    phase_approved_note: "Fase aprobada.",
    checklist: "Lista de verificación",
    item_manual: "Ítem manual: márquelo y escriba la evidencia.",
    item_verified: "Ítem verificado por el programa.",
    remove_item: "Quitar el ítem",
    required_missing: |n| {
        if n == 1 {
            "1 ítem obligatorio pendiente.".to_string()
        } else {
            format!("{n} ítems obligatorios pendientes.")
        }
    },
    new_item_hint: "nuevo ítem de la lista",
    item_manual_choice: "manual",
    approvers: "Aprobadores:",
    anyone_decides: "(cualquier persona)",
    rule: "Regla",
    planned_date: "Fecha planificada",
    pending_outcome: |a, n| {
        format!(
            "En revisión: {a} {} de {n} {}.",
            if a == 1 {
                "aprobación"
            } else {
                "aprobaciones"
            },
            if n == 1 { "aprobador" } else { "aprobadores" }
        )
    },
    your_decision: "Su decisión",
    reservations_hint: "Salvedades, una por línea (cada una se vuelve una acción en el tablero):",
    comment_hint: "comentario",
    record_decision: "Registrar decisión",
    decision_recorded: "Decisión registrada.",
    decisions: "Decisiones",
    round: "ronda",

    wip_over: "Más tarjetas que el límite (movimientos simultáneos).",
    wip_limits: "Límites de tarjetas en curso por columna",
    wip_intro: "Vacío o 0: sin límite. Una columna llena no recibe tarjetas de otra.",
    card_origin: |o| format!("Origen: {o}"),
    tollgate_decision: "Decisión de tollgate",
    card_link: |kind, name| format!("Vinculado a {kind}: {name}"),
    effectiveness: "Verificación de eficacia",
    effectiveness_criterion: "Criterio (vacío: no pide verificación)",
    effectiveness_date: "Fecha de la verificación",
    effectiveness_ok: "Eficacia confirmada",

    risks_intro: "Probabilidad e impacto de 1 a 5; el nivel es el producto (15 o más: alto). La respuesta puede convertirse en una acción en el tablero.",
    risk_matrix: "Matriz probabilidad × impacto (riesgos abiertos)",
    matrix_axes: "Filas: probabilidad; columnas: impacto.",
    new_risk_hint: "nuevo riesgo",
    pick_a_risk: "Elija un riesgo de la lista.",
    r_description: "Descripción",
    r_cause: "Causa",
    r_effect: "Efecto",
    r_probability: "Probabilidad",
    r_impact: "Impacto",
    r_response: "Respuesta",
    r_plan: "Plan de respuesta",
    r_owner: "Responsable",
    r_review: "Revisión",
    r_status: "Situación",
    response_to_card: "Crear acción de la respuesta",
    response_card_title: |r| format!("Respuesta al riesgo: {r}"),
    card_created: "Tarjeta creada en el tablero de acciones.",

    meetings_intro: "Agenda generada con los ítems abiertos del proyecto, asistencia, anotaciones, decisiones y las acciones del acta, que se vuelven tarjetas con un clic.",
    new_meeting_hint: "título de la reunión",
    new_meeting: "Nueva reunión",
    meeting_default_title: |d| format!("Reunión del {d}"),
    no_meetings: "Ninguna reunión todavía.",
    pick_a_meeting: "Elija una reunión.",
    m_title: "Título",
    m_date: "Fecha",
    m_kind: "Tipo",
    m_present: "Presentes",
    m_agenda: "Agenda",
    generate_agenda: "Generar agenda con los ítems abiertos",
    agenda_nothing: "Nada pendiente en el proyecto.",
    m_notes: "Anotaciones",
    m_decisions: "Decisiones",
    minutes_items: "Ítems del acta",
    card_in: |col| format!("tarjeta en {col}"),
    no_card_yet: "sin tarjeta",
    new_minutes_item_hint: "nuevo ítem del acta",
    actions_to_cards: |n| format!("Crear tarjetas de las acciones del acta ({n})"),
    cards_created: |n| {
        if n == 1 {
            "1 tarjeta creada en el tablero.".to_string()
        } else {
            format!("{n} tarjetas creadas en el tablero.")
        }
    },
    linked_cards: |n| {
        if n == 1 {
            "1 tarjeta del tablero proviene de esta reunión.".to_string()
        } else {
            format!("{n} tarjetas del tablero provienen de esta reunión.")
        }
    },
    ag_overdue: |t, d| format!("Acción atrasada: {t} (plazo {d})"),
    ag_due_soon: |t, d| format!("Acción por vencer: {t} (plazo {d})"),
    ag_high_risk: |t, n| format!("Riesgo alto: {t} (nivel {n})"),
    ag_risk_review: |t| format!("Revisar el riesgo: {t}"),
    ag_tollgate: |p| format!("Tollgate en revisión: {p}"),
    ag_pending_minutes: |t| format!("Acción de un acta anterior sin tarjeta: {t}"),
    ag_conflicts: |n| {
        if n == 1 {
            "1 conflicto por revisar".to_string()
        } else {
            format!("{n} conflictos por revisar")
        }
    },

    schedule_intro: "Duraciones en días hábiles (o tres puntos: optimista, más probable, pesimista). Fechas, holguras, ruta crítica y probabilidades calculadas por el PERT/CPM de UTSim.",
    schedule_start: "Inicio",
    workdays: "Días hábiles",
    new_task_hint: "nueva tarea",
    add_milestones: "Hitos de los tollgates",
    freeze_baseline: "Congelar línea base",
    baseline_frozen: "Línea base congelada con las fechas actuales.",
    t_name: "Tarea",
    t_duration: "Días",
    t_three_points: "O / M / P",
    t_preds: "Depende de",
    t_who: "Responsable",
    t_percent: "%",
    t_target: "Fecha objetivo",
    milestone: "hito",
    preds_tip: "\"Depende de\": números de las filas, separados por punto y coma; +N da N días hábiles de desfase (por ejemplo 2; 3+1).",
    holidays: |n| format!("Feriados ({n})"),
    holiday_name_hint: "nombre del feriado",
    add_national_holidays: "Incluir los feriados nacionales del año",
    schedule_finish: |d, n| format!("Término previsto: {d} ({n} días hábiles)."),
    schedule_sd: |sd| format!("Desviación estándar en la ruta crítica: {sd} días."),
    deadline_probability: |d, p| format!("Probabilidad de cumplir el plazo de {d}: {p}."),
    run_monte_carlo: "Ejecutar Monte Carlo",
    monte_carlo_result: |n, ps| format!("Monte Carlo ({n} sorteos): {ps}"),
    monte_carlo_deadline: |p| format!("Probabilidad de cumplir el plazo en el Monte Carlo: {p}."),
    w_negative_lag: |t| format!("{t}: desfase negativo tratado como cero."),
    w_unknown_pred: |t| format!("{t}: depende de una tarea que ya no existe."),
    w_bad_three_point: |t| {
        format!(
            "{t}: los tres puntos exigen optimista ≤ probable ≤ pesimista; se usó la duración simple."
        )
    },
    milestone_probability: |m, d, p| format!("{m}: probabilidad de llegar hasta {d}: {p}."),
    no_tasks: "Ninguna tarea todavía.",
    legend_critical: "ruta crítica",
    legend_baseline: "línea base",
    legend_today: "hoy y fechas objetivo",

    savings_intro: "Estados: estimado (belt), previsto (validado por la contraloría), realizado (registros mensuales) y validado (la contraloría confirma). Un ahorro estimado por simulación solo se valida con registros reales.",
    definitions: "Definiciones estándar (decisión G1)",
    hard: "Hard",
    hard_definition: "Reducción de costos con efecto en el resultado (menos material, energía, horas extra, desecho, retrabajo, flete) o aumento de ingresos con la misma estructura.",
    soft: "Soft",
    soft_definition: "Costo evitado, capacidad liberada, ahorro de tiempo sin reducción de plantilla, mejora de seguridad, calidad percibida o satisfacción, sin efecto directo en el resultado.",
    follow_up_default: |m| format!("Seguimiento estándar: {m} meses después de Controlar."),
    new_saving_hint: "nuevo ahorro",
    value_hint: "valor (R$)",
    from_simulation: "estimado por simulación",
    pick_a_saving: "Elija un ahorro de la lista.",
    s_description: "Descripción",
    s_category: "Categoría",
    s_value: "Valor anual (R$)",
    s_rationale: "Memoria de cálculo (fórmula, supuestos, fuente)",
    s_monthly: "Valor mensual esperado (R$)",
    s_start: "Inicio del seguimiento (MM/AAAA)",
    s_months: "Meses de seguimiento",
    move_saving_to: |s| format!("Pasar a {s}"),
    back_saving_to: |s| format!("Volver a {s}"),
    forecast_by: |w, d| format!("Previsto validado por {w} el {d}."),
    validated_by: |w, d| format!("Realizado validado por {w} el {d}."),
    monthly_entries: "Registros mensuales",
    month_hint: "MM/AAAA",
    record_month: "Registrar",
    bad_month_or_value: "Use el mes como MM/AAAA y un valor numérico.",
    no_follow_up: "Defina el inicio del seguimiento o haga el primer registro.",
    month: "Mes",
    expected: "Esperado",
    realized: "Realizado",
    cumulative_expected: "Acumulado esperado",
    cumulative_realized: "Acumulado realizado",

    closure_intro: "Resultado de la Y frente a la meta, aceptación del dueño del proceso, lecciones aprendidas y las condiciones para cerrar.",
    c_result: "Resultado de la Y frente a la meta",
    c_notes: "Observaciones del cierre",
    owner_accepted: |w| format!("Transferencia aceptada por el dueño del proceso ({w})."),
    accept_transfer: "Aceptar la transferencia (dueño del proceso)",
    advice: "(recomendado)",
    close_project_now: "Cerrar el proyecto",
    project_closed: "Proyecto cerrado.",
    lessons: "Lecciones aprendidas",
    new_lesson_hint: "título de la lección",
    l_title: "Título",
    l_worked: "Lo que funcionó",
    l_did_not_work: "Lo que no funcionó",
    l_repeat: "Lo que se debe repetir",
    l_tags: "Etiquetas (proceso, área, problema, causa, solución, herramientas; separadas por coma)",
    knowledge_base: "Base de conocimiento",
    knowledge_intro: "Busca en las lecciones de este proyecto y de los otros proyectos de la misma carpeta (los acentos y las mayúsculas no importan; todas las palabras deben aparecer).",
    search_hint: "buscar en las lecciones",
    refresh_portfolio: "Actualizar la cartera",
    hits: |n, total| {
        if n == 1 {
            format!("1 lección encontrada entre {total}.")
        } else {
            format!("{n} lecciones encontradas entre {total}.")
        }
    },
    similar_projects: "Proyectos parecidos (mismo proceso o etiquetas en común)",

    portfolio_intro: "La cartera es la carpeta que contiene las carpetas de los proyectos. El banco de ideas, el filtrado, la priorización y la selección quedan en una carpeta de cartera junto a los proyectos, con el mismo diario.",
    no_portfolio: "Aún no hay cartera en esta carpeta.",
    no_portfolio_tip: "Cree la cartera: queda en una carpeta propia junto a los proyectos, vinculada a la unidad de negocio y al programa de implantación.",
    portfolio_name_hint: "nombre de la cartera",
    unit_name_hint: "unidad de negocio (planta, área o dirección)",
    program_name_hint: "programa de implantación",
    create_portfolio: "Crear la cartera",
    default_portfolio_name: "Cartera",
    default_unit_name: "Unidad",
    default_program_name: "Programa Lean Seis Sigma",
    portfolio_header: |c, u, p| format!("{c} · {u} · {p}"),
    new_idea_hint: "nueva idea: oportunidad observada",
    closed_ideas: |n| {
        if n == 1 {
            "1 idea cerrada, cancelada o archivada.".to_string()
        } else {
            format!("{n} ideas cerradas, canceladas o archivadas.")
        }
    },
    belt_overload: |who, n| {
        format!("{who} tiene {n} proyectos, por encima del límite de la cartera.")
    },
    i_title: "Título",
    i_process: "Proceso",
    i_problem: "Problema observado",
    i_data: "Datos disponibles",
    i_rough_gain: "Ahorro estimado aproximado (R$)",
    i_impact_effort: "Impacto × esfuerzo (1 a 5)",
    i_belt: "Belt candidato",
    screening: "Filtrado: guía de preguntas",
    suggested_approach: |a, why| format!("Enfoque sugerido: {a} ({why})."),
    default_reason: "proceso existente, causa desconocida",
    chosen_approach: "Enfoque elegido (champion)",
    to_choose: "(por elegir)",
    scores: "Notas por criterio",
    alignment_with: |o| format!("Alineación: {o} (0, 1, 3, 9)"),
    idea_score: |s, c| format!("Puntuación: {s:.0} de 100 ({c:.0} % de los criterios con nota)."),
    idea_state: |s| format!("Situación: {s}."),
    move_idea_to: |s| format!("→ {s}"),
    open_project_from_idea: "Abrir el proyecto (crea la carpeta junto a la cartera)",
    idea_project: |f, id| format!("Proyecto abierto: {f} ({id})"),
    choose_approach_first: "Elija el enfoque antes de abrir el proyecto.",
    default_project_title: "Proyecto",
    project_opened_from_idea: |p| format!("Proyecto creado en {p}"),
    prioritisation: "Priorización",
    matrix_title: "Matriz impacto × esfuerzo",
    matrix_tip: "Impacto y esfuerzo a partir de 3 cuentan como altos.",
    ranking: "Clasificación por la puntuación ponderada",
    completeness: |c| format!("{c:.0} % con nota"),
    criteria_and_objectives: "Criterios y objetivos estratégicos",
    criterion: "Criterio",
    weight: "Peso",
    scale: "Escala",
    inverse: "Menor es mejor",
    from_alignment: "viene de la alineación",
    new_criterion_hint: "nuevo criterio",
    default_criteria: "Usar los criterios estándar",
    objectives: "Objetivos estratégicos",
    new_objective_hint: "nuevo objetivo estratégico",
    projects_here: "Proyectos en esta carpeta",
    no_projects_here: "Ningún proyecto junto a este.",
    validated_savings: "ahorros validados",
};

pub fn l1() -> &'static L1Texts {
    match language() {
        Language::PtBr => &PT_BR,
        Language::En => &EN,
        Language::Es => &ES,
    }
}

/// Picks the text of the current language.
fn pick(pt: S, en: S, es: S) -> S {
    match language() {
        Language::PtBr => pt,
        Language::En => en,
        Language::Es => es,
    }
}

/// Label of a charter (or other) field shown in the differences.
pub fn field_label(f: &str) -> String {
    use fields::project as p;
    let s = match f {
        p::TITLE => pick("Título", "Title", "Título"),
        p::PROCESS => pick("Processo", "Process", "Proceso"),
        p::AREA => pick("Área", "Area", "Área"),
        p::PROBLEM => pick(
            "Declaração do problema (o quê, onde, quando, quanto; sem causa nem solução)",
            "Problem statement (what, where, when, how much; no cause or solution)",
            "Declaración del problema (qué, dónde, cuándo, cuánto; sin causa ni solución)",
        ),
        charter::fields::Y => pick("Y do projeto", "Project Y", "Y del proyecto"),
        p::GOAL => pick(
            "Meta (SMART, com unidade e fonte do dado)",
            "Goal (SMART, with unit and data source)",
            "Meta (SMART, con unidad y fuente del dato)",
        ),
        p::BASELINE => pick("Linha de base", "Baseline", "Línea base"),
        p::DEADLINE => pick("Prazo", "Deadline", "Plazo"),
        p::SCOPE_IN => pick("Dentro do escopo", "In scope", "Dentro del alcance"),
        p::SCOPE_OUT => pick("Fora do escopo", "Out of scope", "Fuera del alcance"),
        p::BUSINESS_CASE => pick(
            "Caso de negócio e ganho estimado",
            "Business case and estimated saving",
            "Caso de negocio y ahorro estimado",
        ),
        charter::fields::ASSUMPTIONS => pick("Premissas", "Assumptions", "Supuestos"),
        charter::fields::CONSTRAINTS => pick("Restrições", "Constraints", "Restricciones"),
        charter::fields::INITIAL_RISKS => {
            pick("Riscos iniciais", "Initial risks", "Riesgos iniciales")
        }
        schedule::fields::START => pick(
            "Início do cronograma",
            "Schedule start",
            "Inicio del cronograma",
        ),
        closure::fields::RESULT => pick("Resultado do Y", "Result of Y", "Resultado de la Y"),
        other => return other.to_string(),
    };
    s.to_string()
}

pub fn role_name(k: RoleKind) -> &'static str {
    match k {
        RoleKind::Sponsor => pick("Patrocinador", "Sponsor", "Patrocinador"),
        RoleKind::Champion => "Champion",
        RoleKind::ProcessOwner => pick("Dono do processo", "Process owner", "Dueño del proceso"),
        RoleKind::MasterBlackBelt => "Master black belt",
        RoleKind::BlackBelt => "Black belt",
        RoleKind::GreenBelt => "Green belt",
        RoleKind::YellowBelt => "Yellow belt",
        RoleKind::Team => pick("Equipe", "Team", "Equipo"),
        RoleKind::Controller => pick("Controladoria", "Controller", "Contraloría"),
    }
}

pub fn check_name(c: &Check) -> String {
    match c {
        Check::CharterComplete => pick(
            "termo com os campos obrigatórios",
            "charter with the required fields",
            "acta de constitución con los campos obligatorios",
        )
        .into(),
        Check::CharterApproved => pick(
            "termo aprovado, sem mudança de meta ou escopo",
            "charter approved, with no change of goal or scope",
            "acta de constitución aprobada, sin cambio de meta ni alcance",
        )
        .into(),
        Check::TeamDefined => pick(
            "patrocinador e belt definidos",
            "sponsor and belt defined",
            "patrocinador y belt definidos",
        )
        .into(),
        Check::ScheduleDefined => pick(
            "cronograma com tarefas e sem ciclo",
            "schedule with tasks and no cycle",
            "cronograma con tareas y sin ciclo",
        )
        .into(),
        Check::RisksAssessed => pick(
            "riscos registrados, cada um aberto com dono",
            "risks recorded, each open one with an owner",
            "riesgos registrados, cada uno abierto con responsable",
        )
        .into(),
        Check::HighRisksTreated => pick(
            "nenhum risco alto sem dono",
            "no high risk without an owner",
            "ningún riesgo alto sin responsable",
        )
        .into(),
        Check::SavingEstimated => pick(
            "ao menos um ganho registrado",
            "at least one saving recorded",
            "al menos un ahorro registrado",
        )
        .into(),
        Check::SavingsForecast => pick(
            "todo ganho ao menos previsto pela controladoria",
            "every saving at least forecast by the controller",
            "todo ahorro al menos previsto por la contraloría",
        )
        .into(),
        Check::SavingsValidated => pick(
            "ao menos um ganho validado",
            "at least one saving validated",
            "al menos un ahorro validado",
        )
        .into(),
        Check::PhaseActionsDone => pick(
            "toda ação da fase em \"feito\"",
            "every action of the phase in \"done\"",
            "toda acción de la fase en \"hecho\"",
        )
        .into(),
        Check::LessonsRecorded => pick(
            "ao menos uma lição aprendida",
            "at least one lesson learned",
            "al menos una lección aprendida",
        )
        .into(),
        Check::OwnerAccepted => pick(
            "aceite do dono do processo",
            "acceptance by the process owner",
            "aceptación del dueño del proceso",
        )
        .into(),
        Check::DeliverableExists(t) => match language() {
            Language::PtBr => format!("existe o entregável \"{t}\""),
            Language::En => format!("the deliverable \"{t}\" exists"),
            Language::Es => format!("existe el entregable \"{t}\""),
        },
        Check::MsaApproved => pick(
            "Gage R&R com %VE do GRR ≤ 30 % ou justificado",
            "Gage R&R with GRR %SV ≤ 30% or justified",
            "Gage R&R con %VE del GRR ≤ 30 % o justificado",
        )
        .into(),
        Check::BaselineReal => pick(
            "capacidade ou carta sobre dado de origem Real",
            "capability or chart on data of Real origin",
            "capacidad o carta sobre dato de origen Real",
        )
        .into(),
    }
}

pub fn decision_name(d: Decision) -> &'static str {
    match d {
        Decision::Approve => pick("Aprovar", "Approve", "Aprobar"),
        Decision::ApproveWithReservations => pick(
            "Aprovar com ressalvas",
            "Approve with reservations",
            "Aprobar con salvedades",
        ),
        Decision::Redo => pick("Refazer", "Redo", "Rehacer"),
        Decision::Suspend => pick("Suspender", "Suspend", "Suspender"),
        Decision::Cancel => pick("Cancelar", "Cancel", "Cancelar"),
    }
}

pub fn rule_name(r: ApprovalRule) -> &'static str {
    match r {
        ApprovalRule::All => pick("todos aprovam", "all approve", "todos aprueban"),
        ApprovalRule::Majority => pick("maioria", "majority", "mayoría"),
        ApprovalRule::SponsorRequired => pick(
            "patrocinador obrigatório",
            "sponsor required",
            "patrocinador obligatorio",
        ),
    }
}

pub fn tollgate_status_name(s: TollgateStatus) -> &'static str {
    match s {
        TollgateStatus::Open => pick("aberto", "open", "abierto"),
        TollgateStatus::Submitted => pick("em revisão", "under review", "en revisión"),
        TollgateStatus::Approved => pick("aprovado", "approved", "aprobado"),
        TollgateStatus::Redo => pick("refazer", "redo", "rehacer"),
        TollgateStatus::Suspended => pick("suspenso", "suspended", "suspendido"),
        TollgateStatus::Cancelled => pick("cancelado", "cancelled", "cancelado"),
    }
}

pub fn project_status_name(s: ProjectStatus) -> &'static str {
    match s {
        ProjectStatus::Idea => pick("ideia", "idea", "idea"),
        ProjectStatus::Screening => pick("triagem", "screening", "filtrado"),
        ProjectStatus::Approved => pick("aprovada", "approved", "aprobado"),
        ProjectStatus::Running => pick("em execução", "running", "en ejecución"),
        ProjectStatus::Closed => pick("encerrada", "closed", "cerrado"),
        ProjectStatus::Cancelled => pick("cancelada", "cancelled", "cancelado"),
        ProjectStatus::Archived => pick("arquivada", "archived", "archivado"),
        ProjectStatus::Suspended => pick("suspensa", "suspended", "suspendido"),
    }
}

pub fn response_name(r: RiskResponse) -> &'static str {
    match r {
        RiskResponse::Avoid => pick("evitar", "avoid", "evitar"),
        RiskResponse::Mitigate => pick("mitigar", "mitigate", "mitigar"),
        RiskResponse::Transfer => pick("transferir", "transfer", "transferir"),
        RiskResponse::Accept => pick("aceitar", "accept", "aceptar"),
    }
}

pub fn risk_status_name(s: RiskStatus) -> &'static str {
    match s {
        RiskStatus::Open => pick("aberto", "open", "abierto"),
        RiskStatus::Closed => pick("fechado", "closed", "cerrado"),
    }
}

pub fn item_kind_name(k: ItemKind) -> &'static str {
    match k {
        ItemKind::Note => pick("Anotação", "Note", "Anotación"),
        ItemKind::Decision => pick("Decisão", "Decision", "Decisión"),
        ItemKind::Action => pick("Ação", "Action", "Acción"),
    }
}

pub fn meeting_kind_name(k: MeetingKind) -> &'static str {
    match k {
        MeetingKind::Team => pick("reunião da equipe", "team meeting", "reunión del equipo"),
        MeetingKind::Tollgate => pick(
            "revisão de tollgate",
            "tollgate review",
            "revisión de tollgate",
        ),
        MeetingKind::Portfolio => pick(
            "reunião da carteira",
            "portfolio meeting",
            "reunión de la cartera",
        ),
    }
}

pub fn weekday_short(d: u8) -> &'static str {
    let days: [S; 7] = match language() {
        Language::PtBr => ["seg", "ter", "qua", "qui", "sex", "sáb", "dom"],
        Language::En => ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
        Language::Es => ["lun", "mar", "mié", "jue", "vie", "sáb", "dom"],
    };
    days.get(usize::from(d.saturating_sub(1)))
        .copied()
        .unwrap_or("?")
}

pub fn category_name(c: SavingCategory) -> &'static str {
    match c {
        SavingCategory::Hard => "hard",
        SavingCategory::Soft => "soft",
    }
}

pub fn source_name(s: Source) -> &'static str {
    match s {
        Source::Belt => "belt",
        Source::Simulation => pick("simulação", "simulation", "simulación"),
        Source::Controller => pick("controladoria", "controller", "contraloría"),
    }
}

pub fn requirement_name(r: Requirement) -> &'static str {
    match r {
        Requirement::TollgatesApproved => pick(
            "Todos os tollgates aprovados",
            "All tollgates approved",
            "Todos los tollgates aprobados",
        ),
        Requirement::OwnerAccepted => pick(
            "Transferência aceita pelo dono do processo",
            "Transfer accepted by the process owner",
            "Transferencia aceptada por el dueño del proceso",
        ),
        Requirement::LessonsRecorded => pick(
            "Lições aprendidas registradas",
            "Lessons learned recorded",
            "Lecciones aprendidas registradas",
        ),
        Requirement::ResultWritten => pick(
            "Resultado do Y registrado",
            "Result of Y recorded",
            "Resultado de la Y registrado",
        ),
        Requirement::SavingsForecast => pick(
            "Ganhos ao menos previstos (o acompanhamento segue depois)",
            "Savings at least forecast (follow-up continues afterwards)",
            "Ahorros al menos previstos (el seguimiento continúa después)",
        ),
        Requirement::ActionsDone => pick(
            "Ações concluídas (as abertas passam ao dono do processo)",
            "Actions completed (open ones go to the process owner)",
            "Acciones concluidas (las abiertas pasan al dueño del proceso)",
        ),
    }
}

pub fn quadrant_name(q: Quadrant) -> &'static str {
    match q {
        Quadrant::QuickWin => pick("Ganho rápido", "Quick win", "Ganancia rápida"),
        Quadrant::Strategic => pick(
            "Projeto estratégico",
            "Strategic project",
            "Proyecto estratégico",
        ),
        Quadrant::Minor => pick("Tarefa menor", "Minor task", "Tarea menor"),
        Quadrant::Avoid => pick("Evitar", "Avoid", "Evitar"),
    }
}

pub fn question_text(q: Question) -> &'static str {
    match q {
        Question::NewDesign => pick(
            "É um processo ou produto novo, ou o atual não tem como atingir a meta?",
            "Is it a new process or product, or can the current one not reach the goal?",
            "¿Es un proceso o producto nuevo, o el actual no puede alcanzar la meta?",
        ),
        Question::CauseKnown => pick(
            "A causa e a solução já são conhecidas?",
            "Are the cause and the solution already known?",
            "¿Ya se conocen la causa y la solución?",
        ),
        Question::LowRisk => pick(
            "O risco da solução é baixo?",
            "Is the risk of the solution low?",
            "¿El riesgo de la solución es bajo?",
        ),
        Question::LocalQuick => pick(
            "O problema é local e a equipe do processo resolve em poucos dias?",
            "Is the problem local and can the process team solve it in a few days?",
            "¿El problema es local y el equipo del proceso lo resuelve en pocos días?",
        ),
        Question::OnePage => pick(
            "Uma página basta para conduzir e comunicar?",
            "Is one page enough to run and communicate it?",
            "¿Una página basta para conducir y comunicar?",
        ),
        Question::ShortCycle => pick(
            "É um ciclo curto e repetitivo de melhoria contínua?",
            "Is it a short, repetitive continuous improvement cycle?",
            "¿Es un ciclo corto y repetitivo de mejora continua?",
        ),
    }
}

pub fn scale_name(s: Scale) -> &'static str {
    match s {
        Scale::OneToFive => pick("1 a 5", "1 to 5", "1 a 5"),
        Scale::OneThreeNine => "1-3-9",
    }
}

pub fn origin_name(o: Origin) -> &'static str {
    match o {
        Origin::Manual => pick("manual", "manual", "manual"),
        Origin::Fmea => pick("FMEA", "FMEA", "AMEF"),
        Origin::Minutes => pick("ata de reunião", "meeting minutes", "acta de reunión"),
        Origin::Tollgate => pick(
            "ressalva de tollgate",
            "tollgate reservation",
            "salvedad de tollgate",
        ),
        Origin::Risk => pick("resposta a risco", "risk response", "respuesta a riesgo"),
        Origin::ResponsePlan => pick("plano de resposta", "response plan", "plan de respuesta"),
        Origin::Kaizen => "kaizen",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatted_texts() {
        let t = l1();
        assert_eq!(
            (t.tollgate_state)("aberto", 0),
            "Situação: aberto (rodada 0)"
        );
        assert_eq!(weekday_short(1), "seg");
        assert_eq!(weekday_short(9), "?");
        assert_eq!(
            field_label("meta"),
            "Meta (SMART, com unidade e fonte do dado)"
        );
        assert_eq!(field_label("x/y"), "x/y");
        assert!(check_name(&Check::DeliverableExists("fmea".into())).contains("fmea"));
        for q in Question::ALL {
            assert!(question_text(q).ends_with('?'));
        }
    }
}
