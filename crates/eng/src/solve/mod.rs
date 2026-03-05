pub mod numeric;
pub mod ode;
pub mod workflow;

pub use numeric::{
    ConvergenceReport, RootScanResult, RootSolveError, RootSolveOutput, bisect_by_sign_change,
    find_roots_by_scan_bisection,
};
pub use ode::{OdeSolveError, rk4_step_2};
pub use workflow::{
    NozzleShockWorkflowRequest, NozzleShockWorkflowResult, QuantityProvenance, QuantityRecord,
    StationState, WorkflowError, WorkflowRun, WorkflowStepTrace, run_nozzle_normal_shock_workflow,
};
