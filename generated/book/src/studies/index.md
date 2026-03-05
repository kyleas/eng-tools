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

## Rust: Device Study (Generic by Device Key)

```rust
use serde_json::{Map, json};
use eng::solve::{DeviceStudySpec, SweepAxis, run_device_study};

let mut fixed = Map::new();
fixed.insert("input_kind".to_string(), json!("area_ratio"));
fixed.insert("target_kind".to_string(), json!("mach"));
fixed.insert("gamma".to_string(), json!(1.4));
fixed.insert("branch".to_string(), json!("supersonic"));
let table = run_device_study(&DeviceStudySpec {
    device_key: "nozzle_flow_calc".to_string(),
    sweep_arg: "input_value".to_string(),
    axis: SweepAxis::linspace(1.2, 3.0, 20),
    fixed_args: fixed,
    requested_outputs: vec!["value".to_string(), "pivot".to_string(), "path_text".to_string()],
})?;
```

## Rust: Workflow-Chain Study (Generic by Workflow Key)

```rust
use serde_json::{Map, json};
use eng::solve::{WorkflowStudySpec, SweepAxis, run_workflow_study};

let mut fixed = Map::new();
fixed.insert("gamma".to_string(), json!(1.4));
fixed.insert("branch".to_string(), json!("supersonic"));
let table = run_workflow_study(&WorkflowStudySpec {
    workflow_key: "nozzle_normal_shock_chain".to_string(),
    sweep_arg: "area_ratio".to_string(),
    axis: SweepAxis::linspace(1.2, 3.0, 20),
    fixed_args: fixed,
})?;
```

## Python / Excel

- Python module: `engpy.study`
- Generic helpers:
  - `engpy.study.equation_sweep_table(...)`
  - `engpy.study.device_table(...)`
  - `engpy.study.workflow_table(...)`
- Excel spill-table helpers:
  - `ENG_STUDY_EQUATION_TABLE(...)`
  - `ENG_STUDY_DEVICE_TABLE(...)`
  - `ENG_STUDY_WORKFLOW_TABLE(...)`
- Optional named convenience wrappers:
  - `ENG_STUDY_ISENTROPIC_M_TO_P_P0_TABLE(...)`
  - `ENG_STUDY_NOZZLE_FLOW_TABLE(...)`
  - `ENG_STUDY_NORMAL_SHOCK_TABLE(...)`
  - `ENG_STUDY_NOZZLE_NORMAL_SHOCK_WORKFLOW_TABLE(...)`

Each helper returns a structured payload with both a rich `table` object and `spill` rows suitable for worksheet charting.
