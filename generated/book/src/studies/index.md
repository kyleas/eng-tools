# Studies and Parameter Sweeps

`eng::solve::study` is the standard subsystem for diagnostics-aware parameter studies across equations, devices, and solve-layer workflows.

## Scope

- 1D sweeps (`values`, `linspace`, `logspace`)
- per-row status (`ok`/`failed`) without aborting the whole study
- table-first outputs suitable for Python and Excel spill ranges
- concise per-row path/provenance summaries

## Rust: Equation Study

```rust
use std::collections::BTreeMap;
use eng::solve::{EquationStudySpec, SweepAxis, run_equation_study};

let mut fixed = BTreeMap::new();
fixed.insert("gamma".to_string(), 1.4);
let table = run_equation_study(&EquationStudySpec {
    path_id: "compressible.isentropic_pressure_ratio".to_string(),
    target: "p_p0".to_string(),
    sweep_variable: "M".to_string(),
    fixed_inputs: fixed,
    branch: None,
}, SweepAxis::linspace(0.2, 3.0, 21));
```

## Rust: Device Study

```rust
use eng::devices::NozzleFlowBranch;
use eng::solve::{SweepAxis, study_nozzle_flow_area_ratio};

let table = study_nozzle_flow_area_ratio(
    1.4,
    SweepAxis::linspace(1.2, 3.0, 20),
    NozzleFlowBranch::Supersonic,
);
```

## Rust: Workflow-Chain Study

```rust
use eng::devices::NozzleFlowBranch;
use eng::solve::{SweepAxis, study_nozzle_normal_shock_workflow};

let table = study_nozzle_normal_shock_workflow(
    1.4,
    SweepAxis::linspace(1.2, 3.0, 20),
    NozzleFlowBranch::Supersonic,
);
```

## Python / Excel (Targeted v1)

- Python module: `engpy.study`
- Excel spill-table helpers:
  - `ENG_STUDY_ISENTROPIC_M_TO_P_P0_TABLE(...)`
  - `ENG_STUDY_NOZZLE_FLOW_TABLE(...)`
  - `ENG_STUDY_NORMAL_SHOCK_TABLE(...)`
  - `ENG_STUDY_NOZZLE_NORMAL_SHOCK_WORKFLOW_TABLE(...)`

Each helper returns a structured payload with both a rich `table` object and `spill` rows suitable for worksheet charting.
