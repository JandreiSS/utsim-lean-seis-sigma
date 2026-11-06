//! The DMAIC roadmap drawn without a window, and every link of it opening a screen of the
//! app: a deliverable tool, a project screen or an analysis kind; the rest is "em breve".

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use utsim_lss_app::App;
use utsim_lss_app::app::Nav;
use utsim_lss_app::app::screens::analyses::KINDS;
use utsim_lss_core::domain::Approach;
use utsim_lss_core::roteiro::{self, Target};
use utsim_lss_store::user::AppDirs;
use utsim_lss_store::{Clock, SystemClock};

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
fn every_link_of_the_roadmap_opens_a_screen() {
    for t in roteiro::all_tools() {
        match t.target {
            Target::Screen(k) => assert!(
                Nav::from_key(k).is_some_and(|n| Nav::PROJECT.contains(&n)),
                "{}: tela {k}",
                t.name
            ),
            Target::Analysis(k) => assert!(KINDS.contains(&k), "{}: análise {k}", t.name),
            Target::Deliverable(_) | Target::Soon => {}
        }
    }
}

#[test]
fn roadmap_draws_at_every_zoom() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let counter = Arc::new(AtomicU64::new(0));
    let mut app = App::with_clock(
        AppDirs::under(&dir.path().join("cfg")),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 6))),
    );
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com"));
    let work = dir.path().join("Drive");
    std::fs::create_dir_all(&work).unwrap();
    assert!(app.create_project(&work, "Refugo", Approach::Dmaic));
    app.nav = Nav::Roadmap;
    for z in [0.5, 1.0, 2.0] {
        app.set_roadmap_zoom(z);
        for _ in 0..2 {
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| app.draw(ui));
            out.textures_delta.clear();
        }
        assert!(app.status().is_none_or(|(_, e)| !e), "{:?}", app.status());
        assert_eq!(app.nav, Nav::Roadmap);
    }
}
