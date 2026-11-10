//! Texts of the stage L1 screens (portfolio, charter versions, team and RACI, tollgates,
//! risks, meetings, schedule, savings, closing and lessons, folder picker), in a catalogue of
//! their own next to the main one, so the stages grow without touching the same lines.

use crate::text::{fx, tx};
use utsim_idiomas::PorIdioma;
use utsim_lss_core::closure::Requirement;
use utsim_lss_core::domain::{
    Origin, ProjectStatus, RiskResponse, RiskStatus, RoleKind, SavingCategory, fields,
};
use utsim_lss_core::meeting::{ItemKind, MeetingKind};
use utsim_lss_core::portfolio::{Quadrant, Question, Scale};
use utsim_lss_core::savings::Source;
use utsim_lss_core::tollgate::{ApprovalRule, Check, Decision, TollgateStatus};
use utsim_lss_core::{charter, closure, schedule};

type S = &'static str;

pub struct L1Texts {
    pub add: S,
    pub bad_number: S,
    pub browse: S,
    pub nav_team: S,
    pub nav_closure: S,

    // Folder picker.
    pub picker_title: S,
    pub picker_up: S,
    pub picker_empty: S,
    pub picker_choose: S,
    pub picker_cancel: S,
    pub picker_tip: S,
    pub picker_file_tip: S,

    // Charter.
    pub charter_intro: S,
    pub charter_no_version: S,
    pub charter_not_approved: S,
    pub charter_approved: S,
    pub charter_minor_changes: fn(usize) -> String,
    pub charter_needs_reapproval: S,
    pub versions: S,
    pub version_reason_hint: S,
    pub record_version: S,
    pub version_recorded: S,
    pub no_versions: S,
    pub approved_by: fn(&str, &str) -> String,
    pub version_recorded_state: S,
    pub compare: S,
    pub approve_version: S,
    pub version_approved: S,
    pub diff_between: fn(i64, i64) -> String,
    pub diff_to_current: fn(i64) -> String,
    pub no_differences: S,
    pub field: S,
    pub before: S,
    pub after: S,

    // Team.
    pub team_intro: S,
    pub people: S,
    pub remove_role: S,
    pub person_name_hint: S,
    pub person_email_hint: S,
    pub add_person: S,
    pub person_needs_name: S,
    pub give_role: S,
    pub raci: S,
    pub raci_intro: S,
    pub raci_no_roles: S,
    pub row_charter: S,
    pub row_tollgate: fn(&str) -> String,
    pub row_schedule: S,
    pub row_risks: S,
    pub row_savings: S,
    pub row_control_plan: S,
    pub row_closure: S,
    pub raci_single_a: fn(&str) -> String,

    // Phases and tollgates.
    pub phases_intro: S,
    pub project_is: fn(&str) -> String,
    pub tollgate_of: fn(&str) -> String,
    pub no_tollgate: S,
    pub prepare_tollgate: S,
    pub tollgate_state: fn(&str, i64) -> String,
    pub changed_after_approval: S,
    pub frozen_hash: fn(&str) -> String,
    pub resume: S,
    pub resubmit: S,
    pub submit: S,
    pub submitted: S,
    pub phase_approved_note: S,
    pub checklist: S,
    pub item_manual: S,
    pub item_verified: S,
    pub remove_item: S,
    pub required_missing: fn(usize) -> String,
    pub new_item_hint: S,
    pub item_manual_choice: S,
    pub approvers: S,
    pub anyone_decides: S,
    pub rule: S,
    pub planned_date: S,
    pub pending_outcome: fn(usize, usize) -> String,
    pub your_decision: S,
    pub reservations_hint: S,
    pub comment_hint: S,
    pub record_decision: S,
    pub decision_recorded: S,
    pub decisions: S,
    pub round: S,

    // Board.
    pub wip_over: S,
    pub wip_limits: S,
    pub wip_intro: S,
    pub card_origin: fn(&str) -> String,
    pub tollgate_decision: S,
    pub card_link: fn(&str, &str) -> String,
    pub effectiveness: S,
    pub effectiveness_criterion: S,
    pub effectiveness_date: S,
    pub effectiveness_ok: S,

    // Risks.
    pub risks_intro: S,
    pub risk_matrix: S,
    pub matrix_axes: S,
    pub new_risk_hint: S,
    pub pick_a_risk: S,
    pub r_description: S,
    pub r_cause: S,
    pub r_effect: S,
    pub r_probability: S,
    pub r_impact: S,
    pub r_response: S,
    pub r_plan: S,
    pub r_owner: S,
    pub r_review: S,
    pub r_status: S,
    pub response_to_card: S,
    pub response_card_title: fn(&str) -> String,
    pub card_created: S,

