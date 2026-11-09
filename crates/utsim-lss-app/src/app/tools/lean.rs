//! The editors of the P6 tools (`docs/plano_lean_seis_sigma.md`, 1.5 and 2.4): selection of
//! solutions, 5S, SMED, standardised work, the kaizen event and the PDCA cycles. Drawing
//! only: the numbers come from `utsim_lss_core::tools::{lean, kaizen, pdca}` (and through
//! them from the common `utsim_ferramentas`), every change is one field through
//! [`Session`].

use eframe::egui::{self, Align2, Color32, FontId, RichText, Stroke, pos2, vec2};
use egui_plot::{Bar, BarChart, HLine, Legend, Line, Plot, PlotPoints};
use serde_json::Value;
use utsim_lss_core::Id as Uid;
use utsim_lss_core::tools::lean::{
    self, Method, Quadrant, Sense, changeover, five_s, selection, standard_work,
};
use utsim_lss_core::tools::{Deliverable, ItemRef, item_ref, kaizen, pdca};

use super::{App, Col, fmt_num, or_untitled, pct};
use crate::session::Session;
use crate::text::l12::l12;
use crate::theme;

const SENSES: &[(&str, &str)] = &[
    ("utilizacao", "Utilização"),
    ("organizacao", "Organização"),
    ("limpeza", "Limpeza"),
    ("padronizacao", "Padronização"),
    ("disciplina", "Disciplina"),
];
const CLASSES: &[(&str, &str)] = &[("interna", "interna"), ("externa", "externa")];
const CLASSES_AFTER: &[(&str, &str)] = &[
    ("interna", "interna"),
    ("externa", "externa"),
    ("eliminada", "eliminada"),
];

fn numbered(
    values: impl IntoIterator<Item = i64>,
    label: impl Fn(i64) -> String,
) -> Vec<(Value, String)> {
    std::iter::once((Value::from(""), "-".to_string()))
        .chain(values.into_iter().map(|v| (Value::from(v), label(v))))
        .collect()
}

fn quadrant_name(q: Quadrant) -> &'static str {
    let t = l12();
    match q {
        Quadrant::QuickWin => t.q_quick,
        Quadrant::Plan => t.q_plan,
        Quadrant::Filler => t.q_filler,
        Quadrant::Drop => t.q_drop,
    }
}

fn step_name(s: pdca::Step) -> &'static str {
    let t = l12();
    match s {
        pdca::Step::Plan => t.step_plan,
        pdca::Step::Do => t.step_do,
        pdca::Step::Check => t.step_check,
        pdca::Step::Act => t.step_act,
        pdca::Step::Closed => t.step_closed,
    }
}

fn secs(x: f64) -> String {
    format!("{} s", fmt_num((x * 10.0).round() / 10.0))
}

fn mins(x: f64) -> String {
    format!("{} min", fmt_num((x * 10.0).round() / 10.0))
}

impl App {
    // ---- Selection of solutions ------------------------------------------------------

