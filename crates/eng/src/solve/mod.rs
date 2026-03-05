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
    EquationStudySpec, StudyCell, StudyEval, StudyRow, StudySampleStatus, StudyTable, SweepAxis,
    run_equation_study, run_study_1d, study_isentropic_m_to_p_p0, study_normal_shock_m1,
    study_nozzle_flow_area_ratio, study_nozzle_normal_shock_workflow,
};
pub use workflow::{
    NozzleShockWorkflowRequest, NozzleShockWorkflowResult, QuantityProvenance, QuantityRecord,
    StationState, WorkflowError, WorkflowRun, WorkflowStepTrace, run_nozzle_normal_shock_workflow,
};
