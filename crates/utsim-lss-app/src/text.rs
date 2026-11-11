//! Every text of the interface, in one catalogue: the keys of `idiomas/<código>/seis-sigma.ftl`
//! (`docs/traducao.md`), in the five languages of the family (P8). Screens never write
//! literal text: they read `texts().x`.

use utsim_idiomas::{Catalogo, Idioma, PorIdioma, Valor};

use utsim_lss_core::domain::{Approach, Column, Origin, PhaseStatus, SavingStatus};
use utsim_lss_core::tools::ap::Ap;
use utsim_lss_core::tools::ishikawa::CauseStatus;
use utsim_lss_core::tools::plans::CollectionIssue;
use utsim_lss_core::tools::voc::{Issue, Kano};
use utsim_lss_core::tools::{DeliverableStatus, Tool};

pub struct Texts {
    pub app_name: &'static str,

    // First run.
    pub identity_title: &'static str,
    pub identity_intro: &'static str,
    pub identity_name: &'static str,
    pub identity_email: &'static str,
    pub identity_note: &'static str,
    pub continue_: &'static str,

    // Start screen.
    pub projects: &'static str,
    pub recent: &'static str,
    pub no_recent: &'static str,
    pub missing_folder: &'static str,
    pub open_folder: &'static str,
    pub open: &'static str,
    pub path_hint: &'static str,
    pub new_project: &'static str,
    pub title_hint: &'static str,
    pub parent_hint: &'static str,
    pub approach: &'static str,
    /// Project template of a specialised line (`utsim_lss_core::modelos_linha`).
    pub project_model: &'static str,
    pub no_project_model: &'static str,
    pub create: &'static str,
    pub import_lssz: &'static str,
    pub import_hint: &'static str,
    pub import_dest_hint: &'static str,
    pub import: &'static str,
    pub shared_folder_tip: &'static str,
    pub who_am_i: fn(&str, &str) -> String,
    pub change_identity: &'static str,

    // Navigation.
    pub nav_portfolio: &'static str,
    pub nav_project: &'static str,
    pub nav_overview: &'static str,
    pub nav_charter: &'static str,
    pub nav_phases: &'static str,
    pub nav_actions: &'static str,
    pub nav_risks: &'static str,
    pub nav_meetings: &'static str,
    pub nav_schedule: &'static str,
    pub nav_savings: &'static str,
    pub nav_tools: &'static str,
    pub nav_data: &'static str,
    pub nav_conflicts: &'static str,
    pub close_project: &'static str,
    pub coming_in: fn(&str) -> String,

    // Theme.
    pub theme: &'static str,
    pub theme_system: &'static str,
    pub theme_light: &'static str,
    pub theme_dark: &'static str,
    pub language: &'static str,

    // Overview.
    pub current_phase: &'static str,
    pub phases: &'static str,
    pub actions: &'static str,
    pub actions_done: fn(usize, usize) -> String,
    pub overdue: &'static str,
    pub none_overdue: &'static str,
    pub risks_open: &'static str,
    pub risks_high: &'static str,
    pub savings: &'static str,
    pub meetings: &'static str,
    pub deadline: &'static str,
    pub days_left: fn(i64) -> String,
    pub no_deadline: &'static str,
    pub conflicts: &'static str,
    pub folder: &'static str,
    pub operations: fn(usize) -> String,

    // Charter.
    pub charter_intro: &'static str,
    pub f_title: &'static str,
    pub f_process: &'static str,
    pub f_area: &'static str,
    pub f_problem: &'static str,
    pub f_goal: &'static str,
    pub f_baseline: &'static str,
    pub f_deadline: &'static str,
    pub f_scope_in: &'static str,
    pub f_scope_out: &'static str,
    pub f_business_case: &'static str,
    pub date_hint: &'static str,
    pub bad_date: &'static str,

    // Board.
    pub new_action_hint: &'static str,
    pub add: &'static str,
    pub board_tip: &'static str,
    pub a_what: &'static str,
    pub a_why: &'static str,
    pub a_where: &'static str,
    pub a_when: &'static str,
    pub a_who: &'static str,
    pub a_how: &'static str,
    pub a_how_much: &'static str,
    pub a_column: &'static str,
    pub nobody: &'static str,
    pub delete: &'static str,
    pub close: &'static str,
    pub move_left: &'static str,
    pub move_right: &'static str,
    pub move_up: &'static str,
    pub move_down: &'static str,
    pub untitled: &'static str,

    // Presence and conflicts.
    pub here_now: &'static str,
    pub is_on: fn(&str, &str) -> String,
    pub is_editing_this: fn(&str) -> String,
    pub conflicts_banner: fn(usize) -> String,
    pub review: &'static str,
    pub conflicts_intro: &'static str,
    pub no_conflicts: &'static str,
    pub kept: &'static str,
    pub replaced: &'static str,
    pub keep_this: &'static str,
    pub restore_other: &'static str,
    pub by_whom: fn(&str) -> String,
    pub pending_files: fn(usize) -> String,

