//! Profiles and personal data (8.8, decision MB8): the program's role list kept by the
//! MBB, the matrix of what each profile does, the practice of the restricted folders on
//! the Drive, the consent of each person with its purpose, the retention and the
//! anonymisation, and the operations without effect.

use eframe::egui::{self, RichText};
use utsim_lss_core::op::{EntityRef, FIELD_DELETED, Kind};
use utsim_lss_core::perfis::{self, Action, Profile};
use utsim_lss_core::privacidade::{self, Due};
use utsim_lss_core::programa::Member;
use utsim_lss_core::{Edit, Id};
use utsim_lss_store::program;

use super::{Data, profile_name};
use crate::app::App;
use crate::session::Session;
use crate::text::l6::{action_name, l6};
use crate::text::texts;
use crate::theme;

impl App {
    pub(in crate::app) fn program_profiles(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
    ) {
        let tl = l6();
        ui.heading(tl.nav_profiles);
        ui.weak(tl.profiles_intro);
        // The role list.
        ui.strong(tl.role_list);
        let grants = perfis::grants(s.state());
        ui.label((tl.founder_is)(&d.person(d.founder)));
        let mut remove = None;
        egui::Grid::new("perfis").striped(true).show(ui, |ui| {
            for g in &grants {
                ui.label(g.person.map_or(String::new(), |p| d.person(p)));
                ui.label(profile_name(g.profile));
                ui.label(
                    g.unit
                        .map_or(tl.all_units.to_string(), |u| d.unit_name(Some(u))),
                );
                if ui.small_button(texts().delete).clicked() {
                    remove = Some(g.id);
                }
                ui.end_row();
            }
        });
        if let Some(g) = remove {
            let e = vec![Edit::new(
                EntityRef::new(Kind::ProgramRole, g),
                FIELD_DELETED,
                true,
            )];
            self.commit(s, e);
        }
        ui.horizontal(|ui| {
            if let Some(p) = Self::member_combo(ui, d, "perfil_pessoa", self.l6.grant_person, false)
            {
                self.l6.grant_person = p;
            }
            let mut pr = self.l6.grant_profile;
            egui::ComboBox::from_id_salt("perfil_tipo")
                .selected_text(profile_name(pr))
                .show_ui(ui, |ui| {
                    for p in Profile::ALL {
                        ui.selectable_value(&mut pr, p, profile_name(p));
                    }
                });
            self.l6.grant_profile = pr;
            if matches!(pr, Profile::Champion | Profile::Sponsor)
                && let Some(u) =
                    Self::unit_combo(ui, d, "perfil_unidade", self.l6.grant_unit, tl.all_units)
            {
                self.l6.grant_unit = u;
            }
            if ui.button(tl.give_profile).clicked() {
                match self.l6.grant_person {
                    Some(person) => {
                        let unit = if matches!(pr, Profile::Champion | Profile::Sponsor) {
                            self.l6.grant_unit
                        } else {
                            None
                        };
                        let e = perfis::grant(d.program, s.new_id(), person, pr, unit);
                        self.commit(s, e);
                    }
                    None => self.set_status(tl.choose_person, true),
                }
            }
        });
        // What each profile does.
        ui.add_space(6.0);
        ui.collapsing(tl.matrix, |ui| {
            egui::Grid::new("matriz_perfis")
                .striped(true)
                .show(ui, |ui| {
                    ui.strong(tl.action);
                    for p in Profile::ALL {
                        ui.small(profile_name(p));
                    }
                    ui.end_row();
                    for a in ALL_ACTIONS {
                        ui.label(action_name(a));
                        for p in Profile::ALL {
                            ui.label(if perfis::allowed(p, a) { "●" } else { "" });
                        }
                        ui.end_row();
                    }
                });
        });
        ui.collapsing(tl.restricted_folders, |ui| {
            for line in tl.restricted_lines {
                ui.label(format!("• {line}"));
            }
        });
        ui.separator();
        // My consent.
        ui.strong(tl.consent);
        ui.weak(tl.consent_intro);
        let me = d.members.iter().find(|m| m.id == d.me).cloned();
        if let Some(m) = &me {
            ui.horizontal(|ui| {
                let mut c = m.consent;
                if ui.checkbox(&mut c, tl.i_consent).changed() {
                    let purpose = if m.purpose.is_empty() {
                        tl.default_purpose.to_string()
                    } else {
                        m.purpose.clone()
                    };
                    let e = privacidade::consent(d.me, c, &purpose, d.today);
                    self.commit(s, e);
                }
                self.text_field(
                    ui,
                    s,
                    Member::entity_ref(d.me),
                    utsim_lss_core::programa::fields::PURPOSE,
                    false,
                    320.0,
                );
            });
            if let Some(on) = m.consent_on {
                ui.weak((tl.consent_on)(&on.format_br()));
            }
        }
        if d.is_mbb() {
            egui::Grid::new("consentimentos")
                .striped(true)
                .show(ui, |ui| {
                    for m in d.members.iter().filter(|m| !m.anonymised) {
                        ui.label(&m.name);
                        ui.label(if m.consent {
                            tl.consent_yes
                        } else {
                            tl.consent_no
                        });
                        ui.weak(&m.purpose);
                        ui.end_row();
                    }
                });
        }
        ui.separator();
        // Retention and anonymisation.
        ui.strong(tl.retention_title);
        ui.label((tl.retention_is)(privacidade::retention_months(
            s.state(),
            d.program,
        )));
        let due = privacidade::due(
            s.state(),
            d.program,
            d.founder,
            &d.members,
            &d.records,
            d.today,
            s.clock().utc_offset_minutes(),
        );
        if due.is_empty() {
            ui.weak(tl.none_due);
        }
        let mut anon = None;
        for (p, why) in &due {
            ui.horizontal(|ui| {
                let reason = match why {
                    Due::ConsentWithdrawn => tl.due_consent,
                    Due::Retention => tl.due_retention,
                };
                ui.label(format!("{} · {reason}", d.person(*p)));
                if ui.button(tl.anonymise).clicked() {
                    anon = Some(*p);
                }
            });
        }
        if let Some(p) = anon {
            self.anonymise(s, d, p);
        }
        // Operations without effect.
        let un = s.unauthorized();
        if !un.is_empty() {
            ui.separator();
            ui.strong((tl.unauthorized_banner)(un.len()));
            ui.weak(tl.unauthorized_intro);
            for u in un.iter().rev().take(50) {
                ui.label(
                    RichText::new((tl.unauthorized_line)(
                        &d.person(u.author),
                        u.entity.kind.key(),
                        &u.field,
                        action_name(u.action),
                    ))
                    .color(theme::WARN),
                );
            }
        }
    }

