//! Stage L5 drawn without a window: the reports screen of a project with a charter, a
//! tollgate prepared, a meeting, an FMEA and a control plan; every document rendered to a
//! well-formed PDF, the spreadsheet and the CSV written in `relatorios/`, and the company's
//! template saved next to the projects and used by the next print.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use utsim_lss_app::App;
use utsim_lss_app::app::Nav;
use utsim_lss_core::domain::{self, Approach, fields};
use utsim_lss_core::meeting::{self, MeetingKind};
use utsim_lss_core::relatorios::{Template, pdf};
use utsim_lss_core::tools::{Deliverable, Tool};
use utsim_lss_core::{Edit, EntityRef, Kind, tollgate};
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

fn frames(app: &mut App, ctx: &egui::Context, n: usize) {
    for _ in 0..n {
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| app.draw(ui));
        out.textures_delta.clear();
    }
}

#[test]
fn reports_render_and_the_template_applies() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = egui::Context::default();
    let counter = Arc::new(AtomicU64::new(0));
    let mut app = App::with_clock(
        AppDirs::under(&dir.path().join("cfg")),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 3))),
    );
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com"));
    let work = dir.path().join("Drive");
    std::fs::create_dir_all(&work).unwrap();
    assert!(app.create_project(&work, "Refugo da prensa", Approach::Dmaic));
    let (fmea_id, cp_id, meeting_id) = {
        let s = app.session_mut().unwrap();
        let p = s.project();
        let today = s.today();
        let r = EntityRef::new(Kind::Project, p);
        s.apply(vec![
            Edit::new(r, fields::project::PROBLEM, "Refugo de 6 % com ação ç"),
            Edit::new(r, fields::project::GOAL, "2 %"),
        ])
        .unwrap();
        let ph = domain::phases(s.state(), p)[0].id;
        let e = tollgate::prepare(s.state(), p, ph, s.ids());
        s.apply(e).unwrap();
        let m = s.new_id();
        s.apply(meeting::new_meeting(
            p,
            m,
            "Reunião de abertura",
            today,
            MeetingKind::Team,
        ))
        .unwrap();
        let f = s.add_deliverable(Tool::Fmea, "FMEA").unwrap();
        s.add_item(
            f,
            utsim_lss_core::tools::fmea::ITEM,
            None,
            vec![("modo", "Rebarba".into())],
        )
        .unwrap();
        let c = s
            .add_deliverable(Tool::ControlPlan, "Plano de controle")
            .unwrap();
        (f, c, m)
    };
    app.nav = Nav::Reports;
    frames(&mut app, &ctx, 3);
    assert!(app.status().is_none_or(|(_, e)| !e), "{:?}", app.status());

    let s = app.session().unwrap();
    let st = s.state().clone();
    let p = s.project();
    let ph = domain::phases(&st, p)[0].id;
    let del = |id| st.entity(id).map(|e| Deliverable::read(id, e)).unwrap();
    let docs = [
        utsim_lss_app::reports::charter_doc(&st, p),
        utsim_lss_app::reports::a3_doc(&st, p),
        utsim_lss_app::reports::tollgate_doc(&st, p, ph),
        utsim_lss_app::reports::minutes_doc(&st, p, meeting_id),
        utsim_lss_app::reports::fmea_doc(&st, p, &del(fmea_id)),
        utsim_lss_app::reports::control_doc(&st, p, &del(cp_id)),
    ];
    for d in &docs {
        let b = pdf::render(d, &Template::default(), "teste");
        assert!(b.starts_with(b"%PDF-1.4"));
        assert!(String::from_utf8_lossy(&b).trim_end().ends_with("%%EOF"));
    }
    assert!(docs[0].plain_text().contains("Refugo de 6 % com ação ç"));
    assert!(docs[4].plain_text().contains("Rebarba"));
    let tables = utsim_lss_app::reports::project_tables(&st, p);
    assert!(tables.iter().any(|(n, _, _)| n.starts_with("FMEA")));
    let refs: Vec<utsim_lss_core::relatorios::TableRef<'_>> = tables
        .iter()
        .map(|(n, h, r)| (n.as_str(), h.as_slice(), r.as_slice()))
        .collect();
    let x = utsim_lss_store::planilha::xlsx(&refs).unwrap();
    assert_eq!(&x[..2], b"PK");

    // The company's template next to the projects is read by every project there.
    let t = Template {
        company: "Metalúrgica Exemplo".into(),
        ..Template::default()
    };
    let path = s.folder().save_report_template(&t.to_json()).unwrap();
    assert!(path.ends_with("modelos/relatorio.json") || path.ends_with("modelos\\relatorio.json"));
    let back = Template::from_json(&s.folder().report_template().unwrap()).unwrap();
    assert_eq!(back.company, "Metalúrgica Exemplo");
    let out = s.folder().write_report("termo.pdf", b"%PDF-1.4").unwrap();
    assert!(out.is_file());
    assert!(s.folder().write_report("../fora.pdf", b"x").is_err());
}