    // Data.
    pub data_intro: &'static str,
    pub export_lssz: &'static str,
    pub export_dest_hint: &'static str,
    pub exported: fn(&str) -> String,
    pub imported: fn(&str) -> String,
    pub created: fn(&str) -> String,
    pub error: fn(&str) -> String,

    // Screens still to come, with what they will hold.
    pub soon_portfolio: &'static str,
    pub soon_phases: &'static str,
    pub soon_risks: &'static str,
    pub soon_meetings: &'static str,
    pub soon_schedule: &'static str,
    pub soon_savings: &'static str,
    pub soon_data: &'static str,
}

/// Texts of the "Ferramentas" screen (etapa L2).
pub struct ToolTexts {
    pub intro: &'static str,
    pub new_deliverable: &'static str,
    pub no_deliverables: &'static str,
    pub no_phase: &'static str,
    pub pick_deliverable: &'static str,
    pub unknown_tool: &'static str,
    pub phase: &'static str,
    pub status: &'static str,
    pub bad_number: &'static str,
    pub up: &'static str,
    pub down: &'static str,
    pub remove: &'static str,
    pub no_rows: &'static str,
    pub new_row: &'static str,
    pub add_hint: &'static str,
    pub no_data: &'static str,

    // VOC and CTQ.
    pub voc_statements: &'static str,
    pub voc_statement: &'static str,
    pub voc_statement_hint: &'static str,
    pub source: &'static str,
    pub segment: &'static str,
    pub need: &'static str,
    pub need_hint: &'static str,
    pub ungrouped: &'static str,
    pub statements_n: fn(usize) -> String,
    pub ctq_tree: &'static str,
    pub kano_none: &'static str,
    pub driver: &'static str,
    pub driver_hint: &'static str,
    pub ctq: &'static str,
    pub ctq_hint: &'static str,
    pub definition: &'static str,
    pub unit: &'static str,
    pub lsl: &'static str,
    pub usl: &'static str,
    pub target: &'static str,
    pub orphans: fn(usize) -> String,
    pub tree_ok: &'static str,
    pub tree_issues: fn(usize) -> String,

    // SIPOC.
    pub sipoc_steps: fn(usize) -> String,
    pub sipoc_empty: &'static str,

    // Ishikawa.
    pub effect: &'static str,
    pub effect_missing: &'static str,
    pub cycle_broken: &'static str,
    pub evidence: &'static str,
    pub add_sub: &'static str,
    pub cause_hint: &'static str,
    pub new_category_hint: &'static str,

    // Five whys.
    pub problem: &'static str,
    pub from_cause: &'static str,
    pub no_cause: &'static str,
    pub why_n: fn(usize) -> String,
    pub root_cause: &'static str,
    pub next_why: &'static str,
    pub first_why: &'static str,
    pub roots: fn(&str) -> String,
    pub chain_hint: &'static str,
    pub chain_from_cause: &'static str,

    // C&E matrix.
    pub ce_outputs: &'static str,
    pub output: &'static str,
    pub output_hint: &'static str,
    pub weight: &'static str,
    pub ce_matrix: &'static str,
    pub input: &'static str,
    pub input_hint: &'static str,
    pub total: &'static str,
    pub rank: &'static str,
    pub column_sum: &'static str,
    pub ce_pareto: &'static str,

    // Pareto.
    pub pareto_counts: &'static str,
    pub pareto_table: &'static str,
    pub pareto_table_tip: &'static str,
    pub others_below: &'static str,
    pub by_cost: &'static str,
    pub category: &'static str,
    pub category_hint: &'static str,
    pub count: &'static str,
    pub unit_cost: &'static str,
    pub value: &'static str,
    pub percentage: &'static str,
    pub cumulative: &'static str,
    pub vital: &'static str,

    // Plans.
    pub c_role: &'static str,
    pub c_name: &'static str,
    pub c_data_type: &'static str,
    pub c_stratify: &'static str,
    pub c_sample: &'static str,
    pub c_frequency: &'static str,
    pub c_subgroup: &'static str,
    pub c_who: &'static str,
    pub c_where: &'static str,
    pub c_form: &'static str,
    pub c_missing: &'static str,
    pub c_process: &'static str,
    pub c_characteristic: &'static str,
    pub c_char_kind: &'static str,
    pub c_spec: &'static str,
    pub c_method: &'static str,
    pub c_control: &'static str,
    pub c_owner: &'static str,
    pub c_reaction: &'static str,
    pub c_fmea_link: &'static str,
    pub critical: &'static str,
    pub no_gaps: &'static str,
    pub gaps: fn(usize) -> String,
    pub add_control_line: &'static str,
    pub c_signal: &'static str,
    pub c_immediate: &'static str,
    pub c_containment: &'static str,
    pub c_call: &'static str,
    pub c_deadline: &'static str,
    pub c_card: &'static str,
    pub open_card: &'static str,
    pub make_card: &'static str,
    pub fire_signal: &'static str,
    pub card_made: &'static str,

