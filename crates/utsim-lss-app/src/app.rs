//! The window: first run, start screen and the project screens. Drawing only: the numbers
//! come from `utsim-lss-core` and every change goes through [`Session`].

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use eframe::egui::{self, RichText};
use serde_json::Value;
use utsim_lss_core::domain::{self, Action, Approach, Column, Project, fields};
use utsim_lss_core::overview::Overview;
use utsim_lss_core::{Date, EntityRef, Id as Uid, Kind, kanban};
use utsim_lss_store::user::{self, AppDirs, Identity, UserConfig};
use utsim_lss_store::{Clock, SystemClock, lssz, project};

use crate::session::Session;
use crate::text::{self, texts};
use crate::theme;

/// Screens of an open project (the left navigation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nav {
    Portfolio,
    Overview,
    Charter,
    Phases,
    Actions,
    Risks,
    Meetings,
    Schedule,
    Savings,
    Tools,
    Data,
    Conflicts,
}

impl Nav {
    pub const PROJECT: [Nav; 10] = [
        Nav::Overview,
        Nav::Charter,
        Nav::Phases,
        Nav::Actions,
        Nav::Risks,
        Nav::Meetings,
        Nav::Schedule,
        Nav::Savings,
        Nav::Tools,
        Nav::Data,
    ];

    /// Stable key written in the presence file (not translated).
    pub fn key(self) -> &'static str {
        match self {
            Nav::Portfolio => "carteira",
            Nav::Overview => "visao_geral",
            Nav::Charter => "termo",
            Nav::Phases => "fases",
            Nav::Actions => "acoes",
            Nav::Risks => "riscos",
            Nav::Meetings => "reunioes",
            Nav::Schedule => "cronograma",
            Nav::Savings => "ganhos",
            Nav::Tools => "ferramentas",
            Nav::Data => "dados",
            Nav::Conflicts => "conflitos",
        }
    }

    pub fn from_key(key: &str) -> Option<Nav> {
        [Nav::Portfolio, Nav::Conflicts]
            .into_iter()
            .chain(Nav::PROJECT)
            .find(|n| n.key() == key)
    }

    pub fn label(self) -> &'static str {
        let t = texts();
        match self {
            Nav::Portfolio => t.nav_portfolio,
            Nav::Overview => t.nav_overview,
            Nav::Charter => t.nav_charter,
            Nav::Phases => t.nav_phases,
            Nav::Actions => t.nav_actions,
            Nav::Risks => t.nav_risks,
            Nav::Meetings => t.nav_meetings,
            Nav::Schedule => t.nav_schedule,
            Nav::Savings => t.nav_savings,
            Nav::Tools => t.nav_tools,
            Nav::Data => t.nav_data,
            Nav::Conflicts => t.nav_conflicts,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Identity,
    Start,
    Project,
}

/// Text being typed in a field: kept until the field loses the focus, with the version
/// the person saw when they started.
struct Draft {
    text: String,
    seen: Option<Uid>,
}

type ClockFactory = Box<dyn Fn() -> Box<dyn Clock>>;

pub struct App {
    dirs: AppDirs,
    config: UserConfig,
    clock: ClockFactory,
    pub screen: Screen,
    pub nav: Nav,
    session: Option<Session>,
    name_input: String,
    email_input: String,
    identity_error: Option<String>,
    open_input: String,
    new_title: String,
    new_parent: String,
    new_approach: Approach,
    import_file: String,
    import_dest: String,
    export_dest: String,
    new_card: String,
    selected_card: Option<Uid>,
    drafts: HashMap<(Uid, String), Draft>,
    editing: Option<EntityRef>,
    status: Option<(String, bool)>,
    theme_applied: Option<user::Theme>,
}

impl App {
    pub fn new(dirs: AppDirs) -> Self {
        Self::with_clock(dirs, Box::new(|| Box::new(SystemClock)))
    }

    pub fn with_clock(dirs: AppDirs, clock: ClockFactory) -> Self {
        let config = UserConfig::load(&dirs.config_file);
        let screen = if config.identity.is_some() {
            Screen::Start
        } else {
            Screen::Identity
        };
        Self {
            dirs,
            config,
            clock,
            screen,
            nav: Nav::Overview,
            session: None,
            name_input: String::new(),
            email_input: String::new(),
            identity_error: None,
            open_input: String::new(),
            new_title: String::new(),
            new_parent: String::new(),
            new_approach: Approach::Dmaic,
            import_file: String::new(),
            import_dest: String::new(),
            export_dest: String::new(),
            new_card: String::new(),
            selected_card: None,
            drafts: HashMap::new(),
            editing: None,
            status: None,
            theme_applied: None,
        }
    }

    pub fn session(&self) -> Option<&Session> {
        self.session.as_ref()
    }

