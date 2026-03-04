# Plane-Wall Conduction Heat Rate

**Path:** `heat_transfer.conduction_plane_wall_heat_rate`  
**Category:** `heat_transfer`

## Equation

$$
\dot{Q} = k A \frac{T_h - T_c}{L}
$$

- Unicode: `Q_dot = k · A · (T_h - T_c) / L`
- ASCII: `Q_dot = k * A * (T_h - T_c) / L`

## Assumptions

- One-dimensional steady conduction through a homogeneous plane wall.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>Q_dot</code></td><td>Heat transfer rate</td><td><span class="math inline">\(Q_{dot}\)</span></td><td><code>heat_rate</code></td><td><code>W</code></td><td><code>-</code></td></tr>
    <tr><td><code>k</code></td><td>Thermal conductivity</td><td><span class="math inline">\(k\)</span></td><td><code>thermal_conductivity</code></td><td><code>W/(m*K)</code></td><td><code>material_property:thermal_conductivity</code> from <code>material</code></td></tr>
    <tr><td><code>A</code></td><td>Area normal to heat flow</td><td><span class="math inline">\(A\)</span></td><td><code>area</code></td><td><code>m2</code></td><td><code>-</code></td></tr>
    <tr><td><code>T_h</code></td><td>Hot-side temperature</td><td><span class="math inline">\(T_{h}\)</span></td><td><code>temperature</code></td><td><code>K</code></td><td><code>-</code></td></tr>
    <tr><td><code>T_c</code></td><td>Cold-side temperature</td><td><span class="math inline">\(T_{c}\)</span></td><td><code>temperature</code></td><td><code>K</code></td><td><code>-</code></td></tr>
    <tr><td><code>L</code></td><td>Wall thickness</td><td><span class="math inline">\(L\)</span></td><td><code>length</code></td><td><code>m</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Resolvable from Contexts

- `k` from context `material` via `material_property`:`thermal_conductivity`

## Solve Targets

- `A`: explicit
- `L`: explicit
- `Q_dot`: explicit
- `T_c`: explicit
- `T_h`: explicit
- `k`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(heat_transfer::conduction_plane_wall_heat_rate::equation())
    .target_q_dot()
    .given_a(2.0)
    .given_t_h(330.0)
    .given_t_c(280.0)
    .given_l(1.0)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(heat_transfer::conduction_plane_wall_heat_rate::equation())
    .target_q_dot()
    .given_a("2 m2")
    .given_t_h("330 K")
    .given_t_c("280 K")
    .given_l("1 m")
    .value()?;
```

### Typed Builder (Context-Assisted)

```rust
let value = eq
    .solve_with_context(heat_transfer::conduction_plane_wall_heat_rate::equation())
    .material(eng_materials::stainless_304().temperature("350 K")?)
    .target_q_dot()
    .given_a("2 m2")
    .given_t_h("330 K")
    .given_t_c("280 K")
    .given_l("1 m")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>Q_dot</code></td><td><code>solve_q_dot(A, T_h, T_c, L)</code></td><td><code>A</code>, <code>T_h</code>, <code>T_c</code>, <code>L</code></td></tr>
    <tr><td><code>k</code></td><td><code>solve_k(Q_dot, A, T_h, T_c, L)</code></td><td><code>Q_dot</code>, <code>A</code>, <code>T_h</code>, <code>T_c</code>, <code>L</code></td></tr>
    <tr><td><code>A</code></td><td><code>solve_a(Q_dot, T_h, T_c, L)</code></td><td><code>Q_dot</code>, <code>T_h</code>, <code>T_c</code>, <code>L</code></td></tr>
    <tr><td><code>T_h</code></td><td><code>solve_t_h(Q_dot, A, T_c, L)</code></td><td><code>Q_dot</code>, <code>A</code>, <code>T_c</code>, <code>L</code></td></tr>
    <tr><td><code>T_c</code></td><td><code>solve_t_c(Q_dot, A, T_h, L)</code></td><td><code>Q_dot</code>, <code>A</code>, <code>T_h</code>, <code>L</code></td></tr>
    <tr><td><code>L</code></td><td><code>solve_l(Q_dot, A, T_h, T_c)</code></td><td><code>Q_dot</code>, <code>A</code>, <code>T_h</code>, <code>T_c</code></td></tr>
  </tbody>
</table>

### Solve `Q_dot`

**Function signature**

```rust
equations::heat_transfer::conduction_plane_wall_heat_rate::solve_q_dot(A, T_h, T_c, L) -> Result<f64, _>
```

**Example**

```rust
let value = equations::heat_transfer::conduction_plane_wall_heat_rate::solve_q_dot(
    "2 m2",
    "330 K",
    "280 K",
    "1 m",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

## Source

- Incropera et al., Fundamentals of Heat and Mass Transfer