    // FMEA.
    pub ap_table: &'static str,
    pub ap_builtin: &'static str,
    pub ap_company: fn(&str) -> String,
    pub ap_use_builtin: &'static str,
    pub ap_counts: fn(usize, usize, usize) -> String,
    pub ap_non_monotone: fn(usize) -> String,
    pub ap_csv_hint: &'static str,
    pub ap_name_hint: &'static str,
    pub ap_import: &'static str,
    pub ap_export: &'static str,
    pub ap_need_path: &'static str,
    pub ap_exported: fn(&str) -> String,
    pub ap_imported: &'static str,
    pub ap_rejected: &'static str,
    pub ap_tip: &'static str,
    pub scales: &'static str,
    pub scales_tip: &'static str,
    pub level: &'static str,
    pub scale_s: &'static str,
    pub scale_o: &'static str,
    pub scale_d: &'static str,
    pub critical_from: &'static str,
    pub critical_default: fn(u8) -> String,
    pub fmea_headers: &'static [&'static str],

    // A3.
    pub a3_titles: [&'static str; 7],
    pub a3_empty: &'static str,
    pub a3_note: &'static str,
    pub a3_pareto: &'static str,
    pub a3_process: &'static str,
    pub a3_verified: &'static str,
    pub a3_root: &'static str,
    pub a3_fmea_high: &'static str,
    pub a3_control: &'static str,
    pub a3_signals: fn(usize) -> String,
}

/// Data types of the data collection plan: key and name (Portuguese; see `data_types`).
pub const DATA_TYPES: [(&str, &str); 3] = [
    ("continuo", "Contínuo"),
    ("atributo", "Atributo"),
    ("contagem", "Contagem"),
];

/// Data types in the active language.
pub fn data_types() -> &'static [(&'static str, &'static str); 3] {
    static C: PorIdioma<[(&str, &str); 3]> = PorIdioma::new();
    C.obter(|| {
        [
            ("continuo", tx("t-data_types-continuo")),
            ("atributo", tx("t-data_types-atributo")),
            ("contagem", tx("t-data_types-contagem")),
        ]
    })
}

pub fn tool_texts() -> &'static ToolTexts {
    static CACHE: PorIdioma<ToolTexts> = PorIdioma::new();
    CACHE.obter(construir_tool_texts)
}

