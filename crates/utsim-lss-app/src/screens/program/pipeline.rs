//! The pipeline across the portfolios and the selection committee (8.1): ideas by stage
//! and unit, sending an idea in screening to the committee (the unit's champion or the
//! MBB), committee meetings with the generated agenda, and the decisions with their
//! reason, which also move the idea in its portfolio.

use eframe::egui::{self, RichText};
use utsim_lss_core::designacao;
use utsim_lss_core::meeting;
use utsim_lss_core::perfis::Action;
use utsim_lss_core::portfolio;
use utsim_lss_core::programa::{self, AgendaLine, Choice, Stage};

use super::Data;
use crate::app::App;
use crate::app::screens::parse_date;
use crate::session::Session;
use crate::text::l6::{choice_name, l6, level_name, metric_name, stage_name};
use crate::text::texts;
use crate::theme;

impl App {
    pub(in crate::app) fn program_pipeline(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
    ) {
        let tl = l6();
        ui.heading(tl.nav_pipeline);
        ui.weak(tl.pipeline_intro);
        // Ideas by stage.
        let mut by_stage: Vec<(Stage, Vec<&programa::IdeaRecord>)> =
            Stage::ALL.iter().map(|s| (*s, Vec::new())).collect();
        for i in &d.ideas {
            let pr = programa::project_of(&d.records, i);
            if pr.is_some_and(|p| p.training) {
                continue;
            }
            if let Some(st) = programa::stage(i, programa::opportunity_of(&d.opps, i.id), pr)
                && let Some((_, v)) = by_stage.iter_mut().find(|(x, _)| *x == st)
            {
                v.push(i);
            }
        }
        let mut send = None;
        ui.horizontal_top(|ui| {
            for (st, list) in &by_stage {
                ui.vertical(|ui| {
                    ui.set_width(180.0);
                    ui.strong(format!("{} ({})", stage_name(*st), list.len()));
                    theme::card(ui).show(ui, |ui| {
                        ui.set_min_size(egui::vec2(164.0, 40.0));
                        for i in list {
                            ui.label(format!("{} · {:.0}", i.title, i.score));
                            ui.small(d.unit_name(i.unit));
                            if *st == Stage::Screening
                                && ui.small_button(tl.send_to_committee).clicked()
                            {
                                send = Some((*i).clone());
                            }
                        }
                    });
                });
            }
        });
        if let Some(i) = send
            && self.may_do(s, Action::SendToCommittee, i.unit)
        {
            let r =
                programa::send_to_committee(d.program, s.new_id(), &i, &d.opps, s.me(), d.today);
            if self.commit_rule(s, r) {
                self.set_status((tl.sent)(&i.title), false);
            }
        }
        ui.separator();
        self.committee_section(ui, s, d);
    }

