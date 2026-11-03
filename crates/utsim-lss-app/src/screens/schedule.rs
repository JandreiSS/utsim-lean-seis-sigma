//! The schedule (section 1.6): tasks, dependencies with lag, milestones at the tollgates,
//! the calendar (worked weekdays and holidays), the Gantt with the critical path and the
//! baseline, and the probabilities from the PERT crate (classic and Monte Carlo).

use eframe::egui::{
    self, Align2, Color32, FontId, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, vec2,
};
use utsim_lss_core::calendar::{self, weekday};
use utsim_lss_core::date::Date;
use utsim_lss_core::op::{EntityRef, Kind};
use utsim_lss_core::schedule::{self, ScheduleResult, Warning};

use super::super::App;
use super::{parse_date, percent, show_number};
use crate::session::Session;
use crate::text::l1::{l1, weekday_short};
use crate::text::texts;
use crate::theme;

const CRITICAL: Color32 = Color32::from_rgb(205, 70, 60);

impl App {
    pub(in crate::app) fn schedule(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = texts();
        let tl = l1();
        ui.heading(t.nav_schedule);
        ui.weak(tl.schedule_intro);
        let p = schedule::project_ref(s.project());
        ui.horizontal(|ui| {
            ui.label(tl.schedule_start);
            self.date_field(ui, s, p, schedule::fields::START);
            ui.separator();
            ui.label(tl.workdays);
            let cal = calendar::of_project(s.state(), s.project());
            let mut days: Vec<u8> = (1..=7u8)
                .filter(|d| cal.workdays[usize::from(d - 1)])
                .collect();
            let mut changed = false;
            for d in 1..=7u8 {
                let mut on = days.contains(&d);
                if ui.checkbox(&mut on, weekday_short(d)).changed() {
                    if on {
                        days.push(d);
                    } else {
                        days.retain(|x| *x != d);
                    }
                    changed = true;
                }
            }
            if changed {
                let e = calendar::set_workdays(s.project(), &days);
                self.commit(s, vec![e]);
            }
        });
        self.holidays(ui, s);
        ui.separator();
        // Tasks.
        let tasks = schedule::tasks(s.state(), s.project());
        ui.horizontal(|ui| {
            let r = ui.add(
                egui::TextEdit::singleline(&mut self.l1.task_name)
                    .hint_text(tl.new_task_hint)
                    .desired_width(300.0),
            );
            let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if (ui.button(t.add).clicked() || enter) && !self.l1.task_name.trim().is_empty() {
                let e = schedule::new_task(
                    s.state(),
                    s.project(),
                    s.new_id(),
                    &self.l1.task_name,
                    5.0,
                    false,
                );
                if self.commit(s, e) {
                    self.l1.task_name.clear();
                }
            }
            if ui.button(tl.add_milestones).clicked() {
                let e = schedule::phase_milestones(s.state(), s.project(), s.ids());
                self.commit(s, e);
            }
            if ui.button(tl.freeze_baseline).clicked() {
                let e = schedule::freeze_baseline(s.state(), s.project(), s.today());
                if self.commit(s, e) {
                    self.set_status(tl.baseline_frozen, false);
                }
            }
        });
        let mut remove = None;
        egui::Grid::new("tarefas")
            .num_columns(9)
            .striped(true)
            .spacing([6.0, 4.0])
            .show(ui, |ui| {
                for h in [
                    "#",
                    tl.t_name,
                    tl.t_duration,
                    tl.t_three_points,
                    tl.t_preds,
                    tl.t_who,
                    tl.t_percent,
                    tl.t_target,
                    "",
                ] {
                    ui.strong(h);
                }
                ui.end_row();
                for (i, task) in tasks.iter().enumerate() {
                    let e = EntityRef::new(Kind::Task, task.id);
                    ui.label((i + 1).to_string());
                    self.text_field(ui, s, e, schedule::fields::NAME, false, 200.0);
                    if task.milestone {
                        ui.weak(tl.milestone);
                        ui.label("");
                    } else {
                        self.number_field(ui, s, e, schedule::fields::DURATION, 44.0);
                        ui.horizontal(|ui| {
                            for f in [
                                schedule::fields::OPTIMISTIC,
                                schedule::fields::LIKELY,
                                schedule::fields::PESSIMISTIC,
                            ] {
                                self.number_field(ui, s, e, f, 32.0);
                            }
                        });
                    }
                    self.preds_field(ui, s, task, &tasks);
                    self.person_field(ui, s, e, schedule::fields::WHO);
                    self.number_field(ui, s, e, schedule::fields::PERCENT, 36.0);
                    if task.milestone {
                        self.date_field(ui, s, e, schedule::fields::TARGET);
                    } else {
                        ui.label("");
                    }
                    if ui.small_button("✕").on_hover_text(t.delete).clicked() {
                        remove = Some(e);
                    }
                    ui.end_row();
                }
            });
        if let Some(e) = remove {
            let r = s.delete(e);
            self.report(r);
        }
        ui.weak(tl.preds_tip);
        ui.separator();
        let plan = schedule::plan(s.state(), s.project(), s.today());
        match schedule::compute(&plan) {
            Err(e) => {
                ui.colored_label(theme::WARN, e.to_string());
            }
            Ok(r) => {
                self.schedule_summary(ui, &r, &plan);
                gantt(ui, &r, s.today());
            }
        }
    }

