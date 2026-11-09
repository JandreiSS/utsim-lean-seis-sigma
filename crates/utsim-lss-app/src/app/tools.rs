//! The "Ferramentas" screen: the deliverables of the phases and the editor of each tool
//! (etapa L2). Drawing only: the numbers come from `utsim_lss_core::tools` and every change
//! goes through [`Session`], one field per edit, as on the action board.

use std::hash::Hash;

use eframe::egui::{self, Align2, Color32, FontId, RichText, Sense, Stroke, pos2, vec2};
use egui_plot::{Bar, BarChart, HLine, Line, Plot, PlotPoints};
use serde_json::Value;
use utsim_lss_core::domain;
use utsim_lss_core::tools::ap::Ap;
use utsim_lss_core::tools::{
    self, Deliverable, DeliverableStatus, Tool, a3, cause_effect as ce, fmea, ishikawa, item_ref,
    pareto, plans, sipoc, voc, vsm, whys,
};
use utsim_lss_core::{EntityRef, Id as Uid};

use super::{App, Nav};
use crate::session::Session;
use crate::text::l10::l10;
use crate::text::{self, texts, tool_texts};
use crate::theme;

/// A column of a simple item table.
enum Col {
    Text(&'static str, &'static str, f32),
    Choice(
        &'static str,
        &'static str,
        &'static [(&'static str, &'static str)],
        f32,
    ),
    /// A number (decimal comma), committed when the focus leaves.
    Number(&'static str, &'static str, f32),
    /// A yes or no box.
    Check(&'static str, &'static str),
}

type Extra<'a> = &'a mut dyn FnMut(&mut App, &mut egui::Ui, &mut Session, Uid);

/// A number as typed in Brazil: `1,5`; integers without decimals.
pub fn fmt_num(x: f64) -> String {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        format!("{x}").replace('.', ",")
    }
}

fn pct(x: f64) -> String {
    format!("{:.1} %", x).replace('.', ",")
}

fn ap_color(ui: &egui::Ui, ap: Ap) -> Color32 {
    match ap {
        Ap::High => ui.visuals().error_fg_color,
        Ap::Medium => theme::WARN,
        Ap::Low => Color32::from_rgb(70, 160, 95),
    }
}

fn rating_options() -> Vec<(Value, String)> {
    std::iter::once((Value::from(""), "-".to_string()))
        .chain((1..=10).map(|v| (Value::from(v), v.to_string())))
        .collect()
}

fn or_untitled(s: &str) -> String {
    if s.trim().is_empty() {
        texts().untitled.to_string()
    } else {
        s.to_string()
    }
}

