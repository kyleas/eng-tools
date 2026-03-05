# Nozzle Flow Calculator

**Key:** `nozzle_flow_calc`

Calculator-style quasi-1D nozzle device: solve isentropic nozzle input kinds to target kinds through Mach pivot orchestration.

## Overview

A calculator-style quasi-1D nozzle-flow device that resolves a pivot Mach number from one supported input kind and then evaluates the requested target kind.

### Supported input kinds
- `mach`
- `area_ratio` (`A/A*`, branch-sensitive inverse)
- `pressure_ratio` (`p/p0`)
- `temperature_ratio` (`T/T0`)
- `density_ratio` (`rho/rho0`)

### Supported target kinds
- `mach`
- `area_ratio`
- `pressure_ratio`
- `temperature_ratio`
- `density_ratio`
- `p` (requires `p0`)
- `t` (requires `t0`)
- `rho` (requires `rho0`)

### Branch behavior
- `area_ratio -> mach` is double-valued and requires `subsonic` or `supersonic`.

### Rust
```rust
use eng::devices::{nozzle_flow_calc, NozzleFlowBranch, NozzleFlowInputKind, NozzleFlowOutputKind};
let out = nozzle_flow_calc()
    .gamma(1.4)
    .input(NozzleFlowInputKind::AreaRatio, 2.0)
    .target(NozzleFlowOutputKind::Mach)
    .branch(NozzleFlowBranch::Supersonic)
    .solve()?;
println!("M={}, value={}", out.pivot_mach, out.value_si);
```

## Modes

- Input kinds: Mach, A/A*, p/p0, T/T0, rho/rho0
- Branch-aware inversion for A/A* -> Mach
- Optional stagnation-reference scaling for static p/T/rho outputs

## Outputs

- value_si
- pivot_mach
- path diagnostics
## Internal Composition

- [Compressible Area Mach](../equations/compressible/area_mach.md)
- [Compressible Isentropic Pressure Ratio](../equations/compressible/isentropic_pressure_ratio.md)
- [Compressible Isentropic Temperature Ratio](../equations/compressible/isentropic_temperature_ratio.md)
- [Compressible Isentropic Density Ratio](../equations/compressible/isentropic_density_ratio.md)

## Bindings

### Python
```python
engpy.devices.nozzle_flow_calc("area_ratio", 2.0, "mach", 1.4, None, None, None, "supersonic")
engpy.devices.nozzle_flow_from_m_to_a_astar(2.0, 1.4)
engpy.devices.nozzle_flow_from_m_to_p_p0(2.0, 1.4)
engpy.devices.nozzle_flow_path_text("mach", 2.0, "p", 1.4, 2.0e6, None, None, "")
```

### Excel
```excel
=ENG_NOZZLE_FLOW("area_ratio",2.0,"mach",1.4,NA(),NA(),NA(),"supersonic")
=ENG_NOZZLE_FLOW_FROM_M_TO_A_ASTAR(2.0,1.4)
=ENG_NOZZLE_FLOW_FROM_A_ASTAR_TO_M(2.0,1.4,"subsonic")
=ENG_NOZZLE_FLOW_FROM_M_TO_P_P0(2.0,1.4)
=ENG_NOZZLE_FLOW_FROM_M_TO_T_T0(2.0,1.4)
=ENG_NOZZLE_FLOW_FROM_M_TO_RHO_RHO0(2.0,1.4)
=ENG_NOZZLE_FLOW("mach",2.0,"p",1.4,2000000,NA(),NA(),"")
=ENG_NOZZLE_FLOW_PATH_TEXT("area_ratio",2.0,"mach",1.4,NA(),NA(),NA(),"supersonic")
```

**Excel arguments**
- `input_kind`: `mach`, `area_ratio`, `pressure_ratio`, `temperature_ratio`, `density_ratio`
- `input_value`: input value
- `target_kind`: `mach`, `area_ratio`, `pressure_ratio`, `temperature_ratio`, `density_ratio`, `p`, `t`, `rho`
- `gamma`: specific heat ratio
- `p0`, `t0`, `rho0`: optional stagnation references required for static outputs `p`, `t`, `rho`
- `branch`: required for `area_ratio -> mach`
