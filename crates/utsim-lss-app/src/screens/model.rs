//! The UTSim model of a project (stage L4, `docs/plano_lean_seis_sigma.md`, 3.1 to 3.4):
//! the link with the hash and what depends on an old version, the process chart and the
//! diagrams, the SIPOC generated as a deliverable, replications as labelled simulated data
//! sets, the bottleneck and the shifting bottleneck, studies of scenarios compared to the
//! baseline by paired difference (saved as `estudo.json`), the observed critical path and
//! "Abrir no UTSim". The simulations run on a worker thread; the screen keeps drawing.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver};

use eframe::egui;
use serde_json::Value;
use utsim_lss_core::dados;
use utsim_lss_core::domain::SavingCategory;
use utsim_lss_core::id::Id;
use utsim_lss_core::simulacao::{
    self, AnalysisKind, Bottleneck, GainInput, Observed, Share, StudyFile,
};
use utsim_model::Model;
use utsim_model::experiment::Scenario;

use super::super::App;
use crate::session::Session;
use crate::text::l4::l4;
use crate::text::l10::l10;
use crate::text::texts;
use crate::theme;

/// The model read from the link: its text, the parsed model and the hash.
#[derive(Clone)]
pub struct Loaded {
    pub path: PathBuf,
    pub text: String,
    pub hash: String,
    pub model: Model,
    /// When the file was last changed (read again when it changes).
    pub modified: Option<std::time::SystemTime>,
}

/// What a background job hands back.
pub enum JobOut {
    Runs {
        summary: Vec<(String, f64, f64)>,
        per_rep: BTreeMap<String, Vec<f64>>,
        bottleneck: Bottleneck,
    },
    Shifting(Vec<Share>),
    Study(Box<StudyFile>),
    Observed(Box<Observed>),
    /// The VSM page and the processes; whether to make the deliverable too.
    Vsm(String, Vec<utsim_lss_core::tools::vsm::ModelProcess>, bool),
    Spaghetti(String, Vec<simulacao::Travel>),
    Failed(String),
}

/// A simulation running on a worker thread.
pub struct Job {
    pub label: String,
    rx: Receiver<JobOut>,
}

/// The KPIs of the last replications (name, mean, half-width), their values per
/// replication and the bottleneck.
pub type Runs = (
    Vec<(String, f64, f64)>,
    BTreeMap<String, Vec<f64>>,
    Bottleneck,
);

/// The model screen's state between frames.
#[derive(Default)]
pub struct L4Ui {
    pub path_input: String,
    pub copy: bool,
    pub loaded: Option<Loaded>,
    pub error: Option<String>,
    pub job: Option<Job>,
    pub runs: Option<Runs>,
    pub shares: Option<Vec<Share>>,
    pub study: Option<StudyFile>,
    pub observed: Option<Observed>,
    pub keep: Vec<String>,
    pub scenarios: Vec<(String, String)>,
    pub measures: Vec<String>,
    pub minimums: BTreeMap<String, String>,
    pub study_name: String,
    pub from: String,
    pub to: String,
    /// The form "Lançar ganho estimado" of a saved study.
    pub gain: Option<GainForm>,
    /// A VSM or spaghetti result waiting for the session (pages and edits).
    pub pending: Option<JobOut>,
    /// The distances of the last spaghetti diagram.
    pub travels: Vec<simulacao::Travel>,
}

/// The form that turns a difference of a saved study into an estimated saving.
pub struct GainForm {
    pub study: Id,
    pub file: StudyFile,
    pub diff: usize,
    pub description: String,
    pub value: String,
    pub periods: String,
    pub higher: bool,
    pub conservative: bool,
    pub soft: bool,
}

impl L4Ui {
    fn spawn(&mut self, label: String, work: impl FnOnce() -> JobOut + Send + 'static) {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(work());
        });
        self.job = Some(Job { label, rx });
    }
}

/// The replications of a model, summarized for the screen.
fn runs_job(m: &Model) -> JobOut {
    match m.run_replications() {
        Ok(reports) => {
            let stats = utsim_model::analysis::object_stats(m, &reports);
            let per_rep = utsim_model::analysis::per_replication(&reports);
            JobOut::Runs {
                summary: per_rep
                    .iter()
                    .map(|(k, v)| {
                        let su = utsim_core::stats::Summary::from_samples(v);
                        (k.clone(), su.mean, su.half_width)
                    })
                    .collect(),
                per_rep,
                bottleneck: simulacao::bottleneck(&stats),
            }
        }
        Err(e) => JobOut::Failed(e.to_string()),
    }
}

/// Scenario lines `objeto.campo = valor` into the `set` of a scenario.
pub fn parse_set(text: &str) -> Result<BTreeMap<String, String>, String> {
    let mut m = BTreeMap::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            return Err((l4().bad_set_line)(i + 1));
        };
        m.insert(k.trim().to_string(), v.trim().to_string());
    }
    Ok(m)
}