    // Meetings.
    pub meetings_intro: S,
    pub new_meeting_hint: S,
    pub new_meeting: S,
    pub meeting_default_title: fn(&str) -> String,
    pub no_meetings: S,
    pub pick_a_meeting: S,
    pub m_title: S,
    pub m_date: S,
    pub m_kind: S,
    pub m_present: S,
    pub m_agenda: S,
    pub generate_agenda: S,
    pub agenda_nothing: S,
    pub m_notes: S,
    pub m_decisions: S,
    pub minutes_items: S,
    pub card_in: fn(&str) -> String,
    pub no_card_yet: S,
    pub new_minutes_item_hint: S,
    pub actions_to_cards: fn(usize) -> String,
    pub cards_created: fn(usize) -> String,
    pub linked_cards: fn(usize) -> String,
    pub ag_overdue: fn(&str, &str) -> String,
    pub ag_due_soon: fn(&str, &str) -> String,
    pub ag_high_risk: fn(&str, u8) -> String,
    pub ag_risk_review: fn(&str) -> String,
    pub ag_tollgate: fn(&str) -> String,
    pub ag_pending_minutes: fn(&str) -> String,
    pub ag_conflicts: fn(usize) -> String,

    // Schedule.
    pub schedule_intro: S,
    pub schedule_start: S,
    pub workdays: S,
    pub new_task_hint: S,
    pub add_milestones: S,
    pub freeze_baseline: S,
    pub baseline_frozen: S,
    pub t_name: S,
    pub t_duration: S,
    pub t_three_points: S,
    pub t_preds: S,
    pub t_who: S,
    pub t_percent: S,
    pub t_target: S,
    pub milestone: S,
    pub preds_tip: S,
    pub holidays: fn(usize) -> String,
    pub holiday_name_hint: S,
    pub add_national_holidays: S,
    pub schedule_finish: fn(&str, &str) -> String,
    pub schedule_sd: fn(&str) -> String,
    pub deadline_probability: fn(&str, &str) -> String,
    pub run_monte_carlo: S,
    pub monte_carlo_result: fn(usize, &str) -> String,
    pub monte_carlo_deadline: fn(&str) -> String,
    pub w_negative_lag: fn(&str) -> String,
    pub w_unknown_pred: fn(&str) -> String,
    pub w_bad_three_point: fn(&str) -> String,
    pub milestone_probability: fn(&str, &str, &str) -> String,
    pub no_tasks: S,
    pub legend_critical: S,
    pub legend_baseline: S,
    pub legend_today: S,

    // Savings.
    pub savings_intro: S,
    pub definitions: S,
    pub hard: S,
    pub hard_definition: S,
    pub soft: S,
    pub soft_definition: S,
    pub follow_up_default: fn(u32) -> String,
    pub new_saving_hint: S,
    pub value_hint: S,
    pub from_simulation: S,
    pub pick_a_saving: S,
    pub s_description: S,
    pub s_category: S,
    pub s_value: S,
    pub s_rationale: S,
    pub s_monthly: S,
    pub s_start: S,
    pub s_months: S,
    pub move_saving_to: fn(&str) -> String,
    pub back_saving_to: fn(&str) -> String,
    pub forecast_by: fn(&str, &str) -> String,
    pub validated_by: fn(&str, &str) -> String,
    pub monthly_entries: S,
    pub month_hint: S,
    pub record_month: S,
    pub bad_month_or_value: S,
    pub no_follow_up: S,
    pub month: S,
    pub expected: S,
    pub realized: S,
    pub cumulative_expected: S,
    pub cumulative_realized: S,

    // Closing and lessons.
    pub closure_intro: S,
    pub c_result: S,
    pub c_notes: S,
    pub owner_accepted: fn(&str) -> String,
    pub accept_transfer: S,
    pub advice: S,
    pub close_project_now: S,
    pub project_closed: S,
    pub lessons: S,
    pub new_lesson_hint: S,
    pub l_title: S,
    pub l_worked: S,
    pub l_did_not_work: S,
    pub l_repeat: S,
    pub l_tags: S,
    pub knowledge_base: S,
    pub knowledge_intro: S,
    pub search_hint: S,
    pub refresh_portfolio: S,
    pub hits: fn(usize, usize) -> String,
    pub similar_projects: S,

