# Circular Shaft Torsion Stress

**Path ID:** `structures.shaft_torsion_stress`

\[
\tau = \frac{T r}{J}
\]

- Unicode: `\tau = T · r / J`
- ASCII: `tau = T * r / J`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>tau</code></td><td>Shear stress</td><td>\(\tau\)</td><td><code>stress</code></td><td><code>Pa</code></td></tr>
<tr><td><code>T</code></td><td>Torque</td><td>\(T\)</td><td><code>moment</code></td><td><code>N*m</code></td></tr>
<tr><td><code>r</code></td><td>Radius</td><td>\(r\)</td><td><code>length</code></td><td><code>m</code></td></tr>
<tr><td><code>J</code></td><td>Polar moment of inertia</td><td>\(J\)</td><td><code>polar_moment_of_inertia</code></td><td><code>m4</code></td></tr>
</tbody></table>

## Assumptions

- Circular shaft in Saint-Venant torsion.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(structures::shaft_torsion_stress::equation())
    .target_tau()
    .given_t(1200.0)
    .given_r(0.04)
    .given_j(1.6e-5)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(structures::shaft_torsion_stress::equation())
    .target_tau()
    .given_t("1200 N*m")
    .given_r("0.04 m")
    .given_j("1.6e-5 m4")
    .value()?;
```

### convenience_tau

```rust
let value = equations::structures::shaft_torsion_stress::solve_tau(
    "1200 N*m",
    "0.04 m",
    "1.6e-5 m4",
)?;
```

### convenience_t

```rust
let value = equations::structures::shaft_torsion_stress::solve_t(
    "3 MPa",
    "0.04 m",
    "1.6e-5 m4",
)?;
```

### convenience_r

```rust
let value = equations::structures::shaft_torsion_stress::solve_r(
    "3 MPa",
    "1200 N*m",
    "1.6e-5 m4",
)?;
```

### convenience_j

```rust
let value = equations::structures::shaft_torsion_stress::solve_j(
    "3 MPa",
    "1200 N*m",
    "0.04 m",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::structures::shaft_torsion_stress::equation()).for_target("J").value()?;
```

### Python
```python
engpy.equations.structures.solve_j(tau="...", t="...", r="...")
```

### Excel
```excel
=ENG_STRUCTURES_SHAFT_TORSION_STRESS_J("...","...","...")
```

**Excel arguments**
- `tau`: Shear stress
- `t`: Torque
- `r`: Radius

