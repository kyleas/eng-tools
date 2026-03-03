# Darcy-Weisbach Pressure Drop

**Path:** `fluids.darcy_weisbach_pressure_drop`  
**Category:** `fluids`

## Equation

$$
\Delta p = f \frac{L}{D} \frac{\rho V^2}{2}
$$

- Unicode: `Δ p = f · (L / D) · (ρ · V² / 2)`
- ASCII: `delta_p = f * (L / D) * (rho * V^2 / 2)`

## Assumptions

- Fully developed internal flow in a constant-diameter conduit.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>delta_p</code></td><td>Pressure drop</td><td><span class="math inline">\(\Delta p\)</span></td><td><code>pressure</code></td><td><code>Pa</code></td><td><code>-</code></td></tr>
    <tr><td><code>f</code></td><td>Darcy friction factor</td><td><span class="math inline">\(f\)</span></td><td><code>friction_factor</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>L</code></td><td>Pipe length</td><td><span class="math inline">\(L\)</span></td><td><code>length</code></td><td><code>m</code></td><td><code>-</code></td></tr>
    <tr><td><code>D</code></td><td>Pipe diameter</td><td><span class="math inline">\(D\)</span></td><td><code>length</code></td><td><code>m</code></td><td><code>-</code></td></tr>
    <tr><td><code>rho</code></td><td>Fluid density</td><td><span class="math inline">\(\rho\)</span></td><td><code>density</code></td><td><code>kg/m3</code></td><td><code>-</code></td></tr>
    <tr><td><code>V</code></td><td>Mean velocity</td><td><span class="math inline">\(V\)</span></td><td><code>velocity</code></td><td><code>m/s</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `D`: explicit
- `L`: explicit
- `V`: explicit
- `delta_p`: explicit
- `f`: explicit
- `rho`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(fluids::darcy_weisbach_pressure_drop::equation())
    .target_delta_p()
    .given_f(0.02)
    .given_l(10.0)
    .given_d(0.1)
    .given_rho(1000.0)
    .given_v(3.0)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(fluids::darcy_weisbach_pressure_drop::equation())
    .target_delta_p()
    .given_f(0.02)
    .given_l("10 m")
    .given_d("0.1 m")
    .given_rho("1000 kg/m3")
    .given_v("3 m/s")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>delta_p</code></td><td><code>solve_delta_p(f, L, D, rho, V)</code></td><td><code>f</code>, <code>L</code>, <code>D</code>, <code>rho</code>, <code>V</code></td></tr>
    <tr><td><code>f</code></td><td><code>solve_f(delta_p, L, D, rho, V)</code></td><td><code>delta_p</code>, <code>L</code>, <code>D</code>, <code>rho</code>, <code>V</code></td></tr>
    <tr><td><code>L</code></td><td><code>solve_l(delta_p, f, D, rho, V)</code></td><td><code>delta_p</code>, <code>f</code>, <code>D</code>, <code>rho</code>, <code>V</code></td></tr>
    <tr><td><code>D</code></td><td><code>solve_d(delta_p, f, L, rho, V)</code></td><td><code>delta_p</code>, <code>f</code>, <code>L</code>, <code>rho</code>, <code>V</code></td></tr>
    <tr><td><code>rho</code></td><td><code>solve_rho(delta_p, f, L, D, V)</code></td><td><code>delta_p</code>, <code>f</code>, <code>L</code>, <code>D</code>, <code>V</code></td></tr>
    <tr><td><code>V</code></td><td><code>solve_v(delta_p, f, L, D, rho)</code></td><td><code>delta_p</code>, <code>f</code>, <code>L</code>, <code>D</code>, <code>rho</code></td></tr>
  </tbody>
</table>

### Solve `delta_p`

**Function signature**

```rust
equations::fluids::darcy_weisbach_pressure_drop::solve_delta_p(f, L, D, rho, V) -> Result<f64, _>
```

**Example**

```rust
let value = equations::fluids::darcy_weisbach_pressure_drop::solve_delta_p(
    0.02,
    "10 m",
    "0.1 m",
    "1000 kg/m3",
    "3 m/s",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

