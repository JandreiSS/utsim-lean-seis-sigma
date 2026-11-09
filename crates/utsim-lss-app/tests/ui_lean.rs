//! The DMADV and Lean tools without a window (P6): each deliverable filled as a team would,
//! the numbers read back from the journal, and every screen drawn. With
//! `UTSIM_CAPTURAS=<pasta>` the screens are saved as PNG for review.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::egui;
use serde_json::Value;
use utsim_lss_app::App;
use utsim_lss_app::app::Nav;
use utsim_lss_core::domain::{Approach, Column, Origin};
use utsim_lss_core::kanban;
use utsim_lss_core::tools::lean::{self, changeover, five_s, selection, standard_work};
use utsim_lss_core::tools::{self, Deliverable, ItemRef, Tool, kaizen, pdca};
use utsim_lss_store::user::AppDirs;
use utsim_lss_store::{Clock, SystemClock};
use utsim_ui_tema::captura::Captura;

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

fn t(s: &str) -> Value {
    Value::from(s)
}

fn n(x: f64) -> Value {
    Value::from(x)
}

#[test]
fn lean_tools_fill_compute_and_draw() {
    let dir = tempfile::tempdir().unwrap();
    let counter = Arc::new(AtomicU64::new(0));
    let mut app = App::with_clock(
        AppDirs::under(&dir.path().join("cfg")),
        Box::new(move || Box::new(Salted(Arc::clone(&counter), 41))),
    );
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com"));
    let work = dir.path().join("Drive");
    std::fs::create_dir_all(&work).unwrap();
    assert!(app.create_project(&work, "Troca da prensa", Approach::Dmaic));

    let ids = {
        let s = app.session_mut().unwrap();
        // Selection, Pugh against the current method.
        let sel = s
            .add_deliverable(Tool::Selection, "Escolha da solução")
            .unwrap();
        s.set_field(
            Deliverable::entity_ref(sel),
            selection::fields::METHOD,
            t("pugh"),
            None,
        )
        .unwrap();
        let crit: Vec<_> = [("Custo", 5), ("Prazo", 3), ("Segurança", 8)]
            .iter()
            .map(|(c, w)| {
                s.add_item(
                    sel,
                    selection::CRITERION,
                    None,
                    vec![
                        (selection::fields::NAME, t(c)),
                        (selection::fields::WEIGHT, Value::from(*w)),
                    ],
                )
                .unwrap()
            })
            .collect();
        let mut opts = Vec::new();
        for (name, imp, eff) in [
            ("Método atual", 3, 3),
            ("Gabarito rápido", 8, 3),
            ("Robô", 9, 9),
        ] {
            opts.push(
                s.add_item(
                    sel,
                    selection::OPTION,
                    None,
                    vec![
                        (selection::fields::NAME, t(name)),
                        (selection::fields::IMPACT, Value::from(imp)),
                        (selection::fields::EFFORT, Value::from(eff)),
                    ],
                )
                .unwrap(),
            );
        }
        let e = selection::set_reference(s.state(), sel, opts[0]);
        s.apply(e).unwrap();
        let cells = [(1, [1, 1, 1]), (2, [-1, -1, 1])];
        let mut e = Vec::new();
        for (o, vals) in cells {
            for (c, v) in crit.iter().zip(vals) {
                e.push(lean::set_score(opts[o], *c, Some(v)));
            }
        }
        s.apply(e).unwrap();

        // 5S, two audits.
        let fs = s.add_deliverable(Tool::FiveS, "5S da prensa").unwrap();
        let e = five_s::starter_edits(s.state(), s.project(), fs, || s.new_id());
        s.apply(e).unwrap();
        let qs = five_s::questions(s.state(), fs);
        for (date, base) in [("2026-10-12", 1), ("2026-11-09", 3)] {
            let a = s
                .add_item(
                    fs,
                    five_s::AUDIT,
                    None,
                    vec![(five_s::fields::DATE, t(date))],
                )
                .unwrap();
            let e = qs
                .iter()
                .enumerate()
                .map(|(i, q)| lean::set_score(a, q.id, Some((base + (i as i64 % 2)).min(4))))
                .collect();
            s.apply(e).unwrap();
        }

        // SMED.
        let sm = s.add_deliverable(Tool::Smed, "SMED da prensa").unwrap();
        for (name, time, class, after, time_after) in [
            ("Buscar a matriz nova", 12.0, "interna", "externa", None),
            ("Soltar a matriz", 8.0, "interna", "", Some(4.0)),
            ("Procurar as chaves", 6.0, "interna", "eliminada", None),
            ("Ajustar a altura", 15.0, "interna", "", Some(5.0)),
            ("Separar a chapa", 5.0, "externa", "", None),
        ] {
            let mut v = vec![
                (changeover::fields::NAME, t(name)),
                (changeover::fields::TIME, n(time)),
                (changeover::fields::CLASS, t(class)),
            ];
            if !after.is_empty() {
                v.push((changeover::fields::CLASS_AFTER, t(after)));
            }
            if let Some(x) = time_after {
                v.push((changeover::fields::TIME_AFTER, n(x)));
            }
            s.add_item(sm, changeover::ACTIVITY, None, v).unwrap();
        }

        // Standardised work.
        let sw = s
            .add_deliverable(Tool::StandardWork, "Célula da prensa")
            .unwrap();
        let dr = Deliverable::entity_ref(sw);
        s.set_field(dr, standard_work::fields::AVAILABLE, n(27_000.0), None)
            .unwrap();
        s.set_field(dr, standard_work::fields::DEMAND, n(450.0), None)
            .unwrap();
        for (op, name, m, a, w) in [
            ("Ana", "Pegar a chapa", 5.0, 0.0, 3.0),
            ("Ana", "Prensar", 6.0, 35.0, 4.0),
            ("Ana", "Rebarbar", 20.0, 0.0, 2.0),
            ("Bia", "Dobrar", 25.0, 0.0, 3.0),
            ("Bia", "Inspecionar", 40.0, 0.0, 2.0),
        ] {
            s.add_item(
                sw,
                standard_work::ELEMENT,
                None,
                vec![
                    (standard_work::fields::OPERATOR, t(op)),
                    (standard_work::fields::NAME, t(name)),
                    (standard_work::fields::MANUAL, n(m)),
                    (standard_work::fields::AUTO, n(a)),
                    (standard_work::fields::WALK, n(w)),
                    (standard_work::fields::WIP, n(1.0)),
                ],
            )
            .unwrap();
        }

        // Kaizen event with an open item sent to the board.
        let kz = s
            .add_deliverable(Tool::KaizenEvent, "Kaizen da prensa")
            .unwrap();
        let kr = Deliverable::entity_ref(kz);
        s.set_field(kr, kaizen::fields::START, t("2026-09-14"), None)
            .unwrap();
        s.set_field(kr, kaizen::fields::DAYS, Value::from(3), None)
            .unwrap();
        let e = kaizen::template_edits(s.state(), s.project(), kz, || s.new_id());
        s.apply(e).unwrap();
        let open = s
            .add_item(
                kz,
                kaizen::OPEN_ITEM,
                None,
                vec![
                    (kaizen::fields::TEXT, t("Pintar as faixas do corredor")),
                    (kaizen::fields::WHO, t("Caio")),
                    (kaizen::fields::DUE, t("2026-10-01")),
                ],
            )
            .unwrap();
        let card = s.new_id();
        let e = {
            let entity = s.state().entity(open).unwrap();
            kaizen::card_edits(s.state(), s.project(), &ItemRef { id: open, entity }, card)
        };
        s.apply(e).unwrap();

        // PDCA.
        let pd = s.add_deliverable(Tool::Pdca, "PDCA do setup").unwrap();
        s.add_item(
            pd,
            pdca::CYCLE,
            None,
            vec![
                (
                    pdca::fields::HYPOTHESIS,
                    t("Pré-aquecer a matriz reduz o ajuste"),
                ),
                (pdca::fields::PLAN, t("Aquecer fora da prensa por 20 min")),
                (pdca::fields::DO, t("Feito em 8 trocas")),
                (pdca::fields::CHECK, t("Ajuste de 15 para 5 min")),
                (pdca::fields::VERDICT, t("confirmada")),
                (pdca::fields::ACT, t("Aquecimento no padrão da troca")),
            ],
        )
        .unwrap();
        s.add_item(
            pd,
            pdca::CYCLE,
            None,
            vec![(
                pdca::fields::HYPOTHESIS,
                t("Carrinho dedicado elimina a busca"),
            )],
        )
        .unwrap();
        [sel, fs, sm, sw, kz, pd]
    };

    {
        let s = app.session().unwrap();
        let st = s.state();
        let m = selection::matrix(st, ids[0]);
        // Gabarito: 5 + 3 + 8 = 16; Robô: −5 − 3 + 8 = 0.
        assert_eq!(m.result.options[1].total, 16.0);
        assert_eq!(m.result.options[2].total, 0.0);
        assert_eq!(m.result.best().unwrap().name, "Gabarito rápido");
        let au = five_s::audits(st, ids[1]);
        assert_eq!(au.len(), 2);
        assert!(au[1].score.pct > au[0].score.pct);
        let sm = changeover::study(st, ids[2]);
        // Internal before 12 + 8 + 6 + 15 = 41; after 4 + 5 = 9.
        assert_eq!((sm.before.internal, sm.after.internal), (41.0, 9.0));
        let b = standard_work::balance(st, ids[3]);
        assert_eq!(b.takt, Some(60.0));
        assert_eq!(b.min_operators, Some(2));
        let c = utsim_lss_core::domain::Action::read(
            kanban::board(st, s.project())[&Column::Todo][0].id,
            st.entity(kanban::board(st, s.project())[&Column::Todo][0].id)
                .unwrap(),
        );
        assert_eq!(c.origin, Origin::Kaizen);
        let f = kaizen::follow_ups(st, ids[4], s.today());
        assert!(f[0].due, "2026-10-16 is past");
        let cs = pdca::cycles(st, ids[5]);
        assert_eq!(
            cs[1].previous_act.as_deref(),
            Some("Aquecimento no padrão da troca")
        );
        assert_eq!(tools::deliverables(st, s.project()).len(), 6);
    }

    app.nav = Nav::Tools;
    let ctx = egui::Context::default();
    let shots = std::env::var_os("UTSIM_CAPTURAS");
    for (i, id) in ids.iter().enumerate() {
        app.selected_deliverable = Some(*id);
        for _ in 0..3 {
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| app.draw(ui));
            out.textures_delta.clear();
        }
        assert!(
            app.status().is_none_or(|(_, err)| !err),
            "{i}: {:?}",
            app.status()
        );
        if let Some(d) = &shots {
            let mut cap = Captura::new(egui::Context::default(), egui::vec2(1400.0, 1500.0));
            let img = cap.imagem(4, |ui| app.draw(ui));
            std::fs::create_dir_all(d).unwrap();
            img.salvar_png(&std::path::Path::new(d).join(format!("lean_{i}.png")))
                .unwrap();
        }
    }
}
