# Plane-Wall Conduction Heat Rate

**Path ID:** `heat_transfer.conduction_plane_wall_heat_rate`

\[
\dot{Q} = k A \frac{T_h - T_c}{L}
\]

- Unicode: `Q_dot = k · A · (T_h - T_c) / L`
- ASCII: `Q_dot = k * A * (T_h - T_c) / L`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>Q_dot</code></td><td>Heat transfer rate</td><td>\(Q_dot\)</td><td><code>heat_rate</code></td><td><code>W</code></td></tr>
<tr><td><code>k</code></td><td>Thermal conductivity</td><td>\(k\)</td><td><code>thermal_conductivity</code></td><td><code>W/(m*K)</code></td></tr>
<tr><td><code>A</code></td><td>Area normal to heat flow</td><td>\(A\)</td><td><code>area</code></td><td><code>m2</code></td></tr>
<tr><td><code>T_h</code></td><td>Hot-side temperature</td><td>\(T_h\)</td><td><code>temperature</code></td><td><code>K</code></td></tr>
<tr><td><code>T_c</code></td><td>Cold-side temperature</td><td>\(T_c\)</td><td><code>temperature</code></td><td><code>K</code></td></tr>
<tr><td><code>L</code></td><td>Wall thickness</td><td>\(L\)</td><td><code>length</code></td><td><code>m</code></td></tr>
</tbody></table>

## Assumptions

- One-dimensional steady conduction through a homogeneous plane wall.

## Examples

### typed_builder_si

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

### typed_builder_units

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

### typed_builder_context

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

### convenience_q_dot

```rust
let value = equations::heat_transfer::conduction_plane_wall_heat_rate::solve_q_dot(
    "2 m2",
    "330 K",
    "280 K",
    "1 m",
)?;
```

### convenience_k

```rust
let value = equations::heat_transfer::conduction_plane_wall_heat_rate::solve_k(
    "1600 W",
    "2 m2",
    "330 K",
    "280 K",
    "1 m",
)?;
```

### convenience_a

```rust
let value = equations::heat_transfer::conduction_plane_wall_heat_rate::solve_a(
    "1600 W",
    "330 K",
    "280 K",
    "1 m",
)?;
```

### convenience_t_h

```rust
let value = equations::heat_transfer::conduction_plane_wall_heat_rate::solve_t_h(
    "1600 W",
    "2 m2",
    "280 K",
    "1 m",
)?;
```

### convenience_t_c

```rust
let value = equations::heat_transfer::conduction_plane_wall_heat_rate::solve_t_c(
    "1600 W",
    "2 m2",
    "330 K",
    "1 m",
)?;
```

### convenience_l

```rust
let value = equations::heat_transfer::conduction_plane_wall_heat_rate::solve_l(
    "1600 W",
    "2 m2",
    "330 K",
    "280 K",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::heat_transfer::conduction_plane_wall_heat_rate::equation()).for_target("A").value()?;
```

### Python
```python
engpy.equations.heat_transfer.solve_a(q_dot="...", k="...", t_h="...", t_c="...", l="...")
# helper layer
engpy.helpers.format_value(engpy.equations.heat_transfer.solve_a(q_dot="...", k="...", t_h="...", t_c="...", l="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("heat_transfer.conduction_plane_wall_heat_rate")
```

### Excel
```excel
=ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_A("...","...","...","...","...")
=ENG_FORMAT(ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_A("...","...","...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("heat_transfer.conduction_plane_wall_heat_rate")
```

**Excel arguments**
- `q_dot`: Heat transfer rate
- `k`: Thermal conductivity
- `t_h`: Hot-side temperature
- `t_c`: Cold-side temperature
- `l`: Wall thickness