    // Portfolio.
    pub portfolio_intro: S,
    pub no_portfolio: S,
    pub no_portfolio_tip: S,
    pub portfolio_name_hint: S,
    pub unit_name_hint: S,
    pub program_name_hint: S,
    pub create_portfolio: S,
    pub default_portfolio_name: S,
    pub default_unit_name: S,
    pub default_program_name: S,
    pub portfolio_header: fn(&str, &str, &str) -> String,
    pub new_idea_hint: S,
    pub closed_ideas: fn(usize) -> String,
    pub belt_overload: fn(&str, usize) -> String,
    pub i_title: S,
    pub i_process: S,
    pub i_problem: S,
    pub i_data: S,
    pub i_rough_gain: S,
    pub i_impact_effort: S,
    pub i_belt: S,
    pub screening: S,
    pub suggested_approach: fn(&str, &str) -> String,
    pub default_reason: S,
    pub chosen_approach: S,
    pub to_choose: S,
    pub scores: S,
    pub alignment_with: fn(&str) -> String,
    pub idea_score: fn(f64, f64) -> String,
    pub idea_state: fn(&str) -> String,
    pub move_idea_to: fn(&str) -> String,
    pub open_project_from_idea: S,
    pub idea_project: fn(&str, &str) -> String,
    pub choose_approach_first: S,
    pub default_project_title: S,
    pub project_opened_from_idea: fn(&str) -> String,
    pub prioritisation: S,
    pub matrix_title: S,
    pub matrix_tip: S,
    pub ranking: S,
    pub completeness: fn(f64) -> String,
    pub criteria_and_objectives: S,
    pub criterion: S,
    pub weight: S,
    pub scale: S,
    pub inverse: S,
    pub from_alignment: S,
    pub new_criterion_hint: S,
    pub default_criteria: S,
    pub objectives: S,
    pub new_objective_hint: S,
    pub projects_here: S,
    pub no_projects_here: S,
    pub validated_savings: S,
}

pub fn l1() -> &'static L1Texts {
    static CACHE: PorIdioma<L1Texts> = PorIdioma::new();
    CACHE.obter(construir_l1_texts)
}

