pub mod numeric;
pub mod ode;
pub mod study;
pub mod workflow;

pub use numeric::{
    ConvergenceReport, RootScanResult, RootSolveError, RootSolveOutput, bisect_by_sign_change,
    find_roots_by_scan_bisection,
};
pub use ode::{OdeSolveError, rk4_step_2};
pub use study::{
    DeviceStudySpec, EquationStudySpec, StudyCell, StudyEval, StudyRow, StudySampleStatus,
    StudyTable, StudyableDeviceSpec, SweepAxis, WorkflowStudySpec, run_device_study,
    run_equation_study, run_study_1d, run_workflow_study, study_isentropic_m_to_p_p0,
    study_normal_shock_m1, study_nozzle_flow_area_ratio, study_nozzle_normal_shock_workflow,
    studyable_devices,
};
pub use workflow::{
    NozzleShockWorkflowRequest, NozzleShockWorkflowResult, QuantityProvenance, QuantityRecord,
    StationState, WorkflowError, WorkflowRun, WorkflowStepTrace, WorkflowStudySpecEntry,
    run_nozzle_normal_shock_workflow, studyable_workflows,
};
