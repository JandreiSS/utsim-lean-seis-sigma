//! Stage L6 screens drawn without a window (egui's headless context), over a program
//! filled through the journals: two units with their portfolios and projects, targets,
//! objectives, people with capacity, roles, an opportunity at the committee, a tollgate in
//! review with an opinion, an assignment, an absence, a competence and a coaching log.
//! Also: a belt sees the program with the MBB's views refused, and an action outside the
//! profile is refused with a clear message.

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use utsim_lss_app::App;
use utsim_lss_app::app::Screen;
use utsim_lss_app::app::screens::program::ProgramNav;
use utsim_lss_app::session::Session;
use utsim_lss_core::domain::{
    self, Approach, ProjectStatus, RoleKind, SavingCategory, enum_value, fields,
};
use utsim_lss_core::op::{EntityRef, Kind};
use utsim_lss_core::programa::{self, BeltLevel, Member, Metric, fields as pf};
use utsim_lss_core::savings::{self, Month, Source};
use utsim_lss_core::tollgate::{self, TollgateStatus};
use utsim_lss_core::tools::{Deliverable, Tool};
use utsim_lss_core::{
    Edit, Id, designacao, mentoria, perfis, portfolio, privacidade, revisao, roles,
};
use utsim_lss_store::program;
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

fn clock(salt: u8) -> Box<dyn Clock> {
    Box::new(Salted(Arc::new(AtomicU64::new(0)), salt))
}

fn frames(app: &mut App, ctx: &egui::Context, n: usize) {
    for _ in 0..n {
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| app.draw(ui));
        out.textures_delta.clear();
    }
}

fn app_in(dir: &Path, salt: u8) -> App {
    let counter = Arc::new(AtomicU64::new(0));
    App::with_clock(
        AppDirs::under(dir),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), salt))),
    )
}

fn commit(s: &mut Session, e: Vec<Edit>) {
    s.apply(e).unwrap();
}

fn no_error(app: &App, what: &str) {
    assert!(
        app.status().is_none_or(|(_, err)| !err),
        "{what}: {:?}",
        app.status()
    );
}

/// A portfolio and a project in `dir`; returns (portfolio id, screening idea, project id,
/// submitted tollgate).
fn unit_folders(dir: &Path, me: &Identity, belt: Id, salt: u8) -> (Id, Id, Id, Id) {
    std::fs::create_dir_all(dir).unwrap();
    let mut ps = Session::create_portfolio(
        &dir.join("Carteira"),
        ("Carteira", "Unidade", "Programa LSS"),
        me,
        None,
        clock(salt),
    )
    .unwrap();
    let c = ps.project();
    let e = portfolio::default_criteria(ps.state(), c, ps.ids());
    commit(&mut ps, e);
    let idea = ps.new_id();
    commit(
        &mut ps,
        portfolio::new_idea(c, idea, "Setup longo na prensa"),
    );
    commit(
        &mut ps,
        vec![Edit::new(
            portfolio::Idea::entity_ref(idea),
            portfolio::fields::APPROACH,
            enum_value(Approach::Dmaic),
        )],
    );
    let e = portfolio::move_idea(ps.state(), idea, ProjectStatus::Screening, "").unwrap();
    commit(&mut ps, e);
    ps.close();

    let mut s = Session::create(
        &dir.join("Refugo"),
        "Refugo",
        Approach::Dmaic,
        me,
        None,
        clock(salt + 1),
    )
    .unwrap();
    let p = s.project();
    let e = roles::add_role(s.state(), p, s.new_id(), belt, RoleKind::GreenBelt);
    commit(&mut s, e);
    let ph = domain::phases(s.state(), p)[0].id;
    let e = tollgate::prepare(s.state(), p, ph, s.ids());
    commit(&mut s, e);
    let tg = tollgate::of_phase(s.state(), ph).unwrap();
    let today = s.today();
    let r = tollgate::Tollgate::entity_ref(tg.id);
    commit(
        &mut s,
        vec![
            Edit::new(
                r,
                fields::tollgate::STATUS,
                enum_value(TollgateStatus::Submitted),
            ),
            Edit::new(r, tollgate::fields::ROUND, 1),
            Edit::new(
                r,
                tollgate::fields::SUBMITTED_ON,
                today.add_days(-10).to_string(),
            ),
            Edit::new(r, tollgate::fields::PLANNED, today.add_days(-3).to_string()),
        ],
    );
    let d = s.new_id();
    commit(
        &mut s,
        Deliverable::create(p, Some(ph), Tool::Sipoc, d, "SIPOC"),
    );
    let g = s.new_id();
    commit(
        &mut s,
        savings::new_saving(
            p,
            g,
            "Menos refugo",
            SavingCategory::Hard,
            100_000.0,
            Source::Belt,
        ),
    );
    let en = s.new_id();
    let e = savings::record_month(s.state(), g, Month::of(today), 8_000.0, en);
    commit(&mut s, e);
    commit(
        &mut s,
        vec![Edit::new(
            EntityRef::new(Kind::SavingEntry, en),
            savings::fields::ENTRY_VALIDATED,
            true,
        )],
    );
    s.close();
    (c, idea, p, tg.id)
}

