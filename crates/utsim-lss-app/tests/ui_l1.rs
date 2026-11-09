//! Stage L1 screens drawn without a window (egui's headless context), over a project
//! filled through the journal: portfolio, charter versions, team and RACI, tollgates,
//! board with WIP limits and linked cards, risks, meetings, schedule with the Gantt,
//! savings, closing and lessons; the WIP rule through the session.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use serde_json::Value;
use utsim_lss_app::App;
use utsim_lss_app::app::{Nav, Screen};
use utsim_lss_app::session::Session;
use utsim_lss_core::domain::{Approach, Column, RoleKind, SavingCategory, fields};
use utsim_lss_core::meeting::{self, ItemKind, MeetingKind};
use utsim_lss_core::savings::{self, Month, Source};
use utsim_lss_core::tollgate::{self, TollgateStatus};
use utsim_lss_core::{
    Date, Edit, EntityRef, Id, Kind, board, calendar, charter, domain, lessons, portfolio, risk,
    roles, schedule,
};
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

fn app_in(dir: &std::path::Path) -> App {
    let counter = Arc::new(AtomicU64::new(0));
    App::with_clock(
        AppDirs::under(dir),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 7))),
    )
}

fn commit(s: &mut Session, e: Vec<Edit>) {
    s.apply(e).unwrap();
}

/// Fills a project with something on every L1 screen.
fn fill(s: &mut Session) -> Id {
    let p = s.project();
    let me = s.me();
    let today = s.today();
    for k in [
        RoleKind::GreenBelt,
        RoleKind::Sponsor,
        RoleKind::Controller,
        RoleKind::ProcessOwner,
    ] {
        let e = roles::add_role(s.state(), p, s.new_id(), me, k);
        commit(s, e);
    }
    let pr = domain::Project::entity_ref(p);
    commit(
        s,
        vec![
            Edit::new(pr, fields::project::GOAL, "refugo de 6 % para 2 %"),
            Edit::new(pr, fields::project::PROCESS, "Estamparia"),
            Edit::new(pr, schedule::fields::START, today.to_string()),
        ],
    );
    let e = charter::new_version(s.state(), p, s.new_id(), "primeira");
    commit(s, e);
    // Tollgate of the first phase, in review.
    let ph = domain::phases(s.state(), p)[0].id;
    let e = tollgate::prepare(s.state(), p, ph, s.ids());
    commit(s, e);
    let tg = tollgate::of_phase(s.state(), ph).unwrap();
    commit(
        s,
        vec![
            Edit::new(
                tollgate::Tollgate::entity_ref(tg.id),
                fields::tollgate::STATUS,
                domain::enum_value(TollgateStatus::Submitted),
            ),
            Edit::new(
                tollgate::Tollgate::entity_ref(tg.id),
                tollgate::fields::ROUND,
                1,
            ),
        ],
    );
    // Risk with a response card.
    let rid = s.new_id();
    commit(
        s,
        risk::new_risk(p, rid, "Fornecedor atrasa a matriz", 4, 5),
    );
    let rv = risk::risks(s.state(), p).remove(0);
    let e = risk::response_card(
        s.state(),
        p,
        &rv,
        s.new_id(),
        "Qualificar segundo fornecedor",
    );
    commit(s, e);
    // Meeting with minutes.
    let mid = s.new_id();
    commit(
        s,
        meeting::new_meeting(p, mid, "Semanal", today, MeetingKind::Team),
    );
    let e = meeting::add_item(
        s.state(),
        mid,
        s.new_id(),
        ItemKind::Action,
        "Treinar o turno B",
    );
    commit(s, e);
    let e = meeting::add_item(
        s.state(),
        mid,
        s.new_id(),
        ItemKind::Decision,
        "Seguir com o piloto",
    );
    commit(s, e);
    // Schedule: two tasks, the phase milestones, a holiday, the baseline.
    let (a, b) = (s.new_id(), s.new_id());
    let e = schedule::new_task(s.state(), p, a, "Coletar dados", 5.0, false);
    commit(s, e);
    let e = schedule::new_task(s.state(), p, b, "Analisar", 3.0, false);
    commit(s, e);
    commit(
        s,
        vec![schedule::set_predecessors(
            b,
            &[schedule::Pred { task: a, lag: 1.0 }],
        )],
    );
    let e = schedule::phase_milestones(s.state(), p, s.ids());
    commit(s, e);
    let h = s.new_id();
    commit(
        s,
        calendar::add_holiday(p, h, today.add_days(3), "Feriado local"),
    );
    let e = schedule::freeze_baseline(s.state(), p, today);
    commit(s, e);
    // Saving with a monthly entry.
    let g = s.new_id();
    commit(
        s,
        savings::new_saving(
            p,
            g,
            "Menos refugo",
            SavingCategory::Hard,
            120_000.0,
            Source::Belt,
        ),
    );
    let e =
        savings::transition(s.state(), p, g, domain::SavingStatus::Forecast, me, today).unwrap();
    commit(s, e);
    let e = savings::record_month(s.state(), g, Month::of(today), 9_000.0, s.new_id());
    commit(s, e);
    // Lesson and a card with an effectiveness check, WIP limit on "fazendo".
    let l = s.new_id();
    commit(
        s,
        lessons::new_lesson(p, l, "Ouvir o turno antes do piloto"),
    );
    let card = s
        .add_card("Padronizar o setup", Column::Todo)
        .unwrap()
        .unwrap();
    commit(
        s,
        vec![
            Edit::new(
                EntityRef::new(Kind::Action, card),
                board::fields::CHECK_CRITERION,
                "setup < 20 min por 4 semanas",
            ),
            board::set_wip_limit(p, Column::Doing, Some(1)),
        ],
    );
    card
}

