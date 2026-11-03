//! The portfolio (section 1.1): the folder holding the project folders. Its own journal
//! (a portfolio folder next to the projects) keeps the bank of ideas, the screening, the
//! prioritisation and the selection; the projects next to it are summarised from their
//! own folders. The program screens above the portfolios are stage L6.

use std::path::{Path, PathBuf};

use eframe::egui::{self, RichText};
use utsim_lss_core::domain::{Approach, Person, ProjectStatus, RoleKind, SavingStatus, enum_value};
use utsim_lss_core::lessons::{self, Doc};
use utsim_lss_core::portfolio::{
    self, BOARD, Idea, ProjectSummary, Quadrant, Question, Scale, fields as pf,
};
use utsim_lss_core::savings::Category;
use utsim_lss_core::{EntityRef, Id, Kind};
use utsim_lss_store::project::{self, ProjectFolder};
use utsim_lss_store::user::Identity;

use super::super::{App, sanitize_folder_name};
use crate::session::Session;
use crate::text::l1::{l1, project_status_name, quadrant_name, question_text, scale_name};
use crate::text::{self, texts};
use crate::theme;

/// What the portfolio screen found next to the open folder.
#[derive(Default)]
pub struct Scan {
    pub parent: PathBuf,
    pub portfolio: Option<PathBuf>,
    pub projects: Vec<(PathBuf, ProjectSummary)>,
    pub docs: Vec<Doc>,
}

/// Looks at the folder holding `root`: the portfolio folder (if any), and the summary and
/// lessons of every project folder.
pub fn scan(root: &Path, me: &Identity, cache: &Path) -> Scan {
    let parent = root.parent().map(Path::to_path_buf).unwrap_or_default();
    let mut out = Scan {
        parent: parent.clone(),
        ..Scan::default()
    };
    let Ok(rd) = std::fs::read_dir(&parent) else {
        return out;
    };
    let mut dirs: Vec<PathBuf> = rd
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir() && project::is_project(p))
        .collect();
    dirs.sort();
    for d in dirs {
        let Ok(f) = ProjectFolder::open(&d, me, Some(cache)) else {
            continue;
        };
        if f.is_portfolio() {
            if out.portfolio.is_none() {
                out.portfolio = Some(d);
            }
            continue;
        }
        if let Some(sum) = portfolio::summary(f.state(), f.project_id()) {
            out.docs.extend(lessons::docs_of(f.state(), f.project_id()));
            out.projects.push((d, sum));
        }
    }
    out
}

#[derive(Default)]
pub struct PortfolioUi {
    scan: Option<Scan>,
    /// The portfolio folder, opened next to the project.
    session: Option<Session>,
    /// A project to open after this frame (switching projects mid-frame is not possible).
    pub(crate) open_request: Option<PathBuf>,
    name: String,
    unit: String,
    program: String,
    idea_title: String,
    idea: Option<Id>,
    criterion: String,
    objective: String,
}

impl PortfolioUi {
    pub fn close(&mut self) {
        if let Some(mut s) = self.session.take() {
            s.close();
        }
        self.scan = None;
    }

    pub fn rescan(&mut self) {
        self.scan = None;
    }

    pub fn ensure_scan(&mut self, root: &Path, me: &Identity, cache: &Path) {
        if self.scan.is_none() {
            self.scan = Some(scan(root, me, cache));
        }
    }

    /// Lessons of the projects next to the open one (the knowledge base).
    pub fn lesson_docs(&self) -> &[Doc] {
        self.scan.as_ref().map_or(&[], |s| &s.docs)
    }
}

