//! Texts of the control with the twin and the panel (P5): the monitoring section of the
//! control plan.

type S = &'static str;

pub struct L11Texts {
    pub title: S,
    pub intro: S,
    pub address: S,
    pub address_hint: S,
    pub read_now: S,
    pub reading: S,
    pub auto: S,
    pub c_line: S,
    pub c_source: S,
    pub c_response: S,
    pub c_saving: S,
    pub c_baseline: S,
    pub c_per_unit: S,
    pub c_better: S,
    pub higher: S,
    pub lower: S,
    pub limits_model: S,
    pub limits_series: S,
    pub limits_from: fn(&str) -> String,
    pub from_manual: S,
    pub from_data: S,
    pub from_model: S,
    pub read_from: fn(&str, &str) -> String,
    pub sources_seen: fn(usize) -> String,
    pub summary: fn(usize, &str, usize, usize) -> String,
    pub no_limits_yet: S,
    pub read_done: fn(usize, usize) -> String,
    pub keep_dataset: S,
    pub dataset_kept: S,
    pub book_month: S,
    pub month: S,
    pub realized: fn(&str) -> String,
    pub booked: fn(&str) -> String,
    pub no_gain_link: S,
    pub bad_month: S,
    pub alerts: S,
    pub no_lines: S,
}

static PT_BR: L11Texts = L11Texts {
    title: "Controle em operação pelo gêmeo",
    intro: "Cada linha aponta para onde a característica é lida em operação: um KPI do painel \
            do gêmeo por janela (kpi:vazao) ou uma tag do gêmeo (tag:Torno1.diametro). A \
            leitura confere a série com os limites congelados (fase II, regras de Western \
            Electric); um ponto novo fora de controle abre o cartão do sinal de resposta.",
    address: "Painel ou gravações",
    address_hint: "http://127.0.0.1:8080, um JSON salvo do painel, a pasta de gravações ou um .jsonl",
    read_now: "Ler agora",
    reading: "Lendo o gêmeo…",
    auto: "Ler a cada minuto enquanto aberto",
    c_line: "Característica",
    c_source: "Fonte no gêmeo",
    c_response: "Sinal de resposta",
    c_saving: "Ganho",
    c_baseline: "Linha de base",
    c_per_unit: "R$/mês por unidade",
    c_better: "Melhor",
    higher: "maior",
    lower: "menor",
    limits_model: "Do modelo",
    limits_series: "Fase I da série",
    limits_from: |f| format!("limites: {f}"),
    from_manual: "digitados",
    from_data: "fase I dos dados",
    from_model: "modelo validado",
    read_from: |origin, model| {
        if model.is_empty() {
            format!("Lido de {origin}")
        } else {
            format!("Lido de {origin} (modelo {model})")
        }
    },
    sources_seen: |n| format!("{n} séries disponíveis"),
    summary: |n, mean, signals, new| {
        format!("{n} pontos, média {mean}, {signals} fora de controle ({new} novos)")
    },
    no_limits_yet: "sem limites ainda",
    read_done: |lines, cards| {
        format!("Gêmeo lido: {lines} linhas conferidas, {cards} cartões abertos")
    },
    keep_dataset: "Guardar como conjunto Gêmeo",
    dataset_kept: "Série guardada como conjunto de dados de origem Gêmeo",
    book_month: "Lançar realizado",
    month: "Mês (AAAA-MM)",
    realized: |v| format!("realizado ao mês: {v}"),
    booked: |m| format!("Realizado lançado no mês {m}; falta a validação da controladoria"),
    no_gain_link: "ligue a linha a um ganho, com linha de base e valor por unidade",
    bad_month: "mês inválido: use AAAA-MM",
    alerts: "Alertas do gêmeo",
    no_lines: "Acrescente linhas ao plano de controle para monitorá-las.",
};

pub fn l11() -> &'static L11Texts {
    &PT_BR
}
