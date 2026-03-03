# Universal Gas Constant

<table>
  <thead>
    <tr><th>Field</th><th>Value</th></tr>
  </thead>
  <tbody>
    <tr><td>Key</td><td><code>universal_gas_constant</code></td></tr>
    <tr><td>Symbol</td><td><span class="math inline">\(R_{u}\)</span></td></tr>
    <tr><td>Dimension</td><td><code>universal_gas_constant</code></td></tr>
    <tr><td>Value</td><td>8.314462618000e0 <code>J/(mol*K)</code></td></tr>
    <tr><td>Trust</td><td>Exact / defined</td></tr>
    <tr><td>Source</td><td>SI 2019 redefinition via fixed Boltzmann constant and Avogadro constant</td></tr>
    <tr><td>Note</td><td>Exact derived SI constant.</td></tr>
    <tr><td>Aliases</td><td><code>gas_constant_universal</code></td></tr>
  </tbody>
</table>

Universal gas constant used in thermodynamics.

## Rust Usage

```rust
use eng::{constants};
use equations::get_constant;

let c = constants::universal_gas_constant();
assert_eq!(c.key, "universal_gas_constant");
println!("universal_gas_constant = Rᵤ {}", c.value, c.unit);

let by_id = get_constant("universal_gas_constant").expect("constant lookup");
assert_eq!(by_id.key, "universal_gas_constant");
let by_alias = get_constant("gas_constant_universal").expect("alias lookup");
assert_eq!(by_alias.key, "universal_gas_constant");
```
