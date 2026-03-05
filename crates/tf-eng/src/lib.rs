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
use serde_json::{Map, Value};
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
}