    pub(super) fn selection(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        use selection::fields as f;
        let t = l12();
        let dr = Deliverable::entity_ref(del);
        let m = selection::matrix(s.state(), del);
        ui.horizontal(|ui| {
            ui.label(t.method);
            for (mm, key, label) in [
                (Method::Weighted, "ponderada", t.weighted),
                (Method::Pugh, "pugh", t.pugh),
            ] {
                if ui.radio(m.method == mm, label).clicked() && m.method != mm {
                    let r = s.set_field(dr, f::METHOD, Value::from(key), None);
                    self.report(r);
                }
            }
        });
        ui.weak(match m.method {
            Method::Weighted => t.weighted_help,
            Method::Pugh => t.pugh_help,
        });
        ui.add_space(4.0);
        ui.strong(t.criteria);
        let weights = numbered(1..=10, |v| v.to_string());
        egui::Grid::new(("criterios", del))
            .striped(true)
            .show(ui, |ui| {
                for h in [t.criterion, t.weight, ""] {
                    ui.strong(h);
                }
                ui.end_row();
                for (id, _, _) in &m.criteria {
                    let r = item_ref(*id);
                    self.text_field(ui, s, r, f::NAME, false, 240.0);
                    self.combo_field(ui, s, r, f::WEIGHT, &weights, 40.0);
                    ui.horizontal(|ui| self.row_controls(ui, s, *id));
                    ui.end_row();
                }
            });
        if let Some(text) = self.add_box(ui, format!("criterio_{del}"), t.criterion_hint, 240.0) {
            let r = s.add_item(
                del,
                selection::CRITERION,
                None,
                vec![(f::NAME, Value::from(text)), (f::WEIGHT, Value::from(5))],
            );
            self.report(r);
        }
        ui.add_space(8.0);
        ui.strong(t.options);
        let scores: Vec<(Value, String)> = match m.method {
            Method::Weighted => numbered(1..=5, |v| v.to_string()),
            Method::Pugh => numbered([1, 0, -1], |v| {
                match v {
                    1 => "+",
                    0 => "S",
                    _ => "−",
                }
                .to_string()
            }),
        };
        let pugh = m.method == Method::Pugh;
        egui::Grid::new(("opcoes", del))
            .striped(true)
            .show(ui, |ui| {
                ui.strong(t.option);
                if pugh {
                    ui.strong(t.reference);
                }
                for (_, name, w) in &m.criteria {
                    ui.strong(format!("{} ({w})", or_untitled(name)));
                }
                ui.strong(t.total);
                ui.strong(if pugh { t.plus_same_minus } else { t.of_max });
                ui.strong(t.rank);
                ui.strong(t.impact);
                ui.strong(t.effort);
                ui.end_row();
                for (row, res) in m.rows.iter().zip(&m.result.options) {
                    let r = item_ref(row.id);
                    self.text_field(ui, s, r, f::NAME, false, 180.0);
                    if pugh && ui.radio(row.reference, "").clicked() && !row.reference {
                        let e = selection::set_reference(s.state(), del, row.id);
                        let res = s.apply(e);
                        self.report(res);
                    }
                    for (c, _, _) in &m.criteria {
                        if pugh && row.reference {
                            ui.weak("S");
                        } else {
                            self.combo_field(ui, s, r, &lean::score_field(*c), &scores, 40.0);
                        }
                    }
                    ui.strong(fmt_num(res.total));
                    if pugh {
                        ui.label(format!("{} / {} / {}", res.plus, res.same, res.minus));
                    } else {
                        ui.label(pct(res.pct));
                    }
                    ui.label(format!("{}º", res.rank));
                    self.rating_field(ui, s, r, f::IMPACT);
                    self.rating_field(ui, s, r, f::EFFORT);
                    ui.horizontal(|ui| self.row_controls(ui, s, row.id));
                    ui.end_row();
                }
            });
        if let Some(text) = self.add_box(ui, format!("opcao_{del}"), t.option_hint, 240.0) {
            let r = s.add_item(
                del,
                selection::OPTION,
                None,
                vec![(f::NAME, Value::from(text))],
            );
            self.report(r);
        }
        if m.result.reference_issue && !m.rows.is_empty() {
            ui.colored_label(theme::WARN, t.need_reference);
        }
        if let Some(b) = m.result.best() {
            ui.label(RichText::new((t.best)(&or_untitled(&b.name))).strong());
        } else if !m.rows.is_empty() {
            ui.weak(t.incomplete);
        }
        ui.add_space(8.0);
        ui.strong(t.grid);
        egui::Grid::new(("grade", del)).show(ui, |ui| {
            for q in [
                Quadrant::QuickWin,
                Quadrant::Plan,
                Quadrant::Filler,
                Quadrant::Drop,
            ] {
                ui.label(RichText::new(quadrant_name(q)).strong());
                let names: Vec<String> = m
                    .rows
                    .iter()
                    .filter(|r| r.quadrant == Some(q))
                    .map(|r| or_untitled(&r.name))
                    .collect();
                if names.is_empty() {
                    ui.weak("-");
                } else {
                    ui.label(names.join(", "));
                }
                ui.end_row();
            }
        });
    }

    // ---- 5S ----------------------------------------------------------------------------

