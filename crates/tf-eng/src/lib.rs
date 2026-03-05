//! tf-eng bridge: Thermoflow-facing integration layer over `eng`.
//!
//! Ownership boundary:
//! - `eng` owns equation/device/workflow/study execution logic.
//! - `tf-eng` owns app-facing request/response normalization (plot/table/diagnostics).
//! - Thermoflow UI/CLI should call this crate instead of re-implementing study logic.

use std::collections::{BTreeMap, BTreeSet};

use eng::solve::{
    DeviceStudySpec as EngDeviceStudySpec, EquationStudySpec as EngEquationStudySpec,
    StudySampleStatus, StudyTable as EngStudyTable, SweepAxis as EngSweepAxis,
    WorkflowFieldType as EngWorkflowFieldType, WorkflowStudySpec as EngWorkflowStudySpec,
};
use equations::{Registry, registry::ids::derive_path_id};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StudyTargetKind {
    Equation,
    Device,
    Workflow,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StudyFieldType {
    Float,
    Int,
    Bool,
    Enum,
    String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyEnumOption {
    pub key: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyFieldDescriptor {
    pub key: String,
    pub label: String,
    pub description: String,
    pub field_type: StudyFieldType,
    pub dimension: Option<String>,
    pub required: bool,
    pub optional: bool,
    pub sweepable: bool,
    pub default_value: Option<Value>,
    pub enum_options: Vec<StudyEnumOption>,
    pub unit: Option<String>,
    pub placeholder: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyOutputDescriptor {
    pub key: String,
    pub label: String,
    pub description: String,
    pub numeric: bool,
    pub plottable: bool,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyPlotDefault {
    pub x_field: String,
    pub y_output: String,
    pub x_label: String,
    pub y_label: String,
    pub title_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyPresetDescriptor {
    pub id: String,
    pub name: String,
    pub description: String,
    pub target_kind: StudyTargetKind,
    pub target_id: String,
    pub sweep_field: String,
    pub axis: SweepAxisSpec,
    pub input_overrides: Map<String, Value>,
    pub output_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyTargetDescriptor {
    pub id: String,
    pub kind: StudyTargetKind,
    pub name: String,
    pub description: String,
    pub category: Option<String>,
    pub studyable: bool,
    pub input_fields: Vec<StudyFieldDescriptor>,
    pub sweepable_fields: Vec<String>,
    pub outputs: Vec<StudyOutputDescriptor>,
    pub default_output: Option<String>,
    pub plot_default: Option<StudyPlotDefault>,
    pub display_latex: Option<String>,
    pub display_unicode: Option<String>,
    pub display_ascii: Option<String>,
    pub branch_options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleSolveRequest {
    pub target_kind: StudyTargetKind,
    pub target_id: String,
    #[serde(default)]
    pub inputs: Map<String, Value>,
    pub output_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleSolveResult {
    pub target_kind: StudyTargetKind,
    pub target_id: String,
    pub output_key: String,
    pub value: Value,
    pub unit: Option<String>,
    pub path_text: Option<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SolveRowState {
    Incomplete,
    Validating,
    Ready,
    Success,
    Invalid,
    Ambiguous,
    Unsupported,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolveFieldValidation {
    pub key: String,
    pub valid: bool,
    pub empty: bool,
    pub message: Option<String>,
    pub normalized: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolveValidation {
    pub state: SolveRowState,
    pub ready: bool,
    pub output_key: Option<String>,
    pub normalized_inputs: Map<String, Value>,
    pub request_preview: Option<Value>,
    pub fields: Vec<SolveFieldValidation>,
    pub missing_required: Vec<String>,
    pub blocking_reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyRunRequest {
    pub target_kind: StudyTargetKind,
    pub target_id: String,
    pub sweep_field: String,
    pub axis: SweepAxisSpec,
    #[serde(default)]
    pub inputs: Map<String, Value>,
    pub output_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SweepAxisSpec {
    Values(Vec<f64>),
    Linspace { start: f64, end: f64, count: usize },
    Logspace { start: f64, end: f64, count: usize },
}

impl SweepAxisSpec {
    pub fn linspace(start: f64, end: f64, count: usize) -> Self {
        Self::Linspace { start, end, count }
    }

    fn into_eng(self) -> EngSweepAxis {
        match self {
            SweepAxisSpec::Values(v) => EngSweepAxis::values(v),
            SweepAxisSpec::Linspace { start, end, count } => {
                EngSweepAxis::linspace(start, end, count)
            }
            SweepAxisSpec::Logspace { start, end, count } => {
                EngSweepAxis::logspace(start, end, count)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquationStudyRequest {
    pub path_id: String,
    pub target: String,
    pub sweep_variable: String,
    pub axis: SweepAxisSpec,
    #[serde(default)]
    pub fixed_inputs: BTreeMap<String, f64>,
    pub branch: Option<String>,
    pub output_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStudyRequest {
    pub device_key: String,
    pub sweep_arg: String,
    pub axis: SweepAxisSpec,
    #[serde(default)]
    pub fixed_args: Map<String, Value>,
    #[serde(default)]
    pub requested_outputs: Vec<String>,
    pub output_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStudyRequest {
    pub workflow_key: String,
    pub sweep_arg: String,
    pub axis: SweepAxisSpec,
    #[serde(default)]
    pub fixed_args: Map<String, Value>,
    pub output_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotSeries {
    pub name: String,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub x_label: String,
    pub y_label: String,
    pub x_unit: Option<String>,
    pub y_unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyTable {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsBundle {
    pub per_row_path: Vec<Option<String>>,
    pub per_row_error: Vec<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyMeta {
    pub study_id: String,
    pub n_ok: usize,
    pub n_fail: usize,
    pub warnings_summary: Vec<String>,
    pub selected_output_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyResult {
    pub table: StudyTable,
    pub series: Vec<PlotSeries>,
    pub meta: StudyMeta,
    pub diagnostics: Option<DiagnosticsBundle>,
}

#[derive(Debug, Error)]
pub enum BridgeError {
    #[error("{0}")]
    InvalidRequest(String),
    #[error("{0}")]
    Engine(String),
}

pub fn studyable_device_keys() -> Vec<String> {
    eng::solve::studyable_devices()
        .into_iter()
        .map(|d| d.device_key)
        .collect()
}

pub fn studyable_workflow_keys() -> Vec<String> {
    eng::solve::studyable_workflows()
        .into_iter()
        .map(|w| w.key.to_string())
        .collect()
}

pub fn list_study_targets() -> Result<Vec<StudyTargetDescriptor>, BridgeError> {
    let mut out = Vec::new();
    out.extend(describe_all_equations()?);
    out.extend(describe_all_devices());
    out.extend(describe_all_workflows());
    Ok(out)
}

pub fn list_study_presets() -> Result<Vec<StudyPresetDescriptor>, BridgeError> {
    let targets = list_study_targets()?;
    let mut out = Vec::new();

    if targets.iter().any(|t| {
        t.kind == StudyTargetKind::Equation && t.id == "compressible.isentropic_pressure_ratio"
    }) {
        out.push(StudyPresetDescriptor {
            id: "eq_isentropic_m_to_p_p0".to_string(),
            name: "Isentropic Mach -> p/p0".to_string(),
            description: "Equation sweep of Mach against isentropic pressure ratio".to_string(),
            target_kind: StudyTargetKind::Equation,
            target_id: "compressible.isentropic_pressure_ratio".to_string(),
            sweep_field: "M".to_string(),
            axis: SweepAxisSpec::linspace(0.2, 3.0, 40),
            input_overrides: Map::from_iter([("gamma".to_string(), Value::from(1.4))]),
            output_key: "p_p0".to_string(),
        });
    }

    if targets
        .iter()
        .any(|t| t.kind == StudyTargetKind::Device && t.id == "normal_shock_calc")
    {
        out.push(StudyPresetDescriptor {
            id: "device_normal_shock_m1_to_p2p1".to_string(),
            name: "Normal Shock M1 -> p2/p1".to_string(),
            description: "Device sweep of upstream Mach through normal shock pressure ratio"
                .to_string(),
            target_kind: StudyTargetKind::Device,
            target_id: "normal_shock_calc".to_string(),
            sweep_field: "input_value".to_string(),
            axis: SweepAxisSpec::linspace(1.1, 5.0, 35),
            input_overrides: Map::from_iter([
                ("input_kind".to_string(), Value::from("m1")),
                ("target_kind".to_string(), Value::from("p2_p1")),
                ("gamma".to_string(), Value::from(1.4)),
            ]),
            output_key: "value".to_string(),
        });
    }

    if targets
        .iter()
        .any(|t| t.kind == StudyTargetKind::Workflow && t.id == "nozzle_normal_shock_chain")
    {
        out.push(StudyPresetDescriptor {
            id: "workflow_nozzle_normal_shock".to_string(),
            name: "Nozzle + Normal Shock Chain".to_string(),
            description: "Workflow sweep over area ratio".to_string(),
            target_kind: StudyTargetKind::Workflow,
            target_id: "nozzle_normal_shock_chain".to_string(),
            sweep_field: "area_ratio".to_string(),
            axis: SweepAxisSpec::linspace(1.4, 2.4, 25),
            input_overrides: Map::from_iter([
                ("gamma".to_string(), Value::from(1.4)),
                ("branch".to_string(), Value::from("supersonic")),
            ]),
            output_key: "post_shock_mach".to_string(),
        });
    }

    Ok(out)
}

pub fn describe_equation_target(path_id: &str) -> Result<StudyTargetDescriptor, BridgeError> {
    let all = describe_all_equations()?;
    all.into_iter()
        .find(|d| d.id == path_id)
        .ok_or_else(|| BridgeError::InvalidRequest(format!("unknown equation '{path_id}'")))
}

pub fn describe_device_target(device_key: &str) -> Result<StudyTargetDescriptor, BridgeError> {
    describe_all_devices()
        .into_iter()
        .find(|d| d.id == device_key)
        .ok_or_else(|| BridgeError::InvalidRequest(format!("unknown device '{device_key}'")))
}

pub fn describe_workflow_target(workflow_key: &str) -> Result<StudyTargetDescriptor, BridgeError> {
    describe_all_workflows()
        .into_iter()
        .find(|d| d.id == workflow_key)
        .ok_or_else(|| BridgeError::InvalidRequest(format!("unknown workflow '{workflow_key}'")))
}

pub fn run_study_from_form(req: StudyRunRequest) -> Result<StudyResult, BridgeError> {
    let descriptor = match req.target_kind {
        StudyTargetKind::Equation => describe_equation_target(&req.target_id)?,
        StudyTargetKind::Device => describe_device_target(&req.target_id)?,
        StudyTargetKind::Workflow => describe_workflow_target(&req.target_id)?,
    };
    let normalized = normalize_and_validate(&descriptor, req)?;
    match normalized {
        NormalizedStudyRequest::Equation(r) => run_equation_study(r),
        NormalizedStudyRequest::Device(r) => run_device_study(r),
        NormalizedStudyRequest::Workflow(r) => run_workflow_study(r),
    }
}

pub fn run_single_solve(req: SingleSolveRequest) -> Result<SingleSolveResult, BridgeError> {
    let descriptor = match req.target_kind {
        StudyTargetKind::Equation => describe_equation_target(&req.target_id)?,
        StudyTargetKind::Device => describe_device_target(&req.target_id)?,
        StudyTargetKind::Workflow => describe_workflow_target(&req.target_id)?,
    };
    match req.target_kind {
        StudyTargetKind::Equation => run_single_equation(&descriptor, req),
        StudyTargetKind::Device => run_single_device(&descriptor, req),
        StudyTargetKind::Workflow => run_single_workflow(&descriptor, req),
    }
}

#[derive(Debug, Clone)]
struct SingleSolvePlan {
    request: Option<SingleSolveRequest>,
    validation: SolveValidation,
}

pub fn evaluate_single_solve(
    target_kind: StudyTargetKind,
    target_id: &str,
    raw_inputs: &BTreeMap<String, String>,
    output_key: Option<&str>,
) -> Result<(SolveValidation, Option<SingleSolveResult>), BridgeError> {
    let plan = build_single_solve_plan(target_kind, target_id, raw_inputs, output_key)?;
    if !plan.validation.ready {
        return Ok((plan.validation, None));
    }
    let req = plan
        .request
        .ok_or_else(|| BridgeError::InvalidRequest("missing solve request".to_string()))?;
    match run_single_solve(req) {
        Ok(res) => {
            let mut validation = plan.validation;
            validation.state = SolveRowState::Success;
            Ok((validation, Some(res)))
        }
        Err(e) => {
            let mut validation = plan.validation;
            validation.state = SolveRowState::Error;
            validation.ready = false;
            validation.blocking_reasons.push(e.to_string());
            Ok((validation, None))
        }
    }
}

pub fn validate_single_solve(
    target_kind: StudyTargetKind,
    target_id: &str,
    raw_inputs: &BTreeMap<String, String>,
    output_key: Option<&str>,
) -> Result<SolveValidation, BridgeError> {
    Ok(build_single_solve_plan(target_kind, target_id, raw_inputs, output_key)?.validation)
}

fn build_single_solve_plan(
    target_kind: StudyTargetKind,
    target_id: &str,
    raw_inputs: &BTreeMap<String, String>,
    output_key: Option<&str>,
) -> Result<SingleSolvePlan, BridgeError> {
    let descriptor = match target_kind {
        StudyTargetKind::Equation => describe_equation_target(target_id)?,
        StudyTargetKind::Device => describe_device_target(target_id)?,
        StudyTargetKind::Workflow => describe_workflow_target(target_id)?,
    };

    let mut fields = Vec::new();
    let mut normalized = Map::new();
    let mut missing_required = Vec::new();
    let mut blocking_reasons = Vec::new();

    for field in &descriptor.input_fields {
        let raw = raw_inputs
            .get(&field.key)
            .map(|s| s.trim().to_string())
            .unwrap_or_default();
        let empty = raw.is_empty();
        if empty {
            fields.push(SolveFieldValidation {
                key: field.key.clone(),
                valid: !field.required,
                empty: true,
                message: if field.required {
                    Some("required".to_string())
                } else {
                    None
                },
                normalized: None,
            });
            if field.required {
                missing_required.push(field.key.clone());
            }
            continue;
        }

        match parse_field_value(field, &raw) {
            Ok(v) => {
                normalized.insert(field.key.clone(), v.clone());
                fields.push(SolveFieldValidation {
                    key: field.key.clone(),
                    valid: true,
                    empty: false,
                    message: None,
                    normalized: Some(v),
                });
            }
            Err(e) => {
                fields.push(SolveFieldValidation {
                    key: field.key.clone(),
                    valid: false,
                    empty: false,
                    message: Some(e.clone()),
                    normalized: None,
                });
                blocking_reasons.push(format!("{}: {}", field.key, e));
            }
        }
    }

    let mut resolved_output = output_key
        .map(|s| s.to_string())
        .filter(|s| !s.trim().is_empty());
    if matches!(target_kind, StudyTargetKind::Equation) && resolved_output.is_none() {
        let unknown = descriptor
            .input_fields
            .iter()
            .filter(|f| f.key != "branch")
            .filter(|f| {
                raw_inputs
                    .get(&f.key)
                    .map(|v| v.trim().is_empty())
                    .unwrap_or(true)
            })
            .map(|f| f.key.clone())
            .collect::<Vec<_>>();
        match unknown.len() {
            1 => {
                resolved_output = Some(unknown[0].clone());
                missing_required.retain(|m| m != &unknown[0]);
            }
            n if n > 1 => {
                blocking_reasons.push(
                    "equation solve requires exactly one unknown (or explicit output)".to_string(),
                );
            }
            _ => {
                blocking_reasons.push(
                    "equation solve has no inferred unknown; choose output explicitly".to_string(),
                );
            }
        }
    } else if resolved_output.is_none() {
        resolved_output = descriptor.default_output.clone();
    }

    if let Some(out) = &resolved_output {
        if !descriptor.outputs.iter().any(|o| o.key == *out) {
            blocking_reasons.push(format!("invalid output '{out}'"));
        } else if descriptor.kind == StudyTargetKind::Equation {
            missing_required.retain(|m| m != out && m != "branch");
        }
    }

    let has_invalid = fields.iter().any(|f| !f.valid && !f.empty);
    let ready = !has_invalid
        && blocking_reasons.is_empty()
        && resolved_output.is_some()
        && missing_required.is_empty();
    let state = if has_invalid {
        SolveRowState::Invalid
    } else if !blocking_reasons.is_empty() {
        if blocking_reasons
            .iter()
            .any(|m| m.contains("exactly one unknown"))
        {
            SolveRowState::Ambiguous
        } else if blocking_reasons
            .iter()
            .any(|m| m.contains("invalid output"))
        {
            SolveRowState::Unsupported
        } else {
            SolveRowState::Incomplete
        }
    } else if ready {
        SolveRowState::Ready
    } else {
        SolveRowState::Incomplete
    };

    let request_preview = resolved_output.as_ref().map(|out| {
        json!({
            "target_kind": target_kind,
            "target_id": target_id,
            "output_key": out,
            "inputs": normalized.clone(),
        })
    });

    let validation = SolveValidation {
        state,
        ready,
        output_key: resolved_output,
        normalized_inputs: normalized,
        request_preview,
        fields,
        missing_required,
        blocking_reasons,
    };
    let request = if validation.ready {
        Some(SingleSolveRequest {
            target_kind,
            target_id: target_id.to_string(),
            inputs: validation.normalized_inputs.clone(),
            output_key: validation.output_key.clone(),
        })
    } else {
        None
    };
    Ok(SingleSolvePlan {
        request,
        validation,
    })
}

pub fn run_equation_study(req: EquationStudyRequest) -> Result<StudyResult, BridgeError> {
    let spec = EngEquationStudySpec {
        path_id: req.path_id,
        target: req.target.clone(),
        sweep_variable: req.sweep_variable.clone(),
        fixed_inputs: req.fixed_inputs,
        branch: req.branch,
    };
    let table = eng::solve::run_equation_study(&spec, req.axis.into_eng());
    let selected = req.output_key.unwrap_or(req.target);
    Ok(normalize_table(table, &selected))
}

pub fn run_device_study(req: DeviceStudyRequest) -> Result<StudyResult, BridgeError> {
    let mut requested_outputs = req.requested_outputs;
    if requested_outputs.is_empty() {
        requested_outputs = vec!["value".to_string(), "path_text".to_string()];
    }
    let selected = req
        .output_key
        .clone()
        .unwrap_or_else(|| first_numeric_output_hint(&requested_outputs));

    let spec = EngDeviceStudySpec {
        device_key: req.device_key,
        sweep_arg: req.sweep_arg.clone(),
        axis: req.axis.into_eng(),
        fixed_args: req.fixed_args,
        requested_outputs,
    };
    let table = eng::solve::run_device_study(&spec).map_err(BridgeError::Engine)?;
    Ok(normalize_table(table, &selected))
}

pub fn run_workflow_study(req: WorkflowStudyRequest) -> Result<StudyResult, BridgeError> {
    let selected = req
        .output_key
        .clone()
        .unwrap_or_else(|| "pre_shock_mach".to_string());
    let spec = EngWorkflowStudySpec {
        workflow_key: req.workflow_key,
        sweep_arg: req.sweep_arg,
        axis: req.axis.into_eng(),
        fixed_args: req.fixed_args,
    };
    let table = eng::solve::run_workflow_study(&spec).map_err(BridgeError::Engine)?;
    Ok(normalize_table(table, &selected))
}

enum NormalizedStudyRequest {
    Equation(EquationStudyRequest),
    Device(DeviceStudyRequest),
    Workflow(WorkflowStudyRequest),
}

fn normalize_and_validate(
    descriptor: &StudyTargetDescriptor,
    req: StudyRunRequest,
) -> Result<NormalizedStudyRequest, BridgeError> {
    if !descriptor
        .sweepable_fields
        .iter()
        .any(|f| f == &req.sweep_field)
    {
        return Err(BridgeError::InvalidRequest(format!(
            "sweep field '{}' is not sweepable for '{}'",
            req.sweep_field, descriptor.id
        )));
    }

    for field in &descriptor.input_fields {
        if field.required && !req.inputs.contains_key(&field.key) && field.key != req.sweep_field {
            return Err(BridgeError::InvalidRequest(format!(
                "missing required input '{}'",
                field.key
            )));
        }
        if let Some(v) = req.inputs.get(&field.key) {
            validate_field_value(field, v)?;
        }
    }

    let selected_output = req
        .output_key
        .clone()
        .or_else(|| descriptor.default_output.clone())
        .ok_or_else(|| BridgeError::InvalidRequest("no default output available".to_string()))?;

    if !descriptor.outputs.iter().any(|o| o.key == selected_output) {
        return Err(BridgeError::InvalidRequest(format!(
            "unknown output key '{}' for '{}'",
            selected_output, descriptor.id
        )));
    }

    match req.target_kind {
        StudyTargetKind::Equation => {
            let fixed_inputs = descriptor
                .input_fields
                .iter()
                .filter(|f| f.key != req.sweep_field)
                .filter_map(|f| req.inputs.get(&f.key).map(|v| (f.key.clone(), v)))
                .map(|(k, v)| {
                    v.as_f64().map(|n| (k.clone(), n)).ok_or_else(|| {
                        BridgeError::InvalidRequest(format!("input '{}' must be numeric", k))
                    })
                })
                .collect::<Result<BTreeMap<_, _>, _>>()?;
            let branch = req
                .inputs
                .get("branch")
                .and_then(Value::as_str)
                .map(|s| s.to_string());
            Ok(NormalizedStudyRequest::Equation(EquationStudyRequest {
                path_id: descriptor.id.clone(),
                target: selected_output.clone(),
                sweep_variable: req.sweep_field,
                axis: req.axis,
                fixed_inputs,
                branch,
                output_key: Some(selected_output),
            }))
        }
        StudyTargetKind::Device => {
            let mut fixed_args = Map::new();
            for field in &descriptor.input_fields {
                if field.key == req.sweep_field {
                    continue;
                }
                if let Some(v) = req.inputs.get(&field.key) {
                    fixed_args.insert(field.key.clone(), v.clone());
                }
            }
            let mut outputs = vec![selected_output.clone()];
            if descriptor.outputs.iter().any(|o| o.key == "path_text") {
                outputs.push("path_text".to_string());
            }
            if descriptor.outputs.iter().any(|o| o.key == "pivot") {
                outputs.push("pivot".to_string());
            }
            Ok(NormalizedStudyRequest::Device(DeviceStudyRequest {
                device_key: descriptor.id.clone(),
                sweep_arg: req.sweep_field,
                axis: req.axis,
                fixed_args,
                requested_outputs: outputs,
                output_key: Some(selected_output),
            }))
        }
        StudyTargetKind::Workflow => {
            let mut fixed_args = Map::new();
            for field in &descriptor.input_fields {
                if field.key == req.sweep_field {
                    continue;
                }
                if let Some(v) = req.inputs.get(&field.key) {
                    fixed_args.insert(field.key.clone(), v.clone());
                }
            }
            Ok(NormalizedStudyRequest::Workflow(WorkflowStudyRequest {
                workflow_key: descriptor.id.clone(),
                sweep_arg: req.sweep_field,
                axis: req.axis,
                fixed_args,
                output_key: Some(selected_output),
            }))
        }
    }
}

fn validate_field_value(field: &StudyFieldDescriptor, value: &Value) -> Result<(), BridgeError> {
    match field.field_type {
        StudyFieldType::Float => {
            if value.as_f64().is_none() {
                return Err(BridgeError::InvalidRequest(format!(
                    "field '{}' must be float",
                    field.key
                )));
            }
        }
        StudyFieldType::Int => {
            if value.as_i64().is_none() && value.as_u64().is_none() {
                return Err(BridgeError::InvalidRequest(format!(
                    "field '{}' must be int",
                    field.key
                )));
            }
        }
        StudyFieldType::Bool => {
            if value.as_bool().is_none() {
                return Err(BridgeError::InvalidRequest(format!(
                    "field '{}' must be bool",
                    field.key
                )));
            }
        }
        StudyFieldType::Enum => {
            let Some(s) = value.as_str() else {
                return Err(BridgeError::InvalidRequest(format!(
                    "field '{}' must be enum string",
                    field.key
                )));
            };
            if !field.enum_options.is_empty() && !field.enum_options.iter().any(|o| o.key == s) {
                return Err(BridgeError::InvalidRequest(format!(
                    "field '{}' has unsupported value '{}'",
                    field.key, s
                )));
            }
        }
        StudyFieldType::String => {
            if value.as_str().is_none() {
                return Err(BridgeError::InvalidRequest(format!(
                    "field '{}' must be string",
                    field.key
                )));
            }
        }
    }
    Ok(())
}

fn describe_all_equations() -> Result<Vec<StudyTargetDescriptor>, BridgeError> {
    let registry = Registry::load_default().map_err(|e| BridgeError::Engine(e.to_string()))?;
    let mut out = Vec::new();
    for eq in registry.equations() {
        let path_id = derive_path_id(eq);
        let unsupported = eq
            .solve
            .numerical
            .unsupported_targets
            .iter()
            .map(|s| s.as_str())
            .collect::<BTreeSet<_>>();

        let mut outputs = eq
            .variables
            .keys()
            .filter(|k| !unsupported.contains(k.as_str()))
            .map(|k| StudyOutputDescriptor {
                key: k.clone(),
                label: k.clone(),
                description: format!("Solve target {}", k),
                numeric: true,
                plottable: true,
                unit: eq
                    .variables
                    .get(k)
                    .and_then(|v| v.default_unit.as_ref())
                    .map(|s| s.to_string()),
            })
            .collect::<Vec<_>>();
        outputs.sort_by(|a, b| a.key.cmp(&b.key));

        let mut fields = eq
            .variables
            .iter()
            .map(|(k, v)| StudyFieldDescriptor {
                key: k.clone(),
                label: v.name.clone(),
                description: v.description.clone().unwrap_or_else(|| v.name.clone()),
                field_type: StudyFieldType::Float,
                dimension: Some(v.dimension.clone()),
                required: true,
                optional: false,
                sweepable: true,
                default_value: if k.eq_ignore_ascii_case("gamma") {
                    Some(Value::from(1.4))
                } else {
                    None
                },
                enum_options: Vec::new(),
                unit: v.default_unit.clone(),
                placeholder: None,
            })
            .collect::<Vec<_>>();

        if !eq.branches.is_empty() {
            fields.push(StudyFieldDescriptor {
                key: "branch".to_string(),
                label: "Branch".to_string(),
                description: "Optional branch selection".to_string(),
                field_type: StudyFieldType::Enum,
                dimension: None,
                required: false,
                optional: true,
                sweepable: false,
                default_value: eq
                    .branches
                    .iter()
                    .find(|b| b.preferred)
                    .map(|b| Value::from(b.name.clone())),
                enum_options: eq
                    .branches
                    .iter()
                    .map(|b| StudyEnumOption {
                        key: b.name.clone(),
                        label: b.name.clone(),
                    })
                    .collect(),
                unit: None,
                placeholder: None,
            });
        }

        let default_output = eq
            .solve
            .default_target
            .clone()
            .or_else(|| outputs.first().map(|o| o.key.clone()));

        let sweep_default = fields
            .iter()
            .find(|f| f.key != "gamma" && f.sweepable)
            .map(|f| f.key.clone())
            .or_else(|| fields.iter().find(|f| f.sweepable).map(|f| f.key.clone()))
            .unwrap_or_else(|| "sample".to_string());

        out.push(StudyTargetDescriptor {
            id: path_id,
            kind: StudyTargetKind::Equation,
            name: eq.name.clone(),
            description: eq
                .assumptions
                .first()
                .cloned()
                .unwrap_or_else(|| "Equation study target".to_string()),
            category: Some(eq.taxonomy.category.clone()),
            studyable: true,
            sweepable_fields: fields
                .iter()
                .filter(|f| f.sweepable)
                .map(|f| f.key.clone())
                .collect(),
            input_fields: fields,
            outputs: outputs.clone(),
            default_output: default_output.clone(),
            plot_default: default_output.map(|y| StudyPlotDefault {
                x_field: sweep_default.clone(),
                y_output: y.clone(),
                x_label: sweep_default,
                y_label: y,
                title_template: format!("{} study", eq.name),
            }),
            display_latex: Some(eq.display.latex.clone()),
            display_unicode: eq.display.unicode.clone(),
            display_ascii: eq.display.ascii.clone(),
            branch_options: eq.branches.iter().map(|b| b.name.clone()).collect(),
        });
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}

fn describe_all_devices() -> Vec<StudyTargetDescriptor> {
    let studyable = eng::solve::studyable_devices()
        .into_iter()
        .map(|d| (d.device_key.clone(), d))
        .collect::<BTreeMap<_, _>>();

    let mut out = Vec::new();
    for spec in eng::devices::generation_specs() {
        let Some(study_meta) = studyable.get(spec.key) else {
            continue;
        };
        let main_id = format!("device.{}", spec.key);
        let main_fn = spec
            .binding_functions
            .iter()
            .find(|bf| bf.id == main_id)
            .or_else(|| {
                spec.binding_functions
                    .iter()
                    .find(|bf| bf.id.starts_with(&main_id) && bf.returns == "f64")
            });

        let mut fields = main_fn
            .map(|f| {
                f.args
                    .iter()
                    .map(|a| infer_field_from_binding_arg(a.name, a.description))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        if fields.is_empty() {
            fields.push(StudyFieldDescriptor {
                key: "input_value".to_string(),
                label: "Input value".to_string(),
                description: "Numeric sweep/input value".to_string(),
                field_type: StudyFieldType::Float,
                dimension: None,
                required: true,
                optional: false,
                sweepable: true,
                default_value: None,
                enum_options: vec![],
                unit: None,
                placeholder: None,
            });
        }

        let mut outputs = vec![StudyOutputDescriptor {
            key: "value".to_string(),
            label: "Value".to_string(),
            description: "Primary numeric study value".to_string(),
            numeric: true,
            plottable: true,
            unit: None,
        }];
        if study_meta.pivot_op.is_some() {
            outputs.push(StudyOutputDescriptor {
                key: "pivot".to_string(),
                label: "Pivot".to_string(),
                description: "Resolved pivot scalar".to_string(),
                numeric: true,
                plottable: true,
                unit: None,
            });
        }
        if study_meta.path_op.is_some() {
            outputs.push(StudyOutputDescriptor {
                key: "path_text".to_string(),
                label: "Path text".to_string(),
                description: "Step trace text".to_string(),
                numeric: false,
                plottable: false,
                unit: None,
            });
        }

        let default_output = outputs.iter().find(|o| o.plottable).map(|o| o.key.clone());

        let default_sweep = fields
            .iter()
            .find(|f| f.key == "input_value")
            .or_else(|| fields.iter().find(|f| f.sweepable))
            .map(|f| f.key.clone())
            .unwrap_or_else(|| "input_value".to_string());
        let branch_options = fields
            .iter()
            .find(|f| f.key == "branch")
            .map(|f| f.enum_options.iter().map(|o| o.key.clone()).collect())
            .unwrap_or_default();

        out.push(StudyTargetDescriptor {
            id: spec.key.to_string(),
            kind: StudyTargetKind::Device,
            name: spec.name.to_string(),
            description: spec.summary.to_string(),
            category: Some("device".to_string()),
            studyable: true,
            sweepable_fields: fields
                .iter()
                .filter(|f| f.sweepable)
                .map(|f| f.key.clone())
                .collect(),
            input_fields: fields,
            outputs: outputs.clone(),
            default_output: default_output.clone(),
            plot_default: default_output.map(|y| StudyPlotDefault {
                x_field: default_sweep.clone(),
                y_output: y.clone(),
                x_label: default_sweep,
                y_label: y,
                title_template: format!("{} study", spec.name),
            }),
            display_latex: None,
            display_unicode: None,
            display_ascii: None,
            branch_options,
        });
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}

fn describe_all_workflows() -> Vec<StudyTargetDescriptor> {
    let mut out = Vec::new();
    for wf in eng::solve::studyable_workflows() {
        let fields = wf
            .input_fields
            .iter()
            .map(|f| StudyFieldDescriptor {
                key: f.key.to_string(),
                label: f.label.to_string(),
                description: f.description.to_string(),
                field_type: map_workflow_field_type(&f.field_type),
                dimension: None,
                required: f.required,
                optional: !f.required,
                sweepable: f.sweepable,
                default_value: f.default_value.map(Value::from),
                enum_options: f
                    .enum_options
                    .iter()
                    .map(|o| StudyEnumOption {
                        key: (*o).to_string(),
                        label: (*o).to_string(),
                    })
                    .collect(),
                unit: None,
                placeholder: None,
            })
            .collect::<Vec<_>>();

        let outputs = wf
            .output_fields
            .iter()
            .map(|o| StudyOutputDescriptor {
                key: o.key.to_string(),
                label: o.label.to_string(),
                description: o.description.to_string(),
                numeric: o.plottable,
                plottable: o.plottable,
                unit: None,
            })
            .collect::<Vec<_>>();

        let default_output = outputs
            .iter()
            .find(|o| o.plottable)
            .map(|o| o.key.clone())
            .or_else(|| outputs.first().map(|o| o.key.clone()));
        let branch_options = fields
            .iter()
            .find(|f| f.key == "branch")
            .map(|f| f.enum_options.iter().map(|o| o.key.clone()).collect())
            .unwrap_or_default();

        out.push(StudyTargetDescriptor {
            id: wf.key.to_string(),
            kind: StudyTargetKind::Workflow,
            name: wf.name.to_string(),
            description: wf.summary.to_string(),
            category: Some("workflow".to_string()),
            studyable: true,
            sweepable_fields: fields
                .iter()
                .filter(|f| f.sweepable)
                .map(|f| f.key.clone())
                .collect(),
            input_fields: fields,
            outputs: outputs.clone(),
            default_output: default_output.clone(),
            plot_default: default_output.map(|y| StudyPlotDefault {
                x_field: wf.default_sweep_arg.to_string(),
                y_output: y.clone(),
                x_label: wf.default_sweep_arg.to_string(),
                y_label: y,
                title_template: format!("{} study", wf.name),
            }),
            display_latex: None,
            display_unicode: None,
            display_ascii: None,
            branch_options,
        });
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}

fn map_workflow_field_type(kind: &EngWorkflowFieldType) -> StudyFieldType {
    match kind {
        EngWorkflowFieldType::Float => StudyFieldType::Float,
        EngWorkflowFieldType::Int => StudyFieldType::Int,
        EngWorkflowFieldType::Bool => StudyFieldType::Bool,
        EngWorkflowFieldType::Enum => StudyFieldType::Enum,
        EngWorkflowFieldType::String => StudyFieldType::String,
    }
}

fn infer_field_from_binding_arg(name: &str, description: &str) -> StudyFieldDescriptor {
    let lower = description.to_ascii_lowercase();
    let optional = lower.contains("optional") || name == "branch";
    let mut field_type = StudyFieldType::Float;
    let mut enum_options = Vec::new();

    if name.contains("kind") || name == "branch" {
        field_type = StudyFieldType::Enum;
        enum_options = parse_enum_options(description);
    } else if name.starts_with("is_") || lower.contains("true/false") {
        field_type = StudyFieldType::Bool;
    } else if name.ends_with("_id") {
        field_type = StudyFieldType::String;
    }

    let default_value = if name.eq_ignore_ascii_case("gamma") {
        Some(Value::from(1.4))
    } else if name == "branch" {
        enum_options
            .iter()
            .find(|o| o.key == "supersonic" || o.key == "weak")
            .map(|o| Value::from(o.key.clone()))
    } else {
        None
    };

    let sweepable = matches!(field_type, StudyFieldType::Float | StudyFieldType::Int);

    StudyFieldDescriptor {
        key: name.to_string(),
        label: to_label(name),
        description: description.to_string(),
        field_type,
        dimension: None,
        required: !optional,
        optional,
        sweepable,
        default_value,
        enum_options,
        unit: infer_unit(name, description),
        placeholder: None,
    }
}

fn parse_enum_options(description: &str) -> Vec<StudyEnumOption> {
    let mut options = Vec::new();
    if let (Some(l), Some(r)) = (description.find('('), description.rfind(')'))
        && l < r
    {
        for raw in description[l + 1..r].split(',') {
            let key = raw.trim().trim_matches('`').to_string();
            if !key.is_empty() {
                options.push(StudyEnumOption {
                    label: key.clone(),
                    key,
                });
            }
        }
    }

    let lower = description.to_ascii_lowercase();
    for k in ["subsonic", "supersonic", "weak", "strong"] {
        if lower.contains(k) && !options.iter().any(|o| o.key.eq_ignore_ascii_case(k)) {
            options.push(StudyEnumOption {
                key: k.to_string(),
                label: k.to_string(),
            });
        }
    }
    options
}

fn infer_unit(name: &str, description: &str) -> Option<String> {
    let lower = format!(
        "{} {}",
        name.to_ascii_lowercase(),
        description.to_ascii_lowercase()
    );
    if lower.contains("_deg") || lower.contains("deg") {
        return Some("deg".to_string());
    }
    if lower.contains("rad") {
        return Some("rad".to_string());
    }
    None
}

fn to_label(key: &str) -> String {
    key.split('_')
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                Some(c) => c.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn first_numeric_output_hint(requested_outputs: &[String]) -> String {
    requested_outputs
        .iter()
        .find(|k| k.as_str() != "path_text")
        .cloned()
        .unwrap_or_else(|| "value".to_string())
}

fn normalize_table(table: EngStudyTable, output_key: &str) -> StudyResult {
    let n_ok = table
        .rows
        .iter()
        .filter(|r| matches!(r.status, StudySampleStatus::Ok))
        .count();
    let n_fail = table.rows.len().saturating_sub(n_ok);

    let mut warning_set = BTreeSet::new();
    let mut x = Vec::with_capacity(table.rows.len());
    let mut y = Vec::with_capacity(table.rows.len());
    let mut path = Vec::with_capacity(table.rows.len());
    let mut errors = Vec::with_capacity(table.rows.len());

    for row in &table.rows {
        x.push(row.sample_value);
        let y_val = row.outputs.get(output_key).copied().unwrap_or(f64::NAN);
        y.push(y_val);
        for w in &row.warnings {
            warning_set.insert(w.clone());
        }
        path.push(row.path_summary.clone());
        errors.push(row.error.clone());
    }

    let mut columns = vec![
        "sample_index".to_string(),
        table.axis_name.clone(),
        "status".to_string(),
    ];
    columns.extend(table.column_order.clone());
    columns.push("path".to_string());
    columns.push("warnings".to_string());
    columns.push("error".to_string());

    let series = PlotSeries {
        name: output_key.to_string(),
        x,
        y,
        x_label: table.axis_name.clone(),
        y_label: output_key.to_string(),
        x_unit: None,
        y_unit: None,
    };

    StudyResult {
        table: StudyTable {
            columns,
            rows: table.to_spill_strings(true),
        },
        series: vec![series],
        meta: StudyMeta {
            study_id: table.study_id,
            n_ok,
            n_fail,
            warnings_summary: warning_set.into_iter().collect(),
            selected_output_key: output_key.to_string(),
        },
        diagnostics: Some(DiagnosticsBundle {
            per_row_path: path,
            per_row_error: errors,
        }),
    }
}

fn run_single_equation(
    descriptor: &StudyTargetDescriptor,
    req: SingleSolveRequest,
) -> Result<SingleSolveResult, BridgeError> {
    let mut given_keys = BTreeSet::<String>::new();
    for (k, v) in &req.inputs {
        if k == "branch" {
            continue;
        }
        let provided = v.as_f64().is_some() || v.as_str().is_some();
        if provided {
            given_keys.insert(k.clone());
        }
    }
    let mut branch = None::<String>;
    let output_key = if let Some(k) = req.output_key.clone() {
        k
    } else {
        let missing = descriptor
            .input_fields
            .iter()
            .filter(|f| f.key != "branch")
            .filter(|f| !given_keys.contains(&f.key))
            .map(|f| f.key.clone())
            .collect::<Vec<_>>();
        match missing.len() {
            1 => missing[0].clone(),
            0 => descriptor.default_output.clone().ok_or_else(|| {
                BridgeError::InvalidRequest(
                    "equation solve requires an explicit target".to_string(),
                )
            })?,
            _ => {
                return Err(BridgeError::InvalidRequest(
                    "provide exactly one unknown equation variable or set output_key".to_string(),
                ));
            }
        }
    };
    if !descriptor.outputs.iter().any(|o| o.key == output_key) {
        return Err(BridgeError::InvalidRequest(format!(
            "invalid equation target '{output_key}' for '{}'",
            req.target_id
        )));
    }

    let mut solve = eng::eq
        .solve(req.target_id.as_str())
        .for_target(&output_key);
    for (k, v) in &req.inputs {
        if k == "branch" {
            branch = v.as_str().map(|s| s.to_string());
            continue;
        }
        if let Some(n) = v.as_f64() {
            solve = solve.given(k.clone(), n);
            given_keys.insert(k.clone());
        } else if let Some(s) = v.as_str() {
            solve = solve.given(k.clone(), s.to_string());
            given_keys.insert(k.clone());
        } else {
            return Err(BridgeError::InvalidRequest(format!(
                "unsupported input type for equation field '{}'",
                k
            )));
        }
    }
    if let Some(b) = branch {
        solve = solve.branch(&b);
    }
    let v = solve
        .value()
        .map_err(|e| BridgeError::Engine(e.to_string()))?;
    let unit = descriptor
        .outputs
        .iter()
        .find(|o| o.key == output_key)
        .and_then(|o| o.unit.clone());
    Ok(SingleSolveResult {
        target_kind: StudyTargetKind::Equation,
        target_id: req.target_id,
        output_key,
        value: Value::from(v),
        unit,
        path_text: None,
        warnings: Vec::new(),
    })
}

fn run_single_device(
    _descriptor: &StudyTargetDescriptor,
    req: SingleSolveRequest,
) -> Result<SingleSolveResult, BridgeError> {
    let device = eng::solve::studyable_devices()
        .into_iter()
        .find(|d| d.device_key == req.target_id)
        .ok_or_else(|| {
            BridgeError::InvalidRequest(format!("unknown device '{}'", req.target_id))
        })?;
    let output_key = req.output_key.unwrap_or_else(|| "value".to_string());
    let value = match output_key.as_str() {
        "value" | "result" => invoke_op_value(&device.value_op, &req.inputs)?,
        "pivot" => {
            let op = device.pivot_op.clone().ok_or_else(|| {
                BridgeError::InvalidRequest(
                    "pivot output is not available for this device".to_string(),
                )
            })?;
            invoke_op_value(&op, &req.inputs)?
        }
        "path_text" => {
            let op = device.path_op.clone().ok_or_else(|| {
                BridgeError::InvalidRequest(
                    "path_text output is not available for this device".to_string(),
                )
            })?;
            invoke_op_value(&op, &req.inputs)?
        }
        other => {
            let root = invoke_op_value(&device.main_op, &req.inputs)?;
            select_json(&root, other).cloned().ok_or_else(|| {
                BridgeError::InvalidRequest(format!("unknown device output '{other}'"))
            })?
        }
    };
    let path_text = if let Some(op) = device.path_op.clone() {
        invoke_op_value(&op, &req.inputs)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
    } else {
        None
    };
    Ok(SingleSolveResult {
        target_kind: StudyTargetKind::Device,
        target_id: req.target_id,
        output_key,
        value,
        unit: None,
        path_text,
        warnings: Vec::new(),
    })
}

fn run_single_workflow(
    _descriptor: &StudyTargetDescriptor,
    req: SingleSolveRequest,
) -> Result<SingleSolveResult, BridgeError> {
    let wf = eng::solve::studyable_workflows()
        .into_iter()
        .find(|w| w.key == req.target_id)
        .ok_or_else(|| {
            BridgeError::InvalidRequest(format!("unknown workflow '{}'", req.target_id))
        })?;
    let output_key = req
        .output_key
        .or_else(|| wf.default_columns.first().map(|s| s.to_string()))
        .ok_or_else(|| {
            BridgeError::InvalidRequest("workflow has no default outputs".to_string())
        })?;
    let root = invoke_op_value(wf.eval_op, &req.inputs)?;
    let value = root
        .get("outputs")
        .and_then(|v| select_json(v, &output_key))
        .cloned()
        .ok_or_else(|| {
            BridgeError::InvalidRequest(format!("unknown workflow output '{output_key}'"))
        })?;
    let warnings = root
        .get("warnings")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(Value::as_str)
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let path_text = root
        .get("path_text")
        .and_then(Value::as_str)
        .map(|s| s.to_string());
    Ok(SingleSolveResult {
        target_kind: StudyTargetKind::Workflow,
        target_id: req.target_id,
        output_key,
        value,
        unit: None,
        path_text,
        warnings,
    })
}

fn parse_field_value(field: &StudyFieldDescriptor, raw: &str) -> Result<Value, String> {
    match field.field_type {
        StudyFieldType::Enum => {
            if field.enum_options.is_empty()
                || field
                    .enum_options
                    .iter()
                    .any(|o| o.key.eq_ignore_ascii_case(raw))
            {
                Ok(Value::from(raw.to_string()))
            } else {
                Err(format!(
                    "unsupported value '{}'; expected one of: {}",
                    raw,
                    field
                        .enum_options
                        .iter()
                        .map(|o| o.key.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
            }
        }
        StudyFieldType::Bool => {
            let v = match raw.to_ascii_lowercase().as_str() {
                "1" | "true" | "yes" | "y" => true,
                "0" | "false" | "no" | "n" => false,
                _ => {
                    return Err("expected boolean value (true/false/yes/no/1/0)".to_string());
                }
            };
            Ok(Value::from(v))
        }
        StudyFieldType::Int => raw
            .parse::<i64>()
            .map(Value::from)
            .map_err(|_| "expected integer".to_string()),
        StudyFieldType::Float => {
            if let Ok(v) = raw.parse::<f64>() {
                return Ok(Value::from(v));
            }
            if let Some(dim) = &field.dimension {
                eng_core::units::parse_equation_quantity_to_si(dim, raw)
                    .map_err(|e| e.to_string())?;
            } else {
                eng_core::units::parse_quantity_expression(raw).map_err(|e| e.to_string())?;
            }
            Ok(Value::from(raw.to_string()))
        }
        StudyFieldType::String => Ok(Value::from(raw.to_string())),
    }
}

fn invoke_op_value(op: &str, args: &Map<String, Value>) -> Result<Value, BridgeError> {
    let req = eng::bindings::InvokeRequest {
        protocol_version: eng::bindings::INVOKE_PROTOCOL_VERSION.to_string(),
        op: op.to_string(),
        request_id: None,
        args: Value::Object(args.clone()),
    };
    let resp = eng::invoke::handle_invoke(req);
    if !resp.ok {
        let err = resp
            .error
            .map(|e| format!("[{}] {}", e.code, e.message))
            .unwrap_or_else(|| "unknown invoke error".to_string());
        return Err(BridgeError::Engine(err));
    }
    resp.value
        .ok_or_else(|| BridgeError::Engine("missing invoke value".to_string()))
}

fn select_json<'a>(value: &'a Value, selector: &str) -> Option<&'a Value> {
    let mut cur = value;
    for part in selector.split('.') {
        if part.is_empty() {
            return None;
        }
        cur = cur.get(part)?;
    }
    Some(cur)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_discovery_returns_equation_device_and_workflow() {
        let targets = list_study_targets().expect("targets");
        assert!(targets.iter().any(|t| t.kind == StudyTargetKind::Equation));
        assert!(targets.iter().any(|t| t.kind == StudyTargetKind::Device));
        assert!(targets.iter().any(|t| t.kind == StudyTargetKind::Workflow));
    }

    #[test]
    fn equation_descriptor_has_sweep_and_default_output() {
        let d = describe_equation_target("compressible.isentropic_pressure_ratio").expect("desc");
        assert!(d.sweepable_fields.iter().any(|s| s == "M"));
        assert!(d.default_output.is_some());
        assert!(!d.outputs.is_empty());
    }

    #[test]
    fn validation_rejects_invalid_enum_value() {
        let d = describe_device_target("nozzle_flow_calc").expect("device descriptor");
        let req = StudyRunRequest {
            target_kind: StudyTargetKind::Device,
            target_id: d.id.clone(),
            sweep_field: "input_value".to_string(),
            axis: SweepAxisSpec::linspace(1.0, 2.0, 3),
            inputs: Map::from_iter([
                ("input_kind".to_string(), Value::from("not_valid")),
                ("target_kind".to_string(), Value::from("mach")),
                ("gamma".to_string(), Value::from(1.4)),
            ]),
            output_key: Some("value".to_string()),
        };
        let err = run_study_from_form(req).expect_err("should reject invalid enum");
        assert!(err.to_string().contains("unsupported value"));
    }

    #[test]
    fn workflow_study_bridge_supports_output_selection() {
        let out = run_workflow_study(WorkflowStudyRequest {
            workflow_key: "nozzle_normal_shock_chain".to_string(),
            sweep_arg: "area_ratio".to_string(),
            axis: SweepAxisSpec::linspace(1.4, 2.0, 4),
            fixed_args: Map::from_iter([
                ("gamma".to_string(), Value::from(1.4)),
                ("branch".to_string(), Value::from("supersonic")),
            ]),
            output_key: Some("post_shock_mach".to_string()),
        })
        .expect("workflow study");
        assert_eq!(out.series[0].name, "post_shock_mach");
        assert_eq!(out.series[0].x.len(), 4);
    }

    #[test]
    fn list_presets_contains_production_presets() {
        let presets = list_study_presets().expect("presets");
        assert!(!presets.is_empty());
        assert!(presets.iter().any(|p| p.id.contains("isentropic")));
    }

    #[test]
    fn single_solve_equation_supports_missing_one_variable_target() {
        let req = SingleSolveRequest {
            target_kind: StudyTargetKind::Equation,
            target_id: "compressible.isentropic_temperature_ratio".to_string(),
            inputs: Map::from_iter([
                ("M".to_string(), Value::from(2.0)),
                ("gamma".to_string(), Value::from(1.4)),
            ]),
            output_key: None,
        };
        let out = run_single_solve(req).expect("single equation solve");
        assert_eq!(out.output_key, "T_T0");
        assert!(out.value.as_f64().unwrap_or(0.0) > 0.0);
    }

    #[test]
    fn single_solve_device_supports_value_output() {
        let req = SingleSolveRequest {
            target_kind: StudyTargetKind::Device,
            target_id: "normal_shock_calc".to_string(),
            inputs: Map::from_iter([
                ("input_kind".to_string(), Value::from("m1")),
                ("input_value".to_string(), Value::from(2.0)),
                ("target_kind".to_string(), Value::from("p2_p1")),
                ("gamma".to_string(), Value::from(1.4)),
            ]),
            output_key: Some("value".to_string()),
        };
        let out = run_single_solve(req).expect("single device solve");
        assert_eq!(out.output_key, "value");
        assert!(out.value.as_f64().unwrap_or(0.0) > 1.0);
    }

    #[test]
    fn equation_descriptor_exposes_display_metadata() {
        let d = describe_equation_target("compressible.isentropic_pressure_ratio").expect("desc");
        assert!(d.display_latex.is_some());
    }

    #[test]
    fn validate_single_solve_accepts_numeric_and_unit_inputs() {
        let mut raw = BTreeMap::new();
        raw.insert("P".to_string(), "2000000".to_string());
        raw.insert("r".to_string(), "0.25".to_string());
        raw.insert("t".to_string(), "0.01".to_string());
        let v = validate_single_solve(
            StudyTargetKind::Equation,
            "structures.hoop_stress",
            &raw,
            Some("sigma_h"),
        )
        .expect("validation");
        assert!(v.ready, "blocking: {:?}", v.blocking_reasons);

        raw.insert("P".to_string(), "2 MPa".to_string());
        raw.insert("r".to_string(), "0.25 m".to_string());
        raw.insert("t".to_string(), "0.01 m".to_string());
        let v = validate_single_solve(
            StudyTargetKind::Equation,
            "structures.hoop_stress",
            &raw,
            Some("sigma_h"),
        )
        .expect("validation with units");
        assert!(v.ready, "blocking: {:?}", v.blocking_reasons);
    }

    #[test]
    fn validate_single_solve_infers_equation_target_for_one_unknown() {
        let raw = BTreeMap::from([
            ("M".to_string(), "2.0".to_string()),
            ("gamma".to_string(), "1.4".to_string()),
        ]);
        let v = validate_single_solve(
            StudyTargetKind::Equation,
            "compressible.isentropic_temperature_ratio",
            &raw,
            None,
        )
        .expect("validation");
        assert_eq!(v.output_key.as_deref(), Some("T_T0"));
        assert!(v.ready);
    }

    #[test]
    fn validate_single_solve_blocks_when_multiple_unknowns() {
        let raw = BTreeMap::new();
        let v = validate_single_solve(
            StudyTargetKind::Equation,
            "compressible.isentropic_temperature_ratio",
            &raw,
            None,
        )
        .expect("validation");
        assert!(!v.ready);
        assert!(
            v.blocking_reasons
                .iter()
                .any(|m| m.contains("exactly one unknown"))
        );
    }

    #[test]
    fn hoop_stress_one_unknown_inference_executes_with_units() {
        let raw = BTreeMap::from([
            ("P".to_string(), "500 psia".to_string()),
            ("r".to_string(), "1.2 in".to_string()),
            ("t".to_string(), "0.12 in".to_string()),
        ]);
        let (v, solved) = evaluate_single_solve(
            StudyTargetKind::Equation,
            "structures.hoop_stress",
            &raw,
            None,
        )
        .expect("evaluation");
        assert_eq!(v.output_key.as_deref(), Some("sigma_h"));
        let solved = solved.expect("solve result");
        assert_eq!(solved.output_key, "sigma_h");
        assert!(solved.value.as_f64().unwrap_or(0.0) > 0.0);
    }

    #[test]
    fn hoop_stress_inverse_targets_execute() {
        let base = BTreeMap::from([
            ("P".to_string(), "500 psia".to_string()),
            ("r".to_string(), "1.2 in".to_string()),
            ("t".to_string(), "0.12 in".to_string()),
            ("sigma_h".to_string(), "5000 psia".to_string()),
        ]);
        for target in ["P", "r", "t"] {
            let mut raw = base.clone();
            raw.insert(target.to_string(), String::new());
            let (_, solved) = evaluate_single_solve(
                StudyTargetKind::Equation,
                "structures.hoop_stress",
                &raw,
                None,
            )
            .expect("evaluation");
            let solved = solved.expect("solve result");
            assert_eq!(solved.output_key, target);
        }
    }

    #[test]
    fn validation_ready_and_execution_stay_consistent() {
        let raw = BTreeMap::from([
            ("M".to_string(), "2.0".to_string()),
            ("gamma".to_string(), "1.4".to_string()),
        ]);
        let v = validate_single_solve(
            StudyTargetKind::Equation,
            "compressible.isentropic_pressure_ratio",
            &raw,
            Some("p_p0"),
        )
        .expect("validation");
        assert!(v.ready);
        let (_, solved) = evaluate_single_solve(
            StudyTargetKind::Equation,
            "compressible.isentropic_pressure_ratio",
            &raw,
            Some("p_p0"),
        )
        .expect("evaluation");
        assert!(solved.is_some());
    }
}
