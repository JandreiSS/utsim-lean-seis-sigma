//! The scoreboard (8.1) and the two dashboards of 8.7: the MBB view, with every indicator
//! down to the project and the person, and the leadership view, with six indicators,
//! target and trend and no personal data. Every indicator shows its rule and the instant
//! of the data (LSS-RNF-16). Both dashboards export to PDF (tables, rules and charts) and to
//! `.xlsx` (one sheet per indicator), in the program's `relatorios/`.

use eframe::egui::{self, Color32, RichText};
use egui_plot::{Bar, BarChart, Legend, Plot};
use utsim_lss_core::domain::SavingStatus;
use utsim_lss_core::painel::{self, Filter, Inputs, Kpi};
use utsim_lss_core::perfis::Action;
use utsim_lss_core::programa::{self, Stage};
use utsim_lss_core::relatorios::{Block, Chart, Document, pdf};
use utsim_lss_core::savings::Category;

use super::{Data, instant};
use crate::app::App;
use crate::app::screens::show_number;
use crate::session::Session;
use crate::text::l6::{kpi_name, kpi_rule, l6, level_name, metric_name, stage_name};
use crate::text::{approach_name, saving_status_name};
use crate::theme;

fn money(x: f64) -> String {
    let s = format!("{:.0}", x.round());
    let neg = s.starts_with('-');
    let digits: Vec<char> = s.trim_start_matches('-').chars().collect();
    let mut out = String::new();
    for (i, c) in digits.iter().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push('.');
        }
        out.push(*c);
    }
    format!("{}R$ {out}", if neg { "-" } else { "" })
}

fn rule(ui: &mut egui::Ui, text: &str) {
    ui.small(RichText::new(text).weak());
}

/// A bar chart of named values (one or two series).
fn bars(ui: &mut egui::Ui, id: &str, names: Vec<String>, series: Vec<(&str, Vec<f64>, Color32)>) {
    let n = names.len();
    let k = series.len().max(1) as f64;
    Plot::new(id)
        .height(200.0)
        .allow_scroll(false)
        .allow_zoom(false)
        .allow_drag(false)
        .include_y(0.0)
        .legend(Legend::default())
        .x_axis_formatter(move |mark, _| {
            let i = mark.value.round();
            if (mark.value - i).abs() < 1e-6 && i >= 0.0 && (i as usize) < n {
                names[i as usize].clone()
            } else {
                String::new()
            }
        })
        .show(ui, |plot| {
            for (j, (name, values, color)) in series.into_iter().enumerate() {
                let w = 0.8 / k;
                let b: Vec<Bar> = values
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        Bar::new(i as f64 - 0.4 + w * (j as f64 + 0.5), *v).width(w * 0.95)
                    })
                    .collect();
                plot.bar_chart(BarChart::new(name, b).color(color));
            }
        });
}

