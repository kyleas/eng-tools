# Darcy-Weisbach Pressure Drop

**Path ID:** `fluids.darcy_weisbach_pressure_drop`

$$
\Delta p = f \frac{L}{D} \frac{\rho V^2}{2}
$$

- Unicode: `Δ p = f · (L / D) · (ρ · V² / 2)`
- ASCII: `delta_p = f * (L / D) * (rho * V^2 / 2)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>delta_p</code></td><td>Pressure drop</td><td>\(\Delta p\)</td><td><code>pressure</code></td><td><code>Pa</code></td></tr>
<tr><td><code>f</code></td><td>Darcy friction factor</td><td>\(f\)</td><td><code>friction_factor</code></td><td><code>1</code></td></tr>
<tr><td><code>L</code></td><td>Pipe length</td><td>\(L\)</td><td><code>length</code></td><td><code>m</code></td></tr>
<tr><td><code>D</code></td><td>Pipe diameter</td><td>\(D\)</td><td><code>length</code></td><td><code>m</code></td></tr>
<tr><td><code>rho</code></td><td>Fluid density</td><td>\(\rho\)</td><td><code>density</code></td><td><code>kg/m3</code></td></tr>
<tr><td><code>V</code></td><td>Mean velocity</td><td>\(V\)</td><td><code>velocity</code></td><td><code>m/s</code></td></tr>
</tbody></table>

## Assumptions

- Fully developed internal flow in a constant-diameter conduit.

## Examples

### typed_builder_si

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

### typed_builder_units

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

### convenience_delta_p

```rust
let value = equations::fluids::darcy_weisbach_pressure_drop::solve_delta_p(
    0.02,
    "10 m",
    "0.1 m",
    "1000 kg/m3",
    "3 m/s",
)?;
```

### convenience_f

```rust
let value = equations::fluids::darcy_weisbach_pressure_drop::solve_f(
    "9000 Pa",
    "10 m",
    "0.1 m",
    "1000 kg/m3",
    "3 m/s",
)?;
```

### convenience_l

```rust
let value = equations::fluids::darcy_weisbach_pressure_drop::solve_l(
    "9000 Pa",
    0.02,
    "0.1 m",
    "1000 kg/m3",
    "3 m/s",
)?;
```

### convenience_d

```rust
let value = equations::fluids::darcy_weisbach_pressure_drop::solve_d(
    "9000 Pa",
    0.02,
    "10 m",
    "1000 kg/m3",
    "3 m/s",
)?;
```

### convenience_rho

```rust
let value = equations::fluids::darcy_weisbach_pressure_drop::solve_rho(
    "9000 Pa",
    0.02,
    "10 m",
    "0.1 m",
    "3 m/s",
)?;
```

### convenience_v

```rust
let value = equations::fluids::darcy_weisbach_pressure_drop::solve_v(
    "9000 Pa",
    0.02,
    "10 m",
    "0.1 m",
    "1000 kg/m3",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::fluids::darcy_weisbach_pressure_drop::equation()).for_target("D").value()?;
```

### Python
```python
engpy.equations.fluids.darcy_weisbach_pressure_drop.solve_d(delta_p="...", f="...", l="...", rho="...", v="...")
# helper layer
engpy.helpers.format_value(engpy.equations.fluids.darcy_weisbach_pressure_drop.solve_d(delta_p="...", f="...", l="...", rho="...", v="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("fluids.darcy_weisbach_pressure_drop")
engpy.helpers.equation_targets_text("fluids.darcy_weisbach_pressure_drop")
engpy.helpers.equation_variables_table("fluids.darcy_weisbach_pressure_drop")
engpy.helpers.equation_target_count("fluids.darcy_weisbach_pressure_drop")
```

### Excel
```excel
=ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_D("...","...","...","...","...")
=ENG_FORMAT(ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_D("...","...","...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("fluids.darcy_weisbach_pressure_drop")
=ENG_EQUATION_TARGETS_TEXT("fluids.darcy_weisbach_pressure_drop")
=ENG_EQUATION_VARIABLES_TABLE("fluids.darcy_weisbach_pressure_drop")
=ENG_EQUATION_TARGET_COUNT("fluids.darcy_weisbach_pressure_drop")
```

**Excel arguments**
- `delta_p`: Pressure drop
- `f`: Darcy friction factor
- `l`: Pipe length
- `rho`: Fluid density
- `v`: Mean velocity