impl App {
    pub(super) fn tools(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = texts();
        let tt = tool_texts();
        ui.heading(t.nav_tools);
        ui.weak(tt.intro);
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(tt.new_deliverable);
            egui::ComboBox::from_id_salt("nova_ferramenta")
                .selected_text(text::tool_name(self.new_tool))
                .show_ui(ui, |ui| {
                    for tool in Tool::ALL {
                        ui.selectable_value(&mut self.new_tool, tool, text::tool_name(tool));
                    }
                });
            if ui.button(t.create).clicked() {
                let r = s.add_deliverable(self.new_tool, text::tool_name(self.new_tool));
                if let Some(id) = self.report(r) {
                    self.selected_deliverable = Some(id);
                }
            }
        });
        ui.add_space(6.0);
        let list = tools::deliverables(s.state(), s.project());
        let phases = domain::phases(s.state(), s.project());
        if list.is_empty() {
            ui.label(tt.no_deliverables);
        }
        let mut groups: Vec<(String, Vec<&Deliverable>)> = phases
            .iter()
            .map(|ph| {
                let ds = list.iter().filter(|d| d.phase == Some(ph.id)).collect();
                (ph.name.clone(), ds)
            })
            .collect();
        let loose: Vec<&Deliverable> = list
            .iter()
            .filter(|d| !d.phase.is_some_and(|p| phases.iter().any(|ph| ph.id == p)))
            .collect();
        groups.push((tt.no_phase.to_string(), loose));
        for (name, ds) in groups {
            if ds.is_empty() {
                continue;
            }
            ui.horizontal_wrapped(|ui| {
                ui.strong(format!("{name}:"));
                for d in ds {
                    let mut label = or_untitled(&d.name);
                    if d.status == DeliverableStatus::Done {
                        label = format!("{label} ✔");
                    }
                    let selected = self.selected_deliverable == Some(d.id);
                    if ui.selectable_label(selected, label).clicked() {
                        self.selected_deliverable = Some(d.id);
                    }
                }
            });
        }
        ui.separator();
        let Some(id) = self.selected_deliverable else {
            ui.weak(tt.pick_deliverable);
            return;
        };
        let Some(d) = list.iter().find(|d| d.id == id).cloned() else {
            self.selected_deliverable = None;
            return;
        };
        self.deliverable_header(ui, s, &d);
        ui.add_space(6.0);
        match d.tool {
            Some(Tool::Voc) => self.voc(ui, s, d.id),
            Some(Tool::Sipoc) => self.sipoc(ui, s, d.id),
            Some(Tool::Pareto) => self.pareto_tool(ui, s, d.id),
            Some(Tool::DataCollection) => self.collection_plan(ui, s, d.id),
            Some(Tool::Ishikawa) => self.ishikawa(ui, s, d.id),
            Some(Tool::FiveWhys) => self.five_whys(ui, s, d.id),
            Some(Tool::CauseEffect) => self.cause_effect(ui, s, d.id),
            Some(Tool::Fmea) => self.fmea(ui, s, d.id),
            Some(Tool::ControlPlan) => self.control_plan(ui, s, d.id),
            Some(Tool::ResponsePlan) => self.response_plan(ui, s, d.id),
            Some(Tool::A3) => self.a3(ui, s, d.id),
            Some(Tool::Vsm) => self.vsm(ui, s, d.id),
            None => {
                ui.label(tt.unknown_tool);
            }
        }
    }

    fn deliverable_header(&mut self, ui: &mut egui::Ui, s: &mut Session, d: &Deliverable) {
        let t = texts();
        let tt = tool_texts();
        let r = Deliverable::entity_ref(d.id);
        use tools::fields::deliverable as f;
        ui.horizontal(|ui| {
            if let Some(tool) = d.tool {
                ui.label(
                    RichText::new(text::tool_name(tool))
                        .strong()
                        .color(theme::accent(ui)),
                );
            }
            self.text_field(ui, s, r, f::NAME, false, 280.0);
            ui.label(tt.phase);
            let phases: Vec<(Value, String)> = domain::phases(s.state(), s.project())
                .into_iter()
                .map(|p| (Value::from(p.id.to_string()), p.name))
                .collect();
            self.combo_field(ui, s, r, f::PHASE, &phases, 120.0);
            ui.label(tt.status);
            let statuses: Vec<(Value, String)> =
                [DeliverableStatus::Draft, DeliverableStatus::Done]
                    .into_iter()
                    .map(|st| {
                        (
                            domain::enum_value(st),
                            text::deliverable_status_name(st).to_string(),
                        )
                    })
                    .collect();
            self.combo_field(ui, s, r, f::STATUS, &statuses, 120.0);
            if ui
                .button(RichText::new(t.delete).color(ui.visuals().error_fg_color))
                .clicked()
            {
                let res = s.delete(r);
                self.report(res);
                self.selected_deliverable = None;
            }
        });
        self.presence_warning(ui, s, r);
        if let Some(tool) = d.tool {
            ui.weak(text::tool_help(tool));
        }
    }

    // ---- Helpers -----------------------------------------------------------------------

    /// A choice bound to a field: writes as soon as another option is picked.
    pub(super) fn combo_field(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        entity: EntityRef,
        field: &str,
        options: &[(Value, String)],
        width: f32,
    ) {
        let current = s
            .state()
            .entity(entity.id)
            .and_then(|e| e.get(field))
            .cloned()
            .unwrap_or_else(|| Value::from(""));
        let shown = options
            .iter()
            .find(|(v, _)| *v == current)
            .map_or_else(|| "-".to_string(), |(_, l)| l.clone());
        let mut chosen = None;
        egui::ComboBox::from_id_salt((entity.id, field))
            .width(width)
            .selected_text(shown)
            .show_ui(ui, |ui| {
                for (v, l) in options {
                    if ui.selectable_label(*v == current, l).clicked() {
                        chosen = Some(v.clone());
                    }
                }
            });
        if let Some(v) = chosen.filter(|v| *v != current) {
            let res = s.set_field(entity, field, v, None);
            self.report(res);
        }
    }

    fn rating_field(&mut self, ui: &mut egui::Ui, s: &mut Session, entity: EntityRef, field: &str) {
        self.combo_field(ui, s, entity, field, &rating_options(), 36.0);
    }

    /// A number box: `1,5` or `1.5`; empty clears the field.
    fn number_field(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        entity: EntityRef,
        field: &str,
        width: f32,
    ) {
        let stored = s.state().entity(entity.id).and_then(|e| e.number(field));
        let shown = stored.map(fmt_num).unwrap_or_default();
        let key = (entity.id, field.to_string());
        let (mut buf, seen) = match self.drafts.get(&key) {
            Some(d) => (d.text.clone(), d.seen),
            None => (
                shown.clone(),
                s.state().entity(entity.id).and_then(|e| e.version(field)),
            ),
        };
        let r = ui.add(egui::TextEdit::singleline(&mut buf).desired_width(width));
        if r.has_focus() {
            self.drafts.insert(
                key.clone(),
                super::Draft {
                    text: buf.clone(),
                    seen,
                },
            );
            self.editing = Some(entity);
        }
        if r.lost_focus() {
            self.drafts.remove(&key);
            if buf.trim() != shown {
                let value = if buf.trim().is_empty() {
                    Some(Value::from(""))
                } else {
                    tools::parse_number(&buf).map(Value::from)
                };
                match value {
                    Some(v) => {
                        let res = s.set_field(entity, field, v, Some(seen));
                        self.report(res);
                    }
                    None => self.set_status(tool_texts().bad_number, true),
                }
            }
        }
    }

    fn check_field(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        entity: EntityRef,
        field: &str,
        label: &str,
    ) {
        let mut v = s
            .state()
            .entity(entity.id)
            .is_some_and(|e| e.boolean(field));
        if ui.checkbox(&mut v, label).changed() {
            let res = s.set_field(entity, field, Value::Bool(v), None);
            self.report(res);
        }
    }

    /// Up, down and delete buttons of an item.
    fn row_controls(&mut self, ui: &mut egui::Ui, s: &mut Session, item: Uid) {
        let t = texts();
        let tt = tool_texts();
        if ui.small_button(t.move_up).on_hover_text(tt.up).clicked() {
            let r = s.nudge_item(item, -1);
            self.report(r);
        }
        if ui
            .small_button(t.move_down)
            .on_hover_text(tt.down)
            .clicked()
        {
            let r = s.nudge_item(item, 1);
            self.report(r);
        }
        if ui
            .small_button(RichText::new(tt.remove).color(ui.visuals().error_fg_color))
            .on_hover_text(t.delete)
            .clicked()
        {
            let r = s.delete_item(item);
            self.report(r);
        }
    }

    /// A box to type a new entry; returns the text when Enter or "Adicionar" is pressed.
    fn add_box(
        &mut self,
        ui: &mut egui::Ui,
        key: String,
        hint: &str,
        width: f32,
    ) -> Option<String> {
        let buf = self.inputs.entry(key).or_default();
        let mut out = None;
        ui.horizontal(|ui| {
            let r = ui.add(
                egui::TextEdit::singleline(buf)
                    .hint_text(hint)
                    .desired_width(width),
            );
            let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if (ui.button(texts().add).clicked() || enter) && !buf.trim().is_empty() {
                out = Some(std::mem::take(buf).trim().to_string());
            }
        });
        out
    }

    fn add_text_item(
        &mut self,
        s: &mut Session,
        del: Uid,
        kind: &str,
        parent: Option<Uid>,
        field: &str,
        text: String,
    ) {
        let r = s.add_item(del, kind, parent, vec![(field, Value::from(text))]);
        self.report(r);
    }

    /// A table of items with text and choice columns, an optional extra column, the row
    /// buttons and "Nova linha".
    fn item_table(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        del: Uid,
        kind: &str,
        cols: &[Col],
        extra: Option<(&str, Extra)>,
    ) {
        let tt = tool_texts();
        let rows: Vec<Uid> = tools::items(s.state(), del, kind)
            .iter()
            .map(|i| i.id)
            .collect();
        let mut extra = extra;
        egui::Grid::new(("tabela", del, kind))
            .striped(true)
            .spacing([6.0, 4.0])
            .show(ui, |ui| {
                for c in cols {
                    match c {
                        Col::Text(l, _, _)
                        | Col::Choice(l, _, _, _)
                        | Col::Number(l, _, _)
                        | Col::Check(l, _) => ui.strong(*l),
                    };
                }
                if let Some((l, _)) = &extra {
                    ui.strong(*l);
                }
                ui.end_row();
                for id in &rows {
                    let r = item_ref(*id);
                    for c in cols {
                        match c {
                            Col::Text(_, field, w) => self.text_field(ui, s, r, field, false, *w),
                            Col::Choice(_, field, opts, w) => {
                                let options: Vec<(Value, String)> =
                                    std::iter::once((Value::from(""), "-".to_string()))
                                        .chain(
                                            opts.iter()
                                                .map(|(k, l)| (Value::from(*k), l.to_string())),
                                        )
                                        .collect();
                                self.combo_field(ui, s, r, field, &options, *w);
                            }
                            Col::Number(_, field, w) => self.num_field(ui, s, r, field, *w),
                            Col::Check(_, field) => self.check_field(ui, s, r, field, ""),
                        }
                    }
                    if let Some((_, f)) = &mut extra {
                        f(self, ui, s, *id);
                    }
                    ui.horizontal(|ui| self.row_controls(ui, s, *id));
                    ui.end_row();
                }
            });
        if rows.is_empty() {
            ui.weak(tt.no_rows);
        }
        if ui.button(tt.new_row).clicked() {
            let r = s.add_item(del, kind, None, Vec::new());
            self.report(r);
        }
    }

    // ---- Value stream map -----------------------------------------------------------------

    fn vsm(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        use vsm::fields as f;
        let t = l10();
        let h = vsm::header(s.state(), del);
        let dr = Deliverable::entity_ref(del);
        ui.strong(if h.future {
            t.vsm_future
        } else {
            t.vsm_current
        });
        ui.horizontal_wrapped(|ui| {
            ui.label(t.vsm_demand);
            self.num_field(ui, s, dr, f::DEMAND, 70.0);
            ui.label(t.vsm_available);
            self.num_field(ui, s, dr, f::AVAILABLE, 70.0);
            ui.label(t.vsm_unit);
            self.text_field(ui, s, dr, f::TIME_UNIT, false, 50.0);
        });
        let cols = [
            Col::Text(t.vsm_process, f::NAME, 130.0),
            Col::Number(t.vsm_cycle, f::CYCLE, 60.0),
            Col::Number(t.vsm_setup, f::SETUP, 50.0),
            Col::Number(t.vsm_uptime, f::UPTIME, 60.0),
            Col::Number(t.vsm_operators, f::OPERATORS, 45.0),
            Col::Number(t.vsm_inventory, f::INVENTORY, 70.0),
            Col::Check(t.vsm_va, f::VALUE_ADDED),
        ];
        self.item_table(ui, s, del, vsm::PROCESS_ITEM, &cols, None);
        let procs = vsm::processes(s.state(), del);
        let l = vsm::ladder(&procs, &h);
        ui.add_space(6.0);
        ui.strong(t.vsm_ladder);
        let u = if h.unit.trim().is_empty() {
            String::new()
        } else {
            format!(" {}", h.unit.trim())
        };
        let n = |x: f64| format!("{}{u}", crate::app::screens::show_number(x));
        if l.takt.is_none() {
            ui.weak(t.vsm_need_demand);
        }
        draw_ladder(ui, &l);
        egui::Grid::new(("vsm_totais", del)).show(ui, |ui| {
            let line = |ui: &mut egui::Ui, k: &str, v: String| {
                ui.label(k);
                ui.strong(v);
                ui.end_row();
            };
            if let Some(x) = l.takt {
                line(ui, t.vsm_takt, n(x));
            }
            if let Some(x) = l.lead_time {
                line(ui, t.vsm_lead, n(x));
            }
            line(ui, t.vsm_va_time, n(l.value_added));
            if let Some(x) = l.pce {
                line(ui, t.vsm_pce, crate::app::screens::percent(x));
            }
            if let Some(x) = l.inventory_days {
                line(ui, t.vsm_inv_days, crate::app::screens::show_number(x));
            }
        });
        if !l.over_takt.is_empty() {
            ui.colored_label(theme::WARN, (t.vsm_over_takt)(&l.over_takt.join(", ")));
        }
        if let Some(sl) = &l.slowest {
            ui.label((t.vsm_slowest)(sl));
        }
        if let Some(from) = h.from {
            let before = vsm::ladder(
                &vsm::processes(s.state(), from),
                &vsm::header(s.state(), from),
            );
            let name = tools::deliverables(s.state(), s.project())
                .into_iter()
                .find(|d| d.id == from)
                .map(|d| d.name)
                .unwrap_or_default();
            ui.label((t.vsm_compare)(&name));
            if let (Some(a), Some(b)) = (before.lead_time, l.lead_time) {
                ui.label(format!("{}: {} → {}", t.vsm_lead, n(a), n(b)));
            }
            if let (Some(a), Some(b)) = (before.pce, l.pce) {
                ui.label(format!(
                    "{}: {} → {}",
                    t.vsm_pce,
                    crate::app::screens::percent(a),
                    crate::app::screens::percent(b)
                ));
            }
        } else if ui.button(t.vsm_make_future).clicked() {
            self.make_future_vsm(s, del);
        }
    }

    /// Creates the future state map from the current map `from` (the button); its id.
    pub fn create_future_vsm(&mut self, from: Uid) -> Option<Uid> {
        let mut s = self.session.take()?;
        let r = self.make_future_vsm(&mut s, from);
        self.session = Some(s);
        r
    }

    /// Creates the future state map from a current one (Melhorar), as the button does.
    pub(in crate::app) fn make_future_vsm(&mut self, s: &mut Session, from: Uid) -> Option<Uid> {
        let ph = utsim_lss_core::domain::phases(s.state(), s.project());
        let phase = ph.get(3.min(ph.len().saturating_sub(1))).map(|p| p.id);
        let id = s.new_id();
        let name = l10().vsm_future;
        let mut e = Deliverable::create(s.project(), phase, Tool::Vsm, id, name);
        {
            let ids = s.ids();
            e.extend(vsm::copy_edits(s.state(), s.project(), from, id, ids));
        }
        let r = s.apply(e);
        self.report(r)?;
        self.selected_deliverable = Some(id);
        self.set_status(l10().vsm_future_made, false);
        Some(id)
    }

    // ---- VOC and CTQ tree --------------------------------------------------------------

    fn voc(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        let tt = tool_texts();
        use voc::fields as f;
        let tree = voc::tree(s.state(), del);
        ui.strong(tt.voc_statements);
        let statements: Vec<Uid> = tools::items(s.state(), del, voc::STATEMENT)
            .iter()
            .map(|i| i.id)
            .collect();
        let needs: Vec<(Value, String)> = std::iter::once((Value::from(""), tt.ungrouped.into()))
            .chain(
                tree.needs
                    .iter()
                    .map(|n| (Value::from(n.id.to_string()), or_untitled(&n.text))),
            )
            .collect();
        egui::Grid::new(("falas", del))
            .striped(true)
            .show(ui, |ui| {
                for h in [tt.voc_statement, tt.source, tt.segment, tt.need, ""] {
                    ui.strong(h);
                }
                ui.end_row();
                for id in &statements {
                    let r = item_ref(*id);
                    self.text_field(ui, s, r, f::TEXT, false, 320.0);
                    self.text_field(ui, s, r, f::SOURCE, false, 110.0);
                    self.text_field(ui, s, r, f::SEGMENT, false, 110.0);
                    self.combo_field(ui, s, r, tools::fields::item::PARENT, &needs, 160.0);
                    ui.horizontal(|ui| self.row_controls(ui, s, *id));
                    ui.end_row();
                }
            });
        if let Some(text) = self.add_box(ui, format!("fala_{del}"), tt.voc_statement_hint, 320.0) {
            self.add_text_item(s, del, voc::STATEMENT, None, f::TEXT, text);
        }
        ui.add_space(8.0);
        ui.strong(tt.ctq_tree);
        let kano: Vec<(Value, String)> = std::iter::once((Value::from(""), tt.kano_none.into()))
            .chain(
                voc::Kano::ALL
                    .iter()
                    .map(|k| (domain::enum_value(*k), text::kano_name(*k).to_string())),
            )
            .collect();
        for need in &tree.needs {
            theme::card(ui).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.strong(tt.need);
                    self.text_field(ui, s, item_ref(need.id), f::TEXT, false, 300.0);
                    self.combo_field(ui, s, item_ref(need.id), f::KANO, &kano, 120.0);
                    self.row_controls(ui, s, need.id);
                    ui.weak((tt.statements_n)(need.statements.len()));
                });
                for drv in &need.drivers {
                    ui.indent(("direcionador", drv.id), |ui| {
                        ui.horizontal(|ui| {
                            ui.strong(tt.driver);
                            self.text_field(ui, s, item_ref(drv.id), f::TEXT, false, 280.0);
                            self.row_controls(ui, s, drv.id);
                        });
                        ui.indent(("ctqs", drv.id), |ui| {
                            egui::Grid::new(("ctq", drv.id))
                                .striped(true)
                                .show(ui, |ui| {
                                    for h in [
                                        tt.ctq,
                                        tt.definition,
                                        tt.unit,
                                        tt.lsl,
                                        tt.usl,
                                        tt.target,
                                        "",
                                        "",
                                    ] {
                                        ui.strong(h);
                                    }
                                    ui.end_row();
                                    for c in &drv.ctqs {
                                        let r = item_ref(c.id);
                                        self.text_field(ui, s, r, f::TEXT, false, 160.0);
                                        self.text_field(ui, s, r, f::DEFINITION, false, 200.0);
                                        self.text_field(ui, s, r, f::UNIT, false, 60.0);
                                        self.number_field(ui, s, r, f::LSL, 60.0);
                                        self.number_field(ui, s, r, f::USL, 60.0);
                                        self.number_field(ui, s, r, f::TARGET, 60.0);
                                        ui.horizontal(|ui| self.row_controls(ui, s, c.id));
                                        let issues: Vec<&str> = c
                                            .issues()
                                            .iter()
                                            .map(|i| text::voc_issue(*i))
                                            .collect();
                                        ui.colored_label(theme::WARN, issues.join("; "));
                                        ui.end_row();
                                    }
                                });
                            if let Some(t) =
                                self.add_box(ui, format!("ctq_{}", drv.id), tt.ctq_hint, 240.0)
                            {
                                self.add_text_item(s, del, voc::CTQ, Some(drv.id), f::TEXT, t);
                            }
                        });
                    });
                }
                ui.indent(("novo_direcionador", need.id), |ui| {
                    if let Some(t) =
                        self.add_box(ui, format!("drv_{}", need.id), tt.driver_hint, 260.0)
                    {
                        self.add_text_item(s, del, voc::DRIVER, Some(need.id), f::TEXT, t);
                    }
                });
            });
        }
        if let Some(t) = self.add_box(ui, format!("need_{del}"), tt.need_hint, 300.0) {
            self.add_text_item(s, del, voc::NEED, None, f::TEXT, t);
        }
        if !tree.orphans.is_empty() {
            ui.colored_label(theme::WARN, (tt.orphans)(tree.orphans.len()));
        }
        let n = tree.issues().len();
        if n == 0 && tree.ctqs().count() > 0 {
            ui.label(tt.tree_ok);
        } else if n > 0 {
            ui.colored_label(theme::WARN, (tt.tree_issues)(n));
        }
    }

    // ---- SIPOC -------------------------------------------------------------------------

    fn sipoc(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        let tt = tool_texts();
        let sp = sipoc::of_deliverable(s.state(), del);
        ui.horizontal_top(|ui| {
            for (k, (kind, title)) in sipoc::COLUMNS.iter().enumerate() {
                ui.vertical(|ui| {
                    ui.set_width(230.0);
                    ui.strong(*title);
                    for (n, (id, _)) in sp.columns[k].iter().enumerate() {
                        ui.horizontal(|ui| {
                            if k == 2 {
                                ui.label(format!("{}.", n + 1));
                            }
                            self.text_field(ui, s, item_ref(*id), sipoc::TEXT, false, 130.0);
                            self.row_controls(ui, s, *id);
                        });
                    }
                    if let Some(t) =
                        self.add_box(ui, format!("sipoc_{kind}_{del}"), tt.add_hint, 130.0)
                    {
                        self.add_text_item(s, del, kind, None, sipoc::TEXT, t);
                    }
                });
            }
        });
        if !sp.steps_ok() {
            ui.colored_label(theme::WARN, (tt.sipoc_steps)(sp.steps()));
        }
        if !sp.empty_columns().is_empty() {
            ui.weak(tt.sipoc_empty);
        }
    }

    // ---- Ishikawa ----------------------------------------------------------------------

    fn ishikawa(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        let tt = tool_texts();
        use ishikawa::fields as f;
        let d = Deliverable::entity_ref(del);
        ui.horizontal(|ui| {
            ui.strong(tt.effect);
            self.text_field(ui, s, d, f::EFFECT, false, 420.0);
        });
        let g = ishikawa::diagram(s.state(), del);
        fishbone(ui, &g);
        if !g.cycles.is_empty() {
            ui.colored_label(theme::WARN, tt.cycle_broken);
        }
        let statuses: Vec<(Value, String)> = ishikawa::CauseStatus::ALL
            .iter()
            .map(|st| {
                (
                    domain::enum_value(*st),
                    text::cause_status_name(*st).to_string(),
                )
            })
            .collect();
        for bone in &g.bones {
            let title = format!(
                "{} ({})",
                ishikawa::category_name(&bone.category),
                bone.causes.len()
            );
            egui::CollapsingHeader::new(title)
                .id_salt(("osso", del, &bone.category))
                .default_open(!bone.causes.is_empty())
                .show(ui, |ui| {
                    for c in &bone.causes {
                        let r = item_ref(c.id);
                        ui.horizontal(|ui| {
                            ui.add_space(18.0 * c.depth as f32);
                            self.text_field(ui, s, r, f::TEXT, false, 260.0);
                            self.combo_field(ui, s, r, f::STATUS, &statuses, 100.0);
                            ui.label(tt.evidence);
                            self.text_field(ui, s, r, f::EVIDENCE, false, 200.0);
                            if ui.small_button(tt.add_sub).clicked() {
                                let res = s.add_item(del, ishikawa::ITEM, Some(c.id), Vec::new());
                                self.report(res);
                            }
                            self.row_controls(ui, s, c.id);
                        });
                    }
                    let key = format!("ish_{del}_{}", bone.category);
                    if let Some(t) = self.add_box(ui, key, tt.cause_hint, 260.0) {
                        let res = s.add_item(
                            del,
                            ishikawa::ITEM,
                            None,
                            vec![
                                (f::TEXT, Value::from(t)),
                                (f::CATEGORY, Value::from(bone.category.clone())),
                            ],
                        );
                        self.report(res);
                    }
                });
        }
        if let Some(cat) = self.add_box(ui, format!("ish_cat_{del}"), tt.new_category_hint, 200.0) {
            let res = s.add_item(
                del,
                ishikawa::ITEM,
                None,
                vec![(f::TEXT, Value::from("")), (f::CATEGORY, Value::from(cat))],
            );
            self.report(res);
        }
    }

    // ---- Five whys ---------------------------------------------------------------------

    fn five_whys(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        let tt = tool_texts();
        use whys::fields as f;
        let chains = whys::chains(s.state(), del);
        let causes = ishikawa::project_causes(s.state(), s.project());
        let cause_opts: Vec<(Value, String)> =
            std::iter::once((Value::from(""), tt.no_cause.into()))
                .chain(
                    causes
                        .iter()
                        .map(|(_, c)| (Value::from(c.id.to_string()), c.label())),
                )
                .collect();
        for ch in &chains {
            theme::card(ui).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.strong(tt.problem);
                    self.text_field(ui, s, item_ref(ch.id), f::TEXT, false, 360.0);
                    self.row_controls(ui, s, ch.id);
                });
                ui.horizontal(|ui| {
                    ui.label(tt.from_cause);
                    self.combo_field(ui, s, item_ref(ch.id), f::CAUSE, &cause_opts, 300.0);
                });
                for w in &ch.whys {
                    let r = item_ref(w.id);
                    ui.horizontal(|ui| {
                        ui.add_space(18.0 * w.level as f32);
                        ui.label((tt.why_n)(w.level));
                        self.text_field(ui, s, r, f::TEXT, false, 300.0);
                        ui.label(tt.evidence);
                        self.text_field(ui, s, r, f::EVIDENCE, false, 180.0);
                        self.check_field(ui, s, r, f::ROOT, tt.root_cause);
                        if ui.small_button(tt.next_why).clicked() {
                            let res = s.add_item(del, whys::WHY, Some(w.id), Vec::new());
                            self.report(res);
                        }
                        self.row_controls(ui, s, w.id);
                    });
                }
                if ui.small_button(tt.first_why).clicked() {
                    let res = s.add_item(del, whys::WHY, Some(ch.id), Vec::new());
                    self.report(res);
                }
                let roots: Vec<String> = ch.root_causes().iter().map(|w| w.text.clone()).collect();
                if !roots.is_empty() {
                    ui.label(RichText::new((tt.roots)(&roots.join("; "))).color(theme::accent(ui)));
                }
            });
        }
        if let Some(t) = self.add_box(ui, format!("cadeia_{del}"), tt.chain_hint, 360.0) {
            self.add_text_item(s, del, whys::CHAIN, None, f::TEXT, t);
        }
        if !causes.is_empty() {
            let mut pick: Option<ishikawa::Cause> = None;
            egui::ComboBox::from_id_salt(("cadeia_de_causa", del))
                .selected_text(tt.chain_from_cause)
                .width(320.0)
                .show_ui(ui, |ui| {
                    for (_, c) in &causes {
                        if ui.selectable_label(false, c.label()).clicked() {
                            pick = Some(c.clone());
                        }
                    }
                });
            if let Some(c) = pick {
                let res = s.add_item(
                    del,
                    whys::CHAIN,
                    None,
                    vec![
                        (f::TEXT, Value::from(c.text.clone())),
                        (f::CAUSE, Value::from(c.id.to_string())),
                    ],
                );
                self.report(res);
            }
        }
    }

    // ---- C&E matrix --------------------------------------------------------------------

    fn cause_effect(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        let tt = tool_texts();
        use ce::fields as f;
        let m = ce::of_deliverable(s.state(), del);
        ui.strong(tt.ce_outputs);
        let weights: Vec<(Value, String)> =
            (1..=10).map(|v| (Value::from(v), v.to_string())).collect();
        egui::Grid::new(("saidas", del))
            .striped(true)
            .show(ui, |ui| {
                for h in [tt.output, tt.weight, ""] {
                    ui.strong(h);
                }
                ui.end_row();
                for o in &m.outputs {
                    let r = item_ref(o.id);
                    self.text_field(ui, s, r, f::NAME, false, 240.0);
                    self.combo_field(ui, s, r, f::WEIGHT, &weights, 40.0);
                    ui.horizontal(|ui| self.row_controls(ui, s, o.id));
                    ui.end_row();
                }
            });
        if let Some(t) = self.add_box(ui, format!("saida_{del}"), tt.output_hint, 240.0) {
            let res = s.add_item(
                del,
                ce::OUTPUT,
                None,
                vec![(f::NAME, Value::from(t)), (f::WEIGHT, Value::from(5))],
            );
            self.report(res);
        }
        ui.add_space(8.0);
        ui.strong(tt.ce_matrix);
        let scores: Vec<(Value, String)> = std::iter::once((Value::from(""), "-".to_string()))
            .chain(ce::SCORES.iter().map(|v| (Value::from(*v), v.to_string())))
            .collect();
        egui::Grid::new(("matriz", del))
            .striped(true)
            .show(ui, |ui| {
                ui.strong(tt.input);
                for o in &m.outputs {
                    ui.strong(format!("{} ({})", or_untitled(&o.name), o.weight));
                }
                ui.strong(tt.total);
                ui.strong(tt.rank);
                ui.end_row();
                for i in &m.inputs {
                    let r = item_ref(i.id);
                    self.text_field(ui, s, r, f::NAME, false, 200.0);
                    for o in &m.outputs {
                        self.combo_field(ui, s, r, &ce::score_field(o.id), &scores, 40.0);
                    }
                    ui.strong(fmt_num(i.total));
                    ui.label(format!("{}º", i.rank));
                    ui.horizontal(|ui| self.row_controls(ui, s, i.id));
                    ui.end_row();
                }
                if !m.inputs.is_empty() {
                    ui.weak(tt.column_sum);
                    for v in m.column_sums() {
                        ui.weak(v.to_string());
                    }
                    ui.end_row();
                }
            });
        if let Some(t) = self.add_box(ui, format!("entrada_{del}"), tt.input_hint, 200.0) {
            let res = s.add_item(del, ce::INPUT, None, vec![(f::NAME, Value::from(t))]);
            self.report(res);
        }
        ui.add_space(8.0);
        ui.strong(tt.ce_pareto);
        pareto_chart(ui, ("pareto_ce", del), &m.pareto);
    }

    // ---- Pareto ------------------------------------------------------------------------

    fn pareto_tool(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        let tt = tool_texts();
        use pareto::fields as f;
        let d = Deliverable::entity_ref(del);
        let (table_mode, by_cost) = s.state().entity(del).map_or((false, false), |e| {
            (e.text(f::MODE) == "tabela", e.boolean(f::BY_COST))
        });
        ui.horizontal(|ui| {
            let mut mode = table_mode;
            ui.radio_value(&mut mode, false, tt.pareto_counts);
            ui.radio_value(&mut mode, true, tt.pareto_table);
            if mode != table_mode {
                let v = if mode { "tabela" } else { "contagens" };
                let res = s.set_field(d, f::MODE, Value::from(v), None);
                self.report(res);
            }
            ui.separator();
            ui.label(tt.others_below);
            self.number_field(ui, s, d, f::OTHERS_BELOW, 50.0);
            ui.label("%");
            if !table_mode {
                ui.separator();
                self.check_field(ui, s, d, f::BY_COST, tt.by_cost);
            }
        });
        if table_mode {
            ui.weak(tt.pareto_table_tip);
            self.text_field(ui, s, d, f::TABLE, true, 420.0);
        } else {
            let rows: Vec<Uid> = tools::items(s.state(), del, pareto::ITEM)
                .iter()
                .map(|i| i.id)
                .collect();
            egui::Grid::new(("categorias", del))
                .striped(true)
                .show(ui, |ui| {
                    ui.strong(tt.category);
                    ui.strong(tt.count);
                    if by_cost {
                        ui.strong(tt.unit_cost);
                    }
                    ui.strong("");
                    ui.end_row();
                    for id in &rows {
                        let r = item_ref(*id);
                        self.text_field(ui, s, r, f::NAME, false, 220.0);
                        self.number_field(ui, s, r, f::COUNT, 70.0);
                        if by_cost {
                            self.number_field(ui, s, r, f::COST, 70.0);
                        }
                        ui.horizontal(|ui| self.row_controls(ui, s, *id));
                        ui.end_row();
                    }
                });
            if let Some(t) = self.add_box(ui, format!("categoria_{del}"), tt.category_hint, 220.0) {
                self.add_text_item(s, del, pareto::ITEM, None, f::NAME, t);
            }
        }
        ui.add_space(8.0);
        let p = pareto::of_deliverable(s.state(), del);
        pareto_table(ui, ("resultado", del), &p);
        pareto_chart(ui, ("grafico", del), &p);
    }

    // ---- Plans -------------------------------------------------------------------------

    fn collection_plan(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        let tt = tool_texts();
        use plans::fields as f;
        let cols = [
            Col::Choice(tt.c_role, f::ROLE, &[("y", "Y"), ("x", "X")], 40.0),
            Col::Text(tt.c_name, f::NAME, 120.0),
            Col::Text(tt.definition, f::DEFINITION, 180.0),
            Col::Choice(tt.c_data_type, f::DATA_TYPE, &text::DATA_TYPES, 90.0),
            Col::Text(tt.unit, f::UNIT, 50.0),
            Col::Text(tt.c_stratify, f::STRATIFY, 110.0),
            Col::Text(tt.c_sample, f::SAMPLE_SIZE, 60.0),
            Col::Text(tt.c_frequency, f::FREQUENCY, 80.0),
            Col::Text(tt.c_subgroup, f::SUBGROUP, 60.0),
            Col::Text(tt.c_who, f::WHO, 90.0),
            Col::Text(tt.c_where, f::WHERE, 90.0),
            Col::Text(tt.c_form, f::FORM, 110.0),
        ];
        let mut issues = |_: &mut App, ui: &mut egui::Ui, s: &mut Session, id: Uid| {
            let i = s
                .state()
                .entity(id)
                .map(|e| tools::ItemRef { id, entity: e });
            let list: Vec<&str> = i
                .map(|i| plans::collection_issues(&i))
                .unwrap_or_default()
                .into_iter()
                .map(text::collection_issue)
                .collect();
            ui.colored_label(theme::WARN, list.join("; "));
        };
        self.item_table(
            ui,
            s,
            del,
            plans::COLLECTION_ITEM,
            &cols,
            Some((tt.c_missing, &mut issues)),
        );
    }

    fn control_plan(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        let tt = tool_texts();
        use plans::fields as f;
        let cols = [
            Col::Text(tt.c_process, f::PROCESS, 100.0),
            Col::Text(tt.c_characteristic, f::CHARACTERISTIC, 130.0),
            Col::Choice(
                tt.c_char_kind,
                f::CHAR_KIND,
                &[("produto", "Produto"), ("processo", "Processo")],
                80.0,
            ),
            Col::Text(tt.c_spec, f::SPEC, 100.0),
            Col::Text(tt.c_method, f::METHOD, 110.0),
            Col::Text(tt.c_sample, f::SAMPLE_SIZE, 50.0),
            Col::Text(tt.c_frequency, f::FREQUENCY, 70.0),
            Col::Text(tt.c_control, f::CONTROL, 110.0),
            Col::Text(tt.c_owner, f::OWNER, 90.0),
            Col::Text(tt.c_reaction, f::REACTION, 130.0),
        ];
        let rows: Vec<(Value, String)> = std::iter::once((Value::from(""), "-".to_string()))
            .chain(
                tools::of_tool(s.state(), s.project(), Tool::Fmea)
                    .iter()
                    .flat_map(|d| fmea::rows(s.state(), d.id))
                    .map(|r| {
                        let sev = r.current_s().map_or("-".into(), |v| v.to_string());
                        (
                            Value::from(r.id.to_string()),
                            format!("S{sev} · {}", or_untitled(&r.mode)),
                        )
                    }),
            )
            .collect();
        let mut link = |app: &mut App, ui: &mut egui::Ui, s: &mut Session, id: Uid| {
            ui.horizontal(|ui| {
                app.combo_field(ui, s, item_ref(id), f::FMEA_ROW, &rows, 150.0);
                app.check_field(ui, s, item_ref(id), f::CRITICAL, tool_texts().critical);
            });
        };
        self.item_table(
            ui,
            s,
            del,
            plans::CONTROL_ITEM,
            &cols,
            Some((tt.c_fmea_link, &mut link)),
        );
        ui.add_space(8.0);
        let gaps = fmea::critical_without_control(s.state(), s.project());
        if gaps.is_empty() {
            ui.label(tt.no_gaps);
        } else {
            ui.colored_label(theme::WARN, (tt.gaps)(gaps.len()));
            for g in gaps {
                ui.horizontal(|ui| {
                    let sev = g.row.current_s().unwrap_or(0);
                    ui.label(format!(
                        "S{sev} · {} · {}",
                        or_untitled(&g.row.mode),
                        g.row.effect
                    ));
                    if ui.small_button(tt.add_control_line).clicked() {
                        let res = s.add_item(
                            del,
                            plans::CONTROL_ITEM,
                            None,
                            vec![
                                (f::PROCESS, Value::from(g.row.step.clone())),
                                (f::CHARACTERISTIC, Value::from(g.row.mode.clone())),
                                (f::FMEA_ROW, Value::from(g.row.id.to_string())),
                                (f::CRITICAL, Value::Bool(true)),
                            ],
                        );
                        self.report(res);
                    }
                });
            }
        }
        self.control_monitor(ui, s, del);
    }

    fn response_plan(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        let tt = tool_texts();
        use plans::fields as f;
        let cols = [
            Col::Text(tt.c_signal, f::SIGNAL, 180.0),
            Col::Text(tt.c_immediate, f::IMMEDIATE, 180.0),
            Col::Text(tt.c_containment, f::CONTAINMENT, 160.0),
            Col::Text(tt.c_call, f::CALL, 120.0),
            Col::Text(tt.c_deadline, f::DEADLINE, 90.0),
        ];
        let mut card = |app: &mut App, ui: &mut egui::Ui, s: &mut Session, id: Uid| {
            app.card_cell(ui, s, id, plans::fields::CARD, true);
        };
        self.item_table(
            ui,
            s,
            del,
            plans::RESPONSE_ITEM,
            &cols,
            Some((tt.c_card, &mut card)),
        );
    }

    /// The card of an item (FMEA row or signal): its column and "Abrir", or a button that
    /// makes it. `signal`: a response plan signal (a new card each time it fires).
    fn card_cell(
        &mut self,
        ui: &mut egui::Ui,
        s: &mut Session,
        item: Uid,
        field: &str,
        signal: bool,
    ) {
        let tt = tool_texts();
        let card = s
            .state()
            .entity(item)
            .and_then(|e| e.id(field))
            .filter(|c| s.state().entity(*c).is_some_and(|e| !e.is_deleted()));
        ui.horizontal(|ui| {
            if let Some(c) = card {
                let col = s.card_column(c).unwrap_or_default();
                if ui
                    .small_button(format!("{} · {}", tt.open_card, text::column_name(col)))
                    .clicked()
                {
                    self.nav = Nav::Actions;
                    self.selected_card = Some(c);
                }
            }
            if signal {
                if ui.small_button(tt.fire_signal).clicked() {
                    let r = s.signal_card(item);
                    if let Some(Some(_)) = self.report(r) {
                        self.set_status(tt.card_made, false);
                    }
                }
            } else if card.is_none() && ui.small_button(tt.make_card).clicked() {
                let r = s.fmea_card(item);
                if let Some(Some(_)) = self.report(r) {
                    self.set_status(tt.card_made, false);
                }
            }
        });
    }

    // ---- FMEA --------------------------------------------------------------------------

    fn fmea(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        let tt = tool_texts();
        use fmea::fields as f;
        let (table, source) = fmea::project_table(s.state(), s.project());
        theme::card(ui).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.strong(tt.ap_table);
                match &source {
                    fmea::TableSource::BuiltIn => {
                        ui.label(tt.ap_builtin);
                    }
                    fmea::TableSource::Company(name) => {
                        ui.label((tt.ap_company)(name));
                        if ui.small_button(tt.ap_use_builtin).clicked() {
                            let r = s.use_builtin_ap_table();
                            self.report(r);
                        }
                    }
                }
                let [h, m, l] = table.counts();
                ui.weak((tt.ap_counts)(h, m, l));
            });
            let bad = table.non_monotone().len();
            if bad > 0 {
                ui.colored_label(theme::WARN, (tt.ap_non_monotone)(bad));
            }
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.ap_csv_path)
                        .hint_text(tt.ap_csv_hint)
                        .desired_width(300.0),
                );
                ui.add(
                    egui::TextEdit::singleline(&mut self.ap_table_name)
                        .hint_text(tt.ap_name_hint)
                        .desired_width(140.0),
                );
                if ui.button(tt.ap_import).clicked() {
                    self.import_ap(s);
                }
                if ui.button(tt.ap_export).clicked() {
                    let path = self.ap_csv_path.trim().to_string();
                    if path.is_empty() {
                        self.set_status(tt.ap_need_path, true);
                    } else {
                        match std::fs::write(&path, table.to_csv()) {
                            Ok(()) => self.set_status((tt.ap_exported)(&path), false),
                            Err(e) => self.set_status((texts().error)(&e.to_string()), true),
                        }
                    }
                }
            });
            ui.weak(tt.ap_tip);
            for e in &self.ap_errors {
                ui.colored_label(ui.visuals().error_fg_color, e);
            }
        });
        let d = Deliverable::entity_ref(del);
        egui::CollapsingHeader::new(tt.scales)
            .id_salt(("escalas", del))
            .show(ui, |ui| {
                ui.weak(tt.scales_tip);
                egui::Grid::new(("escalas_grade", del))
                    .striped(true)
                    .show(ui, |ui| {
                        for h in [tt.level, tt.scale_s, tt.scale_o, tt.scale_d] {
                            ui.strong(h);
                        }
                        ui.end_row();
                        for level in (1..=10u8).rev() {
                            ui.label(level.to_string());
                            for sc in fmea::Scale::ALL {
                                self.text_field(ui, s, d, &sc.field(level), false, 240.0);
                            }
                            ui.end_row();
                        }
                    });
            });
        let from = fmea::critical_from(s.state(), del);
        ui.horizontal(|ui| {
            ui.label(tt.critical_from);
            self.rating_field(ui, s, d, f::CRITICAL_FROM);
            ui.weak((tt.critical_default)(fmea::CRITICAL_FROM));
        });
        ui.add_space(6.0);
        let rows = fmea::rows(s.state(), del);
        egui::Grid::new(("fmea", del))
            .striped(true)
            .spacing([4.0, 4.0])
            .show(ui, |ui| {
                for h in tt.fmea_headers {
                    ui.strong(*h);
                }
                ui.end_row();
                for row in &rows {
                    let r = item_ref(row.id);
                    if row.is_critical(from) {
                        ui.colored_label(theme::WARN, "◆")
                            .on_hover_text(tt.critical);
                    } else {
                        ui.label("");
                    }
                    self.text_field(ui, s, r, f::STEP, false, 90.0);
                    self.text_field(ui, s, r, f::MODE, false, 120.0);
                    self.text_field(ui, s, r, f::EFFECT, false, 120.0);
                    self.rating_field(ui, s, r, f::S);
                    self.text_field(ui, s, r, f::CAUSE, false, 120.0);
                    self.rating_field(ui, s, r, f::O);
                    self.text_field(ui, s, r, f::PREVENTION, false, 110.0);
                    self.text_field(ui, s, r, f::DETECTION, false, 110.0);
                    self.rating_field(ui, s, r, f::D);
                    ui.strong(row.rpn().map_or("-".into(), |v| v.to_string()));
                    ap_label(ui, row.ap(&table));
                    self.text_field(ui, s, r, f::ACTION, false, 140.0);
                    self.card_cell(ui, s, row.id, f::CARD, false);
                    self.text_field(ui, s, r, f::ACTION_TAKEN, false, 120.0);
                    self.rating_field(ui, s, r, f::S2);
                    self.rating_field(ui, s, r, f::O2);
                    self.rating_field(ui, s, r, f::D2);
                    match row.revision(&table) {
                        Some(rev) => {
                            let txt = format!("{} → {}", rev.rpn_before, rev.rpn_after);
                            if rev.improved() {
                                ui.strong(txt);
                            } else {
                                ui.label(txt);
                            }
                            ap_label(ui, Some(rev.ap_after));
                        }
                        None => {
                            ui.label("-");
                            ui.label("-");
                        }
                    }
                    ui.horizontal(|ui| self.row_controls(ui, s, row.id));
                    ui.end_row();
                }
            });
        if rows.is_empty() {
            ui.weak(tt.no_rows);
        }
        if ui.button(tt.new_row).clicked() {
            let r = s.add_item(del, fmea::ITEM, None, Vec::new());
            self.report(r);
        }
        ui.add_space(6.0);
        let gaps = fmea::critical_without_control(s.state(), s.project())
            .into_iter()
            .filter(|g| g.fmea == del)
            .count();
        if gaps > 0 {
            ui.colored_label(theme::WARN, (tt.gaps)(gaps));
        }
    }

    fn import_ap(&mut self, s: &mut Session) {
        let tt = tool_texts();
        self.ap_errors.clear();
        let path = self.ap_csv_path.trim().to_string();
        let csv = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                self.set_status((texts().error)(&e.to_string()), true);
                return;
            }
        };
        let name = if self.ap_table_name.trim().is_empty() {
            std::path::Path::new(&path)
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default()
        } else {
            self.ap_table_name.trim().to_string()
        };
        let r = s.import_ap_table(&csv, &name);
        match self.report(r) {
            Some(Ok(())) => self.set_status(tt.ap_imported, false),
            Some(Err(errs)) => {
                self.ap_errors = errs.iter().map(|e| e.to_string()).collect();
                self.set_status(tt.ap_rejected, true);
            }
            None => {}
        }
    }

    // ---- A3 ----------------------------------------------------------------------------

    fn a3(&mut self, ui: &mut egui::Ui, s: &mut Session, del: Uid) {
        let tt = tool_texts();
        let page = a3::compose(s.state(), s.project(), Some(del));
        let people = |id: Option<Uid>| id.map(|p| domain::person_name(s.state(), p));
        let lines = |v: &[String]| v.to_vec();
        let mut content: Vec<Vec<String>> = vec![Vec::new(); a3::Section::ALL.len()];
        content[0] = [page.problem.clone(), page.business_case.clone()]
            .into_iter()
            .filter(|x| !x.trim().is_empty())
            .collect();
        content[1] = std::iter::once(page.baseline.clone())
            .filter(|x| !x.trim().is_empty())
            .chain(
                page.pareto_vital
                    .iter()
                    .map(|(n, p)| format!("{} · {n}: {}", tt.a3_pareto, pct(*p))),
            )
            .chain(page.ctqs.iter().map(|c| format!("CTQ · {c}")))
            .chain(
                (!page.process_steps.is_empty())
                    .then(|| format!("{} · {}", tt.a3_process, page.process_steps.join(" → "))),
            )
            .collect();
        content[2] = std::iter::once(page.goal.clone())
            .filter(|x| !x.trim().is_empty())
            .chain(
                page.deadline
                    .map(|d| format!("{}: {}", texts().deadline, d.format_br())),
            )
            .collect();
        content[3] = page
            .verified_causes
            .iter()
            .map(|c| format!("{} · {c}", tt.a3_verified))
            .chain(
                page.root_causes
                    .iter()
                    .map(|c| format!("{} · {c}", tt.a3_root)),
            )
            .chain(
                page.top_inputs
                    .iter()
                    .map(|(n, v)| format!("C&E · {}: {}", or_untitled(n), fmt_num(*v))),
            )
            .collect();
        content[4] = page
            .fmea_high
            .iter()
            .map(|(m, a)| format!("{} · {}: {}", tt.a3_fmea_high, or_untitled(m), a))
            .collect();
        content[5] = page
            .plan
            .iter()
            .map(|p| {
                let mut x = format!("[{}] {}", text::column_name(p.column), p.title);
                if let Some(d) = p.due {
                    x.push_str(&format!(" · {}", d.format_br()));
                }
                if let Some(w) = people(p.who) {
                    x.push_str(&format!(" · {w}"));
                }
                x
            })
            .collect();
        content[6] = lines(&page.control_lines)
            .into_iter()
            .map(|c| format!("{} · {c}", tt.a3_control))
            .chain(std::iter::once((tt.a3_signals)(page.signals)))
            .chain((page.critical_gaps > 0).then(|| (tt.gaps)(page.critical_gaps)))
            .collect();
        ui.heading(or_untitled(&page.title));
        let d = Deliverable::entity_ref(del);
        ui.horizontal_top(|ui| {
            for half in [0..4, 4..7] {
                ui.vertical(|ui| {
                    ui.set_width(470.0);
                    for k in half {
                        let sec = a3::Section::ALL[k];
                        theme::card(ui).show(ui, |ui| {
                            ui.set_width(450.0);
                            ui.strong(format!("{}. {}", k + 1, tt.a3_titles[k]));
                            if content[k].is_empty() {
                                ui.weak(tt.a3_empty);
                            }
                            for line in &content[k] {
                                ui.label(line);
                            }
                            ui.weak(tt.a3_note);
                            self.text_field(ui, s, d, sec.note_field(), true, 430.0);
                        });
                    }
                });
            }
        });
    }
}