impl App {
    pub(in crate::app) fn program_scoreboard(&mut self, ui: &mut egui::Ui, s: &Session, d: &Data) {
        let tl = l6();
        ui.heading(tl.nav_scoreboard);
        if !self.may_view(ui, s, Action::ViewScoreboard) {
            return;
        }
        ui.weak(tl.scoreboard_intro);
        self.filters(ui, d);
        let year = d.year(&self.l6);
        let rows = programa::scoreboard(s.state(), d.program, year, &d.records, &d.members);
        let rows: Vec<_> = rows
            .into_iter()
            .filter(|r| {
                self.l6.unit_filter.is_none() || r.unit == self.l6.unit_filter || r.unit.is_none()
            })
            .collect();
        egui::Grid::new("placar").striped(true).show(ui, |ui| {
            for h in [
                tl.unit,
                tl.metric,
                tl.target,
                tl.actual_official,
                tl.forecast_apart,
                tl.reached,
            ] {
                ui.strong(h);
            }
            ui.end_row();
            for r in &rows {
                let unit = if r.unit.is_none() {
                    tl.program_total.to_string()
                } else {
                    d.unit_name(r.unit)
                };
                ui.label(unit);
                let mut m = metric_name(r.metric).to_string();
                if let Some(l) = r.level {
                    m = format!("{m} ({})", level_name(l));
                }
                ui.label(m);
                let fmt = |x: f64| match r.metric {
                    programa::Metric::HardValidated | programa::Metric::SoftValidated => money(x),
                    _ => show_number((x * 10.0).round() / 10.0),
                };
                ui.label(r.target.map_or("-".into(), fmt));
                ui.label(RichText::new(fmt(r.actual)).strong());
                ui.label(if r.forecast > 0.0 {
                    fmt(r.forecast)
                } else {
                    String::new()
                });
                match r.reached() {
                    Some(f) => {
                        let c = if f >= 1.0 {
                            theme::accent(ui)
                        } else {
                            theme::WARN
                        };
                        ui.colored_label(c, format!("{:.0} %", f * 100.0));
                    }
                    None => {
                        ui.label("");
                    }
                }
                ui.end_row();
            }
        });
        rule(ui, tl.rule_scoreboard);
        let hard: Vec<&programa::ScoreRow> = rows
            .iter()
            .filter(|r| r.unit.is_some() && r.metric == programa::Metric::HardValidated)
            .collect();
        if !hard.is_empty() {
            bars(
                ui,
                "placar_hard",
                hard.iter().map(|r| d.unit_name(r.unit)).collect(),
                vec![
                    (
                        tl.target,
                        hard.iter().map(|r| r.target.unwrap_or(0.0)).collect(),
                        Color32::from_gray(150),
                    ),
                    (
                        tl.actual_official,
                        hard.iter().map(|r| r.actual).collect(),
                        theme::accent(ui),
                    ),
                ],
            );
        }
        ui.weak((tl.data_of)(&instant(
            d.scanned_ms,
            s.clock().utc_offset_minutes(),
        )));
    }

