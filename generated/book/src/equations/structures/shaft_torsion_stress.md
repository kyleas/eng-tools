# Circular Shaft Torsion Stress

**Path:** `structures.shaft_torsion_stress`  
**Category:** `structures`

## Equation

$$
\tau = \frac{T r}{J}
$$

- Unicode: `\tau = T · r / J`
- ASCII: `tau = T * r / J`

## Assumptions

- Circular shaft in Saint-Venant torsion.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>tau</code></td><td>Shear stress</td><td><span class="math inline">\(\tau\)</span></td><td><code>stress</code></td><td><code>Pa</code></td><td><code>-</code></td></tr>
    <tr><td><code>T</code></td><td>Torque</td><td><span class="math inline">\(T\)</span></td><td><code>moment</code></td><td><code>N*m</code></td><td><code>-</code></td></tr>
    <tr><td><code>r</code></td><td>Radius</td><td><span class="math inline">\(r\)</span></td><td><code>length</code></td><td><code>m</code></td><td><code>-</code></td></tr>
    <tr><td><code>J</code></td><td>Polar moment of inertia</td><td><span class="math inline">\(J\)</span></td><td><code>polar_moment_of_inertia</code></td><td><code>m4</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `J`: explicit
- `T`: explicit
- `r`: explicit
- `tau`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(structures::shaft_torsion_stress::equation())
    .target_tau()
    .given_t(1200.0)
    .given_r(0.04)
    .given_j(1.6e-5)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(structures::shaft_torsion_stress::equation())
    .target_tau()
    .given_t("1200 N*m")
    .given_r("0.04 m")
    .given_j("1.6e-5 m4")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>tau</code></td><td><code>solve_tau(T, r, J)</code></td><td><code>T</code>, <code>r</code>, <code>J</code></td></tr>
    <tr><td><code>T</code></td><td><code>solve_t(tau, r, J)</code></td><td><code>tau</code>, <code>r</code>, <code>J</code></td></tr>
    <tr><td><code>r</code></td><td><code>solve_r(tau, T, J)</code></td><td><code>tau</code>, <code>T</code>, <code>J</code></td></tr>
    <tr><td><code>J</code></td><td><code>solve_j(tau, T, r)</code></td><td><code>tau</code>, <code>T</code>, <code>r</code></td></tr>
  </tbody>
</table>

### Solve `tau`

**Function signature**

```rust
equations::structures::shaft_torsion_stress::solve_tau(T, r, J) -> Result<f64, _>
```

**Example**

```rust
let value = equations::structures::shaft_torsion_stress::solve_tau(
    "1200 N*m",
    "0.04 m",
    "1.6e-5 m4",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