fn construir_tool_texts() -> ToolTexts {
    ToolTexts {
        intro: tx("tool-intro"),
        new_deliverable: tx("tool-new_deliverable"),
        no_deliverables: tx("tool-no_deliverables"),
        no_phase: tx("tool-no_phase"),
        pick_deliverable: tx("tool-pick_deliverable"),
        unknown_tool: tx("tool-unknown_tool"),
        phase: tx("tool-phase"),
        status: tx("tool-status"),
        bad_number: tx("tool-bad_number"),
        up: tx("tool-up"),
        down: tx("tool-down"),
        remove: tx("tool-remove"),
        no_rows: tx("tool-no_rows"),
        new_row: tx("tool-new_row"),
        add_hint: tx("tool-add_hint"),
        no_data: tx("tool-no_data"),
        voc_statements: tx("tool-voc_statements"),
        voc_statement: tx("tool-voc_statement"),
        voc_statement_hint: tx("tool-voc_statement_hint"),
        source: tx("tool-source"),
        segment: tx("tool-segment"),
        need: tx("tool-need"),
        need_hint: tx("tool-need_hint"),
        ungrouped: tx("tool-ungrouped"),
        statements_n: |n| fx("tool-statements_n", &[("n", n.into())]),
        ctq_tree: tx("tool-ctq_tree"),
        kano_none: tx("tool-kano_none"),
        driver: tx("tool-driver"),
        driver_hint: tx("tool-driver_hint"),
        ctq: tx("tool-ctq"),
        ctq_hint: tx("tool-ctq_hint"),
        definition: tx("tool-definition"),
        unit: tx("tool-unit"),
        lsl: tx("tool-lsl"),
        usl: tx("tool-usl"),
        target: tx("tool-target"),
        orphans: |n| fx("tool-orphans", &[("n", n.into())]),
        tree_ok: tx("tool-tree_ok"),
        tree_issues: |n| fx("tool-tree_issues", &[("n", n.into())]),
        sipoc_steps: |n| fx("tool-sipoc_steps", &[("n", n.into())]),
        sipoc_empty: tx("tool-sipoc_empty"),
        effect: tx("tool-effect"),
        effect_missing: tx("tool-effect_missing"),
        cycle_broken: tx("tool-cycle_broken"),
        evidence: tx("tool-evidence"),
        add_sub: tx("tool-add_sub"),
        cause_hint: tx("tool-cause_hint"),
        new_category_hint: tx("tool-new_category_hint"),
        problem: tx("tool-problem"),
        from_cause: tx("tool-from_cause"),
        no_cause: tx("tool-no_cause"),
        why_n: |n| fx("tool-why_n", &[("n", n.into())]),
        root_cause: tx("tool-root_cause"),
        next_why: tx("tool-next_why"),
        first_why: tx("tool-first_why"),
        roots: |r| fx("tool-roots", &[("r", r.into())]),
        chain_hint: tx("tool-chain_hint"),
        chain_from_cause: tx("tool-chain_from_cause"),
        ce_outputs: tx("tool-ce_outputs"),
        output: tx("tool-output"),
        output_hint: tx("tool-output_hint"),
        weight: tx("tool-weight"),
        ce_matrix: tx("tool-ce_matrix"),
        input: tx("tool-input"),
        input_hint: tx("tool-input_hint"),
        total: tx("tool-total"),
        rank: tx("tool-rank"),
        column_sum: tx("tool-column_sum"),
        ce_pareto: tx("tool-ce_pareto"),
        pareto_counts: tx("tool-pareto_counts"),
        pareto_table: tx("tool-pareto_table"),
        pareto_table_tip: tx("tool-pareto_table_tip"),
        others_below: tx("tool-others_below"),
        by_cost: tx("tool-by_cost"),
        category: tx("tool-category"),
        category_hint: tx("tool-category_hint"),
        count: tx("tool-count"),
        unit_cost: tx("tool-unit_cost"),
        value: tx("tool-value"),
        percentage: tx("tool-percentage"),
        cumulative: tx("tool-cumulative"),
        vital: tx("tool-vital"),
        c_role: tx("tool-c_role"),
        c_name: tx("tool-c_name"),
        c_data_type: tx("tool-c_data_type"),
        c_stratify: tx("tool-c_stratify"),
        c_sample: tx("tool-c_sample"),
        c_frequency: tx("tool-c_frequency"),
        c_subgroup: tx("tool-c_subgroup"),
        c_who: tx("tool-c_who"),
        c_where: tx("tool-c_where"),
        c_form: tx("tool-c_form"),
        c_missing: tx("tool-c_missing"),
        c_process: tx("tool-c_process"),
        c_characteristic: tx("tool-c_characteristic"),
        c_char_kind: tx("tool-c_char_kind"),
        c_spec: tx("tool-c_spec"),
        c_method: tx("tool-c_method"),
        c_control: tx("tool-c_control"),
        c_owner: tx("tool-c_owner"),
        c_reaction: tx("tool-c_reaction"),
        c_fmea_link: tx("tool-c_fmea_link"),
        critical: tx("tool-critical"),
        no_gaps: tx("tool-no_gaps"),
        gaps: |n| fx("tool-gaps", &[("n", n.into())]),
        add_control_line: tx("tool-add_control_line"),
        c_signal: tx("tool-c_signal"),
        c_immediate: tx("tool-c_immediate"),
        c_containment: tx("tool-c_containment"),
        c_call: tx("tool-c_call"),
        c_deadline: tx("tool-c_deadline"),
        c_card: tx("tool-c_card"),
        open_card: tx("tool-open_card"),
        make_card: tx("tool-make_card"),
        fire_signal: tx("tool-fire_signal"),
        card_made: tx("tool-card_made"),
        ap_table: tx("tool-ap_table"),
        ap_builtin: tx("tool-ap_builtin"),
        ap_company: |n| {
            fx(
                "tool-ap_company",
                &[
                    ("n", n.into()),
                    ("tem", (if n.is_empty() { "nao" } else { "sim" }).into()),
                ],
            )
        },
        ap_use_builtin: tx("tool-ap_use_builtin"),
        ap_counts: |h, m, l| {
            fx(
                "tool-ap_counts",
                &[("h", h.into()), ("m", m.into()), ("l", l.into())],
            )
        },
        ap_non_monotone: |n| fx("tool-ap_non_monotone", &[("n", n.into())]),
        ap_csv_hint: tx("tool-ap_csv_hint"),
        ap_name_hint: tx("tool-ap_name_hint"),
        ap_import: tx("tool-ap_import"),
        ap_export: tx("tool-ap_export"),
        ap_need_path: tx("tool-ap_need_path"),
        ap_exported: |p| fx("tool-ap_exported", &[("p", p.into())]),
        ap_imported: tx("tool-ap_imported"),
        ap_rejected: tx("tool-ap_rejected"),
        ap_tip: tx("tool-ap_tip"),
        scales: tx("tool-scales"),
        scales_tip: tx("tool-scales_tip"),
        level: tx("tool-level"),
        scale_s: tx("tool-scale_s"),
        scale_o: tx("tool-scale_o"),
        scale_d: tx("tool-scale_d"),
        critical_from: tx("tool-critical_from"),
        critical_default: |s| fx("tool-critical_default", &[("s", s.into())]),
        fmea_headers: Box::leak(Box::new([
            tx("tool-fmea_headers-0"),
            tx("tool-fmea_headers-1"),
            tx("tool-fmea_headers-2"),
            tx("tool-fmea_headers-3"),
            tx("tool-fmea_headers-4"),
            tx("tool-fmea_headers-5"),
            tx("tool-fmea_headers-6"),
            tx("tool-fmea_headers-7"),
            tx("tool-fmea_headers-8"),
            tx("tool-fmea_headers-9"),
            tx("tool-fmea_headers-10"),
            tx("tool-fmea_headers-11"),
            tx("tool-fmea_headers-12"),
            tx("tool-fmea_headers-13"),
            tx("tool-fmea_headers-14"),
            tx("tool-fmea_headers-15"),
            tx("tool-fmea_headers-16"),
            tx("tool-fmea_headers-17"),
            tx("tool-fmea_headers-18"),
            tx("tool-fmea_headers-19"),
            tx("tool-fmea_headers-20"),
        ])),
        a3_titles: [
            tx("tool-a3_titles-0"),
            tx("tool-a3_titles-1"),
            tx("tool-a3_titles-2"),
            tx("tool-a3_titles-3"),
            tx("tool-a3_titles-4"),
            tx("tool-a3_titles-5"),
            tx("tool-a3_titles-6"),
        ],
        a3_empty: tx("tool-a3_empty"),
        a3_note: tx("tool-a3_note"),
        a3_pareto: tx("tool-a3_pareto"),
        a3_process: tx("tool-a3_process"),
        a3_verified: tx("tool-a3_verified"),
        a3_root: tx("tool-a3_root"),
        a3_fmea_high: tx("tool-a3_fmea_high"),
        a3_control: tx("tool-a3_control"),
        a3_signals: |n| fx("tool-a3_signals", &[("n", n.into())]),
    }
}

