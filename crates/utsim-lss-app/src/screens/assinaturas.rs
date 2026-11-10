//! The team's signatures (stage P7): the check of the signed journal on the team screen and
//! the seal of an approval next to it (tollgate decisions and validated savings).

use eframe::egui;
use utsim_lss_core::{Date, Id, domain};
use utsim_lss_store::assinatura::{self, Approval, Finding, Problema, SealStatus};

use super::super::App;
use crate::session::Session;
use crate::text::l13::l13;
use crate::theme;

fn when(s: &Session, ms: i64) -> String {
    Date::from_unix_ms(ms, s.clock().utc_offset_minutes()).format_br()
}

fn name(s: &Session, id: Id) -> String {
    domain::person_name(s.state(), id)
}

/// The text of one finding.
pub(crate) fn finding_text(s: &Session, f: &Finding) -> String {
    let t = l13();
    let op_when = |op: &Id| when(s, op.unix_ms() as i64);
    match f {
        Finding::Chain {
            person, problem, ..
        } => {
            let n = name(s, *person);
            match problem {
                Problema::Falta { depois_de: 0 } => (t.chain_start)(&n),
                Problema::Falta { depois_de } => (t.chain_gap)(&n, &depois_de.to_string()),
                Problema::ForaDeOrdem { sequencia } => (t.chain_order)(&n, &sequencia.to_string()),
                Problema::Bifurcacao { sequencia } => (t.chain_fork)(&n, &sequencia.to_string()),
                Problema::HashNaoConfere { .. } => (t.chain_hash)(&n),
                Problema::AssinaturaInvalida { .. } => (t.chain_signature)(&n),
                Problema::OutroAutor { .. } => (t.chain_author)(&n),
                Problema::CorpoInvalido { .. } => (t.chain_body)(&n),
            }
        }
        Finding::Altered { op, author } => (t.altered)(&name(s, *author), &op_when(op)),
        Finding::Redacted { op, author } => (t.redacted)(&name(s, *author), &op_when(op)),
        Finding::OtherSigner { author, signer, .. } => {
            (t.other_signer)(&name(s, *author), &name(s, *signer))
        }
        Finding::Missing { op, author } => (t.missing)(&name(s, *author), &op_when(op)),
        Finding::Unsigned { op, author } => (t.unsigned)(&name(s, *author), &op_when(op)),
        Finding::ForeignKey { person, by, .. } => (t.foreign_key)(&name(s, *person), &name(s, *by)),
        Finding::KeyNotProven { person, short } => {
            (t.key_not_proven)(&name(s, *person), &assinatura::grouped(short))
        }
        Finding::ChainWithoutKey { folder } => (t.chain_without_key)(folder),
    }
}

