# Nitrous Oxide

| Field | Value |
| --- | --- |
| Key | `N2O` |
| Aliases | nitrous oxide |
| Supported state inputs | `T,P, P,h, P,s, rho,h, P,Q, T,Q` |
| Supported properties | 12 |

## Supported State Input Pairs

| Pair | Notes |
| --- | --- |
| `T,P` | General purpose explicit state constructor (`state_tp`) |
| `P,h` | Pressure/enthalpy inversion (`state_ph`) |
| `P,s` | Pressure/entropy inversion (`state_ps`) |
| `rho,h` | Density/enthalpy construction (`state_rho_h`) |
| `P,Q` | Two-phase saturation by pressure (`state_pq`) |
| `T,Q` | Two-phase saturation by temperature (`state_tq`) |

## Verified Constructor and Generic Examples

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
}```

## Generic Property Aliases

| Canonical | Aliases |
| --- | --- |
| Temperature | `T`, `temperature` |
| Pressure | `P`, `pressure` |
| Density | `rho`, `density` |
| Specific enthalpy | `h`, `enthalpy` |
| Specific entropy | `s`, `entropy` |
| Quality | `Q`, `quality`, `x` |

## Supported Property Keys

| Property key | Direct accessor |
| --- | --- |
| `density` | `density()`, `rho()` |
| `specific_heat_capacity` | `specific_heat_capacity()`, `cp()` |
| `specific_heat_capacity_cv` | `specific_heat_capacity_cv()`, `cv()` |
| `gamma` | `gamma()` |
| `speed_of_sound` | `speed_of_sound()`, `a()` |
| `dynamic_viscosity` | `dynamic_viscosity()`, `mu()` |
| `thermal_conductivity` | `thermal_conductivity()`, `k()` |
| `temperature` | `temperature()`, `t()` |
| `pressure` | `pressure()`, `p()` |
| `specific_enthalpy` | `specific_enthalpy()`, `h()` |
| `specific_entropy` | `specific_entropy()`, `s()` |
| `quality` | `quality()` |

## Direct Property Access Example

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

## Saturation and Metadata Example

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

## Using This Fluid With Equations

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

### Equations currently using `fluid` context

- [Reynolds Number](../equations/fluids/reynolds_number.md)

## Error Behavior

- Unsupported input pairs return explicit pair diagnostics with a supported-pairs list.
- Unknown/invalid generic property keys return actionable key guidance.
- `u`/internal-energy keys are rejected intentionally to prevent ambiguity with `h`.
- Backend failures are surfaced as recoverable structured errors with fluid/pair/property context.

## Bindings

### Python
```python
engpy.fluids.fluid_prop("H2O", "T", "300 K", "P", "1 bar", "rho")
engpy.helpers.fluid_properties("H2O")
```

### Excel
```excel
=ENG_FLUID_PROP("H2O","T","300 K","P","1 bar","rho")
=ENG_FLUID_PROPERTIES("H2O")
```

**Excel arguments**
- `fluid`: fluid key or alias
- `state_prop_1`, `state_value_1`: first state-defining property and value
- `state_prop_2`, `state_value_2`: second state-defining property and value
- `out_prop`: property to return (for example `rho`, `mu`, `cp`)