pub fn tool_name(t: Tool) -> &'static str {
    match t {
        Tool::Voc => tx("t-tool_name-voc"),
        Tool::Sipoc => "SIPOC",
        Tool::Pareto => "Pareto",
        Tool::DataCollection => tx("t-tool_name-data_collection"),
        Tool::Ishikawa => "Ishikawa",
        Tool::FiveWhys => tx("t-tool_name-five_whys"),
        Tool::CauseEffect => tx("t-tool_name-cause_effect"),
        Tool::Fmea => tx("t-tool_name-fmea"),
        Tool::ControlPlan => tx("t-tool_name-control_plan"),
        Tool::ResponsePlan => tx("t-tool_name-response_plan"),
        Tool::A3 => "A3",
        Tool::Vsm => tx("t-tool_name-vsm"),
        Tool::Selection => tx("t-tool_name-selection"),
        Tool::FiveS => tx("t-tool_name-five_s"),
        Tool::Smed => tx("t-tool_name-smed"),
        Tool::StandardWork => tx("t-tool_name-standard_work"),
        Tool::KaizenEvent => tx("t-tool_name-kaizen_event"),
        Tool::Pdca => tx("t-tool_name-pdca"),
    }
}

/// One line on how to use each tool.
pub fn tool_help(t: Tool) -> &'static str {
    match t {
        Tool::Voc => tx("t-tool_help-voc"),
        Tool::Sipoc => tx("t-tool_help-sipoc"),
        Tool::Pareto => tx("t-tool_help-pareto"),
        Tool::DataCollection => tx("t-tool_help-data_collection"),
        Tool::Ishikawa => tx("t-tool_help-ishikawa"),
        Tool::FiveWhys => tx("t-tool_help-five_whys"),
        Tool::CauseEffect => tx("t-tool_help-cause_effect"),
        Tool::Fmea => tx("t-tool_help-fmea"),
        Tool::ControlPlan => tx("t-tool_help-control_plan"),
        Tool::ResponsePlan => tx("t-tool_help-response_plan"),
        Tool::A3 => tx("t-tool_help-a3"),
        Tool::Vsm => tx("t-tool_help-vsm"),
        Tool::Selection => tx("t-tool_help-selection"),
        Tool::FiveS => tx("t-tool_help-five_s"),
        Tool::Smed => tx("t-tool_help-smed"),
        Tool::StandardWork => tx("t-tool_help-standard_work"),
        Tool::KaizenEvent => tx("t-tool_help-kaizen_event"),
        Tool::Pdca => tx("t-tool_help-pdca"),
    }
}

pub fn deliverable_status_name(s: DeliverableStatus) -> &'static str {
    match s {
        DeliverableStatus::Draft => tx("t-deliverable_status_name-draft"),
        DeliverableStatus::Done => tx("t-deliverable_status_name-done"),
    }
}

pub fn kano_name(k: Kano) -> &'static str {
    match k {
        Kano::MustBe => tx("t-kano_name-must_be"),
        Kano::Performance => tx("t-kano_name-performance"),
        Kano::Attractive => tx("t-kano_name-attractive"),
    }
}

pub fn cause_status_name(s: CauseStatus) -> &'static str {
    match s {
        CauseStatus::Open => tx("t-cause_status_name-open"),
        CauseStatus::Probable => tx("t-cause_status_name-probable"),
        CauseStatus::Verified => tx("t-cause_status_name-verified"),
        CauseStatus::RuledOut => tx("t-cause_status_name-ruled_out"),
    }
}

pub fn ap_name(a: Ap) -> &'static str {
    match a {
        Ap::High => tx("t-ap_name-high"),
        Ap::Medium => tx("t-ap_name-medium"),
        Ap::Low => tx("t-ap_name-low"),
    }
}

