# Normal Shock Calculator

**Key:** `normal_shock_calc`

Calculator-style compressible device: solve normal-shock input kinds to target kinds through deterministic M1 pivot orchestration.

## Overview

A calculator-style compressible device that resolves upstream Mach (`M1`) from one normal-shock input kind, then computes any supported target kind.

### Supported input kinds
- `m1`
- `m2`
- `p2_p1`
- `rho2_rho1`
- `t2_t1`
- `p02_p01`

### Supported target kinds
- `m1`
- `m2`
- `p2_p1`
- `rho2_rho1`
- `t2_t1`
- `p02_p01`

### Rust
```rust
use eng::devices::{normal_shock_calc, NormalShockInputKind, NormalShockOutputKind};
let out = normal_shock_calc()
    .gamma(1.4)
    .input(NormalShockInputKind::PressureRatio, 4.5)
    .target(NormalShockOutputKind::TemperatureRatio)
    .solve()?;
println!("M1={}, T2/T1={}", out.pivot_m1, out.value_si);
```

## Modes

- Input kinds: M1, M2, p2/p1, rho2/rho1, T2/T1, p02/p01
- Target kinds: M1, M2, p2/p1, rho2/rho1, T2/T1, p02/p01

## Outputs

- value_si
- pivot_m1
- path diagnostics
## Internal Composition

- [Compressible Normal Shock M2](../equations/compressible/normal_shock_m2.md)
- [Compressible Normal Shock Pressure Ratio](../equations/compressible/normal_shock_pressure_ratio.md)
- [Compressible Normal Shock Density Ratio](../equations/compressible/normal_shock_density_ratio.md)
- [Compressible Normal Shock Temperature Ratio](../equations/compressible/normal_shock_temperature_ratio.md)
- [Compressible Normal Shock Stagnation Pressure Ratio](../equations/compressible/normal_shock_stagnation_pressure_ratio.md)

## Bindings

### Python
```python
engpy.devices.normal_shock_calc("m1", 2.0, "p2_p1", 1.4)
engpy.devices.normal_shock_from_m1_to_m2(2.0, 1.4)
engpy.devices.normal_shock_pivot_m1("p2_p1", 4.5, "m2", 1.4)
engpy.devices.normal_shock_path_text("p02_p01", 0.7208738615, "m2", 1.4)
```

### Excel
```excel
=ENG_NORMAL_SHOCK("m1",2.0,"p2_p1",1.4)
=ENG_NORMAL_SHOCK_FROM_M1_TO_M2(2.0,1.4)
=ENG_NORMAL_SHOCK_FROM_M1_TO_P2_P1(2.0,1.4)
=ENG_NORMAL_SHOCK_FROM_M1_TO_RHO2_RHO1(2.0,1.4)
=ENG_NORMAL_SHOCK_FROM_M1_TO_T2_T1(2.0,1.4)
=ENG_NORMAL_SHOCK_FROM_M1_TO_P02_P01(2.0,1.4)
=ENG_NORMAL_SHOCK_PIVOT_M1("p2_p1",4.5,"m2",1.4)
=ENG_NORMAL_SHOCK_PATH_TEXT("p02_p01",0.7208738615,"m2",1.4)
```

**Excel arguments**
- `value_kind_in`: `m1`, `m2`, `p2_p1`, `rho2_rho1`, `t2_t1`, `p02_p01`
- `value_in`: input value
- `target_kind_out`: same enum family as input kind
- `gamma`: specific heat ratio
