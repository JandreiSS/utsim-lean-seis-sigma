//! The DMAIC roadmap screen: a large vertical flowchart of the phases and their steps, the
//! tools of each step as boxes beside it (a tool of the app opens its screen in this
//! project; one not in the app yet is grey, "em breve"), the decisions that send the
//! project back, and what the project already has. Drawn with egui's painter, with
//! scrolling and zoom. The content is `utsim_lss_core::roteiro`.

use eframe::egui::{
    self, Align2, Color32, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, pos2, vec2,
};
use utsim_lss_core::roteiro::{self, Target};
use utsim_lss_core::tools;

use super::super::{App, Nav};
use crate::session::Session;
use crate::text::l9::l9;
use crate::theme;

/// The roadmap screen's state.
pub struct RoadmapUi {
    pub zoom: f32,
}

impl Default for RoadmapUi {
    fn default() -> Self {
        Self { zoom: 1.0 }
    }
}

/// The first `n` characters of a text, with "…" when cut.
fn clip(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        format!("{}…", s.chars().take(n).collect::<String>())
    }
}

const PHASE_W: f32 = 300.0;
const STEP_W: f32 = 260.0;
const STEP_H: f32 = 52.0;
const GAP: f32 = 18.0;
const TOOL_W: f32 = 170.0;
const TOOL_H: f32 = 22.0;

impl App {
    /// Opens where a tool of the roadmap lives.
    fn open_target(&mut self, s: &Session, target: Target) {
        match target {
            Target::Deliverable(tool) => {
                self.nav = Nav::Tools;
                self.new_tool = tool;
                self.selected_deliverable = tools::of_tool(s.state(), s.project(), tool)
                    .first()
                    .map(|d| d.id);
            }
            Target::Screen(key) => {
                if let Some(n) = Nav::from_key(key) {
                    self.nav = n;
                }
            }
            Target::Analysis(key) => {
                self.nav = Nav::Analyses;
                if let Some(i) = super::analyses::KINDS.iter().position(|k| *k == key) {
                    self.l3.kind = i;
                }
            }
            Target::Soon => {}
        }
    }

