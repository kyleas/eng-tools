# Fanno Flow Calculator

**Key:** `fanno_flow_calc`

Calculator-style compressible device: solve Fanno star-reference input kinds to target kinds through Mach pivot orchestration.

## Overview

A calculator-style compressible device that resolves a pivot Mach number from one supported Fanno input kind and then evaluates the requested target kind.

### Supported input kinds
- `mach`
- `t_tstar` (`T/T*`)
- `p_pstar` (`p/p*`)
- `rho_rhostar` (`rho/rho*`)
- `p0_p0star` (`p0/p0*`)
- `four_flstar_d` (`4fL*/D`)

### Supported target kinds
- `mach`
- `t_tstar`
- `p_pstar`
- `rho_rhostar`
- `p0_p0star`
- `v_vstar`
- `four_flstar_d`

### Branch behavior
- Inverse paths (`ratio -> mach`) are branch-sensitive and require `subsonic` or `supersonic`.

### Rust
```rust
use eng::devices::{fanno_flow_calc, FannoFlowBranch, FannoFlowInputKind, FannoFlowOutputKind};
let out = fanno_flow_calc()
    .gamma(1.4)
    .input(FannoFlowInputKind::FrictionParameter, 0.3049965025814798)
    .target(FannoFlowOutputKind::Mach)
    .branch(FannoFlowBranch::Supersonic)
    .solve()?;
println!("M={}, value={}", out.pivot_mach, out.value_si);
```

## Modes

- Input kinds: Mach, T/T*, p/p*, rho/rho*, p0/p0*, 4fL*/D
- Branch-aware inversion for ratio -> Mach

## Outputs

- value_si
- pivot_mach
- path diagnostics
## Internal Composition

- [Compressible Fanno Temperature Ratio](../equations/compressible/fanno_temperature_ratio.md)
- [Compressible Fanno Pressure Ratio](../equations/compressible/fanno_pressure_ratio.md)
- [Compressible Fanno Density Ratio](../equations/compressible/fanno_density_ratio.md)
- [Compressible Fanno Stagnation Pressure Ratio](../equations/compressible/fanno_stagnation_pressure_ratio.md)
- [Compressible Fanno Velocity Ratio](../equations/compressible/fanno_velocity_ratio.md)
- [Compressible Fanno Friction Parameter](../equations/compressible/fanno_friction_parameter.md)

## Bindings

### Python
```python
engpy.devices.fanno_flow_calc("mach", 2.0, "p_pstar", 1.4)
engpy.devices.fanno_flow_from_m_to_p0_p0star(2.0, 1.4)
engpy.devices.fanno_flow_from_4flstar_d_to_m(0.3049965026, 1.4, "supersonic")
engpy.devices.fanno_flow_path_text("p0_p0star", 1.33984375, "mach", 1.4, "subsonic")
```

### Excel
```excel
=ENG_FANNO_FLOW("mach",2.0,"p_pstar",1.4,"")
=ENG_FANNO_FLOW_FROM_M_TO_T_TSTAR(2.0,1.4)
=ENG_FANNO_FLOW_FROM_M_TO_P_PSTAR(2.0,1.4)
=ENG_FANNO_FLOW_FROM_M_TO_RHO_RHOSTAR(2.0,1.4)
=ENG_FANNO_FLOW_FROM_M_TO_P0_P0STAR(2.0,1.4)
=ENG_FANNO_FLOW_FROM_M_TO_V_VSTAR(2.0,1.4)
=ENG_FANNO_FLOW_FROM_M_TO_4FLSTAR_D(2.0,1.4)
=ENG_FANNO_FLOW_FROM_4FLSTAR_D_TO_M(0.3049965026,1.4,"supersonic")
=ENG_FANNO_FLOW_FROM_P0_P0STAR_TO_M(1.33984375,1.4,"subsonic")
=ENG_FANNO_FLOW_PATH_TEXT("four_flstar_d",0.3049965026,"mach",1.4,"supersonic")
```

**Excel arguments**
- `value_kind_in`: `mach`, `t_tstar`, `p_pstar`, `rho_rhostar`, `p0_p0star`, `four_flstar_d`
- `value_in`: input value
- `target_kind_out`: `mach`, `t_tstar`, `p_pstar`, `rho_rhostar`, `p0_p0star`, `v_vstar`, `four_flstar_d`
- `gamma`: specific heat ratio
- `branch`: required for inverse paths (`subsonic`/`supersonic`)
