//! The documents of the reports (stage L5, `docs/plano_lean_seis_sigma.md`, 4.6): charter,
//! tollgate report, minutes, FMEA, control plan, A3 (A3 landscape) and the certificate, built
//! from the journal's state with the screens' texts, and the tables of the spreadsheet.
//! Version 2 adds the charts: a saved analysis with its chart, the Pareto inside the A3 and
//! the value stream map with its ladder.
//! The rendering is [`utsim_lss_core::relatorios`].

use utsim_lss_core::analises;
use utsim_lss_core::certificacao::Certification;
use utsim_lss_core::domain::{self, fields as df};
use utsim_lss_core::relatorios::{Chart, Document, PageSize};
use utsim_lss_core::tools::{self, Deliverable, Tool, a3, fmea, items, plans, vsm};
use utsim_lss_core::{Date, Id, State, charter, kanban, meeting, risk, savings, tollgate};

use crate::text::l1::{decision_name, field_label, response_name, risk_status_name};
use crate::text::l6::level_name;
use crate::text::{column_name, saving_status_name, tool_name};

fn name(state: &State, id: Option<Id>) -> String {
    id.map(|p| domain::person_name(state, p))
        .unwrap_or_default()
}

fn date(d: Option<Date>) -> String {
    d.map(Date::format_br).unwrap_or_default()
}

fn project_title(state: &State, project: Id) -> String {
    state
        .entity(project)
        .map(|e| e.text(df::project::TITLE))
        .unwrap_or_default()
}