    pub(in crate::app) fn roadmap_screen(&mut self, ui: &mut egui::Ui, s: &mut Session) {
        let t = l9();
        ui.heading(t.nav_roadmap);
        ui.weak(t.intro);
        ui.horizontal(|ui| {
            ui.label(t.zoom);
            ui.add(egui::Slider::new(&mut self.roadmap.zoom, 0.5..=2.0));
            ui.colored_label(theme::OK, t.legend_done);
            ui.label(t.legend_todo);
            ui.weak(t.legend_soon);
        });
        let z = self.roadmap.zoom;
        let st = s.state().clone();
        let project = s.project();
        let accent = theme::accent(ui);
        let text = ui.visuals().text_color();
        let weak = ui.visuals().weak_text_color();
        let fill = ui.visuals().extreme_bg_color;
        let total_h: f32 = roteiro::PHASES
            .iter()
            .map(|p| 60.0 + p.steps.len() as f32 * (STEP_H + GAP))
            .sum::<f32>()
            + roteiro::DECISIONS.len() as f32 * 90.0
            + 40.0;
        let total_w = 40.0 + PHASE_W.max(STEP_W) + 40.0 + 3.0 * (TOOL_W + 8.0);
        let mut clicked: Option<Target> = None;
        egui::ScrollArea::both().id_salt("roteiro").show(ui, |ui| {
            let (resp, painter) =
                ui.allocate_painter(vec2(total_w * z, total_h * z), Sense::hover());
            let o = resp.rect.min;
            let p = |x: f32, y: f32| -> Pos2 { pos2(o.x + x * z, o.y + y * z) };
            let r =
                |x: f32, y: f32, w: f32, h: f32| Rect::from_min_size(p(x, y), vec2(w * z, h * z));
            let font = |s: f32| FontId::proportional(s * z);
            let arrow = |a: Pos2, b: Pos2, c: Color32| {
                painter.arrow(a, b - a, Stroke::new(1.5 * z, c));
            };
            let x0 = 40.0;
            let mut y = 20.0;
            let mut phase_top: Vec<f32> = Vec::new();
            let mut prev_bottom: Option<Pos2> = None;
            for (pi, ph) in roteiro::PHASES.iter().enumerate() {
                phase_top.push(y);
                let hr = r(x0, y, PHASE_W, 44.0);
                painter.rect_filled(hr, 6.0 * z, accent);
                painter.text(
                    hr.center() - vec2(0.0, 8.0 * z),
                    Align2::CENTER_CENTER,
                    format!("{}. {}", pi + 1, ph.name),
                    font(16.0),
                    Color32::WHITE,
                );
                painter.text(
                    hr.center() + vec2(0.0, 11.0 * z),
                    Align2::CENTER_CENTER,
                    ph.question,
                    font(9.5),
                    Color32::WHITE,
                );
                if let Some(b) = prev_bottom {
                    arrow(b, hr.center_top(), text);
                }
                y += 44.0 + GAP;
                let mut last = hr.center_bottom();
                for step in ph.steps {
                    let done = roteiro::step_done(&st, project, pi, step);
                    let sr = r(x0 + (PHASE_W - STEP_W) / 2.0, y, STEP_W, STEP_H);
                    arrow(last, sr.center_top(), weak);
                    painter.rect_filled(sr, 4.0 * z, fill);
                    painter.rect_stroke(
                        sr,
                        4.0 * z,
                        Stroke::new(
                            if done { 2.5 } else { 1.0 } * z,
                            if done { theme::OK } else { weak },
                        ),
                        StrokeKind::Inside,
                    );
                    let mark = if done { "✔ " } else { "" };
                    painter.text(
                        sr.left_top() + vec2(8.0 * z, 8.0 * z),
                        Align2::LEFT_TOP,
                        format!("{mark}{}", step.name),
                        font(12.0),
                        if done { theme::OK } else { text },
                    );
                    painter.text(
                        sr.left_top() + vec2(8.0 * z, 27.0 * z),
                        Align2::LEFT_TOP,
                        clip(step.purpose, 52),
                        font(9.0),
                        weak,
                    );
                    // The tools, in rows of three beside the step.
                    for (ti, tool) in step.tools.iter().enumerate() {
                        let col = (ti % 3) as f32;
                        let row = (ti / 3) as f32;
                        let tx = x0 + PHASE_W + 40.0 + col * (TOOL_W + 8.0);
                        let ty = y + row * (TOOL_H + 4.0);
                        let tr = r(tx, ty, TOOL_W, TOOL_H);
                        let soon = tool.target == Target::Soon;
                        let used = roteiro::tool_used(&st, project, tool.target);
                        let id = ui.id().with(("ferramenta", pi, step.name, ti));
                        let resp =
                            ui.interact(tr, id, if soon { Sense::hover() } else { Sense::click() });
                        let c = if soon {
                            weak
                        } else if used {
                            theme::OK
                        } else {
                            accent
                        };
                        painter.rect_stroke(
                            tr,
                            10.0 * z,
                            Stroke::new(1.0 * z, c),
                            StrokeKind::Inside,
                        );
                        let label = if soon {
                            format!("{} ({})", tool.name, t.soon)
                        } else {
                            tool.name.to_string()
                        };
                        painter.text(tr.center(), Align2::CENTER_CENTER, label, font(9.5), c);
                        if ti == 0 {
                            painter.line_segment(
                                [sr.right_center(), tr.left_center()],
                                Stroke::new(0.8 * z, weak),
                            );
                        }
                        if resp.hovered() && !soon {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        if resp.clicked() {
                            clicked = Some(tool.target);
                        }
                    }
                    last = sr.center_bottom();
                    y += STEP_H + GAP;
                }
                prev_bottom = Some(last);
                // A decision after this phase.
                for d in roteiro::DECISIONS.iter().filter(|d| d.after == pi) {
                    let c = p(x0 + PHASE_W / 2.0, y + 35.0);
                    let (hw, hh) = (130.0 * z, 32.0 * z);
                    let pts = vec![
                        pos2(c.x, c.y - hh),
                        pos2(c.x + hw, c.y),
                        pos2(c.x, c.y + hh),
                        pos2(c.x - hw, c.y),
                    ];
                    arrow(last, pts[0], weak);
                    painter.add(egui::Shape::convex_polygon(
                        pts.clone(),
                        fill,
                        Stroke::new(1.5 * z, accent),
                    ));
                    painter.text(c, Align2::CENTER_CENTER, d.question, font(9.5), text);
                    // "não": back to the phase, along the left.
                    let back_y = phase_top[d.back_to] + 22.0;
                    let left = p(x0 - 25.0, 0.0).x;
                    let s1 = pts[3];
                    let stroke = Stroke::new(1.5 * z, theme::WARN);
                    painter.line_segment([s1, pos2(left, s1.y)], stroke);
                    painter.line_segment([pos2(left, s1.y), pos2(left, p(0.0, back_y).y)], stroke);
                    arrow(pos2(left, p(0.0, back_y).y), p(x0, back_y), theme::WARN);
                    painter.text(
                        s1 + vec2(-6.0 * z, -8.0 * z),
                        Align2::RIGHT_BOTTOM,
                        t.no,
                        font(10.0),
                        theme::WARN,
                    );
                    painter.text(
                        pts[2] + vec2(8.0 * z, 2.0 * z),
                        Align2::LEFT_TOP,
                        t.yes,
                        font(10.0),
                        theme::OK,
                    );
                    prev_bottom = Some(pts[2]);
                    y += 90.0;
                }
            }
        });
        if let Some(target) = clicked {
            self.open_target(s, target);
        }
    }
}