pub fn origin_name(o: Origin) -> &'static str {
    match o {
        Origin::Manual => tx("t-origin_name-manual"),
        Origin::Fmea => tx("t-origin_name-fmea"),
        Origin::Minutes => tx("t-origin_name-minutes"),
        Origin::Tollgate => tx("t-origin_name-tollgate"),
        Origin::Risk => tx("t-origin_name-risk"),
        Origin::ResponsePlan => tx("t-origin_name-response_plan"),
        Origin::Kaizen => tx("t-origin_name-kaizen"),
    }
}

pub fn voc_issue(i: Issue) -> &'static str {
    match i {
        Issue::NoUnit(_) => tx("t-voc_issue-no_unit"),
        Issue::NoLimit(_) => tx("t-voc_issue-no_limit"),
        Issue::LimitsReversed(_) => tx("t-voc_issue-limits_reversed"),
        Issue::TargetOutside(_) => tx("t-voc_issue-target_outside"),
        Issue::NeedWithoutDriver(_) => tx("t-voc_issue-need_without_driver"),
        Issue::DriverWithoutCtq(_) => tx("t-voc_issue-driver_without_ctq"),
        Issue::Ungrouped(_) => tx("t-voc_issue-ungrouped"),
    }
}

pub fn collection_issue(i: CollectionIssue) -> &'static str {
    match i {
        CollectionIssue::NoName => tx("t-collection_issue-no_name"),
        CollectionIssue::NoDefinition => tx("t-collection_issue-no_definition"),
        CollectionIssue::NoUnit => tx("t-collection_issue-no_unit"),
        CollectionIssue::NoSampling => tx("t-collection_issue-no_sampling"),
    }
}

pub use utsim_lss_store::user::Language;

/// The texts of the app in the five languages, embedded from `idiomas/`.
pub static TEXTOS: Catalogo = utsim_idiomas::catalogo!("seis-sigma");

/// The text of a key in the language now.
pub(crate) fn tx(chave: &'static str) -> &'static str {
    TEXTOS.t(chave)
}

/// The text of a key with named values.
pub(crate) fn fx(chave: &str, args: &[(&str, Valor)]) -> String {
    TEXTOS.f(chave, args)
}

/// The language of the family that a configured language is.
pub fn idioma(l: Language) -> Idioma {
    match l {
        Language::PtBr => Idioma::PtBr,
        Language::En => Idioma::En,
        Language::Es => Idioma::Es,
        Language::Fr => Idioma::Fr,
        Language::It => Idioma::It,
    }
}

/// The configured language of a language of the family.
pub fn language_of(i: Idioma) -> Language {
    match i {
        Idioma::PtBr => Language::PtBr,
        Idioma::En => Language::En,
        Idioma::Es => Language::Es,
        Idioma::Fr => Language::Fr,
        Idioma::It => Language::It,
    }
}

/// Sets the language of the interface (of this thread: the window's, or one test's).
pub fn set_language(l: Language) {
    utsim_idiomas::definir_nesta_thread(Some(idioma(l)));
}

/// The language of the interface.
pub fn language() -> Language {
    language_of(utsim_idiomas::atual())
}

/// A number written with a point as decimal mark, put in the language's form: a comma in
/// every language but English.
pub fn decimal_mark(s: &str) -> String {
    utsim_idiomas::marca_decimal(s)
}

/// A date on the screen: `DD/MM/AAAA`, or `AAAA-MM-DD` in English (the form no reader takes
/// for month-first or day-first). Both forms are accepted when typed.
pub trait LocalDate {
    fn local(self) -> String;
}

impl LocalDate for utsim_lss_core::date::Date {
    fn local(self) -> String {
        utsim_idiomas::data(self.year, self.month.into(), self.day.into())
    }
}

/// A month the same way: `MM/AAAA` or `AAAA-MM`.
impl LocalDate for utsim_lss_core::savings::Month {
    fn local(self) -> String {
        utsim_idiomas::mes(self.year, self.month.into())
    }
}

/// The active catalogue.
pub fn texts() -> &'static Texts {
    static CACHE: PorIdioma<Texts> = PorIdioma::new();
    CACHE.obter(construir_texts)
}

