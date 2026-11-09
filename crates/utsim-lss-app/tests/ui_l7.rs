//! Stage L7 screens drawn without a window, over a program with a green belt wave filled
//! through the journals: curriculum, instructor, class, nominations approved by the
//! manager and selected, sessions with attendance, the question bank in its restricted
//! folder, an exam drawn and published in the class folder, the trainee's answers handed
//! in from the trainee's own app, the grading on the instructor's machine and an MBB
//! opinion. Every L7 screen is drawn as the MBB, as the instructor and as the trainee
//! (exam mode included), and the trainee cannot write in the bank.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use serde_json::Value;
use utsim_lss_app::App;
use utsim_lss_app::app::screens::program::ProgramNav;
use utsim_lss_app::session::Session;
use utsim_lss_core::domain::{enum_value, fields};
use utsim_lss_core::formacao::exams::{self, ExamInput, Mode, QuestionInput, QuestionKind};
use utsim_lss_core::formacao::{self, Attendance, ClassInput, NominationInput, curriculum};
use utsim_lss_core::perfis::{self, Profile, TrainingWriters};
use utsim_lss_core::programa::{BeltLevel, Member, fields as pf};
use utsim_lss_core::{Edit, certificacao};
use utsim_lss_store::program::{self, TrainingFolder};
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

fn identity(app: &App, name: &str, email: &str) -> Identity {
    Identity {
        person: app.session().unwrap().me(),
        name: name.into(),
        email: email.into(),
    }
}

const L7: [ProgramNav; 5] = [
    ProgramNav::Training,
    ProgramNav::Classroom,
    ProgramNav::Exams,
    ProgramNav::Certification,
    ProgramNav::MyTraining,
];