    pub(super) fn five_s(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        use five_s::fields as f;
        let t = l12();
        ui.horizontal(|ui| {
            ui.strong(t.checklist);
            if ui.button(t.starter).on_hover_text(t.starter_help).clicked() {
                let e = five_s::starter_edits(s.state(), s.project(), del, || s.new_id());
                let r = s.apply(e);
                self.report(r);
            }
        });
        self.item_table(
            ui,
            s,
            del,
            five_s::QUESTION,
            &[
                Col::Choice(t.sense, f::SENSE, SENSES, 130.0),
                Col::Text(t.question, f::TEXT, 420.0),
            ],
            None,
        );
        ui.add_space(8.0);
        let qs = five_s::questions(s.state(), del);
        let audits = five_s::audits(s.state(), del);
        ui.horizontal(|ui| {
            ui.strong(t.audits);
            if ui.button(t.new_audit).clicked() {
                let today = s.today().to_string();
                let r = s.add_item(
                    del,
                    five_s::AUDIT,
                    None,
                    vec![(f::DATE, Value::from(today))],
                );
                self.report(r);
            }
        });
        if audits.is_empty() {
            ui.weak(t.no_audits);
            return;
        }
        let scores = numbered(0..=4, |v| v.to_string());
        egui::ScrollArea::horizontal()
            .id_salt(("auditorias", del))
            .show(ui, |ui| {
                egui::Grid::new(("auditorias_grade", del))
                    .striped(true)
                    .show(ui, |ui| {
                        ui.strong(t.sense);
                        ui.strong(t.question);
                        for a in &audits {
                            ui.vertical(|ui| {
                                self.date_field(ui, s, item_ref(a.id), f::DATE);
                                self.text_field(ui, s, item_ref(a.id), f::AUDITOR, false, 110.0);
                            });
                        }
                        ui.end_row();
                        for q in &qs {
                            ui.label(q.sense.map_or("-", Sense::name));
                            ui.label(or_untitled(&q.text));
                            for a in &audits {
                                let field = lean::score_field(q.id);
                                self.combo_field(ui, s, item_ref(a.id), &field, &scores, 40.0);
                            }
                            ui.end_row();
                        }
                        for (i, sense) in Sense::ALL.iter().enumerate() {
                            ui.strong(sense.name());
                            ui.weak(t.sense_pct);
                            for a in &audits {
                                ui.label(a.score.senses[i].pct.map_or("-".into(), pct));
                            }
                            ui.end_row();
                        }
                        ui.strong(t.total);
                        ui.label("");
                        for a in &audits {
                            ui.strong(a.score.pct.map_or("-".into(), pct));
                        }
                        ui.end_row();
                        ui.label("");
                        ui.label("");
                        for a in &audits {
                            ui.horizontal(|ui| self.row_controls(ui, s, a.id));
                        }
                        ui.end_row();
                    });
            });
        let last = audits.last().expect("not empty");
        if let Some(w) = last.score.weakest() {
            ui.label((t.weakest)(w.name()));
        }
        ui.horizontal(|ui| {
            radar(ui, &audits);
            if audits.len() > 1 {
                let pts: Vec<[f64; 2]> = audits
                    .iter()
                    .enumerate()
                    .filter_map(|(i, a)| a.score.pct.map(|p| [i as f64 + 1.0, p]))
                    .collect();
                ui.vertical(|ui| {
                    let accent = theme::accent(ui);
                    ui.weak(t.history);
                    Plot::new(("historico_5s", del))
                        .width(360.0)
                        .height(220.0)
                        .allow_scroll(false)
                        .allow_zoom(false)
                        .allow_drag(false)
                        .include_y(0.0)
                        .include_y(100.0)
                        .y_axis_formatter(|m, _| format!("{} %", m.value))
                        .show(ui, |p| {
                            p.line(Line::new(t.total, PlotPoints::new(pts)).color(accent));
                        });
                });
            }
        });
    }

    // ---- SMED --------------------------------------------------------------------------

