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
