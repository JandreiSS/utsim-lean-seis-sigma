//! Texts of the control with the twin and the panel (P5): the monitoring section of the
//! control plan.

use crate::text::{fx, tx};
use utsim_idiomas::PorIdioma;

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

pub fn l11() -> &'static L11Texts {
    static CACHE: PorIdioma<L11Texts> = PorIdioma::new();
    CACHE.obter(construir_l11_texts)
}

fn construir_l11_texts() -> L11Texts {
    L11Texts {
        title: tx("l11-title"),
        intro: tx("l11-intro"),
        address: tx("l11-address"),
        address_hint: tx("l11-address_hint"),
        read_now: tx("l11-read_now"),
        reading: tx("l11-reading"),
        auto: tx("l11-auto"),
        c_line: tx("l11-c_line"),
        c_source: tx("l11-c_source"),
        c_response: tx("l11-c_response"),
        c_saving: tx("l11-c_saving"),
        c_baseline: tx("l11-c_baseline"),
        c_per_unit: tx("l11-c_per_unit"),
        c_better: tx("l11-c_better"),
        higher: tx("l11-higher"),
        lower: tx("l11-lower"),
        limits_model: tx("l11-limits_model"),
        limits_series: tx("l11-limits_series"),
        limits_from: |f| fx("l11-limits_from", &[("f", f.into())]),
        from_manual: tx("l11-from_manual"),
        from_data: tx("l11-from_data"),
        from_model: tx("l11-from_model"),
        read_from: |origin, model| {
            fx(
                "l11-read_from",
                &[
                    ("origin", origin.into()),
                    ("model", model.into()),
                    ("tem", (if model.is_empty() { "nao" } else { "sim" }).into()),
                ],
            )
        },
        sources_seen: |n| fx("l11-sources_seen", &[("n", n.into())]),
        summary: |n, mean, signals, new| {
            fx(
                "l11-summary",
                &[
                    ("n", n.into()),
                    ("mean", mean.into()),
                    ("signals", signals.into()),
                    ("new", new.into()),
                ],
            )
        },
        no_limits_yet: tx("l11-no_limits_yet"),
        read_done: |lines, cards| {
            fx(
                "l11-read_done",
                &[("lines", lines.into()), ("cards", cards.into())],
            )
        },
        keep_dataset: tx("l11-keep_dataset"),
        dataset_kept: tx("l11-dataset_kept"),
        book_month: tx("l11-book_month"),
        month: tx("l11-month"),
        realized: |v| fx("l11-realized", &[("v", v.into())]),
        booked: |m| fx("l11-booked", &[("m", m.into())]),
        no_gain_link: tx("l11-no_gain_link"),
        bad_month: tx("l11-bad_month"),
        alerts: tx("l11-alerts"),
        no_lines: tx("l11-no_lines"),
    }
}
