//! The switch of the gemba page in the team screen (P7, access through the local network with
//! a token; the server is in [`crate::rede`]).

use eframe::egui;

use super::super::App;
use crate::rede::{self, Gemba};
use crate::session::Session;
use crate::text::l15::l15;
use crate::theme;

/// The port tried first, so the address saved on a tablet keeps working from one day to the
/// next; when it is taken, any free one.
pub const PORT: u16 = 8790;

/// The state of the gemba page in the window.
#[derive(Default)]
pub struct GembaUi {
    server: Option<Gemba>,
    lan: bool,
    error: Option<String>,
    copied: bool,
}

impl GembaUi {
    /// Turns the page off (closing the project).
    pub fn close(&mut self) {
        *self = Self::default();
    }

    /// Rebuilds the page when it is due, and asks the window to come back for the next one
    /// even when nobody moves the mouse.
    pub fn refresh(&mut self, ctx: &egui::Context, s: &Session) {
        if let Some(g) = &mut self.server {
            if g.due() {
                g.set_page(rede::page(
                    s.state(),
                    s.project(),
                    s.today(),
                    &s.local_time(),
                ));
            }
            ctx.request_repaint_after(rede::REFRESH);
        }
    }

    fn start(&mut self) {
        let r = Gemba::start(self.lan, PORT).or_else(|_| Gemba::start(self.lan, 0));
        match r {
            Ok(g) => {
                self.server = Some(g);
                self.error = None;
                self.copied = false;
            }
            Err(e) => self.error = Some((l15().error)(&e.to_string())),
        }
    }
}

impl App {
    /// The section of the team screen.
    pub(in crate::app) fn gemba_section(&mut self, ui: &mut egui::Ui, s: &Session) {
        let t = l15();
        ui.add_space(12.0);
        ui.strong(t.title);
        ui.weak(t.intro);
        let g = &mut self.gemba;
        match g.server.as_ref().map(|sv| (sv.url(), sv.lan())) {
            None => {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut g.lan, t.open_lan)
                        .on_hover_text(t.open_lan_help);
                    if ui.button(t.start).clicked() {
                        g.start();
                        g.refresh(ui.ctx(), s);
                    }
                });
            }
            Some((url, lan)) => {
                ui.horizontal_wrapped(|ui| {
                    ui.label(t.address);
                    ui.hyperlink_to(&url, &url);
                    if ui.small_button(t.copy).clicked() {
                        ui.ctx().copy_text(url.clone());
                        g.copied = true;
                    }
                    if g.copied {
                        ui.weak(t.copied);
                    }
                });
                if lan {
                    ui.colored_label(theme::WARN, t.lan_warning);
                } else {
                    ui.weak(t.local_only);
                }
                if ui.button(t.stop).clicked() {
                    g.close();
                }
            }
        }
        if let Some(e) = &g.error {
            ui.colored_label(ui.visuals().error_fg_color, e);
        }
    }
}
