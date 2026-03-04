# Isentropic Calculator

**Key:** `isentropic_calc`

Calculator-style compressible device: solve any supported isentropic input to any supported output through Mach pivot orchestration.

## Overview

A calculator-style compressible device that resolves a pivot Mach number from one supported isentropic input kind and then evaluates the requested target kind.

### Supported input kinds
- `mach`
- `mach_angle_deg` (binding convenience; internally radians)
- `prandtl_meyer_angle_deg` (binding convenience; internally radians)
- `pressure_ratio` (`p/p0`)
- `temperature_ratio` (`T/T0`)
- `density_ratio` (`rho/rho0`)
- `area_ratio` (`A/A*`, branch-sensitive)

### Supported target kinds
- `mach`
- `mach_angle_deg`
- `prandtl_meyer_angle_deg`
- `pressure_ratio`
- `temperature_ratio`
- `density_ratio`
- `area_ratio`

### Branch behavior
- `area_ratio -> mach` is double-valued and requires `subsonic` or `supersonic`.

### Rust
```rust
use eng::devices::{isentropic_calc, IsentropicInputKind, IsentropicOutputKind, IsentropicBranch};
let out = isentropic_calc()
    .gamma(1.4)
    .input(IsentropicInputKind::AreaRatio, 2.0)
    .target(IsentropicOutputKind::Mach)
    .branch(IsentropicBranch::Supersonic)
    .solve()?;
println!("M={}, value={}", out.pivot_mach, out.value_si);
```

## Modes

- Input kinds: Mach, MachAngle, Prandtl-Meyer angle, p/p0, T/T0, rho/rho0, A/A*
- Branch-aware inversion for A/A*

## Outputs

- value_si
- pivot_mach
- path diagnostics
## Internal Composition

- [Compressible Area Mach](../equations/compressible/area_mach.md)
- [Compressible Isentropic Pressure Ratio](../equations/compressible/isentropic_pressure_ratio.md)
- [Compressible Isentropic Temperature Ratio](../equations/compressible/isentropic_temperature_ratio.md)
- [Compressible Isentropic Density Ratio](../equations/compressible/isentropic_density_ratio.md)
- [Compressible Mach Angle](../equations/compressible/mach_angle.md)
- [Compressible Prandtl Meyer](../equations/compressible/prandtl_meyer.md)

## Bindings

### Python
```python
engpy.devices.isentropic_calc("mach_angle_deg", 30.0, "pressure_ratio", 1.4)
engpy.devices.isentropic_from_nu_deg_to_m(26.3797608134, 1.4)
engpy.devices.isentropic_pivot_mach("area_ratio", 2.0, "mach", 1.4, "supersonic")
engpy.devices.isentropic_path_text("area_ratio", 2.0, "mach", 1.4, "subsonic")
```

### Excel
```excel
=ENG_ISENTROPIC("mach_angle_deg",30,"pressure_ratio",1.4,"")
=ENG_ISENTROPIC_FROM_M_TO_P_P0(2.0,1.4,"")
=ENG_ISENTROPIC_FROM_MU_DEG_TO_P_P0(30,1.4,"")
=ENG_ISENTROPIC_FROM_NU_DEG_TO_M(26.3797608134,1.4,"")
=ENG_ISENTROPIC_FROM_M_TO_NU_DEG(2.0,1.4,"")
=ENG_ISENTROPIC_FROM_A_ASTAR_TO_M(2.0,1.4,"supersonic")
=ENG_ISENTROPIC_PIVOT_MACH("area_ratio",2.0,"mach",1.4,"subsonic")
=ENG_ISENTROPIC_PATH_TEXT("mach",2.0,"pressure_ratio",1.4,"")
```

**Excel arguments**
- `value_kind_in`: `mach`, `mach_angle_deg`, `prandtl_meyer_angle_deg`, `pressure_ratio`, `temperature_ratio`, `density_ratio`, `area_ratio`
- `value_in`: input value
- `target_kind_out`: same enum family as input kind
- `gamma`: specific heat ratio
- `branch`: optional, required for double-valued inverse paths like `area_ratio -> mach`