    pub(super) fn smed(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        use changeover::fields as f;
        let t = l12();
        self.item_table(
            ui,
            s,
            del,
            changeover::ACTIVITY,
            &[
                Col::Text(t.activity, f::NAME, 220.0),
                Col::Number(t.time_min, f::TIME, 60.0),
                Col::Choice(t.class_before, f::CLASS, CLASSES, 90.0),
                Col::Choice(t.class_after, f::CLASS_AFTER, CLASSES_AFTER, 100.0),
                Col::Number(t.time_after, f::TIME_AFTER, 60.0),
                Col::Text(t.how, f::HOW, 220.0),
            ],
            None,
        );
        let st = changeover::study(s.state(), del);
        ui.add_space(8.0);
        egui::Grid::new(("smed_soma", del))
            .striped(true)
            .show(ui, |ui| {
                for h in ["", t.internal_setup, t.external, t.removed] {
                    ui.strong(h);
                }
                ui.end_row();
                for (label, side) in [(t.before, &st.before), (t.after, &st.after)] {
                    ui.strong(label);
                    ui.label(mins(side.internal));
                    ui.label(mins(side.external));
                    ui.label(mins(side.removed));
                    ui.end_row();
                }
            });
        if let Some(r) = st.reduction_pct {
            ui.label(RichText::new((t.reduction)(&pct(r), st.converted)).strong());
        }
        if st.before.incomplete + st.after.incomplete > 0 {
            ui.colored_label(theme::WARN, t.smed_incomplete);
        }
        if st.before.internal > 0.0 {
            let accent = theme::accent(ui);
            let gray = Color32::from_gray(150);
            Plot::new(("smed_barras", del))
                .legend(Legend::default())
                .width(360.0)
                .height(200.0)
                .allow_scroll(false)
                .allow_zoom(false)
                .allow_drag(false)
                .include_y(0.0)
                .x_axis_formatter(|m, _| {
                    match m.value.round() as i64 {
                        0 => l12().before,
                        1 => l12().after,
                        _ => "",
                    }
                    .to_string()
                })
                .show(ui, |p| {
                    let int = BarChart::new(
                        t.internal_setup,
                        vec![
                            Bar::new(0.0, st.before.internal).width(0.5),
                            Bar::new(1.0, st.after.internal).width(0.5),
                        ],
                    )
                    .color(accent);
                    let ext = BarChart::new(
                        t.external,
                        vec![
                            Bar::new(0.0, st.before.external).width(0.5),
                            Bar::new(1.0, st.after.external).width(0.5),
                        ],
                    )
                    .color(gray)
                    .stack_on(&[&int]);
                    p.bar_chart(int);
                    p.bar_chart(ext);
                });
        }
    }

    // ---- Standardised work -------------------------------------------------------------

