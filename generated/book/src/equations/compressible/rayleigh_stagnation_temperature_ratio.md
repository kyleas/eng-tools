# Rayleigh Stagnation Temperature Ratio

**Path ID:** `compressible.rayleigh_stagnation_temperature_ratio`

$$
\frac{T_0}{T_0^*} = \frac{2(\gamma+1)M^2\left(1+\frac{\gamma-1}{2}M^2\right)}{(1+\gamma M^2)^2}
$$

- Unicode: `T0/T0* = (2*(gamma+1)*M^2*(1 + ((gamma-1)/2)*M^2)) / (1 + gamma*M^2)^2`
- ASCII: `t0_t0star = (2*(gamma+1)*M^2*(1 + ((gamma-1)/2)*M^2)) / (1 + gamma*M^2)^2`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>t0_t0star</code></td><td>Stagnation temperature ratio to star state</td><td>\(\frac{T_0}{T_0^*}\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>M</code></td><td>Mach number</td><td>\(M\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
<tr><td><code>gamma</code></td><td>Specific heat ratio</td><td>\(\gamma\)</td><td><code>dimensionless</code></td><td><code>1</code></td></tr>
</tbody></table>

## Assumptions

- One-dimensional, steady, constant-area, frictionless flow with heat transfer.
- Calorically perfect gas with constant gamma.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(compressible::rayleigh_stagnation_temperature_ratio::equation())
    .target_t0_t0star()
    .branch_subsonic()
    .given_m(0.5)
    .given_gamma(1.4)
    .value()?;
```

### convenience_t0_t0star

```rust
let value = equations::compressible::rayleigh_stagnation_temperature_ratio::solve_t0_t0star(
    0.5,
    1.4,
)?;
```

### typed_builder_branch

```rust
let value = eq
    .solve(compressible::rayleigh_stagnation_temperature_ratio::equation())
    .target_t0_t0star()
    .branch_supersonic()
    .given_m(2.0)
    .given_gamma(1.4)
    .value()?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::rayleigh_stagnation_temperature_ratio::equation()).for_target("M").value()?;
```

### Python
```python
engpy.equations.compressible.rayleigh_stagnation_temperature_ratio.solve_m(t0_t0star="...", gamma="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.rayleigh_stagnation_temperature_ratio.solve_m(t0_t0star="...", gamma="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.rayleigh_stagnation_temperature_ratio")
engpy.helpers.equation_targets_text("compressible.rayleigh_stagnation_temperature_ratio")
engpy.helpers.equation_variables_table("compressible.rayleigh_stagnation_temperature_ratio")
engpy.helpers.equation_target_count("compressible.rayleigh_stagnation_temperature_ratio")
```

### Excel
```excel
=ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_TEMPERATURE_RATIO_M("...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_TEMPERATURE_RATIO_M("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.rayleigh_stagnation_temperature_ratio")
=ENG_EQUATION_TARGETS_TEXT("compressible.rayleigh_stagnation_temperature_ratio")
=ENG_EQUATION_VARIABLES_TABLE("compressible.rayleigh_stagnation_temperature_ratio")
=ENG_EQUATION_TARGET_COUNT("compressible.rayleigh_stagnation_temperature_ratio")
```

**Excel arguments**
- `t0_t0star`: Stagnation temperature ratio to star state
- `gamma`: Specific heat ratio


**Branch behavior**
- Default solver behavior uses preferred branch (`supersonic`) when one is marked.
- Supported branches: `subsonic`, `supersonic`

### Python (explicit branch)
```python
engpy.equations.compressible.rayleigh_stagnation_temperature_ratio.solve_m(t0_t0star="...", gamma="...", branch="supersonic")
```

### Excel (explicit branch)
```excel
=ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_TEMPERATURE_RATIO_M("...","...","supersonic")
=ENG_EQUATION_BRANCHES_TEXT("compressible.rayleigh_stagnation_temperature_ratio")
=ENG_EQUATION_BRANCHES_TABLE("compressible.rayleigh_stagnation_temperature_ratio")
```
