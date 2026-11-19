//! Screens of stage L6, the implementation program (`docs/plano_lean_seis_sigma.md`, 8.1
//! to 8.4, 8.7 and 8.8): structure, targets and objectives; scoreboard; pipeline and
//! selection committee; capacity and assignment; coaching and competence map; tollgate
//! reviews; the MBB and leadership dashboards; profiles and personal data.
//!
//! The program folder is a [`Session`] with the program's guard: every write is checked
//! against the profile of the person (a refusal shows its reason in the status bar) and
//! the state shown is built only from authorized operations. The portfolios and projects
//! around the program are read through [`program::scan`], and their summaries feed the
//! pure functions of `utsim-lss-core` (`programa`, `designacao`, `mentoria`, `revisao`,
//! `painel`, `perfis`, `privacidade`).

mod capacity;
mod certification;
mod classroom;
mod coaching;
pub mod dashboards;
mod exams;
pub mod l7;
mod mytraining;
mod pipeline;
mod profiles;
mod reviews;
mod structure;
mod training;

use crate::text::LocalDate;
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::time::Duration;

use eframe::egui::{self, RichText};
use utsim_lss_core::designacao::{self, Absence, Assignment, Limits, Load};
use utsim_lss_core::mentoria::{self, CompetenceMap};
use utsim_lss_core::perfis::{self, Action, Profile};
use utsim_lss_core::programa::{self, IdeaRecord, Member, Opportunity, ProjectRecord, Unit};
use utsim_lss_core::revisao::{self, Review};
use utsim_lss_core::{Date, Edit, Id};
use utsim_lss_store::program::{self, Scan};
use utsim_lss_store::project::ProjectFolder;

use super::super::App;
use crate::session::Session;
use crate::text::l6::l6;
use crate::text::texts;
use crate::theme;

/// The screens of a program folder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgramNav {
    #[default]
    Structure,
    Scoreboard,
    Pipeline,
    Capacity,
    Coaching,
    Reviews,
    MbbDashboard,
    LeadershipDashboard,
    Profiles,
    // Stage L7.
    Training,
    Classroom,
    Exams,
    Certification,
    MyTraining,
}

impl ProgramNav {
    pub const ALL: [ProgramNav; 14] = [
        ProgramNav::Structure,
        ProgramNav::Scoreboard,
        ProgramNav::Pipeline,
        ProgramNav::Capacity,
        ProgramNav::Coaching,
        ProgramNav::Reviews,
        ProgramNav::MbbDashboard,
        ProgramNav::LeadershipDashboard,
        ProgramNav::Profiles,
        ProgramNav::Training,
        ProgramNav::Classroom,
        ProgramNav::Exams,
        ProgramNav::Certification,
        ProgramNav::MyTraining,
    ];

    /// Stable key written in the presence file.
    pub fn key(self) -> &'static str {
        match self {
            ProgramNav::Structure => "programa",
            ProgramNav::Scoreboard => "placar",
            ProgramNav::Pipeline => "funil",
            ProgramNav::Capacity => "capacidade",
            ProgramNav::Coaching => "mentoria",
            ProgramNav::Reviews => "revisoes",
            ProgramNav::MbbDashboard => "painel_mbb",
            ProgramNav::LeadershipDashboard => "painel_lideranca",
            ProgramNav::Profiles => "perfis",
            ProgramNav::Training => "formacao",
            ProgramNav::Classroom => "turmas",
            ProgramNav::Exams => "provas",
            ProgramNav::Certification => "certificacao",
            ProgramNav::MyTraining => "minha_formacao",
        }
    }

    pub fn label(self) -> &'static str {
        let t = l6();
        match self {
            ProgramNav::Structure => t.nav_structure,
            ProgramNav::Scoreboard => t.nav_scoreboard,
            ProgramNav::Pipeline => t.nav_pipeline,
            ProgramNav::Capacity => t.nav_capacity,
            ProgramNav::Coaching => t.nav_coaching,
            ProgramNav::Reviews => t.nav_reviews,
            ProgramNav::MbbDashboard => t.nav_mbb,
            ProgramNav::LeadershipDashboard => t.nav_leadership,
            ProgramNav::Profiles => t.nav_profiles,
            ProgramNav::Training => crate::text::l7::l7().nav_training,
            ProgramNav::Classroom => crate::text::l7::l7().nav_classroom,
            ProgramNav::Exams => crate::text::l7::l7().nav_exams,
            ProgramNav::Certification => crate::text::l7::l7().nav_certification,
            ProgramNav::MyTraining => crate::text::l7::l7().nav_my_training,
        }
    }
}

