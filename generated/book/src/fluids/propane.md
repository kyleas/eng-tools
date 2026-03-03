# Propane

<table>
  <thead><tr><th>Field</th><th>Value</th></tr></thead>
  <tbody>
    <tr><td>Key</td><td><code>Propane</code></td></tr>
    <tr><td>Aliases</td><td><code>c3h8</code>, <code>n-propane</code></td></tr>
    <tr><td>Supported state inputs</td><td><code>state_tp(T, P)</code></td></tr>
    <tr><td>Supported properties</td><td><code>density</code>, <code>specific_heat_capacity</code>, <code>specific_heat_capacity_cv</code>, <code>gamma</code>, <code>speed_of_sound</code>, <code>dynamic_viscosity</code>, <code>thermal_conductivity</code>, <code>temperature</code>, <code>pressure</code></td></tr>
  </tbody>
</table>

## Example

```rust
use eng_fluids as fluids;

let state = fluids::propane().state_tp("300 K", "1 bar")?;
let rho = state.property(fluids::FluidProperty::Density)?;
println!("rho = {rho} kg/m^3");
```

## Example Equations Using Fluid Context

- [Reynolds Number](../equations/fluids/reynolds_number.md)