    pub(super) fn standard_work(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        use standard_work::fields as f;
        let t = l12();
        let dr = Deliverable::entity_ref(del);
        ui.horizontal_wrapped(|ui| {
            ui.label(t.available);
            self.num_field(ui, s, dr, f::AVAILABLE, 70.0);
            ui.label(t.demand);
            self.num_field(ui, s, dr, f::DEMAND, 60.0);
            ui.label(t.takt_typed);
            self.num_field(ui, s, dr, f::TAKT, 60.0);
        });
        let b = standard_work::balance(s.state(), del);
        match b.takt {
            Some(tk) => ui.label(RichText::new((t.takt_is)(&secs(tk))).strong()),
            None => ui.weak(t.no_takt),
        };
        ui.add_space(4.0);
        self.item_table(
            ui,
            s,
            del,
            standard_work::ELEMENT,
            &[
                Col::Text(t.operator, f::OPERATOR, 100.0),
                Col::Text(t.element, f::NAME, 200.0),
                Col::Number(t.manual, f::MANUAL, 60.0),
                Col::Number(t.auto, f::AUTO, 60.0),
                Col::Number(t.walk, f::WALK, 60.0),
                Col::Number(t.wip, f::WIP, 60.0),
            ],
            None,
        );
        if b.operators.is_empty() {
            return;
        }
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            ui.label((t.content)(&secs(b.total_content)));
            if let Some(n) = b.min_operators {
                ui.label((t.min_operators)(n, b.operators.len()));
            }
            if let Some(e) = b.efficiency {
                ui.label((t.efficiency)(&pct(e)));
            }
            ui.label((t.total_wip)(&fmt_num(b.total_wip)));
        });
        let over: Vec<&str> = b
            .operators
            .iter()
            .filter(|o| o.over_takt)
            .map(|o| o.name.as_str())
            .collect();
        if !over.is_empty() {
            ui.colored_label(theme::WARN, (t.over_takt)(&over.join(", ")));
        }
        ui.strong(t.balance);
        let accent = theme::accent(ui);
        let warn = theme::WARN;
        let names: Vec<String> = b.operators.iter().map(|o| or_untitled(&o.name)).collect();
        let bars: Vec<Bar> = b
            .operators
            .iter()
            .enumerate()
            .map(|(i, o)| {
                Bar::new(i as f64, o.content)
                    .width(0.5)
                    .name(&names[i])
                    .fill(if o.over_takt { warn } else { accent })
            })
            .collect();
        let n = names.len();
        let takt = b.takt;
        Plot::new(("balanceamento", del))
            .width(560.0)
            .height(220.0)
            .include_x(-0.6)
            .include_x(n as f64 - 0.4)
            .allow_scroll(false)
            .allow_zoom(false)
            .allow_drag(false)
            .include_y(0.0)
            .include_y(takt.unwrap_or(0.0) * 1.1)
            .x_axis_formatter(move |mark, _| {
                let k = mark.value.round();
                if (mark.value - k).abs() < 1e-6 && k >= 0.0 && (k as usize) < n {
                    names[k as usize].clone()
                } else {
                    String::new()
                }
            })
            .y_axis_formatter(|m, _| format!("{} s", m.value))
            .show(ui, |p| {
                p.bar_chart(BarChart::new(t.content_s, bars));
                if let Some(tk) = takt {
                    p.hline(HLine::new(t.takt, tk).color(ui_red()));
                }
            });
        ui.strong(t.combination);
        ui.weak(t.combination_help);
        // One time scale for every operator, so the takt lines up.
        let span = b
            .operators
            .iter()
            .flat_map(|o| {
                o.steps
                    .iter()
                    .map(|s| s.start + s.manual + s.auto.max(s.walk))
            })
            .fold(b.takt.unwrap_or(0.0), f64::max)
            .max(1.0)
            * 1.05;
        for o in &b.operators {
            combination(ui, o, b.takt, span);
        }
    }

    // ---- Kaizen event ------------------------------------------------------------------

    pub(super) fn kaizen_event(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        use kaizen::fields as f;
        let t = l12();
        let dr = Deliverable::entity_ref(del);
        let ev = kaizen::event(s.state(), del);
        ui.horizontal_wrapped(|ui| {
            ui.label(t.start);
            self.date_field(ui, s, dr, f::START);
            ui.label(t.days);
            let opts: Vec<(Value, String)> = kaizen::DAYS
                .map(|d| (Value::from(d), d.to_string()))
                .collect();
            self.combo_field(ui, s, dr, f::DAYS, &opts, 40.0);
            if let Some(end) = ev.end {
                ui.weak((t.ends)(&end.format_br()));
            }
            if ui.button(t.fill_template).clicked() {
                let e = kaizen::template_edits(s.state(), s.project(), del, || s.new_id());
                let r = s.apply(e);
                self.report(r);
            }
        });
        ui.add_space(4.0);
        ui.strong(t.prep);
        let prep: Vec<Uid> = utsim_lss_core::tools::items(s.state(), del, kaizen::PREP)
            .iter()
            .map(|i| i.id)
            .collect();
        for id in prep {
            ui.horizontal(|ui| {
                let text = s
                    .state()
                    .entity(id)
                    .map(|e| e.text(f::TEXT))
                    .unwrap_or_default();
                self.check_field(ui, s, item_ref(id), f::DONE, &text);
                self.row_controls(ui, s, id);
            });
        }
        if let Some(text) = self.add_box(ui, format!("preparo_{del}"), t.prep_hint, 320.0) {
            self.add_text_item(s, del, kaizen::PREP, None, f::TEXT, text);
        }
        ui.add_space(4.0);
        ui.strong(t.agenda);
        let slots: Vec<(Uid, i64, String)> =
            utsim_lss_core::tools::items(s.state(), del, kaizen::SLOT)
                .iter()
                .map(|i| (i.id, i.entity.integer(f::DAY).unwrap_or(1), i.text(f::TEXT)))
                .collect();
        for day in 1..=i64::from(ev.days) {
            let today: Vec<&(Uid, i64, String)> = slots.iter().filter(|x| x.1 == day).collect();
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new((t.day)(day)).strong());
                if today.is_empty() {
                    ui.weak("-");
                }
                for (id, _, text) in today {
                    self.check_field(ui, s, item_ref(*id), f::DONE, text);
                }
            });
        }
        let beyond = slots.iter().filter(|x| x.1 > i64::from(ev.days)).count();
        if beyond > 0 {
            ui.colored_label(theme::WARN, t.slots_beyond);
        }
        ui.add_space(4.0);
        ui.strong(t.open_items);
        let opens = kaizen::open_items(s.state(), del);
        let mut draw_extra = |app: &mut App, ui: &mut egui::Ui, s: &mut Session, id: Uid| {
            app.date_field(ui, s, item_ref(id), f::DUE);
            let Some(o) = opens.iter().find(|o| o.id == id) else {
                return;
            };
            if o.done {
                ui.label(RichText::new(l12().done).color(Color32::from_rgb(70, 160, 95)));
            } else if o.card.is_some() {
                ui.weak(l12().on_board);
            } else if ui.small_button(l12().to_board).clicked() {
                let card = s.new_id();
                let e = s
                    .state()
                    .entity(id)
                    .map(|entity| {
                        kaizen::card_edits(s.state(), s.project(), &ItemRef { id, entity }, card)
                    })
                    .unwrap_or_default();
                let r = s.apply(e);
                app.report(r);
            }
        };
        self.item_table(
            ui,
            s,
            del,
            kaizen::OPEN_ITEM,
            &[
                Col::Text(t.open_item, f::TEXT, 280.0),
                Col::Text(t.who, f::WHO, 120.0),
            ],
            Some((t.due_and_board, &mut draw_extra)),
        );
        ui.add_space(4.0);
        ui.strong(t.follow_ups);
        let fu = kaizen::follow_ups(s.state(), del, s.today());
        for x in &fu {
            ui.horizontal(|ui| {
                ui.strong((t.after_days)(x.days));
                ui.label(x.date.map_or("-".into(), |d| d.format_br()));
                if x.late > 0 {
                    ui.colored_label(theme::WARN, (t.late)(x.late));
                } else if x.due && !x.noted {
                    ui.colored_label(theme::WARN, t.review_due);
                } else if x.noted {
                    ui.label(t.reviewed);
                } else {
                    ui.weak(t.not_yet);
                }
            });
            self.text_field(ui, s, dr, &f::review(x.days), true, 560.0);
        }
    }

    // ---- PDCA --------------------------------------------------------------------------

    pub(super) fn pdca(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        use pdca::fields as f;
        let t = l12();
        let cs = pdca::cycles(s.state(), del);
        let (ok, no, open) = pdca::summary(&cs);
        ui.label((t.pdca_summary)(cs.len(), ok, no, open));
        let verdicts = vec![
            (Value::from(""), "-".to_string()),
            (Value::from("confirmada"), t.confirmed.to_string()),
            (Value::from("refutada"), t.refuted.to_string()),
        ];
        for c in &cs {
            let r = item_ref(c.id);
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new((t.cycle)(c.number))
                            .strong()
                            .color(theme::accent(ui)),
                    );
                    ui.label(step_name(c.step));
                    self.row_controls(ui, s, c.id);
                });
                if let Some(prev) = &c.previous_act {
                    ui.weak((t.starts_from)(prev));
                }
                let label = |ui: &mut egui::Ui, text: &str| {
                    let h = ui.spacing().interact_size.y;
                    ui.add_sized([90.0, h], egui::Label::new(text));
                };
                for (name, field) in [
                    (t.hypothesis, f::HYPOTHESIS),
                    (t.plan, f::PLAN),
                    (t.do_, f::DO),
                    (t.check, f::CHECK),
                ] {
                    ui.horizontal(|ui| {
                        label(ui, name);
                        self.text_field(ui, s, r, field, true, 560.0);
                    });
                }
                ui.horizontal(|ui| {
                    label(ui, t.verdict);
                    self.combo_field(ui, s, r, f::VERDICT, &verdicts, 140.0);
                });
                ui.horizontal(|ui| {
                    label(ui, t.act);
                    self.text_field(ui, s, r, f::ACT, true, 560.0);
                });
            });
            ui.add_space(4.0);
        }
        if ui.button(t.new_cycle).clicked() {
            let r = s.add_item(del, pdca::CYCLE, None, Vec::new());
            self.report(r);
        }
    }
}

