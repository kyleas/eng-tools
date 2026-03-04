# Thin-Wall Longitudinal Stress

**Path:** `structures.longitudinal_stress_thin_wall`  
**Category:** `structures`

## Equation

$$
\sigma_l = \frac{P r}{2 t}
$$

- Unicode: `σ_l = P · r / (2 · t)`
- ASCII: `sigma_l = P * r / (2 * t)`

## Assumptions

- Thin-wall cylindrical vessel behavior.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>sigma_l</code></td><td>Longitudinal stress</td><td><span class="math inline">\(\sigma_l\)</span></td><td><code>stress</code></td><td><code>Pa</code></td><td><code>-</code></td></tr>
    <tr><td><code>P</code></td><td>Internal pressure</td><td><span class="math inline">\(P\)</span></td><td><code>pressure</code></td><td><code>Pa</code></td><td><code>-</code></td></tr>
    <tr><td><code>r</code></td><td>Mean radius</td><td><span class="math inline">\(r\)</span></td><td><code>length</code></td><td><code>m</code></td><td><code>-</code></td></tr>
    <tr><td><code>t</code></td><td>Wall thickness</td><td><span class="math inline">\(t\)</span></td><td><code>length</code></td><td><code>m</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `P`: explicit
- `r`: explicit
- `sigma_l`: explicit
- `t`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(structures::longitudinal_stress_thin_wall::equation())
    .target_sigma_l()
    .given_p(2.5e6)
    .given_r(0.2)
    .given_t(0.008)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(structures::longitudinal_stress_thin_wall::equation())
    .target_sigma_l()
    .given_p("2.5 MPa")
    .given_r("0.2 m")
    .given_t("8 mm")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>sigma_l</code></td><td><code>solve_sigma_l(P, r, t)</code></td><td><code>P</code>, <code>r</code>, <code>t</code></td></tr>
    <tr><td><code>P</code></td><td><code>solve_p(sigma_l, r, t)</code></td><td><code>sigma_l</code>, <code>r</code>, <code>t</code></td></tr>
    <tr><td><code>r</code></td><td><code>solve_r(sigma_l, P, t)</code></td><td><code>sigma_l</code>, <code>P</code>, <code>t</code></td></tr>
    <tr><td><code>t</code></td><td><code>solve_t(sigma_l, P, r)</code></td><td><code>sigma_l</code>, <code>P</code>, <code>r</code></td></tr>
  </tbody>
</table>

### Solve `sigma_l`

**Function signature**

```rust
equations::structures::longitudinal_stress_thin_wall::solve_sigma_l(P, r, t) -> Result<f64, _>
```

**Example**

```rust
let value = equations::structures::longitudinal_stress_thin_wall::solve_sigma_l(
    "2.5 MPa",
    "0.2 m",
    "8 mm",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

## Source

- Roark's Formulas for Stress and Strain

