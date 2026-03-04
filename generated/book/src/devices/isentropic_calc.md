# Isentropic Calculator

Calculator-style compressible-flow device that accepts one isentropic input kind and one target kind, resolves a pivot Mach number, then computes the requested output.

All relation math is delegated to equation-registry atomic equations (no device-local formula authority).

## Supported Inputs (v1)

- `mach`
- `mach_angle_deg` (Excel/Python convenience; internally converted to radians)
- `pressure_ratio` (`p/p0`)
- `temperature_ratio` (`T/T0`)
- `density_ratio` (`rho/rho0`)
- `area_ratio` (`A/A*`, branch-sensitive)

## Supported Targets (v1)

- `mach`
- `mach_angle_deg`
- `pressure_ratio`
- `temperature_ratio`
- `density_ratio`
- `area_ratio`

## Branch Behavior

- `area_ratio -> mach` is double-valued and requires explicit branch (`subsonic` or `supersonic`).
- If branch-sensitive inversion is requested without branch, the device returns a structured error.

## Examples

### Rust
```rust
use eng::devices::{isentropic_calc, IsentropicInputKind, IsentropicOutputKind, IsentropicBranch};
let out = isentropic_calc()
    .gamma(1.4)
    .input(IsentropicInputKind::AreaRatio, 2.0)
    .target(IsentropicOutputKind::Mach)
    .branch(IsentropicBranch::Supersonic)
    .solve()?;
println!("M={}, p/p0={}", out.pivot_mach, out.value_si);
```

### Python
```python
engpy.devices.isentropic_calc("mach_angle_deg", 30.0, "pressure_ratio", 1.4)
engpy.devices.isentropic_pivot_mach("area_ratio", 2.0, "mach", 1.4, "subsonic")
engpy.devices.isentropic_path_text("area_ratio", 2.0, "mach", 1.4, "supersonic")
```

### Excel
```excel
=ENG_ISENTROPIC("mach_angle_deg",30,"pressure_ratio",1.4,"")
=ENG_ISENTROPIC_FROM_A_ASTAR_TO_M(2.0,1.4,"supersonic")
=ENG_ISENTROPIC_PIVOT_MACH("area_ratio",2.0,"mach",1.4,"subsonic")
=ENG_ISENTROPIC_PATH_TEXT("mach",2.0,"pressure_ratio",1.4,"")
```

## Internal Composition

- [Mach Angle](../equations/compressible/mach_angle.md)
- [Isentropic Pressure Ratio](../equations/compressible/isentropic_pressure_ratio.md)
- [Isentropic Temperature Ratio](../equations/compressible/isentropic_temperature_ratio.md)
- [Isentropic Density Ratio](../equations/compressible/isentropic_density_ratio.md)
- [Isentropic Area-Mach Relation](../equations/compressible/area_mach.md)

## Bindings

### Python
```python
engpy.devices.isentropic_calc("mach_angle_deg", 30.0, "pressure_ratio", 1.4)
engpy.devices.isentropic_pivot_mach("area_ratio", 2.0, "mach", 1.4, "supersonic")
engpy.devices.isentropic_path_text("area_ratio", 2.0, "mach", 1.4, "subsonic")
```

### Excel
```excel
=ENG_ISENTROPIC("mach_angle_deg",30,"pressure_ratio",1.4,"")
=ENG_ISENTROPIC_FROM_M_TO_P_P0(2.0,1.4,"")
=ENG_ISENTROPIC_FROM_MU_DEG_TO_P_P0(30,1.4,"")
=ENG_ISENTROPIC_FROM_A_ASTAR_TO_M(2.0,1.4,"supersonic")
=ENG_ISENTROPIC_PIVOT_MACH("area_ratio",2.0,"mach",1.4,"subsonic")
=ENG_ISENTROPIC_PATH_TEXT("mach",2.0,"pressure_ratio",1.4,"")
```

**Excel arguments**
- `value_kind_in`: `mach`, `mach_angle_deg`, `pressure_ratio`, `temperature_ratio`, `density_ratio`, `area_ratio`
- `value_in`: input value
- `target_kind_out`: same enum family as input kind
- `gamma`: specific heat ratio
- `branch`: optional, required for double-valued inverse paths like `area_ratio -> mach`