/// Opens a file with the system's program (a saved diagram page).
pub(crate) fn open_with_system(path: &Path) -> std::io::Result<()> {
    #[cfg(windows)]
    let mut c = {
        let mut c = std::process::Command::new("cmd");
        c.args(["/C", "start", ""]).arg(path);
        c
    };
    #[cfg(target_os = "macos")]
    let mut c = {
        let mut c = std::process::Command::new("open");
        c.arg(path);
        c
    };
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut c = {
        let mut c = std::process::Command::new("xdg-open");
        c.arg(path);
        c
    };
    c.spawn().map(|_| ())
}

/// Opens the model in UTSim: the active version of the `utsim` product of the launcher,
/// with the model's path in place of its default arguments; else `utsim-view` on the path.
pub fn open_in_utsim(model: &Path) -> Result<(), String> {
    use utsim_launcher_core::install::{Layout, launch_command};
    if let Some(root) = Layout::default_root()
        && let Ok((exe, _args, dir)) = launch_command(&Layout::new(root), "utsim")
    {
        return std::process::Command::new(&exe)
            .arg(model)
            .current_dir(dir)
            .spawn()
            .map(|_| ())
            .map_err(|e| e.to_string());
    }
    std::process::Command::new("utsim-view")
        .arg(model)
        .spawn()
        .map(|_| ())
        .map_err(|_| l4().utsim_not_found.to_string())
}

impl App {
    /// Starts the replications of the linked model on the worker thread (the button
    /// "Rodar as replicações"); false when no model is loaded or a job is running.
    pub fn start_model_runs(&mut self) -> bool {
        let Some(ld) = self.l4.loaded.clone() else {
            return false;
        };
        if self.l4.job.is_some() {
            return false;
        }
        let m = ld.model;
        self.l4.spawn(l4().run.to_string(), move || runs_job(&m));
        true
    }

    /// Whether a simulation of the model screen is still running.
    pub fn model_job_running(&self) -> bool {
        self.l4.job.is_some()
    }

    /// The KPIs of the last replications run on the model screen.
    pub fn model_kpis(&self) -> Vec<String> {
        self.l4
            .runs
            .as_ref()
            .map(|r| r.0.iter().map(|(k, _, _)| k.clone()).collect())
            .unwrap_or_default()
    }

    /// Reads the linked model (once; again after the link changes).
    pub(in crate::app) fn load_linked(&mut self, s: &Session) {
        let Some(link) = simulacao::link(s.state(), s.project()) else {
            self.l4.loaded = None;
            return;
        };
        let path = s.folder().model_path(&link.path);
        let modified = std::fs::metadata(&path).and_then(|m| m.modified()).ok();
        if self
            .l4
            .loaded
            .as_ref()
            .is_some_and(|l| l.path == path && l.modified == modified)
        {
            return;
        }
        match std::fs::read_to_string(&path) {
            Ok(text) => match simulacao::parse(&text, path.parent()) {
                Ok(model) => {
                    self.l4.loaded = Some(Loaded {
                        hash: simulacao::hash_text(&text),
                        path,
                        text,
                        model,
                        modified,
                    });
                    self.l4.error = None;
                }
                Err(e) => self.l4.error = Some(e.to_string()),
            },
            Err(e) => {
                self.l4.error = Some((l4().cannot_read)(
                    &path.display().to_string(),
                    &e.to_string(),
                ))
            }
        }
    }