    /// The anonymisation: the operation in the program and in the belt's coaching log,
    /// then the rewriting of the history of those fields in both folders.
    fn anonymise(&mut self, s: &mut Session, d: &Data, person: Id) {
        let tl = l6();
        if !self.may_do(s, Action::Anonymise, None) {
            return;
        }
        let e = privacidade::anonymise(s.state(), person);
        if !self.commit(s, e) {
            return;
        }
        let log_path = program::coaching_path(s.folder().root(), person);
        if log_path.join(utsim_lss_store::project::MANIFEST).is_file() {
            if !self.l6.logs.contains_key(&person) {
                let me = s.folder().me().clone();
                let r = Session::open_coaching(
                    &log_path,
                    &me,
                    (self.clock)(),
                    person,
                    d.program,
                    d.mbbs.clone(),
                    false,
                );
                if let Some(log) = self.report(r) {
                    self.l6.logs.insert(person, log);
                }
            }
            if let Some(log) = self.l6.logs.get_mut(&person) {
                let e = privacidade::anonymise_coaching(log.state(), person);
                let r = log.apply(e);
                self.report(r);
            }
            let r = program::redact_coaching(&log_path, person);
            self.report(r);
            if let Some(mut log) = self.l6.logs.remove(&person) {
                log.close();
            }
        }
        let r = program::redact_person(s.folder().root(), s.state(), person);
        if self.report(r).is_some() {
            let r = s.reload();
            self.report(r);
            self.set_status(tl.anonymised, false);
        }
    }
}

/// The actions shown in the matrix.
const ALL_ACTIONS: [Action; 19] = [
    Action::ManageProgram,
    Action::ManageRoles,
    Action::SetTargets,
    Action::ManagePeople,
    Action::SendToCommittee,
    Action::CommitteeAgenda,
    Action::CommitteeDecide,
    Action::Assign,
    Action::ReviewTollgate,
    Action::ConfirmCompetence,
    Action::RecordLearning,
    Action::RecordCoaching,
    Action::Anonymise,
    Action::ValidateSavings,
    Action::ApproveTollgate,
    Action::ViewMbbDashboard,
    Action::ViewLeadership,
    Action::ViewScoreboard,
    Action::ViewReviewQueue,
];
