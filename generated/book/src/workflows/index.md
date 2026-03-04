# Examples & Workflows

These examples are sourced from verified snippets and corresponding tests.

## 1. Simple Equation Solve

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

## 2. Typed Unit Solve

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

## 3. `qty!` Solve

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

## 4. Runtime String Solve

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

## 5. Fluid-Assisted Solve

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

## 6. Family Variant Solve

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

## 7. Direct Fluid/Material Property Lookup

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

```rust
use eng::materials;

let state = materials::stainless_304().temperature("350 K")?;
let e = state.property("elastic_modulus")?;
let sy = state.property("yield_strength")?;
```

## 8. Device Workflow: Pipe Pressure Drop (Fixed f)

```rust
{
    let _dp = eng::devices::pipe_loss()
        .friction_model(eng::devices::PipeFrictionModel::Fixed(0.02))
        .given_rho("1000 kg/m3")
        .given_v("3 m/s")
        .given_d("0.1 m")
        .given_l("10 m")
        .solve_delta_p()?;
}
```

## 9. Device Workflow: Pipe Pressure Drop (Colebrook Direct Inputs)

```rust
{
    let result = eng::devices::pipe_loss()
        .friction_model(eng::devices::PipeFrictionModel::Colebrook)
        .given_rho("1000 kg/m^3")
        .given_mu("1 cP")
        .given_v("3 m/s")
        .given_d("0.1 m")
        .given_l("10 m")
        .given_eps("0.00015 in")
        .solve()?;

    let _dp = result.delta_p();
    let _re = result.reynolds_number().unwrap_or_default();
}
```

## 10. Device Workflow: Pipe Pressure Drop (Colebrook + Fluid Context)

```rust
{
    let _dp = eng::devices::pipe_loss()
        .friction_model(eng::devices::PipeFrictionModel::Colebrook)
        .fluid(eng::fluids::water().state_tp("300 K", "1 atm")?)
        .given_v("3 m/s")
        .given_d("0.1 m")
        .given_l("10 m")
        .given_eps("0.00015 in")
        .solve_delta_p()?;
}
```