    fn committee_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let tl = l6();
        ui.strong(tl.committee);
        ui.weak((tl.next_committee)(
            &programa::next_committee(s.state(), d.program, d.today).format_br(),
        ));
        let cs = programa::committees(s.state(), d.program);
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(self.l6.input("comite_titulo"))
                    .hint_text(tl.committee_title_hint)
                    .desired_width(260.0),
            );
            ui.add(
                egui::TextEdit::singleline(self.l6.input("comite_data"))
                    .hint_text(texts().date_hint)
                    .desired_width(110.0),
            );
            if ui.button(tl.new_committee).clicked() {
                let date = parse_date(self.l6.input("comite_data")).unwrap_or(d.today);
                let mut title = self.l6.take_input("comite_titulo");
                if title.trim().is_empty() {
                    title = (tl.default_committee_title)(&date.format_br());
                }
                let id = s.new_id();
                if self.commit(s, programa::new_committee(d.program, id, &title, date)) {
                    self.l6.committee = Some(id);
                    self.l6.take_input("comite_data");
                }
            }
        });
        ui.horizontal_wrapped(|ui| {
            for m in &cs {
                let label = format!(
                    "{} {}",
                    m.date.map_or(String::new(), |x| x.format_br()),
                    m.title
                );
                if ui
                    .selectable_label(self.l6.committee == Some(m.id), label)
                    .clicked()
                {
                    self.l6.committee = Some(m.id);
                }
            }
        });
        // The generated agenda.
        ui.add_space(4.0);
        ui.strong(tl.agenda);
        let year = d.year(&self.l6);
        let rows = programa::scoreboard(s.state(), d.program, year, &d.records, &d.members);
        let free = designacao::free_by_level(&d.loads);
        let mut agenda = programa::agenda(&d.opps, &free, &rows);
        // The certification projects the training asks for (stage L7).
        agenda.extend(
            utsim_lss_core::formacao::certification_requests(&d.tr.noms, &d.opps, &d.assignments)
                .into_iter()
                .map(|r| AgendaLine::CertificationProject {
                    person: r.person,
                    level: r.level,
                    opportunity: r.opportunity,
                }),
        );
        if agenda.is_empty() {
            ui.weak(tl.agenda_empty);
        }
        for line in &agenda {
            match line {
                AgendaLine::Decide {
                    opportunity,
                    title,
                    unit,
                    score,
                } => {
                    let sel = self.l6.opportunity == Some(*opportunity);
                    if ui
                        .selectable_label(
                            sel,
                            (tl.agenda_decide)(title, &d.unit_name(*unit), *score),
                        )
                        .clicked()
                    {
                        self.l6.opportunity = Some(*opportunity);
                    }
                }
                AgendaLine::Capacity { level, free } => {
                    ui.label((tl.agenda_capacity)(level_name(*level), *free));
                }
                AgendaLine::OpenTarget {
                    unit,
                    metric,
                    level,
                    target,
                    actual,
                } => {
                    let m = match level {
                        Some(l) => format!("{} ({})", metric_name(*metric), level_name(*l)),
                        None => metric_name(*metric).to_string(),
                    };
                    ui.label((tl.agenda_target)(
                        &d.unit_name(*unit),
                        &m,
                        *actual,
                        *target,
                    ));
                }
                AgendaLine::CertificationProject {
                    person,
                    level,
                    opportunity,
                } => {
                    let opp = opportunity
                        .and_then(|o| d.ideas.iter().find(|i| i.id == o))
                        .map_or_else(String::new, |i| i.title.clone());
                    ui.label((crate::text::l7::l7().agenda_cert_project)(
                        &d.person(*person),
                        level_name(*level),
                        &opp,
                    ));
                }
            }
        }
        ui.small(tl.agenda_rule);
        // Decision of the selected opportunity.
        if let Some(o) = self
            .l6
            .opportunity
            .and_then(|id| d.opps.iter().find(|o| o.id == id))
            .filter(|o| o.open())
            .cloned()
        {
            ui.add_space(4.0);
            theme::card(ui).show(ui, |ui| {
                ui.label(RichText::new((tl.decide_on)(&o.title)).strong());
                for past in programa::decisions(s.state(), o.id) {
                    ui.weak(format!(
                        "{} · {} · {}",
                        past.date.map_or(String::new(), |x| x.format_br()),
                        past.choice.map_or("", choice_name),
                        past.reason
                    ));
                }
                ui.horizontal(|ui| {
                    let mut c = self.l6.choice.unwrap_or(Choice::Select);
                    egui::ComboBox::from_id_salt("decisao_comite")
                        .selected_text(choice_name(c))
                        .show_ui(ui, |ui| {
                            for x in Choice::ALL {
                                ui.selectable_value(&mut c, x, choice_name(x));
                            }
                        });
                    self.l6.choice = Some(c);
                    if c == Choice::Select {
                        ui.label(tl.belt_candidate);
                        if let Some(b) =
                            Self::member_combo(ui, d, "belt_comite", self.l6.decision_belt, true)
                        {
                            self.l6.decision_belt = b;
                        }
                    }
                });
                ui.add(
                    egui::TextEdit::singleline(self.l6.input("motivo_comite"))
                        .hint_text(tl.reason_hint)
                        .desired_width(420.0),
                );
                if ui.button(tl.record_decision).clicked() {
                    self.decide(s, d, &o);
                }
            });
        }
        // Minutes of the selected committee.
        if let Some(m) = self.l6.committee {
            let items = meeting::items(s.state(), m);
            if !items.is_empty() {
                ui.add_space(4.0);
                ui.strong(tl.minutes);
                for i in items {
                    ui.label(format!("• {}", i.text));
                }
            }
        }
    }

    /// Records the decision in the program and moves the idea in its portfolio, in this
    /// order of checks: the profile, the portfolio's rule, then both writes.
    fn decide(&mut self, s: &mut Session, d: &Data, o: &programa::Opportunity) {
        let tl = l6();
        if !self.may_do(s, Action::CommitteeDecide, None) {
            return;
        }
        let choice = self.l6.choice.unwrap_or(Choice::Select);
        let reason = self.l6.input("motivo_comite").clone();
        let input = programa::DecisionInput {
            opportunity: o.id,
            meeting: self.l6.committee,
            choice,
            reason: reason.clone(),
            belt: if choice == Choice::Select {
                self.l6.decision_belt
            } else {
                None
            },
            by: s.me(),
            date: d.today,
        };
        let program_edits = match programa::decide(s.state(), &input, s.ids()) {
            Ok(e) => e,
            Err(e) => {
                self.set_status((texts().error)(&e.to_string()), true);
                return;
            }
        };
        // The idea's move in its portfolio, prepared before anything is written.
        let mut portfolio_write = None;
        if let (Some(to), Some(idea), Some(pid)) = (choice.idea_status(), o.idea, o.portfolio) {
            let Some(path) = d.scan.portfolio_path(pid).map(|p| p.to_path_buf()) else {
                self.set_status(tl.portfolio_unreachable, true);
                return;
            };
            let Some(st) = self.folder_state(s, &path) else {
                return;
            };
            let mut e = match portfolio::move_idea(&st, idea, to, reason.trim()) {
                Ok(e) => e,
                Err(e) => {
                    self.set_status((tl.portfolio_refused)(&e.to_string()), true);
                    return;
                }
            };
            if let Some(b) = input.belt {
                e.push(utsim_lss_core::Edit::new(
                    portfolio::Idea::entity_ref(idea),
                    portfolio::fields::BELT,
                    b.to_string(),
                ));
            }
            portfolio_write = Some((path, e));
        }
        if !self.commit(s, program_edits) {
            return;
        }
        if let Some((path, e)) = portfolio_write {
            self.write_folder(s, &path, e);
        }
        self.l6.take_input("motivo_comite");
        self.l6.opportunity = None;
        self.set_status((tl.decided)(&o.title, choice_name(choice)), false);
    }
}
