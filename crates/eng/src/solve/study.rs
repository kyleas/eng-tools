use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use crate::devices::{IsentropicBranch, NozzleFlowBranch};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SweepAxis {
    Values(Vec<f64>),
    Linspace { start: f64, end: f64, count: usize },
    Logspace { start: f64, end: f64, count: usize },
}

impl SweepAxis {
    pub fn values(values: impl Into<Vec<f64>>) -> Self {
        Self::Values(values.into())
    }

    pub fn linspace(start: f64, end: f64, count: usize) -> Self {
        Self::Linspace { start, end, count }
    }

    pub fn logspace(start: f64, end: f64, count: usize) -> Self {
        Self::Logspace { start, end, count }
    }

    pub fn samples(&self) -> Vec<f64> {
        match self {
            Self::Values(v) => v.clone(),
            Self::Linspace { start, end, count } => linspace(*start, *end, *count),
            Self::Logspace { start, end, count } => logspace(*start, *end, *count),
        }
    }
}

fn linspace(start: f64, end: f64, count: usize) -> Vec<f64> {
    if count == 0 {
        return Vec::new();
    }
    if count == 1 {
        return vec![start];
    }
    let step = (end - start) / ((count - 1) as f64);
    (0..count).map(|i| start + (i as f64) * step).collect()
}

