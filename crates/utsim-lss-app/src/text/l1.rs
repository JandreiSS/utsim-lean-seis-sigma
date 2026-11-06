//! Texts of the stage L1 screens (portfolio, charter versions, team and RACI, tollgates,
//! risks, meetings, schedule, savings, closing and lessons, folder picker), in a catalogue of
//! their own next to the main one, so the stages grow without touching the same lines.

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

pub fn l1() -> &'static L1Texts {
    &PT_BR
}

/// Label of a charter (or other) field shown in the differences.
pub fn field_label(f: &str) -> String {
    use fields::project as p;
    let s = match f {
        p::TITLE => "Título",
        p::PROCESS => "Processo",
        p::AREA => "Área",
        p::PROBLEM => "Declaração do problema (o quê, onde, quando, quanto; sem causa nem solução)",
        charter::fields::Y => "Y do projeto",
        p::GOAL => "Meta (SMART, com unidade e fonte do dado)",
        p::BASELINE => "Linha de base",
        p::DEADLINE => "Prazo",
        p::SCOPE_IN => "Dentro do escopo",
        p::SCOPE_OUT => "Fora do escopo",
        p::BUSINESS_CASE => "Caso de negócio e ganho estimado",
        charter::fields::ASSUMPTIONS => "Premissas",
        charter::fields::CONSTRAINTS => "Restrições",
        charter::fields::INITIAL_RISKS => "Riscos iniciais",
        schedule::fields::START => "Início do cronograma",
        closure::fields::RESULT => "Resultado do Y",
        other => return other.to_string(),
    };
    s.to_string()
}

pub fn role_name(k: RoleKind) -> &'static str {
    match k {
        RoleKind::Sponsor => "Patrocinador",
        RoleKind::Champion => "Champion",
        RoleKind::ProcessOwner => "Dono do processo",
        RoleKind::MasterBlackBelt => "Master black belt",
        RoleKind::BlackBelt => "Black belt",
        RoleKind::GreenBelt => "Green belt",
        RoleKind::YellowBelt => "Yellow belt",
        RoleKind::Team => "Equipe",
        RoleKind::Controller => "Controladoria",
    }
}

pub fn check_name(c: &Check) -> String {
    match c {
        Check::CharterComplete => "termo com os campos obrigatórios".into(),
        Check::CharterApproved => "termo aprovado, sem mudança de meta ou escopo".into(),
        Check::TeamDefined => "patrocinador e belt definidos".into(),
        Check::ScheduleDefined => "cronograma com tarefas e sem ciclo".into(),
        Check::RisksAssessed => "riscos registrados, cada um aberto com dono".into(),
        Check::HighRisksTreated => "nenhum risco alto sem dono".into(),
        Check::SavingEstimated => "ao menos um ganho registrado".into(),
        Check::SavingsForecast => "todo ganho ao menos previsto pela controladoria".into(),
        Check::SavingsValidated => "ao menos um ganho validado".into(),
        Check::PhaseActionsDone => "toda ação da fase em \"feito\"".into(),
        Check::LessonsRecorded => "ao menos uma lição aprendida".into(),
        Check::OwnerAccepted => "aceite do dono do processo".into(),
        Check::DeliverableExists(t) => format!("existe o entregável \"{t}\""),
        Check::MsaApproved => "Gage R&R com %VE do GRR ≤ 30 % ou justificado".into(),
        Check::BaselineReal => "capacidade ou carta sobre dado de origem Real".into(),
    }
}

pub fn decision_name(d: Decision) -> &'static str {
    match d {
        Decision::Approve => "Aprovar",
        Decision::ApproveWithReservations => "Aprovar com ressalvas",
        Decision::Redo => "Refazer",
        Decision::Suspend => "Suspender",
        Decision::Cancel => "Cancelar",
    }
}

pub fn rule_name(r: ApprovalRule) -> &'static str {
    match r {
        ApprovalRule::All => "todos aprovam",
        ApprovalRule::Majority => "maioria",
        ApprovalRule::SponsorRequired => "patrocinador obrigatório",
    }
}