fn construir_l1_texts() -> L1Texts {
    L1Texts {
        add: tx("l1-add"),
        bad_number: tx("l1-bad_number"),
        browse: tx("l1-browse"),
        nav_team: tx("l1-nav_team"),
        nav_closure: tx("l1-nav_closure"),
        picker_title: tx("l1-picker_title"),
        picker_up: tx("l1-picker_up"),
        picker_empty: tx("l1-picker_empty"),
        picker_choose: tx("l1-picker_choose"),
        picker_cancel: tx("l1-picker_cancel"),
        picker_tip: tx("l1-picker_tip"),
        picker_file_tip: tx("l1-picker_file_tip"),
        charter_intro: tx("l1-charter_intro"),
        charter_no_version: tx("l1-charter_no_version"),
        charter_not_approved: tx("l1-charter_not_approved"),
        charter_approved: tx("l1-charter_approved"),
        charter_minor_changes: |n| fx("l1-charter_minor_changes", &[("n", n.into())]),
        charter_needs_reapproval: tx("l1-charter_needs_reapproval"),
        versions: tx("l1-versions"),
        version_reason_hint: tx("l1-version_reason_hint"),
        record_version: tx("l1-record_version"),
        version_recorded: tx("l1-version_recorded"),
        no_versions: tx("l1-no_versions"),
        approved_by: |who, when| {
            fx(
                "l1-approved_by",
                &[("who", who.into()), ("when", when.into())],
            )
        },
        version_recorded_state: tx("l1-version_recorded_state"),
        compare: tx("l1-compare"),
        approve_version: tx("l1-approve_version"),
        version_approved: tx("l1-version_approved"),
        diff_between: |a, b| fx("l1-diff_between", &[("a", a.into()), ("b", b.into())]),
        diff_to_current: |a| fx("l1-diff_to_current", &[("a", a.into())]),
        no_differences: tx("l1-no_differences"),
        field: tx("l1-field"),
        before: tx("l1-before"),
        after: tx("l1-after"),
        team_intro: tx("l1-team_intro"),
        people: tx("l1-people"),
        remove_role: tx("l1-remove_role"),
        person_name_hint: tx("l1-person_name_hint"),
        person_email_hint: tx("l1-person_email_hint"),
        add_person: tx("l1-add_person"),
        person_needs_name: tx("l1-person_needs_name"),
        give_role: tx("l1-give_role"),
        raci: tx("l1-raci"),
        raci_intro: tx("l1-raci_intro"),
        raci_no_roles: tx("l1-raci_no_roles"),
        row_charter: tx("l1-row_charter"),
        row_tollgate: |phase| fx("l1-row_tollgate", &[("phase", phase.into())]),
        row_schedule: tx("l1-row_schedule"),
        row_risks: tx("l1-row_risks"),
        row_savings: tx("l1-row_savings"),
        row_control_plan: tx("l1-row_control_plan"),
        row_closure: tx("l1-row_closure"),
        raci_single_a: |rows| fx("l1-raci_single_a", &[("rows", rows.into())]),
        phases_intro: tx("l1-phases_intro"),
        project_is: |st| fx("l1-project_is", &[("st", st.into())]),
        tollgate_of: |p| fx("l1-tollgate_of", &[("p", p.into())]),
        no_tollgate: tx("l1-no_tollgate"),
        prepare_tollgate: tx("l1-prepare_tollgate"),
        tollgate_state: |st, round| {
            fx(
                "l1-tollgate_state",
                &[("st", st.into()), ("round", round.into())],
            )
        },
        changed_after_approval: tx("l1-changed_after_approval"),
        frozen_hash: |h| fx("l1-frozen_hash", &[("h", h.into())]),
        resume: tx("l1-resume"),
        resubmit: tx("l1-resubmit"),
        submit: tx("l1-submit"),
        submitted: tx("l1-submitted"),
        phase_approved_note: tx("l1-phase_approved_note"),
        checklist: tx("l1-checklist"),
        item_manual: tx("l1-item_manual"),
        item_verified: tx("l1-item_verified"),
        remove_item: tx("l1-remove_item"),
        required_missing: |n| fx("l1-required_missing", &[("n", n.into())]),
        new_item_hint: tx("l1-new_item_hint"),
        item_manual_choice: tx("l1-item_manual_choice"),
        approvers: tx("l1-approvers"),
        anyone_decides: tx("l1-anyone_decides"),
        rule: tx("l1-rule"),
        planned_date: tx("l1-planned_date"),
        pending_outcome: |a, n| fx("l1-pending_outcome", &[("a", a.into()), ("n", n.into())]),
        your_decision: tx("l1-your_decision"),
        reservations_hint: tx("l1-reservations_hint"),
        comment_hint: tx("l1-comment_hint"),
        record_decision: tx("l1-record_decision"),
        decision_recorded: tx("l1-decision_recorded"),
        decisions: tx("l1-decisions"),
        round: tx("l1-round"),
        wip_over: tx("l1-wip_over"),
        wip_limits: tx("l1-wip_limits"),
        wip_intro: tx("l1-wip_intro"),
        card_origin: |o| fx("l1-card_origin", &[("o", o.into())]),
        tollgate_decision: tx("l1-tollgate_decision"),
        card_link: |kind, name| {
            fx(
                "l1-card_link",
                &[("kind", kind.into()), ("name", name.into())],
            )
        },
        effectiveness: tx("l1-effectiveness"),
        effectiveness_criterion: tx("l1-effectiveness_criterion"),
        effectiveness_date: tx("l1-effectiveness_date"),
        effectiveness_ok: tx("l1-effectiveness_ok"),
        risks_intro: tx("l1-risks_intro"),
        risk_matrix: tx("l1-risk_matrix"),
        matrix_axes: tx("l1-matrix_axes"),
        new_risk_hint: tx("l1-new_risk_hint"),
        pick_a_risk: tx("l1-pick_a_risk"),
        r_description: tx("l1-r_description"),
        r_cause: tx("l1-r_cause"),
        r_effect: tx("l1-r_effect"),
        r_probability: tx("l1-r_probability"),
        r_impact: tx("l1-r_impact"),
        r_response: tx("l1-r_response"),
        r_plan: tx("l1-r_plan"),
        r_owner: tx("l1-r_owner"),
        r_review: tx("l1-r_review"),
        r_status: tx("l1-r_status"),
        response_to_card: tx("l1-response_to_card"),
        response_card_title: |r| fx("l1-response_card_title", &[("r", r.into())]),
        card_created: tx("l1-card_created"),
        meetings_intro: tx("l1-meetings_intro"),
        new_meeting_hint: tx("l1-new_meeting_hint"),
        new_meeting: tx("l1-new_meeting"),
        meeting_default_title: |d| fx("l1-meeting_default_title", &[("d", d.into())]),
        no_meetings: tx("l1-no_meetings"),
        pick_a_meeting: tx("l1-pick_a_meeting"),
        m_title: tx("l1-m_title"),
        m_date: tx("l1-m_date"),
        m_kind: tx("l1-m_kind"),
        m_present: tx("l1-m_present"),
        m_agenda: tx("l1-m_agenda"),
        generate_agenda: tx("l1-generate_agenda"),
        agenda_nothing: tx("l1-agenda_nothing"),
        m_notes: tx("l1-m_notes"),
        m_decisions: tx("l1-m_decisions"),
        minutes_items: tx("l1-minutes_items"),
        card_in: |col| fx("l1-card_in", &[("col", col.into())]),
        no_card_yet: tx("l1-no_card_yet"),
        new_minutes_item_hint: tx("l1-new_minutes_item_hint"),
        actions_to_cards: |n| fx("l1-actions_to_cards", &[("n", n.into())]),
        cards_created: |n| fx("l1-cards_created", &[("n", n.into())]),
        linked_cards: |n| fx("l1-linked_cards", &[("n", n.into())]),
        ag_overdue: |t, d| fx("l1-ag_overdue", &[("t", t.into()), ("d", d.into())]),
        ag_due_soon: |t, d| fx("l1-ag_due_soon", &[("t", t.into()), ("d", d.into())]),
        ag_high_risk: |t, n| fx("l1-ag_high_risk", &[("t", t.into()), ("n", n.into())]),
        ag_risk_review: |t| fx("l1-ag_risk_review", &[("t", t.into())]),
        ag_tollgate: |p| fx("l1-ag_tollgate", &[("p", p.into())]),
        ag_pending_minutes: |t| fx("l1-ag_pending_minutes", &[("t", t.into())]),
        ag_conflicts: |n| fx("l1-ag_conflicts", &[("n", n.into())]),
        schedule_intro: tx("l1-schedule_intro"),
        schedule_start: tx("l1-schedule_start"),
        workdays: tx("l1-workdays"),
        new_task_hint: tx("l1-new_task_hint"),
        add_milestones: tx("l1-add_milestones"),
        freeze_baseline: tx("l1-freeze_baseline"),
        baseline_frozen: tx("l1-baseline_frozen"),
        t_name: tx("l1-t_name"),
        t_duration: tx("l1-t_duration"),
        t_three_points: tx("l1-t_three_points"),
        t_preds: tx("l1-t_preds"),
        t_who: tx("l1-t_who"),
        t_percent: tx("l1-t_percent"),
        t_target: tx("l1-t_target"),
        milestone: tx("l1-milestone"),
        preds_tip: tx("l1-preds_tip"),
        holidays: |n| fx("l1-holidays", &[("n", n.into())]),
        holiday_name_hint: tx("l1-holiday_name_hint"),
        add_national_holidays: tx("l1-add_national_holidays"),
        schedule_finish: |d, n| fx("l1-schedule_finish", &[("d", d.into()), ("n", n.into())]),
        schedule_sd: |sd| fx("l1-schedule_sd", &[("sd", sd.into())]),
        deadline_probability: |d, p| {
            fx(
                "l1-deadline_probability",
                &[("d", d.into()), ("p", p.into())],
            )
        },
        run_monte_carlo: tx("l1-run_monte_carlo"),
        monte_carlo_result: |n, ps| {
            fx(
                "l1-monte_carlo_result",
                &[("n", n.into()), ("ps", ps.into())],
            )
        },
        monte_carlo_deadline: |p| fx("l1-monte_carlo_deadline", &[("p", p.into())]),
        w_negative_lag: |t| fx("l1-w_negative_lag", &[("t", t.into())]),
        w_unknown_pred: |t| fx("l1-w_unknown_pred", &[("t", t.into())]),
        w_bad_three_point: |t| fx("l1-w_bad_three_point", &[("t", t.into())]),
        milestone_probability: |m, d, p| {
            fx(
                "l1-milestone_probability",
                &[("m", m.into()), ("d", d.into()), ("p", p.into())],
            )
        },
        no_tasks: tx("l1-no_tasks"),
        legend_critical: tx("l1-legend_critical"),
        legend_baseline: tx("l1-legend_baseline"),
        legend_today: tx("l1-legend_today"),
        savings_intro: tx("l1-savings_intro"),
        definitions: tx("l1-definitions"),
        hard: tx("l1-hard"),
        hard_definition: tx("l1-hard_definition"),
        soft: tx("l1-soft"),
        soft_definition: tx("l1-soft_definition"),
        follow_up_default: |m| fx("l1-follow_up_default", &[("m", m.into())]),
        new_saving_hint: tx("l1-new_saving_hint"),
        value_hint: tx("l1-value_hint"),
        from_simulation: tx("l1-from_simulation"),
        pick_a_saving: tx("l1-pick_a_saving"),
        s_description: tx("l1-s_description"),
        s_category: tx("l1-s_category"),
        s_value: tx("l1-s_value"),
        s_rationale: tx("l1-s_rationale"),
        s_monthly: tx("l1-s_monthly"),
        s_start: tx("l1-s_start"),
        s_months: tx("l1-s_months"),
        move_saving_to: |s| fx("l1-move_saving_to", &[("s", s.into())]),
        back_saving_to: |s| fx("l1-back_saving_to", &[("s", s.into())]),
        forecast_by: |w, d| fx("l1-forecast_by", &[("w", w.into()), ("d", d.into())]),
        validated_by: |w, d| fx("l1-validated_by", &[("w", w.into()), ("d", d.into())]),
        monthly_entries: tx("l1-monthly_entries"),
        month_hint: tx("l1-month_hint"),
        record_month: tx("l1-record_month"),
        bad_month_or_value: tx("l1-bad_month_or_value"),
        no_follow_up: tx("l1-no_follow_up"),
        month: tx("l1-month"),
        expected: tx("l1-expected"),
        realized: tx("l1-realized"),
        cumulative_expected: tx("l1-cumulative_expected"),
        cumulative_realized: tx("l1-cumulative_realized"),
        closure_intro: tx("l1-closure_intro"),
        c_result: tx("l1-c_result"),
        c_notes: tx("l1-c_notes"),
        owner_accepted: |w| fx("l1-owner_accepted", &[("w", w.into())]),
        accept_transfer: tx("l1-accept_transfer"),
        advice: tx("l1-advice"),
        close_project_now: tx("l1-close_project_now"),
        project_closed: tx("l1-project_closed"),
        lessons: tx("l1-lessons"),
        new_lesson_hint: tx("l1-new_lesson_hint"),
        l_title: tx("l1-l_title"),
        l_worked: tx("l1-l_worked"),
        l_did_not_work: tx("l1-l_did_not_work"),
        l_repeat: tx("l1-l_repeat"),
        l_tags: tx("l1-l_tags"),
        knowledge_base: tx("l1-knowledge_base"),
        knowledge_intro: tx("l1-knowledge_intro"),
        search_hint: tx("l1-search_hint"),
        refresh_portfolio: tx("l1-refresh_portfolio"),
        hits: |n, total| fx("l1-hits", &[("n", n.into()), ("total", total.into())]),
        similar_projects: tx("l1-similar_projects"),
        portfolio_intro: tx("l1-portfolio_intro"),
        no_portfolio: tx("l1-no_portfolio"),
        no_portfolio_tip: tx("l1-no_portfolio_tip"),
        portfolio_name_hint: tx("l1-portfolio_name_hint"),
        unit_name_hint: tx("l1-unit_name_hint"),
        program_name_hint: tx("l1-program_name_hint"),
        create_portfolio: tx("l1-create_portfolio"),
        default_portfolio_name: tx("l1-default_portfolio_name"),
        default_unit_name: tx("l1-default_unit_name"),
        default_program_name: tx("l1-default_program_name"),
        portfolio_header: |c, u, p| {
            fx(
                "l1-portfolio_header",
                &[("c", c.into()), ("u", u.into()), ("p", p.into())],
            )
        },
        new_idea_hint: tx("l1-new_idea_hint"),
        closed_ideas: |n| fx("l1-closed_ideas", &[("n", n.into())]),
        belt_overload: |who, n| fx("l1-belt_overload", &[("who", who.into()), ("n", n.into())]),
        i_title: tx("l1-i_title"),
        i_process: tx("l1-i_process"),
        i_problem: tx("l1-i_problem"),
        i_data: tx("l1-i_data"),
        i_rough_gain: tx("l1-i_rough_gain"),
        i_impact_effort: tx("l1-i_impact_effort"),
        i_belt: tx("l1-i_belt"),
        screening: tx("l1-screening"),
        suggested_approach: |a, why| {
            fx(
                "l1-suggested_approach",
                &[("a", a.into()), ("why", why.into())],
            )
        },
        default_reason: tx("l1-default_reason"),
        chosen_approach: tx("l1-chosen_approach"),
        to_choose: tx("l1-to_choose"),
        scores: tx("l1-scores"),
        alignment_with: |o| fx("l1-alignment_with", &[("o", o.into())]),
        idea_score: |s, c| {
            fx(
                "l1-idea_score",
                &[
                    ("s", s.round_ties_even().into()),
                    ("c", c.round_ties_even().into()),
                ],
            )
        },
        idea_state: |s| fx("l1-idea_state", &[("s", s.into())]),
        move_idea_to: |s| fx("l1-move_idea_to", &[("s", s.into())]),
        open_project_from_idea: tx("l1-open_project_from_idea"),
        idea_project: |f, id| fx("l1-idea_project", &[("f", f.into()), ("id", id.into())]),
        choose_approach_first: tx("l1-choose_approach_first"),
        default_project_title: tx("l1-default_project_title"),
        project_opened_from_idea: |p| fx("l1-project_opened_from_idea", &[("p", p.into())]),
        prioritisation: tx("l1-prioritisation"),
        matrix_title: tx("l1-matrix_title"),
        matrix_tip: tx("l1-matrix_tip"),
        ranking: tx("l1-ranking"),
        completeness: |c| fx("l1-completeness", &[("c", c.round_ties_even().into())]),
        criteria_and_objectives: tx("l1-criteria_and_objectives"),
        criterion: tx("l1-criterion"),
        weight: tx("l1-weight"),
        scale: tx("l1-scale"),
        inverse: tx("l1-inverse"),
        from_alignment: tx("l1-from_alignment"),
        new_criterion_hint: tx("l1-new_criterion_hint"),
        default_criteria: tx("l1-default_criteria"),
        objectives: tx("l1-objectives"),
        new_objective_hint: tx("l1-new_objective_hint"),
        projects_here: tx("l1-projects_here"),
        no_projects_here: tx("l1-no_projects_here"),
        validated_savings: tx("l1-validated_savings"),
    }
}

