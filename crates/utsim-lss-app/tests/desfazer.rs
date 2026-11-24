//! Desfazer e refazer no UTSim LSS: a volta é uma edição nova no diário (nada some), refazer
//! traz de novo, e desfazer um apagar devolve o que foi apagado.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use serde_json::Value;
use utsim_lss_app::App;
use utsim_lss_core::domain::{Project, fields};
use utsim_lss_core::tools::{self, Tool, ishikawa};
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

fn app_in(dir: &std::path::Path, sal: u8) -> App {
    let counter = Arc::new(AtomicU64::new(0));
    let mut app = App::with_clock(
        AppDirs::under(dir),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), sal))),
    );
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com.br"));
    app
}

fn problema(app: &App) -> String {
    let s = app.session().unwrap();
    Project::read(s.project(), s.state().entity(s.project()).unwrap()).problem
}

#[test]
fn desfazer_volta_o_campo_com_uma_edicao_nova_no_diario() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = app_in(&dir.path().join("config"), 31);
    assert!(app.comecar_sem_pasta());
    assert!(
        !app.session().unwrap().pode_desfazer(),
        "abrir não é um passo"
    );
    {
        let s = app.session_mut().unwrap();
        let p = Project::entity_ref(s.project());
        s.set_field(
            p,
            fields::project::PROBLEM,
            Value::from("Refugo alto"),
            None,
        )
        .unwrap();
        s.set_field(
            p,
            fields::project::PROBLEM,
            Value::from("Refugo de 4,8 %"),
            None,
        )
        .unwrap();
    }
    let ops = app.session().unwrap().folder().journal().len();
    app.desfazer();
    assert_eq!(problema(&app), "Refugo alto");
    assert!(
        app.session().unwrap().folder().journal().len() > ops,
        "a volta é uma operação nova, não um apagar"
    );
    app.desfazer();
    assert_eq!(problema(&app), "");
    assert!(!app.session().unwrap().pode_desfazer());
    app.refazer();
    app.refazer();
    assert_eq!(problema(&app), "Refugo de 4,8 %");
    assert!(!app.session().unwrap().pode_refazer());
    // Um passo novo depois de desfazer apaga o refazer.
    app.desfazer();
    {
        let s = app.session_mut().unwrap();
        let p = Project::entity_ref(s.project());
        s.set_field(p, fields::project::GOAL, Value::from("2 %"), None)
            .unwrap();
    }
    assert!(!app.session().unwrap().pode_refazer());
}

#[test]
fn desfazer_um_apagar_devolve_a_causa() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let mut app = app_in(&dir.path().join("config"), 32);
    assert!(app.comecar_sem_pasta());
    let causa = {
        let s = app.session_mut().unwrap();
        let d = s.add_deliverable(Tool::Ishikawa, "Ishikawa").unwrap();
        s.add_item(
            d,
            ishikawa::ITEM,
            None,
            vec![
                (ishikawa::fields::TEXT, Value::from("Ferramenta gasta")),
                (ishikawa::fields::CATEGORY, Value::from("maquina")),
            ],
        )
        .unwrap()
    };
    let viva = |app: &App| {
        let s = app.session().unwrap();
        s.state().entity(causa).is_some_and(|e| !e.is_deleted())
    };
    app.session_mut().unwrap().delete_item(causa).unwrap();
    assert!(!viva(&app));
    app.desfazer();
    assert!(viva(&app), "desfazer devolve a causa apagada");
    assert_eq!(
        tools::all_items(app.session().unwrap().state(), {
            let s = app.session().unwrap();
            tools::of_tool(s.state(), s.project(), Tool::Ishikawa)[0].id
        })
        .len(),
        1
    );
    // A barra de baixo com Desfazer e Refazer desenha.
    for _ in 0..2 {
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| app.draw(ui));
        out.textures_delta.clear();
    }
}
