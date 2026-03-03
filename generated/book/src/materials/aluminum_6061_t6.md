# Aluminum 6061-T6

<table>
  <thead><tr><th>Field</th><th>Value</th></tr></thead>
  <tbody>
    <tr><td>Key</td><td><code>aluminum_6061_t6</code></td></tr>
    <tr><td>Aliases</td><td><code>al6061_t6</code>, <code>6061_t6</code></td></tr>
    <tr><td>Source</td><td>Representative handbook values for interpolation demonstrations.</td></tr>
  </tbody>
</table>

Heat-treated aluminum alloy with dense temperature-property series.

## Properties

<table>
  <thead><tr><th>Property</th><th>Dimension</th><th>Unit</th><th>Points</th><th>Interpolation</th></tr></thead>
  <tbody>
    <tr><td><code>elastic_modulus</code></td><td><code>pressure</code></td><td><code>Pa</code></td><td>7</td><td><code>linear</code></td></tr>
    <tr><td><code>thermal_conductivity</code></td><td><code>thermal_conductivity</code></td><td><code>W/(m*K)</code></td><td>7</td><td><code>linear</code></td></tr>
    <tr><td><code>yield_strength</code></td><td><code>pressure</code></td><td><code>Pa</code></td><td>7</td><td><code>linear</code></td></tr>
  </tbody>
</table>

## Example

```rust
use eng_materials as materials;

let wall = materials::aluminum_6061_t6().temperature("350 K")?;
let value = wall.property("elastic_modulus")?;
println!("property = {value}");
```

## Example Equations Using Material Context

- [Euler Buckling Critical Load](../equations/structures/euler_buckling_load.md)
- [Plane-Wall Conduction Heat Rate](../equations/heat_transfer/conduction_plane_wall_heat_rate.md)