    pub(in crate::app) fn program_mbb(&mut self, ui: &mut egui::Ui, s: &Session, d: &Data) {
        let tl = l6();
        ui.heading(tl.nav_mbb);
        if !self.may_view(ui, s, Action::ViewMbbDashboard) {
            return;
        }
        self.filters(ui, d);
        let year = d.year(&self.l6);
        let f = Filter {
            unit: self.l6.unit_filter,
            year: Some(year),
        };
        ui.weak((tl.data_of)(&instant(
            d.scanned_ms,
            s.clock().utc_offset_minutes(),
        )));
        self.export_buttons(ui, false);
        self.run_export(s, d, false, year, &f);

        // Pipeline.
        ui.separator();
        ui.strong(tl.ind_funnel);
        let fun = painel::funnel(&d.ideas, &d.opps, &d.records, d.today, &f);
        egui::Grid::new("funil_mbb").striped(true).show(ui, |ui| {
            ui.strong(tl.stage);
            ui.strong(tl.count);
            ui.strong(tl.mean_age);
            ui.strong(tl.conversion);
            ui.end_row();
            for st in Stage::ALL {
                ui.label(stage_name(st));
                ui.label(fun.totals.get(&st).copied().unwrap_or(0).to_string());
                ui.label(
                    fun.mean_age
                        .get(&st)
                        .map_or(String::new(), |a| format!("{a:.0} d")),
                );
                ui.label(
                    fun.conversion
                        .iter()
                        .find(|c| c.0 == st)
                        .map_or(String::new(), |c| format!("→ {:.0} %", c.2 * 100.0)),
                );
                ui.end_row();
            }
        });
        bars(
            ui,
            "funil_barras",
            Stage::ALL
                .iter()
                .map(|s| stage_name(*s).to_string())
                .collect(),
            vec![(
                tl.count,
                Stage::ALL
                    .iter()
                    .map(|s| fun.totals.get(s).copied().unwrap_or(0) as f64)
                    .collect(),
                theme::accent(ui),
            )],
        );
        rule(ui, tl.rule_funnel);

        // Cycle time per phase.
        ui.separator();
        ui.strong(tl.ind_cycle);
        let ct = painel::cycle_times(&d.records, &f, None, |p| d.level_of(p));
        if ct.is_empty() {
            ui.weak(tl.no_data);
        }
        egui::Grid::new("ciclo").striped(true).show(ui, |ui| {
            for h in [
                tl.approach,
                tl.phase,
                tl.samples,
                tl.median,
                tl.p80,
                tl.target,
            ] {
                ui.strong(h);
            }
            ui.end_row();
            for c in &ct {
                ui.label(approach_name(c.approach));
                ui.label(&c.phase);
                ui.label(c.samples.len().to_string());
                let fmt = |x: Option<f64>| x.map_or("-".into(), |v| format!("{v:.0} d"));
                let late = c.median.zip(c.target).is_some_and(|(m, t)| m > t);
                if late {
                    ui.colored_label(theme::WARN, fmt(c.median));
                } else {
                    ui.label(fmt(c.median));
                }
                ui.label(fmt(c.p80));
                ui.label(fmt(c.target));
                ui.end_row();
            }
        });
        rule(ui, tl.rule_cycle);

        // Savings by state.
        ui.separator();
        ui.strong(tl.ind_savings);
        let sv = painel::savings_by_state(&d.records, &f);
        egui::Grid::new("ganhos_estado")
            .striped(true)
            .show(ui, |ui| {
                ui.strong(tl.unit);
                ui.strong(tl.category);
                for st in [
                    SavingStatus::Estimated,
                    SavingStatus::Forecast,
                    SavingStatus::Realized,
                    SavingStatus::Validated,
                ] {
                    ui.strong(saving_status_name(st));
                }
                ui.end_row();
                let mut units: Vec<Option<utsim_lss_core::Id>> = sv.keys().map(|k| k.0).collect();
                units.dedup();
                for u in units {
                    for c in [Category::Hard, Category::Soft] {
                        ui.label(d.unit_name(u));
                        ui.label(if c == Category::Hard { "hard" } else { "soft" });
                        for st in [
                            SavingStatus::Estimated,
                            SavingStatus::Forecast,
                            SavingStatus::Realized,
                            SavingStatus::Validated,
                        ] {
                            ui.label(money(sv.get(&(u, c, st)).copied().unwrap_or(0.0)));
                        }
                        ui.end_row();
                    }
                }
            });
        ui.label(RichText::new((tl.official_total)(&money(painel::official_total(&sv)))).strong());
        rule(ui, tl.rule_savings);

        // Belts by area.
        ui.separator();
        ui.strong(tl.ind_belts);
        let br = painel::belts_by_unit(&d.units, &d.members, &d.records, d.today);
        egui::Grid::new("belts_area").striped(true).show(ui, |ui| {
            for h in [
                tl.unit,
                tl.active,
                tl.trained,
                tl.certified,
                tl.coverage,
                tl.expiring,
            ] {
                ui.strong(h);
            }
            ui.end_row();
            let fmt = |m: &std::collections::BTreeMap<programa::BeltLevel, usize>| {
                m.iter()
                    .map(|(l, n)| format!("{} {n}", level_name(*l)))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            for r in br.iter().filter(|r| f.unit.is_none() || r.unit == f.unit) {
                ui.label(d.unit_name(r.unit));
                ui.label(fmt(&r.active));
                ui.label(fmt(&r.trained));
                ui.label(fmt(&r.certified));
                ui.label(r.coverage.map_or("-".into(), |c| format!("{c:.1}")));
                if r.expiring > 0 {
                    ui.colored_label(theme::WARN, r.expiring.to_string());
                } else {
                    ui.label("0");
                }
                ui.end_row();
            }
        });
        rule(ui, tl.rule_belts);

        // Tool usage.
        ui.separator();
        ui.strong(tl.ind_tools);
        let tools = utsim_lss_core::mentoria::all_tools();
        let tu = painel::tool_usage(&d.records, &tools, &d.map, &f);
        egui::Grid::new("uso_ferramentas")
            .striped(true)
            .show(ui, |ui| {
                ui.strong(tl.tool);
                ui.strong(tl.projects);
                ui.strong(tl.by_phase);
                ui.strong(tl.belts_applied);
                ui.end_row();
                for (tool, n) in &tu.projects {
                    ui.label(crate::text::l6::tool_label(tool));
                    ui.label(n.to_string());
                    let phases = tu.by_tool.get(tool).map_or(String::new(), |m| {
                        m.iter()
                            .map(|(i, n)| format!("{}: {n}", i + 1))
                            .collect::<Vec<_>>()
                            .join(" · ")
                    });
                    ui.label(phases);
                    ui.label(tu.belts.get(tool).copied().unwrap_or(0).to_string());
                    ui.end_row();
                }
            });
        if !tu.never_applied.is_empty() {
            let names: Vec<String> = tu
                .never_applied
                .iter()
                .map(|t| crate::text::l6::tool_label(t))
                .collect();
            ui.weak((tl.never_applied)(&names.join(", ")));
        }
        rule(ui, tl.rule_tools);

        // Overdue tollgates.
        ui.separator();
        ui.strong(tl.ind_overdue);
        let od = painel::overdue(&d.records, &d.reviews, d.today, &f);
        ui.label((tl.pending_reviews)(od.pending_reviews));
        for i in &od.items {
            let belts: Vec<String> = i.belts.iter().map(|b| d.person(*b)).collect();
            ui.colored_label(
                theme::WARN,
                (tl.overdue_line)(
                    &i.title,
                    &i.phase,
                    i.days_late,
                    &d.unit_name(i.unit),
                    &belts.join(", "),
                ),
            );
        }
        if od.items.is_empty() {
            ui.weak(tl.none_overdue);
        }
        rule(ui, tl.rule_overdue);

        // Workload.
        ui.separator();
        ui.strong(tl.ind_load);
        let wl = painel::workload(&d.loads);
        egui::Grid::new("carga_nivel").striped(true).show(ui, |ui| {
            for h in [
                tl.level,
                tl.people,
                tl.hours_committed,
                tl.capacity,
                tl.overloaded,
            ] {
                ui.strong(h);
            }
            ui.end_row();
            for (l, v) in &wl {
                ui.label(l.map_or(tl.no_level, level_name));
                ui.label(v.people.to_string());
                ui.label(format!("{:.1} h", v.hours));
                ui.label(format!("{:.1} h", v.capacity));
                ui.label(v.overloaded.to_string());
                ui.end_row();
            }
        });
        for l in d
            .loads
            .iter()
            .filter(|l| l.over_hours() || l.over_projects())
        {
            ui.colored_label(
                theme::WARN,
                (tl.person_load)(
                    &d.person(l.person),
                    l.hours,
                    l.capacity,
                    l.projects.len(),
                    l.project_limit,
                ),
            );
        }
        rule(ui, tl.rule_load);
        // The certification funnel (stage L7).
        ui.separator();
        self.certification_funnel(ui, s, d);
    }

    pub(in crate::app) fn program_leadership(&mut self, ui: &mut egui::Ui, s: &Session, d: &Data) {
        let tl = l6();
        ui.heading(tl.nav_leadership);
        if !self.may_view(ui, s, Action::ViewLeadership) {
            return;
        }
        ui.weak(tl.leadership_intro);
        self.filters(ui, d);
        let year = d.year(&self.l6);
        let f = Filter {
            unit: self.l6.unit_filter,
            year: Some(year),
        };
        self.export_buttons(ui, true);
        self.run_export(s, d, true, year, &f);
        let inputs = Inputs {
            state: s.state(),
            program: d.program,
            projects: &d.records,
            ideas: &d.ideas,
            members: &d.members,
            opportunities: &d.opps,
            reviews: &d.reviews,
            today: d.today,
        };
        let kpis = painel::leadership(&inputs, year, &f);
        ui.horizontal_wrapped(|ui| {
            for k in &kpis {
                theme::card(ui).show(ui, |ui| {
                    ui.set_width(230.0);
                    ui.strong(kpi_name(k.kpi));
                    let v = if k.kpi == Kpi::ValidatedSavings {
                        money(k.value)
                    } else if k.kpi == Kpi::Coverage {
                        format!("{:.1}", k.value)
                    } else {
                        format!("{:.0}", k.value)
                    };
                    ui.label(RichText::new(v).size(22.0));
                    if let Some(t) = k.target {
                        let t = if k.kpi == Kpi::ValidatedSavings {
                            money(t)
                        } else {
                            show_number(t)
                        };
                        ui.label((tl.target_is)(&t));
                    }
                    let (arrow, color) = match k.trend() {
                        1 => ("▲", theme::accent(ui)),
                        -1 => ("▼", theme::WARN),
                        _ => ("■", Color32::from_gray(140)),
                    };
                    if k.previous.is_some() {
                        ui.colored_label(color, format!("{arrow} {}", tl.vs_last_year));
                    }
                    rule(ui, kpi_rule(k.kpi));
                });
            }
        });
        let sv = painel::savings_by_state(&d.records, &f);
        let units: Vec<Option<utsim_lss_core::Id>> = d.units.iter().map(|u| Some(u.id)).collect();
        if !units.is_empty() {
            bars(
                ui,
                "lideranca_ganhos",
                units.iter().map(|u| d.unit_name(*u)).collect(),
                vec![(
                    tl.validated,
                    units
                        .iter()
                        .map(|u| {
                            sv.get(&(*u, Category::Hard, SavingStatus::Validated))
                                .copied()
                                .unwrap_or(0.0)
                                + sv.get(&(*u, Category::Soft, SavingStatus::Validated))
                                    .copied()
                                    .unwrap_or(0.0)
                        })
                        .collect(),
                    theme::accent(ui),
                )],
            );
        }
        ui.weak((tl.data_of)(&instant(
            d.scanned_ms,
            s.clock().utc_offset_minutes(),
        )));
    }
}

// --- Export (version 2) ----------------------------------------------------------------------

/// Which dashboard to export, and how.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Export {
    Pdf,
    Xlsx,
    /// The annual report of the program (PDF): the leadership indicators with target and
    /// last year, the comparison between the units and the MBB detail.
    Annual,
}

