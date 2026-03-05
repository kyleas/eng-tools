# Conical Shock Calculator

**Key:** `conical_shock_calc`

Calculator-style compressible device for Taylor-Maccoll conical-shock workflows with explicit weak/strong branch handling.

## Overview

A production conical-shock calculator that combines compressible jump relations with Taylor-Maccoll integration for axisymmetric cone flow.

### Supported input forms
- `(M1, cone_angle_deg)`
- `(M1, wave_angle_deg)`

### Supported outputs
- `wave_angle_deg`
- `cone_angle_deg`
- `shock_turn_angle_deg`
- `mc` (cone-surface Mach)
- `p2_p1`
- `rho2_rho1`
- `t2_t1`
- `p02_p01`

### Branch behavior
- `M1 + cone_angle -> wave_angle` paths are branch-sensitive. Use explicit `weak` or `strong` branch selection.

### Domain behavior
- Returns structured errors for invalid Mach/angles, detached/no-solution regimes, and numerical convergence failures.

### Rust
```rust
use eng::devices::{conical_shock_calc, ConicalShockInputKind, ConicalShockOutputKind, ConicalShockBranch};
let out = conical_shock_calc()
    .gamma(1.4)
    .m1(2.2)
    .input(ConicalShockInputKind::ConeAngleRad, 12f64.to_radians())
    .target(ConicalShockOutputKind::WaveAngleRad)
    .branch(ConicalShockBranch::Weak)
    .solve()?;
println!("wave={} deg, Mc={}", out.wave_angle_rad.to_degrees(), out.cone_surface_mach);
```

## Modes

- Input forms: (M1, cone_angle), (M1, wave_angle)
- Taylor-Maccoll integrated conical-flow state resolution

## Outputs

- value_si
- wave_angle_rad
- cone_angle_rad
- shock_turn_angle_rad
- cone_surface_mach
- path diagnostics
## Internal Composition

- [Compressible Oblique Shock Theta Beta M](../equations/compressible/oblique_shock_theta_beta_m.md)
- [Compressible Oblique Shock Mn1](../equations/compressible/oblique_shock_mn1.md)
- [Compressible Normal Shock M2](../equations/compressible/normal_shock_m2.md)
- [Compressible Normal Shock Pressure Ratio](../equations/compressible/normal_shock_pressure_ratio.md)
- [Compressible Normal Shock Density Ratio](../equations/compressible/normal_shock_density_ratio.md)
- [Compressible Normal Shock Temperature Ratio](../equations/compressible/normal_shock_temperature_ratio.md)
- [Compressible Normal Shock Stagnation Pressure Ratio](../equations/compressible/normal_shock_stagnation_pressure_ratio.md)

## Bindings

### Python
```python
engpy.devices.conical_shock_calc(2.2, "cone_angle_deg", 12.0, "wave_angle_deg", 1.4, "weak")
engpy.devices.conical_shock_from_m1_cone_deg_to_wave_deg(2.2, 12.0, 1.4, "weak")
engpy.devices.conical_shock_from_m1_cone_deg_to_p2_p1(2.2, 12.0, 1.4, "weak")
engpy.devices.conical_shock_from_m1_cone_deg_to_mc(2.2, 12.0, 1.4, "weak")
engpy.devices.conical_shock_from_m1_wave_deg_to_cone_deg(2.2, 36.0, 1.4)
engpy.devices.conical_shock_path_text(2.2, "cone_angle_deg", 12.0, "mc", 1.4, "weak")
```

### Excel
```excel
=ENG_CONICAL_SHOCK(2.2,"cone_angle_deg",12.0,"wave_angle_deg",1.4,"weak")
=ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_WAVE_DEG(2.2,12.0,1.4,"weak")
=ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_P2_P1(2.2,12.0,1.4,"weak")
=ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_MC(2.2,12.0,1.4,"weak")
=ENG_CONICAL_SHOCK_FROM_M1_WAVE_DEG_TO_CONE_DEG(2.2,36.0,1.4)
=ENG_CONICAL_SHOCK_PATH_TEXT(2.2,"cone_angle_deg",12.0,"mc",1.4,"weak")
```

**Excel arguments**
- `m1`: upstream Mach number
- `value_kind_in`: `cone_angle_deg` or `wave_angle_deg`
- `value_in`: input angle in degrees
- `target_kind_out`: `wave_angle_deg`, `cone_angle_deg`, `shock_turn_angle_deg`, `mc`, `p2_p1`, `rho2_rho1`, `t2_t1`, `p02_p01`
- `gamma`: specific heat ratio
- `branch`: `weak`/`strong` for cone-angle inversion paths
