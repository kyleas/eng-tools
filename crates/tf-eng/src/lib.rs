//! tf-eng bridge: Thermoflow-facing integration layer over `eng`.
//!
//! Ownership boundary:
//! - `eng` owns equation/device/workflow/study execution logic.
//! - `tf-eng` owns app-facing request/response normalization (plot/table/diagnostics).
//! - Thermoflow UI/CLI should call this crate instead of re-implementing study logic.

use std::collections::BTreeMap;

use eng::solve::{
    DeviceStudySpec as EngDeviceStudySpec, EquationStudySpec as EngEquationStudySpec,
    StudySampleStatus, StudyTable as EngStudyTable, SweepAxis as EngSweepAxis,
    WorkflowStudySpec as EngWorkflowStudySpec,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use thiserror::Error;

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

    let mut warning_set = std::collections::BTreeSet::new();
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
    use serde_json::json;

    #[test]
    fn equation_study_bridge_produces_plot_ready_series() {
        let out = run_equation_study(EquationStudyRequest {
            path_id: "compressible.isentropic_pressure_ratio".to_string(),
            target: "p_p0".to_string(),
            sweep_variable: "M".to_string(),
            axis: SweepAxisSpec::linspace(0.2, 2.0, 5),
            fixed_inputs: BTreeMap::from([("gamma".to_string(), 1.4)]),
            branch: None,
            output_key: None,
        })
        .expect("equation study");
        assert_eq!(out.series.len(), 1);
        assert_eq!(out.series[0].x.len(), 5);
        assert_eq!(out.series[0].y.len(), 5);
        assert_eq!(out.meta.n_fail, 0);
    }

    #[test]
    fn device_study_bridge_handles_failures_without_panicking() {
        let out = run_device_study(DeviceStudyRequest {
            device_key: "fanno_flow_calc".to_string(),
            sweep_arg: "input_value".to_string(),
            axis: SweepAxisSpec::Values(vec![-1.0, 0.5, 1.5]),
            fixed_args: Map::from_iter([
                ("input_kind".to_string(), json!("m")),
                ("target_kind".to_string(), json!("p_pstar")),
                ("gamma".to_string(), json!(1.4)),
            ]),
            requested_outputs: vec!["value".to_string(), "path_text".to_string()],
            output_key: Some("value".to_string()),
        })
        .expect("device study");
        assert_eq!(out.series[0].x.len(), 3);
        assert_eq!(out.series[0].y.len(), 3);
        assert!(out.meta.n_fail >= 1);
    }

    #[test]
    fn workflow_study_bridge_supports_output_selection() {
        let out = run_workflow_study(WorkflowStudyRequest {
            workflow_key: "nozzle_normal_shock_chain".to_string(),
            sweep_arg: "area_ratio".to_string(),
            axis: SweepAxisSpec::linspace(1.4, 2.0, 4),
            fixed_args: Map::from_iter([
                ("gamma".to_string(), json!(1.4)),
                ("branch".to_string(), json!("supersonic")),
            ]),
            output_key: Some("post_shock_mach".to_string()),
        })
        .expect("workflow study");
        assert_eq!(out.series[0].name, "post_shock_mach");
        assert_eq!(out.series[0].x.len(), 4);
    }
}
