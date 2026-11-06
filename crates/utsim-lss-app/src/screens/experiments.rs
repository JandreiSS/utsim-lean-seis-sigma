//! The design of experiments of a project (P3, `docs/plano_lean_seis_sigma.md`, 2.4 and
//! 3.4): the factors with their levels, the full factorial or a fraction with the alias
//! structure, replicates, center points and the seed; the collection worksheet saved in
//! `dados/`; and, with a linked model, the experiment run on the simulator (a worker
//! thread) and saved as a data set of origin Simulado. The analysis is the recipe
//! "Experimento fatorial 2^k" of the analyses screen.

use std::sync::mpsc::{self, Receiver};

use eframe::egui;
use utsim_lss_core::dados;
use utsim_lss_core::experimentos::{self, ExperimentSpec, FactorSpec};
use utsim_lss_core::simulacao;

use super::super::App;
use crate::session::Session;
use crate::text::l10::l10;
use crate::text::texts;
use crate::theme;

/// What the simulated experiment sends back: the data set's CSV, or the error.
type Ran = Result<String, String>;

/// The experiments screen's state.
pub struct ExpUi {
    pub spec: ExperimentSpec,
    pub seed: String,
    pub replicates: String,
    pub centers: String,
    job: Option<Receiver<Ran>>,
}

impl Default for ExpUi {
    fn default() -> Self {
        let f = |n: &str| FactorSpec {
            name: n.to_string(),
            ..FactorSpec::default()
        };
        Self {
            spec: ExperimentSpec {
                factors: vec![f("A"), f("B")],
                response: "resposta".into(),
                ..ExperimentSpec::default()
            },
            seed: "1".into(),
            replicates: "1".into(),
            centers: "0".into(),
            job: None,
        }
    }
}

impl ExpUi {
    /// The spec with the numbers of the text boxes.
    fn read(&self) -> ExperimentSpec {
        let n = |s: &str, d: usize| s.trim().parse::<usize>().unwrap_or(d);
        ExperimentSpec {
            seed: self.seed.trim().parse().unwrap_or(1),
            replicates: n(&self.replicates, 1).max(1),
            center_points: n(&self.centers, 0),
            ..self.spec.clone()
        }
    }
}

impl App {
    /// The experiment described on the screen (tests and tools fill it this way).
    pub fn set_experiment(&mut self, spec: ExperimentSpec) {
        self.exp.seed = spec.seed.to_string();
        self.exp.replicates = spec.replicates.max(1).to_string();
        self.exp.centers = spec.center_points.to_string();
        self.exp.spec = spec;
    }

    /// Whether the simulated experiment is still running.
    pub fn experiment_running(&self) -> bool {
        self.exp.job.is_some()
    }

    /// Saves the collection worksheet of the experiment on the screen in `dados/` (the
    /// button); its file name.
    pub fn save_experiment_sheet(&mut self) -> Option<String> {
        let mut s = self.session.take()?;
        let r = self.save_sheet(&mut s);
        self.session = Some(s);
        r
    }

    /// Starts the experiment on the linked model (the button); false when it cannot.
    pub fn start_experiment_on_model(&mut self) -> bool {
        let Some(s) = self.session.take() else {
            return false;
        };
        let ok = self.start_on_model(&s);
        self.session = Some(s);
        ok
    }

    fn save_sheet(&mut self, s: &mut Session) -> Option<String> {
        let spec = self.exp.read();
        let d = match spec.design() {
            Ok(d) => d,
            Err(e) => {
                self.set_status(e.to_string(), true);
                return None;
            }
        };
        let head = format!(
            "# planejamento: {}\n",
            serde_json::to_string(&spec).unwrap_or_default()
        );
        let csv = format!("{head}{}", experimentos::worksheet(&spec, &d));
        match s.folder().add_data(csv.as_bytes(), "csv") {
            Ok((file, _)) => {
                let path = s.folder().data_path(&file);
                self.set_status((l10().exp_sheet_saved)(&path.display().to_string()), false);
                Some(file)
            }
            Err(e) => {
                self.set_status((texts().error)(&e.to_string()), true);
                None
            }
        }
    }

    fn start_on_model(&mut self, s: &Session) -> bool {
        if self.exp.job.is_some() {
            return false;
        }
        self.load_linked(s);
        let Some(ld) = self.l4.loaded.clone() else {
            self.set_status(l10().exp_no_model, true);
            return false;
        };
        let spec = self.exp.read();
        let points = match spec
            .design()
            .and_then(|d| experimentos::scenarios(&spec, &d))
        {
            Ok(p) => p,
            Err(e) => {
                self.set_status(e.to_string(), true);
                return false;
            }
        };
        let (tx, rx) = mpsc::channel();
        let model = ld.model;
        std::thread::spawn(move || {
            let scen: Vec<_> = points.iter().map(|(_, s)| s.clone()).collect();
            let out = utsim_model::experiment::run_scenarios(&model, &scen, &[], &|_, _| true)
                .map_err(|e| e.to_string())
                .map(|res| {
                    let mut measures: Vec<String> =
                        res.iter().flat_map(|r| r.summary.keys().cloned()).collect();
                    measures.sort();
                    measures.dedup();
                    experimentos::simulated_csv(&spec, &points, &res, &measures)
                });
            let _ = tx.send(out);
        });
        self.exp.job = Some(rx);
        true
    }

