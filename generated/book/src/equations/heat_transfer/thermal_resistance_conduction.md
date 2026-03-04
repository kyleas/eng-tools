# Conduction Thermal Resistance

**Path ID:** `heat_transfer.thermal_resistance_conduction`

\[
R_{th} = \frac{L}{k A}
\]

- Unicode: `R_th = L / (k · A)`
- ASCII: `R_th = L / (k * A)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>R_th</code></td><td>Thermal resistance</td><td>\(R_th\)</td><td><code>thermal_resistance</code></td><td><code>K/W</code></td></tr>
<tr><td><code>L</code></td><td>Wall thickness</td><td>\(L\)</td><td><code>length</code></td><td><code>m</code></td></tr>
<tr><td><code>k</code></td><td>Thermal conductivity</td><td>\(k\)</td><td><code>thermal_conductivity</code></td><td><code>W/(m*K)</code></td></tr>
<tr><td><code>A</code></td><td>Area</td><td>\(A\)</td><td><code>area</code></td><td><code>m2</code></td></tr>
</tbody></table>

## Assumptions

- One-dimensional steady conduction with constant properties.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(heat_transfer::thermal_resistance_conduction::equation())
    .target_r_th()
    .given_l(0.1)
    .given_k(10.0)
    .given_a(0.2)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(heat_transfer::thermal_resistance_conduction::equation())
    .target_r_th()
    .given_l("0.1 m")
    .given_k("10 W/(m*K)")
    .given_a("0.2 m2")
    .value()?;
```

### convenience_r_th

```rust
let value = equations::heat_transfer::thermal_resistance_conduction::solve_r_th(
    "0.1 m",
    "10 W/(m*K)",
    "0.2 m2",
)?;
```

### convenience_l

```rust
let value = equations::heat_transfer::thermal_resistance_conduction::solve_l(
    "0.05 K/W",
    "10 W/(m*K)",
    "0.2 m2",
)?;
```

### convenience_k

```rust
let value = equations::heat_transfer::thermal_resistance_conduction::solve_k(
    "0.05 K/W",
    "0.1 m",
    "0.2 m2",
)?;
```

### convenience_a

```rust
let value = equations::heat_transfer::thermal_resistance_conduction::solve_a(
    "0.05 K/W",
    "0.1 m",
    "10 W/(m*K)",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::heat_transfer::thermal_resistance_conduction::equation()).for_target("A").value()?;
```

### Python
```python
engpy.equations.heat_transfer.solve_a(r_th="...", l="...", k="...")
# helper layer
engpy.helpers.format_value(engpy.equations.heat_transfer.solve_a(r_th="...", l="...", k="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("heat_transfer.thermal_resistance_conduction")
```

### Excel
```excel
=ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONDUCTION_A("...","...","...")
=ENG_FORMAT(ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONDUCTION_A("...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("heat_transfer.thermal_resistance_conduction")
```

**Excel arguments**
- `r_th`: Thermal resistance
- `l`: Wall thickness
- `k`: Thermal conductivity

