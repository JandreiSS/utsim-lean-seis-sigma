//! Texts of the DMAIC roadmap screen, in a catalogue of their own.

use crate::text::tx;
use utsim_idiomas::PorIdioma;

type S = &'static str;

pub struct L9Texts {
    pub nav_roadmap: S,
    pub intro: S,
    pub zoom: S,
    pub legend_done: S,
    pub legend_todo: S,
    pub legend_soon: S,
    pub soon: S,
    pub yes: S,
    pub no: S,
}

pub fn l9() -> &'static L9Texts {
    static CACHE: PorIdioma<L9Texts> = PorIdioma::new();
    CACHE.obter(construir_l9_texts)
}

fn construir_l9_texts() -> L9Texts {
    L9Texts {
        nav_roadmap: tx("l9-nav_roadmap"),
        intro: tx("l9-intro"),
        zoom: tx("l9-zoom"),
        legend_done: tx("l9-legend_done"),
        legend_todo: tx("l9-legend_todo"),
        legend_soon: tx("l9-legend_soon"),
        soon: tx("l9-soon"),
        yes: tx("l9-yes"),
        no: tx("l9-no"),
    }
}
