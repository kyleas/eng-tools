# Fanno Temperature Ratio

**Path ID:** `compressible.fanno_temperature_ratio`

$$
\frac{T}{T^*} = \frac{\frac{\gamma+1}{2}}{1+\frac{\gamma-1}{2}M^2}
$$

- Unicode: `T/T* = ((gamma+1)/2) / (1 + ((gamma-1)/2)*M^2)`
- ASCII: `t_tstar = ((gamma+1)/2) / (1 + ((gamma-1)/2)*M^2)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>t_tstar</code></td><td>Temperature ratio to star state</td><td>\(\frac{T}{T^*}\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>M</code></td><td>Mach number</td><td>\(M\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
<tr><td><code>gamma</code></td><td>Specific heat ratio</td><td>\(\gamma\)</td><td><code>dimensionless</code></td><td><code>1</code></td></tr>
</tbody></table>

## Assumptions

- One-dimensional, steady, adiabatic, constant-area duct flow with wall friction.
- Calorically perfect gas with constant gamma.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(compressible::fanno_temperature_ratio::equation())
    .target_t_tstar()
    .branch_subsonic()
    .given_m(0.5)
    .given_gamma(1.4)
    .value()?;
```

### convenience_t_tstar

```rust
let value = equations::compressible::fanno_temperature_ratio::solve_t_tstar(
    0.5,
    1.4,
)?;
```

### typed_builder_branch

```rust
let value = eq
    .solve(compressible::fanno_temperature_ratio::equation())
    .target_t_tstar()
    .branch_supersonic()
    .given_m(2.0)
    .given_gamma(1.4)
    .value()?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::fanno_temperature_ratio::equation()).for_target("M").value()?;
```

### Python
```python
engpy.equations.compressible.fanno_temperature_ratio.solve_m(t_tstar="...", gamma="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.fanno_temperature_ratio.solve_m(t_tstar="...", gamma="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.fanno_temperature_ratio")
engpy.helpers.equation_targets_text("compressible.fanno_temperature_ratio")
engpy.helpers.equation_variables_table("compressible.fanno_temperature_ratio")
engpy.helpers.equation_target_count("compressible.fanno_temperature_ratio")
```

### Excel
```excel
=ENG_COMPRESSIBLE_FANNO_TEMPERATURE_RATIO_M("...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_FANNO_TEMPERATURE_RATIO_M("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.fanno_temperature_ratio")
=ENG_EQUATION_TARGETS_TEXT("compressible.fanno_temperature_ratio")
=ENG_EQUATION_VARIABLES_TABLE("compressible.fanno_temperature_ratio")
=ENG_EQUATION_TARGET_COUNT("compressible.fanno_temperature_ratio")
```

**Excel arguments**
- `t_tstar`: Temperature ratio to star state
- `gamma`: Specific heat ratio


**Branch behavior**
- Default solver behavior uses preferred branch (`supersonic`) when one is marked.
- Supported branches: `subsonic`, `supersonic`

### Python (explicit branch)
```python
engpy.equations.compressible.fanno_temperature_ratio.solve_m(t_tstar="...", gamma="...", branch="supersonic")
```

### Excel (explicit branch)
```excel
=ENG_COMPRESSIBLE_FANNO_TEMPERATURE_RATIO_M("...","...","supersonic")
=ENG_EQUATION_BRANCHES_TEXT("compressible.fanno_temperature_ratio")
=ENG_EQUATION_BRANCHES_TABLE("compressible.fanno_temperature_ratio")
```
