//! The control plan in operation (P5, `docs/plano_lean_seis_sigma.md`, 3.5): under the
//! table of the control plan, where each line is read on the twin, its frozen limits, the
//! response signal and the saving it measures; "Ler agora" reads the panel or the
//! recordings in a worker thread and `utsim_lss_core::controle` does the rest (phase II,
//! cards, realised value, data set of origin Gêmeo). Drawing only.

use crate::text::LocalDate;
use std::collections::BTreeMap;
use std::sync::mpsc::{self, Receiver};

use eframe::egui;
use egui_plot::{HLine, Line, Plot, PlotPoints, Points};
use serde_json::Value;
use utsim_lss_core::controle::{self, Better, LimitsFrom, Reading, TwinData, Watch, gemeo};
use utsim_lss_core::savings::{self, Month};
use utsim_lss_core::tools::{self, Deliverable, Tool, item_ref, plans};
use utsim_lss_core::{Id as Uid, dados};

use super::App;
use crate::session::Session;
use crate::text::l11::l11;
use crate::text::texts;
use crate::theme;

type Read = Result<TwinData, String>;

/// State of the monitoring section.
#[derive(Default)]
pub struct CtlUi {
    job: Option<(Uid, Receiver<Read>)>,
    /// The last data read, for the control plan it was read for.
    data: Option<(Uid, TwinData)>,
    readings: BTreeMap<Uid, Result<Reading, String>>,
    auto: bool,
    /// Screen time of the last reading, for the reading every minute.
    last: Option<f64>,
    month: String,
}

fn num(x: f64) -> String {
    super::screens::show_number((x * 1000.0).round() / 1000.0)
}

impl App {
    /// Reads the twin of a control plan now, in this thread (tests and tools; the button
    /// uses a worker thread). The number of cards opened, or the error.
    pub fn read_control_plan(&mut self, del: Uid) -> Result<usize, String> {
        let s = self.session.as_ref().ok_or("sem projeto")?;
        let address = controle::twin_address(s.state(), del);
        let data = gemeo::read(&address)?;
        let mut s = self.session.take().ok_or("sem projeto")?;
        let n = self.apply_twin(&mut s, del, data);
        self.session = Some(s);
        Ok(n)
    }

    /// The reading of a line after "Ler agora".
    pub fn control_reading(&self, line: Uid) -> Option<&Result<Reading, String>> {
        self.ctl.readings.get(&line)
    }

    /// Keeps the series read for a line as a data set of origin Gêmeo; its file name.
    pub fn keep_twin_dataset(&mut self, del: Uid, line: Uid) -> Option<String> {
        let mut s = self.session.take()?;
        let r = self.keep_dataset(&mut s, del, line);
        self.session = Some(s);
        r
    }

    /// Enters the realised value of the reading of a line in a month (`AAAA-MM`).
    pub fn book_realized(&mut self, del: Uid, line: Uid, month: &str) -> bool {
        let Some(mut s) = self.session.take() else {
            return false;
        };
        let ok = self.book(&mut s, del, line, month);
        self.session = Some(s);
        ok
    }

    fn watch(s: &Session, del: Uid, line: Uid) -> Option<Watch> {
        controle::watches(s.state(), del)
            .into_iter()
            .find(|w| w.line == line)
    }

    /// Checks every line against the data and commits the cards, the time checked and the
    /// limits taken from the model; the number of cards opened.
    fn apply_twin(&mut self, s: &mut Session, del: Uid, data: TwinData) -> usize {
        let mut edits = Vec::new();
        let mut cards = 0;
        let mut checked = 0;
        self.ctl.readings.clear();
        for w in controle::watches(s.state(), del) {
            if w.source.is_empty() {
                continue;
            }
            let r = controle::check(&w, &data);
            if let Ok(r) = &r {
                checked += 1;
                let card = s.new_id();
                let e = controle::reading_edits(s.state(), s.project(), &w, r, card);
                if e.iter().any(|e| e.entity.id == card) {
                    cards += 1;
                }
                edits.extend(e);
            }
            self.ctl.readings.insert(w.line, r);
        }
        self.ctl.data = Some((del, data));
        if self.commit(s, edits) {
            self.set_status((l11().read_done)(checked, cards), false);
        }
        cards
    }

    fn keep_dataset(&mut self, s: &mut Session, del: Uid, line: Uid) -> Option<String> {
        let w = Self::watch(s, del, line)?;
        let Some(Ok(r)) = self.ctl.readings.get(&line).cloned() else {
            return None;
        };
        let data = self.ctl.data.as_ref().map(|(_, d)| d.clone())?;
        let csv = controle::csv(&w, &r);
        match s.folder().add_data(csv.as_bytes(), "csv") {
            Ok((file, hash)) => {
                let input = controle::dataset(&w, &r, &data, &file, &hash, s.today());
                let e = dados::register(s.project(), s.new_id(), &input);
                if self.commit(s, e) {
                    self.set_status(l11().dataset_kept, false);
                    return Some(file);
                }
                None
            }
            Err(e) => {
                self.set_status((texts().error)(&e.to_string()), true);
                None
            }
        }
    }

