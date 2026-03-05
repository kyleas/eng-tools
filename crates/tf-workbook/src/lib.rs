use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use eng_core::units::parse_quantity_expression;
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
    pub name: String,
    pub file: String,
    pub title: Option<String>,
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
    pub name: String,
    pub file: String,
    pub title: Option<String>,
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
    Text(TextRowContent),
    Markdown(MarkdownRowContent),
    Constant(ConstantRowContent),
    EquationSolve(EquationSolveRowContent),
    Study(StudyRowContent),
    Plot(PlotRowContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRowContent {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownRowContent {
    pub markdown: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantRowContent {
    pub value: String,
    pub dimension_hint: Option<String>,
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
    Text { text: String },
    Markdown { markdown: String },
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
    pub unit: Option<String>,
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
    pub series: Vec<WorkbookPlotSeries>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookTabExecution {
    pub name: String,
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
            name: "inputs".to_string(),
            file: "01_inputs.yaml".to_string(),
            title: Some("Inputs".to_string()),
        }],
        preferred_display_units: BTreeMap::new(),
        execution: WorkbookExecutionDefaults::default(),
    };
    let tabs = vec![WorkbookTab {
        name: "inputs".to_string(),
        file: "01_inputs.yaml".to_string(),
        title: Some("Inputs".to_string()),
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
            name: entry.name.clone(),
            file: entry.file.clone(),
            title: entry.title.clone(),
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

    fs::write(
        doc.root_dir.join("workbook.yaml"),
        serde_yaml::to_string(&doc.manifest)?,
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
            if let Some(key) = &row.key {
                if !keys.insert(key.clone()) {
                    messages.push(format!("duplicate key '{}'", key));
                }
                key_to_id.insert(key.clone(), row.id.clone());
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
        let exec = execute_row(row, &executed, &key_to_id)?;
        executed.insert(row_id, exec);
    }

    let mut tabs = Vec::new();
    for tab in &doc.tabs {
        if let Some(sel) = selected_tab
            && sel != tab.name
            && sel != tab.file
        {
            continue;
        }
        let mut rows = Vec::new();
        for row in &tab.rows {
            if let Some(exec) = executed.get(&row.id) {
                rows.push(exec.clone());
            }
        }
        tabs.push(WorkbookTabExecution {
            name: tab.name.clone(),
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
                text: c.text.clone(),
            });
        }
        WorkbookRowKind::Markdown(c) => {
            out.state = WorkbookRowState::Ok;
            out.result = Some(WorkbookRowResult::Markdown {
                markdown: c.markdown.clone(),
            });
        }
        WorkbookRowKind::Constant(c) => match parse_constant(&c.value) {
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

fn parse_constant(raw: &str) -> Result<ConstantRowResult, String> {
    if let Ok(v) = raw.trim().parse::<f64>() {
        return Ok(ConstantRowResult {
            raw: raw.to_string(),
            value: Value::from(v),
            normalized_si: Some(v),
            unit: None,
        });
    }
    match parse_quantity_expression(raw.trim()) {
        Ok(expr) => Ok(ConstantRowResult {
            raw: raw.to_string(),
            value: Value::from(expr.value_si),
            normalized_si: Some(expr.value_si),
            unit: Some("SI".to_string()),
        }),
        Err(e) => Err(format!("invalid constant '{}': {}", raw, e)),
    }
}

fn eval_expr_to_value(
    expr: &str,
    executed: &HashMap<String, WorkbookRowExecution>,
    key_to_id: &HashMap<String, String>,
) -> Result<Value, WorkbookError> {
    if let Some(ref_id) = resolve_ref_to_id(expr, key_to_id) {
        let row = executed.get(&ref_id).ok_or_else(|| {
            WorkbookError::Execution(format!("referenced row '{}' has no result", ref_id))
        })?;
        return extract_scalar_value(row).ok_or_else(|| {
            WorkbookError::Execution(format!("referenced row '{}' is not scalar", ref_id))
        });
    }
    if let Ok(v) = expr.trim().parse::<f64>() {
        return Ok(Value::from(v));
    }
    if let Ok(expr_input) = parse_quantity_expression(expr.trim()) {
        return Ok(Value::from(expr_input.value_si));
    }
    if expr.eq_ignore_ascii_case("true") || expr.eq_ignore_ascii_case("false") {
        return Ok(Value::from(expr.eq_ignore_ascii_case("true")));
    }
    Ok(Value::from(expr.to_string()))
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
        WorkbookRowKind::EquationSolve(c) => {
            for expr in c.inputs.values() {
                if let Some(k) = parse_ref_key(expr) {
                    deps.push(k.to_string());
                }
            }
        }
        WorkbookRowKind::Study(c) => {
            for expr in c.fixed_inputs.values() {
                if let Some(k) = parse_ref_key(expr) {
                    deps.push(k.to_string());
                }
            }
        }
        WorkbookRowKind::Plot(c) => {
            if let Some(k) = parse_ref_key(&c.source_row) {
                deps.push(k.to_string());
            } else {
                deps.push(c.source_row.clone());
            }
        }
        _ => {}
    }
    deps
}

fn parse_ref_key(expr: &str) -> Option<&str> {
    let t = expr.trim();
    if let Some(k) = t.strip_prefix("ref:") {
        let k = k.trim();
        if !k.is_empty() {
            return Some(k);
        }
    }
    if let Some(k) = t.strip_prefix('@') {
        let k = k.trim();
        if !k.is_empty() {
            return Some(k);
        }
    }
    None
}

fn resolve_ref_to_id(expr: &str, key_to_id: &HashMap<String, String>) -> Option<String> {
    if let Some(k) = parse_ref_key(expr) {
        return key_to_id.get(k).cloned();
    }
    key_to_id.get(expr.trim()).cloned()
}

fn rewrite_refs_in_row(row: &mut WorkbookRow, old: &str, new: &str) -> usize {
    let mut count = 0usize;
    let rewrite = |expr: &mut String, count: &mut usize| {
        let t = expr.trim();
        if t == format!("ref:{old}") || t == format!("@{old}") {
            *expr = format!("ref:{new}");
            *count += 1;
        }
    };
    match &mut row.kind {
        WorkbookRowKind::EquationSolve(c) => {
            for expr in c.inputs.values_mut() {
                rewrite(expr, &mut count);
            }
        }
        WorkbookRowKind::Study(c) => {
            for expr in c.fixed_inputs.values_mut() {
                rewrite(expr, &mut count);
            }
        }
        WorkbookRowKind::Plot(c) => rewrite(&mut c.source_row, &mut count),
        _ => {}
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
                tab.name,
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
                        sanitize_name(&tab.name),
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
                    name: "analysis".to_string(),
                    file: "01_analysis.yaml".to_string(),
                    title: Some("Analysis".to_string()),
                }],
                preferred_display_units: BTreeMap::new(),
                execution: WorkbookExecutionDefaults::default(),
            },
            tabs: vec![WorkbookTab {
                name: "analysis".to_string(),
                file: "01_analysis.yaml".to_string(),
                title: Some("Analysis".to_string()),
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
                text: "hello".to_string(),
            }),
        });
        save_workbook_dir(&doc).expect("save");
        let loaded = load_workbook_dir(dir.path()).expect("load");
        assert_eq!(loaded.tabs[0].rows.len(), 1);
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
                    ("P".to_string(), "ref:p".to_string()),
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
                    dimension_hint: None,
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
                        ("P".to_string(), "ref:p".to_string()),
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
                        ("P".to_string(), "ref:b".to_string()),
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
                        ("P".to_string(), "ref:a".to_string()),
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
                    dimension_hint: Some("pressure".to_string()),
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
                        ("P".to_string(), "ref:p".to_string()),
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
                    source_row: "ref:study".to_string(),
                    x: "input_value".to_string(),
                    y: "value".to_string(),
                    title: Some("Isentropic".to_string()),
                    x_label: None,
                    y_label: None,
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
                source_row: "ref:missing".to_string(),
                x: "x".to_string(),
                y: "y".to_string(),
                title: None,
                x_label: None,
                y_label: None,
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
                text: "hello".to_string(),
            }),
        });
        save_workbook_dir(&doc).expect("save");
        let loaded = load_workbook_dir(dir.path()).expect("load");
        assert!(loaded.tabs[0].rows[0].collapsed);
        assert!(loaded.tabs[0].rows[0].freeze);
    }
}