/// Label of a charter (or other) field shown in the differences.
pub fn field_label(f: &str) -> String {
    use fields::project as p;
    let s = match f {
        p::TITLE => tx("l1-field_label-title"),
        p::PROCESS => tx("l1-field_label-process"),
        p::AREA => tx("l1-field_label-area"),
        p::PROBLEM => tx("l1-field_label-problem"),
        charter::fields::Y => tx("l1-field_label-y"),
        p::GOAL => tx("l1-field_label-goal"),
        p::BASELINE => tx("l1-field_label-baseline"),
        p::DEADLINE => tx("l1-field_label-deadline"),
        p::SCOPE_IN => tx("l1-field_label-scope_in"),
        p::SCOPE_OUT => tx("l1-field_label-scope_out"),
        p::BUSINESS_CASE => tx("l1-field_label-business_case"),
        charter::fields::ASSUMPTIONS => tx("l1-field_label-assumptions"),
        charter::fields::CONSTRAINTS => tx("l1-field_label-constraints"),
        charter::fields::INITIAL_RISKS => tx("l1-field_label-initial_risks"),
        schedule::fields::START => tx("l1-field_label-start"),
        closure::fields::RESULT => tx("l1-field_label-result"),
        other => return other.to_string(),
    };
    s.to_string()
}

