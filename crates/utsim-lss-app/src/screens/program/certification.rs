//! Certification (8.5, decisions MB4 and MB5): the requirements of each level (the table
//! of MB4 until the company edits it), the check of a candidate on the program's data
//! with the evidence of each requirement, the MBB's opinion, the issue with a sequential
//! number, the manual record (MBB level), the registry with validity and renewal, the
//! renewal rules and the certification funnel.

use std::collections::BTreeMap;

use eframe::egui;
use utsim_lss_core::certificacao::{
    self, Context, RenewalRules, Requirement, Stage, Status, Validity,
};
use utsim_lss_core::formacao::exams;
use utsim_lss_core::op::{EntityRef, Kind};
use utsim_lss_core::perfis::Action;
use utsim_lss_core::programa::BeltLevel;
use utsim_lss_store::program::TrainingFolder;

use super::Data;
use crate::app::App;
use crate::session::Session;
use crate::text::l6::level_name;
use crate::text::l7::{l7, requirement_status_name, validity_text};
use crate::text::texts;
use crate::theme;

const ALL_LEVELS: [BeltLevel; 4] = [
    BeltLevel::Yellow,
    BeltLevel::Green,
    BeltLevel::Black,
    BeltLevel::MasterBlack,
];

impl App {
    /// The results the certification reads: the grades folder for the staff, else the
    /// copies in the class folders the person can open.
    pub(in crate::app) fn exam_results(&mut self, s: &Session, d: &Data) -> Vec<exams::ExamResult> {
        if let Some(g) = self.training_state(s, d, TrainingFolder::Grades) {
            return exams::results(&g);
        }
        let mut v = Vec::new();
        for k in d.tr.classes.clone() {
            if let Some(c) = self.training_state(s, d, TrainingFolder::Class(k.id)) {
                v.extend(exams::results(&c));
            }
        }
        v
    }

    pub(in crate::app) fn program_certification(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
    ) {
        let t = l7();
        ui.heading(t.nav_certification);
        ui.weak(t.certification_intro);
        let results = self.exam_results(s, d);
        let opinions = certificacao::opinions(&d.tr.certs, |p| d.mbbs.contains(&p));
        let ctx = Context {
            classes: &d.tr.classes,
            sessions: &d.tr.sessions,
            nominations: &d.tr.noms,
            exams: &d.tr.exams,
            results: &results,
            projects: &d.records,
            ideas: &d.ideas,
            assignments: &d.assignments,
            map: &d.map,
            opinions: &opinions,
        };
        self.requirements_section(ui, s, d);
        ui.separator();
        self.candidate_section(ui, s, d, &ctx);
        ui.separator();
        self.registry_section(ui, s, d, &ctx);
        ui.separator();
        funnel_section(ui, d, &ctx);
    }

    /// The certification funnel for the MBB dashboard.
    pub(in crate::app) fn certification_funnel(
        &mut self,
        ui: &mut egui::Ui,
        s: &Session,
        d: &Data,
    ) {
        let results = self.exam_results(s, d);
        let opinions = certificacao::opinions(&d.tr.certs, |p| d.mbbs.contains(&p));
        let ctx = Context {
            classes: &d.tr.classes,
            sessions: &d.tr.sessions,
            nominations: &d.tr.noms,
            exams: &d.tr.exams,
            results: &results,
            projects: &d.records,
            ideas: &d.ideas,
            assignments: &d.assignments,
            map: &d.map,
            opinions: &opinions,
        };
        funnel_section(ui, d, &ctx);
    }

