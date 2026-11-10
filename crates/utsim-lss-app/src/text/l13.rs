//! Texts of the team's signatures (P7): the signing key, the check of the signed journal and
//! the sealed approvals.

use crate::text::{tx, fx};
use utsim_idiomas::PorIdioma;

type S = &'static str;
type F1 = fn(&str) -> String;
type F2 = fn(&str, &str) -> String;

pub struct L13Texts {
    // Key.
    pub key_title: S,
    pub key_intro: S,
    pub key_later: S,
    pub key_file: F1,
    pub key_error: F1,
    pub no_key: S,
    // Check.
    pub integrity_title: S,
    pub integrity_intro: S,
    pub recheck: S,
    pub integrity_ok: fn(usize, usize) -> String,
    pub integrity_summary: fn(usize, usize, usize, usize) -> String,
    pub nobody_signs: S,
    pub col_person: S,
    pub col_key: S,
    pub col_signed: S,
    pub col_before: S,
    pub col_links: S,
    pub details: S,
    pub chain_gap: F2,
    pub chain_start: F1,
    pub chain_order: F2,
    pub chain_fork: F2,
    pub chain_hash: F1,
    pub chain_signature: F1,
    pub chain_author: F1,
    pub chain_body: F1,
    pub altered: F2,
    pub redacted: F2,
    pub other_signer: F2,
    pub missing: F2,
    pub unsigned: F2,
    pub foreign_key: F2,
    pub key_not_proven: F2,
    pub chain_without_key: F1,
    pub new_key: fn(&str, &str, &str) -> String,
    pub severe_status: fn(usize) -> String,
    // Approvals.
    pub sign: S,
    pub sign_help: S,
    pub signed_by: F2,
    pub outdated: F1,
    pub invalid: S,
    pub not_signed: S,
    pub signed_ok: S,
    // Certificates and the role list (P9).
    pub pdf_signed: fn(&str, &str, &str, &str) -> String,
    pub roles_title: S,
    pub roles_intro: S,
    pub roles_never: S,
    pub roles_current: F2,
    pub roles_changed: fn(&str, &str, usize, usize) -> String,
    pub roles_out: F2,
    pub roles_removed: F1,
    pub roles_not_entitled: F1,
    pub roles_sign: S,
    // My actions.
    pub my_actions: fn(usize) -> String,
    pub my_actions_none: S,
    pub col_project: S,
    pub col_action: S,
    pub col_due: S,
    pub col_column: S,
    pub late: F1,
    pub open: S,
}







pub fn l13() -> &'static L13Texts {
    static CACHE: PorIdioma<L13Texts> = PorIdioma::new();
    CACHE.obter(construir_l13_texts)
}

fn construir_l13_texts() -> L13Texts {
    L13Texts {
        key_title: tx("l13-key_title"),
        key_intro: tx("l13-key_intro"),
        key_later: tx("l13-key_later"),
        key_file: |p| fx("l13-key_file", &[("p", p.into())]),
        key_error: |e| fx("l13-key_error", &[("e", e.into())]),
        no_key: tx("l13-no_key"),
        integrity_title: tx("l13-integrity_title"),
        integrity_intro: tx("l13-integrity_intro"),
        recheck: tx("l13-recheck"),
        integrity_ok: |signed, total| fx("l13-integrity_ok", &[("signed", signed.into()), ("total", total.into())]),
        integrity_summary: |signed, total, severe, other| fx("l13-integrity_summary", &[("signed", signed.into()), ("total", total.into()), ("severe", severe.into()), ("other", other.into())]),
        nobody_signs: tx("l13-nobody_signs"),
        col_person: tx("l13-col_person"),
        col_key: tx("l13-col_key"),
        col_signed: tx("l13-col_signed"),
        col_before: tx("l13-col_before"),
        col_links: tx("l13-col_links"),
        details: tx("l13-details"),
        chain_gap: |n, after| fx("l13-chain_gap", &[("n", n.into()), ("after", after.into())]),
        chain_start: |n| fx("l13-chain_start", &[("n", n.into())]),
        chain_order: |n, seq| fx("l13-chain_order", &[("n", n.into()), ("seq", seq.into())]),
        chain_fork: |n, seq| fx("l13-chain_fork", &[("n", n.into()), ("seq", seq.into())]),
        chain_hash: |n| fx("l13-chain_hash", &[("n", n.into())]),
        chain_signature: |n| fx("l13-chain_signature", &[("n", n.into())]),
        chain_author: |n| fx("l13-chain_author", &[("n", n.into())]),
        chain_body: |n| fx("l13-chain_body", &[("n", n.into())]),
        altered: |n, when| fx("l13-altered", &[("n", n.into()), ("when", when.into())]),
        redacted: |n, when| fx("l13-redacted", &[("n", n.into()), ("when", when.into())]),
        other_signer: |n, signer| fx("l13-other_signer", &[("n", n.into()), ("signer", signer.into())]),
        missing: |n, when| fx("l13-missing", &[("n", n.into()), ("when", when.into())]),
        unsigned: |n, when| fx("l13-unsigned", &[("n", n.into()), ("when", when.into())]),
        foreign_key: |n, by| fx("l13-foreign_key", &[("n", n.into()), ("by", by.into())]),
        key_not_proven: |n, key| fx("l13-key_not_proven", &[("n", n.into()), ("key", key.into())]),
        chain_without_key: |f| fx("l13-chain_without_key", &[("f", f.into())]),
        new_key: |n, key, when| fx("l13-new_key", &[("n", n.into()), ("key", key.into()), ("when", when.into())]),
        severe_status: |n| fx("l13-severe_status", &[("n", n.into())]),
        sign: tx("l13-sign"),
        sign_help: tx("l13-sign_help"),
        signed_by: |n, when| fx("l13-signed_by", &[("n", n.into()), ("when", when.into())]),
        outdated: |n| fx("l13-outdated", &[("n", n.into())]),
        invalid: tx("l13-invalid"),
        not_signed: tx("l13-not_signed"),
        signed_ok: tx("l13-signed_ok"),
        pdf_signed: |issuer, when, key, seal| fx("l13-pdf_signed", &[("issuer", issuer.into()), ("when", when.into()), ("key", key.into()), ("seal", seal.into())]),
        roles_title: tx("l13-roles_title"),
        roles_intro: tx("l13-roles_intro"),
        roles_never: tx("l13-roles_never"),
        roles_current: |who, when| fx("l13-roles_current", &[("who", who.into()), ("when", when.into())]),
        roles_changed: |who, when, added, removed| fx("l13-roles_changed", &[("who", who.into()), ("when", when.into()), ("added", added.into()), ("removed", removed.into())]),
        roles_out: |who, profile| fx("l13-roles_out", &[("who", who.into()), ("profile", profile.into())]),
        roles_removed: |item| fx("l13-roles_removed", &[("item", item.into())]),
        roles_not_entitled: |who| fx("l13-roles_not_entitled", &[("who", who.into())]),
        roles_sign: tx("l13-roles_sign"),
        my_actions: |n| fx("l13-my_actions", &[("n", n.into())]),
        my_actions_none: tx("l13-my_actions_none"),
        col_project: tx("l13-col_project"),
        col_action: tx("l13-col_action"),
        col_due: tx("l13-col_due"),
        col_column: tx("l13-col_column"),
        late: |d| fx("l13-late", &[("d", d.into())]),
        open: tx("l13-open"),
    }
}
