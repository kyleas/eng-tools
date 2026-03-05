# Rayleigh Flow Calculator

**Key:** `rayleigh_calc`

Calculator-style compressible device: solve Rayleigh star-reference input kinds to target kinds through Mach pivot orchestration.

## Overview

A calculator-style compressible device that resolves a pivot Mach number from one supported Rayleigh input kind and then evaluates the requested target kind.

### Supported input kinds
- `mach`
- `t_tstar` (`T/T*`)
- `p_pstar` (`p/p*`)
- `rho_rhostar` (`rho/rho*`)
- `t0_t0star` (`T0/T0*`)
- `p0_p0star` (`p0/p0*`)
- `v_vstar` (`V/V*`)

### Supported target kinds
- `mach`
- `t_tstar`
- `p_pstar`
- `rho_rhostar`
- `t0_t0star`
- `p0_p0star`
- `v_vstar`

### Branch behavior
- Branch-sensitive inverse paths (`T/T* -> M`, `T0/T0* -> M`, `p0/p0* -> M`) require `subsonic` or `supersonic`.

### Rust
```rust
use eng::devices::{rayleigh_calc, RayleighBranch, RayleighInputKind, RayleighOutputKind};
let out = rayleigh_calc()
    .gamma(1.4)
    .input(RayleighInputKind::StagnationTemperatureRatio, 0.7933884297520661)
    .target(RayleighOutputKind::Mach)
    .branch(RayleighBranch::Supersonic)
    .solve()?;
println!("M={}, value={}", out.pivot_mach, out.value_si);
```

## Modes

- Input kinds: Mach, T/T*, p/p*, rho/rho*, T0/T0*, p0/p0*, V/V*
- Branch-aware inversion for selected ratio -> Mach paths

## Outputs

- value_si
- pivot_mach
- path diagnostics
## Internal Composition

- [Compressible Rayleigh Temperature Ratio](../equations/compressible/rayleigh_temperature_ratio.md)
- [Compressible Rayleigh Pressure Ratio](../equations/compressible/rayleigh_pressure_ratio.md)
- [Compressible Rayleigh Density Ratio](../equations/compressible/rayleigh_density_ratio.md)
- [Compressible Rayleigh Stagnation Temperature Ratio](../equations/compressible/rayleigh_stagnation_temperature_ratio.md)
- [Compressible Rayleigh Stagnation Pressure Ratio](../equations/compressible/rayleigh_stagnation_pressure_ratio.md)
- [Compressible Rayleigh Velocity Ratio](../equations/compressible/rayleigh_velocity_ratio.md)

## Bindings

### Python
```python
engpy.devices.rayleigh_calc("mach", 2.0, "p_pstar", 1.4)
engpy.devices.rayleigh_from_m_to_t0_t0star(2.0, 1.4)
engpy.devices.rayleigh_from_t_tstar_to_m(0.7901234568, 1.4, "subsonic")
engpy.devices.rayleigh_path_text("t0_t0star", 0.7933884298, "mach", 1.4, "supersonic")
```

### Excel
```excel
=ENG_RAYLEIGH("mach",2.0,"p_pstar",1.4,"")
=ENG_RAYLEIGH_FROM_M_TO_T_TSTAR(2.0,1.4)
=ENG_RAYLEIGH_FROM_M_TO_P_PSTAR(2.0,1.4)
=ENG_RAYLEIGH_FROM_M_TO_RHO_RHOSTAR(2.0,1.4)
=ENG_RAYLEIGH_FROM_M_TO_T0_T0STAR(2.0,1.4)
=ENG_RAYLEIGH_FROM_M_TO_P0_P0STAR(2.0,1.4)
=ENG_RAYLEIGH_FROM_M_TO_V_VSTAR(2.0,1.4)
=ENG_RAYLEIGH_FROM_T_TSTAR_TO_M(0.7901234568,1.4,"subsonic")
=ENG_RAYLEIGH_FROM_T0_T0STAR_TO_M(0.7933884298,1.4,"supersonic")
=ENG_RAYLEIGH_FROM_P0_P0STAR_TO_M(1.1140525032,1.4,"subsonic")
=ENG_RAYLEIGH_PATH_TEXT("t_tstar",0.7901234568,"mach",1.4,"subsonic")
```

**Excel arguments**
- `input_kind`: `mach`, `t_tstar`, `p_pstar`, `rho_rhostar`, `t0_t0star`, `p0_p0star`, `v_vstar`
- `input_value`: input value
- `target_kind`: `mach`, `t_tstar`, `p_pstar`, `rho_rhostar`, `t0_t0star`, `p0_p0star`, `v_vstar`
- `gamma`: specific heat ratio
- `branch`: required for branch-sensitive inverse paths (`subsonic`/`supersonic`)
