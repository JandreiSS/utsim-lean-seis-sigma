//! Pictures of the screens of version 2 drawn without a window (roadmap, analyses,
//! experiments, value stream map, reports) and the PDFs of the reports with cover and
//! contents. Each picture must paint something; with `UTSIM_CAPTURAS=<pasta>` the PNGs and
//! the PDFs are saved there for review.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui::{Context, vec2};
use serde_json::Value;
use utsim_lss_app::App;
use utsim_lss_app::app::Nav;
use utsim_lss_core::analises::{self, Recipe, Table};
use utsim_lss_core::dados::{self, DataOrigin, DatasetInput};
use utsim_lss_core::domain::{self, Approach};
use utsim_lss_core::experimentos::{ExperimentSpec, FactorSpec};
use utsim_lss_core::relatorios::{Template, pdf};
use utsim_lss_core::tools::{self, Tool, pareto, vsm};
use utsim_lss_store::user::AppDirs;
use utsim_lss_store::{Clock, SystemClock};
use utsim_ui_tema::captura::{Captura, Imagem};

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

fn out_dir() -> Option<std::path::PathBuf> {
    std::env::var_os("UTSIM_CAPTURAS").map(|d| {
        let d = std::path::PathBuf::from(d);
        std::fs::create_dir_all(&d).unwrap();
        d
    })
}

fn save(img: &Imagem, name: &str) {
    if let Some(d) = out_dir() {
        img.salvar_png(&d.join(format!("{name}.png"))).unwrap();
    }
}

fn colours(img: &Imagem) -> usize {
    let mut c = std::collections::BTreeSet::new();
    for y in (0..img.height).step_by(7) {
        for x in (0..img.width).step_by(7) {
            c.insert(img.pixel(x, y));
        }
    }
    c.len()
}

#[test]
fn screens_and_reports_of_version_2() {
    let dir = tempfile::tempdir().unwrap();
    let counter = Arc::new(AtomicU64::new(0));
    let mut app = App::with_clock(
        AppDirs::under(&dir.path().join("cfg")),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 21))),
    );
    assert!(app.submit_identity("Ana Souza", "ana@empresa.com"));
    let work = dir.path().join("Drive");
    std::fs::create_dir_all(&work).unwrap();
    assert!(app.create_project(&work, "Refugo na usinagem da tampa", Approach::Dmaic));
    let (fact, vsm_id) = {
        let s = app.session_mut().unwrap();
        let p = s.project();
        let par = s
            .add_deliverable(Tool::Pareto, "Defeitos da tampa")
            .unwrap();
        for (n, c) in [("Rebarba", 48), ("Risco", 22), ("Mancha", 9), ("Empeno", 5)] {
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
        for (n, c, inv) in [
            ("Corte", 1.5, 0.0),
            ("Usinagem", 2.4, 60.0),
            ("Inspeção", 0.6, 30.0),
        ] {
            s.add_item(
                vm,
                vsm::PROCESS_ITEM,
                None,
                vec![
                    (vsm::fields::NAME, Value::from(n)),
                    (vsm::fields::CYCLE, Value::from(c)),
                    (vsm::fields::INVENTORY, Value::from(inv)),
                    (vsm::fields::UPTIME, Value::from(90.0)),
                    (vsm::fields::VALUE_ADDED, Value::from(n != "Inspeção")),
                ],
            )
            .unwrap();
        }
        s.apply(vsm::header_edits(
            vm,
            Some(200.0),
            Some(450.0),
            Some("min"),
            None,
        ))
        .unwrap();
        let csv = "A;B;rendimento\n-1;-1;10\n1;-1;20\n-1;1;14\n1;1;30\n-1;-1;12\n1;-1;22\n-1;1;16\n1;1;28\n";
        let (file, hash) = s.folder().add_data(csv.as_bytes(), "csv").unwrap();
        let (columns, rows) = dados::shape(csv);
        let ds = s.new_id();
        let input = DatasetInput {
            name: "Ensaios da tampa".into(),
            origin: DataOrigin::Real,
            file,
            hash,
            columns,
            rows,
            ..DatasetInput::default()
        };
        s.apply(dados::register(p, ds, &input)).unwrap();
        let d = dados::datasets(s.state(), p)[0].clone();
        let r = Recipe::Factorial {
            factors: vec![0, 1],
            response: 2,
            order: 2,
        };
        let o = analises::run(&r, &Table::parse(csv)).unwrap();
        let id = s.new_id();
        let today = s.today();
        s.apply(analises::record(
            p,
            id,
            "Efeitos no rendimento",
            &d,
            &r,
            &o,
            today,
        ))
        .unwrap();
        (ds, vm)
    };
    let mut cap = Captura::new(Context::default(), vec2(1280.0, 900.0));
    let shots: [(&str, Nav); 4] = [
        ("roteiro", Nav::Roadmap),
        ("experimentos", Nav::Experiments),
        ("relatorios", Nav::Reports),
        ("analises", Nav::Analyses),
    ];
    app.set_experiment(ExperimentSpec {
        name: "Temperatura e avanço".into(),
        factors: vec![
            FactorSpec {
                name: "Temperatura".into(),
                low: "180".into(),
                high: "200".into(),
                ..FactorSpec::default()
            },
            FactorSpec {
                name: "Avanço".into(),
                low: "0,1".into(),
                high: "0,2".into(),
                ..FactorSpec::default()
            },
            FactorSpec {
                name: "Ferramenta".into(),
                low: "A".into(),
                high: "B".into(),
                ..FactorSpec::default()
            },
        ],
        fraction: 0,
        screening: false,
        surface: None,
        general: false,
        blocks: 1,
        replicates: 2,
        center_points: 0,
        seed: 7,
        response: "rendimento".into(),
    });
    app.select_analysis(fact, "experimento_fatorial");
    for (name, nav) in shots {
        app.nav = nav;
        let img = cap.imagem(4, |ui| app.draw(ui));
        save(&img, name);
        assert!(colours(&img) > 3, "{name}");
    }
    app.nav = Nav::Tools;
    app.selected_deliverable = Some(vsm_id);
    let img = cap.imagem(4, |ui| app.draw(ui));
    save(&img, "vsm");
    assert!(colours(&img) > 3);

    // The reports, with the cover and the contents where they are long.
    let s = app.session().unwrap();
    let st = s.state().clone();
    let p = s.project();
    let ph = domain::phases(&st, p)[1].id;
    let a = analises::analyses(&st, p)[0].clone();
    let v = tools::of_tool(&st, p, Tool::Vsm)[0].clone();
    let docs = [
        ("tollgate", utsim_lss_app::reports::tollgate_doc(&st, p, ph)),
        ("analise", utsim_lss_app::reports::analysis_doc(&st, p, &a)),
        ("vsm", utsim_lss_app::reports::vsm_doc(&st, p, &v)),
        ("a3", utsim_lss_app::reports::a3_doc(&st, p)),
    ];
    for (name, mut d) in docs {
        utsim_lss_app::reports::dress(
            &mut d,
            "Ana Souza",
            "08/11/2026",
            "Refugo na usinagem da tampa",
        );
        let b = pdf::render(
            &d,
            &Template::default(),
            "Impresso por Ana Souza em 08/11/2026",
        );
        assert!(b.starts_with(b"%PDF-1.4"), "{name}");
        if let Some(dir) = out_dir() {
            std::fs::write(dir.join(format!("{name}.pdf")), &b).unwrap();
        }
    }
}
