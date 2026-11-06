//! Texts of the stage L3 and P1 screen (statistical analyses and measurement system
//! analysis), in a catalogue of their own.

use utsim_lss_core::dados::DataOrigin;

type S = &'static str;

pub struct L3Texts {
    pub nav_analyses: S,
    pub intro: S,
    pub no_data: S,
    pub csv_hint: S,
    pub import: S,
    pub imported: S,
    pub dataset: S,
    pub choose: S,
    pub not_real: fn(&str) -> String,
    pub analysis: S,
    pub column: S,
    pub column_a: S,
    pub column_b: S,
    pub group: S,
    pub sizes: S,
    pub lsl: S,
    pub usl: S,
    pub subgroup: S,
    pub mu0: S,
    pub pooled: S,
    pub nelson: S,
    pub part: S,
    pub operator: S,
    pub value: S,
    pub tolerance: S,
    pub anova_method: S,
    pub reference: S,
    pub fill: fn(&str) -> String,
    pub run: S,
    pub name_hint: S,
    pub save: S,
    pub saved: S,
    pub saved_title: S,
    pub values: S,
    pub center: S,
    pub lcl: S,
    pub ucl: S,
    pub signals: S,
    pub justify_hint: S,
    pub justify: S,
    pub rerun: S,
    pub same_numbers: S,
    pub different_numbers: S,
}

pub static PT_BR: L3Texts = L3Texts {
    nav_analyses: "Dados e análises",
    intro: "Importe os dados (CSV, com vírgula ou ponto decimal) com a origem, escolha a análise e as colunas. Cada análise guarda a receita, o conjunto de dados com o hash e a origem e o resultado: o relatório do tollgate a cita, e qualquer pessoa a refaz com os mesmos números.",
    no_data: "Nenhum conjunto de dados ainda: importe um CSV ou guarde um conjunto simulado na tela Modelo UTSim.",
    csv_hint: "caminho do arquivo CSV",
    import: "Importar",
    imported: "Conjunto importado para dados/",
    dataset: "Conjunto:",
    choose: "escolha",
    not_real: |o| format!("Origem {o}: não é coleta real; não serve de linha de base real."),
    analysis: "Análise:",
    column: "Coluna",
    column_a: "Coluna A",
    column_b: "Coluna B",
    group: "Grupo",
    sizes: "Tamanhos",
    lsl: "LIE",
    usl: "LSE",
    subgroup: "Subgrupo",
    mu0: "μ0",
    pooled: "variância combinada",
    nelson: "regras de Nelson",
    part: "Peça",
    operator: "Operador",
    value: "Medida",
    tolerance: "Tolerância",
    anova_method: "método da ANOVA",
    reference: "Referência",
    fill: |w| format!("preencha {w}"),
    run: "Calcular",
    name_hint: "nome da análise",
    save: "Guardar a análise",
    saved: "Análise guardada",
    saved_title: "Análises do projeto",
    values: "valores",
    center: "LC",
    lcl: "LIC",
    ucl: "LSC",
    signals: "sinais",
    justify_hint: "justificativa do MSA acima de 30 %",
    justify: "Justificar",
    rerun: "Refazer",
    same_numbers: "Os mesmos números ao refazer a receita",
    different_numbers: "Os números mudaram ao refazer: o arquivo ou a versão do app mudou",
};

pub fn l3() -> &'static L3Texts {
    &PT_BR
}

pub fn origin_name(o: DataOrigin) -> &'static str {
    o.label()
}

/// Name of an analysis kind on the screen.
pub fn kind_name(k: &str) -> &'static str {
    match k {
        "descritiva" => "Estatística descritiva",
        "normalidade" => "Normalidade (Anderson-Darling)",
        "capacidade" => "Capacidade do processo",
        "carta" => "Carta de controle",
        "t_uma_amostra" => "Teste t de uma amostra",
        "t_duas_amostras" => "Teste t de duas amostras",
        "t_pareado" => "Teste t pareado",
        "anova" => "ANOVA de um fator com Tukey",
        "qui_quadrado" => "Qui-quadrado de independência",
        "proporcao" => "Teste de uma proporção",
        "duas_proporcoes" => "Teste de duas proporções",
        "mann_whitney" => "Mann-Whitney",
        "wilcoxon" => "Wilcoxon pareado",
        "gage_rr" => "MSA: Gage R&R",
        "msa_tipo1" => "MSA: estudo do tipo 1",
        _ => "?",
    }
}