    fn holidays(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let tl = l1();
        let list = calendar::holidays(s.state(), s.project());
        ui.collapsing((tl.holidays)(list.len()), |ui| {
            let mut remove = None;
            for (id, d, name) in &list {
                ui.horizontal(|ui| {
                    ui.label(d.format_br());
                    ui.label(name);
                    if ui.small_button("✕").clicked() {
                        remove = Some(*id);
                    }
                });
            }
            if let Some(id) = remove {
                let r = s.delete(EntityRef::new(Kind::Holiday, id));
                self.report(r);
            }
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.l1.holiday_date)
                        .hint_text(crate::text::texts().date_hint)
                        .desired_width(100.0),
                );
                ui.add(
                    egui::TextEdit::singleline(&mut self.l1.holiday_name)
                        .hint_text(tl.holiday_name_hint)
                        .desired_width(200.0),
                );
                if ui.button(crate::text::texts().add).clicked() {
                    match parse_date(&self.l1.holiday_date) {
                        Some(d) => {
                            let e = calendar::add_holiday(
                                s.project(),
                                s.new_id(),
                                d,
                                &self.l1.holiday_name,
                            );
                            if self.commit(s, e) {
                                self.l1.holiday_date.clear();
                                self.l1.holiday_name.clear();
                            }
                        }
                        None => self.set_status(crate::text::texts().bad_date, true),
                    }
                }
            });
            ui.horizontal(|ui| {
                if self.l1.holiday_year.is_empty() {
                    self.l1.holiday_year = s.today().year.to_string();
                }
                ui.add(egui::TextEdit::singleline(&mut self.l1.holiday_year).desired_width(50.0));
                if ui.button(tl.add_national_holidays).clicked()
                    && let Ok(y) = self.l1.holiday_year.trim().parse::<i32>()
                {
                    let have: Vec<Date> = list.iter().map(|h| h.1).collect();
                    let mut e = Vec::new();
                    for (d, n) in calendar::national_holidays_br(y) {
                        if !have.contains(&d) {
                            e.extend(calendar::add_holiday(s.project(), s.new_id(), d, n));
                        }
                    }
                    self.commit(s, e);
                }
            });
        });
    }

    fn preds_field(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        task: &schedule::Task,
        tasks: &[schedule::Task],
    ) {
        let key = (task.id, schedule::fields::PREDECESSORS.to_string());
        let shown = schedule::format_preds(task, tasks);
        let mut buf = self
            .l1
            .boxes
            .get(&key)
            .cloned()
            .unwrap_or_else(|| shown.clone());
        let r = ui.add(egui::TextEdit::singleline(&mut buf).desired_width(70.0));
        if r.has_focus() {
            self.l1.boxes.insert(key.clone(), buf.clone());
            self.editing = Some(EntityRef::new(Kind::Task, task.id));
        }
        if r.lost_focus() {
            self.l1.boxes.remove(&key);
            if buf.trim() != shown {
                match schedule::parse_preds(&buf, tasks) {
                    Ok(p) => {
                        let e = schedule::set_predecessors(task.id, &p);
                        self.commit(s, vec![e]);
                    }
                    Err(e) => self.set_status((crate::text::texts().error)(&e), true),
                }
            }
        }
    }

    fn schedule_summary(&mut self, ui: &mut egui::Ui, r: &ScheduleResult, plan: &schedule::Plan) {
        let tl = l1();
        ui.horizontal_wrapped(|ui| {
            ui.label((tl.schedule_finish)(
                &r.finish.format_br(),
                &show_number(r.duration),
            ));
            if r.sd > 0.0 {
                ui.label((tl.schedule_sd)(&show_number((r.sd * 10.0).round() / 10.0)));
            }
            if let (Some(d), Some(p)) = (r.deadline, r.deadline_probability) {
                ui.label((tl.deadline_probability)(&d.format_br(), &percent(p)));
            }
            if ui.button(tl.run_monte_carlo).clicked() {
                match schedule::monte_carlo(plan, 5000, 1) {
                    Ok(m) => self.l1.monte_carlo = Some(m),
                    Err(e) => self.set_status(e.to_string(), true),
                }
            }
        });
        if let Some(m) = &self.l1.monte_carlo {
            let ps: Vec<String> = m
                .percentiles
                .iter()
                .map(|(p, d)| format!("P{:.0} {}", p * 100.0, d.format_br()))
                .collect();
            ui.label((tl.monte_carlo_result)(m.samples, &ps.join(" · ")));
            if let Some(p) = m.deadline_probability {
                ui.label((tl.monte_carlo_deadline)(&percent(p)));
            }
        }
        for w in &r.warnings {
            let name = |id| {
                plan.tasks
                    .iter()
                    .find(|t| t.id == id)
                    .map(|t| t.name.clone())
                    .unwrap_or_default()
            };
            let msg = match w {
                Warning::NegativeLag(id) => (tl.w_negative_lag)(&name(*id)),
                Warning::UnknownPredecessor(id) => (tl.w_unknown_pred)(&name(*id)),
                Warning::BadThreePoint(id) => (tl.w_bad_three_point)(&name(*id)),
            };
            ui.colored_label(theme::WARN, msg);
        }
        for row in r.rows.iter().filter(|x| x.target.is_some()) {
            if let (Some(tgt), Some(p)) = (row.target, row.probability) {
                ui.label((tl.milestone_probability)(
                    &row.name,
                    &tgt.format_br(),
                    &percent(p),
                ));
            }
        }
    }
}

