use std::collections::BTreeMap;

use thiserror::Error;

use crate::devices::{
    NormalShockInputKind, NormalShockOutputKind, NozzleFlowBranch, NozzleFlowInputKind,
    NozzleFlowOutputKind, normal_shock_calc, nozzle_flow_calc,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantityProvenance {
    Input,
    Solved,
    Propagated,
}

#[derive(Debug, Clone)]
pub struct QuantityRecord {
    pub value_si: f64,
    pub provenance: QuantityProvenance,
    pub produced_by: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StationState {
    pub name: String,
    pub quantities: BTreeMap<String, QuantityRecord>,
}

#[derive(Debug, Clone)]
pub struct WorkflowStepTrace {
    pub label: String,
    pub operation_id: String,
    pub path_text: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct WorkflowRun {
    pub stations: Vec<StationState>,
    pub steps: Vec<WorkflowStepTrace>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WorkflowStudySpecEntry {
    pub key: &'static str,
    pub name: &'static str,
    pub summary: &'static str,
    pub eval_op: &'static str,
    pub default_sweep_arg: &'static str,
    pub default_columns: &'static [&'static str],
    pub input_fields: &'static [WorkflowInputFieldSpec],
    pub output_fields: &'static [WorkflowOutputFieldSpec],
}

#[derive(Debug, Clone)]
pub enum WorkflowFieldType {
    Float,
    Int,
    Bool,
    Enum,
    String,
}

#[derive(Debug, Clone)]
pub struct WorkflowInputFieldSpec {
    pub key: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub field_type: WorkflowFieldType,
    pub required: bool,
    pub sweepable: bool,
    pub enum_options: &'static [&'static str],
    pub default_value: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct WorkflowOutputFieldSpec {
    pub key: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub plottable: bool,
}

const NOZZLE_NORMAL_SHOCK_INPUT_FIELDS: &[WorkflowInputFieldSpec] = &[
    WorkflowInputFieldSpec {
        key: "gamma",
        label: "Gamma",
        description: "Specific heat ratio",
        field_type: WorkflowFieldType::Float,
        required: true,
        sweepable: true,
        enum_options: &[],
        default_value: Some("1.4"),
    },
    WorkflowInputFieldSpec {
        key: "area_ratio",
        label: "Area ratio (A/A*)",
        description: "Nozzle area ratio",
        field_type: WorkflowFieldType::Float,
        required: true,
        sweepable: true,
        enum_options: &[],
        default_value: Some("2.0"),
    },
    WorkflowInputFieldSpec {
        key: "branch",
        label: "Branch",
        description: "Nozzle branch for area_ratio -> Mach inversion",
        field_type: WorkflowFieldType::Enum,
        required: false,
        sweepable: false,
        enum_options: &["subsonic", "supersonic"],
        default_value: Some("supersonic"),
    },
];

const NOZZLE_NORMAL_SHOCK_OUTPUT_FIELDS: &[WorkflowOutputFieldSpec] = &[
    WorkflowOutputFieldSpec {
        key: "pre_shock_mach",
        label: "Pre-shock Mach",
        description: "Mach immediately before shock",
        plottable: true,
    },
    WorkflowOutputFieldSpec {
        key: "post_shock_mach",
        label: "Post-shock Mach",
        description: "Mach immediately after normal shock",
        plottable: true,
    },
    WorkflowOutputFieldSpec {
        key: "shock_pressure_ratio",
        label: "Shock pressure ratio",
        description: "Normal-shock static pressure ratio p2/p1",
        plottable: true,
    },
    WorkflowOutputFieldSpec {
        key: "s1_mach_provenance",
        label: "S1 Mach provenance",
        description: "Provenance code (0 input, 1 solved, 2 propagated)",
        plottable: true,
    },
];

pub fn studyable_workflows() -> Vec<WorkflowStudySpecEntry> {
    vec![WorkflowStudySpecEntry {
        key: "nozzle_normal_shock_chain",
        name: "Nozzle + Normal Shock Chain",
        summary: "Chained station workflow: nozzle area-ratio solve followed by normal-shock propagation.",
        eval_op: "workflow.nozzle_normal_shock.eval",
        default_sweep_arg: "area_ratio",
        default_columns: &[
            "pre_shock_mach",
            "post_shock_mach",
            "shock_pressure_ratio",
            "s1_mach_provenance",
        ],
        input_fields: NOZZLE_NORMAL_SHOCK_INPUT_FIELDS,
        output_fields: NOZZLE_NORMAL_SHOCK_OUTPUT_FIELDS,
    }]
}

impl WorkflowRun {
    pub fn station(&self, name: &str) -> Option<&StationState> {
        self.stations.iter().find(|s| s.name == name)
    }

    pub fn path_text(&self) -> String {
        self.steps
            .iter()
            .map(|s| format!("{} -> {}", s.operation_id, s.path_text))
            .collect::<Vec<_>>()
            .join(" => ")
    }
}

pub struct WorkflowBuilder {
    run: WorkflowRun,
}

impl WorkflowBuilder {
    pub fn new() -> Self {
        Self {
            run: WorkflowRun::default(),
        }
    }

    pub fn add_station_input(&mut self, station: &str, key: &str, value_si: f64) {
        let st = self.ensure_station(station);
        st.quantities.insert(
            key.to_string(),
            QuantityRecord {
                value_si,
                provenance: QuantityProvenance::Input,
                produced_by: None,
            },
        );
    }

    pub fn add_station_solved(&mut self, station: &str, key: &str, value_si: f64, op: &str) {
        let st = self.ensure_station(station);
        st.quantities.insert(
            key.to_string(),
            QuantityRecord {
                value_si,
                provenance: QuantityProvenance::Solved,
                produced_by: Some(op.to_string()),
            },
        );
    }

    pub fn propagate(
        &mut self,
        from_station: &str,
        to_station: &str,
        key: &str,
        operation: &str,
    ) -> Result<(), WorkflowError> {
        let value = self
            .run
            .station(from_station)
            .and_then(|s| s.quantities.get(key))
            .map(|q| q.value_si)
            .ok_or_else(|| WorkflowError::MissingQuantity {
                station: from_station.to_string(),
                key: key.to_string(),
            })?;
        let st = self.ensure_station(to_station);
        st.quantities.insert(
            key.to_string(),
            QuantityRecord {
                value_si: value,
                provenance: QuantityProvenance::Propagated,
                produced_by: Some(operation.to_string()),
            },
        );
        Ok(())
    }

    pub fn add_step(&mut self, label: &str, operation_id: &str, path_text: String) {
        self.run.steps.push(WorkflowStepTrace {
            label: label.to_string(),
            operation_id: operation_id.to_string(),
            path_text,
            warnings: Vec::new(),
        });
    }

    pub fn finish(self) -> WorkflowRun {
        self.run
    }

    fn ensure_station(&mut self, name: &str) -> &mut StationState {
        if let Some(i) = self.run.stations.iter().position(|s| s.name == name) {
            return &mut self.run.stations[i];
        }
        self.run.stations.push(StationState {
            name: name.to_string(),
            quantities: BTreeMap::new(),
        });
        self.run.stations.last_mut().expect("just pushed station")
    }
}

#[derive(Debug, Clone)]
pub struct NozzleShockWorkflowRequest {
    pub gamma: f64,
    pub area_ratio: f64,
    pub nozzle_branch: NozzleFlowBranch,
}

#[derive(Debug, Clone)]
pub struct NozzleShockWorkflowResult {
    pub pre_shock_mach: f64,
    pub post_shock_mach: f64,
    pub shock_pressure_ratio: f64,
    pub workflow: WorkflowRun,
}

impl NozzleShockWorkflowResult {
    pub fn path_text(&self) -> String {
        self.workflow.path_text()
    }
}

#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error(transparent)]
    Nozzle(#[from] crate::devices::NozzleFlowCalcError),
    #[error(transparent)]
    NormalShock(#[from] crate::devices::NormalShockCalcError),
    #[error("missing quantity '{key}' in station '{station}'")]
    MissingQuantity { station: String, key: String },
}

pub fn run_nozzle_normal_shock_workflow(
    req: NozzleShockWorkflowRequest,
) -> Result<NozzleShockWorkflowResult, WorkflowError> {
    let mut wf = WorkflowBuilder::new();
    wf.add_station_input("s0_inlet", "gamma", req.gamma);
    wf.add_station_input("s0_inlet", "area_ratio", req.area_ratio);

    let nozzle_pre = nozzle_flow_calc()
        .gamma(req.gamma)
        .input(NozzleFlowInputKind::AreaRatio, req.area_ratio)
        .target(NozzleFlowOutputKind::Mach)
        .branch(req.nozzle_branch)
        .solve()?;
    wf.add_step(
        "Nozzle Area->Mach",
        "device.nozzle_flow_calc",
        nozzle_pre.path_text(),
    );
    wf.add_station_solved(
        "s1_pre_shock",
        "mach",
        nozzle_pre.pivot_mach,
        "device.nozzle_flow_calc",
    );
    wf.propagate("s0_inlet", "s1_pre_shock", "gamma", "propagate.gamma")?;

    let nozzle_ratio = nozzle_flow_calc()
        .gamma(req.gamma)
        .input(NozzleFlowInputKind::Mach, nozzle_pre.pivot_mach)
        .target(NozzleFlowOutputKind::PressureRatio)
        .solve()?;
    wf.add_step(
        "Nozzle Mach->p/p0",
        "device.nozzle_flow_calc",
        nozzle_ratio.path_text(),
    );
    wf.add_station_solved(
        "s1_pre_shock",
        "pressure_ratio",
        nozzle_ratio.value_si,
        "device.nozzle_flow_calc",
    );

    let shock_m2 = normal_shock_calc()
        .gamma(req.gamma)
        .input(NormalShockInputKind::M1, nozzle_pre.pivot_mach)
        .target(NormalShockOutputKind::M2)
        .solve()?;
    wf.add_step(
        "Normal Shock M1->M2",
        "device.normal_shock_calc",
        shock_m2.path_text(),
    );
    wf.add_station_solved(
        "s2_post_shock",
        "mach",
        shock_m2.value_si,
        "device.normal_shock_calc",
    );
    wf.propagate("s1_pre_shock", "s2_post_shock", "gamma", "propagate.gamma")?;

    let shock_p_ratio = normal_shock_calc()
        .gamma(req.gamma)
        .input(NormalShockInputKind::M1, nozzle_pre.pivot_mach)
        .target(NormalShockOutputKind::PressureRatio)
        .solve()?;
    wf.add_step(
        "Normal Shock M1->p2/p1",
        "device.normal_shock_calc",
        shock_p_ratio.path_text(),
    );
    wf.add_station_solved(
        "s2_post_shock",
        "shock_pressure_ratio",
        shock_p_ratio.value_si,
        "device.normal_shock_calc",
    );

    Ok(NozzleShockWorkflowResult {
        pre_shock_mach: nozzle_pre.pivot_mach,
        post_shock_mach: shock_m2.value_si,
        shock_pressure_ratio: shock_p_ratio.value_si,
        workflow: wf.finish(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chained_nozzle_shock_workflow_produces_station_provenance() {
        let out = run_nozzle_normal_shock_workflow(NozzleShockWorkflowRequest {
            gamma: 1.4,
            area_ratio: 2.0,
            nozzle_branch: NozzleFlowBranch::Supersonic,
        })
        .expect("workflow should solve");
        assert!(out.pre_shock_mach > 1.0);
        assert!(out.post_shock_mach > 0.0 && out.post_shock_mach < out.pre_shock_mach);
        assert!(out.shock_pressure_ratio > 1.0);
        assert!(
            out.workflow
                .station("s1_pre_shock")
                .and_then(|s| s.quantities.get("mach"))
                .is_some()
        );
        assert!(
            out.path_text().contains("device.nozzle_flow_calc")
                && out.path_text().contains("device.normal_shock_calc")
        );
    }
}
