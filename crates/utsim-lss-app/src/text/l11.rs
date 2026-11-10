//! Texts of the control with the twin and the panel (P5): the monitoring section of the
//! control plan.

use crate::text::{Language, language};

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

static EN: L11Texts = L11Texts {
    title: "Control in operation through the twin",
    intro: "Each line points to where the characteristic is read in operation: a KPI of the twin panel per window (kpi:vazao) or a twin tag (tag:Torno1.diametro). The reading checks the series against the frozen limits (phase II, Western Electric rules); a new out-of-control point opens the response-signal card.",
    address: "Panel or recordings",
    address_hint: "http://127.0.0.1:8080, a JSON saved from the panel, the recordings folder or a .jsonl",
    read_now: "Read now",
    reading: "Reading the twin…",
    auto: "Read every minute while open",
    c_line: "Characteristic",
    c_source: "Source in the twin",
    c_response: "Response signal",
    c_saving: "Savings",
    c_baseline: "Baseline",
    c_per_unit: "R$/month per unit",
    c_better: "Better",
    higher: "higher",
    lower: "lower",
    limits_model: "From the model",
    limits_series: "Phase I of the series",
    limits_from: |f| format!("limits: {f}"),
    from_manual: "typed in",
    from_data: "phase I of the data",
    from_model: "validated model",
    read_from: |origin, model| {
        if model.is_empty() {
            format!("Read from {origin}")
        } else {
            format!("Read from {origin} (model {model})")
        }
    },
    sources_seen: |n| {
        if n == 1 {
            "1 series available".to_string()
        } else {
            format!("{n} series available")
        }
    },
    summary: |n, mean, signals, new| {
        format!("{n} points, mean {mean}, {signals} out of control ({new} new)")
    },
    no_limits_yet: "no limits yet",
    read_done: |lines, cards| format!("Twin read: {lines} lines checked, {cards} cards opened"),
    keep_dataset: "Keep as a Twin dataset",
    dataset_kept: "Series kept as a dataset with Twin as its origin",
    book_month: "Book actual",
    month: "Month (YYYY-MM)",
    realized: |v| format!("actual for the month: {v}"),
    booked: |m| format!("Actual booked for month {m}; controller validation is pending"),
    no_gain_link: "link the line to a saving, with a baseline and a value per unit",
    bad_month: "invalid month: use YYYY-MM",
    alerts: "Twin alerts",
    no_lines: "Add lines to the control plan to monitor them.",
};

static ES: L11Texts = L11Texts {
    title: "Control en operación mediante el gemelo",
    intro: "Cada línea apunta al lugar donde se lee la característica en operación: un KPI del panel del gemelo por ventana (kpi:vazao) o una tag del gemelo (tag:Torno1.diametro). La lectura contrasta la serie con los límites congelados (fase II, reglas de Western Electric); un punto nuevo fuera de control abre la tarjeta de la señal de respuesta.",
    address: "Panel o grabaciones",
    address_hint: "http://127.0.0.1:8080, un JSON guardado del panel, la carpeta de grabaciones o un .jsonl",
    read_now: "Leer ahora",
    reading: "Leyendo el gemelo…",
    auto: "Leer cada minuto mientras esté abierto",
    c_line: "Característica",
    c_source: "Fuente en el gemelo",
    c_response: "Señal de respuesta",
    c_saving: "Ahorro",
    c_baseline: "Línea base",
    c_per_unit: "R$/mes por unidad",
    c_better: "Mejor",
    higher: "mayor",
    lower: "menor",
    limits_model: "Del modelo",
    limits_series: "Fase I de la serie",
    limits_from: |f| format!("límites: {f}"),
    from_manual: "digitados",
    from_data: "fase I de los datos",
    from_model: "modelo validado",
    read_from: |origin, model| {
        if model.is_empty() {
            format!("Leído de {origin}")
        } else {
            format!("Leído de {origin} (modelo {model})")
        }
    },
    sources_seen: |n| {
        if n == 1 {
            "1 serie disponible".to_string()
        } else {
            format!("{n} series disponibles")
        }
    },
    summary: |n, mean, signals, new| {
        format!("{n} puntos, media {mean}, {signals} fuera de control ({new} nuevos)")
    },
    no_limits_yet: "aún sin límites",
    read_done: |lines, cards| {
        format!("Gemelo leído: {lines} líneas contrastadas, {cards} tarjetas abiertas")
    },
    keep_dataset: "Guardar como conjunto Gemelo",
    dataset_kept: "Serie guardada como conjunto de datos de origen Gemelo",
    book_month: "Registrar lo realizado",
    month: "Mes (AAAA-MM)",
    realized: |v| format!("realizado en el mes: {v}"),
    booked: |m| format!("Realizado registrado en el mes {m}; falta la validación de contraloría"),
    no_gain_link: "vincule la línea a un ahorro, con línea base y valor por unidad",
    bad_month: "mes inválido: use AAAA-MM",
    alerts: "Alertas del gemelo",
    no_lines: "Agregue líneas al plan de control para monitorearlas.",
};

pub fn l11() -> &'static L11Texts {
    match language() {
        Language::PtBr => &PT_BR,
        Language::En => &EN,
        Language::Es => &ES,
    }
}