pub fn role_name(k: RoleKind) -> &'static str {
    match k {
        RoleKind::Sponsor => tx("l1-role_name-sponsor"),
        RoleKind::Champion => "Champion",
        RoleKind::ProcessOwner => tx("l1-role_name-process_owner"),
        RoleKind::MasterBlackBelt => "Master black belt",
        RoleKind::BlackBelt => "Black belt",
        RoleKind::GreenBelt => "Green belt",
        RoleKind::YellowBelt => "Yellow belt",
        RoleKind::Team => tx("l1-role_name-team"),
        RoleKind::Controller => tx("l1-role_name-controller"),
    }
}

pub fn check_name(c: &Check) -> String {
    match c {
        Check::CharterComplete => tx("l1-check_name-charter_complete").into(),
        Check::CharterApproved => tx("l1-check_name-charter_approved").into(),
        Check::TeamDefined => tx("l1-check_name-team_defined").into(),
        Check::ScheduleDefined => tx("l1-check_name-schedule_defined").into(),
        Check::RisksAssessed => tx("l1-check_name-risks_assessed").into(),
        Check::HighRisksTreated => tx("l1-check_name-high_risks_treated").into(),
        Check::SavingEstimated => tx("l1-check_name-saving_estimated").into(),
        Check::SavingsForecast => tx("l1-check_name-savings_forecast").into(),
        Check::SavingsValidated => tx("l1-check_name-savings_validated").into(),
        Check::PhaseActionsDone => tx("l1-check_name-phase_actions_done").into(),
        Check::LessonsRecorded => tx("l1-check_name-lessons_recorded").into(),
        Check::OwnerAccepted => tx("l1-check_name-owner_accepted").into(),
        Check::DeliverableExists(t) => fx(
            "l1-check_name-deliverable_exists",
            &[("t", t.as_str().into())],
        ),
        Check::MsaApproved => tx("l1-check_name-msa_approved").into(),
        Check::BaselineReal => tx("l1-check_name-baseline_real").into(),
    }
}

