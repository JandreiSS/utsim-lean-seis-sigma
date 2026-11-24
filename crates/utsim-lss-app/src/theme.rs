//! Look of the app: the family's common theme (`utsim_ui_tema`), with the app's green on
//! the selection and the primary button. The app has no theme of its own.

use eframe::egui::{self, Color32, CornerRadius};

/// Warnings (conflicts, overdue).
pub const WARN: Color32 = Color32::from_rgb(224, 150, 40);
/// Met, valid, passed: blue-green, apart from the green accent; text in this colour
/// always carries a ✔ so it never depends on colour alone.
pub const OK: Color32 = Color32::from_rgb(38, 160, 150);

/// Applies the family's style and the app's colour to `ctx` (the fonts are left alone).
pub fn apply(ctx: &egui::Context) {
    utsim_ui_tema::install_style(ctx);
    utsim_ui_tema::usar_cor_do_app(ctx, crate::app::screens::inicio::COR);
}

/// The accent of the app (links, the current phase, who is online).
pub fn accent(_ui: &egui::Ui) -> Color32 {
    crate::app::screens::inicio::COR
}

/// A card or section frame.
pub fn card(ui: &egui::Ui) -> egui::Frame {
    egui::Frame::group(ui.style())
        .corner_radius(CornerRadius::same(6))
        .inner_margin(8.0)
}
