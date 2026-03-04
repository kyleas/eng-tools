# Convection Thermal Resistance

**Path ID:** `heat_transfer.thermal_resistance_convection`

\[
R_{th} = \frac{1}{h A}
\]

- Unicode: `R_th = 1 / (h · A)`
- ASCII: `R_th = 1 / (h * A)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>R_th</code></td><td>Thermal resistance</td><td>\(R_th\)</td><td><code>thermal_resistance</code></td><td><code>K/W</code></td></tr>
<tr><td><code>h</code></td><td>Convective heat transfer coefficient</td><td>\(h\)</td><td><code>heat_transfer_coefficient</code></td><td><code>W/(m2*K)</code></td></tr>
<tr><td><code>A</code></td><td>Surface area</td><td>\(A\)</td><td><code>area</code></td><td><code>m2</code></td></tr>
</tbody></table>

## Assumptions

- Uniform convection coefficient over the area.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(heat_transfer::thermal_resistance_convection::equation())
    .target_r_th()
    .given_h(35.0)
    .given_a(3.0)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(heat_transfer::thermal_resistance_convection::equation())
    .target_r_th()
    .given_h("35 W/(m2*K)")
    .given_a("3 m2")
    .value()?;
```

### convenience_r_th

```rust
let value = equations::heat_transfer::thermal_resistance_convection::solve_r_th(
    "35 W/(m2*K)",
    "3 m2",
)?;
```

### convenience_h

```rust
let value = equations::heat_transfer::thermal_resistance_convection::solve_h(
    "0.0095238095238 K/W",
    "3 m2",
)?;
```

### convenience_a

```rust
let value = equations::heat_transfer::thermal_resistance_convection::solve_a(
    "0.0095238095238 K/W",
    "35 W/(m2*K)",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::heat_transfer::thermal_resistance_convection::equation()).for_target("A").value()?;
```

### Python
```python
engpy.equations.heat_transfer.solve_a(r_th="...", h="...")
```

### Excel
```excel
=ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONVECTION_A("...","...")
```

**Excel arguments**
- `r_th`: Thermal resistance
- `h`: Convective heat transfer coefficient

