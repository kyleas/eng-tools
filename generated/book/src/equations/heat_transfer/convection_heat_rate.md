# Convection Heat Transfer Rate

**Path ID:** `heat_transfer.convection_heat_rate`

\[
\dot{Q} = h A (T_s - T_\infty)
\]

- Unicode: `Q_dot = h · A · (T_s - T_\infty)`
- ASCII: `Q_dot = h * A * (T_s - T_inf)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>Q_dot</code></td><td>Heat transfer rate</td><td>\(Q_dot\)</td><td><code>heat_rate</code></td><td><code>W</code></td></tr>
<tr><td><code>h</code></td><td>Convective heat transfer coefficient</td><td>\(h\)</td><td><code>heat_transfer_coefficient</code></td><td><code>W/(m2*K)</code></td></tr>
<tr><td><code>A</code></td><td>Surface area</td><td>\(A\)</td><td><code>area</code></td><td><code>m2</code></td></tr>
<tr><td><code>T_s</code></td><td>Surface temperature</td><td>\(T_s\)</td><td><code>temperature</code></td><td><code>K</code></td></tr>
<tr><td><code>T_inf</code></td><td>Free-stream temperature</td><td>\(T_\infty\)</td><td><code>temperature</code></td><td><code>K</code></td></tr>
</tbody></table>

## Assumptions

- Newton's law of cooling with uniform h and area.

## Examples

### typed_builder_si

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

### typed_builder_units

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

### convenience_q_dot

```rust
let value = equations::heat_transfer::convection_heat_rate::solve_q_dot(
    "35 W/(m2*K)",
    "3 m2",
    "400 K",
    "300 K",
)?;
```

### convenience_h

```rust
let value = equations::heat_transfer::convection_heat_rate::solve_h(
    "10500 W",
    "3 m2",
    "400 K",
    "300 K",
)?;
```

### convenience_a

```rust
let value = equations::heat_transfer::convection_heat_rate::solve_a(
    "10500 W",
    "35 W/(m2*K)",
    "400 K",
    "300 K",
)?;
```

### convenience_t_s

```rust
let value = equations::heat_transfer::convection_heat_rate::solve_t_s(
    "10500 W",
    "35 W/(m2*K)",
    "3 m2",
    "300 K",
)?;
```

### convenience_t_inf

```rust
let value = equations::heat_transfer::convection_heat_rate::solve_t_inf(
    "10500 W",
    "35 W/(m2*K)",
    "3 m2",
    "400 K",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::heat_transfer::convection_heat_rate::equation()).for_target("A").value()?;
```

### Python
```python
engpy.equations.heat_transfer.solve_a(q_dot="...", h="...", t_s="...", t_inf="...")
# helper layer
engpy.helpers.format_value(engpy.equations.heat_transfer.solve_a(q_dot="...", h="...", t_s="...", t_inf="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("heat_transfer.convection_heat_rate")
```

### Excel
```excel
=ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_A("...","...","...","...")
=ENG_FORMAT(ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_A("...","...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("heat_transfer.convection_heat_rate")
```

**Excel arguments**
- `q_dot`: Heat transfer rate
- `h`: Convective heat transfer coefficient
- `t_s`: Surface temperature
- `t_inf`: Free-stream temperature

