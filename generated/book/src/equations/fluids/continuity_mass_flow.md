# Continuity Mass Flow

**Path:** `fluids.continuity_mass_flow`  
**Category:** `fluids`

## Equation

$$
\dot{m} = \rho A V
$$

- Unicode: `m_dot = ρ · A · V`
- ASCII: `m_dot = rho * A * V`

## Assumptions

- One-dimensional mean properties represent cross-section averages.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>m_dot</code></td><td>Mass flow rate</td><td><span class="math inline">\(m_{dot}\)</span></td><td><code>mass_flow_rate</code></td><td><code>kg/s</code></td><td><code>-</code></td></tr>
    <tr><td><code>rho</code></td><td>Fluid density</td><td><span class="math inline">\(\rho\)</span></td><td><code>density</code></td><td><code>kg/m3</code></td><td><code>-</code></td></tr>
    <tr><td><code>A</code></td><td>Flow area</td><td><span class="math inline">\(A\)</span></td><td><code>area</code></td><td><code>m2</code></td><td><code>-</code></td></tr>
    <tr><td><code>V</code></td><td>Mean velocity</td><td><span class="math inline">\(V\)</span></td><td><code>velocity</code></td><td><code>m/s</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `A`: explicit
- `V`: explicit
- `m_dot`: explicit
- `rho`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(fluids::continuity_mass_flow::equation())
    .target_m_dot()
    .given_rho(998.0)
    .given_a(0.1)
    .given_v(3.0)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(fluids::continuity_mass_flow::equation())
    .target_m_dot()
    .given_rho("998 kg/m3")
    .given_a("0.1 m2")
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
    <tr><td><code>m_dot</code></td><td><code>solve_m_dot(rho, A, V)</code></td><td><code>rho</code>, <code>A</code>, <code>V</code></td></tr>
    <tr><td><code>rho</code></td><td><code>solve_rho(m_dot, A, V)</code></td><td><code>m_dot</code>, <code>A</code>, <code>V</code></td></tr>
    <tr><td><code>A</code></td><td><code>solve_a(m_dot, rho, V)</code></td><td><code>m_dot</code>, <code>rho</code>, <code>V</code></td></tr>
    <tr><td><code>V</code></td><td><code>solve_v(m_dot, rho, A)</code></td><td><code>m_dot</code>, <code>rho</code>, <code>A</code></td></tr>
  </tbody>
</table>

### Solve `m_dot`

**Function signature**

```rust
equations::fluids::continuity_mass_flow::solve_m_dot(rho, A, V) -> Result<f64, _>
```

**Example**

```rust
let value = equations::fluids::continuity_mass_flow::solve_m_dot(
    "998 kg/m3",
    "0.1 m2",
    "3 m/s",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

