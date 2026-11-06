//! The analyses screen (stage L3 and P1, `docs/plano_lean_seis_sigma.md`, 2.2 and 2.3):
//! data sets imported from CSV with their origin (real, twin or simulated), and the
//! analyses over `utsim-stats` and `utsim-quality` (descriptive, normality, capability,
//! control charts, tests, Gage R&R and the type 1 study), each saved with its recipe, the
//! data set's hash and origin, and drawn with `egui_plot`.

use std::path::PathBuf;

use eframe::egui;
use egui_plot::{HLine, Line, Plot, PlotPoints, Points};
use utsim_lss_core::analises::{self, Outcome, Recipe, Table};
use utsim_lss_core::dados::{self, DataOrigin, Dataset, DatasetInput};
use utsim_quality::charts::ChartKind;

use super::super::App;
use crate::session::Session;
use crate::text::l3::{l3, origin_name};
use crate::text::texts;
use crate::theme;

/// The kinds of analysis offered, in the screen's order.
pub const KINDS: [&str; 15] = [
    "descritiva",
    "normalidade",
    "capacidade",
    "carta",
    "t_uma_amostra",
    "t_duas_amostras",
    "t_pareado",
    "anova",
    "qui_quadrado",
    "proporcao",
    "duas_proporcoes",
    "mann_whitney",
    "wilcoxon",
    "gage_rr",
    "msa_tipo1",
];

/// The analyses screen's state.
#[derive(Default)]
pub struct L3Ui {
    pub import_path: String,
    pub import_origin: DataOrigin,
    pub dataset: Option<utsim_lss_core::Id>,
    pub kind: usize,
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub numbers: [String; 4],
    pub chart: usize,
    pub flag: bool,
    pub result: Option<(Recipe, Result<Outcome, String>)>,
    pub name: String,
    pub justification: String,
}

fn num(s: &str) -> Option<f64> {
    crate::app::screens::parse_number(s)
}

impl App {
    /// Selects the data set of the analyses screen and runs `recipe` on it (what the
    /// "Calcular" button does).
    pub fn select_analysis(&mut self, dataset: utsim_lss_core::Id, kind: &str) {
        self.l3.dataset = Some(dataset);
        self.l3.kind = KINDS.iter().position(|k| *k == kind).unwrap_or(0);
    }

