//! The control plan in operation without a window (P5): a line points to a KPI of the twin
//! panel (a saved panel JSON), the limits come from the model, a point out of control
//! opens the card of the response signal once, the series becomes a data set of origin
//! Gêmeo and the reading enters the realised month of the linked saving.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use serde_json::Value;
use utsim_lss_app::App;
use utsim_lss_app::app::Nav;
use utsim_lss_core::controle::{self, LimitsFrom};
use utsim_lss_core::dados::{self, DataOrigin};
use utsim_lss_core::domain::{Approach, SavingCategory};
use utsim_lss_core::kanban;
use utsim_lss_core::savings;
use utsim_lss_core::tools::{Tool, plans};
use utsim_lss_store::user::AppDirs;
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

const PANEL: &str = r#"{"modelo":"linha","tempo":50,
 "janelas":[{"index":0,"start":0,"end":10,"values":{"vazao":12.1}},
            {"index":1,"start":10,"end":20,"values":{"vazao":11.8}},
            {"index":2,"start":20,"end":30,"values":{"vazao":12.3}},
            {"index":3,"start":30,"end":40,"values":{"vazao":9.9}},
            {"index":4,"start":40,"end":50,"values":{"vazao":12.4}}],
 "kpis":{"vazao":{"simulado_media":12.0,"simulado_dp":0.6,"n_simulado":50}},
 "alertas":[{"t":40,"kpi":"vazao","nivel":"Alerta","teste":"BH","mensagem":"abaixo do modelo"}]}"#;

#[test]
fn control_plan_read_on_the_twin() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let counter = Arc::new(AtomicU64::new(0));
    let mut app = App::with_clock(
        AppDirs::under(&dir.path().join("cfg")),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 31))),
    );
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com"));
    let work = dir.path().join("Drive");
    std::fs::create_dir_all(&work).unwrap();
    assert!(app.create_project(&work, "Vazão da linha", Approach::Dmaic));
    let panel = dir.path().join("painel.json");
    std::fs::write(&panel, PANEL).unwrap();
    let (plan, line, signal, saving) = {
        let s = app.session_mut().unwrap();
        let p = s.project();
        let resp = s
            .add_deliverable(Tool::ResponsePlan, "Plano de resposta")
            .unwrap();
        let signal = s
            .add_item(
                resp,
                plans::RESPONSE_ITEM,
                None,
                vec![
                    (
                        plans::fields::SIGNAL,
                        Value::from("Vazão abaixo do controle"),
                    ),
                    (plans::fields::IMMEDIATE, Value::from("Conferir o gargalo")),
                ],
            )
            .unwrap();
        let saving = s.new_id();
        s.apply(savings::new_saving(
            p,
            saving,
            "Mais peças por hora",
            SavingCategory::Hard,
            6000.0,
            savings::Source::Simulation,
        ))
        .unwrap();
        let plan = s
            .add_deliverable(Tool::ControlPlan, "Plano de controle")
            .unwrap();
        let line = s
            .add_item(
                plan,
                plans::CONTROL_ITEM,
                None,
                vec![
                    (plans::fields::CHARACTERISTIC, Value::from("Vazão")),
                    (plans::fields::SOURCE, Value::from("kpi:vazao")),
                    (plans::fields::RESPONSE, Value::from(signal.to_string())),
                    (plans::fields::SAVING, Value::from(saving.to_string())),
                    (plans::fields::BASELINE, Value::from(11.0)),
                    (plans::fields::PER_UNIT, Value::from(1000.0)),
                ],
            )
            .unwrap();
        s.apply(controle::twin_address_edits(plan, panel.to_str().unwrap()))
            .unwrap();
        (plan, line, signal, saving)
    };

    // The first reading opens one card and freezes the model's limits.
    assert_eq!(app.read_control_plan(plan), Ok(1));
    let r = app.control_reading(line).unwrap().clone().unwrap();
    assert_eq!(r.limits, (12.0, 10.2, 13.8));
    assert_eq!(r.new.len(), 1);
    assert_eq!(r.alerts.len(), 1);
    {
        let s = app.session().unwrap();
        let cards: Vec<_> = kanban::board(s.state(), s.project())
            .into_values()
            .flatten()
            .filter(|c| c.link == Some(signal))
            .collect();
        assert_eq!(cards.len(), 1);
        assert!(cards[0].why.contains("t = 40: 9,9"), "{}", cards[0].why);
        let w = &controle::watches(s.state(), plan)[0];
        assert_eq!(w.limits_from, LimitsFrom::Model);
        assert_eq!(w.checked, Some(50.0));
    }
    // Reading again opens nothing.
    assert_eq!(app.read_control_plan(plan), Ok(0));

    // The series as a data set of origin Gêmeo.
    let file = app.keep_twin_dataset(plan, line).expect("conjunto");
    {
        let s = app.session().unwrap();
        let d = dados::datasets(s.state(), s.project());
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].origin, DataOrigin::Twin);
        let csv = std::fs::read_to_string(s.folder().data_path(&file)).unwrap();
        assert!(csv.starts_with("t;vazao\n10;12.1\n"), "{csv}");
    }

    // Mean 11.7 over the baseline 11: 700 per month.
    assert!(!app.book_realized(plan, line, "2026-13"));
    assert!(app.book_realized(plan, line, "2026-11"));
    {
        let s = app.session().unwrap();
        let e = savings::entries(s.state(), saving);
        assert_eq!(e.len(), 1);
        assert!((e[0].value - 700.0).abs() < 1e-6, "{}", e[0].value);
    }

    // The screen draws with the readings.
    app.nav = Nav::Tools;
    app.selected_deliverable = Some(plan);
    for _ in 0..3 {
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| app.draw(ui));
        out.textures_delta.clear();
    }
    // With `UTSIM_CAPTURAS=<pasta>` the picture is saved for review.
    if let Some(d) = std::env::var_os("UTSIM_CAPTURAS") {
        let mut cap = Captura::new(egui::Context::default(), egui::vec2(1400.0, 1500.0));
        let img = cap.imagem(4, |ui| app.draw(ui));
        std::fs::create_dir_all(&d).unwrap();
        img.salvar_png(&std::path::Path::new(&d).join("controle.png"))
            .unwrap();
    }
}