pub fn tollgate_status_name(s: TollgateStatus) -> &'static str {
    match s {
        TollgateStatus::Open => "aberto",
        TollgateStatus::Submitted => "em revisão",
        TollgateStatus::Approved => "aprovado",
        TollgateStatus::Redo => "refazer",
        TollgateStatus::Suspended => "suspenso",
        TollgateStatus::Cancelled => "cancelado",
    }
}

pub fn project_status_name(s: ProjectStatus) -> &'static str {
    match s {
        ProjectStatus::Idea => "ideia",
        ProjectStatus::Screening => "triagem",
        ProjectStatus::Approved => "aprovada",
        ProjectStatus::Running => "em execução",
        ProjectStatus::Closed => "encerrada",
        ProjectStatus::Cancelled => "cancelada",
        ProjectStatus::Archived => "arquivada",
        ProjectStatus::Suspended => "suspensa",
    }
}

pub fn response_name(r: RiskResponse) -> &'static str {
    match r {
        RiskResponse::Avoid => "evitar",
        RiskResponse::Mitigate => "mitigar",
        RiskResponse::Transfer => "transferir",
        RiskResponse::Accept => "aceitar",
    }
}

pub fn risk_status_name(s: RiskStatus) -> &'static str {
    match s {
        RiskStatus::Open => "aberto",
        RiskStatus::Closed => "fechado",
    }
}

pub fn item_kind_name(k: ItemKind) -> &'static str {
    match k {
        ItemKind::Note => "Anotação",
        ItemKind::Decision => "Decisão",
        ItemKind::Action => "Ação",
    }
}

pub fn meeting_kind_name(k: MeetingKind) -> &'static str {
    match k {
        MeetingKind::Team => "reunião da equipe",
        MeetingKind::Tollgate => "revisão de tollgate",
        MeetingKind::Portfolio => "reunião da carteira",
    }
}

pub fn weekday_short(d: u8) -> &'static str {
    ["seg", "ter", "qua", "qui", "sex", "sáb", "dom"]
        .get(usize::from(d.saturating_sub(1)))
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
        Source::Simulation => "simulação",
        Source::Controller => "controladoria",
    }
}

pub fn requirement_name(r: Requirement) -> &'static str {
    match r {
        Requirement::TollgatesApproved => "Todos os tollgates aprovados",
        Requirement::OwnerAccepted => "Transferência aceita pelo dono do processo",
        Requirement::LessonsRecorded => "Lições aprendidas registradas",
        Requirement::ResultWritten => "Resultado do Y registrado",
        Requirement::SavingsForecast => "Ganhos ao menos previstos (o acompanhamento segue depois)",
        Requirement::ActionsDone => "Ações concluídas (as abertas passam ao dono do processo)",
    }
}

pub fn quadrant_name(q: Quadrant) -> &'static str {
    match q {
        Quadrant::QuickWin => "Ganho rápido",
        Quadrant::Strategic => "Projeto estratégico",
        Quadrant::Minor => "Tarefa menor",
        Quadrant::Avoid => "Evitar",
    }
}

pub fn question_text(q: Question) -> &'static str {
    match q {
        Question::NewDesign => {
            "É um processo ou produto novo, ou o atual não tem como atingir a meta?"
        }
        Question::CauseKnown => "A causa e a solução já são conhecidas?",
        Question::LowRisk => "O risco da solução é baixo?",
        Question::LocalQuick => "O problema é local e a equipe do processo resolve em poucos dias?",
        Question::OnePage => "Uma página basta para conduzir e comunicar?",
        Question::ShortCycle => "É um ciclo curto e repetitivo de melhoria contínua?",
    }
}

pub fn scale_name(s: Scale) -> &'static str {
    match s {
        Scale::OneToFive => "1 a 5",
        Scale::OneThreeNine => "1-3-9",
    }
}

pub fn origin_name(o: Origin) -> &'static str {
    match o {
        Origin::Manual => "manual",
        Origin::Fmea => "FMEA",
        Origin::Minutes => "ata de reunião",
        Origin::Tollgate => "ressalva de tollgate",
        Origin::Risk => "resposta a risco",
        Origin::ResponsePlan => "plano de resposta",
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
