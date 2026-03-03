# Archimedes Constant

<table>
  <thead>
    <tr><th>Field</th><th>Value</th></tr>
  </thead>
  <tbody>
    <tr><td>Key</td><td><code>pi</code></td></tr>
    <tr><td>Symbol</td><td><span class="math inline">\(\pi\)</span></td></tr>
    <tr><td>Dimension</td><td><code>dimensionless</code></td></tr>
    <tr><td>Value</td><td>3.141592653590e0 <code>1</code></td></tr>
    <tr><td>Trust</td><td>Conventional / reference</td></tr>
    <tr><td>Source</td><td>Mathematical constant</td></tr>
    <tr><td>Note</td><td>Irrational constant represented in f64 precision for evaluation.</td></tr>
  </tbody>
</table>

Ratio of a circle's circumference to its diameter.

## Rust Usage

```rust
use eng::{constants};
use equations::get_constant;

let c = constants::pi();
assert_eq!(c.key, "pi");
println!("pi = π {}", c.value, c.unit);

let by_id = get_constant("pi").expect("constant lookup");
assert_eq!(by_id.key, "pi");
```

## Equations Using This Constant

- [Circular Pipe Flow Area](../equations/fluids/circular_pipe_area.md)
- [Euler Buckling Critical Load](../equations/structures/euler_buckling_load.md)