    fn book(&mut self, s: &mut Session, del: Uid, line: Uid, month: &str) -> bool {
        let t = l11();
        let Ok(m) = month.trim().parse::<Month>() else {
            self.set_status(t.bad_month, true);
            return false;
        };
        let (Some(w), Some(Ok(r))) = (
            Self::watch(s, del, line),
            self.ctl.readings.get(&line).cloned(),
        ) else {
            return false;
        };
        match controle::realized_edits(s.state(), &w, &r, m, s.new_id()) {
            Some(e) => {
                let ok = self.commit(s, e);
                if ok {
                    self.set_status((t.booked)(&m.local()), false);
                }
                ok
            }
            None => {
                self.set_status(t.no_gain_link, true);
                false
            }
        }
    }

    /// Collects the reading of the worker thread.
    fn poll_twin(&mut self, s: &mut Session) {
        let Some((del, rx)) = &self.ctl.job else {
            return;
        };
        let del = *del;
        let out = match rx.try_recv() {
            Ok(o) => o,
            Err(mpsc::TryRecvError::Empty) => return,
            Err(mpsc::TryRecvError::Disconnected) => Err("a leitura parou".into()),
        };
        self.ctl.job = None;
        match out {
            Ok(d) => {
                self.apply_twin(s, del, d);
            }
            Err(e) => self.set_status((texts().error)(&e), true),
        }
    }