fn construir_texts() -> Texts {
    Texts {
        app_name: tx("t-app_name"),
        identity_title: tx("t-identity_title"),
        identity_intro: tx("t-identity_intro"),
        identity_name: tx("t-identity_name"),
        identity_email: tx("t-identity_email"),
        identity_note: tx("t-identity_note"),
        continue_: tx("t-continue_"),
        projects: tx("t-projects"),
        recent: tx("t-recent"),
        no_recent: tx("t-no_recent"),
        missing_folder: tx("t-missing_folder"),
        open_folder: tx("t-open_folder"),
        open: tx("t-open"),
        path_hint: tx("t-path_hint"),
        new_project: tx("t-new_project"),
        title_hint: tx("t-title_hint"),
        parent_hint: tx("t-parent_hint"),
        approach: tx("t-approach"),
        project_model: tx("t-project_model"),
        no_project_model: tx("t-no_project_model"),
        create: tx("t-create"),
        import_lssz: tx("t-import_lssz"),
        import_hint: tx("t-import_hint"),
        import_dest_hint: tx("t-import_dest_hint"),
        import: tx("t-import"),
        shared_folder_tip: tx("t-shared_folder_tip"),
        who_am_i: |name, email| {
            fx(
                "t-who_am_i",
                &[("name", name.into()), ("email", email.into())],
            )
        },
        change_identity: tx("t-change_identity"),
        nav_portfolio: tx("t-nav_portfolio"),
        nav_project: tx("t-nav_project"),
        nav_overview: tx("t-nav_overview"),
        nav_charter: tx("t-nav_charter"),
        nav_phases: tx("t-nav_phases"),
        nav_actions: tx("t-nav_actions"),
        nav_risks: tx("t-nav_risks"),
        nav_meetings: tx("t-nav_meetings"),
        nav_schedule: tx("t-nav_schedule"),
        nav_savings: tx("t-nav_savings"),
        nav_tools: tx("t-nav_tools"),
        nav_data: tx("t-nav_data"),
        nav_conflicts: tx("t-nav_conflicts"),
        close_project: tx("t-close_project"),
        coming_in: |phase| fx("t-coming_in", &[("phase", phase.into())]),
        theme: tx("t-theme"),
        theme_system: tx("t-theme_system"),
        theme_light: tx("t-theme_light"),
        theme_dark: tx("t-theme_dark"),
        language: tx("t-language"),
        current_phase: tx("t-current_phase"),
        phases: tx("t-phases"),
        actions: tx("t-actions"),
        actions_done: |done, total| {
            fx(
                "t-actions_done",
                &[("done", done.into()), ("total", total.into())],
            )
        },
        overdue: tx("t-overdue"),
        none_overdue: tx("t-none_overdue"),
        risks_open: tx("t-risks_open"),
        risks_high: tx("t-risks_high"),
        savings: tx("t-savings"),
        meetings: tx("t-meetings"),
        deadline: tx("t-deadline"),
        days_left: |d| {
            fx(
                "t-days_left",
                &[
                    (
                        "caso",
                        (match d {
                            0 => "hoje",
                            1 => "um",
                            -1 => "menos_um",
                            d if d > 1 => "futuro",
                            _ => "atraso",
                        })
                        .into(),
                    ),
                    ("abs", d.unsigned_abs().into()),
                ],
            )
        },
        no_deadline: tx("t-no_deadline"),
        conflicts: tx("t-conflicts"),
        folder: tx("t-folder"),
        operations: |n| fx("t-operations", &[("n", n.into())]),
        charter_intro: tx("t-charter_intro"),
        f_title: tx("t-f_title"),
        f_process: tx("t-f_process"),
        f_area: tx("t-f_area"),
        f_problem: tx("t-f_problem"),
        f_goal: tx("t-f_goal"),
        f_baseline: tx("t-f_baseline"),
        f_deadline: tx("t-f_deadline"),
        f_scope_in: tx("t-f_scope_in"),
        f_scope_out: tx("t-f_scope_out"),
        f_business_case: tx("t-f_business_case"),
        date_hint: tx("t-date_hint"),
        bad_date: tx("t-bad_date"),
        new_action_hint: tx("t-new_action_hint"),
        add: tx("t-add"),
        board_tip: tx("t-board_tip"),
        a_what: tx("t-a_what"),
        a_why: tx("t-a_why"),
        a_where: tx("t-a_where"),
        a_when: tx("t-a_when"),
        a_who: tx("t-a_who"),
        a_how: tx("t-a_how"),
        a_how_much: tx("t-a_how_much"),
        a_column: tx("t-a_column"),
        nobody: tx("t-nobody"),
        delete: tx("t-delete"),
        close: tx("t-close"),
        move_left: tx("t-move_left"),
        move_right: tx("t-move_right"),
        move_up: tx("t-move_up"),
        move_down: tx("t-move_down"),
        untitled: tx("t-untitled"),
        here_now: tx("t-here_now"),
        is_on: |name, screen| {
            fx(
                "t-is_on",
                &[("name", name.into()), ("screen", screen.into())],
            )
        },
        is_editing_this: |name| fx("t-is_editing_this", &[("name", name.into())]),
        conflicts_banner: |n| fx("t-conflicts_banner", &[("n", n.into())]),
        review: tx("t-review"),
        conflicts_intro: tx("t-conflicts_intro"),
        no_conflicts: tx("t-no_conflicts"),
        kept: tx("t-kept"),
        replaced: tx("t-replaced"),
        keep_this: tx("t-keep_this"),
        restore_other: tx("t-restore_other"),
        by_whom: |name| fx("t-by_whom", &[("name", name.into())]),
        pending_files: |n| fx("t-pending_files", &[("n", n.into())]),
        data_intro: tx("t-data_intro"),
        export_lssz: tx("t-export_lssz"),
        export_dest_hint: tx("t-export_dest_hint"),
        exported: |p| fx("t-exported", &[("p", p.into())]),
        imported: |p| fx("t-imported", &[("p", p.into())]),
        created: |p| fx("t-created", &[("p", p.into())]),
        error: |e| fx("t-error", &[("e", e.into())]),
        soon_portfolio: tx("t-soon_portfolio"),
        soon_phases: tx("t-soon_phases"),
        soon_risks: tx("t-soon_risks"),
        soon_meetings: tx("t-soon_meetings"),
        soon_schedule: tx("t-soon_schedule"),
        soon_savings: tx("t-soon_savings"),
        soon_data: tx("t-soon_data"),
    }
}