fn ui_red() -> Color32 {
    Color32::from_rgb(200, 60, 60)
}

/// The radar of the last audit (filled) over the first one (outline).
fn radar(ui: &mut egui::Ui, audits: &[five_s::Audit]) {
    let size = 260.0;
    let (rect, _) = ui.allocate_exact_size(vec2(size + 120.0, size), egui::Sense::hover());
    let p = ui.painter_at(rect);
    let c = rect.center();
    let rad = size * 0.38;
    let n = Sense::ALL.len();
    let at = |i: usize, frac: f32| {
        let a = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::TAU / n as f32;
        c + vec2(a.cos(), a.sin()) * rad * frac
    };
    let grid = ui.visuals().weak_text_color();
    for k in 1..=4 {
        let ring: Vec<_> = (0..n).map(|i| at(i, k as f32 / 4.0)).collect();
        p.add(egui::Shape::closed_line(ring, Stroke::new(0.5, grid)));
    }
    for (i, s) in Sense::ALL.iter().enumerate() {
        p.line_segment([c, at(i, 1.0)], Stroke::new(0.5, grid));
        let pos = at(i, 1.18);
        p.text(
            pos,
            Align2::CENTER_CENTER,
            s.name(),
            FontId::proportional(12.0),
            ui.visuals().text_color(),
        );
    }
    let shape = |a: &five_s::Audit| -> Vec<egui::Pos2> {
        a.score
            .senses
            .iter()
            .enumerate()
            .map(|(i, s)| at(i, (s.pct.unwrap_or(0.0) / 100.0) as f32))
            .collect()
    };
    let accent = theme::accent(ui);
    if audits.len() > 1 {
        p.add(egui::Shape::closed_line(
            shape(&audits[0]),
            Stroke::new(1.5, Color32::from_gray(150)),
        ));
    }
    if let Some(last) = audits.last() {
        let pts = shape(last);
        p.add(egui::Shape::convex_polygon(
            pts.clone(),
            accent.gamma_multiply(0.25),
            Stroke::NONE,
        ));
        p.add(egui::Shape::closed_line(pts, Stroke::new(2.0, accent)));
    }
}

