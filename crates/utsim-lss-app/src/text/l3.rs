//! Texts of the stage L3 and P1 screen (statistical analyses and measurement system
//! analysis), in a catalogue of their own.

use crate::text::{fx, tx};
use utsim_idiomas::PorIdioma;
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

pub fn l3() -> &'static L3Texts {
    static CACHE: PorIdioma<L3Texts> = PorIdioma::new();
    CACHE.obter(construir_l3_texts)
}

fn construir_l3_texts() -> L3Texts {
    L3Texts {
        nav_analyses: tx("l3-nav_analyses"),
        intro: tx("l3-intro"),
        no_data: tx("l3-no_data"),
        csv_hint: tx("l3-csv_hint"),
        import: tx("l3-import"),
        imported: tx("l3-imported"),
        dataset: tx("l3-dataset"),
        choose: tx("l3-choose"),
        not_real: |o| fx("l3-not_real", &[("o", o.into())]),
        analysis: tx("l3-analysis"),
        column: tx("l3-column"),
        column_a: tx("l3-column_a"),
        column_b: tx("l3-column_b"),
        group: tx("l3-group"),
        sizes: tx("l3-sizes"),
        lsl: tx("l3-lsl"),
        usl: tx("l3-usl"),
        subgroup: tx("l3-subgroup"),
        mu0: tx("l3-mu0"),
        pooled: tx("l3-pooled"),
        nelson: tx("l3-nelson"),
        part: tx("l3-part"),
        operator: tx("l3-operator"),
        value: tx("l3-value"),
        tolerance: tx("l3-tolerance"),
        anova_method: tx("l3-anova_method"),
        reference: tx("l3-reference"),
        fill: |w| fx("l3-fill", &[("w", w.into())]),
        run: tx("l3-run"),
        name_hint: tx("l3-name_hint"),
        save: tx("l3-save"),
        saved: tx("l3-saved"),
        saved_title: tx("l3-saved_title"),
        values: tx("l3-values"),
        center: tx("l3-center"),
        lcl: tx("l3-lcl"),
        ucl: tx("l3-ucl"),
        signals: tx("l3-signals"),
        justify_hint: tx("l3-justify_hint"),
        justify: tx("l3-justify"),
        rerun: tx("l3-rerun"),
        same_numbers: tx("l3-same_numbers"),
        different_numbers: tx("l3-different_numbers"),
    }
}

pub fn origin_name(o: DataOrigin) -> &'static str {
    match o {
        DataOrigin::Real => tx("l3-origin_name-real"),
        DataOrigin::Twin => tx("l3-origin_name-twin"),
        DataOrigin::Simulated => tx("l3-origin_name-simulated"),
    }
}

/// Name of an analysis kind on the screen.
pub fn kind_name(k: &str) -> &'static str {
    match k {
        "descritiva" => tx("l3-kind_name-simulated"),
        "normalidade" => tx("l3-kind_name-1"),
        "capacidade" => tx("l3-kind_name-2"),
        "carta" => tx("l3-kind_name-3"),
        "t_uma_amostra" => tx("l3-kind_name-4"),
        "t_duas_amostras" => tx("l3-kind_name-5"),
        "t_pareado" => tx("l3-kind_name-6"),
        "anova" => tx("l3-kind_name-7"),
        "qui_quadrado" => tx("l3-kind_name-8"),
        "proporcao" => tx("l3-kind_name-9"),
        "duas_proporcoes" => tx("l3-kind_name-10"),
        "mann_whitney" => tx("l3-kind_name-11"),
        "wilcoxon" => tx("l3-kind_name-12"),
        "gage_rr" => tx("l3-kind_name-13"),
        "msa_tipo1" => tx("l3-kind_name-14"),
        "msa_vies" => tx("l3-kind_name-15"),
        "msa_linearidade" => tx("l3-kind_name-16"),
        "msa_atributos" => tx("l3-kind_name-17"),
        "experimento_fatorial" => tx("l3-kind_name-18"),
        "variancias" => tx("l3-kind_name-19"),
        "kruskal_wallis" => tx("l3-kind_name-20"),
        "mediana_mood" => tx("l3-kind_name-21"),
        "friedman" => tx("l3-kind_name-22"),
        "anova_dois_fatores" => tx("l3-kind_name-23"),
        "regressao" => tx("l3-kind_name-24"),
        "tamanho_amostra" => tx("l3-kind_name-25"),
        "ewma" => tx("l3-kind_name-ewma"),
        "cusum" => tx("l3-kind_name-cusum"),
        _ => "?",
    }
}
