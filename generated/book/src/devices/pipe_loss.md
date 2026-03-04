# Pipe Pressure Drop

**Key:** `pipe_loss`

Composes Reynolds + friction model + Darcy-Weisbach for pipe pressure loss.

## Overview

Composes Reynolds, friction-factor model, and Darcy-Weisbach pressure drop for a practical pipe-loss solve surface.

### Modes
- `Fixed`: direct friction factor
- `Colebrook`: computes Reynolds + friction factor from roughness and viscosity

### Rust
```rust
use eng::devices::{pipe_loss, PipeFrictionModel};
let dp = pipe_loss()
    .friction_model(PipeFrictionModel::Colebrook)
    .given_rho("1000 kg/m^3")
    .given_mu("1 cP")
    .given_v("3 m/s")
    .given_d("0.1 m")
    .given_l("10 m")
    .given_eps("0.00015 in")
    .solve_delta_p()?;
println!("delta_p = {dp} Pa");
```

## Modes

- Fixed friction factor
- Colebrook

## Outputs

- delta_p (Pa)
- friction_factor
- reynolds_number
## Internal Composition

- [Fluids Reynolds Number](../equations/fluids/reynolds_number.md)
- [Fluids Colebrook](../equations/fluids/colebrook.md)
- [Fluids Darcy Weisbach Pressure Drop](../equations/fluids/darcy_weisbach_pressure_drop.md)

## Bindings

### Python
```python
dp = engpy.devices.pipe_loss_solve_delta_p(friction_model="Colebrook", v="3 m/s", d="0.1 m", l="10 m", eps="0.00015 in", fluid="H2O", in1_key="T", in1_value="300 K", in2_key="P", in2_value="1 atm")
engpy.helpers.format_value(dp, "Pa", "psia")
```

### Excel
```excel
=ENG_PIPE_LOSS_DELTA_P("Colebrook",,"",,"3 m/s","0.1 m","10 m","0.00015 in","H2O","T","300 K","P","1 atm")
=ENG_FORMAT(ENG_PIPE_LOSS_DELTA_P("Colebrook",,"",,"3 m/s","0.1 m","10 m","0.00015 in","H2O","T","300 K","P","1 atm"),"Pa","psia")
=ENG_META("device","pipe_loss","supported_modes")
```

**Excel arguments**
- `friction_model`: `Colebrook` or `Fixed`
- `fixed_f`: fixed Darcy friction factor when model is `Fixed`
- `density` / `viscosity` / `velocity` / `diameter` / `length` / `roughness`: direct engineering inputs
- `fluid`, `in1_key`, `in1_value`, `in2_key`, `in2_value`: optional fluid-state context pair