/// The combination table of one operator: manual time (solid), walking (thin) and machine
/// time (light) of each element from its start in the cycle, with the takt as a red line.
fn combination(
    ui: &mut egui::Ui,
    o: &lean::trabalho_padrao::Operator,
    takt: Option<f64>,
    span: f64,
) {
    let t = l12();
    let label_w = 180.0;
    let row_h = 18.0;
    let width = ui.available_width().clamp(520.0, 1100.0);
    let height = row_h * (o.steps.len() as f32 + 1.0) + 8.0;
    let (rect, _) = ui.allocate_exact_size(vec2(width, height), egui::Sense::hover());
    let p = ui.painter_at(rect);
    let scale = (width - label_w - 8.0) as f64 / span;
    let x = |v: f64| rect.left() + label_w + (v * scale) as f32;
    let text = ui.visuals().text_color();
    let accent = theme::accent(ui);
    p.text(
        pos2(rect.left(), rect.top() + row_h / 2.0),
        Align2::LEFT_CENTER,
        format!("{} · {}", or_untitled(&o.name), secs(o.content)),
        FontId::proportional(13.0),
        text,
    );
    for (i, st) in o.steps.iter().enumerate() {
        let y = rect.top() + row_h * (i as f32 + 1.0) + row_h / 2.0;
        p.text(
            pos2(rect.left() + 8.0, y),
            Align2::LEFT_CENTER,
            or_untitled(&st.name),
            FontId::proportional(12.0),
            text,
        );
        let m0 = st.start;
        let m1 = m0 + st.manual;
        if st.manual > 0.0 {
            p.rect_filled(
                egui::Rect::from_min_max(pos2(x(m0), y - 5.0), pos2(x(m1), y + 5.0)),
                1.0,
                accent,
            );
        }
        if st.auto > 0.0 {
            let colour = if st.machine_late {
                theme::WARN
            } else {
                accent.gamma_multiply(0.35)
            };
            p.rect_filled(
                egui::Rect::from_min_max(pos2(x(m1), y - 3.0), pos2(x(m1 + st.auto), y + 3.0)),
                0.0,
                colour,
            );
        }
        if st.walk > 0.0 {
            p.line_segment(
                [pos2(x(m1), y + 7.0), pos2(x(m1 + st.walk), y + 7.0)],
                Stroke::new(1.5, Color32::from_gray(140)),
            );
        }
    }
    if let Some(tk) = takt {
        p.line_segment(
            [pos2(x(tk), rect.top()), pos2(x(tk), rect.bottom())],
            Stroke::new(1.5, ui_red()),
        );
        p.text(
            pos2(x(tk) + 3.0, rect.top()),
            Align2::LEFT_TOP,
            t.takt,
            FontId::proportional(11.0),
            ui_red(),
        );
    }
}
