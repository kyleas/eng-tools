# Engineering Solve Layer

`eng::solve` is the canonical home for reusable multi-step solve/workflow infrastructure.

## Ownership Scope

- Shared numeric root solve wrappers with convergence diagnostics
- Shared ODE step wrappers for engineering integrations
- Station/state chaining and step provenance records
- Workflow-level warnings and structured errors

## Out of Scope

- Atomic equation definitions (stay in YAML/registry)
- Device-specific binding naming/polish logic
- Full arbitrary graph optimization engines

## Rust Example

```rust
use eng::devices::NozzleFlowBranch;
use eng::solve::{NozzleShockWorkflowRequest, run_nozzle_normal_shock_workflow};

let chain = run_nozzle_normal_shock_workflow(NozzleShockWorkflowRequest {
    gamma: 1.4,
    area_ratio: 2.0,
    nozzle_branch: NozzleFlowBranch::Supersonic,
})?;
println!("station trace: {}", chain.path_text());
```

## Standardized Numeric Homes

- `eng::solve::numeric`: bracketed root solve and scan+bisect helpers.
- `eng::solve::ode`: reusable RK4 stepping wrapper.

Conical-shock Taylor-Maccoll stepping and branch-sensitive oblique/conical inversions use this layer.