/// How often the folders around the program are read again, milliseconds.
pub const RESCAN_MS: i64 = 30_000;

/// What the program screens keep between frames.
#[derive(Default)]
pub struct L6Ui {
    scan: Option<Scan>,
    scanned_ms: i64,
    /// The coaching logs opened, by belt.
    logs: BTreeMap<Id, Session>,
    /// Text typed in the boxes of these screens, by box.
    inputs: HashMap<String, String>,
    pub(crate) year: Option<i32>,
    pub(crate) unit_filter: Option<Id>,
    // Structure.
    target_unit: Option<Id>,
    target_metric: programa::Metric,
    target_level: Option<programa::BeltLevel>,
    // Pipeline.
    committee: Option<Id>,
    opportunity: Option<Id>,
    choice: Option<programa::Choice>,
    decision_belt: Option<Id>,
    // Capacity.
    assign_idea: Option<Id>,
    absence_person: Option<Id>,
    // Coaching.
    pub(crate) belt: Option<Id>,
    session_form: mentoria::Form,
    session_project: Option<Id>,
    selected_session: Option<Id>,
    commitment_target: mentoria::Target,
    confirm_tool: String,
    // Reviews.
    pub(crate) review_item: Option<Id>,
    group_by_unit: bool,
    marks: BTreeMap<revisao::Criterion, (revisao::Mark, String)>,
    answers: BTreeMap<usize, bool>,
    // Profiles.
    grant_person: Option<Id>,
    grant_profile: Profile,
    grant_unit: Option<Id>,
    /// The training screens (stage L7).
    pub(crate) l7: l7::L7Ui,
    /// The report of the last import of people (P9).
    pub(crate) people_report: Option<utsim_lss_core::pessoas::Report>,
    /// An import asked from outside the screen (tests and tools), done on the next draw.
    pub(crate) people_pending: Option<std::path::PathBuf>,
    /// A dashboard export asked: leadership (or MBB), format, open the file.
    pub(crate) export: Option<(bool, dashboards::Export, bool)>,
    /// The file of the last dashboard export.
    pub(crate) exported: Option<std::path::PathBuf>,
}

impl L6Ui {
    pub fn close(&mut self) {
        for (_, mut s) in std::mem::take(&mut self.logs) {
            s.close();
        }
        self.l7.close();
        *self = L6Ui::default();
    }

    fn input(&mut self, key: &str) -> &mut String {
        self.inputs.entry(key.to_string()).or_default()
    }

    fn take_input(&mut self, key: &str) -> String {
        self.inputs.remove(key).unwrap_or_default()
    }

    pub fn rescan(&mut self) {
        self.scan = None;
    }
}

/// Everything the screens read, computed once per frame from the program state and the
/// scan (owned, so the screens may write while they use it).
pub struct Data {
    pub program: Id,
    pub founder: Id,
    pub me: Id,
    pub today: Date,
    pub scanned_ms: i64,
    pub scan: Scan,
    pub records: Vec<ProjectRecord>,
    pub ideas: Vec<IdeaRecord>,
    pub units: Vec<Unit>,
    pub members: Vec<Member>,
    pub opps: Vec<Opportunity>,
    pub reviews: Vec<Review>,
    pub assignments: Vec<Assignment>,
    pub absences: Vec<Absence>,
    pub limits: Limits,
    pub loads: Vec<Load>,
    pub entries: Vec<mentoria::Entry>,
    pub map: CompetenceMap,
    pub mbbs: Vec<Id>,
    pub my_profiles: Vec<(Profile, Option<Id>)>,
    /// The training (stage L7).
    pub tr: l7::Training,
}