impl App {
    pub(in crate::app) fn portfolio_l1(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = texts();
        let tl = l1();
        ui.heading(t.nav_portfolio);
        ui.weak(tl.portfolio_intro);
        let me = s.folder().me().clone();
        let cache = self.dirs.cache.clone();
        self.l1
            .portfolio
            .ensure_scan(s.folder().root(), &me, &cache);
        if ui.button(tl.refresh_portfolio).clicked() {
            self.l1.portfolio.rescan();
            self.l1
                .portfolio
                .ensure_scan(s.folder().root(), &me, &cache);
        }
        ui.add_space(6.0);
        if s.folder().is_portfolio() {
            self.portfolio_body(ui, s);
        } else {
            if self.l1.portfolio.session.is_none() {
                let path = self
                    .l1
                    .portfolio
                    .scan
                    .as_ref()
                    .and_then(|x| x.portfolio.clone());
                if let Some(p) = path {
                    let r = Session::open(&p, &me, Some(&cache), (self.clock)());
                    self.l1.portfolio.session = self.report(r);
                }
            }
            match self.l1.portfolio.session.take() {
                Some(mut ps) => {
                    let r = ps.tick("carteira", None);
                    self.report(r);
                    self.portfolio_body(ui, &mut ps);
                    self.l1.portfolio.session = Some(ps);
                }
                None => self.create_portfolio_form(ui, s, &me, &cache),
            }
        }
        ui.separator();
        self.projects_list(ui, s);
    }

    fn create_portfolio_form(
        &mut self,
        ui: &mut egui::Ui,
        s: &Session,
        me: &Identity,
        cache: &Path,
    ) {
        let tl = l1();
        theme::card(ui).show(ui, |ui| {
            ui.strong(tl.no_portfolio);
            ui.weak(tl.no_portfolio_tip);
            for (hint, buf) in [
                (tl.portfolio_name_hint, &mut self.l1.portfolio.name),
                (tl.unit_name_hint, &mut self.l1.portfolio.unit),
                (tl.program_name_hint, &mut self.l1.portfolio.program),
            ] {
                ui.add(
                    egui::TextEdit::singleline(buf)
                        .hint_text(hint)
                        .desired_width(360.0),
                );
            }
            if ui.button(tl.create_portfolio).clicked() {
                let p = &self.l1.portfolio;
                let name = if p.name.trim().is_empty() {
                    tl.default_portfolio_name.to_string()
                } else {
                    p.name.trim().to_string()
                };
                let unit = if p.unit.trim().is_empty() {
                    tl.default_unit_name.to_string()
                } else {
                    p.unit.trim().to_string()
                };
                let program = if p.program.trim().is_empty() {
                    tl.default_program_name.to_string()
                } else {
                    p.program.trim().to_string()
                };
                let parent = s
                    .folder()
                    .root()
                    .parent()
                    .map(Path::to_path_buf)
                    .unwrap_or_default();
                let path = parent.join(sanitize_folder_name(&name));
                let r = Session::create_portfolio(
                    &path,
                    (&name, &unit, &program),
                    me,
                    Some(cache),
                    (self.clock)(),
                );
                if let Some(mut ps) = self.report(r) {
                    let e = portfolio::default_criteria(ps.state(), ps.project(), ps.ids());
                    let r = ps.apply(e);
                    self.report(r);
                    self.l1.portfolio.session = Some(ps);
                    self.l1.portfolio.rescan();
                }
            }
        });
    }

