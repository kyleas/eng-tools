# Fluids Guide

Fluids are first-class engineering objects with typed wrappers, explicit state constructors, a flexible generic state path, direct property accessors, and context integration with equations.

## Recommended Usage Path

- **Preferred fast path**: explicit constructors like `state_tp`, `state_ph`, `state_ps`, `state_rho_h`, `state_pq`, `state_tq`.
- **Flexible path**: generic `state("T", value, "P", value)` where property identity is explicit.
- Use direct accessor methods (`rho()`, `mu()`, `cp()`, `gamma()`, ...) instead of generic string property reads when writing Rust code.

## Constructor Capability Matrix

| Constructor | Meaning | Typical Use |
| --- | --- | --- |
| `state_tp(T, P)` | Temperature + pressure | Most common fast path |
| `state_ph(P, h)` | Pressure + specific enthalpy | Thermodynamic inversion workflows |
| `state_ps(P, s)` | Pressure + specific entropy | Isentropic/entropy constrained workflows |
| `state_rho_h(rho, h)` | Density + specific enthalpy | Density-driven model integration |
| `state_pq(P, Q)` | Pressure + quality | Two-phase saturation states |
| `state_tq(T, Q)` | Temperature + quality | Two-phase saturation states |
| `state("T", v1, "P", v2)` | Explicit property-name pair | Flexible bindings/CLI/file paths |

## Generic Property Alias Map

| Canonical | Accepted aliases |
| --- | --- |
| Temperature | `T`, `temperature` |
| Pressure | `P`, `pressure` |
| Density | `rho`, `density` |
| Specific enthalpy | `h`, `enthalpy` |
| Specific entropy | `s`, `entropy` |
| Quality | `Q`, `quality`, `x` |

Property identity is required in the generic path; there is no unit-only inference. This avoids ambiguity (for example `h` vs `u`).

## Verified State Construction Examples

```rust
{
    let n2_pt = eng::fluids::nitrogen().state_tp(
        eng::units::typed::temperature::k(300.0),
        eng::units::typed::pressure::bar(1.0),
    )?;
    let n2_h = n2_pt.h()?;
    let n2_s = n2_pt.s()?;
    let n2_rho = n2_pt.rho()?;

    let _n2_ph =
        eng::fluids::nitrogen().state_ph(eng::units::typed::pressure::bar(1.0), n2_h)?;
    let _n2_ps =
        eng::fluids::nitrogen().state_ps(eng::units::typed::pressure::bar(1.0), n2_s)?;
    let _n2_rho_h = eng::fluids::nitrogen().state_rho_h(n2_rho, n2_h)?;

    let _air_generic = eng::fluids::air().state("T", "300 K", "P", "1 bar")?;
    let _air_generic_typed = eng::fluids::air().state(
        "P",
        eng::units::typed::pressure::bar(1.0),
        "T",
        eng::units::typed::temperature::k(300.0),
    )?;
}
```

## Direct Property Accessors

| Accessor | Meaning |
| --- | --- |
| `pressure()` / `p()` | Pressure (Pa) |
| `temperature()` / `t()` | Temperature (K) |
| `density()` / `rho()` | Density (kg/m^3) |
| `dynamic_viscosity()` / `mu()` | Dynamic viscosity (Pa*s) |
| `thermal_conductivity()` / `k()` | Thermal conductivity (W/(m*K)) |
| `specific_heat_capacity()` / `cp()` | Cp (J/(kg*K)) |
| `specific_heat_capacity_cv()` / `cv()` | Cv (J/(kg*K)) |
| `gamma()` | Heat capacity ratio |
| `speed_of_sound()` / `a()` | Speed of sound (m/s) |
| `specific_enthalpy()` / `h()` | Specific enthalpy (J/kg) |
| `specific_entropy()` / `s()` | Specific entropy (J/(kg*K)) |
| `quality()` | Quality in `[0,1]` for Q-based states |

```rust
use eng::fluids;
use eng::units::typed::{pressure, temperature};

let state = fluids::water().state_tp(temperature::k(300.0), pressure::bar(1.0))?;
let rho = state.rho()?;
let mu = state.mu()?;
let cp = state.cp()?;

let state_generic = fluids::air().state("T", "300 K", "P", "1 bar")?;
let gamma = state_generic.gamma()?;
```

## Quality, Saturation, and State Metadata

- `saturation_at_pressure(P)` returns `{ liquid, vapor }`
- `saturation_at_temperature(T)` returns `{ liquid, vapor }`
- State metadata includes `fluid_key()`, `fluid_name()`, `input_pair()`, `input_pair_label()`, `normalized_inputs()`, `quality()`, and `phase()`

```rust
{
    let sat = eng::fluids::water().saturation_at_pressure("1 bar")?;
    let q_liq = sat.liquid.quality();
    let q_vap = sat.vapor.quality();
    let pair = sat.liquid.input_pair_label();
    let fluid_key = sat.liquid.fluid_key();
    let _inputs = sat.liquid.normalized_inputs();

    assert_eq!(q_liq, Some(0.0));
    assert_eq!(q_vap, Some(1.0));
    assert_eq!(fluid_key, "H2O");
    assert_eq!(pair, "P,Q");
}
```

## Equation Context Integration

Use direct property lookup when you need one-off values. Use `solve_with_context(...).fluid(state)` when an equation should auto-resolve fluid-dependent variables.

Use fluid states directly in context solves:

```rust
use eng::{eq, equations, fluids};

let re = eq
    .solve_with_context(equations::fluids::reynolds_number::equation())
    .fluid(fluids::water().state_tp("300 K", "1 bar")?)
    .for_target("Re")
    .given("V", "3 m/s")
    .given("D", "0.1 m")
    .value()?;
```

## Common Errors and Gotchas

- Unknown/invalid generic property keys are rejected with explicit supported-key guidance.
- Unsupported input pairs (for example `rho,T`) return clear pair diagnostics listing supported pairs.
- `u`/internal-energy identifiers are intentionally rejected in the generic state-input path to avoid confusion with enthalpy `h`.
- Parse and backend failures are recoverable and include fluid/pair/property context.

## Catalog

- [Fluids Catalog](./index.md)
