//! Stage L4 screen drawn without a window: a project linked to a small model (copied into
//! the project folder), the process chart drawn, the replications run on the worker
//! thread, a simulated data set saved with its origin, the SIPOC generated as a
//! deliverable, and the warning when the model file changes.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use eframe::egui;
use utsim_lss_app::App;
use utsim_lss_app::app::Nav;
use utsim_lss_core::dados::{self, DataOrigin};
use utsim_lss_core::domain::Approach;
use utsim_lss_core::{simulacao, tools};
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

const MM1: &str = r#"Model(
    name: "M/M/1 curta",
    run: (stop_time: 2000.0, warmup: 100.0, seed: 3, replications: 4),
    objects: [
        (name: "chegada", kind: Source(interarrival: Exp(mean: 1.0)), position: (0.0, 0.0, 0.0)),
        (name: "fila",    kind: Queue(),                                position: (3.0, 0.0, 0.0)),
        (name: "maquina", kind: Processor(process: Exp(mean: 0.8)),     position: (6.0, 0.0, 0.0)),
        (name: "saida",   kind: Sink,                                   position: (9.0, 0.0, 0.0)),
    ],
    connections: [("chegada", "fila"), ("fila", "maquina"), ("maquina", "saida")],
)"#;

fn frames(app: &mut App, ctx: &egui::Context, n: usize) {
    for _ in 0..n {
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| app.draw(ui));
        out.textures_delta.clear();
    }
}

#[test]
fn model_screen_links_runs_and_generates() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let counter = Arc::new(AtomicU64::new(0));
    let mut app = App::with_clock(
        AppDirs::under(&dir.path().join("cfg")),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 5))),
    );
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com"));
    let work = dir.path().join("Drive");
    std::fs::create_dir_all(&work).unwrap();
    assert!(app.create_project(&work, "Fila da máquina", Approach::Dmaic));
    let model_file = dir.path().join("mm1.ron");
    std::fs::write(&model_file, MM1).unwrap();

    // Link (copied into the project) through the session, as the screen does.
    {
        let s = app.session_mut().unwrap();
        let rel = s.folder().copy_model(&model_file, MM1).unwrap();
        assert_eq!(rel, "modelos/mm1.ron");
        let m = simulacao::parse(MM1, None).unwrap();
        let id = s.new_id();
        let today = s.today();
        let e = simulacao::link_edits(s.project(), id, &rel, MM1, &m, true, today);
        s.apply(e).unwrap();
    }
    app.nav = Nav::Model;
    frames(&mut app, &ctx, 3);
    assert!(app.status().is_none_or(|(_, e)| !e), "{:?}", app.status());

    // Replications on the worker thread; the screen keeps drawing.
    assert!(app.start_model_runs());
    let t0 = Instant::now();
    while app.model_job_running() {
        frames(&mut app, &ctx, 1);
        assert!(
            t0.elapsed() < Duration::from_secs(120),
            "simulação não terminou"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    frames(&mut app, &ctx, 2);
    assert!(!app.model_kpis().is_empty());
    assert!(app.status().is_none_or(|(_, e)| !e), "{:?}", app.status());

    // A simulated data set and the generated SIPOC, as the buttons do.
    {
        let s = app.session_mut().unwrap();
        let link = simulacao::link(s.state(), s.project()).unwrap();
        let m = simulacao::parse(MM1, None).unwrap();
        let reports = m.run_replications().unwrap();
        let per = utsim_model::analysis::per_replication(&reports);
        let csv = simulacao::replication_csv(&per, &[]);
        let (file, hash) = s.folder().add_data(csv.as_bytes(), "csv").unwrap();
        assert_eq!(s.folder().read_data(&file).unwrap(), csv);
        let input =
            simulacao::simulated_dataset("Fila", &m, &link.hash, "", &file, &hash, &csv, s.today());
        let e = dados::register(s.project(), s.new_id(), &input);
        s.apply(e).unwrap();
        let cols = simulacao::sipoc_columns(&m).unwrap();
        let dl = s.new_id();
        let e = simulacao::sipoc_edits(s.state(), s.project(), dl, &link.hash, &cols, s.ids());
        s.apply(e).unwrap();
        let sp = tools::sipoc::of_deliverable(s.state(), dl);
        assert!(sp.steps() > 0);
        let ds = dados::datasets(s.state(), s.project());
        assert_eq!(ds[0].origin, DataOrigin::Simulated);
    }
    frames(&mut app, &ctx, 2);

    // The copied model changes: what was made from the old version is listed.
    let root = app.session().unwrap().folder().root().to_path_buf();
    std::fs::write(
        root.join("modelos/mm1.ron"),
        MM1.replace("seed: 3", "seed: 4"),
    )
    .unwrap();
    app.nav = Nav::Overview;
    frames(&mut app, &ctx, 1);
    app.nav = Nav::Model;
    frames(&mut app, &ctx, 2);
    let s = app.session().unwrap();
    let now = simulacao::hash_text(&std::fs::read_to_string(root.join("modelos/mm1.ron")).unwrap());
    assert_eq!(simulacao::stale(s.state(), s.project(), &now).len(), 2);
}