impl Data {
    pub fn new(s: &Session, scan: &Scan, scanned_ms: i64) -> Data {
        let st = s.state();
        let program = s.project();
        let founder = s.founder().unwrap_or(Id::NIL);
        let today = s.today();
        let records = scan.records();
        let ideas = scan.ideas.clone();
        let members = programa::members(st);
        let assignments = designacao::assignments(st);
        let absences = designacao::absences(st);
        let limits = Limits::read(st, program);
        let loads = designacao::loads(
            &members,
            &records,
            &ideas,
            &assignments,
            &absences,
            &limits,
            today,
        );
        let mbbs = perfis::mbbs(st, founder);
        let tr = l7::Training::read(st, program);
        // The training feeds `estudou` and `praticou` (recomputed, never written).
        let mut entries = mentoria::entries(st);
        entries.extend(utsim_lss_core::formacao::learning_entries(
            &tr.sessions,
            &tr.modules,
        ));
        let map = mentoria::competence_map(&entries, &records, |p| mbbs.contains(&p));
        Data {
            program,
            founder,
            me: s.me(),
            today,
            scanned_ms,
            scan: scan.clone(),
            units: programa::units(st, program),
            opps: programa::opportunities(st, program),
            reviews: revisao::reviews(st),
            my_profiles: perfis::profiles(st, founder, s.me()),
            records,
            ideas,
            members,
            assignments,
            absences,
            limits,
            loads,
            entries,
            map,
            mbbs,
            tr,
        }
    }

    pub fn is_mbb(&self) -> bool {
        self.mbbs.contains(&self.me)
    }

    pub fn unit_name(&self, u: Option<Id>) -> String {
        u.and_then(|u| self.units.iter().find(|x| x.id == u))
            .map_or_else(|| l6().no_unit.to_string(), |u| u.name.clone())
    }

    pub fn person(&self, p: Id) -> String {
        self.members
            .iter()
            .find(|m| m.id == p)
            .map_or_else(|| p.short(), |m| m.name.clone())
    }

    pub fn year(&self, ui: &L6Ui) -> i32 {
        ui.year.unwrap_or(self.today.year)
    }

    pub fn level_of(&self, p: Id) -> Option<programa::BeltLevel> {
        self.members
            .iter()
            .find(|m| m.id == p)
            .and_then(|m| m.level)
    }
}

/// `DD/MM/AAAA HH:MM` of an instant, for "dados de".
pub fn instant(ms: i64, offset_minutes: i32) -> String {
    let d = Date::from_unix_ms(ms, offset_minutes);
    let local = ms + i64::from(offset_minutes) * 60_000;
    let mins = local.div_euclid(60_000).rem_euclid(24 * 60);
    format!("{} {:02}:{:02}", d.local(), mins / 60, mins % 60)
}

impl App {
    /// Selects what the program screens show in detail: a belt (coaching), a tollgate of
    /// the review queue, an idea to assign and an opportunity of the committee.
    pub fn select_in_program(
        &mut self,
        belt: Option<Id>,
        review: Option<Id>,
        idea: Option<Id>,
        opportunity: Option<Id>,
    ) {
        self.l6.belt = belt;
        self.l6.review_item = review;
        self.l6.assign_idea = idea;
        self.l6.opportunity = opportunity;
    }

    /// Selects the class of the training screens and the person of the certification
    /// check (stage L7).
    pub fn select_in_training(
        &mut self,
        class: Option<Id>,
        person: Option<Id>,
        level: programa::BeltLevel,
    ) {
        self.l6.l7.class = class;
        self.l6.l7.cert_person = person;
        self.l6.l7.cert_level = level;
    }

    /// Opens the exam mode of "Minha formação" on an exam.
    pub fn start_exam(&mut self, exam: Id) {
        self.l6.l7.exam_mode_on = Some(exam);
        self.l6.l7.answers.clear();
    }