    pub fn status(&self) -> Option<&(String, bool)> {
        self.status.as_ref()
    }

    fn set_status(&mut self, msg: impl Into<String>, error: bool) {
        self.status = Some((msg.into(), error));
    }

    fn report<T>(&mut self, r: utsim_lss_store::Result<T>) -> Option<T> {
        match r {
            Ok(v) => Some(v),
            Err(e) => {
                self.set_status((texts().error)(&e.to_string()), true);
                None
            }
        }
    }

    fn save_config(&mut self) {
        if let Err(e) = self.config.save(&self.dirs.config_file) {
            self.set_status((texts().error)(&e.to_string()), true);
        }
    }

    /// Saves the identity typed on the first run.
    pub fn submit_identity(&mut self, name: &str, email: &str) -> bool {
        if let Err(e) = Identity::validate(name, email) {
            self.identity_error = Some(e);
            return false;
        }
        let person = match &self.config.identity {
            Some(old) => old.person,
            None => (self.clock)().new_id(),
        };
        self.config.identity = Some(Identity {
            person,
            name: name.trim().to_string(),
            email: email.trim().to_string(),
        });
        self.identity_error = None;
        self.save_config();
        self.screen = Screen::Start;
        true
    }

    fn identity(&self) -> Option<Identity> {
        self.config.identity.clone()
    }

    fn entered(&mut self, session: Session) {
        let root = session.folder().root().to_path_buf();
        self.config.touch_recent(&root);
        self.save_config();
        self.export_dest = session.default_export_path().display().to_string();
        self.session = Some(session);
        self.screen = Screen::Project;
        self.nav = Nav::Overview;
        self.selected_card = None;
        self.drafts.clear();
    }

    pub fn open_project(&mut self, path: &Path) -> bool {
        let Some(me) = self.identity() else {
            return false;
        };
        let cache = self.dirs.cache.clone();
        let r = Session::open(path, &me, Some(&cache), (self.clock)());
        match self.report(r) {
            Some(s) => {
                self.status = None;
                self.entered(s);
                true
            }
            None => false,
        }
    }

    pub fn create_project(&mut self, parent: &Path, title: &str, approach: Approach) -> bool {
        let Some(me) = self.identity() else {
            return false;
        };
        let cache = self.dirs.cache.clone();
        let path = parent.join(sanitize_folder_name(title));
        let r = Session::create(&path, title, approach, &me, Some(&cache), (self.clock)());
        match self.report(r) {
            Some(s) => {
                let root = s.folder().root().display().to_string();
                self.entered(s);
                self.set_status((texts().created)(&root), false);
                true
            }
            None => false,
        }
    }

    pub fn close_project(&mut self) {
        if let Some(mut s) = self.session.take() {
            s.close();
        }
        self.screen = Screen::Start;
        self.drafts.clear();
        self.editing = None;
    }