impl App {
    /// The check of the signed journal, at the end of the team screen.
    pub(in crate::app) fn integrity_section(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = l13();
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.strong(t.integrity_title);
            if ui.small_button(t.recheck).clicked() {
                s.recheck();
            }
        });
        ui.weak(t.integrity_intro);
        let r = s.signatures().0.clone();
        if r.people.is_empty() {
            ui.label(t.nobody_signs);
            return;
        }
        let severe = r.severe();
        let other = r.findings.len() - severe;
        if r.findings.is_empty() {
            ui.colored_label(
                theme::OK,
                format!("✔ {}", (t.integrity_ok)(r.signed, r.ops)),
            );
        } else {
            let color = if severe > 0 {
                ui.visuals().error_fg_color
            } else {
                theme::WARN
            };
            ui.colored_label(
                color,
                format!(
                    "⚠ {}",
                    (t.integrity_summary)(r.signed, r.ops, severe, other)
                ),
            );
        }
        egui::Grid::new("integridade")
            .num_columns(5)
            .striped(true)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                for h in [
                    t.col_person,
                    t.col_key,
                    t.col_signed,
                    t.col_before,
                    t.col_links,
                ] {
                    ui.strong(h);
                }
                ui.end_row();
                for p in &r.people {
                    ui.label(name(s, p.person));
                    let keys: Vec<String> = p
                        .keys
                        .iter()
                        .filter(|k| k.proven)
                        .map(|k| assinatura::grouped(&k.short))
                        .collect();
                    ui.monospace(keys.join(", "));
                    ui.label(p.signed.to_string());
                    ui.label(p.before_key.to_string());
                    ui.label(p.links.to_string());
                    ui.end_row();
                }
            });
        let mut notes: Vec<String> = Vec::new();
        for p in &r.people {
            let proven: Vec<_> = p.keys.iter().filter(|k| k.proven).collect();
            for k in proven.iter().skip(1) {
                notes.push((t.new_key)(
                    &name(s, p.person),
                    &assinatura::grouped(&k.short),
                    &when(s, k.since_ms),
                ));
            }
        }
        if r.findings.is_empty() && notes.is_empty() {
            return;
        }
        egui::CollapsingHeader::new(t.details)
            .default_open(severe > 0)
            .show(ui, |ui| {
                for f in &r.findings {
                    let txt = finding_text(s, f);
                    if f.severe() {
                        ui.colored_label(ui.visuals().error_fg_color, format!("⚠ {txt}"));
                    } else {
                        ui.label(format!("• {txt}"));
                    }
                }
                for n in &notes {
                    ui.weak(format!("• {n}"));
                }
            });
    }

    /// The seal of an approval: its status and, for the person who decided on a computer
    /// with a key, the button that signs it.
    pub(in crate::app) fn approval_seal(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        approval: Option<Approval>,
    ) {
        let Some(a) = approval else {
            return;
        };
        let t = l13();
        let (r, seals) = s.signatures().clone();
        let status = seals.status(&r, &a);
        let who = name(s, a.person);
        match status {
            SealStatus::Valid { at_ms } => {
                ui.colored_label(theme::OK, (t.signed_by)(&who, &when(s, at_ms)));
            }
            SealStatus::Outdated { .. } => {
                ui.colored_label(theme::WARN, (t.outdated)(&who));
            }
            SealStatus::Invalid => {
                ui.colored_label(ui.visuals().error_fg_color, t.invalid);
            }
            SealStatus::None => {
                ui.weak(t.not_signed);
            }
        }
        let mine = a.person == s.me() && s.folder().signer().is_some();
        if mine
            && !matches!(status, SealStatus::Valid { .. })
            && ui.small_button(t.sign).on_hover_text(t.sign_help).clicked()
        {
            match s.sign(&a) {
                Ok(()) => self.set_status(t.signed_ok, false),
                Err(e) => self.set_status(e.to_string(), true),
            }
        }
    }
}

impl App {
    /// The signature of the program's role list by the MBB (P9, decision MB2).
    pub(in crate::app) fn roles_signature(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &super::program::Data,
        grants: &[utsim_lss_core::perfis::Grant],
    ) {
        let t = l13();
        ui.add_space(8.0);
        ui.strong(t.roles_title);
        ui.weak(t.roles_intro);
        let (r, seals) = s.signatures().clone();
        let c = assinatura::roles_check(&seals, &r, d.program, d.founder, grants);
        match c.signed {
            None => {
                ui.colored_label(theme::WARN, t.roles_never);
            }
            Some((who, at)) if c.current() => {
                ui.colored_label(theme::OK, (t.roles_current)(&d.person(who), &when(s, at)));
            }
            Some((who, at)) => {
                ui.colored_label(
                    theme::WARN,
                    (t.roles_changed)(
                        &d.person(who),
                        &when(s, at),
                        c.unsigned.len(),
                        c.removed.len(),
                    ),
                );
                for g in &c.unsigned {
                    ui.label(format!(
                        "• {}",
                        (t.roles_out)(
                            &g.person.map(|p| d.person(p)).unwrap_or_default(),
                            super::program::profile_name(g.profile),
                        )
                    ));
                }
                for item in &c.removed {
                    let who = item
                        .split(':')
                        .next()
                        .and_then(|p| p.parse::<Id>().ok())
                        .map(|p| d.person(p))
                        .unwrap_or_default();
                    let profile = item.split(':').nth(1).unwrap_or_default();
                    ui.weak(format!(
                        "• {}",
                        (t.roles_removed)(&format!("{who} ({profile})"))
                    ));
                }
            }
        }
        for p in &c.not_entitled {
            ui.colored_label(
                ui.visuals().error_fg_color,
                format!("⚠ {}", (t.roles_not_entitled)(&d.person(*p))),
            );
        }
        let may = (d.me == d.founder || d.is_mbb()) && s.folder().signer().is_some();
        if may && !c.current() && ui.button(t.roles_sign).on_hover_text(t.sign_help).clicked() {
            let a = assinatura::roles_approval(d.program, grants, d.me);
            match s.sign(&a) {
                Ok(()) => self.set_status(t.signed_ok, false),
                Err(e) => self.set_status(e.to_string(), true),
            }
        }
    }
}