    fn portfolio_body(&mut self, ui: &mut egui::Ui, ps: &mut Session) {
        let tl = l1();
        let t = texts();
        let c = ps.project();
        if let Some(p) = portfolio::read(ps.state(), c) {
            ui.label(
                RichText::new((tl.portfolio_header)(
                    &p.name,
                    &portfolio::name_of(ps.state(), p.unit),
                    &portfolio::name_of(ps.state(), p.program),
                ))
                .strong(),
            );
        }
        // Ideas board.
        ui.horizontal(|ui| {
            let r = ui.add(
                egui::TextEdit::singleline(&mut self.l1.portfolio.idea_title)
                    .hint_text(tl.new_idea_hint)
                    .desired_width(360.0),
            );
            let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if (ui.button(t.add).clicked() || enter)
                && !self.l1.portfolio.idea_title.trim().is_empty()
            {
                let id = ps.new_id();
                let e = portfolio::new_idea(c, id, &self.l1.portfolio.idea_title);
                if self.commit(ps, e) {
                    self.l1.portfolio.idea_title.clear();
                    self.l1.portfolio.idea = Some(id);
                }
            }
        });
        let ranking = portfolio::ranking(ps.state(), c);
        let score_of = |id: Id| {
            ranking
                .iter()
                .find(|(i, _)| i.id == id)
                .map(|(_, s)| s.total)
        };
        let by = portfolio::by_status(ps.state(), c);
        ui.horizontal_top(|ui| {
            for st in BOARD.iter().take(5) {
                let list = by
                    .get(&portfolio::status_order(*st))
                    .cloned()
                    .unwrap_or_default();
                ui.vertical(|ui| {
                    ui.set_width(170.0);
                    ui.strong(format!("{} ({})", project_status_name(*st), list.len()));
                    theme::card(ui).show(ui, |ui| {
                        ui.set_min_size(egui::vec2(154.0, 40.0));
                        for i in &list {
                            let mut label = if i.title.is_empty() {
                                t.untitled.to_string()
                            } else {
                                i.title.clone()
                            };
                            if let Some(sc) = score_of(i.id) {
                                label = format!("{label} · {:.0}", sc);
                            }
                            if ui
                                .selectable_label(self.l1.portfolio.idea == Some(i.id), label)
                                .clicked()
                            {
                                self.l1.portfolio.idea = Some(i.id);
                            }
                        }
                    });
                });
            }
        });
        let closed: usize = BOARD
            .iter()
            .skip(5)
            .map(|st| by.get(&portfolio::status_order(*st)).map_or(0, Vec::len))
            .sum();
        if closed > 0 {
            ui.weak((tl.closed_ideas)(closed));
        }
        for (b, n, over) in portfolio::belt_load(ps.state(), c) {
            if over {
                ui.colored_label(
                    theme::WARN,
                    (tl.belt_overload)(&utsim_lss_core::domain::person_name(ps.state(), b), n),
                );
            }
        }
        ui.separator();
        if self.l1.portfolio.idea.is_none() {
            self.l1.portfolio.idea = ranking.first().map(|(i, _)| i.id);
        }
        if let Some(id) = self.l1.portfolio.idea
            && let Some(e) = ps
                .state()
                .entity(id)
                .filter(|e| e.kind == Kind::Idea && !e.is_deleted())
        {
            let idea = Idea::read(id, e);
            self.idea_panel(ui, ps, &idea);
            ui.separator();
        }
        self.prioritisation(ui, ps, &ranking);
    }

