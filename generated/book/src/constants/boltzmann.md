# Boltzmann Constant

<table>
  <thead>
    <tr><th>Field</th><th>Value</th></tr>
  </thead>
  <tbody>
    <tr><td>Key</td><td><code>boltzmann</code></td></tr>
    <tr><td>Symbol</td><td><span class="math inline">\(k_{B}\)</span></td></tr>
    <tr><td>Dimension</td><td><code>boltzmann_constant</code></td></tr>
    <tr><td>Value</td><td>1.380649000000e-23 <code>J/K</code></td></tr>
    <tr><td>Trust</td><td>Exact / defined</td></tr>
    <tr><td>Source</td><td>SI 2019 definition</td></tr>
    <tr><td>Note</td><td>Exact defining constant.</td></tr>
    <tr><td>Aliases</td><td><code>k_b</code></td></tr>
  </tbody>
</table>

Boltzmann constant linking thermal energy and temperature.

## Rust Usage

```rust
use eng::{constants};
use equations::get_constant;

let c = constants::boltzmann();
assert_eq!(c.key, "boltzmann");
println!("boltzmann = k_B {}", c.value, c.unit);

let by_id = get_constant("boltzmann").expect("constant lookup");
assert_eq!(by_id.key, "boltzmann");
let by_alias = get_constant("k_b").expect("alias lookup");
assert_eq!(by_alias.key, "boltzmann");
```
