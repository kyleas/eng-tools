# Fluids Guide

The fluids layer provides typed wrappers, explicit state constructors, a flexible generic state API, direct property accessors, quality support, saturation helpers, and structured state metadata/errors.

## Recommended Usage Paths

- **Preferred fast path:** explicit constructors such as `state_tp`, `state_ph`, `state_ps`, `state_rho_h`, `state_pq`, and `state_tq`.
- **Flexible path:** generic `state("T", ..., "P", ...)` with explicit property identity.
- Generic path does not infer property identity from units; you must provide property keys.

## State Constructor Patterns

| Constructor | Inputs | Typical use |
| --- | --- | --- |
| `state_tp(T, P)` | temperature, pressure | standard single-phase setup |
| `state_ph(P, h)` | pressure, specific enthalpy | inversion from energy state |
| `state_ps(P, s)` | pressure, specific entropy | entropy-based state setup |
| `state_rho_h(rho, h)` | density, specific enthalpy | density/energy-defined state |
| `state_pq(P, Q)` | pressure, quality | saturation/two-phase state at pressure |
| `state_tq(T, Q)` | temperature, quality | saturation/two-phase state at temperature |
| `state("T", v1, "P", v2)` | explicit property keys + values | flexible pair-input construction |

## Generic Property Key Aliases

| Canonical | Aliases |
| --- | --- |
| Temperature | `T`, `temperature` |
| Pressure | `P`, `pressure` |
| Density | `rho`, `density` |
| Specific enthalpy | `h`, `enthalpy` |
| Specific entropy | `s`, `entropy` |
| Quality | `Q`, `quality`, `x` |

## Direct Property Accessors

| Property | Accessors |
| --- | --- |
| Pressure | `pressure()`, `p()` |
| Temperature | `temperature()`, `t()` |
| Density | `density()`, `rho()` |
| Dynamic viscosity | `dynamic_viscosity()`, `mu()` |
| Thermal conductivity | `thermal_conductivity()`, `k()` |
| Specific heat (cp) | `specific_heat_capacity()`, `cp()` |
| Specific heat (cv) | `specific_heat_capacity_cv()`, `cv()` |
| Speed of sound | `speed_of_sound()`, `a()` |
| Specific enthalpy | `specific_enthalpy()`, `h()` |
| Specific entropy | `specific_entropy()`, `s()` |
| Heat capacity ratio | `gamma()` |
| Quality | `quality()` |

## Verified Constructor + Generic State Examples

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

## Verified Direct Property Access Example

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

## Verified Saturation + Metadata Example

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

## State Metadata

- `fluid_key()` and `fluid_name()` identify the wrapper/backend fluid.
- `input_pair()` and `input_pair_label()` show which pair initialized the state.
- `normalized_inputs()` reports normalized SI inputs used for backend calls.
- `quality()` and `phase()` expose two-phase context where available.

## Error Behavior

- Unsupported state pairs return an explicit supported-pairs list.
- Unknown generic property keys return actionable key errors.
- Duplicate keys in generic `state(...)` are rejected.
- `u`/internal-energy keys are intentionally rejected to avoid ambiguity with enthalpy.
- Backend failures remain recoverable and include fluid/pair/property context.

## Equation Integration

Use direct fluid property lookups for ad hoc calculations. Use equation context solving (`solve_with_context(...).fluid(...)`) when equations declare resolver metadata and you want automatic property resolution.

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

See the [Fluids Catalog](./index.md) for per-fluid reference pages.
