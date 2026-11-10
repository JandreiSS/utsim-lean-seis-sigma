//! Training (8.5): waves with their targets, classes, the nomination criteria, the
//! nominations with the manager's approval, the selection with the waiting list, and the
//! training teams suggested under constraints and confirmed by the MBB.

use crate::text::LocalDate;
use eframe::egui;
use utsim_lss_core::formacao::{
    self, ClassInput, CriterionKind, Nomination, NominationInput, NominationStatus, teams,
};
use utsim_lss_core::op::EntityRef;
use utsim_lss_core::perfis::Action;
use utsim_lss_core::programa::BeltLevel;
use utsim_lss_core::{Edit, Id};

use super::Data;
use crate::app::App;
use crate::session::Session;
use crate::text::l6::level_name;
use crate::text::l7::{criterion_name, l7, status_name};
use crate::text::texts;
use crate::theme;

/// The training levels (the MBB is certified by hand).
pub const LEVELS: [BeltLevel; 3] = [BeltLevel::Yellow, BeltLevel::Green, BeltLevel::Black];

pub(super) fn level_combo(ui: &mut egui::Ui, salt: &str, v: &mut BeltLevel) {
    egui::ComboBox::from_id_salt(salt)
        .selected_text(level_name(*v))
        .show_ui(ui, |ui| {
            for l in LEVELS {
                ui.selectable_value(v, l, level_name(l));
            }
        });
}

impl App {
    pub(in crate::app) fn program_training(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
    ) {
        let t = l7();
        ui.heading(t.nav_training);
        ui.weak(t.training_intro);
        self.waves_section(ui, s, d);
        ui.separator();
        self.classes_section(ui, s, d);
        ui.separator();
        self.criteria_section(ui, s, d);
        ui.separator();
        self.nominations_section(ui, s, d);
        if let Some(k) = d.tr.class(self.l6.l7.class).cloned() {
            ui.separator();
            self.selection_section(ui, s, d, &k);
            ui.separator();
            self.groups_section(ui, s, d, &k);
        }
    }

