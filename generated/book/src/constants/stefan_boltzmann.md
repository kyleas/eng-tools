# Stefan-Boltzmann Constant

<table>
  <thead>
    <tr><th>Field</th><th>Value</th></tr>
  </thead>
  <tbody>
    <tr><td>Key</td><td><code>stefan_boltzmann</code></td></tr>
    <tr><td>Symbol</td><td><span class="math inline">\(\sigma\)</span></td></tr>
    <tr><td>Dimension</td><td><code>stefan_boltzmann_constant</code></td></tr>
    <tr><td>Value</td><td>5.670374419000e-8 <code>W/(m2*K4)</code></td></tr>
    <tr><td>Trust</td><td>Conventional / reference</td></tr>
    <tr><td>Source</td><td>CODATA 2018</td></tr>
    <tr><td>Note</td><td>Derived from exact constants; finite reported decimal precision.</td></tr>
    <tr><td>Aliases</td><td><code>sigma_sb</code></td></tr>
  </tbody>
</table>

Stefan-Boltzmann radiation constant.

## Rust Usage

```rust
use eng::{constants};
use equations::get_constant;

let c = constants::stefan_boltzmann();
assert_eq!(c.key, "stefan_boltzmann");
println!("stefan_boltzmann = σ {}", c.value, c.unit);

let by_id = get_constant("stefan_boltzmann").expect("constant lookup");
assert_eq!(by_id.key, "stefan_boltzmann");
let by_alias = get_constant("sigma_sb").expect("alias lookup");
assert_eq!(by_alias.key, "stefan_boltzmann");
```
