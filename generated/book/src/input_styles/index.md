# Input Styles

All equation input methods support four styles:

1. plain numeric SI
2. typed unit constructors
3. `qty!("...")`
4. runtime strings

## Which Style Should I Use?

- **Fastest path**: plain SI numeric (`f64`) when values are already canonical.
- **Most explicit Rust path**: typed unit constructors (`pressure::mpa(2.5)`).
- **Preferred expression path in Rust runtime code**: `qty!("...")`.
- **Boundary convenience path**: runtime strings from CLI/UI/files.

## Why `qty!` Is Preferred in Rust Code

- Fixed literal expressions are validated from source literals, which catches malformed expressions earlier.
- The resulting quantity is already canonicalized and dimension-tagged, so you avoid repeatedly parsing freeform runtime strings.
- `qty!` uses the same dimensional rules as runtime strings, so behavior stays consistent.
- Runtime strings remain the right choice for user-entered or file/CLI-provided values.

## Performance Notes

- `f64` and typed constructors are lowest-overhead internal paths.
- `qty!` is preferred for static expressions in Rust code.
- Runtime strings are boundary convenience and include parse/validation cost.

## Plain SI

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

## Typed Units

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

## `qty!`

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

## Runtime Strings

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
