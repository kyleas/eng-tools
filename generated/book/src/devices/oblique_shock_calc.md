# Oblique Shock Calculator

**Key:** `oblique_shock_calc`

Calculator-style compressible device: solve oblique-shock input pairs (M1+beta / M1+theta) to target outputs with explicit weak/strong branch handling.

## Overview

A calculator-style compressible device for oblique shocks. It supports the two practical input pairs:
- `(M1, beta)`
- `(M1, theta)`

The solver resolves the shock geometry and normal component (`Mn1`) then reuses normal-shock equations for downstream ratios and `Mn2`.

### Supported outputs
- `theta_deg`
- `beta_deg`
- `mn1`
- `mn2`
- `m2`
- `p2_p1`
- `rho2_rho1`
- `t2_t1`
- `p02_p01`

### Branch behavior
- `M1 + theta -> beta` is double-valued and requires explicit `weak` or `strong` branch selection.

### Rust
```rust
use eng::devices::{oblique_shock_calc, ObliqueShockInputKind, ObliqueShockOutputKind, ObliqueShockBranch};
let out = oblique_shock_calc()
    .gamma(1.4)
    .m1(2.0)
    .input(ObliqueShockInputKind::ThetaRad, 10f64.to_radians())
    .target(ObliqueShockOutputKind::PressureRatio)
    .branch(ObliqueShockBranch::Weak)
    .solve()?;
println!("beta={} deg, p2/p1={}", out.beta_rad.to_degrees(), out.value_si);
```

## Modes

- Input pairs: (M1, beta), (M1, theta)
- Branch-aware inversion for (M1, theta) -> beta

## Outputs

- value_si
- beta_rad
- theta_rad
- mn1
- mn2
- m2
- path diagnostics
## Internal Composition

- [Compressible Oblique Shock Theta Beta M](../equations/compressible/oblique_shock_theta_beta_m.md)
- [Compressible Oblique Shock Mn1](../equations/compressible/oblique_shock_mn1.md)
- [Compressible Oblique Shock M2](../equations/compressible/oblique_shock_m2.md)
- [Compressible Normal Shock M2](../equations/compressible/normal_shock_m2.md)
- [Compressible Normal Shock Pressure Ratio](../equations/compressible/normal_shock_pressure_ratio.md)
- [Compressible Normal Shock Density Ratio](../equations/compressible/normal_shock_density_ratio.md)
- [Compressible Normal Shock Temperature Ratio](../equations/compressible/normal_shock_temperature_ratio.md)
- [Compressible Normal Shock Stagnation Pressure Ratio](../equations/compressible/normal_shock_stagnation_pressure_ratio.md)

## Bindings

### Python
```python
engpy.devices.oblique_shock_calc(2.0, "theta_deg", 10.0, "beta_deg", 1.4, "weak")
engpy.devices.oblique_shock_from_m1_beta_to_theta(2.0, 40.0, 1.4)
engpy.devices.oblique_shock_from_m1_theta_to_beta(2.0, 10.0, 1.4, "strong")
engpy.devices.oblique_shock_from_m1_theta_to_p2_p1(2.0, 10.0, 1.4, "weak")
engpy.devices.oblique_shock_path_text(2.0, "theta_deg", 10.0, "m2", 1.4, "weak")
```

### Excel
```excel
=ENG_OBLIQUE_SHOCK(2.0,"theta_deg",10.0,"beta_deg",1.4,"weak")
=ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_THETA(2.0,40.0,1.4)
=ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_BETA(2.0,10.0,1.4,"strong")
=ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_P2_P1(2.0,10.0,1.4,"weak")
=ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_M2(2.0,40.0,1.4)
=ENG_OBLIQUE_SHOCK_PATH_TEXT(2.0,"theta_deg",10.0,"m2",1.4,"weak")
```

**Excel arguments**
- `m1`: upstream Mach number
- `value_kind_in`: `beta_deg` or `theta_deg`
- `value_in`: angle input in degrees
- `target_kind_out`: `theta_deg`, `beta_deg`, `mn1`, `mn2`, `m2`, `p2_p1`, `rho2_rho1`, `t2_t1`, `p02_p01`
- `gamma`: specific heat ratio
- `branch`: `weak`/`strong` (required for `theta_deg` input paths)