#[test]
fn every_l7_screen_draws_over_a_green_belt_wave() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let shared = dir.path().join("Drive");
    std::fs::create_dir_all(&shared).unwrap();

    let mut app = app_in(&dir.path().join("cfg_ana"), 7);
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com"));
    assert!(app.create_program(&shared, "Programa LSS"));
    let root = app.session().unwrap().folder().root().to_path_buf();
    let ana = identity(&app, "Ana Lima", "ana@empresa.com");
    app.close_project();
    // The trainee and the instructor register themselves on the program.
    let mut bia_app = app_in(&dir.path().join("cfg_bia"), 9);
    assert!(bia_app.submit_identity("Bia Reis", "bia@empresa.com"));
    assert!(bia_app.open_project(&root));
    let bia = identity(&bia_app, "Bia Reis", "bia@empresa.com");
    bia_app.close_project();
    let mut ivo_app = app_in(&dir.path().join("cfg_ivo"), 21);
    assert!(ivo_app.submit_identity("Ivo Instrutor", "ivo@empresa.com"));
    assert!(ivo_app.open_project(&root));
    let ivo = identity(&ivo_app, "Ivo Instrutor", "ivo@empresa.com");
    ivo_app.close_project();

    let mut s = Session::open_program(&root, &ana, clock(11)).unwrap();
    let pg = s.project();
    let today = s.today();
    let gestor = s.new_id();
    commit(
        &mut s,
        vec![Edit::new(
            Member::entity_ref(gestor),
            fields::person::NAME,
            "Gil Gestor",
        )],
    );
    commit(
        &mut s,
        vec![Edit::new(
            Member::entity_ref(bia.person),
            pf::LEVEL,
            enum_value(BeltLevel::Yellow),
        )],
    );
    let e = perfis::grant(pg, s.new_id(), ivo.person, Profile::Instructor, None);
    commit(&mut s, e);
    let e = curriculum::install_default(pg, BeltLevel::Green, s.ids());
    commit(&mut s, e);
    let mods = curriculum::modules(s.state(), pg);
    let measure = mods.iter().find(|m| m.name == "Medir").unwrap().clone();
    let e = formacao::add_instructor(pg, s.new_id(), ivo.person, &[measure.id]);
    commit(&mut s, e);
    let e = formacao::default_criteria(pg, s.ids());
    commit(&mut s, e);
    let wave = s.new_id();
    commit(
        &mut s,
        formacao::new_wave(pg, wave, "Onda 1", Some(today), None).unwrap(),
    );
    commit(
        &mut s,
        vec![formacao::set_wave_target(wave, BeltLevel::Green, None, 5)],
    );
    let class = s.new_id();
    let e = formacao::new_class(
        pg,
        class,
        &ClassInput {
            wave: Some(wave),
            name: "GB 1".into(),
            level: BeltLevel::Green,
            start: Some(today),
            end: Some(today.add_days(30)),
            place: "Sala 1".into(),
            seats: 10,
        },
    )
    .unwrap();
    commit(&mut s, e);
    commit(
        &mut s,
        vec![formacao::set_instructors(class, &[ivo.person])],
    );
    let nid = s.new_id();
    let e = formacao::nominate(
        pg,
        nid,
        &NominationInput {
            wave: Some(wave),
            person: Some(bia.person),
            level: BeltLevel::Green,
            by: gestor,
            justification: "Lidera a célula".into(),
            opportunity: Some(s.new_id()),
            manager: Some(gestor),
            tenure_months: Some(24.0),
            function: "Analista".into(),
            rating: Some(4.0),
            hours_available: Some(20.0),
            date: Some(today),
        },
        &[],
    )
    .unwrap();
    commit(&mut s, e);
    let n = formacao::nominations(s.state(), pg)[0].clone();
    // The MBB overrides the manager's approval here (the manager has no app in the test).
    commit(
        &mut s,
        formacao::manager_decide(&n, ana.person, true, true, "", today).unwrap(),
    );
    let k = formacao::classes(s.state(), pg)[0].clone();
    let noms = formacao::nominations(s.state(), pg);
    let members = utsim_lss_core::programa::members(s.state());
    let ctx_c = formacao::CriteriaContext {
        members: &members,
        open_targets: Default::default(),
    };
    let ranked = formacao::rank(&k, &noms, &[], &ctx_c, &[], &[]);
    let prop = formacao::proposal(&ranked, k.seats);
    assert_eq!(prop.selected, vec![nid]);
    commit(
        &mut s,
        formacao::confirm_selection(class, &prop, ana.person, today),
    );
    for i in 0..3 {
        let x = s.new_id();
        let e = formacao::new_session(
            class,
            x,
            today.add_days(i),
            4.0,
            &[measure.id],
            Some(ivo.person),
        );
        commit(&mut s, e);
        commit(
            &mut s,
            vec![formacao::mark(x, bia.person, Attendance::Present)],
        );
    }
    s.close();

    // The instructor fills the bank in the restricted folder and draws the exam.
    let staff = TrainingWriters {
        staff: vec![ana.person, ivo.person],
        students: vec![],
    };
    let bank_path = program::training_path(&root, TrainingFolder::Questions);
    let mut bank_s = Session::open_training(
        &bank_path,
        &ivo,
        clock(23),
        TrainingFolder::Questions,
        pg,
        staff.clone(),
        true,
    )
    .unwrap();
    for qi in exams::sample_questions(measure.id, &measure.tools) {
        let e = exams::add_question(bank_s.new_id(), &qi).unwrap();
        commit(&mut bank_s, e);
    }
    let e = exams::add_question(
        bank_s.new_id(),
        &QuestionInput {
            module: Some(measure.id),
            kind: QuestionKind::MultipleChoice,
            statement: "Qual estudo confere o sistema de medição?".into(),
            options: vec!["MSA".into(), "Pareto".into()],
            right_option: Some(0),
            difficulty: 1,
            ..QuestionInput::default()
        },
    )
    .unwrap();
    commit(&mut bank_s, e);
    let bank = exams::bank(bank_s.state());
    assert!(bank.len() >= 3);
    // The trainee never writes in the bank.
    let bia_writers = Session::open_training(
        &bank_path,
        &bia,
        clock(29),
        TrainingFolder::Questions,
        pg,
        staff.clone(),
        false,
    );
    let mut bia_bank = bia_writers.unwrap();
    assert!(
        bia_bank
            .apply(
                exams::add_question(
                    bia_bank.new_id(),
                    &QuestionInput {
                        statement: "forjada".into(),
                        kind: QuestionKind::TrueFalse,
                        ..QuestionInput::default()
                    }
                )
                .unwrap()
            )
            .is_err()
    );
    bia_bank.close();
    bank_s.close();

    let mut s = Session::open_program(&root, &ivo, clock(31)).unwrap();
    let exam = s.new_id();
    let ones = bank.iter().filter(|q| q.difficulty == 1).count() as u32;
    let twos = bank.iter().filter(|q| q.difficulty == 2).count() as u32;
    let e = exams::new_exam(
        pg,
        exam,
        &ExamInput {
            class,
            name: "Prova do Medir".into(),
            modules: vec![measure.id],
            counts: [ones, twos, 0],
            seed: 2026,
            mode: Mode::App,
            min_grade: 0.7,
            date: Some(today),
            retake_of: None,
            counts_for_cert: true,
        },
        &bank,
    )
    .unwrap();
    commit(&mut s, e);
    let ex = exams::exams(s.state())[0].clone();
    s.close();
    let class_path = program::training_path(&root, TrainingFolder::Class(class));
    let class_writers = TrainingWriters {
        staff: vec![ana.person, ivo.person],
        students: vec![bia.person],
    };
    let mut cs = Session::open_training(
        &class_path,
        &ivo,
        clock(37),
        TrainingFolder::Class(class),
        pg,
        class_writers.clone(),
        true,
    )
    .unwrap();
    commit(&mut cs, exams::publish(&ex, &bank, &[bia.person]).unwrap());
    cs.close();

    // The trainee answers from her own app (exam mode drawn), right where the key says.
    let mut bs = Session::open_training(
        &class_path,
        &bia,
        clock(41),
        TrainingFolder::Class(class),
        pg,
        class_writers.clone(),
        false,
    )
    .unwrap();
    let paper = exams::published(bs.state())[0].papers[&bia.person].clone();
    let text = serde_json::to_string(&paper).unwrap();
    assert!(!text.contains("gabarito"));
    let mut answers = BTreeMap::new();
    for it in &paper.items {
        let q = bank.iter().find(|q| q.id == it.question).unwrap();
        answers.insert(q.id, exams::expected(q, &it.data));
    }
    let aid = bs.new_id();
    commit(
        &mut bs,
        exams::answer(aid, exam, bia.person, &answers, Some(today)),
    );
    // Handed in: no more changes.
    assert!(
        bs.apply(exams::answer(aid, exam, bia.person, &BTreeMap::new(), None))
            .is_err()
    );
    bs.close();

    // The instructor grades on her machine and records the result.
    let mut cs = Session::open_training(
        &class_path,
        &ivo,
        clock(43),
        TrainingFolder::Class(class),
        pg,
        class_writers,
        false,
    )
    .unwrap();
    let handed = exams::all_answers(cs.state());
    assert_eq!(handed.len(), 1);
    let g = exams::grade(&ex, &bank, bia.person, &handed[0].answers).unwrap();
    assert!(g.passed && (g.score - 1.0).abs() < 1e-12, "{g:?}");
    cs.close();
    let grades_path = program::training_path(&root, TrainingFolder::Grades);
    let mut gs = Session::open_training(
        &grades_path,
        &ivo,
        clock(47),
        TrainingFolder::Grades,
        pg,
        staff,
        true,
    )
    .unwrap();
    let rid = gs.new_id();
    commit(
        &mut gs,
        exams::record_result(rid, &ex, bia.person, g.score, &g.items, ivo.person, today),
    );
    gs.close();

    // The MBB's opinion.
    let mut s = Session::open_program(&root, &ana, clock(53)).unwrap();
    let e = certificacao::record_opinion(
        pg,
        s.new_id(),
        bia.person,
        BeltLevel::Green,
        "Pronta",
        ana.person,
        today,
    )
    .unwrap();
    commit(&mut s, e);
    assert!(s.unauthorized().is_empty(), "{:?}", s.unauthorized());
    s.close();

    // The trainee's reaction evaluation of the class: hers only, once.
    {
        use utsim_lss_core::formacao::reacao;
        let w = TrainingWriters {
            staff: vec![ana.person, ivo.person],
            students: vec![bia.person],
        };
        let mut bs = Session::open_training(
            &class_path,
            &bia,
            clock(57),
            TrainingFolder::Class(class),
            pg,
            w,
            false,
        )
        .unwrap();
        let rid = bs.new_id();
        let scores = [Some(5), Some(4), Some(5), Some(3), Some(5)];
        commit(
            &mut bs,
            reacao::answer(class, rid, bia.person, &scores, "Gostei", today),
        );
        assert!(
            bs.apply(reacao::answer(
                class,
                rid,
                bia.person,
                &[Some(1); 5],
                "",
                today
            ))
            .is_err(),
            "sent: no more changes"
        );
        let other = bs.new_id();
        assert!(
            bs.apply(reacao::answer(class, other, ivo.person, &scores, "", today))
                .is_err(),
            "not someone else's"
        );
        let r = reacao::reactions(bs.state());
        assert_eq!(r.len(), 1);
        assert_eq!(reacao::summary(&r, class), None, "one answer: no summary");
        bs.close();
    }

    // Every L7 screen as the MBB, the instructor and the trainee.
    for (who, a) in [
        ("mbb", &mut app),
        ("instrutor", &mut ivo_app),
        ("aluna", &mut bia_app),
    ] {
        assert!(a.open_project(&root), "{who}");
        a.select_in_training(Some(class), Some(bia.person), BeltLevel::Green);
        for nav in L7 {
            a.pnav = nav;
            frames(a, &ctx, 3);
            no_error(a, &format!("{who} {nav:?}"));
        }
        a.close_project();
    }
    assert!(bia_app.open_project(&root));
    bia_app.start_exam(exam);
    bia_app.pnav = ProgramNav::MyTraining;
    frames(&mut bia_app, &ctx, 3);
    no_error(&bia_app, "modo prova");
    bia_app.close_project();

    // Learning fed the competence map: Bia studied the tools of Medir.
    let s = Session::open_program(&root, &ana, clock(59)).unwrap();
    let ss = formacao::all_sessions(s.state());
    let le = formacao::learning_entries(&ss, &curriculum::modules(s.state(), pg));
    assert!(
        le.iter()
            .any(|e| e.person == Some(bia.person) && e.tool == "msa")
    );
    let _ = Value::Null;
}