fn ap_label(ui: &mut egui::Ui, ap: Option<Ap>) {
    match ap {
        Some(a) => {
            let c = ap_color(ui, a);
            ui.label(RichText::new(text::ap_name(a)).strong().color(c));
        }
        None => {
            ui.label("-");
        }
    }
}

/// The Pareto as a table: category, value, percentage, cumulative, vital few.
fn pareto_table(ui: &mut egui::Ui, id: impl Hash + std::fmt::Debug, p: &pareto::Pareto) {
    let tt = tool_texts();
    if p.rows.is_empty() {
        return;
    }
    egui::Grid::new(id).striped(true).show(ui, |ui| {
        for h in [tt.category, tt.value, tt.percentage, tt.cumulative, ""] {
            ui.strong(h);
        }
        ui.end_row();
        for r in &p.rows {
            ui.label(or_untitled(&r.name));
            ui.label(fmt_num((r.value * 100.0).round() / 100.0));
            ui.label(pct(r.pct));
            ui.label(pct(r.cum_pct));
            if r.vital {
                ui.label(RichText::new(tt.vital).color(theme::accent(ui)));
            } else {
                ui.label("");
            }
            ui.end_row();
        }
        ui.strong(tt.total);
        ui.strong(fmt_num((p.total * 100.0).round() / 100.0));
        ui.end_row();
    });
}