    /// Collects the simulated experiment when it is done.
    fn poll_experiment(&mut self, s: &mut Session) {
        let Some(rx) = &self.exp.job else {
            return;
        };
        let out = match rx.try_recv() {
            Ok(o) => o,
            Err(mpsc::TryRecvError::Empty) => return,
            Err(mpsc::TryRecvError::Disconnected) => Err("o experimento parou".into()),
        };
        self.exp.job = None;
        let csv = match out {
            Ok(c) => c,
            Err(e) => {
                self.set_status((texts().error)(&e), true);
                return;
            }
        };
        let Some(ld) = self.l4.loaded.clone() else {
            return;
        };
        let spec = self.exp.read();
        let name = if spec.name.trim().is_empty() {
            l10().exp_title.to_string()
        } else {
            spec.name.clone()
        };
        match s.folder().add_data(csv.as_bytes(), "csv") {
            Ok((file, hash)) => {
                let input = simulacao::simulated_dataset(
                    &name,
                    &ld.model,
                    &ld.hash,
                    "",
                    &file,
                    &hash,
                    &csv,
                    s.today(),
                );
                let e = dados::register(s.project(), s.new_id(), &input);
                if self.commit(s, e) {
                    self.set_status(l10().exp_saved_sim, false);
                }
            }
            Err(e) => self.set_status((texts().error)(&e.to_string()), true),
        }
    }

    pub(in crate::app) fn experiments_screen(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = l10();
        self.poll_experiment(s);
        if self.exp.job.is_some() {
            ui.ctx().request_repaint();
        }
        ui.heading(t.exp_title);
        ui.weak(t.exp_intro);
        ui.horizontal(|ui| {
            ui.label(t.exp_name);
            ui.add(egui::TextEdit::singleline(&mut self.exp.spec.name).desired_width(280.0));
        });
        let mut remove = None;
        egui::Grid::new("fatores").striped(true).show(ui, |ui| {
            for h in [
                "",
                t.exp_factor,
                t.exp_low,
                t.exp_high,
                t.exp_center,
                t.exp_key,
                "",
            ] {
                ui.strong(h);
            }
            ui.end_row();
            let n = self.exp.spec.factors.len();
            for (i, f) in self.exp.spec.factors.iter_mut().enumerate() {
                ui.label(utsim_stats::doe::letter(i).to_string());
                ui.add(egui::TextEdit::singleline(&mut f.name).desired_width(120.0));
                ui.add(egui::TextEdit::singleline(&mut f.low).desired_width(110.0));
                ui.add(egui::TextEdit::singleline(&mut f.high).desired_width(110.0));
                ui.add(egui::TextEdit::singleline(&mut f.center).desired_width(90.0));
                ui.add(egui::TextEdit::singleline(&mut f.key).desired_width(150.0));
                if n > 2 && ui.small_button("✕").clicked() {
                    remove = Some(i);
                }
                ui.end_row();
            }
        });
        if let Some(i) = remove {
            self.exp.spec.factors.remove(i);
        }
        let k = self.exp.spec.factors.len();
        if k < experimentos::MAX_FACTORS && ui.button(t.exp_add_factor).clicked() {
            self.exp.spec.factors.push(FactorSpec {
                name: utsim_stats::doe::letter(k).to_string(),
                ..FactorSpec::default()
            });
        }
        let k = self.exp.spec.factors.len();
        let fractions = ExperimentSpec::fractions(k);
        if !fractions.contains(&self.exp.spec.fraction) {
            self.exp.spec.fraction = 0;
        }
        let label = |p: usize| {
            if p == 0 {
                (t.exp_full)(1 << k)
            } else {
                (t.exp_frac)(p, 1 << (k - p))
            }
        };
        ui.horizontal_wrapped(|ui| {
            ui.label(t.exp_fraction);
            egui::ComboBox::from_id_salt("fracao")
                .selected_text(label(self.exp.spec.fraction))
                .show_ui(ui, |ui| {
                    for p in &fractions {
                        ui.selectable_value(&mut self.exp.spec.fraction, *p, label(*p));
                    }
                });
            for (l, v) in [
                (t.exp_replicates, &mut self.exp.replicates),
                (t.exp_centers, &mut self.exp.centers),
                (t.exp_seed, &mut self.exp.seed),
            ] {
                ui.label(l);
                ui.add(egui::TextEdit::singleline(v).desired_width(60.0));
            }
            ui.label(t.exp_response);
            ui.add(egui::TextEdit::singleline(&mut self.exp.spec.response).desired_width(120.0));
        });
        let spec = self.exp.read();
        match spec.design() {
            Ok(d) => {
                ui.label((t.exp_runs)(d.runs.len()));
                ui.strong(t.exp_aliases);
                for l in experimentos::alias_lines(&d) {
                    ui.monospace(l);
                }
            }
            Err(e) => {
                ui.colored_label(theme::WARN, e.to_string());
            }
        }
        ui.horizontal(|ui| {
            if ui.button(t.exp_save_sheet).clicked() {
                self.save_sheet(s);
            }
            let busy = self.exp.job.is_some();
            if ui
                .add_enabled(!busy, egui::Button::new(t.exp_run_model))
                .clicked()
            {
                self.start_on_model(s);
            }
            if busy {
                ui.spinner();
                ui.weak(t.exp_running);
            }
        });
        ui.weak(t.exp_analyse_hint);
    }
}