    /// The whole window.
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        if self.theme_applied != Some(self.config.theme) {
            theme::apply(ui.ctx(), self.config.theme);
            self.theme_applied = Some(self.config.theme);
        }
        match self.screen {
            Screen::Identity => {
                egui::CentralPanel::default().show(ui, |ui| self.identity_screen(ui));
            }
            Screen::Start => self.start_screen(ui),
            Screen::Project => self.project_screen(ui),
        }
    }

    fn identity_screen(&mut self, ui: &mut egui::Ui) {
        let t = texts();
        ui.vertical_centered(|ui| {
            ui.add_space(48.0);
            ui.heading(t.identity_title);
            ui.add_space(8.0);
            ui.set_max_width(520.0);
            ui.label(t.identity_intro);
            ui.add_space(16.0);
        });
        let mut submit = false;
        ui.vertical_centered(|ui| {
            egui::Grid::new("identidade")
                .num_columns(2)
                .spacing([12.0, 8.0])
                .show(ui, |ui| {
                    ui.label(t.identity_name);
                    ui.add(egui::TextEdit::singleline(&mut self.name_input).desired_width(300.0));
                    ui.end_row();
                    ui.label(t.identity_email);
                    let r = ui.add(
                        egui::TextEdit::singleline(&mut self.email_input).desired_width(300.0),
                    );
                    submit |= r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                    ui.end_row();
                });
            ui.add_space(8.0);
            submit |= ui.button(t.continue_).clicked();
            if let Some(e) = &self.identity_error {
                ui.colored_label(ui.visuals().error_fg_color, e);
            }
            ui.add_space(12.0);
            ui.small(t.identity_note);
        });
        if submit {
            let (n, e) = (self.name_input.clone(), self.email_input.clone());
            self.submit_identity(&n, &e);
        }
    }

    fn start_screen(&mut self, ui: &mut egui::Ui) {
        let t = texts();
        egui::Panel::top("topo").show(ui, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.heading(t.app_name);
                ui.separator();
                if let Some(me) = &self.config.identity {
                    ui.label((t.who_am_i)(&me.name, &me.email));
                }
                if ui.small_button(t.change_identity).clicked() {
                    if let Some(me) = &self.config.identity {
                        self.name_input = me.name.clone();
                        self.email_input = me.email.clone();
                    }
                    self.screen = Screen::Identity;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    self.theme_picker(ui);
                });
            });
            ui.add_space(4.0);
        });
        egui::Panel::bottom("estado").show(ui, |ui| self.status_bar(ui));
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_max_width(760.0);
                ui.heading(t.recent);
                let mut open = None;
                if self.config.recent.is_empty() {
                    ui.label(t.no_recent);
                }
                for p in &self.config.recent {
                    ui.horizontal(|ui| {
                        let exists = project::is_project(p);
                        let name = p
                            .file_stem()
                            .map(|s| s.to_string_lossy().into_owned())
                            .unwrap_or_default();
                        if ui.add_enabled(exists, egui::Button::new(name)).clicked() {
                            open = Some(p.clone());
                        }
                        ui.weak(p.display().to_string());
                        if !exists {
                            ui.weak(format!("({})", t.missing_folder));
                        }
                    });
                }
                if let Some(p) = open {
                    self.open_project(&p);
                }
                ui.add_space(16.0);

                theme::card(ui).show(ui, |ui| {
                    ui.strong(t.open_folder);
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.open_input)
                                .hint_text(t.path_hint)
                                .desired_width(520.0),
                        );
                        if ui.button(t.open).clicked() {
                            let p = PathBuf::from(self.open_input.trim());
                            self.open_project(&p);
                        }
                    });
                });
                ui.add_space(8.0);
                theme::card(ui).show(ui, |ui| {
                    ui.strong(t.new_project);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_title)
                            .hint_text(t.title_hint)
                            .desired_width(520.0),
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_parent)
                            .hint_text(t.parent_hint)
                            .desired_width(520.0),
                    );
                    ui.horizontal(|ui| {
                        ui.label(t.approach);
                        egui::ComboBox::from_id_salt("abordagem")
                            .selected_text(text::approach_name(self.new_approach))
                            .show_ui(ui, |ui| {
                                for a in Approach::ALL {
                                    ui.selectable_value(
                                        &mut self.new_approach,
                                        a,
                                        text::approach_name(a),
                                    );
                                }
                            });
                        if ui.button(t.create).clicked() {
                            let (parent, title) = (
                                PathBuf::from(self.new_parent.trim()),
                                self.new_title.clone(),
                            );
                            if self.create_project(&parent, &title, self.new_approach) {
                                self.new_title.clear();
                            }
                        }
                    });
                    ui.small(t.shared_folder_tip);
                });
                ui.add_space(8.0);
                theme::card(ui).show(ui, |ui| {
                    ui.strong(t.import_lssz);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.import_file)
                            .hint_text(t.import_hint)
                            .desired_width(520.0),
                    );
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.import_dest)
                                .hint_text(t.import_dest_hint)
                                .desired_width(420.0),
                        );
                        if ui.button(t.import).clicked() {
                            let (file, dest) = (
                                PathBuf::from(self.import_file.trim()),
                                PathBuf::from(self.import_dest.trim()),
                            );
                            let r = lssz::import(&file, &dest);
                            if let Some(root) = self.report(r) {
                                self.set_status((t.imported)(&root.display().to_string()), false);
                                self.open_project(&root);
                            }
                        }
                    });
                });
            });
        });
    }

    fn theme_picker(&mut self, ui: &mut egui::Ui) {
        let t = texts();
        let name = |th: user::Theme| match th {
            user::Theme::System => t.theme_system,
            user::Theme::Light => t.theme_light,
            user::Theme::Dark => t.theme_dark,
        };
        let before = self.config.theme;
        egui::ComboBox::from_id_salt("tema")
            .selected_text(format!("{}: {}", t.theme, name(before)))
            .show_ui(ui, |ui| {
                for th in [user::Theme::System, user::Theme::Light, user::Theme::Dark] {
                    ui.selectable_value(&mut self.config.theme, th, name(th));
                }
            });
        if self.config.theme != before {
            self.save_config();
        }
    }

    fn status_bar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(3.0);
        ui.horizontal(|ui| {
            if let Some((msg, err)) = &self.status {
                let color = if *err {
                    ui.visuals().error_fg_color
                } else {
                    ui.visuals().text_color()
                };
                ui.colored_label(color, msg);
            } else {
                ui.label(" ");
            }
            if let Some(s) = &self.session {
                let pending = s.folder().pending().len();
                if pending > 0 {
                    ui.separator();
                    ui.colored_label(theme::WARN, (texts().pending_files)(pending));
                }
            }
        });
        ui.add_space(3.0);
    }

    fn project_screen(&mut self, ui: &mut egui::Ui) {
        let Some(mut s) = self.session.take() else {
            self.screen = Screen::Start;
            return;
        };
        let t = texts();
        let mut close = false;
        egui::Panel::top("topo").show(ui, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.heading(s.folder().title());
                ui.separator();
                let others = s.others();
                if !others.is_empty() {
                    ui.label(t.here_now);
                    for p in others {
                        let screen = Nav::from_key(&p.screen)
                            .map_or_else(|| p.screen.clone(), |n| n.label().to_string());
                        ui.label(RichText::new(format!("● {}", p.name)).color(theme::accent(ui)))
                            .on_hover_text((t.is_on)(&p.name, &screen));
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(t.close_project).clicked() {
                        close = true;
                    }
                    self.theme_picker(ui);
                });
            });
            let n = s.state().conflicts.len();
            if n > 0 {
                ui.horizontal(|ui| {
                    ui.colored_label(theme::WARN, (t.conflicts_banner)(n));
                    if ui.small_button(t.review).clicked() {
                        self.nav = Nav::Conflicts;
                    }
                });
            }
            ui.add_space(4.0);
        });
        egui::Panel::bottom("estado").show(ui, |ui| self.status_bar(ui));
        egui::Panel::left("navegacao")
            .default_size(190.0)
            .resizable(false)
            .show(ui, |ui| self.navigation(ui, &s));
        if self.nav == Nav::Actions && self.selected_card.is_some() {
            egui::Panel::right("cartao")
                .default_size(320.0)
                .show(ui, |ui| self.card_panel(ui, &mut s));
        }
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::both().show(ui, |ui| match self.nav {
                Nav::Overview => self.overview(ui, &s),
                Nav::Charter => self.charter(ui, &mut s),
                Nav::Phases => self.phases(ui, &s),
                Nav::Actions => self.board(ui, &mut s),
                Nav::Conflicts => self.conflicts(ui, &mut s),
                Nav::Data => self.data(ui, &s),
                Nav::Portfolio => self.portfolio(ui),
                Nav::Risks => soon(ui, t.nav_risks, t.soon_risks, "L1"),
                Nav::Meetings => soon(ui, t.nav_meetings, t.soon_meetings, "L1"),
                Nav::Schedule => soon(ui, t.nav_schedule, t.soon_schedule, "L1"),
                Nav::Savings => soon(ui, t.nav_savings, t.soon_savings, "L1"),
                Nav::Tools => soon(ui, t.nav_tools, t.soon_tools, "L2"),
            });
        });

        // Periodic work after drawing: the field being edited is known now.
        let editing = self.editing.take();
        let r = s.tick(self.nav.key(), editing);
        self.report(r);
        ui.ctx().request_repaint_after(Duration::from_millis(500));
        if close {
            s.close();
            self.screen = Screen::Start;
            self.drafts.clear();
        } else {
            self.session = Some(s);
        }
    }

    fn navigation(&mut self, ui: &mut egui::Ui, s: &Session) {
        let t = texts();
        ui.add_space(6.0);
        if ui
            .selectable_label(self.nav == Nav::Portfolio, t.nav_portfolio)
            .clicked()
        {
            self.nav = Nav::Portfolio;
        }
        ui.add_space(6.0);
        ui.strong(t.nav_project);
        for n in Nav::PROJECT {
            let mut label = n.label().to_string();
            if n == Nav::Actions {
                let count =
                    domain::of_project(s.state(), Kind::Action, s.project(), |id, _| id).len();
                if count > 0 {
                    label = format!("{label} ({count})");
                }
            }
            if ui
                .selectable_label(self.nav == n, format!("  {label}"))
                .clicked()
            {
                self.nav = n;
            }
        }
        let n = s.state().conflicts.len();
        if n > 0 {
            ui.add_space(6.0);
            let label = RichText::new(format!("{} ({n})", t.nav_conflicts)).color(theme::WARN);
            if ui
                .selectable_label(self.nav == Nav::Conflicts, label)
                .clicked()
            {
                self.nav = Nav::Conflicts;
            }
        }
    }

    /// A text box bound to a field: shows the current value, keeps what is typed while it
    /// has the focus (with the version seen then) and commits when the focus leaves.
    fn text_field(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        entity: EntityRef,
        field: &str,
        multiline: bool,
        width: f32,
    ) {
        let current = s
            .state()
            .entity(entity.id)
            .map(|e| e.text(field))
            .unwrap_or_default();
        let key = (entity.id, field.to_string());
        let (mut buf, seen) = match self.drafts.get(&key) {
            Some(d) => (d.text.clone(), d.seen),
            None => (
                current.clone(),
                s.state().entity(entity.id).and_then(|e| e.version(field)),
            ),
        };
        let edit = if multiline {
            egui::TextEdit::multiline(&mut buf).desired_rows(3)
        } else {
            egui::TextEdit::singleline(&mut buf)
        };
        let r = ui.add(edit.desired_width(width));
        if r.has_focus() {
            self.drafts.insert(
                key.clone(),
                Draft {
                    text: buf.clone(),
                    seen,
                },
            );
            self.editing = Some(entity);
        }
        if r.lost_focus() {
            self.drafts.remove(&key);
            if buf != current {
                let res = s.set_field(entity, field, Value::from(buf), Some(seen));
                self.report(res);
            }
        }
    }

    /// A date box (`DD/MM/AAAA` on screen, `AAAA-MM-DD` in the file).
    fn date_field(&mut self, ui: &mut egui::Ui, s: &mut Session, entity: EntityRef, field: &str) {
        let stored = s
            .state()
            .entity(entity.id)
            .map(|e| e.text(field))
            .unwrap_or_default();
        let shown = stored
            .parse::<Date>()
            .map(Date::format_br)
            .unwrap_or(stored.clone());
        let key = (entity.id, field.to_string());
        let (mut buf, seen) = match self.drafts.get(&key) {
            Some(d) => (d.text.clone(), d.seen),
            None => (
                shown.clone(),
                s.state().entity(entity.id).and_then(|e| e.version(field)),
            ),
        };
        let r = ui.add(
            egui::TextEdit::singleline(&mut buf)
                .hint_text(texts().date_hint)
                .desired_width(110.0),
        );
        if r.has_focus() {
            self.drafts.insert(
                key.clone(),
                Draft {
                    text: buf.clone(),
                    seen,
                },
            );
            self.editing = Some(entity);
        }
        if r.lost_focus() {
            self.drafts.remove(&key);
            if buf.trim() != shown {
                let value = if buf.trim().is_empty() {
                    Some(String::new())
                } else {
                    buf.parse::<Date>().ok().map(|d| d.to_string())
                };
                match value {
                    Some(v) => {
                        let res = s.set_field(entity, field, Value::from(v), Some(seen));
                        self.report(res);
                    }
                    None => self.set_status(texts().bad_date, true),
                }
            }
        }
    }

    fn presence_warning(&self, ui: &mut egui::Ui, s: &Session, entity: EntityRef) {
        for p in s.editing(entity) {
            ui.colored_label(theme::WARN, (texts().is_editing_this)(&p.name));
        }
    }

    fn overview(&mut self, ui: &mut egui::Ui, s: &Session) {
        let t = texts();
        let Some(o) = Overview::compute(s.state(), s.project(), s.today()) else {
            ui.label(t.untitled);
            return;
        };
        ui.heading(&o.project.title);
        ui.label(format!(
            "{} · {}",
            text::approach_name(o.project.approach),
            o.project.area
        ));
        ui.add_space(8.0);
        // Phases as a row of steps.
        ui.horizontal_wrapped(|ui| {
            for (i, ph) in o.phases.iter().enumerate() {
                let current = o.current_phase == Some(i);
                let label = format!("{}. {}", i + 1, ph.name);
                let rt = if current {
                    RichText::new(label).strong().color(theme::accent(ui))
                } else {
                    RichText::new(label)
                };
                theme::card(ui).show(ui, |ui| {
                    ui.label(rt);
                    ui.small(text::phase_status_name(ph.status));
                });
            }
        });
        ui.add_space(8.0);
        egui::Grid::new("visao")
            .num_columns(2)
            .spacing([24.0, 8.0])
            .striped(true)
            .show(ui, |ui| {
                ui.strong(t.current_phase);
                ui.label(
                    o.current_phase
                        .map_or("-".into(), |i| o.phases[i].name.clone()),
                );
                ui.end_row();
                ui.strong(t.actions);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::ProgressBar::new(o.actions_done_fraction as f32).desired_width(160.0),
                    );
                    ui.label((t.actions_done)(
                        o.actions_by_column.get(&Column::Done).copied().unwrap_or(0),
                        o.actions_total,
                    ));
                });
                ui.end_row();
                ui.strong(t.overdue);
                ui.vertical(|ui| {
                    if o.actions_overdue.is_empty() {
                        ui.label(t.none_overdue);
                    }
                    for a in &o.actions_overdue {
                        let due = a.due.map(Date::format_br).unwrap_or_default();
                        ui.colored_label(theme::WARN, format!("{due}  {}", a.title));
                    }
                });
                ui.end_row();
                ui.strong(t.risks_open);
                ui.label(format!(
                    "{} ({} {})",
                    o.risks_open,
                    o.risks_high.len(),
                    t.risks_high
                ));
                ui.end_row();
                ui.strong(t.savings);
                ui.vertical(|ui| {
                    if o.savings_by_status.is_empty() {
                        ui.label("-");
                    }
                    for (st, v) in &o.savings_by_status {
                        ui.label(format!(
                            "{}: {}",
                            text::saving_status_name(*st),
                            text::money(*v)
                        ));
                    }
                });
                ui.end_row();
                ui.strong(t.meetings);
                ui.label(o.meetings.to_string());
                ui.end_row();
                ui.strong(t.deadline);
                match (o.project.deadline, o.days_to_deadline) {
                    (Some(d), Some(n)) => {
                        let txt = format!("{} ({})", d.format_br(), (t.days_left)(n));
                        if n < 0 {
                            ui.colored_label(theme::WARN, txt)
                        } else {
                            ui.label(txt)
                        }
                    }
                    _ => ui.weak(t.no_deadline),
                };
                ui.end_row();
                ui.strong(t.conflicts);
                ui.label(o.conflicts.to_string());
                ui.end_row();
                ui.strong(t.folder);
                ui.vertical(|ui| {
                    ui.label(s.folder().root().display().to_string());
                    ui.weak((t.operations)(s.folder().journal().len()));
                });
                ui.end_row();
            });
    }

    fn charter(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = texts();
        let p = Project::entity_ref(s.project());
        ui.heading(t.nav_charter);
        ui.weak(t.charter_intro);
        self.presence_warning(ui, s, p);
        ui.add_space(8.0);
        let w = 520.0;
        use fields::project as f;
        egui::Grid::new("termo")
            .num_columns(2)
            .spacing([16.0, 8.0])
            .show(ui, |ui| {
                for (label, field, multi) in [
                    (t.f_title, f::TITLE, false),
                    (t.f_process, f::PROCESS, false),
                    (t.f_area, f::AREA, false),
                    (t.f_problem, f::PROBLEM, true),
                    (t.f_goal, f::GOAL, false),
                    (t.f_baseline, f::BASELINE, false),
                ] {
                    ui.label(label);
                    self.text_field(ui, s, p, field, multi, w);
                    ui.end_row();
                }
                ui.label(t.f_deadline);
                self.date_field(ui, s, p, f::DEADLINE);
                ui.end_row();
                for (label, field) in [
                    (t.f_scope_in, f::SCOPE_IN),
                    (t.f_scope_out, f::SCOPE_OUT),
                    (t.f_business_case, f::BUSINESS_CASE),
                ] {
                    ui.label(label);
                    self.text_field(ui, s, p, field, true, w);
                    ui.end_row();
                }
            });
    }

    fn phases(&mut self, ui: &mut egui::Ui, s: &Session) {
        let t = texts();
        ui.heading(t.nav_phases);
        let current = s
            .state()
            .entity(s.project())
            .and_then(|e| e.id(fields::project::CURRENT_PHASE));
        for (i, ph) in domain::phases(s.state(), s.project()).iter().enumerate() {
            theme::card(ui).show(ui, |ui| {
                ui.set_min_width(420.0);
                let title = format!("{}. {}", i + 1, ph.name);
                if current == Some(ph.id) {
                    ui.label(RichText::new(title).strong().color(theme::accent(ui)));
                } else {
                    ui.strong(title);
                }
                ui.small(text::phase_status_name(ph.status));
            });
        }
        ui.add_space(8.0);
        ui.weak(t.soon_phases);
        ui.weak((t.coming_in)("L1"));
    }

    fn board(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = texts();
        ui.heading(t.nav_actions);
        ui.horizontal(|ui| {
            let r = ui.add(
                egui::TextEdit::singleline(&mut self.new_card)
                    .hint_text(t.new_action_hint)
                    .desired_width(360.0),
            );
            let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if ui.button(t.add).clicked() || enter {
                let title = std::mem::take(&mut self.new_card);
                let res = s.add_card(&title, Column::Todo);
                if let Some(Some(id)) = self.report(res) {
                    self.selected_card = Some(id);
                }
            }
        });
        ui.weak(t.board_tip);
        ui.add_space(6.0);
        let board = kanban::board(s.state(), s.project());
        let today = s.today();
        let mut dropped: Option<(Uid, Column)> = None;
        let mut select = None;
        ui.horizontal_top(|ui| {
            for col in Column::ALL {
                let cards = &board[&col];
                ui.vertical(|ui| {
                    ui.set_width(220.0);
                    ui.strong(format!("{} ({})", text::column_name(col), cards.len()));
                    let frame = theme::card(ui);
                    let (_, payload) = ui.dnd_drop_zone::<Uid, ()>(frame, |ui| {
                        ui.set_min_size(egui::vec2(204.0, 80.0));
                        for a in cards {
                            if self.card_widget(ui, s, a, today) {
                                select = Some(a.id);
                            }
                        }
                    });
                    if let Some(id) = payload {
                        dropped = Some((*id, col));
                    }
                });
            }
        });
        if let Some(id) = select {
            self.selected_card = Some(id);
        }
        if let Some((id, col)) = dropped {
            let res = s.move_card(id, col, usize::MAX);
            self.report(res);
        }
    }

    /// One card; returns whether it was clicked.
    fn card_widget(&self, ui: &mut egui::Ui, s: &Session, a: &Action, today: Date) -> bool {
        let selected = self.selected_card == Some(a.id);
        let mut frame = theme::card(ui);
        if selected {
            frame = frame.stroke(egui::Stroke::new(1.5, theme::accent(ui)));
        }
        let mut clicked = false;
        frame.show(ui, |ui| {
            ui.set_width(196.0);
            ui.horizontal(|ui| {
                ui.dnd_drag_source(egui::Id::new(("cartao", a.id)), a.id, |ui| {
                    ui.label(RichText::new("≡").strong());
                });
                let title = if a.title.is_empty() {
                    texts().untitled
                } else {
                    a.title.as_str()
                };
                clicked |= ui.selectable_label(selected, title).clicked();
            });
            ui.horizontal_wrapped(|ui| {
                if let Some(d) = a.due {
                    let txt = d.format_br();
                    if a.is_overdue(today) {
                        ui.colored_label(theme::WARN, txt);
                    } else {
                        ui.small(txt);
                    }
                }
                if let Some(who) = a.who {
                    ui.small(domain::person_name(s.state(), who));
                }
                if !s.editing(Action::entity_ref(a.id)).is_empty() {
                    ui.colored_label(theme::WARN, "✎");
                }
            });
        });
        clicked
    }

    fn card_panel(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = texts();
        let Some(id) = self.selected_card else { return };
        let Some(entity) = s.state().entity(id).filter(|e| !e.is_deleted()) else {
            self.selected_card = None;
            return;
        };
        let card = Action::read(id, entity);
        let r = Action::entity_ref(id);
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.strong(t.nav_actions);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(t.close).clicked() {
                    self.selected_card = None;
                }
            });
        });
        self.presence_warning(ui, s, r);
        let w = 290.0;
        use fields::action as f;
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (label, field, multi) in [
                (t.a_what, f::TITLE, false),
                (t.a_why, f::WHY, true),
                (t.a_where, f::WHERE, false),
            ] {
                ui.label(label);
                self.text_field(ui, s, r, field, multi, w);
            }
            ui.label(t.a_when);
            self.date_field(ui, s, r, f::DUE);
            ui.label(t.a_who);
            let people = domain::people(s.state());
            let mut who = card.who;
            let shown = who.map_or(t.nobody.to_string(), |p| domain::person_name(s.state(), p));
            egui::ComboBox::from_id_salt("quem")
                .selected_text(shown)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut who, None, t.nobody);
                    for p in &people {
                        ui.selectable_value(&mut who, Some(p.id), &p.name);
                    }
                });
            if who != card.who {
                let v = who.map_or(Value::from(""), |p| Value::from(p.to_string()));
                let res = s.set_field(r, f::WHO, v, None);
                self.report(res);
            }
            for (label, field) in [(t.a_how, f::HOW), (t.a_how_much, f::HOW_MUCH)] {
                ui.label(label);
                self.text_field(ui, s, r, field, false, w);
            }
            ui.label(t.a_column);
            ui.horizontal(|ui| {
                let mut col = card.column;
                egui::ComboBox::from_id_salt("coluna")
                    .selected_text(text::column_name(col))
                    .show_ui(ui, |ui| {
                        for c in Column::ALL {
                            ui.selectable_value(&mut col, c, text::column_name(c));
                        }
                    });
                if col != card.column {
                    let res = s.move_card(id, col, usize::MAX);
                    self.report(res);
                }
            });
            ui.horizontal(|ui| {
                let moves = [
                    (t.move_left, 0, -1),
                    (t.move_right, 0, 1),
                    (t.move_up, -1, 0),
                    (t.move_down, 1, 0),
                ];
                for (label, nudge, shift) in moves {
                    if ui.button(label).clicked() {
                        let res = if shift != 0 {
                            s.shift_card(id, shift)
                        } else {
                            s.nudge_card(id, nudge)
                        };
                        self.report(res);
                    }
                }
            });
            ui.add_space(12.0);
            if ui
                .button(RichText::new(t.delete).color(ui.visuals().error_fg_color))
                .clicked()
            {
                let res = s.delete(r);
                self.report(res);
                self.selected_card = None;
            }
        });
    }

    fn conflicts(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = texts();
        ui.heading(t.nav_conflicts);
        ui.weak(t.conflicts_intro);
        ui.add_space(8.0);
        let list = s.state().conflicts.clone();
        if list.is_empty() {
            ui.label(t.no_conflicts);
        }
        let mut action = None;
        for (i, c) in list.iter().enumerate() {
            theme::card(ui).show(ui, |ui| {
                ui.set_min_width(520.0);
                let what = entity_label(s, c.entity);
                ui.strong(format!("{what} · {}", c.field));
                let show = |v: &Value| match v {
                    Value::String(x) => x.clone(),
                    other => other.to_string(),
                };
                egui::Grid::new(("conflito", i))
                    .num_columns(3)
                    .spacing([12.0, 4.0])
                    .show(ui, |ui| {
                        ui.label(t.kept);
                        ui.label(RichText::new(show(&c.kept.value)).strong());
                        ui.weak((t.by_whom)(&domain::person_name(s.state(), c.kept.author)));
                        ui.end_row();
                        ui.label(t.replaced);
                        ui.label(show(&c.replaced.value));
                        ui.weak((t.by_whom)(&domain::person_name(
                            s.state(),
                            c.replaced.author,
                        )));
                        ui.end_row();
                    });
                ui.horizontal(|ui| {
                    if ui.button(t.keep_this).clicked() {
                        action = Some((i, false));
                    }
                    if ui.button(t.restore_other).clicked() {
                        action = Some((i, true));
                    }
                });
            });
        }
        if let Some((i, restore)) = action {
            let res = if restore {
                s.restore(&list[i])
            } else {
                s.keep(&list[i])
            };
            self.report(res);
        }
    }

    fn data(&mut self, ui: &mut egui::Ui, s: &Session) {
        let t = texts();
        ui.heading(t.nav_data);
        ui.weak(t.data_intro);
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.export_dest)
                    .hint_text(t.export_dest_hint)
                    .desired_width(460.0),
            );
            if ui.button(t.export_lssz).clicked() {
                let dest = PathBuf::from(self.export_dest.trim());
                let res = s.export(&dest);
                if self.report(res).is_some() {
                    self.set_status((t.exported)(&dest.display().to_string()), false);
                }
            }
        });
        ui.add_space(8.0);
        ui.weak(t.soon_data);
        ui.weak((t.coming_in)("L3"));
    }

    fn portfolio(&mut self, ui: &mut egui::Ui) {
        let t = texts();
        ui.heading(t.nav_portfolio);
        ui.weak(t.soon_portfolio);
        ui.weak((t.coming_in)("L1"));
        ui.add_space(8.0);
        ui.strong(t.recent);
        let mut open = None;
        for p in &self.config.recent {
            let name = p
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default();
            let current = self
                .session
                .as_ref()
                .is_some_and(|s| s.folder().root() == p.as_path());
            if ui
                .add_enabled(!current && project::is_project(p), egui::Button::new(name))
                .clicked()
            {
                open = Some(p.clone());
            }
        }
        if let Some(p) = open {
            self.close_project();
            self.open_project(&p);
        }
    }
}

