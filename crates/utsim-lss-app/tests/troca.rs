//! The opportunity of UTSim Ambiental (`nova_oportunidade`, `docs/troca_ambiental.md` of
//! the Ambiental): the request written as the Ambiental writes it becomes an idea in the
//! portfolio's bank of ideas, with the waste and energy data and the scenarios file
//! attached; a changed file, another action or a request without summary are refused.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use utsim_lss_app::session::Session;
use utsim_lss_app::troca;
use utsim_lss_app::{App, app::Nav};
use utsim_lss_core::{Kind, portfolio};
use utsim_lss_store::user::{AppDirs, Identity};
use utsim_lss_store::{Clock, SystemClock};
use utsim_troca::{Arquivo, Pedido};

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

fn app_in(dir: &std::path::Path) -> App {
    let counter = Arc::new(AtomicU64::new(0));
    App::with_clock(
        AppDirs::under(dir),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 7))),
    )
}

fn frames(app: &mut App, ctx: &egui::Context, n: usize) {
    for _ in 0..n {
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| app.draw(ui));
        out.textures_delta.clear();
    }
}

/// The summary as UTSim Ambiental 0.3.0 writes it (`troca::Oportunidade` there).
const RESUMO: &str = r#"{
  "cenario": "Forno com isolamento",
  "alavancas": ["Forno 1: isolamento térmico (-15 % de energia)", "Desligar a esteira parada"],
  "diferencas": [
    ["ambiental.co2e_kg", -12.34, -15.0, -9.6],
    ["ambiental.energia_kwh", -150.0, -180.5, -119.5],
    ["ambiental.custo", -310.0, -380.0, -240.0]
  ],
  "investimento": 25000.0,
  "vida_anos": 10.0,
  "linha_vsm": [
    { "no": "Forno 1", "etapa": "Assar", "kwh_por_peca": 0.82, "co2e_por_peca": 0.067 },
    { "no": "Esteira", "etapa": null, "kwh_por_peca": 0.05, "co2e_por_peca": null }
  ],
  "testes": [
    { "metrica": "ambiental.co2e_kg", "diferenca": -12.34, "de": -15.0, "ate": -9.6,
      "p_valor": 0.0012, "n": 10, "origem": "UTSim Ambiental" }
  ],
  "campo_novo": "um Ambiental mais novo pode mandar mais"
}"#;

