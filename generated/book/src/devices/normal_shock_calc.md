# Normal Shock Calculator

Calculator-style compressible-flow device that accepts one normal-shock input kind and one target kind, resolves a pivot upstream Mach number `M1`, then computes the requested output.

All relation math is delegated to equation-registry atomic equations (no device-local formula authority).

## Supported Inputs (v1)

- `m1`
- `m2`
- `p2_p1`
- `rho2_rho1`
- `t2_t1`
- `p02_p01`

## Supported Targets (v1)

- `m1`
- `m2`
- `p2_p1`
- `rho2_rho1`
- `t2_t1`
- `p02_p01`

## Pivot Strategy

- Deterministic orchestration always resolves a pivot `M1` first.
- Then target value is computed from `M1` through the corresponding normal-shock equation.

## Domain Notes

- `M1 >= 1` for normal shocks.
- Ratios are checked for physically valid ranges (e.g., `p2_p1 >= 1`, `0 < p02_p01 <= 1`).

## Examples

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

### Python
```python
engpy.devices.normal_shock_calc("m1", 2.0, "p2_p1", 1.4)
engpy.devices.normal_shock_from_m1_to_m2(2.0, 1.4)
engpy.devices.normal_shock_pivot_m1("p02_p01", 0.7208738615, "m2", 1.4)
engpy.devices.normal_shock_path_text("p2_p1", 4.5, "m2", 1.4)
```

### Excel
```excel
=ENG_NORMAL_SHOCK("m1",2.0,"p2_p1",1.4)
=ENG_NORMAL_SHOCK_FROM_M1_TO_M2(2.0,1.4)
=ENG_NORMAL_SHOCK_FROM_M1_TO_P2_P1(2.0,1.4)
=ENG_NORMAL_SHOCK_PIVOT_M1("p2_p1",4.5,"m2",1.4)
=ENG_NORMAL_SHOCK_PATH_TEXT("p02_p01",0.7208738615,"m2",1.4)
```

## Internal Composition

- [Normal Shock Downstream Mach Number](../equations/compressible/normal_shock_m2.md)
- [Normal Shock Static Pressure Ratio](../equations/compressible/normal_shock_pressure_ratio.md)
- [Normal Shock Density Ratio](../equations/compressible/normal_shock_density_ratio.md)
- [Normal Shock Temperature Ratio](../equations/compressible/normal_shock_temperature_ratio.md)
- [Normal Shock Stagnation Pressure Ratio](../equations/compressible/normal_shock_stagnation_pressure_ratio.md)

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
