//! Coaching and the growth of the belts (8.3): the coaching log of a belt (its own folder,
//! seen only by the MBB and the belt), sessions with their commitments turned into cards
//! on the project's board, the progress, the competence map with its evidence and the
//! MBB's confirmation, and the belt's timeline.

use eframe::egui::{self, RichText};
use utsim_lss_core::mentoria::{
    self, Commitment, Evidence, Form, Level, Session as Coaching, SessionInput, Target,
};
use utsim_lss_core::perfis::Action;
use utsim_lss_core::{Id, State};
use utsim_lss_store::program;

use super::Data;
use crate::app::App;
use crate::app::screens::parse_date;
use crate::session::Session;
use crate::text::l6::{form_name, l6, level_short, mentoring_level_name, target_name, tool_label};
use crate::text::texts;
use crate::theme;

impl App {
    pub(in crate::app) fn program_coaching(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
    ) {
        let tl = l6();
        ui.heading(tl.nav_coaching);
        ui.weak(tl.coaching_intro);
        let visible: Vec<&utsim_lss_core::programa::Member> = d
            .members
            .iter()
            .filter(|m| m.is_belt() && !m.anonymised)
            .filter(|m| mentoria::can_see(d.me, m.id, d.is_mbb()))
            .collect();
        if visible.is_empty() {
            ui.weak(tl.no_belts_visible);
        }
        ui.horizontal_wrapped(|ui| {
            for m in &visible {
                if ui
                    .selectable_label(self.l6.belt == Some(m.id), &m.name)
                    .clicked()
                {
                    self.l6.belt = Some(m.id);
                    self.l6.selected_session = None;
                }
            }
        });
        if let Some(b) = self
            .l6
            .belt
            .and_then(|b| visible.iter().find(|m| m.id == b))
            .map(|m| (*m).clone())
        {
            ui.separator();
            self.coaching_log(ui, s, d, &b);
        }
        ui.separator();
        self.competence_section(ui, s, d);
    }

    /// The log of one belt: opened lazily from its folder.
    fn coaching_log(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
        belt: &utsim_lss_core::programa::Member,
    ) {
        let tl = l6();
        if !belt.consent {
            ui.colored_label(theme::WARN, tl.no_consent);
        }
        let path = program::coaching_path(s.folder().root(), belt.id);
        if !self.l6.logs.contains_key(&belt.id)
            && path.join(utsim_lss_store::project::MANIFEST).is_file()
        {
            let me = s.folder().me().clone();
            let r = Session::open_coaching(
                &path,
                &me,
                (self.clock)(),
                belt.id,
                d.program,
                d.mbbs.clone(),
                false,
            );
            if let Some(log) = self.report(r) {
                self.l6.logs.insert(belt.id, log);
            }
        }
        let empty = State::new();
        let st: &State = self.l6.logs.get(&belt.id).map_or(&empty, |l| l.state());
        let sessions = mentoria::sessions(st);
        let commitments = mentoria::commitments(st);
        let progress = mentoria::progress(st, |_| false);
        ui.label((tl.progress)(
            progress.sessions,
            progress.hours,
            progress.done,
            progress.commitments,
        ));
        for (tool, (a, b)) in &progress.ratings {
            ui.small(format!("{}: {a} → {b}", tool_label(tool)));
        }
        let mut select = None;
        for x in &sessions {
            let label = format!(
                "{} · {} · {} min · {}",
                x.date.map_or(String::new(), |d| d.format_br()),
                form_name(x.form),
                x.minutes,
                x.topics
            );
            if ui
                .selectable_label(self.l6.selected_session == Some(x.id), label)
                .clicked()
            {
                select = Some(x.id);
            }
        }
        if let Some(x) = select {
            self.l6.selected_session = Some(x);
        }
        if self.l6.selected_session.is_none() {
            self.l6.selected_session = sessions.last().map(|x| x.id);
        }
        if let Some(sel) = self
            .l6
            .selected_session
            .and_then(|x| sessions.iter().find(|y| y.id == x))
            .cloned()
        {
            self.session_detail(ui, s, d, belt, &sel, &commitments);
        }
        self.new_session_form(ui, s, d, belt);
    }