pub fn approach_name(a: Approach) -> &'static str {
    match a {
        Approach::DirectAction => tx("t-approach_name-direct_action"),
        Approach::Kaizen => tx("t-approach_name-kaizen"),
        Approach::A3 => "A3",
        Approach::Pdca => "PDCA",
        Approach::Dmaic => "DMAIC",
        Approach::Dmadv => "DMADV (DFSS)",
    }
}

pub fn column_name(c: Column) -> &'static str {
    match c {
        Column::Todo => tx("t-column_name-todo"),
        Column::Doing => tx("t-column_name-doing"),
        Column::Blocked => tx("t-column_name-blocked"),
        Column::Verify => tx("t-column_name-verify"),
        Column::Done => tx("t-column_name-done"),
    }
}

pub fn phase_status_name(s: PhaseStatus) -> &'static str {
    match s {
        PhaseStatus::NotStarted => tx("t-phase_status_name-not_started"),
        PhaseStatus::InProgress => tx("t-phase_status_name-in_progress"),
        PhaseStatus::InTollgate => tx("t-phase_status_name-in_tollgate"),
        PhaseStatus::Approved => tx("t-phase_status_name-approved"),
    }
}

pub fn saving_status_name(s: SavingStatus) -> &'static str {
    match s {
        SavingStatus::Estimated => tx("t-saving_status_name-estimated"),
        SavingStatus::Forecast => tx("t-saving_status_name-forecast"),
        SavingStatus::Realized => tx("t-saving_status_name-realized"),
        SavingStatus::Validated => tx("t-saving_status_name-validated"),
    }
}

/// An amount of money: `R$ 1.234,56`, `R$ 1,234.56` in English. The currency is the
/// company's, not the language's, so the symbol stays.
pub fn money(v: f64) -> String {
    utsim_idiomas::dinheiro(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatted_texts() {
        let t = texts();
        assert_eq!((t.days_left)(0), "vence hoje");
        assert_eq!((t.days_left)(5), "faltam 5 dias");
        assert_eq!((t.days_left)(-3), "atrasado 3 dias");
        assert_eq!((t.is_on)("Ana", "Ações"), "Ana está em Ações");
        assert!((t.conflicts_banner)(2).starts_with("2 conflitos"));
        assert_eq!(money(1234567.891), "R$ 1.234.567,89");
        assert_eq!(money(-12.5), "−R$ 12,50");
        assert_eq!(money(0.0), "R$ 0,00");
        for a in Approach::ALL {
            assert!(!approach_name(a).is_empty());
        }
        let tt = tool_texts();
        for tool in Tool::ALL {
            assert!(!tool_name(tool).is_empty() && !tool_help(tool).is_empty());
        }
        assert_eq!(tt.fmea_headers.len(), 21, "one per column of the FMEA grid");
        assert_eq!((tt.why_n)(3), "Por quê? 3");
        assert!((tt.gaps)(2).starts_with("2 características"));
    }

    #[test]
    fn formats_follow_the_language() {
        use crate::app::screens::{parse_number, show_number};
        use utsim_lss_core::date::Date;
        use utsim_lss_core::savings::Month;
        let d: Date = "2026-11-10".parse().unwrap();
        let m: Month = "2026-11".parse().unwrap();
        set_language(Language::En);
        assert_eq!(money(1234567.891), "R$ 1,234,567.89");
        assert_eq!(show_number(2.5), "2.5");
        assert_eq!(d.local(), "2026-11-10");
        assert_eq!(m.local(), "2026-11");
        assert_eq!(parse_number("1,234.5"), Some(1234.5));
        assert_eq!(parse_number("1,234"), Some(1234.0));
        assert_eq!(parse_number("1,5"), Some(1.5));
        set_language(Language::Es);
        assert_eq!(money(1234567.891), "R$ 1.234.567,89");
        assert_eq!(show_number(2.5), "2,5");
        assert_eq!(d.local(), "10/11/2026");
        assert_eq!(parse_number("1.234,5"), Some(1234.5));
        set_language(Language::PtBr);
        assert_eq!(m.local(), "11/2026");
        assert_eq!(parse_number("1234.5"), Some(1234.5));
    }
}

/// Texts of the stage L1 screens.
pub mod l1;
pub mod l10;
pub mod l11;
pub mod l12;
pub mod l13;
pub mod l14;
pub mod l15;
pub mod l3;
pub mod l4;
pub mod l5;
pub mod l6;
pub mod l7;
pub mod l9;