const STATES: [SavingStatus; 4] = [
    SavingStatus::Estimated,
    SavingStatus::Forecast,
    SavingStatus::Realized,
    SavingStatus::Validated,
];

/// The MBB dashboard as a document: every indicator as a table, with its rule, the funnel
/// and the cycle times as charts, and the instant of the data.
fn mbb_doc(s: &Session, d: &Data, f: &Filter) -> Document {
    let tl = l6();
    let mut doc = Document::new(tl.nav_mbb, &s.folder().title());
    doc.para(&(tl.data_of)(&instant(
        d.scanned_ms,
        s.clock().utc_offset_minutes(),
    )));
    let fun = painel::funnel(&d.ideas, &d.opps, &d.records, d.today, f);
    doc.heading(tl.ind_funnel);
    doc.table(
        tl.ind_funnel,
        &[tl.stage, tl.count, tl.mean_age, tl.conversion],
        Stage::ALL
            .iter()
            .map(|st| {
                vec![
                    stage_name(*st).to_string(),
                    fun.totals.get(st).copied().unwrap_or(0).to_string(),
                    fun.mean_age
                        .get(st)
                        .map_or(String::new(), |a| format!("{a:.0} d")),
                    fun.conversion
                        .iter()
                        .find(|c| c.0 == *st)
                        .map_or(String::new(), |c| format!("{:.0} %", c.2 * 100.0)),
                ]
            })
            .collect(),
        &[],
    );
    doc.chart(Chart {
        title: tl.ind_funnel.into(),
        bars: true,
        labels: Stage::ALL
            .iter()
            .map(|s| stage_name(*s).to_string())
            .collect(),
        series: vec![(
            String::new(),
            Stage::ALL
                .iter()
                .map(|s| fun.totals.get(s).copied().unwrap_or(0) as f64)
                .collect(),
        )],
        ..Chart::default()
    });
    doc.para(tl.rule_funnel);

    let ct = painel::cycle_times(&d.records, f, None, |p| d.level_of(p));
    doc.heading(tl.ind_cycle);
    let fmt = |x: Option<f64>| x.map_or("-".into(), |v| format!("{v:.0} d"));
    doc.table(
        tl.ind_cycle,
        &[
            tl.approach,
            tl.phase,
            tl.samples,
            tl.median,
            tl.p80,
            tl.target,
        ],
        ct.iter()
            .map(|c| {
                vec![
                    approach_name(c.approach).to_string(),
                    c.phase.clone(),
                    c.samples.len().to_string(),
                    fmt(c.median),
                    fmt(c.p80),
                    fmt(c.target),
                ]
            })
            .collect(),
        &[],
    );
    if !ct.is_empty() {
        doc.chart(Chart {
            title: tl.ind_cycle.into(),
            bars: true,
            labels: ct.iter().map(|c| c.phase.clone()).collect(),
            series: vec![
                (
                    tl.median.into(),
                    ct.iter().map(|c| c.median.unwrap_or(f64::NAN)).collect(),
                ),
                (
                    tl.target.into(),
                    ct.iter().map(|c| c.target.unwrap_or(f64::NAN)).collect(),
                ),
            ],
            ..Chart::default()
        });
    }
    doc.para(tl.rule_cycle);

    let sv = painel::savings_by_state(&d.records, f);
    doc.heading(tl.ind_savings);
    let mut units: Vec<Option<utsim_lss_core::Id>> = sv.keys().map(|k| k.0).collect();
    units.dedup();
    let mut head = vec![tl.unit, tl.category];
    head.extend(STATES.iter().map(|s| saving_status_name(*s)));
    doc.table(
        tl.ind_savings,
        &head,
        units
            .iter()
            .flat_map(|u| {
                [Category::Hard, Category::Soft]
                    .into_iter()
                    .map(move |c| (*u, c))
            })
            .map(|(u, c)| {
                let mut r = vec![
                    d.unit_name(u),
                    if c == Category::Hard { "hard" } else { "soft" }.to_string(),
                ];
                r.extend(
                    STATES
                        .iter()
                        .map(|st| money(sv.get(&(u, c, *st)).copied().unwrap_or(0.0))),
                );
                r
            })
            .collect(),
        &[],
    );
    doc.para(&(tl.official_total)(&money(painel::official_total(&sv))));
    doc.para(tl.rule_savings);

    let br = painel::belts_by_unit(&d.units, &d.members, &d.records, d.today);
    doc.heading(tl.ind_belts);
    let lv = |m: &std::collections::BTreeMap<programa::BeltLevel, usize>| {
        m.iter()
            .map(|(l, n)| format!("{} {n}", level_name(*l)))
            .collect::<Vec<_>>()
            .join(", ")
    };
    doc.table(
        tl.ind_belts,
        &[
            tl.unit,
            tl.active,
            tl.trained,
            tl.certified,
            tl.coverage,
            tl.expiring,
        ],
        br.iter()
            .filter(|r| f.unit.is_none() || r.unit == f.unit)
            .map(|r| {
                vec![
                    d.unit_name(r.unit),
                    lv(&r.active),
                    lv(&r.trained),
                    lv(&r.certified),
                    r.coverage.map_or("-".into(), |c| format!("{c:.1}")),
                    r.expiring.to_string(),
                ]
            })
            .collect(),
        &[],
    );
    doc.para(tl.rule_belts);

    let tools = utsim_lss_core::mentoria::all_tools();
    let tu = painel::tool_usage(&d.records, &tools, &d.map, f);
    doc.heading(tl.ind_tools);
    doc.table(
        tl.ind_tools,
        &[tl.tool, tl.projects, tl.belts_applied],
        tu.projects
            .iter()
            .map(|(tool, n)| {
                vec![
                    crate::text::l6::tool_label(tool),
                    n.to_string(),
                    tu.belts.get(tool).copied().unwrap_or(0).to_string(),
                ]
            })
            .collect(),
        &[],
    );
    doc.para(tl.rule_tools);

    let od = painel::overdue(&d.records, &d.reviews, d.today, f);
    doc.heading(tl.ind_overdue);
    doc.para(&(tl.pending_reviews)(od.pending_reviews));
    doc.bullets(
        od.items
            .iter()
            .map(|i| {
                let belts: Vec<String> = i.belts.iter().map(|b| d.person(*b)).collect();
                (tl.overdue_line)(
                    &i.title,
                    &i.phase,
                    i.days_late,
                    &d.unit_name(i.unit),
                    &belts.join(", "),
                )
            })
            .collect(),
    );
    doc.para(tl.rule_overdue);

    let wl = painel::workload(&d.loads);
    doc.heading(tl.ind_load);
    doc.table(
        tl.ind_load,
        &[
            tl.level,
            tl.people,
            tl.hours_committed,
            tl.capacity,
            tl.overloaded,
        ],
        wl.iter()
            .map(|(l, v)| {
                vec![
                    l.map_or(tl.no_level, level_name).to_string(),
                    v.people.to_string(),
                    format!("{:.1}", v.hours),
                    format!("{:.1}", v.capacity),
                    v.overloaded.to_string(),
                ]
            })
            .collect(),
        &[],
    );
    doc.para(tl.rule_load);
    doc
}