    fn start_read(&mut self, ui: &egui::Ui, s: &Session, del: Uid) {
        if self.ctl.job.is_some() {
            return;
        }
        let address = controle::twin_address(s.state(), del);
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(gemeo::read(&address));
        });
        self.ctl.job = Some((del, rx));
        self.ctl.last = Some(ui.ctx().input(|i| i.time));
    }

    /// The monitoring section under the control plan table.
    pub(super) fn control_monitor(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        let t = l11();
        self.poll_twin(s);
        if self.ctl.data.as_ref().is_some_and(|(d, _)| *d != del) {
            self.ctl.data = None;
            self.ctl.readings.clear();
        }
        if self.ctl.month.is_empty() {
            self.ctl.month = Month::of(s.today()).to_string();
        }
        ui.add_space(10.0);
        ui.separator();
        ui.heading(t.title);
        ui.scope(|ui| {
            ui.set_max_width(900.0);
            ui.weak(t.intro);
        });
        let busy = self.ctl.job.is_some();
        ui.horizontal_wrapped(|ui| {
            ui.label(t.address);
            self.text_field(
                ui,
                s,
                Deliverable::entity_ref(del),
                plans::fields::TWIN,
                false,
                320.0,
            );
            if ui
                .add_enabled(!busy, egui::Button::new(t.read_now))
                .clicked()
            {
                self.start_read(ui, s, del);
            }
            ui.checkbox(&mut self.ctl.auto, t.auto);
            if busy {
                ui.spinner();
                ui.weak(t.reading);
            }
        });
        ui.weak(t.address_hint);
        if busy {
            ui.ctx().request_repaint();
        } else if self.ctl.auto {
            let now = ui.ctx().input(|i| i.time);
            if self.ctl.last.is_none_or(|l| now - l >= 60.0) {
                self.start_read(ui, s, del);
            }
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_secs(5));
        }
        if let Some((_, d)) = &self.ctl.data {
            ui.weak(format!(
                "{} · {}",
                (t.read_from)(&d.origin, &d.model),
                (t.sources_seen)(d.series.len())
            ));
        }
        let lines = controle::watches(s.state(), del);
        if lines.is_empty() {
            ui.weak(t.no_lines);
            return;
        }
        let signals: Vec<(Value, String)> = std::iter::once((Value::from(""), "-".to_string()))
            .chain(
                tools::of_tool(s.state(), s.project(), Tool::ResponsePlan)
                    .iter()
                    .flat_map(|d| tools::items(s.state(), d.id, plans::RESPONSE_ITEM))
                    .map(|i| {
                        let name = i.text(plans::fields::SIGNAL);
                        (
                            Value::from(i.id.to_string()),
                            if name.trim().is_empty() {
                                "?".into()
                            } else {
                                name
                            },
                        )
                    }),
            )
            .collect();
        let gains: Vec<(Value, String)> = std::iter::once((Value::from(""), "-".to_string()))
            .chain(
                savings::plans(s.state(), s.project())
                    .into_iter()
                    .map(|p| (Value::from(p.saving.id.to_string()), p.saving.description)),
            )
            .collect();
        let better = [
            // Unset means higher is better.
            (Value::from(""), t.higher.to_string()),
            (Value::from(Better::Lower.as_str()), t.lower.to_string()),
        ];
        egui::Grid::new(("monitor", del))
            .striped(true)
            .show(ui, |ui| {
                for h in [
                    t.c_line,
                    t.c_source,
                    "LIC",
                    "LC",
                    "LSC",
                    "",
                    t.c_response,
                    t.c_saving,
                    t.c_baseline,
                    t.c_per_unit,
                    t.c_better,
                ] {
                    ui.strong(h);
                }
                ui.end_row();
                for w in &lines {
                    let r = item_ref(w.line);
                    ui.label(w.name());
                    self.text_field(ui, s, r, plans::fields::SOURCE, false, 150.0);
                    for f in [
                        plans::fields::LCL,
                        plans::fields::CENTER,
                        plans::fields::UCL,
                    ] {
                        self.num_field(ui, s, r, f, 64.0);
                    }
                    ui.horizontal(|ui| {
                        let model = self
                            .ctl
                            .data
                            .as_ref()
                            .and_then(|(_, d)| controle::limits_from_model(d, &w.source));
                        if ui
                            .add_enabled(model.is_some(), egui::Button::new(t.limits_model).small())
                            .clicked()
                            && let Some(l) = model
                        {
                            self.commit(s, controle::limits_edits(w.line, l, LimitsFrom::Model));
                        }
                        let series: Option<Vec<f64>> = match self.ctl.readings.get(&w.line) {
                            Some(Ok(rd)) => Some(rd.points.iter().map(|p| p.value).collect()),
                            _ => None,
                        };
                        if ui
                            .add_enabled(
                                series.is_some(),
                                egui::Button::new(t.limits_series).small(),
                            )
                            .clicked()
                        {
                            let r = controle::limits_from_values(&series.unwrap_or_default())
                                .map(|l| controle::limits_edits(w.line, l, LimitsFrom::Data));
                            self.commit_rule(s, r);
                        }
                    });
                    self.combo_field(ui, s, r, plans::fields::RESPONSE, &signals, 150.0);
                    self.combo_field(ui, s, r, plans::fields::SAVING, &gains, 150.0);
                    self.num_field(ui, s, r, plans::fields::BASELINE, 70.0);
                    self.num_field(ui, s, r, plans::fields::PER_UNIT, 70.0);
                    self.combo_field(ui, s, r, plans::fields::BETTER, &better, 70.0);
                    ui.end_row();
                }
            });
        ui.horizontal(|ui| {
            ui.label(t.month);
            ui.add(egui::TextEdit::singleline(&mut self.ctl.month).desired_width(70.0));
        });
        for w in &lines {
            let Some(r) = self.ctl.readings.get(&w.line).cloned() else {
                continue;
            };
            ui.add_space(6.0);
            ui.strong(format!("{} · {}", w.name(), w.source));
            match r {
                Err(e) => {
                    ui.colored_label(theme::WARN, e);
                }
                Ok(r) => self.reading_view(ui, s, del, w, &r),
            }
        }
    }

    fn reading_view(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        del: Uid,
        w: &Watch,
        r: &Reading,
    ) {
        let t = l11();
        let from = match r.limits_from {
            LimitsFrom::Manual => t.from_manual,
            LimitsFrom::Data => t.from_data,
            LimitsFrom::Model => t.from_model,
        };
        let line = (t.summary)(r.points.len(), &num(r.mean), r.signals.len(), r.new.len());
        let colour = if r.new.is_empty() {
            ui.visuals().text_color()
        } else {
            theme::WARN
        };
        ui.colored_label(colour, format!("{line} · {}", (t.limits_from)(from)));
        let pts: Vec<[f64; 2]> = r.points.iter().map(|p| [p.t, p.value]).collect();
        let out: Vec<[f64; 2]> = r
            .signals
            .iter()
            .map(|(i, _)| [r.points[*i].t, r.points[*i].value])
            .collect();
        Plot::new(("carta_gemeo", w.line))
            .height(150.0)
            .allow_scroll(false)
            .show(ui, |p| {
                p.line(Line::new("", PlotPoints::new(pts)).color(crate::app::screens::inicio::COR));
                p.points(
                    Points::new("", PlotPoints::new(out))
                        .radius(4.0)
                        .color(theme::WARN),
                );
                let red = egui::Color32::from_rgb(178, 31, 31);
                p.hline(HLine::new("LIC", r.limits.1).color(red));
                p.hline(HLine::new("LC", r.limits.0).color(egui::Color32::GRAY));
                p.hline(HLine::new("LSC", r.limits.2).color(red));
            });
        for (i, rules) in &r.signals {
            let p = r.points[*i];
            ui.weak(format!("t = {}: {} · {rules}", num(p.t), num(p.value)));
        }
        if !r.alerts.is_empty() {
            ui.strong(t.alerts);
            for a in &r.alerts {
                ui.weak(a);
            }
        }
        ui.horizontal_wrapped(|ui| {
            if ui.button(t.keep_dataset).clicked() {
                self.keep_dataset(s, del, w.line);
            }
            match controle::realized_value(w, r) {
                Some(v) if w.saving.is_some() => {
                    ui.label((t.realized)(&num(v)));
                    if ui.button(t.book_month).clicked() {
                        let m = self.ctl.month.clone();
                        self.book(s, del, w.line, &m);
                    }
                }
                _ => {
                    ui.weak(t.no_gain_link);
                }
            }
        });
    }
}