#[test]
fn every_l6_screen_draws_over_a_filled_program() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let shared = dir.path().join("Drive");
    std::fs::create_dir_all(&shared).unwrap();

    // Bia, a belt, first opens nothing: her app knows her identity.
    let mut bia_app = app_in(&dir.path().join("config_bia"), 9);
    assert!(bia_app.submit_identity("Bia Reis", "bia@empresa.com"));

    // Ana creates the program from the start screen.
    let mut app = app_in(&dir.path().join("config"), 7);
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com"));
    assert!(app.create_program(&shared, "Programa LSS"));
    assert!(app.session().unwrap().is_program());
    let root = app.session().unwrap().folder().root().to_path_buf();
    let ana = Identity {
        person: app.session().unwrap().me(),
        name: "Ana Lima".into(),
        email: "ana@empresa.com".into(),
    };
    app.close_project();

    // Bia opens the program once: she registers herself (allowed for everyone).
    assert!(bia_app.open_project(&root));
    let bia = bia_app.session().unwrap().me();
    bia_app.close_project();

    // Two units, each with a portfolio and a project.
    let (c1, idea1, p1, tg1) = unit_folders(&shared.join("Planta 1"), &ana, bia, 3);
    let (c2, _idea2, _p2, _tg2) = unit_folders(&shared.join("Planta 2"), &ana, ana.person, 5);

    // The program, filled by the MBB.
    let mut s = Session::open_program(&root, &ana, clock(11)).unwrap();
    let pg = s.project();
    let today = s.today();
    let (u1, u2) = (s.new_id(), s.new_id());
    commit(&mut s, programa::add_unit(pg, u1, "Planta 1"));
    commit(&mut s, programa::add_unit(pg, u2, "Planta 2"));
    commit(
        &mut s,
        vec![
            Edit::new(
                programa::Unit::entity_ref(u1),
                pf::PORTFOLIO,
                c1.to_string(),
            ),
            Edit::new(
                programa::Unit::entity_ref(u2),
                pf::PORTFOLIO,
                c2.to_string(),
            ),
            Edit::new(programa::Unit::entity_ref(u1), pf::HEADCOUNT, 300),
        ],
    );
    let o = s.new_id();
    commit(&mut s, programa::add_objective(pg, o, "Reduzir custo", 3.0));
    commit(
        &mut s,
        vec![programa::deploy(o, u1, 1.0), programa::deploy(o, u2, 2.0)],
    );
    for (u, m, l, v) in [
        (u1, Metric::HardValidated, None, 200_000.0),
        (u1, Metric::ProjectsClosed, None, 3.0),
        (u2, Metric::BeltsCertified, Some(BeltLevel::Green), 2.0),
        (u1, Metric::Coverage, None, 3.0),
    ] {
        let e = programa::set_target(s.state(), pg, s.new_id(), u, today.year, m, l, v).unwrap();
        commit(&mut s, e);
    }
    for (who, level, unit, hours) in [
        (bia, BeltLevel::Green, u1, 8.0),
        (ana.person, BeltLevel::MasterBlack, u2, 20.0),
    ] {
        let r = Member::entity_ref(who);
        commit(
            &mut s,
            vec![
                Edit::new(r, pf::LEVEL, enum_value(level)),
                Edit::new(r, pf::UNIT, unit.to_string()),
                Edit::new(r, pf::HOURS_WEEK, hours),
                Edit::new(r, pf::NEEDS_CERTIFICATION, true),
                Edit::new(r, pf::CONSENT, true),
            ],
        );
    }
    let caio = s.new_id();
    commit(
        &mut s,
        vec![Edit::new(
            Member::entity_ref(caio),
            fields::person::NAME,
            "Caio Champion",
        )],
    );
    let e = perfis::grant(pg, s.new_id(), bia, perfis::Profile::Belt, None);
    commit(&mut s, e);
    let e = perfis::grant(pg, s.new_id(), caio, perfis::Profile::Champion, Some(u1));
    commit(&mut s, e);
    // Program scan: the opportunity of the screening idea, then the committee.
    let me = s.folder().me().clone();
    let scan = program::scan(&root, s.state(), pg, &me, None, 0);
    assert_eq!(scan.records().len(), 2);
    assert_eq!(scan.ideas.len(), 2);
    let i1 = scan.ideas.iter().find(|i| i.id == idea1).unwrap().clone();
    assert_eq!(i1.unit, Some(u1));
    let opp = s.new_id();
    let e = programa::send_to_committee(pg, opp, &i1, &[], s.me(), today).unwrap();
    commit(&mut s, e);
    let m = s.new_id();
    commit(&mut s, programa::new_committee(pg, m, "Comitê", today));
    // A review of the tollgate in the queue.
    let q = revisao::queue(&scan.records(), &[], today);
    assert_eq!(q.len(), 2);
    let item = q.iter().find(|i| i.tollgate == tg1).unwrap().clone();
    assert!(item.overdue());
    let mut input = revisao::ReviewInput::default();
    for c in revisao::Criterion::ALL {
        input
            .marks
            .insert(c, (revisao::Mark::Reservation, "amostra pequena".into()));
    }
    let e = revisao::record(pg, s.new_id(), &item, s.me(), today, &input).unwrap();
    commit(&mut s, e);
    // Assignment, absence, competence.
    let e = designacao::assign(
        pg,
        s.new_id(),
        &designacao::AssignInput {
            opportunity: Some(opp),
            idea: Some(idea1),
            person: bia,
            hours_week: 8.0,
            mentor: Some(ana.person),
            position: Some(0),
            reason: "precisa de projeto".into(),
            by: s.me(),
            date: today,
        },
    );
    commit(&mut s, e);
    let e = designacao::add_absence(
        s.new_id(),
        bia,
        today.add_days(3),
        today.add_days(9),
        "férias",
    );
    commit(&mut s, e);
    let e = mentoria::record_level(
        pg,
        s.new_id(),
        bia,
        "msa",
        mentoria::Level::Demonstrated,
        mentoria::Source::Mbb,
        "revisão do Medir",
        s.me(),
        today,
    );
    commit(&mut s, e);
    commit(&mut s, privacidade::consent(caio, false, "", today));
    assert!(s.unauthorized().is_empty());
    s.close();

    // Bia's coaching log, in its own folder.
    let log_path = program::coaching_path(&root, bia);
    let mut log =
        Session::open_coaching(&log_path, &ana, clock(13), bia, pg, vec![ana.person], true)
            .unwrap();
    let belt = Member {
        id: bia,
        level: Some(BeltLevel::Green),
        consent: true,
        ..Member::default()
    };
    let x = log.new_id();
    let e = mentoria::new_session(
        &belt,
        x,
        &mentoria::SessionInput {
            belt: bia,
            mentor: ana.person,
            date: today,
            minutes: 60.0,
            form: mentoria::Form::Gemba,
            project: Some(p1),
            phase: "Definir".into(),
            topics: "SIPOC com o turno".into(),
            tools: vec!["sipoc".into()],
        },
    )
    .unwrap();
    commit(&mut log, e);
    let e = mentoria::add_commitment(
        x,
        log.new_id(),
        "Revisar o SIPOC",
        Some(today.add_days(7)),
        mentoria::Target::Project,
    )
    .unwrap();
    commit(&mut log, e);
    log.close();

    // Every screen, as the MBB.
    assert!(app.open_project(&root));
    assert_eq!(app.screen, Screen::Project);
    app.select_in_program(Some(bia), Some(tg1), Some(idea1), Some(opp));
    for nav in ProgramNav::ALL {
        app.pnav = nav;
        frames(&mut app, &ctx, 3);
        assert_eq!(app.screen, Screen::Project, "{nav:?}");
        no_error(&app, &format!("{nav:?}"));
    }
    // People imported from HR's CSV on the Capacity screen.
    let csv = root.parent().unwrap().join("pessoas.csv");
    std::fs::write(
        &csv,
        "nome;email;nivel;horas_semana
Zeca Prado;zeca@empresa.com;GB;6
;sem@nome;;
",
    )
    .unwrap();
    app.request_people_import(&csv);
    app.pnav = ProgramNav::Capacity;
    frames(&mut app, &ctx, 1);
    let rep = app
        .people_report()
        .expect("relatório da importação")
        .clone();
    assert_eq!(rep.created, vec!["Zeca Prado".to_string()]);
    assert_eq!(rep.errors.len(), 1);
    assert!(
        utsim_lss_core::programa::members(app.session().unwrap().state())
            .iter()
            .any(|m| m.email == "zeca@empresa.com")
    );
    // Both dashboards exported to PDF and to the spreadsheet, into relatorios/.
    use utsim_lss_app::app::screens::program::dashboards::Export;
    for (nav, leadership) in [
        (ProgramNav::MbbDashboard, false),
        (ProgramNav::LeadershipDashboard, true),
    ] {
        app.pnav = nav;
        for how in [Export::Pdf, Export::Xlsx] {
            app.request_dashboard_export(leadership, how);
            frames(&mut app, &ctx, 1);
            no_error(&app, &format!("exportar {nav:?} {how:?}"));
            let f = app
                .last_dashboard_export()
                .expect("arquivo exportado")
                .to_path_buf();
            let bytes = std::fs::read(&f).unwrap();
            assert!(f.starts_with(root.join("relatorios")), "{f:?}");
            match how {
                Export::Pdf => {
                    assert!(bytes.starts_with(b"%PDF-1.4"));
                    assert!(String::from_utf8_lossy(&bytes).contains(" re f 0 g"));
                }
                Export::Xlsx => assert!(bytes.starts_with(b"PK")),
            }
        }
    }
    app.close_project();

    // Every screen, as the belt: the MBB's views show a refusal, never an error.
    assert!(bia_app.open_project(&root));
    bia_app.select_in_program(Some(bia), None, None, None);
    for nav in ProgramNav::ALL {
        bia_app.pnav = nav;
        frames(&mut bia_app, &ctx, 3);
        no_error(&bia_app, &format!("belt {nav:?}"));
    }
    bia_app.close_project();

    // An action outside the profile is refused with the reason, and nothing is written.
    let bia_id = Identity {
        person: bia,
        name: "Bia Reis".into(),
        email: "bia@empresa.com".into(),
    };
    let mut sb = Session::open_program(&root, &bia_id, clock(17)).unwrap();
    let before = sb.folder().journal().len();
    let e = programa::set_target(
        sb.state(),
        pg,
        sb.new_id(),
        u1,
        today.year,
        Metric::ProjectsClosed,
        None,
        99.0,
    )
    .unwrap();
    let err = sb.apply(e).unwrap_err().to_string();
    assert!(err.contains("reservada a: MBB"), "{err}");
    assert!(err.contains("belt"), "{err}");
    assert_eq!(sb.folder().journal().len(), before);
    assert!(sb.may(perfis::Action::ViewMbbDashboard, None).is_err());
    assert!(sb.may(perfis::Action::RecordCoaching, None).is_ok());
    // Her own consent she may record.
    let e = privacidade::consent(bia, true, "formação", today);
    sb.apply(e).unwrap();
    // Her coaching log opens for her; someone else's does not.
    let mbbs = perfis::mbbs(sb.state(), sb.founder().unwrap());
    assert!(
        Session::open_coaching(&log_path, &bia_id, clock(19), bia, pg, mbbs.clone(), false).is_ok()
    );
    let other = program::coaching_path(&root, ana.person);
    assert!(
        Session::open_coaching(&other, &bia_id, clock(21), ana.person, pg, mbbs, true).is_err()
    );
    sb.close();
    // Opening a coaching log folder directly is refused with a hint.
    let mut app3 = app_in(&dir.path().join("config3"), 23);
    app3.submit_identity("Ana Lima", "ana@empresa.com");
    assert!(!app3.open_project(&log_path));
}
