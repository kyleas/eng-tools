# Devices Guide

Devices/components compose multiple atomic equations into higher-level engineering workflows. Device docs and bindings are generated from typed Rust metadata specs.

## Pipe Pressure Drop

- Key: `pipe_loss`
- Composes Reynolds + friction model + Darcy-Weisbach for pipe pressure loss.
- Route: `devices/pipe_loss.md`
- Fixed friction factor
- Colebrook

## Isentropic Calculator

- Key: `isentropic_calc`
- Calculator-style compressible device: solve any supported isentropic input to any supported output through Mach pivot orchestration.
- Route: `devices/isentropic_calc.md`
- Input kinds: Mach, MachAngle, Prandtl-Meyer angle, p/p0, T/T0, rho/rho0, A/A*
- Branch-aware inversion for A/A*

## Normal Shock Calculator

- Key: `normal_shock_calc`
- Calculator-style compressible device: solve normal-shock input kinds to target kinds through deterministic M1 pivot orchestration.
- Route: `devices/normal_shock_calc.md`
- Input kinds: M1, M2, p2/p1, rho2/rho1, T2/T1, p02/p01
- Target kinds: M1, M2, p2/p1, rho2/rho1, T2/T1, p02/p01

## Nozzle Flow Calculator

- Key: `nozzle_flow_calc`
- Calculator-style quasi-1D nozzle device: solve isentropic nozzle input kinds to target kinds through Mach pivot orchestration.
- Route: `devices/nozzle_flow_calc.md`
- Input kinds: Mach, A/A*, p/p0, T/T0, rho/rho0
- Branch-aware inversion for A/A* -> Mach
- Optional stagnation-reference scaling for static p/T/rho outputs

## Oblique Shock Calculator

- Key: `oblique_shock_calc`
- Calculator-style compressible device: solve oblique-shock input pairs (M1+beta / M1+theta) to target outputs with explicit weak/strong branch handling.
- Route: `devices/oblique_shock_calc.md`
- Input pairs: (M1, beta), (M1, theta)
- Branch-aware inversion for (M1, theta) -> beta

## Fanno Flow Calculator

- Key: `fanno_flow_calc`
- Calculator-style compressible device: solve Fanno star-reference input kinds to target kinds through Mach pivot orchestration.
- Route: `devices/fanno_flow_calc.md`
- Input kinds: Mach, T/T*, p/p*, rho/rho*, p0/p0*, 4fL*/D
- Branch-aware inversion for ratio -> Mach

## Rayleigh Flow Calculator

- Key: `rayleigh_calc`
- Calculator-style compressible device: solve Rayleigh star-reference input kinds to target kinds through Mach pivot orchestration.
- Route: `devices/rayleigh_calc.md`
- Input kinds: Mach, T/T*, p/p*, rho/rho*, T0/T0*, p0/p0*, V/V*
- Branch-aware inversion for selected ratio -> Mach paths

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

See [Devices Index](./index.md) for full generated device pages.
