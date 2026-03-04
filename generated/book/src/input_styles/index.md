# Input Styles

Use these input styles in order of preference:

1. plain numeric SI (`f64`) for fastest path
2. typed unit constructors for explicit units in Rust
3. `qty!("...")` for compile-time parsed literal expressions
4. runtime strings for boundary input (CLI/UI/import)

## Plain Numeric (Canonical SI)

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

## Typed Unit Constructors

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

## `qty!` Expressions

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

## Runtime String Expressions

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

