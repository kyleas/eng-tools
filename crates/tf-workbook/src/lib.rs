use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use eng_core::units::{
    default_unit_for_dimension, parse_equation_quantity_to_si, parse_quantity_expression,
    signature_for_dimension,
};
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tf_eng::{
    SingleSolveResult, SolveRowState, SolveValidation, StudyResult, StudyRunRequest,
    StudyTargetKind, SweepAxisSpec, evaluate_single_solve, run_study_from_form,
};
use thiserror::Error;

pub const WORKBOOK_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookManifest {
    pub schema_version: u32,
    pub title: String,
    pub tabs: Vec<WorkbookTabEntry>,
    #[serde(default)]
    pub preferred_display_units: BTreeMap<String, String>,
    #[serde(default)]
    pub execution: WorkbookExecutionDefaults,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkbookExecutionDefaults {
    #[serde(default = "default_auto_run")]
    pub auto_run: bool,
}

fn default_auto_run() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookTabEntry {
    pub id: String,
    pub title: String,
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookTabFile {
    pub rows: Vec<WorkbookRow>,
}

#[derive(Debug, Clone)]
pub struct WorkbookDocument {
    pub root_dir: PathBuf,
    pub manifest: WorkbookManifest,
    pub tabs: Vec<WorkbookTab>,
}

#[derive(Debug, Clone)]
pub struct WorkbookTab {
    pub id: String,
    pub title: String,
    pub file: String,
    pub rows: Vec<WorkbookRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookRow {
    pub id: String,
    pub key: Option<String>,
    pub title: Option<String>,
    #[serde(default)]
    pub collapsed: bool,
    #[serde(default)]
    pub freeze: bool,
    #[serde(flatten)]
    pub kind: WorkbookRowKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum WorkbookRowKind {
    #[serde(alias = "narrative", alias = "markdown")]
    Text(TextRowContent),
    Constant(ConstantRowContent),
    EquationSolve(EquationSolveRowContent),
    Study(StudyRowContent),
    Plot(PlotRowContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRowContent {
    #[serde(default, alias = "text")]
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantRowContent {
    pub value: String,
    #[serde(default, alias = "dimension_hint")]
    pub dimension: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquationSolveRowContent {
    pub path_id: String,
    pub target: Option<String>,
    pub branch: Option<String>,
    #[serde(default)]
    pub inputs: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyRowContent {
    pub kind: StudyTargetKind,
    #[serde(alias = "target_key")]
    pub target_id: String,
    pub sweep_field: String,
    pub sweep: WorkbookSweepAxis,
    #[serde(default)]
    pub fixed_inputs: BTreeMap<String, String>,
    pub output_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum WorkbookSweepAxis {
    Values { values: Vec<f64> },
    Linspace { start: f64, end: f64, count: usize },
    Logspace { start: f64, end: f64, count: usize },
}

impl WorkbookSweepAxis {
    fn into_spec(self) -> SweepAxisSpec {
        match self {
            WorkbookSweepAxis::Values { values } => SweepAxisSpec::Values(values),
            WorkbookSweepAxis::Linspace { start, end, count } => {
                SweepAxisSpec::Linspace { start, end, count }
            }
            WorkbookSweepAxis::Logspace { start, end, count } => {
                SweepAxisSpec::Logspace { start, end, count }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotRowContent {
    pub source_row: String,
    pub x: String,
    pub y: String,
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    #[serde(default)]
    pub x_bounds: Option<[f64; 2]>,
    #[serde(default)]
    pub y_bounds: Option<[f64; 2]>,
    #[serde(default)]
    pub legend: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WorkbookRowState {
    Incomplete,
    Ready,
    Invalid,
    Ambiguous,
    Error,
    Ok,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookRowExecution {
    pub id: String,
    pub key: Option<String>,
    pub state: WorkbookRowState,
    #[serde(default)]
    pub messages: Vec<String>,
    pub result: Option<WorkbookRowResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum WorkbookRowResult {
    Text { content: String },
    Constant(ConstantRowResult),
    Equation(EquationRowResult),
    Study(StudyResult),
    Plot(WorkbookPlotResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantRowResult {
    pub raw: String,
    pub value: Value,
    pub normalized_si: Option<f64>,
    pub dimension: Option<String>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkbookReferenceMatch {
    pub start: usize,
    pub end: usize,
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkbookReferenceCandidate {
    pub row_id: String,
    pub key: String,
    pub row_type: String,
    pub preview: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkbookDimensionOption {
    pub key: &'static str,
    pub label: &'static str,
    pub default_unit: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquationRowResult {
    pub validation: SolveValidation,
    pub solve: SingleSolveResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookPlotSeries {
    pub name: String,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookPlotResult {
    pub source_row_id: String,
    pub x: String,
    pub y: String,
    pub title: Option<String>,
    pub x_label: String,
    pub y_label: String,
    pub x_bounds: Option<[f64; 2]>,
    pub y_bounds: Option<[f64; 2]>,
    pub legend: bool,
    pub series: Vec<WorkbookPlotSeries>,
}

const WORKBOOK_DIMENSIONS: &[WorkbookDimensionOption] = &[
    WorkbookDimensionOption {
        key: "dimensionless",
        label: "Dimensionless",
        default_unit: "1",
    },
    WorkbookDimensionOption {
        key: "ratio",
        label: "Ratio",
        default_unit: "1",
    },
    WorkbookDimensionOption {
        key: "count",
        label: "Count",
        default_unit: "1",
    },
    WorkbookDimensionOption {
        key: "gamma",
        label: "Gamma",
        default_unit: "1",
    },
    WorkbookDimensionOption {
        key: "mach",
        label: "Mach",
        default_unit: "1",
    },
    WorkbookDimensionOption {
        key: "angle",
        label: "Angle",
        default_unit: "rad",
    },
    WorkbookDimensionOption {
        key: "pressure",
        label: "Pressure",
        default_unit: "Pa",
    },
    WorkbookDimensionOption {
        key: "stress",
        label: "Stress",
        default_unit: "Pa",
    },
    WorkbookDimensionOption {
        key: "temperature",
        label: "Temperature",
        default_unit: "K",
    },
    WorkbookDimensionOption {
        key: "density",
        label: "Density",
        default_unit: "kg/m3",
    },
    WorkbookDimensionOption {
        key: "length",
        label: "Length",
        default_unit: "m",
    },
    WorkbookDimensionOption {
        key: "diameter",
        label: "Diameter",
        default_unit: "m",
    },
    WorkbookDimensionOption {
        key: "roughness",
        label: "Roughness",
        default_unit: "m",
    },
    WorkbookDimensionOption {
        key: "area",
        label: "Area",
        default_unit: "m2",
    },
    WorkbookDimensionOption {
        key: "volume",
        label: "Volume",
        default_unit: "m3",
    },
    WorkbookDimensionOption {
        key: "velocity",
        label: "Velocity",
        default_unit: "m/s",
    },
    WorkbookDimensionOption {
        key: "mass",
        label: "Mass",
        default_unit: "kg",
    },
    WorkbookDimensionOption {
        key: "mass_flow",
        label: "Mass Flow",
        default_unit: "kg/s",
    },
    WorkbookDimensionOption {
        key: "mass_flow_rate",
        label: "Mass Flow Rate",
        default_unit: "kg/s",
    },
    WorkbookDimensionOption {
        key: "volumetric_flow",
        label: "Volumetric Flow",
        default_unit: "m3/s",
    },
    WorkbookDimensionOption {
        key: "volumetric_flow_rate",
        label: "Volumetric Flow Rate",
        default_unit: "m3/s",
    },
    WorkbookDimensionOption {
        key: "dynamic_viscosity",
        label: "Dynamic Viscosity",
        default_unit: "Pa*s",
    },
    WorkbookDimensionOption {
        key: "viscosity",
        label: "Viscosity",
        default_unit: "Pa*s",
    },
    WorkbookDimensionOption {
        key: "friction_factor",
        label: "Friction Factor",
        default_unit: "1",
    },
    WorkbookDimensionOption {
        key: "efficiency",
        label: "Efficiency",
        default_unit: "1",
    },
    WorkbookDimensionOption {
        key: "branch",
        label: "Branch",
        default_unit: "1",
    },
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookTabExecution {
    pub id: String,
    pub title: String,
    pub file: String,
    pub rows: Vec<WorkbookRowExecution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookRunResult {
    pub title: String,
    pub tabs: Vec<WorkbookTabExecution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookValidation {
    pub ok: bool,
    pub messages: Vec<String>,
}

#[derive(Debug, Error)]
pub enum WorkbookError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("yaml: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("invalid workbook: {0}")]
    Invalid(String),
    #[error("execution failed: {0}")]
    Execution(String),
}

pub fn workbook_dimension_options() -> &'static [WorkbookDimensionOption] {
    WORKBOOK_DIMENSIONS
}

pub fn reference_candidates(doc: &WorkbookDocument) -> Vec<WorkbookReferenceCandidate> {
    let mut out = Vec::new();
    for tab in &doc.tabs {
        for row in &tab.rows {
            let Some(key) = row.key.as_deref().map(str::trim).filter(|k| !k.is_empty()) else {
                continue;
            };
            out.push(WorkbookReferenceCandidate {
                row_id: row.id.clone(),
                key: key.to_string(),
                row_type: match row.kind {
                    WorkbookRowKind::Text(_) => "text",
                    WorkbookRowKind::Constant(_) => "constant",
                    WorkbookRowKind::EquationSolve(_) => "equation",
                    WorkbookRowKind::Study(_) => "study",
                    WorkbookRowKind::Plot(_) => "plot",
                }
                .to_string(),
                preview: row_preview_seed(row),
            });
        }
    }
    out
}

pub fn find_reference_matches(text: &str) -> Vec<WorkbookReferenceMatch> {
    find_reference_matches_with_mode(text, true)
}

fn find_reference_matches_all(text: &str) -> Vec<WorkbookReferenceMatch> {
    find_reference_matches_with_mode(text, false)
}

fn find_reference_matches_with_mode(
    text: &str,
    ignore_backticks: bool,
) -> Vec<WorkbookReferenceMatch> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    let mut in_code = false;
    while i < bytes.len() {
        if ignore_backticks && bytes[i] == b'`' {
            in_code = !in_code;
            i += 1;
            continue;
        }
        if ignore_backticks && in_code {
            i += 1;
            continue;
        }
        if bytes[i] != b'@' {
            i += 1;
            continue;
        }
        let start = i;
        i += 1;
        let key_start = i;
        while i < bytes.len() {
            let ch = bytes[i] as char;
            if ch.is_ascii_alphanumeric() || ch == '_' {
                i += 1;
            } else {
                break;
            }
        }
        if i > key_start {
            out.push(WorkbookReferenceMatch {
                start,
                end: i,
                key: text[key_start..i].to_string(),
            });
        }
    }
    out
}

pub fn format_engineering_number(value: f64) -> String {
    if !value.is_finite() {
        return value.to_string();
    }
    let abs = value.abs();
    if abs == 0.0 {
        return "0".to_string();
    }
    if !(1.0e-3..1.0e4).contains(&abs) {
        return format!("{value:.4e}")
            .replace("e+0", "e")
            .replace("e+", "e")
            .replace("e0", "e0");
    }
    if abs >= 1000.0 {
        return format!("{value:.0}");
    }
    if abs >= 100.0 {
        return format!("{value:.1}");
    }
    if abs >= 10.0 {
        return format!("{value:.2}");
    }
    format!("{value:.3}")
}

pub fn format_scalar_display(label: Option<&str>, value_si: f64, unit: Option<&str>) -> String {
    let number = format_engineering_number(value_si);
    match (label, unit.filter(|u| !u.is_empty() && *u != "1")) {
        (Some(label), Some(unit)) => format!("{label} = {number} [{unit}]"),
        (Some(label), None) => format!("{label} = {number}"),
        (None, Some(unit)) => format!("{number} [{unit}]"),
        (None, None) => number,
    }
}

pub fn create_workbook_skeleton(
    dir: &Path,
    title: &str,
) -> Result<WorkbookDocument, WorkbookError> {
    fs::create_dir_all(dir.join("tabs"))?;
    fs::create_dir_all(dir.join("assets"))?;
    let manifest = WorkbookManifest {
        schema_version: WORKBOOK_SCHEMA_VERSION,
        title: title.to_string(),
        tabs: vec![WorkbookTabEntry {
            id: "inputs".to_string(),
            title: "Inputs".to_string(),
            file: "01_inputs.yaml".to_string(),
        }],
        preferred_display_units: BTreeMap::new(),
        execution: WorkbookExecutionDefaults::default(),
    };
    let tabs = vec![WorkbookTab {
        id: "inputs".to_string(),
        title: "Inputs".to_string(),
        file: "01_inputs.yaml".to_string(),
        rows: Vec::new(),
    }];
    let doc = WorkbookDocument {
        root_dir: dir.to_path_buf(),
        manifest,
        tabs,
    };
    save_workbook_dir(&doc)?;
    Ok(doc)
}

pub fn load_workbook_dir(dir: &Path) -> Result<WorkbookDocument, WorkbookError> {
    let manifest_path = dir.join("workbook.yaml");
    let manifest: WorkbookManifest = serde_yaml::from_str(&fs::read_to_string(&manifest_path)?)?;
    if manifest.schema_version != WORKBOOK_SCHEMA_VERSION {
        return Err(WorkbookError::Invalid(format!(
            "unsupported schema_version {}",
            manifest.schema_version
        )));
    }

    let mut tabs = Vec::new();
    for entry in &manifest.tabs {
        let tab_path = dir.join("tabs").join(&entry.file);
        let tab_file: WorkbookTabFile = serde_yaml::from_str(&fs::read_to_string(tab_path)?)?;
        tabs.push(WorkbookTab {
            id: entry.id.clone(),
            title: entry.title.clone(),
            file: entry.file.clone(),
            rows: tab_file.rows,
        });
    }

    Ok(WorkbookDocument {
        root_dir: dir.to_path_buf(),
        manifest,
        tabs,
    })
}

pub fn save_workbook_dir(doc: &WorkbookDocument) -> Result<(), WorkbookError> {
    fs::create_dir_all(doc.root_dir.join("tabs"))?;
    fs::create_dir_all(doc.root_dir.join("assets"))?;

    let mut manifest = doc.manifest.clone();
    manifest.tabs = doc
        .tabs
        .iter()
        .map(|tab| WorkbookTabEntry {
            id: tab.id.clone(),
            title: tab.title.clone(),
            file: tab.file.clone(),
        })
        .collect();

    fs::write(
        doc.root_dir.join("workbook.yaml"),
        serde_yaml::to_string(&manifest)?,
    )?;

    for tab in &doc.tabs {
        let f = WorkbookTabFile {
            rows: tab.rows.clone(),
        };
        fs::write(
            doc.root_dir.join("tabs").join(&tab.file),
            serde_yaml::to_string(&f)?,
        )?;
    }
    Ok(())
}

pub fn rename_row_key(
    doc: &mut WorkbookDocument,
    old_key: &str,
    new_key: &str,
) -> Result<usize, WorkbookError> {
    if old_key == new_key {
        return Ok(0);
    }
    for tab in &doc.tabs {
        for row in &tab.rows {
            if row.key.as_deref() == Some(new_key) {
                return Err(WorkbookError::Invalid(format!(
                    "new key '{}' already exists",
                    new_key
                )));
            }
        }
    }

    let mut found = false;
    let mut updates = 0usize;
    for tab in &mut doc.tabs {
        for row in &mut tab.rows {
            if row.key.as_deref() == Some(old_key) {
                row.key = Some(new_key.to_string());
                found = true;
                updates += 1;
            }
            updates += rewrite_refs_in_row(row, old_key, new_key);
        }
    }
    if !found {
        return Err(WorkbookError::Invalid(format!("unknown key '{}'", old_key)));
    }
    save_workbook_dir(doc)?;
    Ok(updates)
}

pub fn validate_workbook(doc: &WorkbookDocument) -> WorkbookValidation {
    let mut messages = Vec::new();

    let mut ids = BTreeSet::new();
    let mut keys = BTreeSet::new();
    let mut key_to_id = HashMap::new();

    for tab in &doc.tabs {
        for row in &tab.rows {
            if !ids.insert(row.id.clone()) {
                messages.push(format!("duplicate row id '{}'", row.id));
            }
            let trimmed_key = row.key.as_deref().map(str::trim).unwrap_or_default();
            if row_requires_key(row) && trimmed_key.is_empty() {
                messages.push(format!(
                    "row '{}' requires a non-empty key because it is referenceable",
                    row.id
                ));
            }
            if !trimmed_key.is_empty() {
                if !keys.insert(trimmed_key.to_string()) {
                    messages.push(format!("duplicate key '{}'", trimmed_key));
                }
                key_to_id.insert(trimmed_key.to_string(), row.id.clone());
            }
        }
    }

    for tab in &doc.tabs {
        for row in &tab.rows {
            for dep_key in row_dependencies(row) {
                if !key_to_id.contains_key(&dep_key) {
                    messages.push(format!(
                        "row '{}' references unknown key '{}'",
                        row.id, dep_key
                    ));
                }
            }
        }
    }

    if messages.is_empty() {
        if let Err(e) = topo_order(doc, &key_to_id) {
            messages.push(e.to_string());
        }
    }

    WorkbookValidation {
        ok: messages.is_empty(),
        messages,
    }
}

pub fn execute_workbook(
    doc: &WorkbookDocument,
    selected_tab: Option<&str>,
) -> Result<WorkbookRunResult, WorkbookError> {
    let validation = validate_workbook(doc);
    if !validation.ok {
        return Err(WorkbookError::Invalid(validation.messages.join("; ")));
    }

    let mut key_to_id = HashMap::new();
    let mut rows_by_id: HashMap<String, &WorkbookRow> = HashMap::new();
    for tab in &doc.tabs {
        for row in &tab.rows {
            rows_by_id.insert(row.id.clone(), row);
            if let Some(key) = &row.key {
                key_to_id.insert(key.clone(), row.id.clone());
            }
        }
    }

    let order = topo_order(doc, &key_to_id)?;
    let mut executed = HashMap::new();
    for row_id in order {
        let row = rows_by_id
            .get(&row_id)
            .ok_or_else(|| WorkbookError::Execution(format!("missing row '{}'", row_id)))?;
        let exec = match execute_row(row, &executed, &key_to_id) {
            Ok(exec) => exec,
            Err(error) => WorkbookRowExecution {
                id: row.id.clone(),
                key: row.key.clone(),
                state: match error {
                    WorkbookError::Invalid(_) => WorkbookRowState::Invalid,
                    WorkbookError::Execution(_) => WorkbookRowState::Error,
                    WorkbookError::Io(_) | WorkbookError::Yaml(_) => WorkbookRowState::Error,
                },
                messages: vec![error.to_string()],
                result: None,
            },
        };
        executed.insert(row_id, exec);
    }

    let mut tabs = Vec::new();
    for tab in &doc.tabs {
        if let Some(sel) = selected_tab {
            if sel != tab.id && sel != tab.title && sel != tab.file {
                continue;
            }
        }
        let mut rows = Vec::new();
        for row in &tab.rows {
            if let Some(exec) = executed.get(&row.id) {
                rows.push(exec.clone());
            }
        }
        tabs.push(WorkbookTabExecution {
            id: tab.id.clone(),
            title: tab.title.clone(),
            file: tab.file.clone(),
            rows,
        });
    }

    Ok(WorkbookRunResult {
        title: doc.manifest.title.clone(),
        tabs,
    })
}

fn topo_order(
    doc: &WorkbookDocument,
    key_to_id: &HashMap<String, String>,
) -> Result<Vec<String>, WorkbookError> {
    let mut graph = DiGraph::<String, ()>::new();
    let mut nodes = HashMap::new();
    for tab in &doc.tabs {
        for row in &tab.rows {
            let ix = graph.add_node(row.id.clone());
            nodes.insert(row.id.clone(), ix);
        }
    }

    for tab in &doc.tabs {
        for row in &tab.rows {
            let row_ix = *nodes.get(&row.id).ok_or_else(|| {
                WorkbookError::Execution(format!("missing row node '{}'", row.id))
            })?;
            for dep_key in row_dependencies(row) {
                let dep_id = key_to_id.get(&dep_key).ok_or_else(|| {
                    WorkbookError::Invalid(format!(
                        "row '{}' references unknown key '{}'",
                        row.id, dep_key
                    ))
                })?;
                let dep_ix = *nodes.get(dep_id).ok_or_else(|| {
                    WorkbookError::Execution(format!("missing dep node '{}'", dep_id))
                })?;
                graph.add_edge(dep_ix, row_ix, ());
            }
        }
    }

    let sorted = toposort(&graph, None).map_err(|c| {
        WorkbookError::Invalid(format!(
            "dependency cycle detected at row '{}'",
            graph[c.node_id()]
        ))
    })?;
    Ok(sorted.into_iter().map(|ix| graph[ix].clone()).collect())
}

fn execute_row(
    row: &WorkbookRow,
    executed: &HashMap<String, WorkbookRowExecution>,
    key_to_id: &HashMap<String, String>,
) -> Result<WorkbookRowExecution, WorkbookError> {
    let mut out = WorkbookRowExecution {
        id: row.id.clone(),
        key: row.key.clone(),
        state: WorkbookRowState::Ready,
        messages: Vec::new(),
        result: None,
    };

    if row.freeze {
        out.messages
            .push("row is frozen; execution skipped".to_string());
        return Ok(out);
    }

    match &row.kind {
        WorkbookRowKind::Text(c) => {
            out.state = WorkbookRowState::Ok;
            out.result = Some(WorkbookRowResult::Text {
                content: c.content.clone(),
            });
        }
        WorkbookRowKind::Constant(c) => match parse_constant(c, executed, key_to_id) {
            Ok(r) => {
                out.state = WorkbookRowState::Ok;
                out.result = Some(WorkbookRowResult::Constant(r));
            }
            Err(e) => {
                out.state = WorkbookRowState::Invalid;
                out.messages.push(e);
            }
        },
        WorkbookRowKind::EquationSolve(c) => {
            let mut raw_inputs = BTreeMap::new();
            for (k, expr) in &c.inputs {
                let v = eval_expr_to_value(expr, executed, key_to_id)?;
                raw_inputs.insert(k.clone(), value_to_raw_input(&v));
            }
            if let Some(branch) = &c.branch {
                raw_inputs.insert("branch".to_string(), branch.clone());
            }
            let (validation, maybe_result) = evaluate_single_solve(
                StudyTargetKind::Equation,
                &c.path_id,
                &raw_inputs,
                c.target.as_deref(),
            )
            .map_err(|e| WorkbookError::Execution(e.to_string()))?;

            if let Some(solve) = maybe_result {
                out.state = WorkbookRowState::Ok;
                out.result = Some(WorkbookRowResult::Equation(EquationRowResult {
                    validation,
                    solve,
                }));
            } else {
                out.state = map_solve_state(&validation.state);
                out.messages = validation.blocking_reasons.clone();
            }
        }
        WorkbookRowKind::Study(c) => {
            let mut inputs = Map::new();
            for (k, expr) in &c.fixed_inputs {
                inputs.insert(k.clone(), eval_expr_to_value(expr, executed, key_to_id)?);
            }
            let req = StudyRunRequest {
                target_kind: c.kind.clone(),
                target_id: c.target_id.clone(),
                sweep_field: c.sweep_field.clone(),
                axis: c.sweep.clone().into_spec(),
                inputs,
                output_key: Some(c.output_key.clone()),
            };
            match run_study_from_form(req) {
                Ok(study) => {
                    out.state = WorkbookRowState::Ok;
                    out.result = Some(WorkbookRowResult::Study(study));
                }
                Err(e) => {
                    out.state = WorkbookRowState::Error;
                    out.messages.push(e.to_string());
                }
            }
        }
        WorkbookRowKind::Plot(c) => {
            let source_id = resolve_ref_to_id(&c.source_row, key_to_id).ok_or_else(|| {
                WorkbookError::Execution(format!("plot source '{}' not found", c.source_row))
            })?;
            let source_exec = executed.get(&source_id).ok_or_else(|| {
                WorkbookError::Execution(format!("plot source row '{}' not executed", source_id))
            })?;
            let plot = build_plot_from_source(c, source_exec)?;
            out.state = WorkbookRowState::Ok;
            out.result = Some(WorkbookRowResult::Plot(plot));
        }
    }

    Ok(out)
}

fn build_plot_from_source(
    content: &PlotRowContent,
    source: &WorkbookRowExecution,
) -> Result<WorkbookPlotResult, WorkbookError> {
    let Some(result) = &source.result else {
        return Err(WorkbookError::Execution(
            "plot source has no result".to_string(),
        ));
    };
    let WorkbookRowResult::Study(study) = result else {
        return Err(WorkbookError::Invalid(
            "plot source must reference a study row".to_string(),
        ));
    };

    let x_idx = study
        .table
        .columns
        .iter()
        .position(|c| c == &content.x)
        .ok_or_else(|| WorkbookError::Invalid(format!("plot x '{}' not found", content.x)))?;
    let y_idx = study
        .table
        .columns
        .iter()
        .position(|c| c == &content.y)
        .ok_or_else(|| WorkbookError::Invalid(format!("plot y '{}' not found", content.y)))?;

    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for row in &study.table.rows {
        let x = row.get(x_idx).and_then(|s| s.parse::<f64>().ok());
        let y = row.get(y_idx).and_then(|s| s.parse::<f64>().ok());
        if let (Some(x), Some(y)) = (x, y) {
            xs.push(x);
            ys.push(y);
        }
    }

    Ok(WorkbookPlotResult {
        source_row_id: source.id.clone(),
        x: content.x.clone(),
        y: content.y.clone(),
        title: content.title.clone(),
        x_label: content.x_label.clone().unwrap_or_else(|| content.x.clone()),
        y_label: content.y_label.clone().unwrap_or_else(|| content.y.clone()),
        x_bounds: content.x_bounds,
        y_bounds: content.y_bounds,
        legend: content.legend.unwrap_or(true),
        series: vec![WorkbookPlotSeries {
            name: content
                .title
                .clone()
                .unwrap_or_else(|| format!("{} vs {}", content.y, content.x)),
            x: xs,
            y: ys,
        }],
    })
}

fn parse_constant(
    content: &ConstantRowContent,
    executed: &HashMap<String, WorkbookRowExecution>,
    key_to_id: &HashMap<String, String>,
) -> Result<ConstantRowResult, String> {
    let raw = content.value.trim();
    if raw.is_empty() {
        return Err("constant is empty".to_string());
    }

    let dimension = non_empty_trimmed(content.dimension.as_deref()).map(str::to_string);
    let normalized_dimension = dimension
        .as_deref()
        .and_then(normalized_dimension_key)
        .map(str::to_string);
    let resolved =
        evaluate_numeric_expression(raw, executed, key_to_id, normalized_dimension.as_deref())
            .map_err(|e| format!("invalid constant '{}': {}", raw, e))?;

    let explicit_sig = normalized_dimension
        .as_deref()
        .and_then(|dim| signature_for_dimension(dim).ok());
    let inferred_dimension = dimension
        .clone()
        .or_else(|| infer_dimension_from_signature(resolved.signature));

    if let (Some(dim), Some(sig)) = (&dimension, explicit_sig)
        && sig != resolved.signature
    {
        return Err(format!(
            "dimension '{}' conflicts with parsed units/expression",
            dim
        ));
    }

    let display_unit = inferred_dimension
        .as_deref()
        .and_then(normalized_dimension_key)
        .and_then(default_unit_for_dimension)
        .map(str::to_string);

    Ok(ConstantRowResult {
        raw: raw.to_string(),
        value: Value::from(resolved.value_si),
        normalized_si: Some(resolved.value_si),
        dimension: inferred_dimension,
        unit: display_unit,
    })
}

fn eval_expr_to_value(
    expr: &str,
    executed: &HashMap<String, WorkbookRowExecution>,
    key_to_id: &HashMap<String, String>,
) -> Result<Value, WorkbookError> {
    if let Ok(resolved) = evaluate_numeric_expression(expr, executed, key_to_id, None) {
        return Ok(Value::from(resolved.value_si));
    }
    if expr.eq_ignore_ascii_case("true") || expr.eq_ignore_ascii_case("false") {
        return Ok(Value::from(expr.eq_ignore_ascii_case("true")));
    }
    Ok(Value::from(expr.to_string()))
}

struct ResolvedExpression {
    value_si: f64,
    signature: eng_core::units::typed::DimensionSignature,
}

fn evaluate_numeric_expression(
    expr: &str,
    executed: &HashMap<String, WorkbookRowExecution>,
    key_to_id: &HashMap<String, String>,
    expected_dimension: Option<&str>,
) -> Result<ResolvedExpression, WorkbookError> {
    let trimmed = expr.trim();
    if trimmed.is_empty() {
        return Err(WorkbookError::Invalid("empty expression".to_string()));
    }

    let expanded = expand_expression_refs_and_constants(trimmed, executed, key_to_id)?;
    if let Some(dim) = expected_dimension {
        let signature =
            signature_for_dimension(dim).map_err(|e| WorkbookError::Invalid(e.to_string()))?;
        let value_si = if let Ok(v) = expanded.trim().parse::<f64>() {
            v
        } else if let Ok(parsed) = parse_quantity_expression(&expanded) {
            if parsed.signature == signature
                || parsed.signature == eng_core::units::typed::DimensionSignature::dimless()
            {
                parsed.value_si
            } else {
                return Err(WorkbookError::Invalid(format!(
                    "expression units conflicts with expected dimension '{}'",
                    dim
                )));
            }
        } else {
            parse_equation_quantity_to_si(dim, &expanded)
                .map_err(|e| WorkbookError::Invalid(e.to_string()))?
        };
        return Ok(ResolvedExpression {
            value_si,
            signature,
        });
    }

    let evaluated =
        parse_quantity_expression(&expanded).map_err(|e| WorkbookError::Invalid(e.to_string()))?;
    Ok(ResolvedExpression {
        value_si: evaluated.value_si,
        signature: evaluated.signature,
    })
}

fn expand_expression_refs_and_constants(
    expr: &str,
    executed: &HashMap<String, WorkbookRowExecution>,
    key_to_id: &HashMap<String, String>,
) -> Result<String, WorkbookError> {
    let mut expanded = replace_reference_tokens(expr, |key| {
        let ref_id = key_to_id
            .get(key)
            .ok_or_else(|| WorkbookError::Execution(format!("unknown key '{}'", key)))?;
        let row = executed.get(ref_id).ok_or_else(|| {
            WorkbookError::Execution(format!("referenced row '{}' has no result", ref_id))
        })?;
        let scalar = extract_scalar_value(row).ok_or_else(|| {
            WorkbookError::Execution(format!("referenced row '{}' is not scalar", ref_id))
        })?;
        scalar
            .as_f64()
            .map(format_engineering_number)
            .ok_or_else(|| {
                WorkbookError::Execution(format!("referenced row '{}' is not numeric", ref_id))
            })
    })?;
    expanded = rewrite_builtin_constants(&expanded);
    expand_abs_function(&expanded, executed, key_to_id)
}

fn rewrite_builtin_constants(expr: &str) -> String {
    let mut out = String::new();
    let chars = expr.chars().collect::<Vec<_>>();
    let mut i = 0usize;
    while i < chars.len() {
        let ch = chars[i];
        if ch.is_ascii_alphabetic() || ch == '_' {
            let start = i;
            i += 1;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let ident = chars[start..i].iter().collect::<String>();
            match ident.as_str() {
                "pi" | "PI" | "Pi" => out.push_str(&std::f64::consts::PI.to_string()),
                _ => out.push_str(&ident),
            }
        } else {
            out.push(ch);
            i += 1;
        }
    }
    out
}

fn expand_abs_function(
    expr: &str,
    executed: &HashMap<String, WorkbookRowExecution>,
    key_to_id: &HashMap<String, String>,
) -> Result<String, WorkbookError> {
    let mut current = expr.to_string();
    while let Some(start) = current.find("abs(") {
        let open = start + 3;
        let close = find_matching_paren(&current, open)
            .ok_or_else(|| WorkbookError::Invalid("unclosed abs(...) expression".to_string()))?;
        let inner = &current[(open + 1)..close];
        let resolved = evaluate_numeric_expression(inner, executed, key_to_id, None)?;
        current.replace_range(
            start..=close,
            &format_engineering_number(resolved.value_si.abs()),
        );
    }
    Ok(current)
}

fn find_matching_paren(text: &str, open_index: usize) -> Option<usize> {
    let chars = text.chars().collect::<Vec<_>>();
    let mut depth = 0i32;
    for (idx, ch) in chars.iter().enumerate().skip(open_index) {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(idx);
                }
            }
            _ => {}
        }
    }
    None
}

fn extract_scalar_value(row: &WorkbookRowExecution) -> Option<Value> {
    match row.result.as_ref()? {
        WorkbookRowResult::Constant(c) => Some(c.value.clone()),
        WorkbookRowResult::Equation(e) => Some(e.solve.value.clone()),
        _ => None,
    }
}

fn value_to_raw_input(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        _ => serde_json::to_string(v).unwrap_or_default(),
    }
}

fn map_solve_state(state: &SolveRowState) -> WorkbookRowState {
    match state {
        SolveRowState::Incomplete => WorkbookRowState::Incomplete,
        SolveRowState::Ready => WorkbookRowState::Ready,
        SolveRowState::Invalid => WorkbookRowState::Invalid,
        SolveRowState::Ambiguous => WorkbookRowState::Ambiguous,
        SolveRowState::Unsupported => WorkbookRowState::Invalid,
        SolveRowState::Error => WorkbookRowState::Error,
        SolveRowState::Success | SolveRowState::Validating => WorkbookRowState::Ok,
    }
}

fn row_dependencies(row: &WorkbookRow) -> Vec<String> {
    let mut deps = Vec::new();
    match &row.kind {
        WorkbookRowKind::Text(c) => deps.extend(extract_reference_keys(&c.content)),
        WorkbookRowKind::Constant(c) => deps.extend(extract_reference_keys(&c.value)),
        WorkbookRowKind::EquationSolve(c) => {
            for expr in c.inputs.values() {
                deps.extend(extract_reference_keys(expr));
            }
        }
        WorkbookRowKind::Study(c) => {
            for expr in c.fixed_inputs.values() {
                deps.extend(extract_reference_keys(expr));
            }
        }
        WorkbookRowKind::Plot(c) => {
            deps.extend(extract_reference_keys(&c.source_row));
        }
    }
    deps.sort();
    deps.dedup();
    deps
}

pub fn row_requires_key(row: &WorkbookRow) -> bool {
    match row.kind {
        WorkbookRowKind::Text(_) => false,
        WorkbookRowKind::Constant(_)
        | WorkbookRowKind::EquationSolve(_)
        | WorkbookRowKind::Study(_)
        | WorkbookRowKind::Plot(_) => true,
    }
}

fn parse_ref_key(expr: &str) -> Option<&str> {
    let t = expr.trim();
    if let Some(k) = t.strip_prefix('@') {
        let k = k.trim();
        if !k.is_empty() {
            return Some(k);
        }
    }
    None
}

fn extract_reference_keys(text: &str) -> Vec<String> {
    find_reference_matches(text)
        .into_iter()
        .map(|m| m.key)
        .collect()
}

fn replace_reference_tokens(
    text: &str,
    mut resolver: impl FnMut(&str) -> Result<String, WorkbookError>,
) -> Result<String, WorkbookError> {
    let matches = find_reference_matches(text);
    if matches.is_empty() {
        return Ok(text.to_string());
    }
    let mut out = String::with_capacity(text.len());
    let mut cursor = 0usize;
    for m in matches {
        out.push_str(&text[cursor..m.start]);
        out.push_str(&resolver(&m.key)?);
        cursor = m.end;
    }
    out.push_str(&text[cursor..]);
    Ok(out)
}

fn rewrite_reference_tokens_in_place(text: &mut String, old: &str, new: &str) -> usize {
    let matches = find_reference_matches_all(text);
    if matches.is_empty() {
        return 0;
    }
    let new_ref = format!("@{new}");
    let mut out = String::with_capacity(text.len());
    let mut cursor = 0usize;
    let mut count = 0usize;
    for m in matches {
        out.push_str(&text[cursor..m.start]);
        if m.key == old {
            out.push_str(&new_ref);
            count += 1;
        } else {
            out.push_str(&text[m.start..m.end]);
        }
        cursor = m.end;
    }
    out.push_str(&text[cursor..]);
    *text = out;
    count
}

fn infer_dimension_from_signature(
    signature: eng_core::units::typed::DimensionSignature,
) -> Option<String> {
    for key in [
        "pressure",
        "temperature",
        "density",
        "length",
        "area",
        "volume",
        "velocity",
        "mass",
        "mass_flow",
        "volumetric_flow",
        "dynamic_viscosity",
        "dimensionless",
        "angle",
    ] {
        if normalized_dimension_key(key).and_then(|dim| signature_for_dimension(dim).ok())
            == Some(signature)
        {
            return Some(key.to_string());
        }
    }
    None
}

fn row_preview_seed(row: &WorkbookRow) -> String {
    match &row.kind {
        WorkbookRowKind::Text(c) => preview_seed(&c.content),
        WorkbookRowKind::Constant(c) => preview_seed(&c.value),
        WorkbookRowKind::EquationSolve(c) => c.path_id.clone(),
        WorkbookRowKind::Study(c) => c.target_id.clone(),
        WorkbookRowKind::Plot(c) => c
            .title
            .as_deref()
            .map(str::to_string)
            .unwrap_or_else(|| format!("{} vs {}", c.y, c.x)),
    }
}

fn preview_seed(text: &str) -> String {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(|line| line.chars().take(64).collect())
        .unwrap_or_default()
}

fn non_empty_trimmed(text: Option<&str>) -> Option<&str> {
    text.map(str::trim).filter(|s| !s.is_empty())
}

fn normalized_dimension_key(dimension: &str) -> Option<&str> {
    match dimension.trim() {
        "" => None,
        "gamma" | "mach" | "count" | "efficiency" | "branch" => Some("dimensionless"),
        "mass_flow" => Some("mass_flow_rate"),
        "volumetric_flow" => Some("volumetric_flow_rate"),
        other => Some(other),
    }
}

fn resolve_ref_to_id(expr: &str, key_to_id: &HashMap<String, String>) -> Option<String> {
    parse_ref_key(expr).and_then(|k| key_to_id.get(k).cloned())
}

fn rewrite_refs_in_row(row: &mut WorkbookRow, old: &str, new: &str) -> usize {
    let mut count = 0usize;
    let mut rewrite = |expr: &mut String| {
        count += rewrite_reference_tokens_in_place(expr, old, new);
    };
    match &mut row.kind {
        WorkbookRowKind::Text(c) => rewrite(&mut c.content),
        WorkbookRowKind::Constant(c) => rewrite(&mut c.value),
        WorkbookRowKind::EquationSolve(c) => {
            for expr in c.inputs.values_mut() {
                rewrite(expr);
            }
        }
        WorkbookRowKind::Study(c) => {
            for expr in c.fixed_inputs.values_mut() {
                rewrite(expr);
            }
        }
        WorkbookRowKind::Plot(c) => rewrite(&mut c.source_row),
    }
    count
}

pub fn run_result_to_csv_map(run: &WorkbookRunResult) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let mut summary = String::from("tab,row_id,row_key,state,message\n");
    for tab in &run.tabs {
        for row in &tab.rows {
            let msg = row.messages.join(" | ").replace(',', " ");
            summary.push_str(&format!(
                "{},{},{},{:?},{}\n",
                tab.title,
                row.id,
                row.key.clone().unwrap_or_default(),
                row.state,
                msg
            ));
            if let Some(WorkbookRowResult::Study(study)) = &row.result {
                let mut csv = String::new();
                csv.push_str(&study.table.columns.join(","));
                csv.push('\n');
                for r in &study.table.rows {
                    csv.push_str(&r.join(","));
                    csv.push('\n');
                }
                out.insert(
                    format!(
                        "{}__{}.csv",
                        sanitize_name(&tab.title),
                        sanitize_name(row.key.as_deref().unwrap_or(&row.id))
                    ),
                    csv,
                );
            }
        }
    }
    out.insert("summary.csv".to_string(), summary);
    out
}

fn sanitize_name(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn sample_doc(root: &Path) -> WorkbookDocument {
        WorkbookDocument {
            root_dir: root.to_path_buf(),
            manifest: WorkbookManifest {
                schema_version: 1,
                title: "Sample".to_string(),
                tabs: vec![WorkbookTabEntry {
                    id: "analysis".to_string(),
                    title: "Analysis".to_string(),
                    file: "01_analysis.yaml".to_string(),
                }],
                preferred_display_units: BTreeMap::new(),
                execution: WorkbookExecutionDefaults::default(),
            },
            tabs: vec![WorkbookTab {
                id: "analysis".to_string(),
                title: "Analysis".to_string(),
                file: "01_analysis.yaml".to_string(),
                rows: Vec::new(),
            }],
        }
    }

    #[test]
    fn schema_roundtrip_load_save() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        doc.tabs[0].rows.push(WorkbookRow {
            id: "r1".to_string(),
            key: Some("note".to_string()),
            title: None,
            collapsed: false,
            freeze: false,
            kind: WorkbookRowKind::Text(TextRowContent {
                content: "hello".to_string(),
            }),
        });
        save_workbook_dir(&doc).expect("save");
        let loaded = load_workbook_dir(dir.path()).expect("load");
        assert_eq!(loaded.tabs[0].rows.len(), 1);
        assert!(matches!(
            loaded.tabs[0].rows[0].kind,
            WorkbookRowKind::Text(_)
        ));
    }

    #[test]
    fn missing_reference_reports_validation_error() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        doc.tabs[0].rows.push(WorkbookRow {
            id: "eq1".to_string(),
            key: Some("eq".to_string()),
            title: None,
            collapsed: false,
            freeze: false,
            kind: WorkbookRowKind::EquationSolve(EquationSolveRowContent {
                path_id: "structures.hoop_stress".to_string(),
                target: None,
                branch: None,
                inputs: BTreeMap::from([
                    ("P".to_string(), "@p".to_string()),
                    ("r".to_string(), "1.2 in".to_string()),
                    ("t".to_string(), "0.12 in".to_string()),
                ]),
            }),
        });
        let v = validate_workbook(&doc);
        assert!(!v.ok);
    }

    #[test]
    fn rename_rewrites_references_and_keeps_id() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        doc.tabs[0].rows = vec![
            WorkbookRow {
                id: "c1".to_string(),
                key: Some("p".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Constant(ConstantRowContent {
                    value: "500 psia".to_string(),
                    dimension: None,
                }),
            },
            WorkbookRow {
                id: "e1".to_string(),
                key: Some("sigma".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::EquationSolve(EquationSolveRowContent {
                    path_id: "structures.hoop_stress".to_string(),
                    target: None,
                    branch: None,
                    inputs: BTreeMap::from([
                        ("P".to_string(), "@p".to_string()),
                        ("r".to_string(), "1.2 in".to_string()),
                        ("t".to_string(), "0.12 in".to_string()),
                    ]),
                }),
            },
        ];
        save_workbook_dir(&doc).expect("save");
        let n = rename_row_key(&mut doc, "p", "pressure").expect("rename");
        assert!(n > 0);
        assert_eq!(doc.tabs[0].rows[0].id, "c1");
    }

    #[test]
    fn cycle_detection_reports_error() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        doc.tabs[0].rows = vec![
            WorkbookRow {
                id: "a".to_string(),
                key: Some("a".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::EquationSolve(EquationSolveRowContent {
                    path_id: "structures.hoop_stress".to_string(),
                    target: Some("sigma_h".to_string()),
                    branch: None,
                    inputs: BTreeMap::from([
                        ("P".to_string(), "@b".to_string()),
                        ("r".to_string(), "1.0 in".to_string()),
                        ("t".to_string(), "0.1 in".to_string()),
                    ]),
                }),
            },
            WorkbookRow {
                id: "b".to_string(),
                key: Some("b".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::EquationSolve(EquationSolveRowContent {
                    path_id: "structures.hoop_stress".to_string(),
                    target: Some("sigma_h".to_string()),
                    branch: None,
                    inputs: BTreeMap::from([
                        ("P".to_string(), "@a".to_string()),
                        ("r".to_string(), "1.0 in".to_string()),
                        ("t".to_string(), "0.1 in".to_string()),
                    ]),
                }),
            },
        ];
        let v = validate_workbook(&doc);
        assert!(!v.ok);
        assert!(v.messages.iter().any(|m| m.contains("cycle")));
    }

    #[test]
    fn execution_works_for_constant_equation_study_plot_chain() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        doc.tabs[0].rows = vec![
            WorkbookRow {
                id: "p1".to_string(),
                key: Some("p".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Constant(ConstantRowContent {
                    value: "500 psia".to_string(),
                    dimension: Some("pressure".to_string()),
                }),
            },
            WorkbookRow {
                id: "eq1".to_string(),
                key: Some("sigma".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::EquationSolve(EquationSolveRowContent {
                    path_id: "structures.hoop_stress".to_string(),
                    target: None,
                    branch: None,
                    inputs: BTreeMap::from([
                        ("P".to_string(), "@p".to_string()),
                        ("r".to_string(), "1.2 in".to_string()),
                        ("t".to_string(), "0.12 in".to_string()),
                    ]),
                }),
            },
            WorkbookRow {
                id: "st1".to_string(),
                key: Some("study".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Study(StudyRowContent {
                    kind: StudyTargetKind::Device,
                    target_id: "normal_shock_calc".to_string(),
                    sweep_field: "input_value".to_string(),
                    sweep: WorkbookSweepAxis::Linspace {
                        start: 1.2,
                        end: 2.0,
                        count: 6,
                    },
                    fixed_inputs: BTreeMap::from([
                        ("input_kind".to_string(), "m1".to_string()),
                        ("target_kind".to_string(), "p2_p1".to_string()),
                        ("gamma".to_string(), "1.4".to_string()),
                    ]),
                    output_key: "value".to_string(),
                }),
            },
            WorkbookRow {
                id: "pl1".to_string(),
                key: Some("plot".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Plot(PlotRowContent {
                    source_row: "@study".to_string(),
                    x: "input_value".to_string(),
                    y: "value".to_string(),
                    title: Some("Isentropic".to_string()),
                    x_label: None,
                    y_label: None,
                    x_bounds: None,
                    y_bounds: None,
                    legend: None,
                }),
            },
        ];

        let run = execute_workbook(&doc, None).expect("execute");
        let rows = &run.tabs[0].rows;
        assert!(matches!(rows[1].state, WorkbookRowState::Ok));
        assert!(matches!(rows[2].result, Some(WorkbookRowResult::Study(_))));
        assert!(matches!(rows[3].result, Some(WorkbookRowResult::Plot(_))));
    }

    #[test]
    fn plot_missing_source_is_reported() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        doc.tabs[0].rows.push(WorkbookRow {
            id: "pl1".to_string(),
            key: Some("plot".to_string()),
            title: None,
            collapsed: false,
            freeze: false,
            kind: WorkbookRowKind::Plot(PlotRowContent {
                source_row: "@missing".to_string(),
                x: "x".to_string(),
                y: "y".to_string(),
                title: None,
                x_label: None,
                y_label: None,
                x_bounds: None,
                y_bounds: None,
                legend: None,
            }),
        });
        let v = validate_workbook(&doc);
        assert!(!v.ok);
        assert!(
            v.messages
                .iter()
                .any(|m| m.contains("unknown key 'missing'"))
        );
    }

    #[test]
    fn collapsed_and_freeze_roundtrip_persisted() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        doc.tabs[0].rows.push(WorkbookRow {
            id: "r1".to_string(),
            key: Some("k".to_string()),
            title: None,
            collapsed: true,
            freeze: true,
            kind: WorkbookRowKind::Text(TextRowContent {
                content: "hello".to_string(),
            }),
        });
        save_workbook_dir(&doc).expect("save");
        let loaded = load_workbook_dir(dir.path()).expect("load");
        assert!(loaded.tabs[0].rows[0].collapsed);
        assert!(loaded.tabs[0].rows[0].freeze);
    }

    #[test]
    fn text_row_without_key_is_valid() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        doc.tabs[0].rows.push(WorkbookRow {
            id: "txt1".to_string(),
            key: None,
            title: None,
            collapsed: false,
            freeze: false,
            kind: WorkbookRowKind::Text(TextRowContent {
                content: "notes".to_string(),
            }),
        });
        let validation = validate_workbook(&doc);
        assert!(validation.ok, "{:?}", validation.messages);
    }

    #[test]
    fn constant_without_key_is_invalid() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        doc.tabs[0].rows.push(WorkbookRow {
            id: "c1".to_string(),
            key: None,
            title: None,
            collapsed: false,
            freeze: false,
            kind: WorkbookRowKind::Constant(ConstantRowContent {
                value: "1.4".to_string(),
                dimension: None,
            }),
        });
        let validation = validate_workbook(&doc);
        assert!(!validation.ok);
        assert!(
            validation
                .messages
                .iter()
                .any(|m| m.contains("requires a non-empty key"))
        );
    }

    #[test]
    fn duplicate_keys_are_invalid_after_trim() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        for id in ["a", "b"] {
            doc.tabs[0].rows.push(WorkbookRow {
                id: id.to_string(),
                key: Some(" gamma ".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Constant(ConstantRowContent {
                    value: "1.4".to_string(),
                    dimension: None,
                }),
            });
        }
        let validation = validate_workbook(&doc);
        assert!(!validation.ok);
        assert!(
            validation
                .messages
                .iter()
                .any(|m| m.contains("duplicate key"))
        );
    }

    #[test]
    fn row_order_roundtrip_persists() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        doc.tabs[0].rows = vec![
            WorkbookRow {
                id: "r2".to_string(),
                key: Some("b".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Text(TextRowContent {
                    content: "b".to_string(),
                }),
            },
            WorkbookRow {
                id: "r1".to_string(),
                key: Some("a".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Text(TextRowContent {
                    content: "a".to_string(),
                }),
            },
        ];
        save_workbook_dir(&doc).expect("save");
        let loaded = load_workbook_dir(dir.path()).expect("load");
        let ids = loaded.tabs[0]
            .rows
            .iter()
            .map(|r| r.id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(ids, vec!["r2", "r1"]);
    }

    #[test]
    fn text_rows_roundtrip_without_legacy_render_flags() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        doc.tabs[0].rows.push(WorkbookRow {
            id: "t1".to_string(),
            key: None,
            title: None,
            collapsed: false,
            freeze: false,
            kind: WorkbookRowKind::Text(TextRowContent {
                content: "# heading".to_string(),
            }),
        });
        save_workbook_dir(&doc).expect("save");
        let loaded = load_workbook_dir(dir.path()).expect("load");
        match &loaded.tabs[0].rows[0].kind {
            WorkbookRowKind::Text(n) => assert_eq!(n.content, "# heading"),
            other => panic!("expected text, got {other:?}"),
        }
    }

    #[test]
    fn text_rows_allow_title_without_affecting_content_roundtrip() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        doc.tabs[0].rows.push(WorkbookRow {
            id: "t1".to_string(),
            key: None,
            title: Some("legacy".to_string()),
            collapsed: false,
            freeze: false,
            kind: WorkbookRowKind::Text(TextRowContent {
                content: "plain text".to_string(),
            }),
        });
        save_workbook_dir(&doc).expect("save");
        let loaded = load_workbook_dir(dir.path()).expect("load");
        match &loaded.tabs[0].rows[0].kind {
            WorkbookRowKind::Text(n) => assert_eq!(n.content, "plain text"),
            other => panic!("expected text, got {other:?}"),
        }
    }

    #[test]
    fn workbook_tabs_roundtrip_with_id_title_and_file() {
        let dir = tempdir().expect("temp");
        let doc = sample_doc(dir.path());
        save_workbook_dir(&doc).expect("save");
        let loaded = load_workbook_dir(dir.path()).expect("load");
        assert_eq!(loaded.tabs[0].id, "analysis");
        assert_eq!(loaded.tabs[0].title, "Analysis");
        assert_eq!(loaded.tabs[0].file, "01_analysis.yaml");
    }

    #[test]
    fn only_at_key_reference_syntax_is_accepted() {
        assert_eq!(parse_ref_key("@gamma"), Some("gamma"));
        assert_eq!(parse_ref_key("ref:gamma"), None);
    }

    #[test]
    fn plot_rows_derive_default_labels_and_legend() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        doc.tabs[0].rows = vec![
            WorkbookRow {
                id: "study".to_string(),
                key: Some("study".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Study(StudyRowContent {
                    kind: StudyTargetKind::Device,
                    target_id: "normal_shock_calc".to_string(),
                    sweep_field: "input_value".to_string(),
                    sweep: WorkbookSweepAxis::Linspace {
                        start: 1.2,
                        end: 2.0,
                        count: 6,
                    },
                    fixed_inputs: BTreeMap::from([
                        ("input_kind".to_string(), "m1".to_string()),
                        ("target_kind".to_string(), "p2_p1".to_string()),
                        ("gamma".to_string(), "1.4".to_string()),
                    ]),
                    output_key: "value".to_string(),
                }),
            },
            WorkbookRow {
                id: "plot".to_string(),
                key: Some("plot".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Plot(PlotRowContent {
                    source_row: "@study".to_string(),
                    x: "input_value".to_string(),
                    y: "value".to_string(),
                    title: None,
                    x_label: None,
                    y_label: None,
                    x_bounds: None,
                    y_bounds: None,
                    legend: None,
                }),
            },
        ];
        let run = execute_workbook(&doc, None).expect("run");
        let plot = match &run.tabs[0].rows[1].result {
            Some(WorkbookRowResult::Plot(plot)) => plot,
            other => panic!("expected plot result, got {other:?}"),
        };
        assert_eq!(plot.x_label, "input_value");
        assert_eq!(plot.y_label, "value");
        assert!(plot.legend);
    }

    #[test]
    fn reference_candidates_include_referenceable_rows() {
        let dir = tempdir().expect("temp");
        let mut doc = sample_doc(dir.path());
        doc.tabs[0].rows = vec![
            WorkbookRow {
                id: "c1".to_string(),
                key: Some("gamma".to_string()),
                title: None,
                collapsed: true,
                freeze: false,
                kind: WorkbookRowKind::Constant(ConstantRowContent {
                    value: "1.4".to_string(),
                    dimension: Some("dimensionless".to_string()),
                }),
            },
            WorkbookRow {
                id: "t1".to_string(),
                key: None,
                title: None,
                collapsed: true,
                freeze: false,
                kind: WorkbookRowKind::Text(TextRowContent {
                    content: "see @gamma".to_string(),
                }),
            },
        ];
        let candidates = reference_candidates(&doc);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].key, "gamma");
    }

    #[test]
    fn find_reference_matches_extracts_inline_refs() {
        let refs = find_reference_matches("Use @gamma with @p_upstream - @p_downstream.");
        let keys = refs.into_iter().map(|m| m.key).collect::<Vec<_>>();
        assert_eq!(keys, vec!["gamma", "p_upstream", "p_downstream"]);
    }

    #[test]
    fn numeric_expression_supports_refs_pi_and_abs() {
        let executed = HashMap::from([
            (
                "p1".to_string(),
                WorkbookRowExecution {
                    id: "p1".to_string(),
                    key: Some("p_upstream".to_string()),
                    state: WorkbookRowState::Ok,
                    messages: vec![],
                    result: Some(WorkbookRowResult::Constant(ConstantRowResult {
                        raw: "260 psia".to_string(),
                        value: Value::from(260.0 * 6_894.757_293_168_f64),
                        normalized_si: Some(260.0 * 6_894.757_293_168_f64),
                        dimension: Some("pressure".to_string()),
                        unit: Some("Pa".to_string()),
                    })),
                },
            ),
            (
                "p2".to_string(),
                WorkbookRowExecution {
                    id: "p2".to_string(),
                    key: Some("p_downstream".to_string()),
                    state: WorkbookRowState::Ok,
                    messages: vec![],
                    result: Some(WorkbookRowResult::Constant(ConstantRowResult {
                        raw: "40 psia".to_string(),
                        value: Value::from(40.0 * 6_894.757_293_168_f64),
                        normalized_si: Some(40.0 * 6_894.757_293_168_f64),
                        dimension: Some("pressure".to_string()),
                        unit: Some("Pa".to_string()),
                    })),
                },
            ),
        ]);
        let key_to_id = HashMap::from([
            ("p_upstream".to_string(), "p1".to_string()),
            ("p_downstream".to_string(), "p2".to_string()),
        ]);
        let resolved = evaluate_numeric_expression(
            "abs(@p_upstream - @p_downstream) + pi/4 * 0",
            &executed,
            &key_to_id,
            Some("pressure"),
        )
        .expect("expression");
        assert!(resolved.value_si > 0.0);
    }

    #[test]
    fn constant_dimension_is_inferred_from_units() {
        let executed = HashMap::new();
        let key_to_id = HashMap::new();
        let result = parse_constant(
            &ConstantRowContent {
                value: "220 psia".to_string(),
                dimension: None,
            },
            &executed,
            &key_to_id,
        )
        .expect("constant");
        assert_eq!(result.dimension.as_deref(), Some("pressure"));
        assert_eq!(result.unit.as_deref(), Some("Pa"));
    }

    #[test]
    fn conflicting_explicit_dimension_is_invalid() {
        let executed = HashMap::new();
        let key_to_id = HashMap::new();
        let err = parse_constant(
            &ConstantRowContent {
                value: "220 psia".to_string(),
                dimension: Some("length".to_string()),
            },
            &executed,
            &key_to_id,
        )
        .expect_err("expected conflict");
        assert!(err.contains("conflicts"));
    }

    #[test]
    fn engineering_number_formatting_uses_compact_policy() {
        assert_eq!(
            format_engineering_number(1_516_846.604_496_959_8),
            "1.5168e6"
        );
        assert_eq!(format_engineering_number(12.3456), "12.35");
        assert_eq!(format_engineering_number(0.000_42), "4.2000e-4");
    }
}
