# Incompressible Orifice Mass Flow

**Path:** `fluids.orifice_mass_flow_incompressible`  
**Category:** `fluids`

## Equation

$$
\dot{m} = C_d A \sqrt{2 \rho \Delta p}
$$

- Unicode: `m_dot = C_d · A · √(2 · ρ · Δ p)`
- ASCII: `m_dot = C_d * A * sqrt(2 * rho * delta_p)`

## Assumptions

- Incompressible single-phase flow through a sharp-edged restriction.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>m_dot</code></td><td>Mass flow rate</td><td><span class="math inline">\(m_{dot}\)</span></td><td><code>mass_flow_rate</code></td><td><code>kg/s</code></td><td><code>-</code></td></tr>
    <tr><td><code>C_d</code></td><td>Discharge coefficient</td><td><span class="math inline">\(C_{d}\)</span></td><td><code>ratio</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>A</code></td><td>Orifice area</td><td><span class="math inline">\(A\)</span></td><td><code>area</code></td><td><code>m2</code></td><td><code>-</code></td></tr>
    <tr><td><code>rho</code></td><td>Fluid density</td><td><span class="math inline">\(\rho\)</span></td><td><code>density</code></td><td><code>kg/m3</code></td><td><code>-</code></td></tr>
    <tr><td><code>delta_p</code></td><td>Pressure drop</td><td><span class="math inline">\(\Delta p\)</span></td><td><code>pressure</code></td><td><code>Pa</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `A`: explicit
- `C_d`: explicit
- `delta_p`: explicit
- `m_dot`: explicit
- `rho`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(fluids::orifice_mass_flow_incompressible::equation())
    .target_m_dot()
    .given_c_d(1.0)
    .given_a(1.0)
    .given_rho(1.0)
    .given_delta_p(0.5)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(fluids::orifice_mass_flow_incompressible::equation())
    .target_m_dot()
    .given_c_d(1.0)
    .given_a("1 m2")
    .given_rho("1 kg/m3")
    .given_delta_p("0.5 Pa")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>m_dot</code></td><td><code>solve_m_dot(C_d, A, rho, delta_p)</code></td><td><code>C_d</code>, <code>A</code>, <code>rho</code>, <code>delta_p</code></td></tr>
    <tr><td><code>C_d</code></td><td><code>solve_c_d(m_dot, A, rho, delta_p)</code></td><td><code>m_dot</code>, <code>A</code>, <code>rho</code>, <code>delta_p</code></td></tr>
    <tr><td><code>A</code></td><td><code>solve_a(m_dot, C_d, rho, delta_p)</code></td><td><code>m_dot</code>, <code>C_d</code>, <code>rho</code>, <code>delta_p</code></td></tr>
    <tr><td><code>rho</code></td><td><code>solve_rho(m_dot, C_d, A, delta_p)</code></td><td><code>m_dot</code>, <code>C_d</code>, <code>A</code>, <code>delta_p</code></td></tr>
    <tr><td><code>delta_p</code></td><td><code>solve_delta_p(m_dot, C_d, A, rho)</code></td><td><code>m_dot</code>, <code>C_d</code>, <code>A</code>, <code>rho</code></td></tr>
  </tbody>
</table>

### Solve `m_dot`

**Function signature**

```rust
equations::fluids::orifice_mass_flow_incompressible::solve_m_dot(C_d, A, rho, delta_p) -> Result<f64, _>
```

**Example**

```rust
let value = equations::fluids::orifice_mass_flow_incompressible::solve_m_dot(
    1.0,
    "1 m2",
    "1 kg/m3",
    "0.5 Pa",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

