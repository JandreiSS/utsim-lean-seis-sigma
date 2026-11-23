//! Começar sem pasta no UTSim LSS: o projeto em branco numa pasta temporária, o painel com a
//! trilha, o Salvar como que cria a pasta `.lss`, o salvamento automático desligado que
//! segura as operações até o Salvar, e a guarda que pergunta antes de fechar.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use serde_json::Value;
use utsim_lss_app::App;
use utsim_lss_app::app::documento::{Depois, JanelaSalvar};
use utsim_lss_app::app::{Nav, Screen};
use utsim_lss_app::session::Session;
use utsim_lss_core::domain::{Project, fields};
use utsim_lss_store::user::{AppDirs, Identity};
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

fn app_in(dir: &std::path::Path, sal: u8) -> App {
    let counter = Arc::new(AtomicU64::new(0));
    let mut app = App::with_clock(
        AppDirs::under(dir),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), sal))),
    );
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com.br"));
    app
}

fn escrever(app: &mut App, campo: &str, texto: &str) {
    let s = app.session_mut().expect("projeto aberto");
    let p = Project::entity_ref(s.project());
    s.set_field(p, campo, Value::from(texto), None).unwrap();
}

#[test]
fn o_projeto_em_branco_vira_uma_pasta_no_salvar_como() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let mut app = app_in(&dir.path().join("config"), 9);
    assert!(app.comecar_sem_pasta());
    assert_eq!(app.screen, Screen::Project);
    assert_eq!(app.nav, Nav::Overview);
    let temp = app.session().unwrap().folder().root().to_path_buf();
    assert!(app.session().unwrap().is_draft());
    assert!(!app.nao_salvo(), "nada escrito ainda");
    // O painel com a trilha e o "agora faça" desenha o projeto em branco.
    frames(&mut app, &ctx, 3);

    escrever(
        &mut app,
        fields::project::PROBLEM,
        "Refugo da tampa subiu para 4,8 %",
    );
    assert!(app.nao_salvo());
    // Fechar com trabalho não salvo pergunta: o projeto continua aberto.
    app.pedir_troca(Depois::Fechar);
    assert_eq!(app.screen, Screen::Project);
    app.doc.guarda.cancelar();

    let j = JanelaSalvar {
        nome: "Refugo da tampa".into(),
        pasta: dir.path().display().to_string(),
    };
    assert!(app.salvar_em(&j));
    let s = app.session().unwrap();
    assert!(!s.is_draft());
    assert!(!app.nao_salvo());
    let raiz = s.folder().root().to_path_buf();
    assert_eq!(raiz, dir.path().join("Refugo da tampa.lss"));
    assert!(!temp.exists(), "a pasta temporária foi embora");
    assert_eq!(
        s.folder().title(),
        "Refugo da tampa",
        "o nome vira o título"
    );
    frames(&mut app, &ctx, 2);

    // Reaberta, a pasta tem o que foi escrito antes de salvar.
    let me: Identity = app.session().unwrap().folder().me().clone();
    app.close_project();
    let s = Session::open(&raiz, &me, None, Box::new(SystemClock)).unwrap();
    let p = Project::read(s.project(), s.state().entity(s.project()).unwrap());
    assert_eq!(p.problem, "Refugo da tampa subiu para 4,8 %");
}

#[test]
fn com_o_automatico_desligado_as_mudancas_esperam_o_salvar() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = app_in(&dir.path().join("config"), 10);
    assert!(app.comecar_sem_pasta());
    let j = JanelaSalvar {
        nome: "Setup da prensa".into(),
        pasta: dir.path().display().to_string(),
    };
    assert!(app.salvar_em(&j));
    let raiz = app.session().unwrap().folder().root().to_path_buf();
    let arquivos = || walk(&raiz.join(utsim_lss_store::project::OPS));
    app.session_mut().unwrap().set_automatico(false).unwrap();
    let antes = arquivos();
    escrever(&mut app, fields::project::PROBLEM, "Setup leva 50 min");
    assert!(app.nao_salvo());
    assert_eq!(arquivos(), antes, "nada foi para a pasta");
    app.salvar_doc();
    assert!(!app.nao_salvo());
    assert!(arquivos() > antes);
    // Ligar de novo grava sozinho.
    app.session_mut().unwrap().set_automatico(true).unwrap();
    escrever(&mut app, fields::project::GOAL, "Setup em 20 min");
    assert!(!app.nao_salvo());
}

fn walk(d: &std::path::Path) -> usize {
    std::fs::read_dir(d)
        .map(|r| {
            r.flatten()
                .map(|e| {
                    let p = e.path();
                    if p.is_dir() { walk(&p) } else { 1 }
                })
                .sum()
        })
        .unwrap_or(0)
}