pub fn decision_name(d: Decision) -> &'static str {
    match d {
        Decision::Approve => tx("l1-decision_name-approve"),
        Decision::ApproveWithReservations => tx("l1-decision_name-approve_with_reservations"),
        Decision::Redo => tx("l1-decision_name-redo"),
        Decision::Suspend => tx("l1-decision_name-suspend"),
        Decision::Cancel => tx("l1-decision_name-cancel"),
    }
}

pub fn rule_name(r: ApprovalRule) -> &'static str {
    match r {
        ApprovalRule::All => tx("l1-rule_name-all"),
        ApprovalRule::Majority => tx("l1-rule_name-majority"),
        ApprovalRule::SponsorRequired => tx("l1-rule_name-sponsor_required"),
    }
}

pub fn tollgate_status_name(s: TollgateStatus) -> &'static str {
    match s {
        TollgateStatus::Open => tx("l1-tollgate_status_name-open"),
        TollgateStatus::Submitted => tx("l1-tollgate_status_name-submitted"),
        TollgateStatus::Approved => tx("l1-tollgate_status_name-approved"),
        TollgateStatus::Redo => tx("l1-tollgate_status_name-redo"),
        TollgateStatus::Suspended => tx("l1-tollgate_status_name-suspended"),
        TollgateStatus::Cancelled => tx("l1-tollgate_status_name-cancelled"),
    }
}

pub fn project_status_name(s: ProjectStatus) -> &'static str {
    match s {
        ProjectStatus::Idea => tx("l1-project_status_name-idea"),
        ProjectStatus::Screening => tx("l1-project_status_name-screening"),
        ProjectStatus::Approved => tx("l1-project_status_name-approved"),
        ProjectStatus::Running => tx("l1-project_status_name-running"),
        ProjectStatus::Closed => tx("l1-project_status_name-closed"),
        ProjectStatus::Cancelled => tx("l1-project_status_name-cancelled"),
        ProjectStatus::Archived => tx("l1-project_status_name-archived"),
        ProjectStatus::Suspended => tx("l1-project_status_name-suspended"),
    }
}