    fn idea_panel(&mut self, ui: &mut egui::Ui, ps: &mut Session, idea: &Idea) {
        let tl = l1();
        let r = Idea::entity_ref(idea.id);
        let c = ps.project();
        self.presence_warning(ui, ps, r);
        egui::Grid::new("ideia")
            .num_columns(2)
            .spacing([12.0, 6.0])
            .show(ui, |ui| {
                for (label, f, multi) in [
                    (tl.i_title, pf::TITLE, false),
                    (tl.i_process, pf::PROCESS, false),
                    (tl.i_problem, pf::PROBLEM, true),
                    (tl.i_data, pf::DATA, true),
                ] {
                    ui.label(label);
                    self.text_field(ui, ps, r, f, multi, 420.0);
                    ui.end_row();
                }
                ui.label(tl.i_rough_gain);
                self.number_field(ui, ps, r, pf::ROUGH_GAIN, 120.0);
                ui.end_row();
                ui.label(tl.i_impact_effort);
                ui.horizontal(|ui| {
                    self.scale_field(ui, ps, r, pf::IMPACT, &[1, 2, 3, 4, 5]);
                    ui.label("×");
                    self.scale_field(ui, ps, r, pf::EFFORT, &[1, 2, 3, 4, 5]);
                    if let (Some(i), Some(e)) = (idea.impact, idea.effort) {
                        ui.weak(quadrant_name(portfolio::quadrant(i, e)));
                    }
                });
                ui.end_row();
                ui.label(tl.i_belt);
                self.person_field(ui, ps, r, pf::BELT);
                ui.end_row();
            });
        // Screening questions and the approach.
        ui.strong(tl.screening);
        for q in Question::ALL {
            self.bool_field(ui, ps, r, &q.field(), question_text(q));
        }
        let (suggested, why) = portfolio::suggest(&idea.answers);
        ui.label((tl.suggested_approach)(
            text::approach_name(suggested),
            why.map(question_text).unwrap_or(tl.default_reason),
        ));
        ui.horizontal(|ui| {
            ui.label(tl.chosen_approach);
            let mut v = idea.approach;
            egui::ComboBox::from_id_salt(("abordagem_ideia", idea.id))
                .selected_text(v.map_or(tl.to_choose, text::approach_name))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut v, None, tl.to_choose);
                    for a in Approach::ALL {
                        ui.selectable_value(&mut v, Some(a), text::approach_name(a));
                    }
                });
            if v != idea.approach {
                let value = v.map_or(serde_json::Value::Null, enum_value);
                let res = ps.set_field(r, pf::APPROACH, value, None);
                self.report(res);
            }
        });
        // Scores.
        let cs = portfolio::criteria(ps.state(), c);
        let os = portfolio::objectives(ps.state(), c);
        ui.strong(tl.scores);
        egui::Grid::new(("notas", idea.id))
            .num_columns(2)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                for cr in cs.iter().filter(|x| !x.from_alignment) {
                    ui.label(&cr.name);
                    let values: Vec<i64> =
                        cr.scale.values().iter().map(|v| i64::from(*v)).collect();
                    self.scale_field(ui, ps, r, &cr.score_field(), &values);
                    ui.end_row();
                }
                for o in &os {
                    ui.label((tl.alignment_with)(&o.name));
                    self.scale_field(ui, ps, r, &o.align_field(), &[0, 1, 3, 9]);
                    ui.end_row();
                }
            });
        let sc = portfolio::score(idea, &cs, &os);
        ui.label((tl.idea_score)(sc.total, sc.completeness * 100.0));
        // States.
        ui.horizontal_wrapped(|ui| {
            ui.label((tl.idea_state)(project_status_name(idea.status)));
            for st in BOARD {
                if portfolio::can_move(idea.status, st)
                    && ui
                        .button((tl.move_idea_to)(project_status_name(st)))
                        .clicked()
                {
                    let res = portfolio::move_idea(ps.state(), idea.id, st, "");
                    self.commit_rule(ps, res);
                }
            }
        });
        if idea.status == ProjectStatus::Approved
            && idea.project.is_none()
            && ui.button(tl.open_project_from_idea).clicked()
        {
            self.open_from_idea(ps, idea);
        }
        if let Some(pid) = idea.project {
            let folder = idea.folder.clone();
            ui.horizontal(|ui| {
                ui.weak((tl.idea_project)(&folder, &pid.short()));
                if ui.button(texts().open).clicked()
                    && let Some(scan) = &self.l1.portfolio.scan
                {
                    self.l1.portfolio.open_request = Some(scan.parent.join(&folder));
                }
            });
        }
    }

    /// Creates the project folder of an approved idea next to the portfolio (with the
    /// approach, the problem and process copied, and the belt with the green belt role) and
    /// records it in the idea.
    fn open_from_idea(&mut self, ps: &mut Session, idea: &Idea) {
        let tl = l1();
        let Some(approach) = idea.approach else {
            self.set_status(tl.choose_approach_first, true);
            return;
        };
        let parent = ps
            .folder()
            .root()
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_default();
        let title = if idea.title.trim().is_empty() {
            tl.default_project_title.to_string()
        } else {
            idea.title.trim().to_string()
        };
        let path = project::with_extension(&parent.join(sanitize_folder_name(&title)));
        let clock = (self.clock)();
        let me = ps.folder().me().clone();
        let cache = self.dirs.cache.clone();
        let r = ProjectFolder::create(&path, &title, approach, &me, clock.as_ref(), Some(&cache));
        let Some(mut folder) = self.report(r) else {
            return;
        };
        let pid = folder.project_id();
        let p = EntityRef::new(Kind::Project, pid);
        use utsim_lss_core::domain::fields::project as f;
        let mut edits = vec![
            utsim_lss_core::Edit::new(p, f::PROCESS, idea.process.as_str()),
            utsim_lss_core::Edit::new(p, f::PROBLEM, idea.problem.as_str()),
        ];
        if let Some(b) = idea.belt {
            let name = utsim_lss_core::domain::person_name(ps.state(), b);
            let email = ps
                .state()
                .entity(b)
                .map(|e| e.text(utsim_lss_core::domain::fields::person::EMAIL))
                .unwrap_or_default();
            edits.extend(Person::register(folder.state(), b, &name, &email));
            edits.extend(utsim_lss_core::roles::add_role(
                folder.state(),
                pid,
                clock.new_id(),
                b,
                RoleKind::GreenBelt,
            ));
        }
        let r = folder.commit(edits, clock.as_ref());
        self.report(r);
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let e = portfolio::opened(idea.id, pid, &name);
        if self.commit(ps, e) {
            self.set_status(
                (tl.project_opened_from_idea)(&path.display().to_string()),
                false,
            );
            self.l1.portfolio.rescan();
        }
    }

    fn prioritisation(
        &mut self,
        ui: &mut egui::Ui,
        ps: &mut Session,
        ranking: &[(Idea, portfolio::Score)],
    ) {
        let tl = l1();
        let t = texts();
        let c = ps.project();
        ui.heading(tl.prioritisation);
        ui.horizontal_top(|ui| {
            // Impact × effort.
            ui.vertical(|ui| {
                ui.strong(tl.matrix_title);
                egui::Grid::new("matriz_ie")
                    .num_columns(2)
                    .spacing([4.0, 4.0])
                    .show(ui, |ui| {
                        for row in [
                            [Quadrant::QuickWin, Quadrant::Strategic],
                            [Quadrant::Minor, Quadrant::Avoid],
                        ] {
                            for q in row {
                                theme::card(ui).show(ui, |ui| {
                                    ui.set_min_size(egui::vec2(150.0, 60.0));
                                    ui.strong(quadrant_name(q));
                                    for (i, _) in ranking {
                                        if let (Some(a), Some(b)) = (i.impact, i.effort)
                                            && portfolio::quadrant(a, b) == q
                                        {
                                            ui.small(&i.title);
                                        }
                                    }
                                });
                            }
                            ui.end_row();
                        }
                    });
                ui.weak(tl.matrix_tip);
            });
            ui.add_space(12.0);
            ui.vertical(|ui| {
                ui.strong(tl.ranking);
                egui::Grid::new("ranking")
                    .num_columns(4)
                    .striped(true)
                    .spacing([10.0, 3.0])
                    .show(ui, |ui| {
                        for (k, (i, s)) in ranking.iter().enumerate() {
                            ui.label(format!("{}.", k + 1));
                            ui.label(&i.title);
                            ui.label(format!("{:.0}", s.total));
                            ui.weak((tl.completeness)(s.completeness * 100.0));
                            ui.end_row();
                        }
                    });
            });
        });
        ui.collapsing(tl.criteria_and_objectives, |ui| {
            let cs = portfolio::criteria(ps.state(), c);
            egui::Grid::new("criterios")
                .num_columns(5)
                .striped(true)
                .spacing([8.0, 4.0])
                .show(ui, |ui| {
                    for h in [tl.criterion, tl.weight, tl.scale, tl.inverse, ""] {
                        ui.strong(h);
                    }
                    ui.end_row();
                    for cr in &cs {
                        let r = EntityRef::new(Kind::Criterion, cr.id);
                        self.text_field(ui, ps, r, pf::NAME, false, 180.0);
                        self.number_field(ui, ps, r, pf::WEIGHT, 50.0);
                        self.enum_field(
                            ui,
                            ps,
                            r,
                            pf::SCALE,
                            &[Scale::OneToFive, Scale::OneThreeNine],
                            scale_name,
                        );
                        self.bool_field(ui, ps, r, pf::INVERSE, "");
                        if cr.from_alignment {
                            ui.weak(tl.from_alignment);
                        } else {
                            ui.label("");
                        }
                        ui.end_row();
                    }
                });
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.l1.portfolio.criterion)
                        .hint_text(tl.new_criterion_hint)
                        .desired_width(220.0),
                );
                if ui.button(t.add).clicked() && !self.l1.portfolio.criterion.trim().is_empty() {
                    let order = cs.iter().map(|x| x.order).fold(0.0, f64::max) + 1.0;
                    let e = portfolio::add_criterion(
                        c,
                        ps.new_id(),
                        &self.l1.portfolio.criterion,
                        1.0,
                        Scale::OneToFive,
                        false,
                        false,
                        order,
                    );
                    if self.commit(ps, e) {
                        self.l1.portfolio.criterion.clear();
                    }
                }
                if cs.is_empty() && ui.button(tl.default_criteria).clicked() {
                    let e = portfolio::default_criteria(ps.state(), c, ps.ids());
                    self.commit(ps, e);
                }
            });
            ui.add_space(6.0);
            ui.strong(tl.objectives);
            for o in portfolio::objectives(ps.state(), c) {
                let r = EntityRef::new(Kind::StrategicObjective, o.id);
                ui.horizontal(|ui| {
                    self.text_field(ui, ps, r, pf::NAME, false, 260.0);
                    ui.label(tl.weight);
                    self.number_field(ui, ps, r, pf::WEIGHT, 50.0);
                });
            }
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.l1.portfolio.objective)
                        .hint_text(tl.new_objective_hint)
                        .desired_width(260.0),
                );
                if ui.button(t.add).clicked() && !self.l1.portfolio.objective.trim().is_empty() {
                    let e =
                        portfolio::add_objective(c, ps.new_id(), &self.l1.portfolio.objective, 1.0);
                    if self.commit(ps, e) {
                        self.l1.portfolio.objective.clear();
                    }
                }
            });
        });
    }

    fn projects_list(&mut self, ui: &mut egui::Ui, s: &Session) {
        let tl = l1();
        ui.strong(tl.projects_here);
        let Some(scan) = &self.l1.portfolio.scan else {
            return;
        };
        if scan.projects.is_empty() {
            ui.weak(tl.no_projects_here);
            return;
        }
        let mut open = None;
        egui::Grid::new("projetos_carteira")
            .num_columns(6)
            .striped(true)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                for (path, p) in &scan.projects {
                    ui.label(&p.title);
                    ui.label(text::approach_name(p.approach));
                    ui.label(project_status_name(p.status));
                    ui.label(&p.phase);
                    let validated = p.savings.get(Category::Hard, SavingStatus::Validated)
                        + p.savings.get(Category::Soft, SavingStatus::Validated);
                    ui.label(text::money(validated))
                        .on_hover_text(tl.validated_savings);
                    let current = s.folder().root() == path.as_path();
                    if ui
                        .add_enabled(!current, egui::Button::new(texts().open))
                        .clicked()
                    {
                        open = Some(path.clone());
                    }
                    ui.end_row();
                }
            });
        if let Some(p) = open {
            self.l1.portfolio.open_request = Some(p);
        }
    }
}