    fn waves_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let t = l7();
        ui.strong(t.waves);
        egui::Grid::new("ondas").striped(true).show(ui, |ui| {
            for h in [t.name, t.start, t.end, t.targets] {
                ui.strong(h);
            }
            ui.end_row();
            for w in &d.tr.waves {
                let r = formacao::Wave::entity_ref(w.id);
                self.text_field(ui, s, r, formacao::fields::NAME, false, 160.0);
                self.date_field(ui, s, r, formacao::fields::START);
                self.date_field(ui, s, r, formacao::fields::END);
                ui.horizontal(|ui| {
                    for l in LEVELS {
                        let mut n = w.targets.get(&(l, None)).copied().unwrap_or(0);
                        ui.label(level_name(l));
                        if ui
                            .add(egui::DragValue::new(&mut n).range(0..=10_000))
                            .changed()
                            && self.may_do(s, Action::ManageTraining, None)
                        {
                            let r = s.apply(vec![formacao::set_wave_target(w.id, l, None, n)]);
                            self.report(r);
                        }
                    }
                });
                ui.end_row();
            }
        });
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(self.l6.input("onda"))
                    .hint_text(t.new_wave_hint)
                    .desired_width(240.0),
            );
            if ui.button(texts().add).clicked() && self.may_do(s, Action::ManageTraining, None) {
                let name = self.l6.take_input("onda");
                match formacao::new_wave(d.program, s.new_id(), &name, None, None) {
                    Ok(e) => {
                        let r = s.apply(e);
                        self.report(r);
                    }
                    Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                }
            }
        });
    }

    fn classes_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let t = l7();
        ui.strong(t.classes);
        egui::Grid::new("turmas").striped(true).show(ui, |ui| {
            for h in [
                "",
                t.name,
                t.level,
                t.wave,
                t.start,
                t.end,
                t.place,
                t.seats,
                t.students,
                t.min_attendance,
                t.min_grade,
            ] {
                ui.strong(h);
            }
            ui.end_row();
            for k in &d.tr.classes {
                let r = formacao::Class::entity_ref(k.id);
                if ui.radio(self.l6.l7.class == Some(k.id), "").clicked() {
                    self.l6.l7.class = Some(k.id);
                    self.l6.l7.suggestion = None;
                }
                self.text_field(ui, s, r, formacao::fields::NAME, false, 140.0);
                ui.label(level_name(k.level));
                ui.label(d.tr.wave_name(k.wave));
                self.date_field(ui, s, r, formacao::fields::START);
                self.date_field(ui, s, r, formacao::fields::END);
                self.text_field(ui, s, r, formacao::fields::PLACE, false, 100.0);
                self.num_field(ui, s, r, formacao::fields::SEATS, 40.0);
                ui.label(formacao::students(&d.tr.noms, k.id).len().to_string());
                self.num_field(ui, s, r, formacao::fields::MIN_ATTENDANCE, 40.0);
                self.num_field(ui, s, r, formacao::fields::MIN_GRADE, 40.0);
                ui.end_row();
            }
        });
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(self.l6.input("turma"))
                    .hint_text(t.new_class_hint)
                    .desired_width(200.0),
            );
            level_combo(ui, "nivel_turma", &mut self.l6.l7.level);
            self.wave_combo(ui, d);
            ui.label(t.seats);
            ui.add(
                egui::TextEdit::singleline(self.l6.input("vagas"))
                    .hint_text("20")
                    .desired_width(40.0),
            );
            if ui.button(texts().add).clicked() && self.may_do(s, Action::ManageTraining, None) {
                let name = self.l6.take_input("turma");
                let seats = crate::app::screens::parse_number(&self.l6.take_input("vagas"))
                    .map_or(formacao::DEFAULT_SEATS, |x| x.max(1.0) as u32);
                let input = ClassInput {
                    wave: self.l6.l7.wave,
                    name,
                    level: self.l6.l7.level,
                    start: None,
                    end: None,
                    place: String::new(),
                    seats,
                };
                let id = s.new_id();
                match formacao::new_class(d.program, id, &input) {
                    Ok(e) => {
                        let r = s.apply(e);
                        if self.report(r).is_some() {
                            self.l6.l7.class = Some(id);
                        }
                    }
                    Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                }
            }
        });
        ui.weak(t.minimum_note);
    }

    fn wave_combo(&mut self, ui: &mut egui::Ui, d: &Data) {
        let t = l7();
        egui::ComboBox::from_id_salt("onda_escolhida")
            .selected_text(match self.l6.l7.wave {
                Some(w) => d.tr.wave_name(Some(w)),
                None => t.no_wave.to_string(),
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.l6.l7.wave, None, t.no_wave);
                for w in &d.tr.waves {
                    ui.selectable_value(&mut self.l6.l7.wave, Some(w.id), &w.name);
                }
            });
    }

    fn criteria_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let t = l7();
        ui.strong(t.criteria);
        ui.weak(t.criteria_intro);
        egui::Grid::new("criterios").striped(true).show(ui, |ui| {
            for h in [
                t.criterion,
                t.eliminatory,
                t.threshold,
                t.weight,
                t.functions,
                t.level,
            ] {
                ui.strong(h);
            }
            ui.end_row();
            for c in &d.tr.criteria {
                let r = formacao::Criterion::entity_ref(c.id);
                ui.label(criterion_name(c.kind));
                self.bool_field(ui, s, r, formacao::fields::ELIMINATORY, "");
                self.num_field(ui, s, r, formacao::fields::THRESHOLD, 50.0);
                self.num_field(ui, s, r, formacao::fields::WEIGHT, 40.0);
                if c.kind == CriterionKind::Function {
                    self.text_field(ui, s, r, formacao::fields::TEXT, false, 160.0);
                } else {
                    ui.label("");
                }
                ui.label(c.level.map_or(t.every_level, level_name));
                ui.end_row();
            }
        });
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt("tipo_criterio")
                .selected_text(criterion_name(self.l6.l7.criterion_kind))
                .show_ui(ui, |ui| {
                    for k in CriterionKind::ALL {
                        ui.selectable_value(&mut self.l6.l7.criterion_kind, k, criterion_name(k));
                    }
                });
            ui.checkbox(&mut self.l6.l7.criterion_elim, t.eliminatory);
            if ui.button(texts().add).clicked() && self.may_do(s, Action::ManageTraining, None) {
                let e = formacao::add_criterion(
                    d.program,
                    s.new_id(),
                    self.l6.l7.criterion_kind,
                    self.l6.l7.criterion_elim,
                    0.0,
                    "",
                    1.0,
                    None,
                );
                let r = s.apply(e);
                self.report(r);
            }
            if d.tr.criteria.is_empty()
                && ui.button(t.default_criteria).clicked()
                && self.may_do(s, Action::ManageTraining, None)
            {
                let e = formacao::default_criteria(d.program, s.ids());
                let r = s.apply(e);
                self.report(r);
            }
        });
    }

    fn nominations_section(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Data) {
        let t = l7();
        ui.strong(t.nominations);
        ui.weak(t.nominations_intro);
        egui::Grid::new("nova_indicacao")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label(t.person);
                if let Some(v) = Self::member_combo(ui, d, "indicado", self.l6.l7.person, false) {
                    self.l6.l7.person = v;
                }
                ui.end_row();
                ui.label(t.level);
                level_combo(ui, "nivel_indicacao", &mut self.l6.l7.level);
                ui.end_row();
                ui.label(t.wave);
                self.wave_combo(ui, d);
                ui.end_row();
                ui.label(t.manager);
                if let Some(v) = Self::member_combo(ui, d, "gestor", self.l6.l7.manager, false) {
                    self.l6.l7.manager = v;
                }
                ui.end_row();
                ui.label(t.opportunity);
                egui::ComboBox::from_id_salt("oportunidade_indicacao")
                    .selected_text(
                        self.l6
                            .l7
                            .opportunity
                            .and_then(|o| d.ideas.iter().find(|i| i.id == o))
                            .map_or_else(|| texts().nobody.to_string(), |i| i.title.clone()),
                    )
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.l6.l7.opportunity, None, texts().nobody);
                        for i in &d.ideas {
                            ui.selectable_value(&mut self.l6.l7.opportunity, Some(i.id), &i.title);
                        }
                    });
                ui.end_row();
                for (k, label) in [
                    ("justificativa", t.justification),
                    ("funcao", t.function),
                    ("tempo", t.tenure),
                    ("desempenho", t.rating),
                    ("horas", t.hours_available),
                ] {
                    ui.label(label);
                    ui.add(egui::TextEdit::singleline(self.l6.input(k)).desired_width(280.0));
                    ui.end_row();
                }
            });
        if ui.button(t.nominate).clicked() {
            let num = |x: String| crate::app::screens::parse_number(&x);
            let input = NominationInput {
                wave: self.l6.l7.wave,
                person: self.l6.l7.person,
                level: self.l6.l7.level,
                by: d.me,
                justification: self.l6.input("justificativa").clone(),
                opportunity: self.l6.l7.opportunity,
                manager: self.l6.l7.manager,
                tenure_months: num(self.l6.input("tempo").clone()),
                function: self.l6.input("funcao").clone(),
                rating: num(self.l6.input("desempenho").clone()),
                hours_available: num(self.l6.input("horas").clone()),
                date: Some(d.today),
            };
            match formacao::nominate(d.program, s.new_id(), &input, &d.tr.noms) {
                Ok(e) => {
                    let r = s.apply(e);
                    if self.report(r).is_some() {
                        for k in ["justificativa", "funcao", "tempo", "desempenho", "horas"] {
                            self.l6.take_input(k);
                        }
                        self.set_status(t.nominated, false);
                    }
                }
                Err(e) => self.set_status((texts().error)(&e.to_string()), true),
            }
        }
        let is_mbb = d.is_mbb();
        egui::Grid::new("indicacoes").striped(true).show(ui, |ui| {
            for h in [
                t.person,
                t.level,
                t.wave,
                t.manager,
                t.manager_ok,
                t.status,
                "",
            ] {
                ui.strong(h);
            }
            ui.end_row();
            for n in &d.tr.noms {
                ui.label(n.person.map_or_else(String::new, |p| d.person(p)));
                ui.label(level_name(n.level));
                ui.label(d.tr.wave_name(n.wave));
                ui.label(n.manager.map_or_else(String::new, |p| d.person(p)));
                ui.label(match n.manager_approval {
                    Some(true) => t.approved.to_string(),
                    Some(false) => (t.refused_because)(&n.manager_reason),
                    None => t.pending.to_string(),
                });
                ui.label(match n.position {
                    Some(p) if n.status == NominationStatus::Waiting => {
                        format!("{} ({p}º)", status_name(n.status))
                    }
                    _ => status_name(n.status).to_string(),
                });
                ui.horizontal(|ui| {
                    let mine = n.manager == Some(d.me) && n.person != Some(d.me);
                    if n.manager_approval.is_none() && (mine || is_mbb) {
                        if ui.small_button(t.approve).clicked() {
                            self.decide_nomination(s, d, n, true);
                        }
                        if ui.small_button(t.refuse).clicked() {
                            self.decide_nomination(s, d, n, false);
                        }
                    }
                    if n.active()
                        && (n.person == Some(d.me) || is_mbb)
                        && ui.small_button(t.withdraw).clicked()
                    {
                        let reason = self.l6.take_input("motivo_indicacao");
                        match formacao::withdraw(n, &reason) {
                            Ok(e) => {
                                let r = s.apply(e);
                                self.report(r);
                            }
                            Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                        }
                    }
                });
                ui.end_row();
            }
        });
        ui.add(
            egui::TextEdit::singleline(self.l6.input("motivo_indicacao"))
                .hint_text(t.reason_hint)
                .desired_width(320.0),
        );
    }

    fn decide_nomination(&mut self, s: &mut Session, d: &Data, n: &Nomination, approve: bool) {
        let reason = self.l6.take_input("motivo_indicacao");
        match formacao::manager_decide(n, d.me, d.is_mbb(), approve, &reason, d.today) {
            Ok(e) => {
                let r = s.apply(e);
                self.report(r);
            }
            Err(e) => self.set_status((texts().error)(&e.to_string()), true),
        }
    }

    fn selection_section(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
        k: &formacao::Class,
    ) {
        let t = l7();
        ui.strong((t.selection_of)(&k.name, k.seats));
        let wave = d.tr.waves.iter().find(|w| Some(w.id) == k.wave);
        let open = wave.map_or_else(Default::default, |w| {
            formacao::open_targets(w, &d.tr.noms, &d.members)
        });
        let ctx = formacao::CriteriaContext {
            members: &d.members,
            open_targets: open,
        };
        let ranked = formacao::rank(
            k,
            &d.tr.noms,
            &d.tr.criteria,
            &ctx,
            &d.tr.sessions,
            &d.loads,
        );
        let prop = formacao::proposal(&ranked, k.seats);
        egui::Grid::new("selecao").striped(true).show(ui, |ui| {
            for h in [t.person, t.proposal, t.score, t.capacity, t.why] {
                ui.strong(h);
            }
            ui.end_row();
            for r in &ranked {
                ui.label(r.person.map_or_else(String::new, |p| d.person(p)));
                let where_ = if prop.selected.contains(&r.nomination) {
                    t.to_select
                } else if prop.waiting.contains(&r.nomination) {
                    t.to_wait
                } else {
                    t.out
                };
                ui.label(where_);
                ui.label(format!("{:.0}", r.evaluation.score));
                let cap = (t.capacity_line)(
                    r.capacity.training_hours_week + r.capacity.project_hours_week,
                    r.capacity.free_hours,
                );
                if r.capacity.fits {
                    ui.label(cap);
                } else {
                    ui.colored_label(theme::WARN, cap);
                }
                let mut why: Vec<String> = r
                    .evaluation
                    .failures()
                    .map(|f| (t.failed_criterion)(criterion_name(f.kind)))
                    .collect();
                let nom = d.tr.noms.iter().find(|n| n.id == r.nomination);
                if !nom.is_some_and(Nomination::approved) {
                    why.push(t.no_manager_ok.to_string());
                }
                ui.label(why.join("; "));
                ui.end_row();
            }
        });
        ui.horizontal(|ui| {
            if ui.button(t.confirm_selection).clicked()
                && self.may_do(s, Action::SelectTrainees, None)
            {
                let e = formacao::confirm_selection(k.id, &prop, d.me, d.today);
                let r = s.apply(e);
                self.report(r);
            }
            if ui.button(t.promote).clicked() && self.may_do(s, Action::SelectTrainees, None) {
                let e = formacao::promote(k, &d.tr.noms, d.me, d.today);
                if e.is_empty() {
                    self.set_status(t.nobody_to_promote, false);
                } else {
                    let r = s.apply(e);
                    self.report(r);
                }
            }
        });
        let wl = formacao::waiting_list(&d.tr.noms, k.id);
        if !wl.is_empty() {
            ui.label((t.waiting_list)(
                &wl.iter()
                    .filter_map(|n| n.person)
                    .map(|p| d.person(p))
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }
        // Requests of certification projects for the committee.
        let reqs = formacao::certification_requests(&d.tr.noms, &d.opps, &d.assignments);
        for r in reqs
            .iter()
            .filter(|r| formacao::students(&d.tr.noms, k.id).contains(&r.person))
        {
            ui.colored_label(theme::WARN, (t.needs_project)(&d.person(r.person)));
        }
    }

    fn groups_section(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        d: &Data,
        k: &formacao::Class,
    ) {
        let t = l7();
        ui.strong(t.groups);
        ui.weak(t.groups_intro);
        let rules = formacao::group_rules(s.state(), d.program, k.level);
        let trainees = formacao::trainees(
            k,
            &d.tr.noms,
            &d.members,
            &d.assignments,
            &d.records,
            &d.units,
        );
        ui.horizontal(|ui| {
            ui.label((t.group_rules)(rules.group_size, rules.max_same_area));
            let pr = EntityRef::new(utsim_lss_core::Kind::Program, d.program);
            ui.label(t.group_size);
            self.num_field(ui, s, pr, formacao::fields::GROUP_SIZE, 36.0);
            ui.label(t.max_same_area);
            self.num_field(ui, s, pr, formacao::fields::MAX_SAME_AREA, 36.0);
        });
        if ui.button(t.suggest_groups).clicked() {
            self.l6.l7.suggestion = Some(teams::suggest(&trainees, &rules));
        }
        if let Some(sug) = self.l6.l7.suggestion.clone() {
            for (i, g) in sug.groups.iter().enumerate() {
                ui.label(format!(
                    "{} {}: {}",
                    t.group,
                    i + 1,
                    g.iter()
                        .map(|p| d.person(*p))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            for v in &sug.violations {
                ui.colored_label(theme::WARN, violation_text(v, d));
            }
            if ui.button(t.save_groups).clicked() && self.may_do(s, Action::ManageTraining, None) {
                let old = formacao::groups(s.state(), k.id);
                let e = formacao::save_groups(k.id, &old, &sug.groups, s.ids());
                let r = s.apply(e);
                if self.report(r).is_some() {
                    self.l6.l7.suggestion = None;
                }
            }
        }
        let gs = formacao::groups(s.state(), k.id);
        if gs.is_empty() {
            return;
        }
        let mut moves: Vec<Edit> = Vec::new();
        egui::Grid::new("grupos").striped(true).show(ui, |ui| {
            for g in &gs {
                ui.strong(&g.name);
                ui.vertical(|ui| {
                    for p in &g.members {
                        ui.horizontal(|ui| {
                            ui.label(d.person(*p));
                            egui::ComboBox::from_id_salt(("mover", *p))
                                .selected_text(t.move_to)
                                .show_ui(ui, |ui| {
                                    for o in gs.iter().filter(|o| o.id != g.id) {
                                        if ui.selectable_label(false, &o.name).clicked() {
                                            moves = formacao::move_to_group(&gs, *p, o.id);
                                        }
                                    }
                                });
                        });
                    }
                });
                ui.label(match (g.confirmed_by, g.confirmed_on) {
                    (Some(b), Some(on)) => (t.confirmed_by)(&d.person(b), &on.local()),
                    _ => t.not_confirmed.to_string(),
                });
                ui.end_row();
            }
        });
        if !moves.is_empty() && self.may_do(s, Action::ManageTraining, None) {
            let r = s.apply(moves);
            self.report(r);
        }
        let groups: Vec<Vec<Id>> = gs.iter().map(|g| g.members.clone()).collect();
        for v in teams::check(&groups, &trainees, &rules) {
            ui.colored_label(theme::WARN, violation_text(&v, d));
        }
        if ui.button(t.confirm_groups).clicked() && self.may_do(s, Action::ManageTraining, None) {
            let r = s.apply(formacao::confirm_groups(&gs, d.me, d.today));
            self.report(r);
        }
    }
}

pub(super) fn violation_text(v: &teams::Violation, d: &Data) -> String {
    let t = l7();
    match v {
        teams::Violation::AreaCap {
            group,
            area,
            count,
            max,
        } => (t.v_area)(group + 1, area, *count, *max),
        teams::Violation::Conflict { group, a, b } => {
            (t.v_conflict)(group + 1, &d.person(*a), &d.person(*b))
        }
        teams::Violation::Size {
            group,
            size,
            min,
            max,
        } => (t.v_size)(group + 1, *size, *min, *max),
        teams::Violation::NoProject { person } => (t.v_no_project)(&d.person(*person)),
        teams::Violation::NoChampion { person } => (t.v_no_champion)(&d.person(*person)),
    }
}
