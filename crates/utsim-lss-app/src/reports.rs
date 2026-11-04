//! The documents of the reports (stage L5, `docs/plano_lean_seis_sigma.md`, 4.6): charter,
//! tollgate report, minutes, FMEA, control plan, A3 (A3 landscape) and the certificate, built
//! from the journal's state with the screens' texts, and the tables of the spreadsheet.
//! The rendering is [`utsim_lss_core::relatorios`].

use utsim_lss_core::certificacao::Certification;
use utsim_lss_core::domain::{self, fields as df};
use utsim_lss_core::relatorios::{Document, PageSize};
use utsim_lss_core::tools::{self, Deliverable, Tool, a3, fmea, items, plans};
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
