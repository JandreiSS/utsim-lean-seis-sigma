//! The screens drawn without a window (egui's headless context): first run, start screen,
//! every project screen, and the action board picking up a card written by someone else.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use utsim_lss_app::App;
use utsim_lss_app::app::{Nav, Screen, sanitize_folder_name};
use utsim_lss_app::session::Session;
use utsim_lss_core::domain::{Approach, Column};
use utsim_lss_core::{Id, kanban};
use utsim_lss_store::user::{AppDirs, Identity, UserConfig};
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
        // No renderer here: drop the texture uploads on purpose.
        out.textures_delta.clear();
    }
}

fn app_in(dir: &std::path::Path) -> App {
    let counter = Arc::new(AtomicU64::new(0));
    App::with_clock(
        AppDirs::under(dir),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 7))),
    )
}

#[test]
fn first_run_then_project_screens() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let mut app = app_in(&dir.path().join("config"));
    assert_eq!(app.screen, Screen::Identity);
    frames(&mut app, &ctx, 2);
    assert!(!app.submit_identity("A", "x"));
    assert_eq!(app.screen, Screen::Identity);
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com.br"));
    assert_eq!(app.screen, Screen::Start);
    // The identity is in the user's configuration, not in the project.
    let config = UserConfig::load(&AppDirs::under(&dir.path().join("config")).config_file);
    assert_eq!(config.identity.as_ref().unwrap().name, "Ana Lima");
    frames(&mut app, &ctx, 2);

    let shared = dir.path().join("Drive");
    std::fs::create_dir_all(&shared).unwrap();
    assert!(app.create_project(&shared, "Refugo: linha 2?", Approach::Dmaic));
    assert_eq!(app.screen, Screen::Project);
    let root = app.session().unwrap().folder().root().to_path_buf();
    assert_eq!(root, shared.join("Refugo- linha 2-.lss"));
    assert!(!root.join("config.json").exists());

    for nav in [Nav::Portfolio, Nav::Conflicts]
        .into_iter()
        .chain(Nav::PROJECT)
    {
        app.nav = nav;
        frames(&mut app, &ctx, 2);
        assert_eq!(app.screen, Screen::Project, "{nav:?}");
        assert!(
            app.status().is_none_or(|(_, err)| !err),
            "{nav:?}: {:?}",
            app.status()
        );
        assert_eq!(Nav::from_key(nav.key()), Some(nav));
    }
    // Recent list remembers the project; closing goes back to the start screen.
    app.close_project();
    assert_eq!(app.screen, Screen::Start);
    let config = UserConfig::load(&AppDirs::under(&dir.path().join("config")).config_file);
    assert_eq!(config.recent, vec![root.clone()]);
    frames(&mut app, &ctx, 1);
    assert!(app.open_project(&root));
    assert!(!app.open_project(&dir.path().join("nada.lss")));
    assert!(app.status().unwrap().1, "error shown");
}