fn soon(ui: &mut egui::Ui, title: &str, what: &str, phase: &str) {
    ui.heading(title);
    ui.add_space(4.0);
    ui.label(what);
    ui.add_space(4.0);
    ui.weak((texts().coming_in)(phase));
}

/// "Ação: Calibrar o paquímetro" for the conflict list.
fn entity_label(s: &Session, e: EntityRef) -> String {
    let t = texts();
    let entity = s.state().entity(e.id);
    let name = entity
        .map(|x| {
            let title = x.text("titulo");
            if title.is_empty() {
                x.text("nome")
            } else {
                title
            }
        })
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| e.id.short());
    let kind = match e.kind {
        Kind::Action => t.nav_actions,
        Kind::Project => t.nav_project,
        Kind::Risk => t.nav_risks,
        Kind::Meeting => t.nav_meetings,
        Kind::Saving => t.nav_savings,
        Kind::Phase => t.phases,
        other => other.key(),
    };
    format!("{kind}: {name}")
}

/// A folder name from a title: without the characters file systems refuse.
pub fn sanitize_folder_name(title: &str) -> String {
    let cleaned: String = title
        .trim()
        .chars()
        .map(|c| {
            if matches!(c, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*') || c.is_control() {
                '-'
            } else {
                c
            }
        })
        .collect();
    let cleaned = cleaned
        .trim_matches(|c: char| c == '.' || c == ' ')
        .to_string();
    if cleaned.is_empty() {
        "projeto".into()
    } else {
        cleaned.chars().take(80).collect()
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.draw(ui);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Some(mut s) = self.session.take() {
            s.close();
        }
    }
}
