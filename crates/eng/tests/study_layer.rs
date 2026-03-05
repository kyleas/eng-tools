use std::collections::BTreeMap;

use eng::devices::NozzleFlowBranch;
use eng::solve::{
    EquationStudySpec, StudySampleStatus, SweepAxis, run_equation_study,
    study_isentropic_m_to_p_p0, study_normal_shock_m1, study_nozzle_flow_area_ratio,
    study_nozzle_normal_shock_workflow,
};

#[test]
fn sweep_axis_generation_is_deterministic() {
    assert_eq!(
        SweepAxis::linspace(0.0, 1.0, 3).samples(),
        vec![0.0, 0.5, 1.0]
    );
    assert_eq!(SweepAxis::values(vec![2.0, 3.0]).samples(), vec![2.0, 3.0]);
    let log = SweepAxis::logspace(1.0, 100.0, 3).samples();
    assert_eq!(log.len(), 3);
    assert!((log[0] - 1.0).abs() < 1e-12);
    assert!((log[2] - 100.0).abs() < 1e-9);
}

#[test]
fn equation_study_rows_capture_success_and_failure_without_aborting() {
    let mut fixed = BTreeMap::new();
    fixed.insert("gamma".to_string(), 1.4);
    let table = run_equation_study(
        &EquationStudySpec {
            path_id: "compressible.prandtl_meyer".to_string(),
            target: "M".to_string(),
            sweep_variable: "nu".to_string(),
            fixed_inputs: fixed,
            branch: None,
        },
        SweepAxis::values(vec![0.2, 2.8]),
    );
    assert_eq!(table.rows.len(), 2);
    assert!(matches!(table.rows[0].status, StudySampleStatus::Ok));
    assert!(matches!(table.rows[1].status, StudySampleStatus::Failed));
    assert!(table.rows[1].error.as_deref().unwrap_or("").contains("nu"));
}

#[test]
fn device_studies_produce_table_outputs() {
    let isen = study_isentropic_m_to_p_p0(1.4, SweepAxis::linspace(0.2, 2.0, 6), None);
    assert!(!isen.rows.is_empty());
    assert!(
        isen.rows
            .iter()
            .all(|r| matches!(r.status, StudySampleStatus::Ok))
    );
    assert!(
        isen.rows
            .iter()
            .all(|r| r.outputs.contains_key("value"))
    );

    let nozzle = study_nozzle_flow_area_ratio(
        1.4,
        SweepAxis::linspace(1.2, 2.0, 4),
        NozzleFlowBranch::Supersonic,
    );
    assert_eq!(nozzle.rows.len(), 4);
    assert!(
        nozzle
            .rows
            .iter()
            .all(|r| r.outputs.contains_key("value"))
    );

    let normal = study_normal_shock_m1(1.4, SweepAxis::linspace(1.1, 2.0, 4));
    assert_eq!(normal.rows.len(), 4);
    assert!(normal.rows.iter().all(|r| r.outputs.contains_key("value")));
}

#[test]
fn workflow_chain_study_contains_path_and_station_outputs() {
    let flow = study_nozzle_normal_shock_workflow(
        1.4,
        SweepAxis::values(vec![1.6, 2.0]),
        NozzleFlowBranch::Supersonic,
    );
    assert_eq!(flow.rows.len(), 2);
    assert!(
        flow.rows
            .iter()
            .all(|r| matches!(r.status, StudySampleStatus::Ok))
    );
    assert!(
        flow.rows[0]
            .path_summary
            .as_deref()
            .unwrap_or("")
            .contains("device.nozzle_flow_calc")
    );
    let spill = flow.to_spill_strings(true);
    assert!(!spill.is_empty());
    assert_eq!(spill[0][0], "sample_index");
}