#[test]
fn board_shows_cards_written_by_someone_else() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let mut app = app_in(&dir.path().join("config"));
    app.submit_identity("Ana", "ana@x.com");
    assert!(app.create_project(dir.path(), "P", Approach::Kaizen));
    let root = app.session().unwrap().folder().root().to_path_buf();
    app.nav = Nav::Actions;
    frames(&mut app, &ctx, 1);

    // Bia, on another computer, adds a card into the same (synced) folder.
    let bia = Identity {
        person: Id::v7(1_700_000_000_000, [2; 10]),
        name: "Bia".into(),
        email: "bia@x.com".into(),
    };
    let mut other = Session::open(
        &root,
        &bia,
        None,
        Box::new(Salted(Arc::new(AtomicU64::new(0)), 9)),
    )
    .unwrap();
    let card = other
        .add_card("Padronizar o setup", Column::Todo)
        .unwrap()
        .unwrap();
    other.tick("acoes", None).unwrap();

    // The app picks it up on its periodic refresh.
    let mut seen = false;
    for _ in 0..40 {
        frames(&mut app, &ctx, 1);
        let s = app.session().unwrap();
        if s.state().entity(card).is_some() {
            seen = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    assert!(seen, "card from the other computer arrived");
    let s = app.session().unwrap();
    let board = kanban::board(s.state(), s.project());
    assert_eq!(board[&Column::Todo][0].title, "Padronizar o setup");
    assert!(
        s.others().iter().any(|p| p.name == "Bia"),
        "Bia's presence is shown"
    );
    other.close();
}

#[test]
fn folder_names_are_safe() {
    assert_eq!(
        sanitize_folder_name("  Linha 2 / turno B  "),
        "Linha 2 - turno B"
    );
    assert_eq!(sanitize_folder_name("..."), "projeto");
    assert_eq!(sanitize_folder_name("a:b*c?"), "a-b-c-");
}

#[test]
fn every_tool_screen_draws() {
    use serde_json::Value;
    use utsim_lss_core::tools::{
        self, Tool, cause_effect as ce, fmea, ishikawa, pareto, plans, sipoc, voc, whys,
    };
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let mut app = app_in(&dir.path().join("config"));
    app.submit_identity("Ana", "ana@x.com");
    assert!(app.create_project(dir.path(), "Refugo", Approach::Dmaic));
    app.nav = Nav::Tools;
    frames(&mut app, &ctx, 2);

    // Bia, on another computer, fills one deliverable of each tool in the same folder.
    let root = app.session().unwrap().folder().root().to_path_buf();
    let bia = Identity {
        person: Id::v7(1_700_000_000_000, [2; 10]),
        name: "Bia".into(),
        email: "bia@x.com".into(),
    };
    let mut o = Session::open(
        &root,
        &bia,
        None,
        Box::new(Salted(Arc::new(AtomicU64::new(0)), 9)),
    )
    .unwrap();
    let t = Value::from;
    let n = |x: i64| Value::from(x);
    let mut ids = Vec::new();
    for tool in Tool::ALL {
        let d = o.add_deliverable(tool, "x").unwrap();
        ids.push(d);
        match tool {
            Tool::Voc => {
                let need = o
                    .add_item(d, voc::NEED, None, vec![(voc::fields::TEXT, t("Prazo"))])
                    .unwrap();
                o.add_item(
                    d,
                    voc::STATEMENT,
                    Some(need),
                    vec![(voc::fields::TEXT, t("Atrasa"))],
                )
                .unwrap();
                let dr = o.add_item(d, voc::DRIVER, Some(need), vec![]).unwrap();
                o.add_item(d, voc::CTQ, Some(dr), vec![(voc::fields::USL, n(48))])
                    .unwrap();
            }
            Tool::Vsm => {
                use utsim_lss_core::tools::vsm;
                for (name, c, inv) in [("Corte", 1.5, 0.0), ("Solda", 2.5, 40.0)] {
                    o.add_item(
                        d,
                        vsm::PROCESS_ITEM,
                        None,
                        vec![
                            (vsm::fields::NAME, t(name)),
                            (vsm::fields::CYCLE, Value::from(c)),
                            (vsm::fields::INVENTORY, Value::from(inv)),
                            (vsm::fields::VALUE_ADDED, Value::from(true)),
                        ],
                    )
                    .unwrap();
                }
            }
            Tool::Sipoc => {
                o.add_item(d, sipoc::STEP, None, vec![(sipoc::TEXT, t("Cortar"))])
                    .unwrap();
            }
            Tool::Pareto => {
                for (name, c) in [("Rebarba", 30), ("Risco", 5)] {
                    o.add_item(
                        d,
                        pareto::ITEM,
                        None,
                        vec![
                            (pareto::fields::NAME, t(name)),
                            (pareto::fields::COUNT, n(c)),
                        ],
                    )
                    .unwrap();
                }
            }
            Tool::Ishikawa => {
                use ishikawa::fields as f;
                let c = o
                    .add_item(
                        d,
                        ishikawa::ITEM,
                        None,
                        vec![
                            (f::TEXT, t("Ferramenta gasta")),
                            (f::CATEGORY, t("maquina")),
                        ],
                    )
                    .unwrap();
                o.add_item(d, ishikawa::ITEM, Some(c), vec![(f::TEXT, t("Sem troca"))])
                    .unwrap();
                o.add_item(
                    d,
                    ishikawa::ITEM,
                    None,
                    vec![(f::TEXT, t("Calor")), (f::CATEGORY, t("Turno da noite"))],
                )
                .unwrap();
            }
            Tool::FiveWhys => {
                let c = o
                    .add_item(
                        d,
                        whys::CHAIN,
                        None,
                        vec![(whys::fields::TEXT, t("Rebarba"))],
                    )
                    .unwrap();
                o.add_item(
                    d,
                    whys::WHY,
                    Some(c),
                    vec![(whys::fields::TEXT, t("Gasta"))],
                )
                .unwrap();
            }
            Tool::CauseEffect => {
                let y = o
                    .add_item(
                        d,
                        ce::OUTPUT,
                        None,
                        vec![
                            (ce::fields::NAME, t("Diâmetro")),
                            (ce::fields::WEIGHT, n(9)),
                        ],
                    )
                    .unwrap();
                let cell = ce::score_field(y);
                o.add_item(
                    d,
                    ce::INPUT,
                    None,
                    vec![(ce::fields::NAME, t("Temperatura")), (cell.as_str(), n(9))],
                )
                .unwrap();
            }
            Tool::Fmea => {
                use fmea::fields as f;
                let r = o
                    .add_item(
                        d,
                        fmea::ITEM,
                        None,
                        vec![
                            (f::MODE, t("Furo deslocado")),
                            (f::S, n(9)),
                            (f::O, n(5)),
                            (f::D, n(4)),
                            (f::O2, n(2)),
                            (f::ACTION, t("Pino de referência")),
                        ],
                    )
                    .unwrap();
                o.fmea_card(r).unwrap().unwrap();
            }
            Tool::DataCollection => {
                o.add_item(
                    d,
                    plans::COLLECTION_ITEM,
                    None,
                    vec![(plans::fields::NAME, t("Diâmetro"))],
                )
                .unwrap();
            }
            Tool::ControlPlan => {
                o.add_item(
                    d,
                    plans::CONTROL_ITEM,
                    None,
                    vec![(plans::fields::CHARACTERISTIC, t("Diâmetro"))],
                )
                .unwrap();
            }
            Tool::ResponsePlan => {
                o.add_item(
                    d,
                    plans::RESPONSE_ITEM,
                    None,
                    vec![(plans::fields::SIGNAL, t("Ponto fora"))],
                )
                .unwrap();
            }
            Tool::A3 => {}
        }
    }
    o.close();

    // The app picks them up on its periodic refresh.
    let mut arrived = false;
    for _ in 0..40 {
        frames(&mut app, &ctx, 1);
        let s = app.session().unwrap();
        if tools::deliverables(s.state(), s.project()).len() == Tool::ALL.len() {
            arrived = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    assert!(arrived, "the deliverables arrived");
    for (tool, d) in Tool::ALL.iter().zip(&ids) {
        app.selected_deliverable = Some(*d);
        frames(&mut app, &ctx, 3);
        assert_eq!(app.selected_deliverable, Some(*d), "{tool:?}");
        assert!(
            app.status().is_none_or(|(_, err)| !err),
            "{tool:?}: {:?}",
            app.status()
        );
    }
    // The A3 sees the other deliverables; the FMEA card is on the board.
    let s = app.session().unwrap();
    let page = tools::a3::compose(s.state(), s.project(), ids.last().copied());
    assert_eq!(page.fmea_high.len(), 1);
    assert_eq!(page.critical_gaps, 1);
    assert_eq!(page.pareto_vital.len(), 1);
    assert_eq!(
        kanban::board(s.state(), s.project())[&Column::Todo].len(),
        1
    );
    app.nav = Nav::Actions;
    frames(&mut app, &ctx, 1);
}

#[test]
fn a_project_from_a_line_template_records_it() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = app_in(&dir.path().join("config"));
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com.br"));
    let model = utsim_lss_core::modelos_linha::find("kaizen_administrativo").unwrap();
    assert!(app.create_line_project(dir.path(), "Reembolso", model));
    let s = app.session().unwrap();
    let project = s.state().entity(s.project()).unwrap();
    assert_eq!(
        project.text(utsim_lss_core::domain::fields::project::MODEL),
        "kaizen_administrativo"
    );
    let p = utsim_lss_core::domain::Project::read(s.project(), project);
    assert_eq!(p.approach, Approach::Kaizen);
    let ctx = egui::Context::default();
    frames(&mut app, &ctx, 2);
}