/// The annual report of the program: the six indicators of the year with target and last
/// year, the same indicators unit by unit (with the charts of validated savings and closed
/// projects), and the detail of the MBB dashboard.
fn annual_doc(s: &Session, d: &Data, year: i32) -> Document {
    let tl = l6();
    let t = crate::text::l10::l10();
    let all = Filter {
        unit: None,
        year: Some(year),
    };
    let mut doc = leadership_doc(s, d, year, &all);
    doc.title = (t.annual_title)(year);
    // The leadership document starts with the instant and the table of the indicators.
    doc.blocks.insert(1, Block::Heading(t.annual_kpis.into()));
    let inputs = Inputs {
        state: s.state(),
        program: d.program,
        projects: &d.records,
        ideas: &d.ideas,
        members: &d.members,
        opportunities: &d.opps,
        reviews: &d.reviews,
        today: d.today,
    };
    let units: Vec<&programa::Unit> = d.units.iter().collect();
    if !units.is_empty() {
        doc.heading(t.annual_units);
        let per: Vec<Vec<painel::KpiValue>> = units
            .iter()
            .map(|u| {
                painel::leadership(
                    &inputs,
                    year,
                    &Filter {
                        unit: Some(u.id),
                        year: Some(year),
                    },
                )
            })
            .collect();
        let mut head = vec![tl.unit];
        head.extend(Kpi::ALL.iter().map(|k| kpi_name(*k)));
        let value =
            |ks: &[painel::KpiValue], k: Kpi| ks.iter().find(|x| x.kpi == k).map(|x| x.value);
        doc.table(
            t.annual_units,
            &head,
            units
                .iter()
                .zip(&per)
                .map(|(u, ks)| {
                    let mut r = vec![u.name.clone()];
                    r.extend(Kpi::ALL.iter().map(|k| match value(ks, *k) {
                        Some(v) if *k == Kpi::ValidatedSavings => money(v),
                        Some(v) => show_number((v * 10.0).round() / 10.0),
                        None => String::new(),
                    }));
                    r
                })
                .collect(),
            &[],
        );
        for k in [Kpi::ValidatedSavings, Kpi::ProjectsClosed] {
            doc.chart(Chart {
                title: kpi_name(k).into(),
                bars: true,
                labels: units.iter().map(|u| u.name.clone()).collect(),
                series: vec![(
                    String::new(),
                    per.iter().map(|ks| value(ks, k).unwrap_or(0.0)).collect(),
                )],
                ..Chart::default()
            });
        }
    }
    doc.blocks.push(Block::PageBreak);
    doc.heading(t.annual_detail);
    let detail = mbb_doc(s, d, &all);
    doc.blocks.extend(detail.blocks);
    doc
}

