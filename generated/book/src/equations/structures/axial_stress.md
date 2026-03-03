# Axial Normal Stress

**Path:** `structures.axial_stress`  
**Category:** `structures`

## Equation

$$
\sigma = \frac{F}{A}
$$

- Unicode: `σ = F / A`
- ASCII: `sigma = F / A`

## Assumptions

- Uniform axial loading and stress distribution.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>sigma</code></td><td>Axial stress</td><td><span class="math inline">\(\sigma\)</span></td><td><code>stress</code></td><td><code>Pa</code></td><td><code>-</code></td></tr>
    <tr><td><code>F</code></td><td>Axial force</td><td><span class="math inline">\(F\)</span></td><td><code>force</code></td><td><code>N</code></td><td><code>-</code></td></tr>
    <tr><td><code>A</code></td><td>Cross-sectional area</td><td><span class="math inline">\(A\)</span></td><td><code>area</code></td><td><code>m2</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `A`: explicit
- `F`: explicit
- `sigma`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(structures::axial_stress::equation())
    .target_sigma()
    .given_f(10000.0)
    .given_a(1.9999999999999998e-4)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(structures::axial_stress::equation())
    .target_sigma()
    .given_f("10000 N")
    .given_a("200 mm2")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>sigma</code></td><td><code>solve_sigma(F, A)</code></td><td><code>F</code>, <code>A</code></td></tr>
    <tr><td><code>F</code></td><td><code>solve_f(sigma, A)</code></td><td><code>sigma</code>, <code>A</code></td></tr>
    <tr><td><code>A</code></td><td><code>solve_a(sigma, F)</code></td><td><code>sigma</code>, <code>F</code></td></tr>
  </tbody>
</table>

### Solve `sigma`

**Function signature**

```rust
equations::structures::axial_stress::solve_sigma(F, A) -> Result<f64, _>
```

**Example**

```rust
let value = equations::structures::axial_stress::solve_sigma(
    "10000 N",
    "200 mm2",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

