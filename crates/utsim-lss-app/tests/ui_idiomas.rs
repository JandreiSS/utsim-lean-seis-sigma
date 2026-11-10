//! The interface in the three languages (P8): every project screen draws in Portuguese,
//! English and Spanish without an error, the catalogues really change, and the preference is
//! kept in the configuration. With `UTSIM_CAPTURAS=<pasta>` the overview and the team screen
//! are saved in each language for review.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use utsim_lss_app::App;
use utsim_lss_app::app::Nav;
use utsim_lss_app::text::{self, Language};
use utsim_lss_core::domain::Approach;
use utsim_lss_store::user::{AppDirs, UserConfig};
use utsim_lss_store::{Clock, SystemClock};
use utsim_ui_tema::captura::Captura;

#[derive(Clone)]
struct Salted(Arc<AtomicU64>, u8);

impl Clock for Salted {
    fn now_ms(&self) -> i64 {
        SystemClock.now_ms()
    }
    fn random(&self) -> [u8; 10] {
        let n = self.0.fetch_add(1, Ordering::Relaxed);
        let mut r = [self.1; 10];
        r[2..].copy_from_slice(&n.to_be_bytes());
        r
    }
}

#[test]
fn every_screen_draws_in_every_language() {
    let dir = tempfile::tempdir().unwrap();
    let counter = Arc::new(AtomicU64::new(0));
    let cfg = dir.path().join("cfg");
    let mut app = App::with_clock(
        AppDirs::under(&cfg),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 71))),
    );
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com"));
    let work = dir.path().join("Drive");
    std::fs::create_dir_all(&work).unwrap();
    assert!(app.create_project(&work, "Linha 2", Approach::Dmaic));
    let shots = std::env::var_os("UTSIM_CAPTURAS");
    let mut names = Vec::new();
    for l in Language::ALL {
        text::set_language(l);
        names.push(text::texts().nav_overview);
        let ctx = egui::Context::default();
        for nav in Nav::PROJECT {
            app.nav = nav;
            for _ in 0..2 {
                let mut out = ctx.run_ui(egui::RawInput::default(), |ui| app.draw(ui));
                out.textures_delta.clear();
            }
            assert!(
                app.status().is_none_or(|(_, err)| !*err),
                "{l:?} {nav:?}: {:?}",
                app.status()
            );
            if let Some(d) = &shots
                && matches!(nav, Nav::Overview | Nav::Team)
            {
                let mut cap = Captura::new(egui::Context::default(), egui::vec2(1400.0, 900.0));
                let img = cap.imagem(3, |ui| app.draw(ui));
                std::fs::create_dir_all(d).unwrap();
                img.salvar_png(&std::path::Path::new(d).join(format!("idioma_{l:?}_{nav:?}.png")))
                    .unwrap();
            }
        }
    }
    names.dedup();
    assert_eq!(names.len(), 3, "each language has its own texts: {names:?}");
    // The preference is saved and read back.
    let mut c = UserConfig::load(&cfg.join("config.json"));
    c.language = Language::Es;
    c.save(&cfg.join("config.json")).unwrap();
    assert_eq!(
        UserConfig::load(&cfg.join("config.json")).language,
        Language::Es
    );
    text::set_language(Language::PtBr);
}

#[test]
fn start_screen_draws_in_every_language() {
    let dir = tempfile::tempdir().unwrap();
    let counter = Arc::new(AtomicU64::new(0));
    let mut app = App::with_clock(
        AppDirs::under(&dir.path().join("cfg")),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 72))),
    );
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com"));
    let shots = std::env::var_os("UTSIM_CAPTURAS");
    let mut subtitles = Vec::new();
    for l in Language::ALL {
        text::set_language(l);
        subtitles.push(text::l14::l14().subtitle);
        let ctx = egui::Context::default();
        for _ in 0..2 {
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| app.draw(ui));
            out.textures_delta.clear();
        }
        if let Some(d) = &shots {
            let mut cap = Captura::new(egui::Context::default(), egui::vec2(1400.0, 900.0));
            let img = cap.imagem(3, |ui| app.draw(ui));
            std::fs::create_dir_all(d).unwrap();
            img.salvar_png(&std::path::Path::new(d).join(format!("idioma_{l:?}_inicio.png")))
                .unwrap();
        }
    }
    subtitles.dedup();
    assert_eq!(subtitles.len(), 3, "{subtitles:?}");
    text::set_language(Language::PtBr);
}