/// Bars of the percentage of each category, the cumulative line and the 80 % marker.
fn pareto_chart(ui: &mut egui::Ui, id: impl Hash + std::fmt::Debug, p: &pareto::Pareto) {
    let tt = tool_texts();
    if p.rows.is_empty() || p.total <= 0.0 {
        ui.weak(tt.no_data);
        return;
    }
    let accent = theme::accent(ui);
    let gray = Color32::from_gray(140);
    let names: Vec<String> = p.rows.iter().map(|r| or_untitled(&r.name)).collect();
    let bars: Vec<Bar> = p
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| {
            Bar::new(i as f64, r.pct)
                .width(0.7)
                .name(&names[i])
                .fill(if r.vital { accent } else { gray })
        })
        .collect();
    let cum: Vec<[f64; 2]> = p
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| [i as f64, r.cum_pct])
        .collect();
    let n = names.len();
    Plot::new(id)
        .height(240.0)
        .allow_scroll(false)
        .allow_zoom(false)
        .allow_drag(false)
        .include_y(0.0)
        .include_y(105.0)
        .x_axis_formatter(move |mark, _| {
            let k = mark.value.round();
            if (mark.value - k).abs() < 1e-6 && k >= 0.0 && (k as usize) < n {
                names[k as usize].clone()
            } else {
                String::new()
            }
        })
        .y_axis_formatter(|mark, _| format!("{} %", mark.value))
        .show(ui, |plot| {
            plot.bar_chart(BarChart::new(tt.percentage, bars));
            plot.line(Line::new(tt.cumulative, PlotPoints::new(cum)).color(theme::WARN));
            plot.hline(HLine::new("80 %", pareto::VITAL_PCT).color(Color32::from_gray(170)));
        });
}