    pub(in crate::app) fn model_screen(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = l4();
        ui.heading(t.nav_model);
        ui.weak(t.intro);
        self.poll_job();
        self.load_linked(s);
        self.link_section(ui, s);
        let Some(loaded) = self.l4.loaded.clone() else {
            if let Some(e) = &self.l4.error {
                ui.colored_label(theme::WARN, e);
            }
            return;
        };
        if let Some(j) = &self.l4.job {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label((t.running)(&j.label));
            });
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(200));
        }
        ui.separator();
        self.maps_section(ui, s, &loaded);
        ui.separator();
        self.runs_section(ui, s, &loaded);
        ui.separator();
        self.study_section(ui, s, &loaded);
        ui.separator();
        self.observed_section(ui, s, &loaded);
    }

    fn poll_job(&mut self) {
        let Some(j) = &self.l4.job else { return };
        let Ok(out) = j.rx.try_recv() else { return };
        self.l4.job = None;
        match out {
            JobOut::Runs {
                summary,
                per_rep,
                bottleneck,
            } => self.l4.runs = Some((summary, per_rep, bottleneck)),
            JobOut::Shifting(v) => self.l4.shares = Some(v),
            JobOut::Study(st) => self.l4.study = Some(*st),
            JobOut::Observed(o) => self.l4.observed = Some(*o),
            o @ (JobOut::Vsm(..) | JobOut::Spaghetti(..)) => self.l4.pending = Some(o),
            JobOut::Failed(e) => self.set_status((texts().error)(&e), true),
        }
    }

    fn link_section(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = l4();
        let link = simulacao::link(s.state(), s.project());
        if let Some(l) = &link {
            ui.label((t.linked)(
                &l.name,
                &l.path,
                &l.hash[..12.min(l.hash.len())],
            ));
            if let Some(ld) = &self.l4.loaded
                && ld.hash != l.hash
            {
                ui.colored_label(theme::WARN, t.changed);
                let st = simulacao::stale(s.state(), s.project(), &ld.hash);
                for x in &st {
                    ui.label(format!("· {} ({})", x.name, x.kind.key()));
                }
                if ui.button(t.update_link).clicked() {
                    let e = simulacao::link_edits(
                        s.project(),
                        l.id,
                        &l.path,
                        &ld.text,
                        &ld.model,
                        l.copied,
                        s.today(),
                    );
                    let r = s.apply(e);
                    self.report(r);
                }
            }
        } else {
            ui.label(t.no_link);
        }
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.l4.path_input)
                    .hint_text(t.path_hint)
                    .desired_width(380.0),
            );
            ui.checkbox(&mut self.l4.copy, t.copy_into_project);
            if ui.button(t.link).clicked() {
                self.link_model(s, link.map(|l| l.id));
            }
        });
    }

    fn link_model(&mut self, s: &mut Session, existing: Option<utsim_lss_core::Id>) {
        let src = PathBuf::from(self.l4.path_input.trim());
        let text = match std::fs::read_to_string(&src) {
            Ok(x) => x,
            Err(e) => {
                self.set_status(
                    (l4().cannot_read)(&src.display().to_string(), &e.to_string()),
                    true,
                );
                return;
            }
        };
        let model = match simulacao::parse(&text, src.parent()) {
            Ok(m) => m,
            Err(e) => {
                self.set_status((texts().error)(&e.to_string()), true);
                return;
            }
        };
        let rel = if self.l4.copy {
            match s.folder().copy_model(&src, &text) {
                Ok(r) => r,
                Err(e) => {
                    self.set_status((texts().error)(&e.to_string()), true);
                    return;
                }
            }
        } else {
            src.display().to_string()
        };
        let id = existing.unwrap_or_else(|| s.new_id());
        let e = simulacao::link_edits(
            s.project(),
            id,
            &rel,
            &text,
            &model,
            self.l4.copy,
            s.today(),
        );
        let r = s.apply(e);
        if self.report(r).is_some() {
            self.l4.loaded = None;
            self.set_status(l4().linked_ok, false);
        }
    }

    /// Saves a page in `dados/` and opens it.
    fn save_page(&mut self, s: &Session, html: &str) {
        match s.folder().add_data(html.as_bytes(), "html") {
            Ok((file, _)) => {
                let p = s.folder().data_path(&file);
                let _ = open_with_system(&p);
                self.set_status((l4().saved_at)(&p.display().to_string()), false);
            }
            Err(e) => self.set_status((texts().error)(&e.to_string()), true),
        }
    }

    /// A VSM deliverable in Medir with the model's processes; its id.
    fn vsm_deliverable(
        &mut self,
        s: &mut Session,
        rows: &[utsim_lss_core::tools::vsm::ModelProcess],
    ) -> Option<Id> {
        use utsim_lss_core::tools::{self, Deliverable, Tool, vsm};
        let id = s.new_id();
        let phase = tools::default_phase(s.state(), s.project(), Tool::Vsm);
        let mut e = Deliverable::create(s.project(), phase, Tool::Vsm, id, l10().vsm_current);
        e.extend(vsm::header_edits(id, None, None, None, Some(false)));
        {
            let ids = s.ids();
            e.extend(vsm::model_rows_edits(s.state(), s.project(), id, rows, ids));
        }
        let r = s.apply(e);
        self.report(r)?;
        self.selected_deliverable = Some(id);
        self.set_status(l10().vsm_generated, false);
        Some(id)
    }

    /// Generates the current VSM deliverable from the linked model, at once (tests and
    /// tools; the button does it on the worker thread).
    pub fn generate_vsm_from_model(&mut self) -> Option<Id> {
        let mut s = self.session.take()?;
        self.load_linked(&s);
        let r = self
            .l4
            .loaded
            .clone()
            .and_then(|ld| simulacao::vsm_from_model(&ld.model).ok())
            .and_then(|(_, rows)| self.vsm_deliverable(&mut s, &rows));
        self.session = Some(s);
        r
    }

    /// Saves the distances of a spaghetti diagram as an analysis of the model.
    fn record_spaghetti(&mut self, s: &mut Session, ld: &Loaded, travels: Vec<simulacao::Travel>) {
        let e = simulacao::record_analysis(
            s.project(),
            s.new_id(),
            AnalysisKind::Spaghetti,
            l10().spaghetti_page,
            &ld.hash,
            serde_json::to_value(&travels).unwrap_or(Value::Null),
            1,
            s.today(),
        );
        if self.commit(s, e) {
            self.set_status(l10().spaghetti_saved, false);
        }
        self.l4.travels = travels;
    }

    /// Records the spaghetti distances of the linked model, at once (tests and tools).
    pub fn record_spaghetti_from_model(&mut self) -> bool {
        let Some(mut s) = self.session.take() else {
            return false;
        };
        self.load_linked(&s);
        let ok = match self.l4.loaded.clone() {
            Some(ld) => match simulacao::spaghetti(&ld.model) {
                Ok((_, tr)) => {
                    self.record_spaghetti(&mut s, &ld, tr);
                    true
                }
                Err(_) => false,
            },
            None => false,
        };
        self.session = Some(s);
        ok
    }

    fn lean_maps(&mut self, ui: &mut egui::Ui, s: &mut Session, ld: &Loaded) {
        let t = l10();
        if let Some(out) = self.l4.pending.take() {
            match out {
                JobOut::Vsm(html, rows, make) => {
                    if make {
                        self.vsm_deliverable(s, &rows);
                    } else {
                        self.save_page(s, &html);
                    }
                }
                JobOut::Spaghetti(html, travels) => {
                    self.save_page(s, &html);
                    self.record_spaghetti(s, ld, travels);
                }
                _ => {}
            }
        }
        let busy = self.l4.job.is_some();
        ui.horizontal_wrapped(|ui| {
            for (label, make) in [(t.vsm_page, false), (t.vsm_generate, true)] {
                if ui.add_enabled(!busy, egui::Button::new(label)).clicked() {
                    let m = ld.model.clone();
                    self.l4.spawn(
                        t.working.to_string(),
                        move || match simulacao::vsm_from_model(&m) {
                            Ok((h, rows)) => JobOut::Vsm(h, rows, make),
                            Err(e) => JobOut::Failed(e.to_string()),
                        },
                    );
                }
            }
            if ui
                .add_enabled(!busy, egui::Button::new(t.spaghetti_page))
                .clicked()
            {
                let m = ld.model.clone();
                self.l4.spawn(t.working.to_string(), move || {
                    match simulacao::spaghetti(&m) {
                        Ok((h, tr)) => JobOut::Spaghetti(h, tr),
                        Err(e) => JobOut::Failed(e.to_string()),
                    }
                });
            }
        });
        if !self.l4.travels.is_empty() {
            ui.label(t.spaghetti_totals);
            egui::Grid::new("espaguete").striped(true).show(ui, |ui| {
                for h in [
                    t.spaghetti_resource,
                    t.spaghetti_trips,
                    t.spaghetti_distance,
                ] {
                    ui.strong(h);
                }
                ui.end_row();
                for tr in &self.l4.travels {
                    ui.label(&tr.resource);
                    ui.label(tr.trips.to_string());
                    ui.label(crate::app::screens::show_number(tr.distance));
                    ui.end_row();
                }
            });
        }
    }

    fn maps_section(&mut self, ui: &mut egui::Ui, s: &mut Session, ld: &Loaded) {
        let t = l4();
        ui.strong(t.maps);
        self.lean_maps(ui, s, ld);
        let c = simulacao::carta(&ld.model);
        egui::Grid::new("carta").striped(true).show(ui, |ui| {
            for h in [
                t.step,
                t.symbol,
                t.description,
                t.time,
                t.distance,
                t.resources,
            ] {
                ui.strong(h);
            }
            ui.end_row();
            for r in &c.rows {
                ui.label(r.n.to_string());
                ui.label(format!("{:?}", r.symbol));
                ui.label(&r.descricao);
                ui.label(
                    r.tempo
                        .map(crate::app::screens::show_number)
                        .unwrap_or_default(),
                );
                ui.label(
                    r.distancia
                        .map(crate::app::screens::show_number)
                        .unwrap_or_default(),
                );
                ui.label(&r.recursos);
                ui.end_row();
            }
        });
        ui.horizontal_wrapped(|ui| {
            for (label, kind) in [
                (t.save_flowchart, utsim_diagram::TipoDiagrama::Blocos),
                (t.save_carta, utsim_diagram::TipoDiagrama::Carta),
                (t.save_sipoc, utsim_diagram::TipoDiagrama::Sipoc),
            ] {
                if ui.button(label).clicked() {
                    match simulacao::diagram_html(&ld.model, kind)
                        .map_err(|e| e.to_string())
                        .and_then(|h| {
                            s.folder()
                                .add_data(h.as_bytes(), "html")
                                .map_err(|e| e.to_string())
                        }) {
                        Ok((file, _)) => {
                            let p = s.folder().data_path(&file);
                            let _ = open_with_system(&p);
                            self.set_status((t.saved_at)(&p.display().to_string()), false);
                        }
                        Err(e) => self.set_status((texts().error)(&e), true),
                    }
                }
            }
            if ui.button(t.generate_sipoc).clicked() {
                match simulacao::sipoc_columns(&ld.model) {
                    Ok(cols) => {
                        let id = s.new_id();
                        let e = simulacao::sipoc_edits(
                            s.state(),
                            s.project(),
                            id,
                            &ld.hash,
                            &cols,
                            s.ids(),
                        );
                        let r = s.apply(e);
                        if self.report(r).is_some() {
                            self.selected_deliverable = Some(id);
                            self.set_status(t.sipoc_done, false);
                        }
                    }
                    Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                }
            }
            if ui.button(t.open_in_utsim).clicked() {
                match open_in_utsim(&ld.path) {
                    Ok(()) => self.set_status(t.opening, false),
                    Err(e) => self.set_status((texts().error)(&e), true),
                }
            }
        });
    }

    fn runs_section(&mut self, ui: &mut egui::Ui, s: &mut Session, ld: &Loaded) {
        let t = l4();
        ui.strong(t.runs);
        ui.label((t.run_facts)(
            ld.model.run.replications,
            ld.model.run.seed,
            ld.model.run.stop_time,
        ));
        let busy = self.l4.job.is_some();
        ui.horizontal(|ui| {
            if ui.add_enabled(!busy, egui::Button::new(t.run)).clicked() {
                let m = ld.model.clone();
                self.l4.spawn(t.run.to_string(), move || runs_job(&m));
            }
            if ui
                .add_enabled(!busy, egui::Button::new(t.shifting))
                .clicked()
            {
                let m = ld.model.clone();
                self.l4.spawn(t.shifting.to_string(), move || {
                    match utsim_model::analysis::run_traced(&m) {
                        Ok(tr) => JobOut::Shifting(simulacao::shares(
                            &utsim_model::analysis::shifting_bottleneck(&tr, 0.95),
                        )),
                        Err(e) => JobOut::Failed(e.to_string()),
                    }
                });
            }
        });
        if let Some((summary, per_rep, b)) = self.l4.runs.clone() {
            ui.colored_label(theme::WARN, t.simulated_label);
            egui::Grid::new("kpis").striped(true).show(ui, |ui| {
                for h in [t.keep, t.kpi, t.mean, t.half_width] {
                    ui.strong(h);
                }
                ui.end_row();
                for (k, m, hw) in &summary {
                    let mut on = self.l4.keep.contains(k);
                    if ui.checkbox(&mut on, "").changed() {
                        if on {
                            self.l4.keep.push(k.clone());
                        } else {
                            self.l4.keep.retain(|x| x != k);
                        }
                    }
                    ui.label(k);
                    ui.label(crate::app::screens::show_number(*m));
                    ui.label(crate::app::screens::show_number(*hw));
                    ui.end_row();
                }
            });
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(
                        self.inputs.entry("nome_conjunto".into()).or_default(),
                    )
                    .hint_text(t.dataset_name_hint)
                    .desired_width(240.0),
                );
                if ui.button(t.save_dataset).clicked() {
                    let name = self.inputs.remove("nome_conjunto").unwrap_or_default();
                    let name = if name.trim().is_empty() {
                        ld.model.name.clone()
                    } else {
                        name
                    };
                    let csv = simulacao::replication_csv(&per_rep, &self.l4.keep);
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
                            let r = s.apply(e);
                            if self.report(r).is_some() {
                                self.set_status(t.dataset_saved, false);
                            }
                        }
                        Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                    }
                }
                if ui.button(t.save_bottleneck).clicked() {
                    let e = simulacao::record_analysis(
                        s.project(),
                        s.new_id(),
                        AnalysisKind::Bottleneck,
                        t.bottleneck,
                        &ld.hash,
                        serde_json::to_value(&b).unwrap_or(Value::Null),
                        ld.model.run.replications,
                        s.today(),
                    );
                    let r = s.apply(e);
                    self.report(r);
                }
            });
            ui.label((t.bottleneck_is)(
                b.bottleneck.as_deref().unwrap_or("—"),
                b.in_shift.as_deref().unwrap_or("—"),
            ));
            egui::Grid::new("gargalo").striped(true).show(ui, |ui| {
                for h in [t.resource, t.utilization, t.in_shift] {
                    ui.strong(h);
                }
                ui.end_row();
                for r in &b.rows {
                    ui.label(&r.name);
                    ui.label(format!(
                        "{:.1} % ± {:.1}",
                        r.utilization * 100.0,
                        r.half_width * 100.0
                    ));
                    ui.label(
                        r.in_shift
                            .map(|x| format!("{:.1} %", x * 100.0))
                            .unwrap_or_default(),
                    );
                    ui.end_row();
                }
            });
        }
        if let Some(sh) = self.l4.shares.clone() {
            ui.strong(t.shifting);
            for x in &sh {
                ui.label((t.share_line)(&x.name, x.sole * 100.0, x.shifting * 100.0));
            }
            if ui.button(t.save_shifting).clicked() {
                let e = simulacao::record_analysis(
                    s.project(),
                    s.new_id(),
                    AnalysisKind::ShiftingBottleneck,
                    t.shifting,
                    &ld.hash,
                    serde_json::to_value(&sh).unwrap_or(Value::Null),
                    ld.model.run.replications,
                    s.today(),
                );
                let r = s.apply(e);
                self.report(r);
            }
        }
        for ds in dados::datasets(s.state(), s.project()) {
            ui.label(format!("· {} · {} linhas", ds.labeled(), ds.rows));
        }
    }

    fn study_section(&mut self, ui: &mut egui::Ui, s: &mut Session, ld: &Loaded) {
        let t = l4();
        ui.strong(t.study);
        ui.weak(t.study_intro);
        if self.l4.scenarios.is_empty() {
            self.l4
                .scenarios
                .push((t.baseline.to_string(), String::new()));
            self.l4
                .scenarios
                .push((t.scenario_1.to_string(), String::new()));
        }
        ui.add(
            egui::TextEdit::singleline(&mut self.l4.study_name)
                .hint_text(t.study_name_hint)
                .desired_width(300.0),
        );
        let mut remove = None;
        for (i, (name, set)) in self.l4.scenarios.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(name).desired_width(140.0));
                if i == 0 {
                    ui.weak(t.baseline_note);
                } else {
                    ui.add(
                        egui::TextEdit::multiline(set)
                            .hint_text(t.set_hint)
                            .desired_rows(2)
                            .desired_width(360.0),
                    );
                    if ui.small_button("✕").clicked() {
                        remove = Some(i);
                    }
                }
            });
        }
        if let Some(i) = remove {
            self.l4.scenarios.remove(i);
        }
        if ui.button(t.add_scenario).clicked() {
            let n = self.l4.scenarios.len();
            self.l4.scenarios.push(((t.scenario_n)(n), String::new()));
        }
        if let Some((summary, _, _)) = &self.l4.runs {
            ui.horizontal_wrapped(|ui| {
                ui.label(t.measures);
                for (k, _, _) in summary {
                    let mut on = self.l4.measures.contains(k);
                    if ui.checkbox(&mut on, k).changed() {
                        if on {
                            self.l4.measures.push(k.clone());
                        } else {
                            self.l4.measures.retain(|x| x != k);
                        }
                    }
                }
            });
            for m in self.l4.measures.clone() {
                ui.horizontal(|ui| {
                    ui.label((t.minimum_of)(&m));
                    ui.add(
                        egui::TextEdit::singleline(self.l4.minimums.entry(m.clone()).or_default())
                            .desired_width(70.0),
                    );
                });
            }
        } else {
            ui.weak(t.run_first);
        }
        let busy = self.l4.job.is_some();
        if ui
            .add_enabled(!busy, egui::Button::new(t.run_study))
            .clicked()
        {
            let mut scenarios = Vec::new();
            for (name, set) in &self.l4.scenarios {
                match parse_set(set) {
                    Ok(m) => scenarios.push(Scenario {
                        name: name.trim().to_string(),
                        set: m,
                    }),
                    Err(e) => {
                        self.set_status((texts().error)(&e), true);
                        return;
                    }
                }
            }
            let mins: BTreeMap<String, f64> = self
                .l4
                .minimums
                .iter()
                .filter_map(|(k, v)| crate::app::screens::parse_number(v).map(|x| (k.clone(), x)))
                .collect();
            let (m, hash, measures, name) = (
                ld.model.clone(),
                ld.hash.clone(),
                self.l4.measures.clone(),
                self.l4.study_name.clone(),
            );
            self.l4.spawn(t.run_study.to_string(), move || {
                match utsim_model::experiment::run_scenarios(&m, &scenarios, &measures, &|_, _| {
                    true
                }) {
                    Ok(res) => match simulacao::build_study(
                        &name, &m, &hash, &scenarios, &measures, &res, &mins,
                    ) {
                        Ok(st) => JobOut::Study(Box::new(st)),
                        Err(e) => JobOut::Failed(e.to_string()),
                    },
                    Err(e) => JobOut::Failed(e.to_string()),
                }
            });
        }
        if let Some(st) = self.l4.study.clone() {
            egui::Grid::new("diferencas").striped(true).show(ui, |ui| {
                for h in [t.scenario, t.kpi, t.difference, t.verdict] {
                    ui.strong(h);
                }
                ui.end_row();
                for d in &st.differences {
                    ui.label(&d.scenario);
                    ui.label(&d.metric);
                    ui.label(format!(
                        "{} [{}; {}]",
                        crate::app::screens::show_number(d.mean),
                        crate::app::screens::show_number(d.lo()),
                        crate::app::screens::show_number(d.hi())
                    ));
                    let v = if d.relevant {
                        t.relevant
                    } else if d.significant {
                        t.significant_only
                    } else {
                        t.not_significant
                    };
                    if d.relevant {
                        ui.colored_label(theme::OK, v);
                    } else {
                        ui.label(v);
                    }
                    ui.end_row();
                }
            });
            if ui.button(t.save_study).clicked() {
                let json = st.to_json();
                match s.folder().add_data(json.as_bytes(), "json") {
                    Ok((file, hash)) => {
                        let e = simulacao::record_study(
                            s.project(),
                            s.new_id(),
                            &st,
                            &file,
                            &hash,
                            s.today(),
                        );
                        let r = s.apply(e);
                        if self.report(r).is_some() {
                            self.set_status(t.study_saved, false);
                        }
                    }
                    Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                }
            }
        }
        for x in simulacao::studies(s.state(), s.project()) {
            let old = x.model_hash != ld.hash;
            let line = format!("· {} — {}", x.name, x.conclusion);
            ui.horizontal_wrapped(|ui| {
                if old {
                    ui.colored_label(theme::WARN, format!("{line} ({})", t.old_version));
                } else {
                    ui.label(line);
                }
                if self.l4.gain.is_none() && ui.small_button(l10().book_gain).clicked() {
                    match read_study(s, &x) {
                        Ok(file) => {
                            let diff = file
                                .differences
                                .iter()
                                .position(|d| d.significant)
                                .unwrap_or(0);
                            self.l4.gain = Some(GainForm {
                                study: x.id,
                                file,
                                diff,
                                description: String::new(),
                                value: String::new(),
                                periods: String::new(),
                                higher: true,
                                conservative: true,
                                soft: false,
                            });
                        }
                        Err(e) => self.set_status(e, true),
                    }
                }
            });
        }
        self.gain_form(ui, s);
    }

    fn gain_form(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = l10();
        let Some(f) = self.l4.gain.as_mut() else {
            return;
        };
        ui.group(|ui| {
            ui.strong(t.gain_title);
            ui.weak(t.gain_intro);
            let label = |d: &simulacao::Difference| {
                format!(
                    "{} · {}: {} [{}; {}]",
                    d.scenario,
                    d.metric,
                    super::show_number(d.mean),
                    super::show_number(d.lo()),
                    super::show_number(d.hi())
                )
            };
            ui.horizontal(|ui| {
                ui.label(t.gain_difference);
                egui::ComboBox::from_id_salt("ganho_diferenca")
                    .selected_text(
                        f.file
                            .differences
                            .get(f.diff)
                            .map(label)
                            .unwrap_or_default(),
                    )
                    .show_ui(ui, |ui| {
                        for (i, d) in f.file.differences.iter().enumerate() {
                            ui.add_enabled_ui(d.significant, |ui| {
                                ui.selectable_value(&mut f.diff, i, label(d));
                            });
                        }
                    });
            });
            ui.add(
                egui::TextEdit::singleline(&mut f.description)
                    .hint_text(t.gain_description)
                    .desired_width(360.0),
            );
            ui.horizontal(|ui| {
                ui.label(t.gain_value_per_unit);
                ui.add(egui::TextEdit::singleline(&mut f.value).desired_width(80.0));
                ui.label(t.gain_periods);
                ui.add(egui::TextEdit::singleline(&mut f.periods).desired_width(60.0));
            });
            ui.checkbox(&mut f.higher, t.gain_higher);
            ui.checkbox(&mut f.conservative, t.gain_conservative);
            ui.checkbox(&mut f.soft, t.gain_soft);
        });
        let Some(f) = self.l4.gain.as_ref() else {
            return;
        };
        let input = gain_input(f);
        let study = simulacao::studies(s.state(), s.project())
            .into_iter()
            .find(|x| x.id == f.study);
        let preview = match (&input, &study, f.file.differences.get(f.diff)) {
            (Some(g), Some(st), Some(d)) => Some(simulacao::gain_value(st, &f.file, d, g)),
            _ => None,
        };
        let (study_id, diff) = (f.study, f.diff);
        let mut book = false;
        let mut cancel = false;
        ui.horizontal(|ui| {
            match &preview {
                Some(Ok((v, _))) => {
                    ui.label((t.gain_preview)(&crate::text::money(*v)));
                }
                Some(Err(e)) => {
                    ui.colored_label(theme::WARN, e.to_string());
                }
                None => {
                    ui.weak(t.gain_bad_number);
                }
            }
            book = ui
                .add_enabled(
                    matches!(preview, Some(Ok(_))),
                    egui::Button::new(t.gain_book),
                )
                .clicked();
            cancel = ui.button(t.cancel).clicked();
        });
        if book {
            if let Some(g) = input {
                self.book_gain(s, study_id, diff, &g);
            }
        } else if cancel {
            self.l4.gain = None;
        }
    }

    /// Books an estimated saving from a difference of a saved study; whether it worked.
    fn book_gain(&mut self, s: &mut Session, study: Id, diff: usize, g: &GainInput) -> bool {
        let Some(x) = simulacao::studies(s.state(), s.project())
            .into_iter()
            .find(|x| x.id == study)
        else {
            return false;
        };
        let file = match read_study(s, &x) {
            Ok(f) => f,
            Err(e) => {
                self.set_status(e, true);
                return false;
            }
        };
        let Some(d) = file.differences.get(diff) else {
            return false;
        };
        match simulacao::gain_edits(s.project(), s.new_id(), &x, &file, d, g) {
            Ok(e) => {
                let ok = self.commit(s, e);
                if ok {
                    self.l4.gain = None;
                    self.set_status(l10().gain_booked, false);
                }
                ok
            }
            Err(e) => {
                self.set_status(e.to_string(), true);
                false
            }
        }
    }

    /// Books an estimated saving from the difference `diff` of a saved study, as the form
    /// "Lançar ganho estimado" does (tests and tools).
    pub fn book_study_gain(&mut self, study: Id, diff: usize, g: &GainInput) -> bool {
        let Some(mut s) = self.session.take() else {
            return false;
        };
        let ok = self.book_gain(&mut s, study, diff, g);
        self.session = Some(s);
        ok
    }

    fn observed_section(&mut self, ui: &mut egui::Ui, s: &mut Session, ld: &Loaded) {
        let t = l4();
        ui.strong(t.observed);
        ui.weak(t.observed_intro);
        let names = simulacao::object_names(&ld.model);
        ui.horizontal(|ui| {
            for (label, v, salt) in [
                (t.from, &mut self.l4.from, "de"),
                (t.to, &mut self.l4.to, "ate"),
            ] {
                ui.label(label);
                egui::ComboBox::from_id_salt(("pert", salt))
                    .selected_text(v.clone())
                    .show_ui(ui, |ui| {
                        for n in &names {
                            ui.selectable_value(v, n.clone(), n);
                        }
                    });
            }
        });
        let busy = self.l4.job.is_some();
        let ready = !self.l4.from.is_empty() && !self.l4.to.is_empty();
        if ui
            .add_enabled(!busy && ready, egui::Button::new(t.run_observed))
            .clicked()
        {
            let (m, a, b) = (ld.model.clone(), self.l4.from.clone(), self.l4.to.clone());
            self.l4.spawn(
                t.run_observed.to_string(),
                move || match simulacao::observed_pert(&m, &a, &b, None) {
                    Ok(o) => JobOut::Observed(Box::new(o)),
                    Err(e) => JobOut::Failed(e.to_string()),
                },
            );
        }
        if let Some(o) = self.l4.observed.clone() {
            ui.label((t.observed_total)(o.units, o.total, o.half_width));
            egui::Grid::new("pert_obs").striped(true).show(ui, |ui| {
                for h in [t.activity, t.criticality, t.wait, t.work, t.blocked] {
                    ui.strong(h);
                }
                ui.end_row();
                for (n, c, w, k, bl) in &o.activities {
                    ui.label(n);
                    ui.label(format!("{:.0} %", c * 100.0));
                    for x in [w, k, bl] {
                        ui.label(crate::app::screens::show_number(*x));
                    }
                    ui.end_row();
                }
            });
            for w in &o.warnings {
                ui.colored_label(theme::WARN, w);
            }
            if ui.button(t.save_observed).clicked() {
                let e = simulacao::record_analysis(
                    s.project(),
                    s.new_id(),
                    AnalysisKind::ObservedPert,
                    &format!("{} → {}", o.from, o.to),
                    &ld.hash,
                    serde_json::to_value(&o).unwrap_or(Value::Null),
                    ld.model.run.replications,
                    s.today(),
                );
                let r = s.apply(e);
                self.report(r);
            }
        }
    }
}

/// The package of a saved study, read from `dados/` and checked against its hash.
fn read_study(s: &Session, x: &simulacao::Study) -> Result<StudyFile, String> {
    let text = s
        .folder()
        .read_data(&x.file)
        .map_err(|e| (texts().error)(&e.to_string()))?;
    if simulacao::hash_text(&text) != x.hash {
        return Err(l10().gain_file_changed.to_string());
    }
    StudyFile::from_json(&text).map_err(|e| (texts().error)(&e))
}

/// The numbers of the gain form, when they read.
fn gain_input(f: &GainForm) -> Option<GainInput> {
    Some(GainInput {
        description: f.description.clone(),
        category: if f.soft {
            SavingCategory::Soft
        } else {
            SavingCategory::Hard
        },
        value_per_unit: super::parse_number(&f.value)?,
        periods_per_year: super::parse_number(&f.periods)?,
        higher_is_better: f.higher,
        conservative: f.conservative,
    })
}