#[test]
fn every_l1_screen_draws_over_a_filled_project() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let mut app = app_in(&dir.path().join("config"));
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com.br"));
    let shared = dir.path().join("Drive");
    std::fs::create_dir_all(&shared).unwrap();
    assert!(app.create_project(&shared, "Refugo na prensa", Approach::Dmaic));
    let root = app.session().unwrap().folder().root().to_path_buf();
    let me = Identity {
        person: app.session().unwrap().me(),
        name: "Ana Lima".into(),
        email: "ana@empresa.com.br".into(),
    };
    app.close_project();

    // A portfolio next to the project, with ideas, criteria and an objective.
    let mut ps = Session::create_portfolio(
        &shared.join("Carteira"),
        ("Carteira Estamparia", "Planta 1", "Programa LSS"),
        &me,
        None,
        clock(3),
    )
    .unwrap();
    let c = ps.project();
    let e = portfolio::default_criteria(ps.state(), c, ps.ids());
    commit(&mut ps, e);
    let o = ps.new_id();
    commit(
        &mut ps,
        portfolio::add_objective(c, o, "Reduzir custo", 2.0),
    );
    let idea = ps.new_id();
    commit(
        &mut ps,
        portfolio::new_idea(c, idea, "Setup longo na prensa 3"),
    );
    commit(
        &mut ps,
        vec![
            Edit::new(
                portfolio::Idea::entity_ref(idea),
                portfolio::fields::IMPACT,
                4,
            ),
            Edit::new(
                portfolio::Idea::entity_ref(idea),
                portfolio::fields::EFFORT,
                2,
            ),
        ],
    );
    ps.close();

    // The project filled by Ana on this computer.
    let mut s = Session::open(&root, &me, None, clock(5)).unwrap();
    let card = fill(&mut s);
    s.close();

    assert!(app.open_project(&root));
    assert_eq!(app.screen, Screen::Project);
    for nav in [Nav::Portfolio, Nav::Conflicts]
        .into_iter()
        .chain(Nav::PROJECT)
    {
        app.nav = nav;
        frames(&mut app, &ctx, 3);
        assert_eq!(app.screen, Screen::Project, "{nav:?}");
        assert!(
            app.status().is_none_or(|(_, err)| !err),
            "{nav:?}: {:?}",
            app.status()
        );
        assert_eq!(Nav::from_key(nav.key()), Some(nav));
    }
    // The card panel with its L1 part.
    app.nav = Nav::Actions;
    app.select_card(card);
    frames(&mut app, &ctx, 2);
    assert!(
        app.status().is_none_or(|(_, err)| !err),
        "{:?}",
        app.status()
    );

    // An idea sent by the 8D of UTSim Qualidade, with an attachment.
    app.close_project();
    let anexo = shared.join("8D-0042.pdf");
    std::fs::write(&anexo, b"%PDF-1.4 8D").unwrap();
    let sent = utsim_lss_store::intake::submit_idea(
        &shared.join("Carteira.lss"),
        &me,
        &utsim_lss_store::SystemClock,
        &portfolio::Submission {
            title: "Rebarba no furo".into(),
            source: portfolio::SOURCE_8D.into(),
            reference: "8D-0042".into(),
            ..Default::default()
        },
        &[anexo],
    )
    .unwrap();

    // Opening the portfolio folder itself lands on the portfolio screen.
    assert!(app.open_project(&shared.join("Carteira.lss")));
    assert_eq!(app.nav, Nav::Portfolio);
    frames(&mut app, &ctx, 3);
    app.select_idea(sent);
    frames(&mut app, &ctx, 2);
    assert!(
        app.status().is_none_or(|(_, err)| !err),
        "{:?}",
        app.status()
    );
    assert!(app.session().unwrap().folder().is_portfolio());
}