/// The leadership dashboard as a document: the six indicators with target, last year and
/// rule, and the validated savings per unit. No personal data, as on the screen.
fn leadership_doc(s: &Session, d: &Data, year: i32, f: &Filter) -> Document {
    let tl = l6();
    let inputs = Inputs {
        state: s.state(),
        program: d.program,
        projects: &d.records,
        ideas: &d.ideas,
        members: &d.members,
        opportunities: &d.opps,
        reviews: &d.reviews,
        today: d.today,
    };
    let kpis = painel::leadership(&inputs, year, f);
    let mut doc = Document::new(
        tl.nav_leadership,
        &format!("{} · {year}", s.folder().title()),
    );
    doc.para(&(tl.data_of)(&instant(
        d.scanned_ms,
        s.clock().utc_offset_minutes(),
    )));
    let show = |k: Kpi, v: f64| {
        if k == Kpi::ValidatedSavings {
            money(v)
        } else {
            show_number((v * 10.0).round() / 10.0)
        }
    };
    doc.table(
        tl.nav_leadership,
        &["Indicador", "Valor", "Meta", "Ano anterior", "Regra"],
        kpis.iter()
            .map(|k| {
                vec![
                    kpi_name(k.kpi).to_string(),
                    show(k.kpi, k.value),
                    k.target.map(|t| show(k.kpi, t)).unwrap_or_default(),
                    k.previous.map(|t| show(k.kpi, t)).unwrap_or_default(),
                    kpi_rule(k.kpi).to_string(),
                ]
            })
            .collect(),
        &[1.6, 1.0, 1.0, 1.0, 3.0],
    );
    let sv = painel::savings_by_state(&d.records, f);
    let units: Vec<Option<utsim_lss_core::Id>> = d.units.iter().map(|u| Some(u.id)).collect();
    if !units.is_empty() {
        doc.chart(Chart {
            title: tl.validated.into(),
            bars: true,
            labels: units.iter().map(|u| d.unit_name(*u)).collect(),
            series: vec![(
                String::new(),
                units
                    .iter()
                    .map(|u| {
                        [Category::Hard, Category::Soft]
                            .iter()
                            .map(|c| {
                                sv.get(&(*u, *c, SavingStatus::Validated))
                                    .copied()
                                    .unwrap_or(0.0)
                            })
                            .sum()
                    })
                    .collect(),
            )],
            ..Chart::default()
        });
    }
    doc
}

