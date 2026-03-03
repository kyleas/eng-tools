# Examples & Workflows

## Unified Top-Level API

```rust
use eng::{constants, eq, equations, fluids, materials, qty};
```

## Simple Equation Solve

```rust
use eng::{eq, equations};

let sigma_h_pa = eq
    .solve(equations::structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p(2.5e6)
    .given_r(0.2)
    .given_t(0.008)
    .value()?;
```

## Typed Unit Solve

```rust
use eng::{eq, equations};
use eng::core::units::typed::{length, pressure};

let sigma_h_pa = eq
    .solve(equations::structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p(pressure::mpa(2.5))
    .given_r(length::m(0.2))
    .given_t(length::mm(8.0))
    .value()?;
```

## `qty!` Solve

```rust
use eng::{eq, equations, qty};

let sigma_h_pa = eq
    .solve(equations::structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p(qty!("5 MPa + 12 psi"))
    .given_r(qty!("0.2 m"))
    .given_t(qty!("8 mm"))
    .value()?;
```

## Runtime String Solve

```rust
use eng::{eq, equations};

let sigma_h_pa = eq
    .solve(equations::structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p("5 MPa + 12 psi")
    .given_r("0.2 m")
    .given_t("8 mm")
    .value()?;
```

## Fluid-Assisted Solve

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

## Family Variant Solve

```rust
use eng::{eq, equations};

let p_pa = eq
    .solve(equations::thermo::ideal_gas::density::equation())
    .target_p()
    .given_rho("1.225 kg/m^3")
    .given_r("287 J/(kg*K)")
    .given_t("288.15 K")
    .value()?;
```

## Property Lookups

```rust
use eng::fluids::{self, FluidProperty};

let state = fluids::water().state_tp("300 K", "1 bar")?;
let rho = state.property(FluidProperty::Density)?;
let mu = state.property(FluidProperty::DynamicViscosity)?;
```

```rust
use eng::materials;

let state = materials::stainless_304().temperature("350 K")?;
let e = state.property("elastic_modulus")?;
let sy = state.property("yield_strength")?;
```
