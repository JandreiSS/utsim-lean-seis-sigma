//! Texts of the version 2 of the app (estimated gain from a study, the other MSA studies, design
//! of experiments, value stream and spaghetti maps), in a catalogue of their own.

type S = &'static str;

pub struct L10Texts {
    pub book_gain: S,
    pub gain_title: S,
    pub gain_intro: S,
    pub gain_difference: S,
    pub gain_description: S,
    pub gain_value_per_unit: S,
    pub gain_periods: S,
    pub gain_higher: S,
    pub gain_conservative: S,
    pub gain_soft: S,
    pub gain_preview: fn(&str) -> String,
    pub gain_book: S,
    pub gain_booked: S,
    pub gain_file_changed: S,
    pub gain_bad_number: S,
    pub cancel: S,
    pub reference_col: S,
    pub measured_col: S,
    pub process_variation: S,
    pub tolerance_opt: S,
    pub item: S,
    pub appraiser: S,
    pub response: S,
    pub with_standard: S,
    pub standard: S,
}

pub static PT_BR: L10Texts = L10Texts {
    book_gain: "Lançar ganho estimado",
    gain_title: "Ganho estimado a partir do estudo",
    gain_intro: "Escolha uma diferença significativa do estudo e diga quanto vale uma unidade do indicador e quantas vezes por ano o horizonte simulado acontece. O ganho entra em Ganhos como estimado, origem simulação, com a memória de cálculo e o estudo de onde veio. Ele só passa a validado com lançamentos reais.",
    gain_difference: "Diferença",
    gain_description: "Descrição do ganho",
    gain_value_per_unit: "Valor por unidade do indicador",
    gain_periods: "Períodos por ano",
    gain_higher: "Aumento do indicador é melhora (desmarque para tempos e filas)",
    gain_conservative: "Conservador: usar a ponta do intervalo mais próxima de zero",
    gain_soft: "Ganho soft (evitado, não entra no resultado)",
    gain_preview: |v| format!("Valor estimado: {v} por ano"),
    gain_book: "Lançar em Ganhos",
    gain_booked: "Ganho estimado lançado em Ganhos",
    gain_file_changed: "O arquivo do estudo mudou desde que foi guardado (hash diferente); guarde o estudo de novo.",
    gain_bad_number: "Valor por unidade ou períodos por ano inválido",
    cancel: "Cancelar",
    reference_col: "Coluna da referência",
    measured_col: "Coluna da medida",
    process_variation: "Variação do processo (6σ, opcional)",
    tolerance_opt: "Tolerância (opcional)",
    item: "Item",
    appraiser: "Avaliador",
    response: "Resposta",
    with_standard: "Com padrão conhecido",
    standard: "Padrão",
};

pub fn l10() -> &'static L10Texts {
    &PT_BR
}