pub fn response_name(r: RiskResponse) -> &'static str {
    match r {
        RiskResponse::Avoid => tx("l1-response_name-avoid"),
        RiskResponse::Mitigate => tx("l1-response_name-mitigate"),
        RiskResponse::Transfer => tx("l1-response_name-transfer"),
        RiskResponse::Accept => tx("l1-response_name-accept"),
    }
}

pub fn risk_status_name(s: RiskStatus) -> &'static str {
    match s {
        RiskStatus::Open => tx("l1-risk_status_name-open"),
        RiskStatus::Closed => tx("l1-risk_status_name-closed"),
    }
}

pub fn item_kind_name(k: ItemKind) -> &'static str {
    match k {
        ItemKind::Note => tx("l1-item_kind_name-note"),
        ItemKind::Decision => tx("l1-item_kind_name-decision"),
        ItemKind::Action => tx("l1-item_kind_name-action"),
    }
}

pub fn meeting_kind_name(k: MeetingKind) -> &'static str {
    match k {
        MeetingKind::Team => tx("l1-meeting_kind_name-team"),
        MeetingKind::Tollgate => tx("l1-meeting_kind_name-tollgate"),
        MeetingKind::Portfolio => tx("l1-meeting_kind_name-portfolio"),
    }
}

pub fn weekday_short(d: u8) -> &'static str {
    let days: [S; 7] = [
        tx("l1-weekday-0"),
        tx("l1-weekday-1"),
        tx("l1-weekday-2"),
        tx("l1-weekday-3"),
        tx("l1-weekday-4"),
        tx("l1-weekday-5"),
        tx("l1-weekday-6"),
    ];
    days.get(usize::from(d.saturating_sub(1)))
        .copied()
        .unwrap_or("?")
}

pub fn category_name(c: SavingCategory) -> &'static str {
    match c {
        SavingCategory::Hard => "hard",
        SavingCategory::Soft => "soft",
    }
}

pub fn source_name(s: Source) -> &'static str {
    match s {
        Source::Belt => "belt",
        Source::Simulation => tx("l1-source_name-simulation"),
        Source::Controller => tx("l1-source_name-controller"),
    }
}

pub fn requirement_name(r: Requirement) -> &'static str {
    match r {
        Requirement::TollgatesApproved => tx("l1-requirement_name-tollgates_approved"),
        Requirement::OwnerAccepted => tx("l1-requirement_name-owner_accepted"),
        Requirement::LessonsRecorded => tx("l1-requirement_name-lessons_recorded"),
        Requirement::ResultWritten => tx("l1-requirement_name-result_written"),
        Requirement::SavingsForecast => tx("l1-requirement_name-savings_forecast"),
        Requirement::ActionsDone => tx("l1-requirement_name-actions_done"),
    }
}

pub fn quadrant_name(q: Quadrant) -> &'static str {
    match q {
        Quadrant::QuickWin => tx("l1-quadrant_name-quick_win"),
        Quadrant::Strategic => tx("l1-quadrant_name-strategic"),
        Quadrant::Minor => tx("l1-quadrant_name-minor"),
        Quadrant::Avoid => tx("l1-quadrant_name-avoid"),
    }
}

pub fn question_text(q: Question) -> &'static str {
    match q {
        Question::NewDesign => tx("l1-question_text-new_design"),
        Question::CauseKnown => tx("l1-question_text-cause_known"),
        Question::LowRisk => tx("l1-question_text-low_risk"),
        Question::LocalQuick => tx("l1-question_text-local_quick"),
        Question::OnePage => tx("l1-question_text-one_page"),
        Question::ShortCycle => tx("l1-question_text-short_cycle"),
    }
}

pub fn scale_name(s: Scale) -> &'static str {
    match s {
        Scale::OneToFive => tx("l1-scale_name-one_to_five"),
        Scale::OneThreeNine => "1-3-9",
    }
}

pub fn origin_name(o: Origin) -> &'static str {
    match o {
        Origin::Manual => tx("l1-origin_name-manual"),
        Origin::Fmea => tx("l1-origin_name-fmea"),
        Origin::Minutes => tx("l1-origin_name-minutes"),
        Origin::Tollgate => tx("l1-origin_name-tollgate"),
        Origin::Risk => tx("l1-origin_name-risk"),
        Origin::ResponsePlan => tx("l1-origin_name-response_plan"),
        Origin::Kaizen => "kaizen",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatted_texts() {
        let t = l1();
        assert_eq!(
            (t.tollgate_state)("aberto", 0),
            "Situação: aberto (rodada 0)"
        );
        assert_eq!(weekday_short(1), "seg");
        assert_eq!(weekday_short(9), "?");
        assert_eq!(
            field_label("meta"),
            "Meta (SMART, com unidade e fonte do dado)"
        );
        assert_eq!(field_label("x/y"), "x/y");
        assert!(check_name(&Check::DeliverableExists("fmea".into())).contains("fmea"));
        for q in Question::ALL {
            assert!(question_text(q).ends_with('?'));
        }
    }
}