    fn requirements_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let t = l7();
        ui.strong(t.requirements);
        ui.horizontal(|ui| {
            ui.label(t.level);
            super::training::level_combo(ui, "nivel_requisitos", &mut self.l6.l7.cert_level);
        });
        let level = self.l6.l7.cert_level;
        let reqs = certificacao::requirements(s.state(), d.program, level);
        let defaults = reqs.iter().all(|r| s.state().entity(r.id).is_none());
        if defaults {
            ui.weak(t.default_requirements_note);
        }
        egui::Grid::new("requisitos").striped(true).show(ui, |ui| {
            for h in [t.requirement, t.count, t.value, t.tool_list] {
                ui.strong(h);
            }
            ui.end_row();
            for r in &reqs {
                ui.label(r.kind.label());
                if defaults {
                    ui.label(r.count.to_string());
                    ui.label(format!("{:.2}", r.value));
                    ui.label(r.tools.join(", "));
                } else {
                    let e = Requirement::entity_ref(r.id);
                    self.num_field(ui, s, e, certificacao::fields::COUNT, 40.0);
                    self.num_field(ui, s, e, certificacao::fields::VALUE, 90.0);
                    ui.label(r.tools.join(", "));
                }
                ui.end_row();
            }
        });
        if defaults
            && level != BeltLevel::MasterBlack
            && ui.button(t.edit_requirements).clicked()
            && self.may_do(s, Action::ManageTraining, None)
        {
            let e = certificacao::install_defaults(d.program, level, s.ids());
            let r = s.apply(e);
            self.report(r);
        }
        let pr = EntityRef::new(Kind::Program, d.program);
        let rules = RenewalRules::read(s.state(), d.program);
        ui.horizontal(|ui| {
            ui.label((t.renewal_rules)(
                rules.projects,
                rules.coached,
                rules.hours,
            ));
            ui.label(t.projects);
            self.num_field(ui, s, pr, certificacao::fields::RENEW_PROJECTS, 36.0);
            ui.label(t.coached);
            self.num_field(ui, s, pr, certificacao::fields::RENEW_COACHED, 36.0);
            ui.label(t.hours);
            self.num_field(ui, s, pr, certificacao::fields::RENEW_HOURS, 44.0);
        });
    }

    fn candidate_section(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
        ctx: &Context<'_>,
    ) {
        let t = l7();
        ui.strong(t.candidate_check);
        ui.horizontal(|ui| {
            if let Some(v) = Self::member_combo(ui, d, "candidato", self.l6.l7.cert_person, false) {
                self.l6.l7.cert_person = v;
            }
            egui::ComboBox::from_id_salt("nivel_candidato")
                .selected_text(level_name(self.l6.l7.cert_level))
                .show_ui(ui, |ui| {
                    for l in ALL_LEVELS {
                        ui.selectable_value(&mut self.l6.l7.cert_level, l, level_name(l));
                    }
                });
        });
        let Some(person) = self.l6.l7.cert_person else {
            return;
        };
        let level = self.l6.l7.cert_level;
        if level == BeltLevel::MasterBlack {
            ui.weak(t.mbb_manual);
        } else {
            let reqs = certificacao::requirements(s.state(), d.program, level);
            let checks = certificacao::check(&reqs, person, level, ctx);
            egui::Grid::new("conferencia").striped(true).show(ui, |ui| {
                for h in [t.requirement, t.status, t.why, t.evidence] {
                    ui.strong(h);
                }
                ui.end_row();
                for c in &checks {
                    ui.label(c.kind.label());
                    let st = requirement_status_name(c.status);
                    match c.status {
                        Status::Met => ui.colored_label(theme::OK, st),
                        Status::Ambiguous => ui.colored_label(theme::WARN, st),
                        Status::NotMet => ui.label(st),
                    };
                    ui.label(&c.reason);
                    ui.label(
                        c.evidence
                            .iter()
                            .map(|e| format!("{} ({}…)", e.text, &e.hash[..8.min(e.hash.len())]))
                            .collect::<Vec<_>>()
                            .join("; "),
                    );
                    ui.end_row();
                }
            });
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(self.l6.input("parecer"))
                        .hint_text(t.opinion_hint)
                        .desired_width(360.0),
                );
                if ui.button(t.record_opinion).clicked() && self.may_do(s, Action::Certify, None) {
                    let text = self.l6.take_input("parecer");
                    match certificacao::record_opinion(
                        d.program,
                        s.new_id(),
                        person,
                        level,
                        &text,
                        d.me,
                        d.today,
                    ) {
                        Ok(e) => {
                            let r = s.apply(e);
                            self.report(r);
                        }
                        Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                    }
                }
            });
            let ready = certificacao::all_met(&checks);
            if ui
                .add_enabled(ready, egui::Button::new(t.issue))
                .on_disabled_hover_text(t.issue_needs_all)
                .clicked()
                && self.may_do(s, Action::Certify, None)
            {
                let entity = ctx
                    .opinions
                    .get(&(person, level))
                    .map_or_else(|| s.new_id(), |(id, _)| *id);
                match certificacao::issue(
                    d.program,
                    entity,
                    person,
                    level,
                    &checks,
                    &d.tr.certs,
                    d.me,
                    d.is_mbb(),
                    d.today,
                ) {
                    Ok(e) => {
                        let r = s.apply(e);
                        if self.report(r).is_some() {
                            self.set_status(t.issued, false);
                        }
                    }
                    Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                }
            }
        }
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(self.l6.input("nota_manual"))
                    .hint_text(t.manual_note_hint)
                    .desired_width(360.0),
            );
            if ui.button(t.record_manual).clicked() && self.may_do(s, Action::Certify, None) {
                let note = self.l6.take_input("nota_manual");
                match certificacao::record_manual(
                    d.program,
                    s.new_id(),
                    person,
                    level,
                    &note,
                    &d.tr.certs,
                    d.me,
                    d.is_mbb(),
                    d.today,
                ) {
                    Ok(e) => {
                        let r = s.apply(e);
                        self.report(r);
                    }
                    Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                }
            }
        });
    }

    fn registry_section(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
        ctx: &Context<'_>,
    ) {
        let t = l7();
        ui.strong(t.registry);
        let rules = RenewalRules::read(s.state(), d.program);
        let mut act: Option<Vec<utsim_lss_core::Edit>> = None;
        egui::Grid::new("registro").striped(true).show(ui, |ui| {
            for h in [
                t.number,
                t.person,
                t.level,
                t.date,
                t.valid_until,
                t.validity,
                "",
            ] {
                ui.strong(h);
            }
            ui.end_row();
            for c in d.tr.certs.iter().filter(|c| c.number.is_some()) {
                ui.label(format!("{:04}", c.number.unwrap_or(0)));
                ui.label(c.person.map(|p| d.person(p)).unwrap_or_default());
                ui.label(level_name(c.level));
                ui.label(c.date.map(|x| x.format_br()).unwrap_or_default());
                ui.label(c.valid_until.map(|x| x.format_br()).unwrap_or_default());
                let v = certificacao::validity(c, &d.tr.certs, d.today);
                let vt = validity_text(v);
                match v {
                    Validity::Valid | Validity::Renewed => ui.label(vt),
                    _ => ui.colored_label(theme::WARN, vt),
                };
                ui.horizontal(|ui| {
                    if matches!(v, Validity::Expiring { .. } | Validity::Expired) {
                        let rc = certificacao::renewal_check(c, &rules, ctx);
                        if ui.small_button(t.renew).on_hover_text(&rc.reason).clicked() {
                            match certificacao::renew(
                                d.program,
                                s.new_id(),
                                c,
                                &rc,
                                &d.tr.certs,
                                d.me,
                                d.is_mbb(),
                                d.today,
                            ) {
                                Ok(e) => act = Some(e),
                                Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                            }
                        }
                    }
                    if c.issued() && ui.small_button(t.revoke).clicked() {
                        let reason = self.l6.take_input("motivo_revogacao");
                        match certificacao::revoke(c, &reason) {
                            Ok(e) => act = Some(e),
                            Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                        }
                    }
                });
                ui.end_row();
            }
        });
        ui.add(
            egui::TextEdit::singleline(self.l6.input("motivo_revogacao"))
                .hint_text(t.revoke_reason_hint)
                .desired_width(320.0),
        );
        if let Some(e) = act
            && self.may_do(s, Action::Certify, None)
        {
            let r = s.apply(e);
            self.report(r);
        }
        let soon = certificacao::to_renew(&d.tr.certs, d.today);
        if !soon.is_empty() {
            ui.colored_label(theme::WARN, (t.to_renew)(soon.len()));
        }
    }
}

fn funnel_section(ui: &mut egui::Ui, d: &Data, ctx: &Context<'_>) {
    let t = l7();
    ui.strong(t.cert_funnel);
    for level in [BeltLevel::Yellow, BeltLevel::Green, BeltLevel::Black] {
        let mut waves: Vec<Option<utsim_lss_core::Id>> =
            d.tr.waves.iter().map(|w| Some(w.id)).collect();
        waves.push(None);
        for w in waves {
            let f = certificacao::funnel(w, level, ctx, &d.tr.certs);
            if f.counts.is_empty() {
                continue;
            }
            let wave = if w.is_some() {
                d.tr.wave_name(w)
            } else {
                t.all_waves.to_string()
            };
            ui.label(format!(
                "{} · {}: {}",
                level_name(level),
                wave,
                Stage::ALL
                    .iter()
                    .map(|s| format!("{} {}", s.label(), f.counts.get(s).copied().unwrap_or(0)))
                    .collect::<Vec<_>>()
                    .join(" → ")
            ));
            let exits: BTreeMap<_, _> = f.exits.clone();
            for ((stage, why), n) in exits {
                ui.weak(format!("   {} — {why}: {n}", stage.label()));
            }
        }
    }
}