/// The Gantt: one bar per task from its start to its finish (critical in red), the baseline
/// as a thin bar under it, milestones as diamonds, today as a vertical line; one column per
/// calendar day, weekends shaded.
pub(super) fn gantt(ui: &mut egui::Ui, r: &ScheduleResult, today: Date) {
    let tl = l1();
    if r.rows.is_empty() {
        ui.weak(tl.no_tasks);
        return;
    }
    let mut first = r.start;
    let mut last = r.finish;
    for row in &r.rows {
        if let Some((bs, bf)) = row.baseline {
            first = first.min(bs);
            last = last.max(bf);
        }
        if let Some(tg) = row.target {
            last = last.max(tg);
        }
    }
    let days = (first.days_until(last) + 1).clamp(1, 800);
    let day_w = (900.0 / days as f32).clamp(4.0, 24.0);
    let label_w = 180.0;
    let row_h = 22.0;
    let header = 20.0;
    let size = vec2(
        label_w + day_w * days as f32 + 8.0,
        header + row_h * r.rows.len() as f32 + 4.0,
    );
    egui::ScrollArea::horizontal()
        .id_salt("gantt")
        .show(ui, |ui| {
            let (resp, painter) = ui.allocate_painter(size, Sense::hover());
            let origin = resp.rect.min;
            let x_of = |d: Date| origin.x + label_w + day_w * first.days_until(d) as f32;
            let text_color = ui.visuals().text_color();
            let weak = ui.visuals().weak_text_color();
            // Weekends and the month marks.
            for i in 0..days {
                let d = first.add_days(i);
                let x = x_of(d);
                if weekday(d) >= 6 {
                    painter.rect_filled(
                        Rect::from_min_size(
                            Pos2::new(x, origin.y + header),
                            vec2(day_w, row_h * r.rows.len() as f32),
                        ),
                        0.0,
                        ui.visuals().faint_bg_color,
                    );
                }
                if d.day == 1 || i == 0 {
                    painter.text(
                        Pos2::new(x + 2.0, origin.y + 2.0),
                        Align2::LEFT_TOP,
                        format!("{:02}/{}", d.month, d.year),
                        FontId::proportional(11.0),
                        weak,
                    );
                }
            }
            let bar = theme::accent(ui);
            for (k, row) in r.rows.iter().enumerate() {
                let y = origin.y + header + row_h * k as f32;
                painter.text(
                    Pos2::new(origin.x + 2.0, y + row_h / 2.0),
                    Align2::LEFT_CENTER,
                    &row.name,
                    FontId::proportional(12.0),
                    if row.critical { CRITICAL } else { text_color },
                );
                let color = if row.critical { CRITICAL } else { bar };
                if let Some((bs, bf)) = row.baseline {
                    let rect = Rect::from_min_max(
                        Pos2::new(x_of(bs), y + row_h - 6.0),
                        Pos2::new(x_of(bf) + day_w, y + row_h - 3.0),
                    );
                    painter.rect_filled(rect, 1.0, weak);
                }
                if row.milestone {
                    let c = Pos2::new(x_of(row.finish) + day_w / 2.0, y + row_h / 2.0 - 2.0);
                    let pts = vec![
                        c + vec2(0.0, -6.0),
                        c + vec2(6.0, 0.0),
                        c + vec2(0.0, 6.0),
                        c + vec2(-6.0, 0.0),
                    ];
                    painter.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
                    if let Some(tg) = row.target {
                        let x = x_of(tg) + day_w / 2.0;
                        painter.vline(
                            x,
                            (y + 2.0)..=(y + row_h - 2.0),
                            Stroke::new(1.5, theme::WARN),
                        );
                    }
                } else {
                    let rect = Rect::from_min_max(
                        Pos2::new(x_of(row.start), y + 4.0),
                        Pos2::new(x_of(row.finish) + day_w, y + row_h - 8.0),
                    );
                    painter.rect_stroke(rect, 2.0, Stroke::new(1.0, color), StrokeKind::Inside);
                    let mut done = rect;
                    done.set_width(rect.width() * (row.percent / 100.0) as f32);
                    painter.rect_filled(done, 2.0, color.gamma_multiply(0.8));
                    if row.percent < 100.0 {
                        let mut rest = rect;
                        rest.min.x = done.max.x;
                        painter.rect_filled(rest, 2.0, color.gamma_multiply(0.35));
                    }
                }
            }
            if today >= first && today <= last {
                let x = x_of(today) + day_w / 2.0;
                painter.vline(
                    x,
                    origin.y..=(origin.y + size.y),
                    Stroke::new(1.0, theme::WARN),
                );
            }
        });
    ui.horizontal(|ui| {
        ui.label(RichText::new("■").color(CRITICAL));
        ui.weak(tl.legend_critical);
        ui.label(RichText::new("▬").color(ui.visuals().weak_text_color()));
        ui.weak(tl.legend_baseline);
        ui.label(RichText::new("│").color(theme::WARN));
        ui.weak(tl.legend_today);
    });
}
