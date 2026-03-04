# Devices Guide

Devices/components compose multiple atomic equations into a higher-level engineering solve workflow. Users provide engineering inputs once; the device orchestrates intermediate solves internally.

## First Production Device: Pipe Pressure Drop

- API entrypoint: `eng::devices::pipe_loss()`
- Supported friction models: `Fixed(f)`, `Colebrook`
- Internal composed equations: Reynolds + Colebrook (when selected) + Darcy-Weisbach
- Outputs: `delta_p`, plus intermediate `friction_factor` and `reynolds_number` when available

## Fixed-f Mode

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

## Colebrook Mode (direct properties)

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

## Colebrook Mode (fluid context)

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

## Calculator Device: Isentropic

- API entrypoint: `eng::devices::isentropic_calc()`
- Uses deterministic Mach-pivot orchestration (`input_kind -> pivot Mach -> target_kind`).
- All mathematical relations are resolved through registry-backed atomic equations.
- Branch-sensitive inverse paths (such as `area_ratio -> mach`) require explicit branch selection.
- Outputs include scalar value, pivot Mach, and step diagnostics/path text.

See [Devices Index](./index.md) and [Pipe Pressure Drop](./pipe_loss.md).
See also [Isentropic Calculator](./isentropic_calc.md).