    pub(in crate::app) fn analyses_screen(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = l3();
        ui.heading(t.nav_analyses);
        ui.weak(t.intro);
        self.import_section(ui, s);
        let ds = dados::datasets(s.state(), s.project());
        if ds.is_empty() {
            ui.weak(t.no_data);
            return;
        }
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(t.dataset);
            let cur = self.l3.dataset.and_then(|i| ds.iter().find(|d| d.id == i));
            egui::ComboBox::from_id_salt("conjunto")
                .selected_text(cur.map_or_else(|| t.choose.to_string(), Dataset::labeled))
                .show_ui(ui, |ui| {
                    for d in &ds {
                        ui.selectable_value(&mut self.l3.dataset, Some(d.id), d.labeled());
                    }
                });
        });
        let Some(d) = self
            .l3
            .dataset
            .and_then(|i| ds.iter().find(|d| d.id == i))
            .cloned()
        else {
            return;
        };
        if d.origin != DataOrigin::Real {
            ui.colored_label(theme::WARN, (t.not_real)(origin_name(d.origin)));
        }
        let text = match s.folder().read_data(&d.file) {
            Ok(x) => x,
            Err(e) => {
                ui.colored_label(theme::WARN, e.to_string());
                return;
            }
        };
        let table = Table::parse(&text);
        self.recipe_form(ui, &table);
        if ui.button(t.run).clicked() {
            match self.build_recipe() {
                Ok(r) => {
                    let o = analises::run(&r, &table);
                    self.l3.result = Some((r, o));
                }
                Err(e) => self.set_status((texts().error)(&e), true),
            }
        }
        if let Some((r, o)) = self.l3.result.clone() {
            match o {
                Ok(o) => {
                    self.outcome_view(ui, &o);
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.l3.name)
                                .hint_text(t.name_hint)
                                .desired_width(220.0),
                        );
                        if ui.button(t.save).clicked() {
                            let e = analises::record(
                                s.project(),
                                s.new_id(),
                                &self.l3.name,
                                &d,
                                &r,
                                &o,
                                s.today(),
                            );
                            let res = s.apply(e);
                            if self.report(res).is_some() {
                                self.set_status(t.saved, false);
                            }
                        }
                    });
                }
                Err(e) => {
                    ui.colored_label(theme::WARN, e);
                }
            }
        }
        ui.separator();
        self.saved_analyses(ui, s);
    }

    fn import_section(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = l3();
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.l3.import_path)
                    .hint_text(t.csv_hint)
                    .desired_width(320.0),
            );
            egui::ComboBox::from_id_salt("origem_importacao")
                .selected_text(origin_name(self.l3.import_origin))
                .show_ui(ui, |ui| {
                    for o in [DataOrigin::Real, DataOrigin::Twin] {
                        ui.selectable_value(&mut self.l3.import_origin, o, origin_name(o));
                    }
                });
            if ui.button(t.import).clicked() {
                let p = PathBuf::from(self.l3.import_path.trim());
                match std::fs::read(&p) {
                    Ok(bytes) => {
                        let (text, _) = utsim_quality::data::decode(&bytes);
                        match s.folder().add_data(text.as_bytes(), "csv") {
                            Ok((file, hash)) => {
                                let (columns, rows) = dados::shape(&text);
                                let name = p.file_stem().map_or_else(
                                    || "dados".to_string(),
                                    |x| x.to_string_lossy().into_owned(),
                                );
                                let input = DatasetInput {
                                    name,
                                    origin: self.l3.import_origin,
                                    file,
                                    hash,
                                    columns,
                                    rows,
                                    date: Some(s.today()),
                                    note: p.display().to_string(),
                                    ..DatasetInput::default()
                                };
                                let id = s.new_id();
                                let r = s.apply(dados::register(s.project(), id, &input));
                                if self.report(r).is_some() {
                                    self.l3.dataset = Some(id);
                                    self.set_status(t.imported, false);
                                }
                            }
                            Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                        }
                    }
                    Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                }
            }
        });
    }

    fn column_combo(ui: &mut egui::Ui, salt: &str, label: &str, table: &Table, v: &mut usize) {
        ui.label(label);
        egui::ComboBox::from_id_salt(salt)
            .selected_text(table.header.get(*v).cloned().unwrap_or_default())
            .show_ui(ui, |ui| {
                for (i, h) in table.header.iter().enumerate() {
                    ui.selectable_value(v, i, h);
                }
            });
    }

    fn recipe_form(&mut self, ui: &mut egui::Ui, table: &Table) {
        let t = l3();
        ui.horizontal_wrapped(|ui| {
            ui.label(t.analysis);
            egui::ComboBox::from_id_salt("tipo_analise")
                .selected_text(crate::text::l3::kind_name(
                    KINDS[self.l3.kind.min(KINDS.len() - 1)],
                ))
                .show_ui(ui, |ui| {
                    for (i, k) in KINDS.iter().enumerate() {
                        ui.selectable_value(&mut self.l3.kind, i, crate::text::l3::kind_name(k));
                    }
                });
        });
        let k = KINDS[self.l3.kind.min(KINDS.len() - 1)];
        let u = &mut self.l3;
        ui.horizontal_wrapped(|ui| {
            let numbox = |ui: &mut egui::Ui, label: &str, i: usize, n: &mut [String; 4]| {
                ui.label(label);
                ui.add(egui::TextEdit::singleline(&mut n[i]).desired_width(70.0));
            };
            match k {
                "descritiva" | "normalidade" => {
                    Self::column_combo(ui, "col_a", t.column, table, &mut u.a)
                }
                "capacidade" => {
                    Self::column_combo(ui, "col_a", t.column, table, &mut u.a);
                    numbox(ui, t.lsl, 0, &mut u.numbers);
                    numbox(ui, t.usl, 1, &mut u.numbers);
                    numbox(ui, t.subgroup, 2, &mut u.numbers);
                }
                "carta" => {
                    egui::ComboBox::from_id_salt("tipo_carta")
                        .selected_text(CHARTS[u.chart.min(6)].label())
                        .show_ui(ui, |ui| {
                            for (i, c) in CHARTS.iter().enumerate() {
                                ui.selectable_value(&mut u.chart, i, c.label());
                            }
                        });
                    Self::column_combo(ui, "col_a", t.column, table, &mut u.a);
                    if CHARTS[u.chart.min(6)].is_attribute() {
                        Self::column_combo(ui, "col_b", t.sizes, table, &mut u.b);
                    } else {
                        numbox(ui, t.subgroup, 2, &mut u.numbers);
                    }
                    ui.checkbox(&mut u.flag, t.nelson);
                }
                "t_uma_amostra" => {
                    Self::column_combo(ui, "col_a", t.column, table, &mut u.a);
                    numbox(ui, t.mu0, 0, &mut u.numbers);
                }
                "t_duas_amostras" | "t_pareado" | "mann_whitney" | "wilcoxon" | "qui_quadrado" => {
                    Self::column_combo(ui, "col_a", t.column_a, table, &mut u.a);
                    Self::column_combo(ui, "col_b", t.column_b, table, &mut u.b);
                    if k == "t_duas_amostras" {
                        ui.checkbox(&mut u.flag, t.pooled);
                    }
                }
                "anova" => {
                    Self::column_combo(ui, "col_a", t.column, table, &mut u.a);
                    Self::column_combo(ui, "col_b", t.group, table, &mut u.b);
                }
                "proporcao" => {
                    numbox(ui, "x", 0, &mut u.numbers);
                    numbox(ui, "n", 1, &mut u.numbers);
                    numbox(ui, "p0", 2, &mut u.numbers);
                }
                "duas_proporcoes" => {
                    numbox(ui, "x1", 0, &mut u.numbers);
                    numbox(ui, "n1", 1, &mut u.numbers);
                    numbox(ui, "x2", 2, &mut u.numbers);
                    numbox(ui, "n2", 3, &mut u.numbers);
                }
                "gage_rr" => {
                    Self::column_combo(ui, "col_a", t.part, table, &mut u.a);
                    Self::column_combo(ui, "col_b", t.operator, table, &mut u.b);
                    Self::column_combo(ui, "col_c", t.value, table, &mut u.c);
                    numbox(ui, t.tolerance, 0, &mut u.numbers);
                    ui.checkbox(&mut u.flag, t.anova_method);
                }
                "msa_tipo1" => {
                    Self::column_combo(ui, "col_a", t.column, table, &mut u.a);
                    numbox(ui, t.reference, 0, &mut u.numbers);
                    numbox(ui, t.tolerance, 1, &mut u.numbers);
                }
                _ => {}
            }
        });
    }

    fn build_recipe(&self) -> Result<Recipe, String> {
        let u = &self.l3;
        let n = |i: usize| num(&u.numbers[i]);
        let need = |i: usize, what: &str| n(i).ok_or_else(|| (l3().fill)(what));
        let k = KINDS[u.kind.min(KINDS.len() - 1)];
        Ok(match k {
            "descritiva" => Recipe::Descriptive { col: u.a },
            "normalidade" => Recipe::Normality { col: u.a },
            "capacidade" => Recipe::Capability {
                col: u.a,
                lsl: n(0),
                usl: n(1),
                target: None,
                subgroup: n(2).map_or(1, |x| x.max(1.0) as usize),
            },
            "carta" => Recipe::Chart {
                kind: CHARTS[u.chart.min(6)],
                col: u.a,
                subgroup: n(2).map_or(5, |x| x.max(1.0) as usize),
                size_col: CHARTS[u.chart.min(6)].is_attribute().then_some(u.b),
                nelson: u.flag,
            },
            "t_uma_amostra" => Recipe::TOne {
                col: u.a,
                mu0: need(0, "μ0")?,
            },
            "t_duas_amostras" => Recipe::TTwo {
                a: u.a,
                b: u.b,
                pooled: u.flag,
            },
            "t_pareado" => Recipe::TPaired { a: u.a, b: u.b },
            "anova" => Recipe::Anova {
                col: u.a,
                group: u.b,
            },
            "qui_quadrado" => Recipe::ChiSquare { a: u.a, b: u.b },
            "proporcao" => Recipe::Proportion {
                x: need(0, "x")? as u64,
                n: need(1, "n")? as u64,
                p0: need(2, "p0")?,
            },
            "duas_proporcoes" => Recipe::TwoProportions {
                x1: need(0, "x1")? as u64,
                n1: need(1, "n1")? as u64,
                x2: need(2, "x2")? as u64,
                n2: need(3, "n2")? as u64,
            },
            "mann_whitney" => Recipe::MannWhitney { a: u.a, b: u.b },
            "wilcoxon" => Recipe::Wilcoxon { a: u.a, b: u.b },
            "gage_rr" => Recipe::GageRr {
                part: u.a,
                operator: u.b,
                value: u.c,
                tolerance: n(0),
                anova: u.flag,
            },
            "msa_tipo1" => Recipe::Type1 {
                col: u.a,
                reference: need(0, l3().reference)?,
                tolerance: need(1, l3().tolerance)?,
            },
            _ => return Err("análise desconhecida".into()),
        })
    }

    fn outcome_view(&mut self, ui: &mut egui::Ui, o: &Outcome) {
        let t = l3();
        ui.strong(&o.verdict);
        egui::Grid::new("resultado").striped(true).show(ui, |ui| {
            for (l, v) in &o.lines {
                ui.label(l);
                ui.label(crate::app::screens::show_number(*v));
                ui.end_row();
            }
        });
        for n in &o.notes {
            ui.weak(n);
        }
        if o.series.is_empty() {
            return;
        }
        let pts: PlotPoints = o
            .series
            .iter()
            .enumerate()
            .filter(|(_, y)| y.is_finite())
            .map(|(i, y)| [i as f64 + 1.0, *y])
            .collect();
        let sig: PlotPoints = o
            .signals
            .iter()
            .filter_map(|i| o.series.get(*i).map(|y| [*i as f64 + 1.0, *y]))
            .collect();
        let limits = o.limits;
        Plot::new("grafico_analise").height(220.0).show(ui, |p| {
            p.line(Line::new(t.values, pts));
            if let Some((c, l, u)) = limits {
                p.hline(HLine::new(t.center, c));
                p.hline(HLine::new(t.lcl, l));
                p.hline(HLine::new(t.ucl, u));
            }
            p.points(Points::new(t.signals, sig).radius(4.0).color(theme::WARN));
        });
    }

    fn saved_analyses(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = l3();
        ui.strong(t.saved_title);
        let ds = dados::datasets(s.state(), s.project());
        for a in analises::analyses(s.state(), s.project()) {
            let d = a.dataset.and_then(|i| ds.iter().find(|x| x.id == i));
            ui.horizontal(|ui| {
                ui.label(format!(
                    "{} · {} · {}",
                    a.name,
                    d.map_or_else(String::new, Dataset::labeled),
                    a.verdict
                ));
                if let (Some(g), true) = (a.grr(), a.justification.is_empty())
                    && g > analises::GRR_LIMIT
                {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.l3.justification)
                            .hint_text(t.justify_hint)
                            .desired_width(200.0),
                    );
                    if ui.small_button(t.justify).clicked() {
                        let r = s.set_field(
                            analises::Analysis::entity_ref(a.id),
                            analises::fields::JUSTIFICATION,
                            serde_json::Value::from(self.l3.justification.clone()),
                            None,
                        );
                        self.report(r);
                    }
                }
                if let Some(d) = d
                    && ui.small_button(t.rerun).clicked()
                {
                    match s
                        .folder()
                        .read_data(&d.file)
                        .map_err(|e| e.to_string())
                        .and_then(|x| analises::rerun(&a, &x))
                    {
                        Ok(o) => {
                            let same = o.lines == a.outcome.lines;
                            self.set_status(
                                if same {
                                    t.same_numbers
                                } else {
                                    t.different_numbers
                                },
                                !same,
                            );
                        }
                        Err(e) => self.set_status((texts().error)(&e), true),
                    }
                }
            });
        }
    }
}

/// The control charts offered.
pub const CHARTS: [ChartKind; 7] = [
    ChartKind::IMr,
    ChartKind::XbarR,
    ChartKind::XbarS,
    ChartKind::P,
    ChartKind::Np,
    ChartKind::C,
    ChartKind::U,
];
