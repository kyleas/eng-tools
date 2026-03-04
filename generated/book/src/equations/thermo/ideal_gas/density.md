# Ideal Gas Law (Density Form)

**Path ID:** `thermo.ideal_gas.density`

\[
P = \rho R T
\]

- Unicode: `P = ρ · R · T`
- ASCII: `P = rho * R * T`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>P</code></td><td>Absolute pressure</td><td>\(P\)</td><td><code>pressure</code></td><td><code>Pa</code></td></tr>
<tr><td><code>rho</code></td><td>Density</td><td>\(\rho\)</td><td><code>density</code></td><td><code>kg/m3</code></td></tr>
<tr><td><code>R</code></td><td>Specific gas constant</td><td>\(R\)</td><td><code>gas_constant</code></td><td><code>J/(kg*K)</code></td></tr>
<tr><td><code>T</code></td><td>Absolute temperature</td><td>\(T\)</td><td><code>temperature</code></td><td><code>K</code></td></tr>
</tbody></table>

## Assumptions

- Thermally and calorically ideal gas behavior.
- Single-phase gas state.

## Family

- Family: [`Ideal Gas Law`](../../families/ideal_gas.md)
- Variant: `Density Form` (`density`)
- Use when: Use when density-based flow/property calculations are primary.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(thermo::density::equation())
    .target_p()
    .given_rho(1.17683)
    .given_r(287.0)
    .given_t(300.0)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(thermo::density::equation())
    .target_p()
    .given_rho("1.17683 kg/m3")
    .given_r("287 J/(kg*K)")
    .given_t("300 K")
    .value()?;
```

### convenience_p

```rust
let value = equations::thermo::density::solve_p(
    "1.17683 kg/m3",
    "287 J/(kg*K)",
    "300 K",
)?;
```

### convenience_rho

```rust
let value = equations::thermo::density::solve_rho(
    "101325.063 Pa",
    "287 J/(kg*K)",
    "300 K",
)?;
```

### convenience_r

```rust
let value = equations::thermo::density::solve_r(
    "101325.063 Pa",
    "1.17683 kg/m3",
    "300 K",
)?;
```

### convenience_t

```rust
let value = equations::thermo::density::solve_t(
    "101325.063 Pa",
    "1.17683 kg/m3",
    "287 J/(kg*K)",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::thermo::ideal_gas::density::equation()).for_target("P").value()?;
```

### Python
```python
engpy.equations.thermo.solve_p(rho="...", r="...", t="...")
# helper layer
engpy.helpers.format_value(engpy.equations.thermo.solve_p(rho="...", r="...", t="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("thermo.ideal_gas.density")
```

### Excel
```excel
=ENG_THERMO_IDEAL_GAS_DENSITY_P("...","...","...")
=ENG_FORMAT(ENG_THERMO_IDEAL_GAS_DENSITY_P("...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("thermo.ideal_gas.density")
```

**Excel arguments**
- `rho`: Density
- `r`: Specific gas constant
- `t`: Absolute temperature