/// Writes the scenarios file and the request in `troca/` next to the model, as the
/// Ambiental does; returns the request's path and the scenarios file.
fn pedido_do_ambiental(
    dir: &std::path::Path,
    acao: &str,
    resumo: Option<&str>,
) -> (std::path::PathBuf, std::path::PathBuf) {
    std::fs::create_dir_all(dir).unwrap();
    let modelo = dir.join("linha.ron");
    std::fs::write(&modelo, "()").unwrap();
    let cenarios = dir.join("linha.cenarios_ambientais.json");
    std::fs::write(&cenarios, br#"{"cenarios":[]}"#).unwrap();
    let mut p = Pedido::novo("ambiental 0.3.0", acao);
    p.contexto.modelo = Some(modelo);
    p.entrada.arquivos = vec![Arquivo::de(&cenarios).unwrap()];
    p.entrada.conteudo = resumo.map(str::to_string);
    let caminho = p.gravar(&dir.join("troca")).unwrap();
    (caminho, cenarios)
}

#[test]
fn the_opportunity_of_ambiental_becomes_an_idea_in_the_portfolio() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let mut app = app_in(&dir.path().join("config"));
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com.br"));
    let shared = dir.path().join("Drive");
    std::fs::create_dir_all(&shared).unwrap();
    // A project opened once gives the person's id; then the portfolio, next to it.
    assert!(app.create_project(&shared, "Projeto", utsim_lss_core::domain::Approach::Dmaic));
    let me = Identity {
        person: app.session().unwrap().me(),
        name: "Ana Lima".into(),
        email: "ana@empresa.com.br".into(),
    };
    app.close_project();
    let mut ps = Session::create_portfolio(
        &shared.join("Carteira"),
        ("Carteira Padaria", "Planta 1", "Programa LSS"),
        &me,
        None,
        Box::new(Salted(Arc::new(AtomicU64::new(0)), 3)),
    )
    .unwrap();
    let carteira = ps.folder().root().to_path_buf();
    ps.close();

    let (caminho, cenarios) =
        pedido_do_ambiental(dir.path(), troca::NOVA_OPORTUNIDADE, Some(RESUMO));

    // The summary read with every field, and the text of the idea.
    let lido = troca::ler(&caminho).unwrap();
    assert_eq!(lido.oportunidade.cenario, "Forno com isolamento");
    assert_eq!(lido.oportunidade.diferencas.len(), 3);
    assert_eq!(lido.oportunidade.linha_vsm[1].co2e_por_peca, None);
    assert_eq!(lido.modelo(), "linha.ron");
    let ideia = lido.submissao();
    assert_eq!(ideia.source, troca::ORIGEM_AMBIENTAL);
    assert_eq!(ideia.reference, "Forno com isolamento");
    assert!(
        ideia.title.contains("Forno com isolamento"),
        "{}",
        ideia.title
    );
    assert!(
        ideia.problem.contains("isolamento térmico"),
        "{}",
        ideia.problem
    );
    // No conversion of the environmental gain into money (A25).
    assert_eq!(ideia.rough_gain, 0.0);
    for t in [
        "CO₂e (kg): -12,34 (de -15 a -9,6)",
        "Energia (kWh): -150 (de -180,5 a -119,5)",
        "Custo: -310",
        "Investimento: 25000",
        "Vida útil: 10 anos",
        "Forno 1 (Assar): 0,82 kWh e 0,067 kg CO₂e",
        "Esteira: 0,05 kWh e – kg CO₂e",
        "p = 0,0012 (UTSim Ambiental)",
    ] {
        assert!(ideia.data.contains(t), "falta {t:?} em\n{}", ideia.data);
    }

    // The app opened with the request: the window waits for the portfolio.
    assert!(app.receber_pedido(&caminho));
    assert!(app.pedido().is_some());
    frames(&mut app, &ctx, 3);
    // With `UTSIM_CAPTURAS=<pasta>` the picture of the request is saved for review.
    if let Some(d) = std::env::var_os("UTSIM_CAPTURAS") {
        let d = std::path::PathBuf::from(d);
        std::fs::create_dir_all(&d).unwrap();
        let mut cap = utsim_ui_tema::captura::Captura::new(
            egui::Context::default(),
            egui::vec2(1280.0, 860.0),
        );
        let img = cap.imagem(30, |ui| app.draw(ui));
        img.salvar_png(&d.join("seis_sigma_pedido_ambiental.png"))
            .unwrap();
    }
    assert!(app.enviar_pedido(&carteira));
    assert!(app.pedido().is_none());
    assert_eq!(app.nav, Nav::Portfolio);
    assert!(
        app.status()
            .is_some_and(|(m, err)| !err && m.contains("Forno")),
        "{:?}",
        app.status()
    );
    frames(&mut app, &ctx, 3);

    // The idea in the bank of ideas, with its origin, data and the scenarios attached.
    let s = app.session().unwrap();
    assert!(s.folder().is_portfolio());
    let ideas: Vec<_> = s
        .state()
        .of_kind(Kind::Idea)
        .filter(|(_, e)| !e.is_deleted())
        .collect();
    assert_eq!(ideas.len(), 1);
    let (id, e) = ideas[0];
    assert_eq!(
        e.text(portfolio::fields::SOURCE_APP),
        troca::ORIGEM_AMBIENTAL
    );
    assert!(e.text(portfolio::fields::DATA).contains("kWh"));
    let anexos = portfolio::attachments(s.state(), id);
    assert_eq!(anexos.len(), 1);
    assert_eq!(anexos[0].name, "linha.cenarios_ambientais.json");
    let copia = utsim_lss_store::intake::attachment_path(&carteira, &anexos[0].file);
    assert_eq!(
        std::fs::read(copia).unwrap(),
        std::fs::read(&cenarios).unwrap()
    );

    // The scenarios file changed after the request: refused, nothing created.
    let (caminho2, cenarios2) = pedido_do_ambiental(
        &dir.path().join("outro"),
        troca::NOVA_OPORTUNIDADE,
        Some(RESUMO),
    );
    let lido2 = troca::ler(&caminho2).unwrap();
    std::fs::write(&cenarios2, b"mudou").unwrap();
    assert!(lido2.enviar(&carteira, &me, &SystemClock).is_err());
    assert!(troca::ler(&caminho2).is_err());
    assert!(!app.receber_pedido(&caminho2));
    assert!(app.status().is_some_and(|(_, err)| *err));
}

#[test]
fn another_action_or_no_summary_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let (c1, _) = pedido_do_ambiental(&dir.path().join("a"), "analisar", Some(RESUMO));
    let e = troca::ler(&c1).unwrap_err();
    assert!(e.contains("analisar"), "{e}");
    let (c2, _) = pedido_do_ambiental(&dir.path().join("b"), troca::NOVA_OPORTUNIDADE, None);
    assert!(troca::ler(&c2).is_err());
    let (c3, _) = pedido_do_ambiental(
        &dir.path().join("c"),
        troca::NOVA_OPORTUNIDADE,
        Some("não é JSON"),
    );
    assert!(troca::ler(&c3).is_err());
}

#[test]
fn numbers_keep_four_significant_digits() {
    assert_eq!(troca::num(0.0), "0");
    assert_eq!(troca::num(-12.3456), "-12,35");
    assert_eq!(troca::num(25000.0), "25000");
    assert_eq!(troca::num(0.00123456), "0,001235");
    assert_eq!(troca::num(f64::NAN), "–");
    assert_eq!(troca::metrica("ambiental.agua_m3"), "Água (m³)");
    assert_eq!(troca::metrica("outra"), "outra");
}
