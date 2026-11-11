//! Texts of the gemba page (P7, access through the local network with a token): the switch in
//! the team screen and the page itself.

use crate::text::{fx, tx};
use utsim_idiomas::PorIdioma;

type S = &'static str;
type F1 = fn(&str) -> String;

pub struct L15Texts {
    // Team screen.
    pub title: S,
    pub intro: S,
    pub open_lan: S,
    pub open_lan_help: S,
    pub start: S,
    pub stop: S,
    pub address: S,
    pub copy: S,
    pub copied: S,
    pub lan_warning: S,
    pub local_only: S,
    pub error: F1,
    // The page.
    pub page_title: F1,
    pub page_updated: F1,
    pub page_no_owner: S,
    pub page_none: S,
    pub page_where: S,
    pub forbidden: S,
}

pub fn l15() -> &'static L15Texts {
    static CACHE: PorIdioma<L15Texts> = PorIdioma::new();
    CACHE.obter(construir_l15_texts)
}

fn construir_l15_texts() -> L15Texts {
    L15Texts {
        title: tx("l15-title"),
        intro: tx("l15-intro"),
        open_lan: tx("l15-open_lan"),
        open_lan_help: tx("l15-open_lan_help"),
        start: tx("l15-start"),
        stop: tx("l15-stop"),
        address: tx("l15-address"),
        copy: tx("l15-copy"),
        copied: tx("l15-copied"),
        lan_warning: tx("l15-lan_warning"),
        local_only: tx("l15-local_only"),
        error: |e| fx("l15-error", &[("e", e.into())]),
        page_title: |p| fx("l15-page_title", &[("p", p.into())]),
        page_updated: |t| fx("l15-page_updated", &[("t", t.into())]),
        page_no_owner: tx("l15-page_no_owner"),
        page_none: tx("l15-page_none"),
        page_where: tx("l15-page_where"),
        forbidden: tx("l15-forbidden"),
    }
}