fn value_text(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => match s.parse::<Date>() {
            Ok(d) => d.format_br(),
            Err(_) => s.clone(),
        },
        serde_json::Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// The charter: the current fields, the version and its approval.
pub fn charter_doc(state: &State, project: Id) -> Document {
    let t = crate::text::l5::l5();
    let title = project_title(state, project);
    let mut d = Document::new(t.charter, &title);
    let cur = charter::current(state, project);
    let mut kv = Vec::new();
    for f in charter::CHARTER_FIELDS {
        kv.push((
            field_label(f),
            cur.get(f).map(value_text).unwrap_or_default(),
        ));
    }
    d.heading(t.charter_fields).kv(kv);
    if let Some(v) = charter::latest_approved(state, project) {
        d.heading(t.approval).kv(vec![
            (t.version.into(), v.number.to_string()),
            (t.approved_by.into(), name(state, v.approved_by)),
            (t.approved_on.into(), date(v.approved_on)),
        ]);
    } else {
        d.para(t.not_approved);
    }
    let team: Vec<Vec<String>> = utsim_lss_core::roles::roles(state, project)
        .into_iter()
        .map(|r| {
            vec![
                name(state, r.person),
                crate::text::l1::role_name(r.role).to_string(),
            ]
        })
        .collect();
    d.heading(t.team)
        .table(t.team, &[t.person, t.role], team, &[2.0, 1.0]);
    d.signatures(vec![t.sponsor_sign.into(), t.belt_sign.into()]);
    d
}

/// The tollgate report of a phase: the checklist with evidence, the decisions and the
/// reservations, the frozen hash.
pub fn tollgate_doc(state: &State, project: Id, phase: Id) -> Document {
    let t = crate::text::l5::l5();
    let ph = domain::phases(state, project)
        .into_iter()
        .find(|p| p.id == phase);
    let phase_name = ph.map(|p| p.name).unwrap_or_default();
    let mut d = Document::new(
        &(t.tollgate_of)(&phase_name),
        &project_title(state, project),
    );
    let Some(tg) = tollgate::of_phase(state, phase) else {
        d.para(t.no_tollgate);
        return d;
    };
    d.kv(vec![
        (
            t.status.into(),
            crate::text::l1::tollgate_status_name(tg.status).to_string(),
        ),
        (t.round.into(), tg.round.to_string()),
        (t.submitted_on.into(), date(tg.submitted_on)),
        (t.frozen_hash.into(), tg.frozen_hash.clone()),
    ]);
    let rows: Vec<Vec<String>> = tollgate::evaluate(state, project, &tg)
        .into_iter()
        .map(|r| {
            vec![
                r.item.text.clone(),
                if r.ok { t.yes.into() } else { t.no.into() },
                r.item.evidence.clone(),
            ]
        })
        .collect();
    d.heading(t.checklist).table(
        t.checklist,
        &[t.item, t.done, t.evidence],
        rows,
        &[3.0, 0.7, 3.0],
    );
    // The deliverables of the phase.
    let dels: Vec<Vec<String>> = tools::deliverables(state, project)
        .into_iter()
        .filter(|x| x.phase == Some(phase))
        .map(|x| {
            vec![
                x.name.clone(),
                x.tool.map(tool_name).unwrap_or_default().to_string(),
                crate::text::deliverable_status_name(x.status).to_string(),
            ]
        })
        .collect();
    d.heading(t.deliverables).table(
        t.deliverables,
        &[t.name, t.tool, t.status],
        dels,
        &[2.0, 1.5, 1.0],
    );
    let decs: Vec<Vec<String>> = tollgate::decisions(state, tg.id)
        .into_iter()
        .map(|x| {
            vec![
                name(state, x.approver),
                x.decision
                    .map(decision_name)
                    .unwrap_or_default()
                    .to_string(),
                date(x.date),
                x.reservations.join("; "),
                x.comment.clone(),
            ]
        })
        .collect();
    d.heading(t.decisions).table(
        t.decisions,
        &[t.approver, t.decision, t.date, t.reservations, t.comment],
        decs,
        &[1.5, 1.0, 0.8, 2.0, 2.0],
    );
    d
}

/// The minutes of a meeting.
pub fn minutes_doc(state: &State, project: Id, meeting_id: Id) -> Document {
    let t = crate::text::l5::l5();
    let m = state
        .entity(meeting_id)
        .map(|e| domain::Meeting::read(meeting_id, e))
        .unwrap_or_default_meeting();
    let mut d = Document::new(&(t.minutes_of)(&m.title), &project_title(state, project));
    let present: Vec<String> = meeting::present(state, meeting_id)
        .into_iter()
        .map(|p| domain::person_name(state, p))
        .collect();
    d.kv(vec![
        (t.date.into(), date(m.date)),
        (t.present.into(), present.join(", ")),
    ]);
    if !m.agenda.trim().is_empty() {
        d.heading(t.agenda).para(&m.agenda);
    }
    let its = meeting::items(state, meeting_id);
    let rows: Vec<Vec<String>> = its
        .iter()
        .map(|i| {
            vec![
                crate::text::l1::item_kind_name(i.kind).to_string(),
                i.text.clone(),
                name(state, i.who),
                date(i.due),
            ]
        })
        .collect();
    d.heading(t.items).table(
        t.items,
        &[t.kind, t.text, t.who, t.due],
        rows,
        &[0.8, 3.0, 1.2, 0.8],
    );
    if !m.notes.trim().is_empty() {
        d.heading(t.notes).para(&m.notes);
    }
    d
}

trait OrDefaultMeeting {
    fn unwrap_or_default_meeting(self) -> domain::Meeting;
}

impl OrDefaultMeeting for Option<domain::Meeting> {
    fn unwrap_or_default_meeting(self) -> domain::Meeting {
        self.unwrap_or(domain::Meeting {
            id: Id::NIL,
            project: None,
            title: String::new(),
            date: None,
            agenda: String::new(),
            notes: String::new(),
            decisions: String::new(),
        })
    }
}

fn rating(v: Option<u8>) -> String {
    v.map(|x| x.to_string()).unwrap_or_default()
}

/// The table of an FMEA deliverable, ranked by the action priority.
pub fn fmea_rows(
    state: &State,
    project: Id,
    deliverable: Id,
) -> (Vec<&'static str>, Vec<Vec<String>>) {
    let t = crate::text::l5::l5();
    let (table, _) = fmea::project_table(state, project);
    let rows = fmea::ranked(&fmea::rows(state, deliverable), &table);
    let header = vec![
        t.step,
        t.mode,
        t.effect,
        "S",
        t.cause,
        "O",
        t.prevention,
        t.detection,
        "D",
        "RPN",
        "AP",
        t.action,
        "S'",
        "O'",
        "D'",
    ];
    let body = rows
        .iter()
        .map(|r| {
            let rev = r.revision(&table);
            vec![
                r.step.clone(),
                r.mode.clone(),
                r.effect.clone(),
                rating(r.s),
                r.cause.clone(),
                rating(r.o),
                r.prevention.clone(),
                r.detection.clone(),
                rating(r.d),
                rev.map(|x| x.rpn_before.to_string()).unwrap_or_default(),
                rev.map(|x| format!("{:?}", x.ap_before))
                    .unwrap_or_default(),
                r.action.clone(),
                rating(r.s2),
                rating(r.o2),
                rating(r.d2),
            ]
        })
        .collect();
    (header, body)
}

pub fn fmea_doc(state: &State, project: Id, deliverable: &Deliverable) -> Document {
    let t = crate::text::l5::l5();
    let mut d = Document::new(
        &(t.fmea_of)(&deliverable.name),
        &project_title(state, project),
    );
    d.size = PageSize::A3Landscape;
    let (h, rows) = fmea_rows(state, project, deliverable.id);
    d.table(
        t.fmea,
        &h,
        rows,
        &[
            1.2, 1.5, 1.5, 0.35, 1.5, 0.35, 1.3, 1.3, 0.35, 0.5, 0.45, 1.8, 0.35, 0.35, 0.35,
        ],
    );
    d
}

/// The control plan lines of a deliverable.
pub fn control_rows(state: &State, deliverable: Id) -> (Vec<&'static str>, Vec<Vec<String>>) {
    let t = crate::text::l5::l5();
    use plans::fields as f;
    let header = vec![
        t.process,
        t.characteristic,
        t.kind,
        t.spec,
        t.method,
        t.control,
        t.owner,
        t.reaction,
        t.critical,
    ];
    let body = items(state, deliverable, plans::CONTROL_ITEM)
        .iter()
        .map(|i| {
            vec![
                i.text(f::PROCESS),
                i.text(f::CHARACTERISTIC),
                i.text(f::CHAR_KIND),
                i.text(f::SPEC),
                i.text(f::METHOD),
                i.text(f::CONTROL),
                i.text(f::OWNER),
                i.text(f::REACTION),
                if i.entity.boolean(f::CRITICAL) {
                    t.yes.into()
                } else {
                    String::new()
                },
            ]
        })
        .collect();
    (header, body)
}

pub fn control_doc(state: &State, project: Id, deliverable: &Deliverable) -> Document {
    let t = crate::text::l5::l5();
    let mut d = Document::new(
        &(t.control_of)(&deliverable.name),
        &project_title(state, project),
    );
    d.size = PageSize::A3Landscape;
    let (h, rows) = control_rows(state, deliverable.id);
    d.table(
        t.control_plan,
        &h,
        rows,
        &[1.2, 1.5, 0.8, 1.0, 1.2, 1.5, 1.0, 1.8, 0.5],
    );
    d
}

/// The A3 of the project, on one A3 landscape sheet when it fits.
pub fn a3_doc(state: &State, project: Id) -> Document {
    let t = crate::text::l5::l5();
    let a3d = tools::deliverables(state, project)
        .into_iter()
        .find(|x| x.tool == Some(Tool::A3))
        .map(|x| x.id);
    let a = a3::compose(state, project, a3d);
    let mut d = Document::new(t.a3, &a.title);
    d.size = PageSize::A3Landscape;
    d.heading(t.a3_background).kv(vec![
        (t.problem.into(), a.problem.clone()),
        (t.business_case.into(), a.business_case.clone()),
        (t.goal.into(), a.goal.clone()),
        (t.baseline.into(), a.baseline.clone()),
        (t.deadline.into(), date(a.deadline)),
    ]);
    d.heading(t.a3_current).bullets(
        a.ctqs
            .iter()
            .cloned()
            .chain(a.process_steps.iter().map(|s| format!("{}: {s}", t.step)))
            .chain(
                a.pareto_vital
                    .iter()
                    .map(|(n, p)| format!("Pareto: {n} ({p:.0} %)")),
            )
            .collect(),
    );
    for p in tools::of_tool(state, project, Tool::Pareto) {
        if let Some(c) = pareto_chart(state, p.id) {
            d.chart(Chart {
                title: format!("Pareto: {}", p.name),
                ..c
            });
        }
    }
    d.heading(t.a3_causes).bullets(
        a.root_causes
            .iter()
            .chain(a.verified_causes.iter())
            .cloned()
            .chain(
                a.top_inputs
                    .iter()
                    .map(|(n, v)| format!("C&E: {n} ({v:.0})")),
            )
            .chain(a.fmea_high.iter().map(|(m, x)| format!("FMEA: {m} → {x}")))
            .collect(),
    );
    d.heading(t.a3_plan).table(
        t.a3_plan,
        &[t.action, t.column, t.who, t.due],
        a.plan
            .iter()
            .map(|p| {
                vec![
                    p.title.clone(),
                    column_name(p.column).to_string(),
                    name(state, p.who),
                    date(p.due),
                ]
            })
            .collect(),
        &[3.0, 1.0, 1.2, 0.8],
    );
    d.heading(t.a3_control).bullets(a.control_lines.clone());
    let notes: Vec<String> = a
        .notes
        .iter()
        .filter(|n| !n.trim().is_empty())
        .cloned()
        .collect();
    if !notes.is_empty() {
        d.heading(t.notes).bullets(notes);
    }
    d
}

/// The certificate of a certification (A4, printed by the program).
pub fn certificate_doc(c: &Certification, person: &str, program: &str, issuer: &str) -> Document {
    let t = crate::text::l5::l5();
    let mut d = Document::new(t.certificate, program);
    d.blocks
        .push(utsim_lss_core::relatorios::Block::Space(30.0));
    d.para(&(t.certifies)(person, level_name(c.level)));
    d.kv(vec![
        (t.number.into(), format!("{:04}", c.number.unwrap_or(0))),
        (t.date.into(), date(c.date)),
        (t.valid_until.into(), date(c.valid_until)),
    ]);
    if c.manual {
        d.para(&(t.manual_record)(&c.note));
    } else {
        let rows: Vec<Vec<String>> = c
            .met
            .iter()
            .map(|m| {
                vec![
                    m.kind.label().to_string(),
                    m.reason.clone(),
                    m.evidence
                        .iter()
                        .map(|e| format!("{} ({}…)", e.text, &e.hash[..12.min(e.hash.len())]))
                        .collect::<Vec<_>>()
                        .join("; "),
                ]
            })
            .collect();
        d.heading(t.requirements_met).table(
            t.requirements_met,
            &[t.requirement, t.why, t.evidence],
            rows,
            &[2.0, 2.0, 2.5],
        );
    }
    d.signatures(vec![(t.issuer_sign)(issuer)]);
    d
}

/// The project's spreadsheet tables: actions, risks, savings, FMEA and control plans, data
/// sets.
pub fn project_tables(state: &State, project: Id) -> Vec<(String, Vec<String>, Vec<Vec<String>>)> {
    let t = crate::text::l5::l5();
    let s = |v: &[&str]| v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let mut out = Vec::new();
    let actions: Vec<Vec<String>> = kanban::board(state, project)
        .into_values()
        .flatten()
        .map(|a| {
            vec![
                a.title.clone(),
                column_name(a.column).to_string(),
                name(state, a.who),
                date(a.due),
                a.why.clone(),
                a.how.clone(),
            ]
        })
        .collect();
    out.push((
        t.actions.to_string(),
        s(&[t.action, t.column, t.who, t.due, t.why, t.how]),
        actions,
    ));
    let risks: Vec<Vec<String>> = risk::risks(state, project)
        .into_iter()
        .map(|r| {
            vec![
                r.description.clone(),
                r.probability.to_string(),
                r.impact.to_string(),
                (u32::from(r.probability) * u32::from(r.impact)).to_string(),
                response_name(r.response).to_string(),
                name(state, r.owner),
                risk_status_name(r.status).to_string(),
            ]
        })
        .collect();
    out.push((
        t.risks.to_string(),
        s(&[
            t.risk,
            t.probability,
            t.impact,
            t.score,
            t.response,
            t.owner,
            t.status,
        ]),
        risks,
    ));
    let sv: Vec<Vec<String>> = savings::plans(state, project)
        .into_iter()
        .map(|p| p.saving)
        .map(|x| {
            vec![
                x.description.clone(),
                crate::text::l1::category_name(x.category).to_string(),
                saving_status_name(x.status).to_string(),
                crate::app::screens::show_number(x.value),
            ]
        })
        .collect();
    out.push((
        t.savings.to_string(),
        s(&[t.saving, t.category, t.status, t.value]),
        sv,
    ));
    for dl in tools::deliverables(state, project) {
        match dl.tool {
            Some(Tool::Fmea) => {
                let (h, rows) = fmea_rows(state, project, dl.id);
                out.push((format!("FMEA {}", dl.name), s(&h), rows));
            }
            Some(Tool::ControlPlan) => {
                let (h, rows) = control_rows(state, dl.id);
                out.push((format!("{} {}", t.control_plan, dl.name), s(&h), rows));
            }
            _ => {}
        }
    }
    let ds: Vec<Vec<String>> = utsim_lss_core::dados::datasets(state, project)
        .into_iter()
        .map(|x| {
            vec![
                x.name.clone(),
                x.origin.label().to_string(),
                x.rows.to_string(),
                x.columns.join(", "),
                x.file.clone(),
            ]
        })
        .collect();
    out.push((
        t.datasets.to_string(),
        s(&[t.name, t.origin, t.rows, t.columns, t.file]),
        ds,
    ));
    out
}

// --- Charts (version 2) ----------------------------------------------------------------------

/// The charts of the result of an analysis: the values with the limits and the signals
/// (control chart, run of the data) and the further plots (effects of an experiment).
pub fn outcome_charts(o: &analises::Outcome, title: &str) -> Vec<Chart> {
    let mut v = Vec::new();
    if !o.series.is_empty() {
        let mut hlines = Vec::new();
        if let Some((c, l, u)) = o.limits {
            hlines.push(("LC".to_string(), c));
            hlines.push(("LIC".to_string(), l));
            hlines.push(("LSC".to_string(), u));
        }
        v.push(Chart {
            title: title.to_string(),
            bars: false,
            labels: (1..=o.series.len()).map(|i| i.to_string()).collect(),
            series: vec![("valores".to_string(), o.series.clone())],
            hlines,
            marks: o.signals.clone(),
            ..Chart::default()
        });
    }
    for p in &o.plots {
        v.push(Chart {
            title: p.title.clone(),
            bars: p.bars,
            labels: p.labels.clone(),
            series: p.series.clone(),
            ..Chart::default()
        });
    }
    v
}

/// The report of a saved analysis: the data set with its origin, the recipe, the numbers,
/// the conclusion, the notes and the charts.
pub fn analysis_doc(state: &State, project: Id, a: &analises::Analysis) -> Document {
    let label = a.recipe.as_ref().map_or("", |r| r.label());
    let mut d = Document::new(
        if a.name.trim().is_empty() {
            label
        } else {
            a.name.as_str()
        },
        &project_title(state, project),
    );
    let ds = a.dataset.and_then(|i| {
        utsim_lss_core::dados::datasets(state, project)
            .into_iter()
            .find(|x| x.id == i)
    });
    d.kv(vec![
        ("Análise".into(), label.to_string()),
        (
            "Conjunto de dados".into(),
            ds.map(|x| x.labeled()).unwrap_or_default(),
        ),
        ("Hash do conjunto".into(), a.dataset_hash.clone()),
        ("Data".into(), date(a.date)),
        ("Conclusão".into(), a.verdict.clone()),
    ]);
    if !a.justification.trim().is_empty() {
        d.kv(vec![("Justificativa".into(), a.justification.clone())]);
    }
    d.table(
        "Resultado",
        &["Medida", "Valor"],
        a.outcome
            .lines
            .iter()
            .map(|(k, v)| vec![k.clone(), crate::app::screens::show_number(*v)])
            .collect(),
        &[2.0, 1.0],
    );
    if !a.outcome.notes.is_empty() {
        d.bullets(a.outcome.notes.clone());
    }
    for c in outcome_charts(&a.outcome, label) {
        d.chart(c);
    }
    d
}

/// The chart of a Pareto deliverable: the bars and the cumulative percentage.
pub fn pareto_chart(state: &State, deliverable: Id) -> Option<Chart> {
    let p = tools::pareto::of_deliverable(state, deliverable);
    if p.rows.is_empty() {
        return None;
    }
    Some(Chart {
        title: "Pareto".into(),
        bars: true,
        labels: p.rows.iter().map(|r| r.name.clone()).collect(),
        series: vec![(
            "ocorrências".into(),
            p.rows.iter().map(|r| r.value).collect(),
        )],
        kind: utsim_lss_core::relatorios::ChartKind::Pareto,
        ..Chart::default()
    })
}

/// The report of a value stream map: the header, the processes, the ladder and the
/// indicators (with the current map when it is a future one).
pub fn vsm_doc(state: &State, project: Id, del: &Deliverable) -> Document {
    let t = crate::text::l10::l10();
    let h = vsm::header(state, del.id);
    let procs = vsm::processes(state, del.id);
    let l = vsm::ladder(&procs, &h);
    let mut d = Document::new(&del.name, &project_title(state, project));
    let n = |x: Option<f64>| x.map(crate::app::screens::show_number).unwrap_or_default();
    let mut kv = vec![
        (
            "Estado".to_string(),
            if h.future {
                t.vsm_future
            } else {
                t.vsm_current
            }
            .to_string(),
        ),
        (t.vsm_demand.into(), n(h.demand)),
        (t.vsm_available.into(), n(h.available)),
        (t.vsm_unit.into(), h.unit.clone()),
        (t.vsm_takt.into(), n(l.takt)),
        (t.vsm_lead.into(), n(l.lead_time)),
        (t.vsm_va_time.into(), n(Some(l.value_added))),
        (
            t.vsm_pce.into(),
            l.pce.map(crate::app::screens::percent).unwrap_or_default(),
        ),
    ];
    if let Some(from) = h.from {
        let b = vsm::ladder(&vsm::processes(state, from), &vsm::header(state, from));
        kv.push((format!("{} antes", t.vsm_lead), n(b.lead_time)));
        kv.push((
            format!("{} antes", t.vsm_pce),
            b.pce.map(crate::app::screens::percent).unwrap_or_default(),
        ));
    }
    d.kv(kv);
    d.table(
        "Processos",
        &[
            t.vsm_process,
            t.vsm_cycle,
            t.vsm_setup,
            t.vsm_uptime,
            t.vsm_operators,
            t.vsm_inventory,
            t.vsm_va,
        ],
        procs
            .iter()
            .map(|p| {
                vec![
                    p.name.clone(),
                    n(p.cycle),
                    n(p.setup),
                    n(p.uptime.map(|u| u * 100.0)),
                    n(p.operators),
                    n(p.inventory),
                    if p.value_added { "sim" } else { "não" }.to_string(),
                ]
            })
            .collect(),
        &[2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.8],
    );
    if !l.over_takt.is_empty() {
        d.para(&(t.vsm_over_takt)(&l.over_takt.join(", ")));
    }
    if !l.steps.is_empty() {
        d.chart(Chart {
            title: t.vsm_ladder.into(),
            bars: true,
            labels: l.steps.iter().map(|s| s.name.clone()).collect(),
            series: vec![
                (
                    t.vsm_wait.into(),
                    l.steps.iter().map(|s| s.wait.unwrap_or(f64::NAN)).collect(),
                ),
                (
                    t.vsm_proc.into(),
                    l.steps.iter().map(|s| s.process).collect(),
                ),
            ],
            ..Chart::default()
        });
    }
    d
}
