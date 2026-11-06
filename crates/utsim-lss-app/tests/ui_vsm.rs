//! The value stream map and the spaghetti diagram without a window: the current map
//! generated from a small model, the demand filled, the ladder drawn, the future map made
//! from it with less inventory, and the spaghetti distances saved as a model analysis.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use utsim_lss_app::App;
use utsim_lss_app::app::Nav;
use utsim_lss_core::domain::Approach;
use utsim_lss_core::simulacao::{self, AnalysisKind};
use utsim_lss_core::tools::vsm;
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
fn value_stream_map_and_spaghetti_from_the_model() {
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
    assert!(app.create_project(&work, "Fluxo da máquina", Approach::Dmaic));
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
    app.nav = Nav::Model;
    frames(&mut app, &ctx, 2);
    let cur = app.generate_vsm_from_model().expect("VSM do modelo");
    {
        let s = app.session_mut().unwrap();
        let rows = vsm::processes(s.state(), cur);
        assert_eq!(rows.len(), 1);
        assert!(rows[0].from_model && rows[0].inventory.is_some());
        s.apply(vsm::header_edits(
            cur,
            Some(400.0),
            Some(480.0),
            Some("min"),
            None,
        ))
        .unwrap();
    }
    app.nav = Nav::Tools;
    app.selected_deliverable = Some(cur);
    frames(&mut app, &ctx, 2);
    assert!(app.status().is_none_or(|(_, e)| !e), "{:?}", app.status());

    // The future state: a copy, then the inventory goes down.
    let fut = app.create_future_vsm(cur).unwrap();
    {
        let s = app.session_mut().unwrap();
        let h = vsm::header(s.state(), fut);
        assert!(h.future && h.from == Some(cur));
        let row = vsm::processes(s.state(), fut)[0].id;
        let e = vec![utsim_lss_core::Edit::new(
            utsim_lss_core::tools::item_ref(row),
            vsm::fields::INVENTORY,
            0.1,
        )];
        s.apply(e).unwrap();
        let before = vsm::ladder(
            &vsm::processes(s.state(), cur),
            &vsm::header(s.state(), cur),
        );
        let after = vsm::ladder(&vsm::processes(s.state(), fut), &h);
        assert!(after.lead_time.unwrap() <= before.lead_time.unwrap());
    }
    frames(&mut app, &ctx, 2);
    assert!(app.status().is_none_or(|(_, e)| !e), "{:?}", app.status());

    // The spaghetti distances as an analysis of the model.
    assert!(app.record_spaghetti_from_model());
    app.nav = Nav::Model;
    frames(&mut app, &ctx, 2);
    let s = app.session().unwrap();
    let a = simulacao::model_analyses(s.state(), s.project());
    assert!(a.iter().any(|x| x.kind == AnalysisKind::Spaghetti));
}

#[test]
fn charts_inside_the_reports() {
    use serde_json::Value;
    use utsim_lss_core::analises::{self, Recipe, Table};
    use utsim_lss_core::dados::{self, DataOrigin, DatasetInput};
    use utsim_lss_core::relatorios::{Block, Template, pdf};
    use utsim_lss_core::tools::{self, Tool, pareto};
    let dir = tempfile::tempdir().unwrap();
    let counter = Arc::new(AtomicU64::new(0));
    let mut app = App::with_clock(
        AppDirs::under(&dir.path().join("cfg")),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 13))),
    );
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com"));
    let work = dir.path().join("Drive");
    std::fs::create_dir_all(&work).unwrap();
    assert!(app.create_project(&work, "Rendimento", Approach::Dmaic));
    let s = app.session_mut().unwrap();
    let p = s.project();
    let par = s.add_deliverable(Tool::Pareto, "Defeitos").unwrap();
    for (n, c) in [("Rebarba", 30), ("Risco", 12), ("Mancha", 3)] {
        s.add_item(
            par,
            pareto::ITEM,
            None,
            vec![
                (pareto::fields::NAME, Value::from(n)),
                (pareto::fields::COUNT, Value::from(c)),
            ],
        )
        .unwrap();
    }
    let vm = s.add_deliverable(Tool::Vsm, "VSM atual").unwrap();
    s.add_item(
        vm,
        vsm::PROCESS_ITEM,
        None,
        vec![
            (vsm::fields::NAME, Value::from("Corte")),
            (vsm::fields::CYCLE, Value::from(2.0)),
            (vsm::fields::INVENTORY, Value::from(10.0)),
        ],
    )
    .unwrap();
    s.apply(vsm::header_edits(
        vm,
        Some(100.0),
        Some(400.0),
        Some("min"),
        None,
    ))
    .unwrap();
    // A 2² filled by hand, analysed and saved.
    let csv = "A;B;y\n-1;-1;10\n1;-1;20\n-1;1;14\n1;1;30\n-1;-1;12\n1;-1;22\n-1;1;16\n1;1;28\n";
    let (file, hash) = s.folder().add_data(csv.as_bytes(), "csv").unwrap();
    let (columns, rows) = dados::shape(csv);
    let ds_id = s.new_id();
    let input = DatasetInput {
        name: "Ensaios".into(),
        origin: DataOrigin::Real,
        file,
        hash,
        columns,
        rows,
        ..DatasetInput::default()
    };
    s.apply(dados::register(p, ds_id, &input)).unwrap();
    let ds = dados::datasets(s.state(), p)[0].clone();
    let r = Recipe::Factorial {
        factors: vec![0, 1],
        response: 2,
        order: 2,
    };
    let o = analises::run(&r, &Table::parse(csv)).unwrap();
    let id = s.new_id();
    let today = s.today();
    s.apply(analises::record(p, id, "Efeitos", &ds, &r, &o, today))
        .unwrap();
    let st = s.state().clone();
    let charts = |d: &utsim_lss_core::relatorios::Document| {
        d.blocks
            .iter()
            .filter(|b| matches!(b, Block::Chart(_)))
            .count()
    };
    let a = &analises::analyses(&st, p)[0];
    let ad = utsim_lss_app::reports::analysis_doc(&st, p, a);
    assert_eq!(charts(&ad), 3, "Pareto dos efeitos, principais e AB");
    let a3 = utsim_lss_app::reports::a3_doc(&st, p);
    assert_eq!(charts(&a3), 1);
    let vd = utsim_lss_app::reports::vsm_doc(&st, p, &tools::of_tool(&st, p, Tool::Vsm)[0]);
    assert_eq!(charts(&vd), 1);
    assert!(vd.plain_text().contains("Escada do lead time"));
    for d in [ad, a3, vd] {
        let b = pdf::render(&d, &Template::default(), "");
        assert!(b.starts_with(b"%PDF-1.4"));
        assert!(String::from_utf8_lossy(&b).contains(" re f 0 g"));
    }
}