impl App {
    /// Asks the next drawing of a dashboard (MBB or leadership) to export it, without
    /// opening the file (tests and tools; the buttons open it).
    pub fn request_dashboard_export(&mut self, leadership: bool, how: Export) {
        self.l6.export = Some((leadership, how, false));
    }

    /// The file of the last dashboard export.
    pub fn last_dashboard_export(&self) -> Option<&std::path::Path> {
        self.l6.exported.as_deref()
    }

    fn export_buttons(&mut self, ui: &mut egui::Ui, leadership: bool) {
        let t = crate::text::l10::l10();
        ui.horizontal(|ui| {
            if ui.button(t.dash_pdf).clicked() {
                self.l6.export = Some((leadership, Export::Pdf, true));
            }
            if ui.button(t.dash_xlsx).clicked() {
                self.l6.export = Some((leadership, Export::Xlsx, true));
            }
            if leadership && ui.button(t.dash_annual).clicked() {
                self.l6.export = Some((true, Export::Annual, true));
            }
        });
    }

    /// Does the export asked for this dashboard, if any.
    fn run_export(&mut self, s: &Session, d: &Data, leadership: bool, year: i32, f: &Filter) {
        let Some((which, how, open)) = self.l6.export else {
            return;
        };
        if which != leadership {
            return;
        }
        self.l6.export = None;
        let doc = if how == Export::Annual {
            annual_doc(s, d, year)
        } else if leadership {
            leadership_doc(s, d, year, f)
        } else {
            mbb_doc(s, d, f)
        };
        let (name, bytes) = match how {
            Export::Pdf | Export::Annual => {
                let tp = self.report_template(s);
                let footer =
                    (crate::text::l5::l5().printed_by)(&s.folder().me().name, &d.today.format_br());
                (
                    crate::app::screens::reports::file_name(&doc.title, "pdf"),
                    pdf::render(&doc, &tp, &footer),
                )
            }
            Export::Xlsx => match utsim_lss_store::planilha::xlsx(&doc.tables()) {
                Ok(b) => (
                    crate::app::screens::reports::file_name(&doc.title, "xlsx"),
                    b,
                ),
                Err(e) => {
                    self.set_status((crate::text::texts().error)(&e.to_string()), true);
                    return;
                }
            },
        };
        match s.folder().write_report(&name, &bytes) {
            Ok(p) => {
                if open {
                    let _ = crate::app::screens::model::open_with_system(&p);
                }
                self.set_status(
                    (crate::text::l5::l5().saved_at)(&p.display().to_string()),
                    false,
                );
                self.l6.exported = Some(p);
            }
            Err(e) => self.set_status((crate::text::texts().error)(&e.to_string()), true),
        }
    }
}
