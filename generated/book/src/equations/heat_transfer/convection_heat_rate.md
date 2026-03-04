# Convection Heat Transfer Rate

**Path:** `heat_transfer.convection_heat_rate`  
**Category:** `heat_transfer`

## Equation

$$
\dot{Q} = h A (T_s - T_\infty)
$$

- Unicode: `Q_dot = h · A · (T_s - T_\infty)`
- ASCII: `Q_dot = h * A * (T_s - T_inf)`

## Assumptions

- Newton's law of cooling with uniform h and area.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>Q_dot</code></td><td>Heat transfer rate</td><td><span class="math inline">\(Q_{dot}\)</span></td><td><code>heat_rate</code></td><td><code>W</code></td><td><code>-</code></td></tr>
    <tr><td><code>h</code></td><td>Convective heat transfer coefficient</td><td><span class="math inline">\(h\)</span></td><td><code>heat_transfer_coefficient</code></td><td><code>W/(m2*K)</code></td><td><code>-</code></td></tr>
    <tr><td><code>A</code></td><td>Surface area</td><td><span class="math inline">\(A\)</span></td><td><code>area</code></td><td><code>m2</code></td><td><code>-</code></td></tr>
    <tr><td><code>T_s</code></td><td>Surface temperature</td><td><span class="math inline">\(T_{s}\)</span></td><td><code>temperature</code></td><td><code>K</code></td><td><code>-</code></td></tr>
    <tr><td><code>T_inf</code></td><td>Free-stream temperature</td><td><span class="math inline">\(T_\infty\)</span></td><td><code>temperature</code></td><td><code>K</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `A`: explicit
- `Q_dot`: explicit
- `T_inf`: explicit
- `T_s`: explicit
- `h`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(heat_transfer::convection_heat_rate::equation())
    .target_q_dot()
    .given_h(35.0)
    .given_a(3.0)
    .given_t_s(400.0)
    .given_t_inf(300.0)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(heat_transfer::convection_heat_rate::equation())
    .target_q_dot()
    .given_h("35 W/(m2*K)")
    .given_a("3 m2")
    .given_t_s("400 K")
    .given_t_inf("300 K")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>Q_dot</code></td><td><code>solve_q_dot(h, A, T_s, T_inf)</code></td><td><code>h</code>, <code>A</code>, <code>T_s</code>, <code>T_inf</code></td></tr>
    <tr><td><code>h</code></td><td><code>solve_h(Q_dot, A, T_s, T_inf)</code></td><td><code>Q_dot</code>, <code>A</code>, <code>T_s</code>, <code>T_inf</code></td></tr>
    <tr><td><code>A</code></td><td><code>solve_a(Q_dot, h, T_s, T_inf)</code></td><td><code>Q_dot</code>, <code>h</code>, <code>T_s</code>, <code>T_inf</code></td></tr>
    <tr><td><code>T_s</code></td><td><code>solve_t_s(Q_dot, h, A, T_inf)</code></td><td><code>Q_dot</code>, <code>h</code>, <code>A</code>, <code>T_inf</code></td></tr>
    <tr><td><code>T_inf</code></td><td><code>solve_t_inf(Q_dot, h, A, T_s)</code></td><td><code>Q_dot</code>, <code>h</code>, <code>A</code>, <code>T_s</code></td></tr>
  </tbody>
</table>

### Solve `Q_dot`

**Function signature**

```rust
equations::heat_transfer::convection_heat_rate::solve_q_dot(h, A, T_s, T_inf) -> Result<f64, _>
```

**Example**

```rust
let value = equations::heat_transfer::convection_heat_rate::solve_q_dot(
    "35 W/(m2*K)",
    "3 m2",
    "400 K",
    "300 K",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

## Source

- Incropera et al., Fundamentals of Heat and Mass Transfer

