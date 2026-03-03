# Beam Bending Stress

**Path:** `structures.beam_bending_stress`  
**Category:** `structures`

## Equation

$$
\sigma_b = \frac{M c}{I}
$$

- Unicode: `σ_b = M · c / I`
- ASCII: `sigma_b = M * c / I`

## Assumptions

- Linear elastic beam bending with small strains.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>sigma_b</code></td><td>Bending stress</td><td><span class="math inline">\(\sigma_b\)</span></td><td><code>stress</code></td><td><code>Pa</code></td><td><code>-</code></td></tr>
    <tr><td><code>M</code></td><td>Bending moment</td><td><span class="math inline">\(M\)</span></td><td><code>moment</code></td><td><code>N*m</code></td><td><code>-</code></td></tr>
    <tr><td><code>c</code></td><td>Distance to outer fiber</td><td><span class="math inline">\(c\)</span></td><td><code>length</code></td><td><code>m</code></td><td><code>-</code></td></tr>
    <tr><td><code>I</code></td><td>Area moment of inertia</td><td><span class="math inline">\(I\)</span></td><td><code>area_moment_of_inertia</code></td><td><code>m4</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `I`: explicit
- `M`: explicit
- `c`: explicit
- `sigma_b`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(structures::beam_bending_stress::equation())
    .target_sigma_b()
    .given_m(5000.0)
    .given_c(0.05)
    .given_i(5e-6)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(structures::beam_bending_stress::equation())
    .target_sigma_b()
    .given_m("5000 N*m")
    .given_c("0.05 m")
    .given_i("5e-6 m4")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>sigma_b</code></td><td><code>solve_sigma_b(M, c, I)</code></td><td><code>M</code>, <code>c</code>, <code>I</code></td></tr>
    <tr><td><code>M</code></td><td><code>solve_m(sigma_b, c, I)</code></td><td><code>sigma_b</code>, <code>c</code>, <code>I</code></td></tr>
    <tr><td><code>c</code></td><td><code>solve_c(sigma_b, M, I)</code></td><td><code>sigma_b</code>, <code>M</code>, <code>I</code></td></tr>
    <tr><td><code>I</code></td><td><code>solve_i(sigma_b, M, c)</code></td><td><code>sigma_b</code>, <code>M</code>, <code>c</code></td></tr>
  </tbody>
</table>

### Solve `sigma_b`

**Function signature**

```rust
equations::structures::beam_bending_stress::solve_sigma_b(M, c, I) -> Result<f64, _>
```

**Example**

```rust
let value = equations::structures::beam_bending_stress::solve_sigma_b(
    "5000 N*m",
    "0.05 m",
    "5e-6 m4",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