    /// The window of an open program folder.
    pub(in crate::app) fn program_screen(&mut self, ui: &mut egui::Ui, mut s: Session) {
        let t = texts();
        let tl = l6();
        let mut close = false;
        // The folders around the program, read again every `RESCAN_MS`.
        let now = s.now_ms();
        if self.l6.scan.is_none() || now - self.l6.scanned_ms >= RESCAN_MS {
            let me = s.folder().me().clone();
            let cache = self.dirs.cache.clone();
            self.l6.scan = Some(program::scan(
                s.folder().root(),
                s.state(),
                s.project(),
                &me,
                Some(&cache),
                s.clock().utc_offset_minutes(),
            ));
            self.l6.scanned_ms = now;
        }
        let scan = self.l6.scan.clone().unwrap_or_default();
        let d = Data::new(&s, &scan, self.l6.scanned_ms);
        egui::Panel::top("topo").show(ui, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.heading(s.folder().title());
                ui.separator();
                let mine: Vec<&str> = d
                    .my_profiles
                    .iter()
                    .map(|(p, _)| profile_name(*p))
                    .collect();
                ui.label(if mine.is_empty() {
                    tl.no_profile.to_string()
                } else {
                    (tl.my_profiles)(&mine.join(", "))
                });
                for p in s.others() {
                    ui.label(RichText::new(format!("● {}", p.name)).color(theme::accent(ui)));
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(t.close_project).clicked() {
                        close = true;
                    }
                    self.theme_picker(ui);
                });
            });
            let n = s.unauthorized().len();
            if n > 0 {
                ui.colored_label(theme::WARN, (tl.unauthorized_banner)(n));
            }
            ui.add_space(4.0);
        });
        egui::Panel::bottom("estado").show(ui, |ui| self.status_bar(ui));
        egui::Panel::left("navegacao")
            .default_size(200.0)
            .resizable(false)
            .show(ui, |ui| {
                ui.add_space(6.0);
                ui.strong(tl.nav_program);
                for n in ProgramNav::ALL {
                    if ui
                        .selectable_label(self.pnav == n, format!("  {}", n.label()))
                        .clicked()
                    {
                        self.pnav = n;
                    }
                }
                ui.add_space(8.0);
                if ui.button(tl.rescan).clicked() {
                    self.l6.rescan();
                }
                ui.weak((tl.data_of)(&instant(
                    d.scanned_ms,
                    s.clock().utc_offset_minutes(),
                )));
                ui.weak((tl.found)(d.scan.portfolios.len(), d.records.len()));
            });
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::both().show(ui, |ui| match self.pnav {
                ProgramNav::Structure => self.program_structure(ui, &mut s, &d),
                ProgramNav::Scoreboard => self.program_scoreboard(ui, &s, &d),
                ProgramNav::Pipeline => self.program_pipeline(ui, &mut s, &d),
                ProgramNav::Capacity => self.program_capacity(ui, &mut s, &d),
                ProgramNav::Coaching => self.program_coaching(ui, &mut s, &d),
                ProgramNav::Reviews => self.program_reviews(ui, &mut s, &d),
                ProgramNav::MbbDashboard => self.program_mbb(ui, &mut s, &d),
                ProgramNav::LeadershipDashboard => self.program_leadership(ui, &s, &d),
                ProgramNav::Profiles => self.program_profiles(ui, &mut s, &d),
                ProgramNav::Training => self.program_training(ui, &mut s, &d),
                ProgramNav::Classroom => self.program_classroom(ui, &mut s, &d),
                ProgramNav::Exams => self.program_exams(ui, &mut s, &d),
                ProgramNav::Certification => self.program_certification(ui, &mut s, &d),
                ProgramNav::MyTraining => self.program_my_training(ui, &mut s, &d),
            });
        });
        let editing = self.editing.take();
        let r = s.tick(self.pnav.key(), editing);
        self.report(r);
        ui.ctx().request_repaint_after(Duration::from_millis(500));
        if close {
            s.close();
            self.l6.close();
            self.screen = super::super::Screen::Start;
            self.drafts.clear();
        } else {
            self.session = Some(s);
        }
    }

    /// Shows the refusal of a view (no data drawn) and returns false.
    fn may_view(&mut self, ui: &mut egui::Ui, s: &Session, action: Action) -> bool {
        match s.may(action, None) {
            Ok(()) => true,
            Err(e) => {
                ui.colored_label(theme::WARN, (l6().view_denied)(&e.to_string()));
                false
            }
        }
    }

    /// Checks an action before doing it; the refusal goes to the status bar.
    fn may_do(&mut self, s: &Session, action: Action, unit: Option<Id>) -> bool {
        match s.may(action, unit) {
            Ok(()) => true,
            Err(e) => {
                self.set_status((texts().error)(&e.to_string()), true);
                false
            }
        }
    }

    /// Writes edits into another folder (a portfolio or a project next to the program).
    fn write_folder(&mut self, s: &Session, path: &Path, edits: Vec<Edit>) -> bool {
        if edits.is_empty() {
            return true;
        }
        let me = s.folder().me().clone();
        let clock = (self.clock)();
        let r = ProjectFolder::open(path, &me, Some(&self.dirs.cache))
            .and_then(|mut f| f.commit(edits, clock.as_ref()).map(|_| ()));
        let ok = self.report(r).is_some();
        if ok {
            self.l6.rescan();
        }
        ok
    }

    /// Reads another folder's state (to prepare edits for it).
    fn folder_state(&mut self, s: &Session, path: &Path) -> Option<utsim_lss_core::State> {
        let me = s.folder().me().clone();
        let r = ProjectFolder::open(path, &me, Some(&self.dirs.cache)).map(|f| f.state().clone());
        self.report(r)
    }

    /// A combo over the program units (or none); the new choice when it changed.
    fn unit_combo(
        ui: &mut egui::Ui,
        d: &Data,
        salt: impl std::hash::Hash + std::fmt::Debug,
        current: Option<Id>,
        none: &str,
    ) -> Option<Option<Id>> {
        let mut v = current;
        egui::ComboBox::from_id_salt(salt)
            .selected_text(v.map_or_else(|| none.to_string(), |u| d.unit_name(Some(u))))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut v, None, none);
                for u in &d.units {
                    ui.selectable_value(&mut v, Some(u.id), &u.name);
                }
            });
        (v != current).then_some(v)
    }

    /// A combo over the program's people (or nobody).
    fn member_combo(
        ui: &mut egui::Ui,
        d: &Data,
        salt: impl std::hash::Hash + std::fmt::Debug,
        current: Option<Id>,
        only_belts: bool,
    ) -> Option<Option<Id>> {
        let mut v = current;
        egui::ComboBox::from_id_salt(salt)
            .selected_text(v.map_or_else(|| texts().nobody.to_string(), |p| d.person(p)))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut v, None, texts().nobody);
                for m in d.members.iter().filter(|m| !only_belts || m.is_belt()) {
                    ui.selectable_value(&mut v, Some(m.id), &m.name);
                }
            });
        (v != current).then_some(v)
    }

    /// The year filter and the unit filter of the dashboards.
    fn filters(&mut self, ui: &mut egui::Ui, d: &Data) {
        let tl = l6();
        ui.horizontal(|ui| {
            ui.label(tl.year);
            let mut y = d.year(&self.l6);
            if ui
                .add(egui::DragValue::new(&mut y).range(2000..=2100))
                .changed()
            {
                self.l6.year = Some(y);
            }
            ui.label(tl.unit);
            if let Some(u) =
                Self::unit_combo(ui, d, "filtro_unidade", self.l6.unit_filter, tl.all_units)
            {
                self.l6.unit_filter = u;
            }
        });
    }
}

/// Name of a profile on the screen.
pub fn profile_name(p: Profile) -> &'static str {
    crate::text::l6::profile_name(p)
}
