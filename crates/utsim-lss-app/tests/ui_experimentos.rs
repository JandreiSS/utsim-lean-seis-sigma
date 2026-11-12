//! The design of experiments drawn without a window: a 2² on two parameters of a small
//! model, the collection worksheet saved, the experiment run on the worker thread and saved
//! as a simulated data set, and the factorial analysis of it on the analyses screen.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use eframe::egui;
use utsim_lss_app::App;
use utsim_lss_app::app::Nav;
use utsim_lss_core::analises::{self, Recipe, Table};
use utsim_lss_core::dados::{self, DataOrigin};
use utsim_lss_core::domain::Approach;
use utsim_lss_core::experimentos::{self, ExperimentSpec, FactorSpec};
use utsim_lss_core::simulacao;
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
fn experiment_designed_run_on_the_model_and_analysed() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let counter = Arc::new(AtomicU64::new(0));
    let mut app = App::with_clock(
        AppDirs::under(&dir.path().join("cfg")),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 11))),
    );
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com"));
    let work = dir.path().join("Drive");
    std::fs::create_dir_all(&work).unwrap();
    assert!(app.create_project(&work, "Fila da máquina", Approach::Dmaic));
    let model_file = dir.path().join("mm1.ron");
    std::fs::write(&model_file, MM1).unwrap();
    {
        let s = app.session_mut().unwrap();
        let rel = s.folder().copy_model(&model_file, MM1).unwrap();
        let m = simulacao::parse(MM1, None).unwrap();
        let id = s.new_id();
        let today = s.today();
        let e = simulacao::link_edits(s.project(), id, &rel, MM1, &m, true, today);
        s.apply(e).unwrap();
    }
    app.nav = Nav::Experiments;
    frames(&mut app, &ctx, 2);
    app.set_experiment(ExperimentSpec {
        name: "Máquina e chegadas".into(),
        factors: vec![
            FactorSpec {
                name: "Processo".into(),
                low: "Exp(mean: 0.8)".into(),
                high: "Exp(mean: 0.5)".into(),
                key: "maquina.process".into(),
                ..FactorSpec::default()
            },
            FactorSpec {
                name: "Chegadas".into(),
                low: "Exp(mean: 1.0)".into(),
                high: "Exp(mean: 1.2)".into(),
                key: "chegada.interarrival".into(),
                ..FactorSpec::default()
            },
        ],
        replicates: 1,
        seed: 5,
        ..ExperimentSpec::default()
    });
    frames(&mut app, &ctx, 2);
    assert!(app.status().is_none_or(|(_, e)| !e), "{:?}", app.status());

    // The worksheet in dados/, with the plan in its first line.
    let sheet = app.save_experiment_sheet().unwrap();
    let text = app.session().unwrap().folder().read_data(&sheet).unwrap();
    assert!(text.starts_with("# planejamento: {"));
    assert_eq!(Table::parse(&text).rows.len(), 4);

    // The experiment on the model, on the worker thread.
    assert!(app.start_experiment_on_model());
    let t0 = Instant::now();
    while app.experiment_running() {
        frames(&mut app, &ctx, 1);
        assert!(
            t0.elapsed() < Duration::from_secs(120),
            "o experimento não terminou"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    frames(&mut app, &ctx, 1);
    assert!(app.status().is_some_and(|(_, e)| !e), "{:?}", app.status());
    let s = app.session().unwrap();
    let ds = dados::datasets(s.state(), s.project());
    assert_eq!(ds.len(), 1);
    assert_eq!(ds[0].origin, DataOrigin::Simulated);
    let csv = s.folder().read_data(&ds[0].file).unwrap();
    let table = Table::parse(&csv);
    assert_eq!(table.rows.len(), 4 * 4);
    let (f, _) = experimentos::columns(&table.header);
    let util = table
        .header
        .iter()
        .position(|h| h.contains("maquina") && h.contains("util"))
        .unwrap();
    let o = analises::run(
        &Recipe::Factorial {
            factors: f,
            response: util,
            order: 2,
        },
        &table,
    )
    .unwrap();
    assert!(o.value("efeito A").unwrap() < 0.0);

    // The analyses screen finds the factor columns and draws the effects.
    let id = ds[0].id;
    app.nav = Nav::Analyses;
    app.select_analysis(id, "experimento_fatorial");
    frames(&mut app, &ctx, 3);
    assert!(app.status().is_none_or(|(_, e)| !e), "{:?}", app.status());
}

#[test]
fn plackett_burman_screening_drawn_and_saved() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let counter = Arc::new(AtomicU64::new(0));
    let mut app = App::with_clock(
        AppDirs::under(&dir.path().join("cfg")),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 12))),
    );
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com"));
    let work = dir.path().join("Drive");
    std::fs::create_dir_all(&work).unwrap();
    assert!(app.create_project(&work, "Triagem da solda", Approach::Dmaic));
    app.nav = Nav::Experiments;
    frames(&mut app, &ctx, 2);
    // Eleven factors: past the table of fractions, so the screen keeps the screening.
    app.set_experiment(ExperimentSpec {
        name: "Triagem".into(),
        factors: (0..11)
            .map(|i| FactorSpec {
                name: format!("Fator {}", i + 1),
                low: "1".into(),
                high: "2".into(),
                ..FactorSpec::default()
            })
            .collect(),
        replicates: 1,
        center_points: 2,
        seed: 9,
        ..ExperimentSpec::default()
    });
    frames(&mut app, &ctx, 2);
    assert!(app.status().is_none_or(|(_, e)| !e), "{:?}", app.status());
    let sheet = app.save_experiment_sheet().unwrap();
    let text = app.session().unwrap().folder().read_data(&sheet).unwrap();
    let t = Table::parse(&text);
    assert_eq!(t.rows.len(), 12 + 2);
    let (f, resp) = experimentos::columns(&t.header);
    assert_eq!(f.len(), 11);
    assert!(resp.is_some());
}