#[test]
fn portfolio_screen_offers_to_create_the_portfolio() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let mut app = app_in(&dir.path().join("config"));
    app.submit_identity("Ana", "ana@x.com");
    assert!(app.create_project(dir.path(), "P", Approach::Kaizen));
    app.nav = Nav::Portfolio;
    frames(&mut app, &ctx, 2);
    assert!(
        app.status().is_none_or(|(_, err)| !err),
        "{:?}",
        app.status()
    );
}

#[test]
fn session_respects_wip_limits_and_effectiveness() {
    let dir = tempfile::tempdir().unwrap();
    let me = Identity {
        person: Id::v7(1_700_000_000_000, [1; 10]),
        name: "Ana".into(),
        email: "ana@x.com".into(),
    };
    let mut s = Session::create(
        &dir.path().join("P"),
        "P",
        Approach::Dmaic,
        &me,
        None,
        clock(1),
    )
    .unwrap();
    let p = s.project();
    commit(
        &mut s,
        vec![board::set_wip_limit(p, Column::Doing, Some(1))],
    );
    let a = s.add_card("A", Column::Todo).unwrap().unwrap();
    let b = s.add_card("B", Column::Todo).unwrap().unwrap();
    s.shift_card(a, 1).unwrap();
    assert_eq!(s.card_column(a), Some(Column::Doing));
    let err = s.move_card(b, Column::Doing, 0).unwrap_err().to_string();
    assert!(err.contains("limite"), "{err}");
    assert!(s.shift_card(b, 1).is_err());
    assert_eq!(s.card_column(b), Some(Column::Todo));
    // Effectiveness asked: "feito" refused until confirmed.
    commit(
        &mut s,
        vec![Edit::new(
            EntityRef::new(Kind::Action, a),
            board::fields::CHECK_CRITERION,
            "sem refugo por 4 semanas",
        )],
    );
    assert!(s.move_card(a, Column::Done, 0).is_err());
    commit(
        &mut s,
        vec![Edit::new(
            EntityRef::new(Kind::Action, a),
            board::fields::CHECKED,
            Value::Bool(true),
        )],
    );
    s.move_card(a, Column::Done, 0).unwrap();
    assert_eq!(s.card_column(a), Some(Column::Done));
    // Today is a real date of the local calendar.
    let today: Date = s.today();
    assert!(today.year >= 2024);
}