/// The fish: the effect at the head, one bone per category (half above, half below) and
/// the first causes of each bone, coloured by their state.
fn fishbone(ui: &mut egui::Ui, g: &ishikawa::Diagram) {
    let width = ui.available_width().clamp(640.0, 1100.0);
    let height = 340.0;
    let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let painter = ui.painter_at(rect);
    let fg = ui.visuals().text_color();
    let weak = ui.visuals().weak_text_color();
    let accent = theme::accent(ui);
    let mid = rect.center().y;
    let head_w = 180.0;
    let x0 = rect.left() + 10.0;
    let x1 = rect.right() - head_w - 10.0;
    painter.line_segment([pos2(x0, mid), pos2(x1, mid)], Stroke::new(2.5, fg));
    let head = egui::Rect::from_min_size(pos2(x1, mid - 40.0), vec2(head_w, 80.0));
    painter.rect_stroke(
        head,
        6.0,
        Stroke::new(2.0, accent),
        egui::StrokeKind::Inside,
    );
    let effect = if g.effect.trim().is_empty() {
        tool_texts().effect_missing.to_string()
    } else {
        g.effect.clone()
    };
    let galley = painter.layout(effect, FontId::proportional(13.0), fg, head_w - 12.0);
    let pos = head.center() - galley.size() / 2.0;
    painter.galley(pos, galley, fg);
    let n = g.bones.len();
    let top = n.div_ceil(2);
    for (i, b) in g.bones.iter().enumerate() {
        let up = i < top;
        let (k, side) = if up { (i, top) } else { (i - top, n - top) };
        let span = (x1 - x0) / (side as f32 + 0.3);
        let xe = x0 + span * (k as f32 + 1.0);
        let ys = if up {
            rect.top() + 26.0
        } else {
            rect.bottom() - 26.0
        };
        let xs = xe - 70.0;
        painter.line_segment([pos2(xs, ys), pos2(xe, mid)], Stroke::new(1.5, fg));
        let (anchor, ly) = if up {
            (Align2::CENTER_BOTTOM, ys - 2.0)
        } else {
            (Align2::CENTER_TOP, ys + 2.0)
        };
        painter.text(
            pos2(xs, ly),
            anchor,
            ishikawa::category_name(&b.category),
            FontId::proportional(14.0),
            accent,
        );
        let roots: Vec<&ishikawa::Cause> = b.causes.iter().filter(|c| c.depth == 0).collect();
        let shown = roots.len().min(6);
        for (j, c) in roots.iter().take(shown).enumerate() {
            let t = (j as f32 + 1.0) / (shown as f32 + 1.0);
            let py = ys + (mid - ys) * t;
            let px = xs + (xe - xs) * t;
            painter.line_segment([pos2(px - 10.0, py), pos2(px, py)], Stroke::new(1.0, weak));
            let color = match c.status {
                ishikawa::CauseStatus::Verified => accent,
                ishikawa::CauseStatus::Probable => theme::WARN,
                ishikawa::CauseStatus::RuledOut => weak,
                ishikawa::CauseStatus::Open => fg,
            };
            let mut label: String = c.text.chars().take(26).collect();
            if c.text.chars().count() > 26 {
                label.push('…');
            }
            painter.text(
                pos2(px - 12.0, py),
                Align2::RIGHT_CENTER,
                label,
                FontId::proportional(11.0),
                color,
            );
        }
        if roots.len() > shown {
            painter.text(
                pos2(xe - 4.0, if up { mid - 10.0 } else { mid + 10.0 }),
                Align2::RIGHT_CENTER,
                format!("+{}", roots.len() - shown),
                FontId::proportional(11.0),
                weak,
            );
        }
    }
}

