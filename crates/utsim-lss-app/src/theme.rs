//! Look of the app: egui's own dark and light themes, as in the launcher, with a little
//! more air and one accent colour.

use eframe::egui::{self, Color32, CornerRadius, Theme, ThemePreference, vec2};
use utsim_lss_store::user;

/// Accent (selection, links, the current phase).
pub const ACCENT_DARK: Color32 = Color32::from_rgb(72, 140, 220);
pub const ACCENT_LIGHT: Color32 = Color32::from_rgb(28, 98, 180);
/// Warnings (conflicts, overdue).
pub const WARN: Color32 = Color32::from_rgb(224, 150, 40);
/// Met, valid, passed.
pub const OK: Color32 = Color32::from_rgb(70, 160, 90);

pub fn apply(ctx: &egui::Context, pref: user::Theme) {
    ctx.set_theme(match pref {
        user::Theme::System => ThemePreference::System,
        user::Theme::Light => ThemePreference::Light,
        user::Theme::Dark => ThemePreference::Dark,
    });
    for (theme, accent) in [(Theme::Dark, ACCENT_DARK), (Theme::Light, ACCENT_LIGHT)] {
        ctx.style_mut_of(theme, |s| {
            s.spacing.item_spacing = vec2(8.0, 6.0);
            s.spacing.button_padding = vec2(8.0, 4.0);
            s.visuals.selection.bg_fill = accent;
            s.visuals.selection.stroke.color = Color32::WHITE;
            s.visuals.hyperlink_color = accent;
            for w in [
                &mut s.visuals.widgets.inactive,
                &mut s.visuals.widgets.hovered,
                &mut s.visuals.widgets.active,
                &mut s.visuals.widgets.open,
                &mut s.visuals.widgets.noninteractive,
            ] {
                w.corner_radius = CornerRadius::same(4);
            }
        });
    }
}

/// The accent of the theme in use.
pub fn accent(ui: &egui::Ui) -> Color32 {
    if ui.visuals().dark_mode {
        ACCENT_DARK
    } else {
        ACCENT_LIGHT
    }
}

/// A card or section frame.
pub fn card(ui: &egui::Ui) -> egui::Frame {
    egui::Frame::group(ui.style())
        .corner_radius(CornerRadius::same(6))
        .inner_margin(8.0)
}
