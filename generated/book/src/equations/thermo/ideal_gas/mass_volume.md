# Ideal Gas Law (Mass-Volume Form)

**Path ID:** `thermo.ideal_gas.mass_volume`

$$
P V = m R T
$$

- Unicode: `P = m · R · T / V`
- ASCII: `P = m * R * T / V`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>P</code></td><td>Absolute pressure</td><td>\(P\)</td><td><code>pressure</code></td><td><code>Pa</code></td></tr>
<tr><td><code>V</code></td><td>Control-volume</td><td>\(V\)</td><td><code>volume</code></td><td><code>m3</code></td></tr>
<tr><td><code>m</code></td><td>Gas mass</td><td>\(m\)</td><td><code>mass</code></td><td><code>kg</code></td></tr>
<tr><td><code>R</code></td><td>Specific gas constant</td><td>\(R\)</td><td><code>gas_constant</code></td><td><code>J/(kg*K)</code></td></tr>
<tr><td><code>T</code></td><td>Absolute temperature</td><td>\(T\)</td><td><code>temperature</code></td><td><code>K</code></td></tr>
</tbody></table>

## Assumptions

- Thermally and calorically ideal gas behavior.
- Gas is in equilibrium at a single representative state.

## Family

- Family: [`Ideal Gas Law`](../../families/ideal_gas.md)
- Variant: `Mass-Volume Form` (`mass_volume`)
- Use when: Use when total mass and control-volume size are primary knowns.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(thermo::mass_volume::equation())
    .target_p()
    .given_v(0.1)
    .given_m(0.1176829268)
    .given_r(287.0)
    .given_t(300.0)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(thermo::mass_volume::equation())
    .target_p()
    .given_v("0.1 m3")
    .given_m("0.1176829268292683 kg")
    .given_r("287 J/(kg*K)")
    .given_t("300 K")
    .value()?;
```

### convenience_p

```rust
let value = equations::thermo::mass_volume::solve_p(
    "0.1 m3",
    "0.1176829268292683 kg",
    "287 J/(kg*K)",
    "300 K",
)?;
```

### convenience_v

```rust
let value = equations::thermo::mass_volume::solve_v(
    "101325 Pa",
    "0.1176829268292683 kg",
    "287 J/(kg*K)",
    "300 K",
)?;
```

### convenience_m

```rust
let value = equations::thermo::mass_volume::solve_m(
    "101325 Pa",
    "0.1 m3",
    "287 J/(kg*K)",
    "300 K",
)?;
```

### convenience_r

```rust
let value = equations::thermo::mass_volume::solve_r(
    "101325 Pa",
    "0.1 m3",
    "0.1176829268292683 kg",
    "300 K",
)?;
```

### convenience_t

```rust
let value = equations::thermo::mass_volume::solve_t(
    "101325 Pa",
    "0.1 m3",
    "0.1176829268292683 kg",
    "287 J/(kg*K)",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::thermo::ideal_gas::mass_volume::equation()).for_target("P").value()?;
```

### Python
```python
engpy.equations.thermo.mass_volume.solve_p(v="...", m="...", r="...", t="...")
# helper layer
engpy.helpers.format_value(engpy.equations.thermo.mass_volume.solve_p(v="...", m="...", r="...", t="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("thermo.ideal_gas.mass_volume")
engpy.helpers.equation_targets_text("thermo.ideal_gas.mass_volume")
engpy.helpers.equation_variables_table("thermo.ideal_gas.mass_volume")
engpy.helpers.equation_target_count("thermo.ideal_gas.mass_volume")
```

### Excel
```excel
=ENG_THERMO_IDEAL_GAS_MASS_VOLUME_P("...","...","...","...")
=ENG_FORMAT(ENG_THERMO_IDEAL_GAS_MASS_VOLUME_P("...","...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("thermo.ideal_gas.mass_volume")
=ENG_EQUATION_TARGETS_TEXT("thermo.ideal_gas.mass_volume")
=ENG_EQUATION_VARIABLES_TABLE("thermo.ideal_gas.mass_volume")
=ENG_EQUATION_TARGET_COUNT("thermo.ideal_gas.mass_volume")
```

**Excel arguments**
- `v`: Control-volume
- `m`: Gas mass
- `r`: Specific gas constant
- `t`: Absolute temperature