fn logspace(start: f64, end: f64, count: usize) -> Vec<f64> {
    if start <= 0.0 || end <= 0.0 {
        return Vec::new();
    }
    let log_start = start.log10();
    let log_end = end.log10();
    linspace(log_start, log_end, count)
        .into_iter()
        .map(|v| 10f64.powf(v))
        .collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StudySampleStatus {
    Ok,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StudyCell {
    Number(f64),
    Text(String),
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyRow {
    pub sample_index: usize,
    pub sample_value: f64,
    pub status: StudySampleStatus,
    pub outputs: BTreeMap<String, f64>,
    pub warnings: Vec<String>,
    pub path_summary: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyTable {
    pub study_id: String,
    pub axis_name: String,
    pub column_order: Vec<String>,
    pub rows: Vec<StudyRow>,
}

impl StudyTable {
    pub fn to_spill_cells(&self, include_header: bool) -> Vec<Vec<StudyCell>> {
        let mut out = Vec::<Vec<StudyCell>>::new();
        if include_header {
            let mut header = vec![
                StudyCell::Text("sample_index".to_string()),
                StudyCell::Text(self.axis_name.clone()),
                StudyCell::Text("status".to_string()),
            ];
            for c in &self.column_order {
                header.push(StudyCell::Text(c.clone()));
            }
            header.push(StudyCell::Text("path".to_string()));
            header.push(StudyCell::Text("warnings".to_string()));
            header.push(StudyCell::Text("error".to_string()));
            out.push(header);
        }
        for row in &self.rows {
            let mut r = Vec::new();
            r.push(StudyCell::Number(row.sample_index as f64));
            r.push(StudyCell::Number(row.sample_value));
            r.push(StudyCell::Text(match row.status {
                StudySampleStatus::Ok => "ok".to_string(),
                StudySampleStatus::Failed => "failed".to_string(),
            }));
            for c in &self.column_order {
                if let Some(v) = row.outputs.get(c) {
                    r.push(StudyCell::Number(*v));
                } else {
                    r.push(StudyCell::Empty);
                }
            }
            r.push(
                row.path_summary
                    .as_ref()
                    .map(|v| StudyCell::Text(v.clone()))
                    .unwrap_or(StudyCell::Empty),
            );
            r.push(if row.warnings.is_empty() {
                StudyCell::Empty
            } else {
                StudyCell::Text(row.warnings.join(" | "))
            });
            r.push(
                row.error
                    .as_ref()
                    .map(|v| StudyCell::Text(v.clone()))
                    .unwrap_or(StudyCell::Empty),
            );
            out.push(r);
        }
        out
    }

    pub fn to_spill_strings(&self, include_header: bool) -> Vec<Vec<String>> {
        self.to_spill_cells(include_header)
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|cell| match cell {
                        StudyCell::Number(v) => format!("{v:.12}"),
                        StudyCell::Text(v) => v,
                        StudyCell::Empty => String::new(),
                    })
                    .collect()
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct StudyEval {
    pub outputs: BTreeMap<String, f64>,
    pub warnings: Vec<String>,
    pub path_summary: Option<String>,
}

impl StudyEval {
    pub fn one(output_name: &str, value: f64) -> Self {
        let mut outputs = BTreeMap::new();
        outputs.insert(output_name.to_string(), value);
        Self {
            outputs,
            warnings: Vec::new(),
            path_summary: None,
        }
    }
}

pub fn run_study_1d<F>(
    study_id: &str,
    axis_name: &str,
    axis: SweepAxis,
    column_order: Vec<String>,
    mut f: F,
) -> StudyTable
where
    F: FnMut(f64) -> Result<StudyEval, String>,
{
    let rows = axis
        .samples()
        .into_iter()
        .enumerate()
        .map(|(i, sample)| match f(sample) {
            Ok(eval) => StudyRow {
                sample_index: i,
                sample_value: sample,
                status: StudySampleStatus::Ok,
                outputs: eval.outputs,
                warnings: eval.warnings,
                path_summary: eval.path_summary,
                error: None,
            },
            Err(err) => StudyRow {
                sample_index: i,
                sample_value: sample,
                status: StudySampleStatus::Failed,
                outputs: BTreeMap::new(),
                warnings: Vec::new(),
                path_summary: None,
                error: Some(err),
            },
        })
        .collect();
    StudyTable {
        study_id: study_id.to_string(),
        axis_name: axis_name.to_string(),
        column_order,
        rows,
    }
}

#[derive(Debug, Clone)]
pub struct EquationStudySpec {
    pub path_id: String,
    pub target: String,
    pub sweep_variable: String,
    pub fixed_inputs: BTreeMap<String, f64>,
    pub branch: Option<String>,
}

pub fn run_equation_study(spec: &EquationStudySpec, axis: SweepAxis) -> StudyTable {
    let path_id = spec.path_id.clone();
    let target = spec.target.clone();
    let sweep_variable = spec.sweep_variable.clone();
    let fixed_inputs = spec.fixed_inputs.clone();
    let branch = spec.branch.clone();
    let axis_name = sweep_variable.clone();
    run_study_1d(
        "equation_sweep",
        &axis_name,
        axis,
        vec![target.clone()],
        move |sample| {
            let mut solve = crate::eq.solve(path_id.as_str()).for_target(&target);
            for (k, v) in &fixed_inputs {
                solve = solve.given(k.as_str(), *v);
            }
            solve = solve.given(sweep_variable.as_str(), sample);
            if let Some(b) = &branch {
                solve = solve.branch(b.as_str());
            }
            let value = solve.value().map_err(|e| e.to_string())?;
            let mut out = BTreeMap::new();
            out.insert(target.clone(), value);
            Ok(StudyEval {
                outputs: out,
                warnings: Vec::new(),
                path_summary: Some(format!(
                    "equation.solve {}:{}",
                    path_id.as_str(),
                    target.as_str()
                )),
            })
        },
    )
}

#[derive(Debug, Clone)]
pub struct DeviceStudySpec {
    pub device_key: String,
    pub sweep_arg: String,
    pub axis: SweepAxis,
    pub fixed_args: Map<String, Value>,
    pub requested_outputs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WorkflowStudySpec {
    pub workflow_key: String,
    pub sweep_arg: String,
    pub axis: SweepAxis,
    pub fixed_args: Map<String, Value>,
}

#[derive(Debug, Clone)]
pub struct StudyableDeviceSpec {
    pub device_key: String,
    pub main_op: String,
    pub value_op: String,
    pub path_op: Option<String>,
    pub pivot_op: Option<String>,
}

pub fn studyable_devices() -> Vec<StudyableDeviceSpec> {
    crate::devices::generation_specs()
        .into_iter()
        .filter_map(|spec| {
            let mut main_op: Option<String> = None;
            let mut value_op: Option<String> = None;
            let mut path_op: Option<String> = None;
            let mut pivot_op: Option<String> = None;
            for bf in spec.binding_functions {
                if bf.id == format!("device.{}", spec.key) {
                    main_op = Some(format!("device.{}", spec.key));
                    value_op = Some(bf.op.to_string());
                } else if bf.id.ends_with(".path_text") {
                    path_op = Some(bf.op.to_string());
                } else if bf.id.ends_with(".pivot") {
                    pivot_op = Some(bf.op.to_string());
                } else if bf.id.starts_with(&format!("device.{}.", spec.key))
                    && value_op.is_none()
                    && bf.returns == "f64"
                {
                    // Non-calculator devices can still be studyable when they expose
                    // one numeric binding operation.
                    value_op = Some(bf.op.to_string());
                }
            }
            value_op.map(|op| StudyableDeviceSpec {
                device_key: spec.key.to_string(),
                main_op: main_op.unwrap_or_else(|| format!("device.{}", spec.key)),
                value_op: op,
                path_op,
                pivot_op,
            })
        })
        .collect()
}

pub fn run_device_study(spec: &DeviceStudySpec) -> Result<StudyTable, String> {
    let Some(device_spec) = studyable_devices()
        .into_iter()
        .find(|d| d.device_key == spec.device_key)
    else {
        return Err(format!(
            "unknown or non-studyable device '{}'",
            spec.device_key
        ));
    };
    let mut columns = if spec.requested_outputs.is_empty() {
        vec!["value".to_string()]
    } else {
        spec.requested_outputs.clone()
    };
    if columns.iter().any(|c| c == "path_text") {
        columns.retain(|c| c != "path_text");
    }

    Ok(run_study_1d(
        "device_sweep",
        &spec.sweep_arg,
        spec.axis.clone(),
        columns,
        |sample| {
            let mut args = spec.fixed_args.clone();
            args.insert(spec.sweep_arg.clone(), json!(sample));
            let needs_main_response = spec
                .requested_outputs
                .iter()
                .any(|c| !matches!(c.as_str(), "value" | "result" | "pivot" | "path_text"));
            let main_response = if needs_main_response {
                Some(invoke_value(&device_spec.main_op, &args)?)
            } else {
                None
            };

            let mut outputs = BTreeMap::new();
            let mut warnings = Vec::new();
            let mut path_summary = None;

            if spec
                .requested_outputs
                .iter()
                .any(|c| c == "value" || c == "result")
            {
                let v = invoke_value(&device_spec.value_op, &args)?;
                let n = v
                    .as_f64()
                    .ok_or_else(|| "device value op did not return numeric scalar".to_string())?;
                outputs.insert("value".to_string(), n);
            }

            if spec.requested_outputs.iter().any(|c| c == "pivot") {
                if let Some(op) = &device_spec.pivot_op {
                    let v = invoke_value(op, &args)?;
                    if let Some(n) = v.as_f64() {
                        outputs.insert("pivot".to_string(), n);
                    } else {
                        warnings.push("pivot op returned non-numeric value".to_string());
                    }
                } else {
                    warnings.push("pivot output requested but pivot op not available".to_string());
                }
            }

            if spec.requested_outputs.iter().any(|c| c == "path_text") {
                if let Some(op) = &device_spec.path_op {
                    let v = invoke_value(op, &args)?;
                    path_summary = v.as_str().map(|s| s.to_string());
                } else {
                    warnings.push(
                        "path_text requested but path diagnostics op not available".to_string(),
                    );
                }
            }

            for c in &spec.requested_outputs {
                if matches!(c.as_str(), "value" | "result" | "pivot" | "path_text") {
                    continue;
                }
                if let Some(v) = main_response.as_ref().and_then(|v| select_json(v, c)) {
                    if let Some(n) = v.as_f64() {
                        outputs.insert(c.clone(), n);
                    } else {
                        warnings.push(format!(
                            "requested output '{c}' is non-numeric in device response"
                        ));
                    }
                } else {
                    warnings.push(format!("requested output '{c}' missing in device response"));
                }
            }

            Ok(StudyEval {
                outputs,
                warnings,
                path_summary,
            })
        },
    ))
}

pub fn run_workflow_study(spec: &WorkflowStudySpec) -> Result<StudyTable, String> {
    let workflow = crate::solve::workflow::studyable_workflows()
        .into_iter()
        .find(|w| w.key == spec.workflow_key)
        .ok_or_else(|| format!("unknown workflow '{}'", spec.workflow_key))?;
    let mut args = spec.fixed_args.clone();
    let axis = spec.axis.samples();
    let mut rows = Vec::new();
    for (i, sample) in axis.into_iter().enumerate() {
        args.insert(spec.sweep_arg.clone(), json!(sample));
        let resp = invoke_response(workflow.eval_op, &args)?;
        let outputs_obj = resp
            .get("outputs")
            .and_then(Value::as_object)
            .ok_or_else(|| "workflow eval op did not return outputs object".to_string())?;
        let mut outputs = BTreeMap::new();
        for col in workflow.default_columns {
            if let Some(v) = outputs_obj.get(*col).and_then(Value::as_f64) {
                outputs.insert((*col).to_string(), v);
            }
        }
        let warnings = resp
            .get("warnings")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(Value::as_str)
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let path_summary = resp
            .get("path_text")
            .and_then(Value::as_str)
            .map(|s| s.to_string());
        let row = StudyRow {
            sample_index: i,
            sample_value: sample,
            status: StudySampleStatus::Ok,
            outputs,
            warnings,
            path_summary,
            error: None,
        };
        rows.push(StudyRow {
            sample_index: i,
            sample_value: sample,
            ..row
        });
    }
    Ok(StudyTable {
        study_id: "workflow_sweep".to_string(),
        axis_name: spec.sweep_arg.clone(),
        column_order: workflow
            .default_columns
            .iter()
            .map(|s| s.to_string())
            .collect(),
        rows,
    })
}

fn invoke_value(op: &str, args: &Map<String, Value>) -> Result<Value, String> {
    let resp = invoke_response(op, args)?;
    Ok(resp)
}

fn invoke_response(op: &str, args: &Map<String, Value>) -> Result<Value, String> {
    let req = crate::bindings::InvokeRequest {
        protocol_version: crate::bindings::INVOKE_PROTOCOL_VERSION.to_string(),
        op: op.to_string(),
        request_id: None,
        args: Value::Object(args.clone()),
    };
    let resp = crate::invoke::handle_invoke(req);
    if !resp.ok {
        let err = resp
            .error
            .map(|e| format!("[{}] {}", e.code, e.message))
            .unwrap_or_else(|| "unknown invoke error".to_string());
        return Err(err);
    }
    resp.value
        .ok_or_else(|| "missing invoke response value".to_string())
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

fn isentropic_branch_key(branch: IsentropicBranch) -> &'static str {
    match branch {
        IsentropicBranch::Subsonic => "subsonic",
        IsentropicBranch::Supersonic => "supersonic",
    }
}

fn nozzle_branch_key(branch: NozzleFlowBranch) -> &'static str {
    match branch {
        NozzleFlowBranch::Subsonic => "subsonic",
        NozzleFlowBranch::Supersonic => "supersonic",
    }
}

pub fn study_isentropic_m_to_p_p0(
    gamma: f64,
    axis: SweepAxis,
    branch: Option<IsentropicBranch>,
) -> StudyTable {
    let mut fixed_args = Map::new();
    fixed_args.insert("input_kind".to_string(), json!("mach"));
    fixed_args.insert("target_kind".to_string(), json!("pressure_ratio"));
    fixed_args.insert("gamma".to_string(), json!(gamma));
    if let Some(b) = branch {
        fixed_args.insert("branch".to_string(), json!(isentropic_branch_key(b)));
    }
    let spec = DeviceStudySpec {
        device_key: "isentropic_calc".to_string(),
        sweep_arg: "input_value".to_string(),
        axis,
        fixed_args,
        requested_outputs: vec![
            "value".to_string(),
            "pivot".to_string(),
            "path_text".to_string(),
        ],
    };
    run_device_study(&spec).unwrap_or_else(|err| StudyTable {
        study_id: "study.isentropic.m_to_p_p0".to_string(),
        axis_name: "input_value".to_string(),
        column_order: vec!["value".to_string(), "pivot".to_string()],
        rows: vec![StudyRow {
            sample_index: 0,
            sample_value: f64::NAN,
            status: StudySampleStatus::Failed,
            outputs: BTreeMap::new(),
            warnings: Vec::new(),
            path_summary: None,
            error: Some(err),
        }],
    })
}

pub fn study_nozzle_flow_area_ratio(
    gamma: f64,
    axis: SweepAxis,
    branch: NozzleFlowBranch,
) -> StudyTable {
    let mut fixed_args = Map::new();
    fixed_args.insert("input_kind".to_string(), json!("area_ratio"));
    fixed_args.insert("target_kind".to_string(), json!("mach"));
    fixed_args.insert("gamma".to_string(), json!(gamma));
    fixed_args.insert("branch".to_string(), json!(nozzle_branch_key(branch)));
    let spec = DeviceStudySpec {
        device_key: "nozzle_flow_calc".to_string(),
        sweep_arg: "input_value".to_string(),
        axis,
        fixed_args,
        requested_outputs: vec![
            "value".to_string(),
            "pivot".to_string(),
            "path_text".to_string(),
        ],
    };
    run_device_study(&spec).unwrap_or_else(|err| StudyTable {
        study_id: "study.nozzle_flow.area_ratio".to_string(),
        axis_name: "input_value".to_string(),
        column_order: vec!["value".to_string(), "pivot".to_string()],
        rows: vec![StudyRow {
            sample_index: 0,
            sample_value: f64::NAN,
            status: StudySampleStatus::Failed,
            outputs: BTreeMap::new(),
            warnings: Vec::new(),
            path_summary: None,
            error: Some(err),
        }],
    })
}

pub fn study_normal_shock_m1(gamma: f64, axis: SweepAxis) -> StudyTable {
    let mut fixed_args = Map::new();
    fixed_args.insert("input_kind".to_string(), json!("m1"));
    fixed_args.insert("target_kind".to_string(), json!("m2"));
    fixed_args.insert("gamma".to_string(), json!(gamma));
    let spec = DeviceStudySpec {
        device_key: "normal_shock_calc".to_string(),
        sweep_arg: "input_value".to_string(),
        axis,
        fixed_args,
        requested_outputs: vec![
            "value".to_string(),
            "pivot".to_string(),
            "path_text".to_string(),
        ],
    };
    run_device_study(&spec).unwrap_or_else(|err| StudyTable {
        study_id: "study.normal_shock.m1".to_string(),
        axis_name: "input_value".to_string(),
        column_order: vec!["value".to_string(), "pivot".to_string()],
        rows: vec![StudyRow {
            sample_index: 0,
            sample_value: f64::NAN,
            status: StudySampleStatus::Failed,
            outputs: BTreeMap::new(),
            warnings: Vec::new(),
            path_summary: None,
            error: Some(err),
        }],
    })
}

pub fn study_nozzle_normal_shock_workflow(
    gamma: f64,
    axis: SweepAxis,
    nozzle_branch: NozzleFlowBranch,
) -> StudyTable {
    let mut fixed_args = Map::new();
    fixed_args.insert("gamma".to_string(), json!(gamma));
    fixed_args.insert(
        "branch".to_string(),
        json!(nozzle_branch_key(nozzle_branch)),
    );
    let spec = WorkflowStudySpec {
        workflow_key: "nozzle_normal_shock_chain".to_string(),
        sweep_arg: "area_ratio".to_string(),
        axis,
        fixed_args,
    };
    run_workflow_study(&spec).unwrap_or_else(|err| StudyTable {
        study_id: "study.workflow.nozzle_normal_shock".to_string(),
        axis_name: "area_ratio".to_string(),
        column_order: vec![],
        rows: vec![StudyRow {
            sample_index: 0,
            sample_value: f64::NAN,
            status: StudySampleStatus::Failed,
            outputs: BTreeMap::new(),
            warnings: Vec::new(),
            path_summary: None,
            error: Some(err),
        }],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sweep_axis_generators_are_stable() {
        assert_eq!(
            SweepAxis::linspace(0.0, 1.0, 3).samples(),
            vec![0.0, 0.5, 1.0]
        );
        assert_eq!(SweepAxis::values(vec![1.0, 2.0]).samples(), vec![1.0, 2.0]);
        let ls = SweepAxis::logspace(1.0, 100.0, 3).samples();
        assert_eq!(ls.len(), 3);
        assert!((ls[0] - 1.0).abs() < 1e-12);
        assert!((ls[2] - 100.0).abs() < 1e-9);
    }

    #[test]
    fn equation_study_captures_per_row_failures() {
        let mut fixed = BTreeMap::new();
        fixed.insert("gamma".to_string(), 1.4);
        let spec = EquationStudySpec {
            path_id: "compressible.isentropic_pressure_ratio".to_string(),
            target: "p_p0".to_string(),
            sweep_variable: "M".to_string(),
            fixed_inputs: fixed,
            branch: None,
        };
        let table = run_equation_study(&spec, SweepAxis::values(vec![0.5, 2.0]));
        assert_eq!(table.rows.len(), 2);
        assert!(matches!(table.rows[0].status, StudySampleStatus::Ok));
        assert!(matches!(table.rows[1].status, StudySampleStatus::Ok));
    }

    #[test]
    fn studyable_devices_cover_registered_device_specs() {
        let registered = crate::devices::generation_specs()
            .into_iter()
            .map(|s| s.key.to_string())
            .collect::<std::collections::BTreeSet<_>>();
        let studyable = studyable_devices()
            .into_iter()
            .map(|d| d.device_key)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(registered, studyable);
    }

    #[test]
    fn generic_device_study_runner_handles_multiple_devices() {
        let mut isen_args = serde_json::Map::new();
        isen_args.insert("input_kind".to_string(), json!("mach"));
        isen_args.insert("target_kind".to_string(), json!("pressure_ratio"));
        isen_args.insert("gamma".to_string(), json!(1.4));
        let isen = run_device_study(&DeviceStudySpec {
            device_key: "isentropic_calc".to_string(),
            sweep_arg: "input_value".to_string(),
            axis: SweepAxis::values(vec![0.5, 1.5]),
            fixed_args: isen_args,
            requested_outputs: vec![
                "value".to_string(),
                "pivot".to_string(),
                "path_text".to_string(),
            ],
        })
        .expect("isentropic generic study should run");
        assert_eq!(isen.rows.len(), 2);
        assert!(matches!(isen.rows[0].status, StudySampleStatus::Ok));

        let mut nshock_args = serde_json::Map::new();
        nshock_args.insert("input_kind".to_string(), json!("m1"));
        nshock_args.insert("target_kind".to_string(), json!("m2"));
        nshock_args.insert("gamma".to_string(), json!(1.4));
        let nshock = run_device_study(&DeviceStudySpec {
            device_key: "normal_shock_calc".to_string(),
            sweep_arg: "input_value".to_string(),
            axis: SweepAxis::values(vec![1.5, 2.0]),
            fixed_args: nshock_args,
            requested_outputs: vec!["value".to_string(), "pivot".to_string()],
        })
        .expect("normal-shock generic study should run");
        assert_eq!(nshock.rows.len(), 2);
        assert!(matches!(nshock.rows[1].status, StudySampleStatus::Ok));
    }

    #[test]
    fn workflow_study_uses_solve_layer_chain() {
        let mut fixed = serde_json::Map::new();
        fixed.insert("gamma".to_string(), json!(1.4));
        fixed.insert("branch".to_string(), json!("supersonic"));
        let table = run_workflow_study(&WorkflowStudySpec {
            workflow_key: "nozzle_normal_shock_chain".to_string(),
            sweep_arg: "area_ratio".to_string(),
            axis: SweepAxis::values(vec![2.0]),
            fixed_args: fixed,
        })
        .expect("workflow study should run");
        assert_eq!(table.rows.len(), 1);
        assert!(matches!(table.rows[0].status, StudySampleStatus::Ok));
        assert!(
            table.rows[0]
                .path_summary
                .as_deref()
                .unwrap_or("")
                .contains("device.normal_shock_calc")
        );
    }
}