    fn session_detail(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
        belt: &utsim_lss_core::programa::Member,
        x: &Coaching,
        commitments: &[Commitment],
    ) {
        let tl = l6();
        theme::card(ui).show(ui, |ui| {
            ui.strong((tl.session_of)(
                &x.date.map_or(String::new(), |d| d.format_br()),
            ));
            if !x.tools.is_empty() {
                let names: Vec<String> = x.tools.iter().map(|t| tool_label(t)).collect();
                ui.label((tl.tools_discussed)(&names.join(", ")));
            }
            if let Some(p) = x.project {
                ui.label((tl.session_project)(
                    &d.records
                        .iter()
                        .find(|r| r.id == p)
                        .map_or_else(|| p.short(), |r| r.title.clone()),
                ));
            }
            let mut to_card = None;
            for c in commitments.iter().filter(|c| c.session == Some(x.id)) {
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{} {} ({})",
                        if c.done { "☑" } else { "☐" },
                        c.text,
                        target_name(c.target)
                    ));
                    if let Some(due) = c.due {
                        ui.weak(due.format_br());
                    }
                    if c.target == Target::Project
                        && c.card.is_none()
                        && ui.small_button(tl.to_card).clicked()
                    {
                        to_card = Some(c.clone());
                    }
                    if c.card.is_some() {
                        ui.weak(tl.is_card);
                    }
                });
            }
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(self.l6.input("compromisso"))
                        .hint_text(tl.commitment_hint)
                        .desired_width(260.0),
                );
                ui.add(
                    egui::TextEdit::singleline(self.l6.input("compromisso_prazo"))
                        .hint_text(texts().date_hint)
                        .desired_width(100.0),
                );
                let mut t = self.l6.commitment_target;
                egui::ComboBox::from_id_salt("destino_compromisso")
                    .selected_text(target_name(t))
                    .show_ui(ui, |ui| {
                        for v in [Target::Project, Target::Personal] {
                            ui.selectable_value(&mut t, v, target_name(v));
                        }
                    });
                self.l6.commitment_target = t;
                if ui.button(texts().add).clicked() {
                    let text = self.l6.input("compromisso").clone();
                    let due = parse_date(self.l6.input("compromisso_prazo"));
                    let id = s.new_id();
                    match mentoria::add_commitment(x.id, id, &text, due, t) {
                        Ok(e) => {
                            if self.log_commit(s, d, belt.id, e) {
                                self.l6.take_input("compromisso");
                                self.l6.take_input("compromisso_prazo");
                            }
                        }
                        Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                    }
                }
            });
            if let Some(c) = to_card {
                self.commitment_to_card(s, d, belt.id, x, &c);
            }
        });
    }

    /// Turns a commitment into a card on the board of the session's project.
    fn commitment_to_card(
        &mut self,
        s: &mut Session,
        d: &Data,
        belt: Id,
        x: &Coaching,
        c: &Commitment,
    ) {
        let tl = l6();
        let Some(project) = x.project else {
            self.set_status(
                (texts().error)(&mentoria::CoachingError::NoProject.to_string()),
                true,
            );
            return;
        };
        let Some(path) = d.scan.path_of(project).map(|p| p.to_path_buf()) else {
            self.set_status(tl.project_unreachable, true);
            return;
        };
        let Some(pst) = self.folder_state(s, &path) else {
            return;
        };
        let card = s.new_id();
        match mentoria::commitment_card(&pst, project, c, belt, card) {
            Ok((card_edits, link)) => {
                if self.write_folder(s, &path, card_edits) {
                    self.log_commit(s, d, belt, vec![link]);
                    self.set_status(tl.card_made, false);
                }
            }
            Err(e) => self.set_status((texts().error)(&e.to_string()), true),
        }
    }

    fn new_session_form(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
        belt: &utsim_lss_core::programa::Member,
    ) {
        let tl = l6();
        ui.add_space(4.0);
        ui.strong(tl.new_session);
        ui.horizontal(|ui| {
            for (k, hint, w) in [
                ("sessao_data", texts().date_hint, 100.0),
                ("sessao_min", tl.minutes_hint, 70.0),
                ("sessao_fase", tl.phase_hint, 100.0),
            ] {
                ui.add(
                    egui::TextEdit::singleline(self.l6.input(k))
                        .hint_text(hint)
                        .desired_width(w),
                );
            }
            let mut f = self.l6.session_form;
            egui::ComboBox::from_id_salt("forma_sessao")
                .selected_text(form_name(f))
                .show_ui(ui, |ui| {
                    for v in Form::ALL {
                        ui.selectable_value(&mut f, v, form_name(v));
                    }
                });
            self.l6.session_form = f;
            let mine: Vec<&utsim_lss_core::programa::ProjectRecord> = d
                .records
                .iter()
                .filter(|r| r.belts.contains(&belt.id))
                .collect();
            let mut p = self.l6.session_project;
            egui::ComboBox::from_id_salt("projeto_sessao")
                .selected_text(
                    p.and_then(|x| mine.iter().find(|r| r.id == x))
                        .map_or(tl.no_project.to_string(), |r| r.title.clone()),
                )
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut p, None, tl.no_project);
                    for r in &mine {
                        ui.selectable_value(&mut p, Some(r.id), &r.title);
                    }
                });
            self.l6.session_project = p;
        });
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(self.l6.input("sessao_temas"))
                    .hint_text(tl.topics_hint)
                    .desired_width(300.0),
            );
            ui.add(
                egui::TextEdit::singleline(self.l6.input("sessao_ferramentas"))
                    .hint_text(tl.planned_tools_hint)
                    .desired_width(220.0),
            );
            if ui.button(tl.record_session).clicked() {
                let input = SessionInput {
                    belt: belt.id,
                    mentor: d.me,
                    date: parse_date(self.l6.input("sessao_data")).unwrap_or(d.today),
                    minutes: super::super::parse_number(self.l6.input("sessao_min"))
                        .unwrap_or(60.0),
                    form: self.l6.session_form,
                    project: self.l6.session_project,
                    phase: self.l6.input("sessao_fase").clone(),
                    topics: self.l6.input("sessao_temas").clone(),
                    tools: self
                        .l6
                        .input("sessao_ferramentas")
                        .split(',')
                        .map(|t| t.trim().to_lowercase())
                        .filter(|t| !t.is_empty())
                        .collect(),
                };
                let id = s.new_id();
                match mentoria::new_session(belt, id, &input) {
                    Ok(e) => {
                        if self.log_commit(s, d, belt.id, e) {
                            for k in [
                                "sessao_data",
                                "sessao_min",
                                "sessao_fase",
                                "sessao_temas",
                                "sessao_ferramentas",
                            ] {
                                self.l6.take_input(k);
                            }
                            self.l6.selected_session = Some(id);
                        }
                    }
                    Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                }
            }
        });
    }

    /// Writes into the belt's coaching log, creating its folder on the first session.
    fn log_commit(
        &mut self,
        s: &Session,
        d: &Data,
        belt: Id,
        edits: Vec<utsim_lss_core::Edit>,
    ) -> bool {
        if !self.l6.logs.contains_key(&belt) {
            let path = program::coaching_path(s.folder().root(), belt);
            let me = s.folder().me().clone();
            let r = Session::open_coaching(
                &path,
                &me,
                (self.clock)(),
                belt,
                d.program,
                d.mbbs.clone(),
                true,
            );
            match self.report(r) {
                Some(log) => {
                    self.l6.logs.insert(belt, log);
                }
                None => return false,
            }
        }
        let Some(log) = self.l6.logs.get_mut(&belt) else {
            return false;
        };
        let r = log.apply(edits);
        self.report(r).is_some()
    }

    fn competence_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let tl = l6();
        ui.strong(tl.competence_map);
        ui.weak(tl.competence_intro);
        let belts: Vec<&utsim_lss_core::programa::Member> = d
            .members
            .iter()
            .filter(|m| m.is_belt() && !m.anonymised)
            .filter(|m| d.is_mbb() || m.id == d.me)
            .collect();
        let tools = mentoria::all_tools();
        egui::ScrollArea::horizontal()
            .id_salt("mapa_competencias")
            .show(ui, |ui| {
                egui::Grid::new("mapa").striped(true).show(ui, |ui| {
                    ui.strong(tl.person);
                    for t in &tools {
                        ui.small(tool_label(t));
                    }
                    ui.end_row();
                    for b in &belts {
                        ui.label(&b.name);
                        for t in &tools {
                            let cell = d.map.cells.get(&(b.id, t.clone()));
                            let txt = cell.and_then(|c| c.level).map_or("·", level_short);
                            let r = ui.label(RichText::new(txt).monospace());
                            if let Some(c) = cell {
                                let ev: Vec<String> = c
                                    .evidence
                                    .iter()
                                    .map(|e| match e {
                                        Evidence::Entry { level, text, .. } => {
                                            format!("{}: {text}", mentoring_level_name(*level))
                                        }
                                        Evidence::Deliverable { project, .. } => (tl
                                            .ev_deliverable)(
                                            &d.records
                                                .iter()
                                                .find(|r| r.id == *project)
                                                .map_or_else(
                                                    || project.short(),
                                                    |r| r.title.clone(),
                                                ),
                                        ),
                                    })
                                    .collect();
                                r.on_hover_text(ev.join("\n"));
                            }
                        }
                        ui.end_row();
                    }
                });
            });
        ui.small(tl.levels_legend);
        if !d.is_mbb() {
            return;
        }
        // The MBB confirms `demonstrou`.
        ui.horizontal(|ui| {
            let cur = self.l6.belt;
            if let Some(b) = Self::member_combo(ui, d, "confirma_belt", cur, true) {
                self.l6.belt = b;
            }
            let mut tool = self.l6.confirm_tool.clone();
            egui::ComboBox::from_id_salt("confirma_ferramenta")
                .selected_text(if tool.is_empty() {
                    tl.tool.to_string()
                } else {
                    tool_label(&tool)
                })
                .show_ui(ui, |ui| {
                    for t in &tools {
                        ui.selectable_value(&mut tool, t.clone(), tool_label(t));
                    }
                });
            self.l6.confirm_tool = tool;
            ui.add(
                egui::TextEdit::singleline(self.l6.input("evidencia"))
                    .hint_text(tl.evidence_hint)
                    .desired_width(220.0),
            );
            if ui.button(tl.confirm_demonstrated).clicked() {
                let (Some(b), false) = (self.l6.belt, self.l6.confirm_tool.is_empty()) else {
                    self.set_status(tl.choose_belt_tool, true);
                    return;
                };
                if !self.may_do(s, Action::ConfirmCompetence, None) {
                    return;
                }
                let ev = self.l6.take_input("evidencia");
                let e = mentoria::record_level(
                    d.program,
                    s.new_id(),
                    b,
                    &self.l6.confirm_tool.clone(),
                    Level::Demonstrated,
                    mentoria::Source::Mbb,
                    &ev,
                    s.me(),
                    d.today,
                );
                self.commit(s, e);
            }
        });
        // Timeline of the selected belt.
        if let Some(b) = self
            .l6
            .belt
            .and_then(|b| d.members.iter().find(|m| m.id == b))
        {
            let sessions = self
                .l6
                .logs
                .get(&b.id)
                .map(|l| mentoria::sessions(l.state()))
                .unwrap_or_default();
            let ev = mentoria::timeline(b, &sessions, &d.records, &d.entries);
            if !ev.is_empty() {
                ui.strong((tl.timeline_of)(&b.name));
                for e in ev {
                    let line = match e {
                        mentoria::Event::Session { date, topics } => {
                            (tl.t_session)(&date.format_br(), &topics)
                        }
                        mentoria::Event::TollgateApproved { date, phase, .. } => {
                            (tl.t_tollgate)(&date.format_br(), &phase)
                        }
                        mentoria::Event::NewTool { date, tool } => {
                            (tl.t_tool)(&date.format_br(), &tool_label(&tool))
                        }
                        mentoria::Event::Certified { date } => (tl.t_certified)(&date.format_br()),
                    };
                    ui.label(line);
                }
            }
        }
    }
}
