use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::devices::{
    IsentropicBranch, IsentropicInputKind, IsentropicOutputKind, NormalShockInputKind,
    NormalShockOutputKind, NozzleFlowBranch, NozzleFlowInputKind, NozzleFlowOutputKind,
    isentropic_calc, normal_shock_calc, nozzle_flow_calc,
};
use crate::solve::{
    NozzleShockWorkflowRequest, QuantityProvenance, run_nozzle_normal_shock_workflow,
};

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

pub fn study_isentropic_m_to_p_p0(
    gamma: f64,
    axis: SweepAxis,
    branch: Option<IsentropicBranch>,
) -> StudyTable {
    run_study_1d(
        "study.isentropic.m_to_p_p0",
        "mach",
        axis,
        vec!["pressure_ratio".to_string(), "pivot_mach".to_string()],
        move |mach| {
            let mut dev = isentropic_calc()
                .gamma(gamma)
                .input(IsentropicInputKind::Mach, mach)
                .target(IsentropicOutputKind::PressureRatio);
            if let Some(b) = branch {
                dev = dev.branch(b);
            }
            let r = dev.solve().map_err(|e| e.to_string())?;
            let mut out = BTreeMap::new();
            out.insert("pressure_ratio".to_string(), r.value_si);
            out.insert("pivot_mach".to_string(), r.pivot_mach);
            let path_text = r.path_text();
            Ok(StudyEval {
                outputs: out,
                warnings: r.warnings,
                path_summary: Some(path_text),
            })
        },
    )
}

pub fn study_nozzle_flow_area_ratio(
    gamma: f64,
    axis: SweepAxis,
    branch: NozzleFlowBranch,
) -> StudyTable {
    run_study_1d(
        "study.nozzle_flow.area_ratio",
        "area_ratio",
        axis,
        vec![
            "mach".to_string(),
            "pressure_ratio".to_string(),
            "pivot_mach".to_string(),
        ],
        move |area_ratio| {
            let m = nozzle_flow_calc()
                .gamma(gamma)
                .input(NozzleFlowInputKind::AreaRatio, area_ratio)
                .target(NozzleFlowOutputKind::Mach)
                .branch(branch)
                .solve()
                .map_err(|e| e.to_string())?;
            let p = nozzle_flow_calc()
                .gamma(gamma)
                .input(NozzleFlowInputKind::Mach, m.pivot_mach)
                .target(NozzleFlowOutputKind::PressureRatio)
                .solve()
                .map_err(|e| e.to_string())?;
            let mut out = BTreeMap::new();
            out.insert("mach".to_string(), m.value_si);
            out.insert("pressure_ratio".to_string(), p.value_si);
            out.insert("pivot_mach".to_string(), m.pivot_mach);
            let path_text = format!("{} | {}", m.path_text(), p.path_text());
            Ok(StudyEval {
                outputs: out,
                warnings: m.warnings,
                path_summary: Some(path_text),
            })
        },
    )
}

pub fn study_normal_shock_m1(gamma: f64, axis: SweepAxis) -> StudyTable {
    run_study_1d(
        "study.normal_shock.m1",
        "m1",
        axis,
        vec![
            "m2".to_string(),
            "pressure_ratio".to_string(),
            "pivot_m1".to_string(),
        ],
        move |m1| {
            let m2 = normal_shock_calc()
                .gamma(gamma)
                .input(NormalShockInputKind::M1, m1)
                .target(NormalShockOutputKind::M2)
                .solve()
                .map_err(|e| e.to_string())?;
            let p = normal_shock_calc()
                .gamma(gamma)
                .input(NormalShockInputKind::M1, m1)
                .target(NormalShockOutputKind::PressureRatio)
                .solve()
                .map_err(|e| e.to_string())?;
            let mut out = BTreeMap::new();
            out.insert("m2".to_string(), m2.value_si);
            out.insert("pressure_ratio".to_string(), p.value_si);
            out.insert("pivot_m1".to_string(), m2.pivot_m1);
            let path_text = format!("{} | {}", m2.path_text(), p.path_text());
            Ok(StudyEval {
                outputs: out,
                warnings: m2.warnings,
                path_summary: Some(path_text),
            })
        },
    )
}

pub fn study_nozzle_normal_shock_workflow(
    gamma: f64,
    axis: SweepAxis,
    nozzle_branch: NozzleFlowBranch,
) -> StudyTable {
    run_study_1d(
        "study.workflow.nozzle_normal_shock",
        "area_ratio",
        axis,
        vec![
            "pre_shock_mach".to_string(),
            "post_shock_mach".to_string(),
            "shock_pressure_ratio".to_string(),
            "s1_mach_provenance".to_string(),
        ],
        move |area_ratio| {
            let r = run_nozzle_normal_shock_workflow(NozzleShockWorkflowRequest {
                gamma,
                area_ratio,
                nozzle_branch,
            })
            .map_err(|e| e.to_string())?;
            let mut out = BTreeMap::new();
            out.insert("pre_shock_mach".to_string(), r.pre_shock_mach);
            out.insert("post_shock_mach".to_string(), r.post_shock_mach);
            out.insert("shock_pressure_ratio".to_string(), r.shock_pressure_ratio);
            let prov = r
                .workflow
                .station("s1_pre_shock")
                .and_then(|s| s.quantities.get("mach"))
                .map(|q| match q.provenance {
                    QuantityProvenance::Input => 0.0,
                    QuantityProvenance::Solved => 1.0,
                    QuantityProvenance::Propagated => 2.0,
                })
                .unwrap_or(-1.0);
            out.insert("s1_mach_provenance".to_string(), prov);
            let path_text = r.path_text();
            let warnings = r.workflow.warnings.clone();
            Ok(StudyEval {
                outputs: out,
                warnings,
                path_summary: Some(path_text),
            })
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn workflow_study_uses_solve_layer_chain() {
        let table = study_nozzle_normal_shock_workflow(
            1.4,
            SweepAxis::values(vec![2.0]),
            NozzleFlowBranch::Supersonic,
        );
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
