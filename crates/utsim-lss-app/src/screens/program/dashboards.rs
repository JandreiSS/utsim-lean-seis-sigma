//! The scoreboard (8.1) and the two dashboards of 8.7: the MBB view, with every indicator
//! down to the project and the person, and the leadership view, with six indicators,
//! target and trend and no personal data. Every indicator shows its rule and the instant
//! of the data (LSS-RNF-16).

use eframe::egui::{self, Color32, RichText};
use egui_plot::{Bar, BarChart, Legend, Plot};
use utsim_lss_core::domain::SavingStatus;
use utsim_lss_core::painel::{self, Filter, Inputs, Kpi};
use utsim_lss_core::perfis::Action;
use utsim_lss_core::programa::{self, Stage};
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
