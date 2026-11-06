//! Stage L3 and P1 drawn without a window: a project with a real data set and a Gage R&R
//! study imported into `dados/`, the analyses screen drawn on each kind of analysis, the
//! capability and the Gage R&R saved with their recipes, and the two Medir checks of the
//! tollgate verified by the app from them.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use utsim_lss_app::App;
use utsim_lss_app::app::Nav;
use utsim_lss_app::app::screens::analyses::KINDS;
use utsim_lss_core::analises::{self, Recipe, Table};
use utsim_lss_core::dados::{self, DataOrigin, DatasetInput};
use utsim_lss_core::domain::{self, Approach};
use utsim_lss_core::tollgate::{self, Check};
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

fn frames(app: &mut App, ctx: &egui::Context, n: usize) {
    for _ in 0..n {
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| app.draw(ui));
        out.textures_delta.clear();
    }
}

/// A crossed study (10 parts, 3 operators, 2 repetitions) with a small measurement error.
fn gage_csv() -> String {
    let mut s = String::from("peca;operador;medida\n");
    for o in 0..3 {
        for p in 0..10 {
            for r in 0..2 {
                let v = 10.0 + 0.3 * p as f64 + 0.002 * ((o * 7 + p * 3 + r * 5) % 9) as f64;
                s.push_str(&format!(
                    "P{};O{};{}\n",
                    p + 1,
                    o + 1,
                    format!("{v:.4}").replace('.', ",")
                ));
            }
        }
    }
    s
}

fn process_csv() -> String {
    let mut s = String::from("diametro\n");
    for i in 0..40 {
        let v = 10.0 + 0.05 * (((i * 37) % 11) as f64 - 5.0) / 5.0;
        s.push_str(&format!("{v:.4}\n"));
    }
    s
}

#[test]
fn analyses_screen_and_the_medir_checks() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let counter = Arc::new(AtomicU64::new(0));
    let mut app = App::with_clock(
        AppDirs::under(&dir.path().join("cfg")),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 4))),
    );
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com"));
    let work = dir.path().join("Drive");
    std::fs::create_dir_all(&work).unwrap();
    assert!(app.create_project(&work, "Diâmetro do eixo", Approach::Dmaic));
    let (proc_id, gage_id) = {
        let s = app.session_mut().unwrap();
        let mut ids = Vec::new();
        for (name, csv) in [
            ("Diâmetros", process_csv()),
            ("Estudo do paquímetro", gage_csv()),
        ] {
            let (file, hash) = s.folder().add_data(csv.as_bytes(), "csv").unwrap();
            let (columns, rows) = dados::shape(&csv);
            let id = s.new_id();
            let input = DatasetInput {
                name: name.into(),
                origin: DataOrigin::Real,
                file,
                hash,
                columns,
                rows,
                date: Some(s.today()),
                ..DatasetInput::default()
            };
            let e = dados::register(s.project(), id, &input);
            s.apply(e).unwrap();
            ids.push(id);
        }
        (ids[0], ids[1])
    };
    // Every kind of analysis draws on the screen, on each data set.
    app.nav = Nav::Analyses;
    for ds in [proc_id, gage_id] {
        for k in KINDS {
            app.select_analysis(ds, k);
            frames(&mut app, &ctx, 2);
            assert!(
                app.status().is_none_or(|(_, e)| !e),
                "{k}: {:?}",
                app.status()
            );
        }
    }

    // The capability and the Gage R&R saved, as the buttons do.
    let s = app.session_mut().unwrap();
    let p = s.project();
    let ds = dados::datasets(s.state(), p);
    for (d, recipe) in [
        (
            &ds[0],
            Recipe::Capability {
                col: 0,
                lsl: Some(9.8),
                usl: Some(10.2),
                target: None,
                subgroup: 1,
            },
        ),
        (
            &ds[1],
            Recipe::GageRr {
                part: 0,
                operator: 1,
                value: 2,
                tolerance: Some(1.0),
                anova: true,
            },
        ),
    ] {
        let text = s.folder().read_data(&d.file).unwrap();
        let o = analises::run(&recipe, &Table::parse(&text)).unwrap();
        let e = analises::record(p, s.new_id(), "", d, &recipe, &o, s.today());
        s.apply(e).unwrap();
    }
    let saved = analises::analyses(s.state(), p);
    assert_eq!(saved.len(), 2);
    assert!(saved[1].grr().unwrap() < 30.0, "{:?}", saved[1].outcome);
    // The Medir checks pass from the saved analyses.
    let medir = domain::phases(s.state(), p)[1].id;
    assert!(tollgate::verify(s.state(), p, medir, &Check::MsaApproved));
    assert!(tollgate::verify(s.state(), p, medir, &Check::BaselineReal));
    assert_eq!(
        Check::from_key(&Check::MsaApproved.key()),
        Some(Check::MsaApproved)
    );
    // A rerun gives the same numbers.
    let text = s.folder().read_data(&ds[1].file).unwrap();
    assert_eq!(
        format!("{:?}", analises::rerun(&saved[1], &text).unwrap().lines),
        format!("{:?}", saved[1].outcome.lines)
    );
    frames(&mut app, &ctx, 2);
}