/// The lead time ladder: for each process, the wait of the inventory before it (the low
/// step, orange) and its cycle time (the high step, green when it adds value).
fn draw_ladder(ui: &mut egui::Ui, l: &vsm::Ladder) {
    if l.steps.is_empty() {
        return;
    }
    let t = l10();
    let w = ui.available_width().clamp(320.0, 1100.0);
    let (resp, painter) = ui.allocate_painter(egui::vec2(w, 96.0), egui::Sense::hover());
    let r = resp.rect;
    let n = l.steps.len() as f32;
    let seg = r.width() / (2.0 * n);
    let (hi, lo) = (r.top() + 30.0, r.top() + 62.0);
    let text = ui.visuals().text_color();
    let weak = ui.visuals().weak_text_color();
    let font = egui::FontId::proportional(10.0);
    let num = |x: f64| crate::app::screens::show_number((x * 100.0).round() / 100.0);
    for (i, st) in l.steps.iter().enumerate() {
        let x0 = r.left() + 2.0 * i as f32 * seg;
        let x1 = x0 + seg;
        let x2 = x1 + seg;
        let wait = egui::Stroke::new(2.5, theme::WARN);
        painter.line_segment([egui::pos2(x0, lo), egui::pos2(x1, lo)], wait);
        painter.line_segment(
            [egui::pos2(x1, lo), egui::pos2(x1, hi)],
            egui::Stroke::new(1.0, weak),
        );
        let c = if st.value_added { theme::OK } else { weak };
        painter.line_segment(
            [egui::pos2(x1, hi), egui::pos2(x2, hi)],
            egui::Stroke::new(2.5, c),
        );
        if i + 1 < l.steps.len() {
            painter.line_segment(
                [egui::pos2(x2, hi), egui::pos2(x2, lo)],
                egui::Stroke::new(1.0, weak),
            );
        }
        painter.text(
            egui::pos2(x1 + seg / 2.0, hi - 4.0),
            egui::Align2::CENTER_BOTTOM,
            &st.name,
            font.clone(),
            text,
        );
        painter.text(
            egui::pos2(x1 + seg / 2.0, hi + 4.0),
            egui::Align2::CENTER_TOP,
            format!("{} {}", t.vsm_proc, num(st.process)),
            font.clone(),
            c,
        );
        painter.text(
            egui::pos2(x0 + seg / 2.0, lo + 4.0),
            egui::Align2::CENTER_TOP,
            st.wait
                .map_or_else(|| "?".to_string(), |w| format!("{} {}", t.vsm_wait, num(w))),
            font.clone(),
            theme::WARN,
        );
    }
}
