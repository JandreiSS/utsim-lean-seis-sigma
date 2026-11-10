//! The team's signatures in the app without a window (P7): the identity creates the key, a
//! new project is signed from its first commit, the team screen draws the check, an edited
//! operation file shows up on reopening, and an approval sealed by the person who decided
//! reads back as valid.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use utsim_lss_app::App;
use utsim_lss_app::app::Nav;
use utsim_lss_core::domain::Approach;
use utsim_lss_core::{Edit, EntityRef, Id, Kind};
use utsim_lss_store::assinatura::{self, Approval, SealStatus};
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

fn draw(app: &mut App) {
    let ctx = egui::Context::default();
    for _ in 0..3 {
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| app.draw(ui));
        out.textures_delta.clear();
    }
}

fn files_with(dir: &std::path::Path, needle: &str) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() {
            out.extend(files_with(&p, needle));
        } else if std::fs::read_to_string(&p).is_ok_and(|t| t.contains(needle)) {
            out.push(p);
        }
    }
    out
}

#[test]
fn a_signed_project_is_checked_and_approvals_are_sealed() {
    let dir = tempfile::tempdir().unwrap();
    let counter = Arc::new(AtomicU64::new(0));
    let cfg = dir.path().join("cfg");
    let mut app = App::with_clock(
        AppDirs::under(&cfg),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 61))),
    );
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com"));
    assert!(
        cfg.join(assinatura::KEY_FILE).is_file(),
        "the key is created"
    );
    let work = dir.path().join("Drive");
    std::fs::create_dir_all(&work).unwrap();
    assert!(app.create_project(&work, "Linha 2", Approach::Dmaic));
    let root = {
        let s = app.session_mut().unwrap();
        assert!(s.folder().signer().is_some());
        let e = Edit::new(
            EntityRef::new(Kind::Project, s.project()),
            "descricao",
            "Reduzir o refugo da linha 2",
        );
        s.apply(vec![e]).unwrap();
        let (r, _) = s.signatures();
        assert!(r.findings.is_empty(), "{:?}", r.findings);
        assert_eq!(r.signed, r.ops, "every operation is signed");
        s.folder().root().to_path_buf()
    };
    // The portfolio screen with "minhas ações" (no portfolio folder yet: the form).
    app.nav = Nav::Portfolio;
    draw(&mut app);
    app.nav = Nav::Team;
    draw(&mut app);
    assert!(
        app.status().is_none_or(|(_, err)| !err),
        "{:?}",
        app.status()
    );

    // An approval of Ana, sealed by her and read back.
    let a = {
        let s = app.session_mut().unwrap();
        let a = Approval {
            project: s.project(),
            kind: assinatura::APPROVAL_TOLLGATE.into(),
            record: Id::v7(1_793_000_000_000, [3; 10]),
            person: s.me(),
            decision: "aprovar".into(),
            content: "abc".into(),
            items: Vec::new(),
        };
        let (r, seals) = s.signatures().clone();
        assert_eq!(seals.status(&r, &a), SealStatus::None);
        s.sign(&a).unwrap();
        let (r, seals) = s.signatures().clone();
        assert!(matches!(seals.status(&r, &a), SealStatus::Valid { .. }));
        a
    };

    // Someone edits an operation file by hand; reopening the project warns.
    let edited = files_with(&root.join("ops"), "Reduzir o refugo");
    assert_eq!(edited.len(), 1);
    let t = std::fs::read_to_string(&edited[0])
        .unwrap()
        .replace("Reduzir o refugo", "Aumentar o refugo");
    std::fs::write(&edited[0], t).unwrap();
    assert!(app.open_project(&root));
    assert!(
        app.status().is_some_and(|(_, err)| *err),
        "a warning on opening: {:?}",
        app.status()
    );
    let s = app.session_mut().unwrap();
    let (r, seals) = s.signatures().clone();
    assert_eq!(r.severe(), 1, "{:?}", r.findings);
    // The seal is over its own body, so it still holds.
    assert!(matches!(seals.status(&r, &a), SealStatus::Valid { .. }));
    draw(&mut app);
}

#[test]
fn the_founder_signs_the_role_list_of_a_program() {
    use utsim_lss_app::app::screens::program::ProgramNav;
    let dir = tempfile::tempdir().unwrap();
    let counter = Arc::new(AtomicU64::new(0));
    let mut app = App::with_clock(
        AppDirs::under(&dir.path().join("cfg")),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 62))),
    );
    assert!(app.submit_identity("Mara Souza", "mara@empresa.com"));
    let work = dir.path().join("Drive");
    std::fs::create_dir_all(&work).unwrap();
    assert!(app.create_program(&work, "Programa 2027"));
    {
        let s = app.session_mut().unwrap();
        let program = s.project();
        let founder = s.folder().manifest().created_by;
        let grants = utsim_lss_core::perfis::grants(s.state());
        let a = assinatura::roles_approval(program, &grants, s.me());
        s.sign(&a).unwrap();
        let (r, seals) = s.signatures().clone();
        let c = assinatura::roles_check(&seals, &r, program, founder, &grants);
        assert!(c.current(), "{c:?}");
    }
    for nav in [ProgramNav::Profiles, ProgramNav::Certification] {
        app.pnav = nav;
        draw(&mut app);
        assert!(
            app.status().is_none_or(|(_, err)| !*err),
            "{:?}",
            app.status()
        );
    }
}
