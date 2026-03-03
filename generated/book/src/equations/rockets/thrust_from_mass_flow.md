# Thrust From Mass Flow and Effective Exhaust Velocity

**Path:** `rockets.thrust_from_mass_flow`  
**Category:** `rockets`

## Equation

$$
F = \dot{m} c_{eff}
$$

- Unicode: `F = m_dot · c_eff`
- ASCII: `F = m_dot * c_eff`

## Assumptions

- Pressure-thrust contribution is embedded in effective exhaust velocity.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>F</code></td><td>Thrust</td><td><span class="math inline">\(F\)</span></td><td><code>force</code></td><td><code>N</code></td><td><code>-</code></td></tr>
    <tr><td><code>m_dot</code></td><td>Mass flow rate</td><td><span class="math inline">\(m_{dot}\)</span></td><td><code>mass_flow_rate</code></td><td><code>kg/s</code></td><td><code>-</code></td></tr>
    <tr><td><code>c_eff</code></td><td>Effective exhaust velocity</td><td><span class="math inline">\(c_{eff}\)</span></td><td><code>velocity</code></td><td><code>m/s</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `F`: explicit
- `c_eff`: explicit
- `m_dot`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(rockets::thrust_from_mass_flow::equation())
    .target_f()
    .given_m_dot(250.0)
    .given_c_eff(2000.0)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(rockets::thrust_from_mass_flow::equation())
    .target_f()
    .given_m_dot("250 kg/s")
    .given_c_eff("2000 m/s")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>F</code></td><td><code>solve_f(m_dot, c_eff)</code></td><td><code>m_dot</code>, <code>c_eff</code></td></tr>
    <tr><td><code>m_dot</code></td><td><code>solve_m_dot(F, c_eff)</code></td><td><code>F</code>, <code>c_eff</code></td></tr>
    <tr><td><code>c_eff</code></td><td><code>solve_c_eff(F, m_dot)</code></td><td><code>F</code>, <code>m_dot</code></td></tr>
  </tbody>
</table>

### Solve `F`

**Function signature**

```rust
equations::rockets::thrust_from_mass_flow::solve_f(m_dot, c_eff) -> Result<f64, _>
```

**Example**

```rust
let value = equations::rockets::thrust_from_mass_flow::solve_f(
    "250 kg/s",
    "2000 m/s",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

