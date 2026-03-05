# Rayleigh Velocity Ratio

**Path ID:** `compressible.rayleigh_velocity_ratio`

$$
\frac{V}{V^*} = \frac{(\gamma+1)M^2}{1+\gamma M^2}
$$

- Unicode: `V/V* = ((gamma+1)*M^2) / (1 + gamma*M^2)`
- ASCII: `v_vstar = ((gamma+1)*M^2) / (1 + gamma*M^2)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>v_vstar</code></td><td>Velocity ratio to star state</td><td>\(\frac{V}{V^*}\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
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
    .solve(compressible::rayleigh_velocity_ratio::equation())
    .target_v_vstar()
    .given_m(0.5)
    .given_gamma(1.4)
    .value()?;
```

### convenience_v_vstar

```rust
let value = equations::compressible::rayleigh_velocity_ratio::solve_v_vstar(
    0.5,
    1.4,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::rayleigh_velocity_ratio::equation()).for_target("M").value()?;
```

### Python
```python
engpy.equations.compressible.rayleigh_velocity_ratio.solve_m(v_vstar="...", gamma="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.rayleigh_velocity_ratio.solve_m(v_vstar="...", gamma="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.rayleigh_velocity_ratio")
engpy.helpers.equation_targets_text("compressible.rayleigh_velocity_ratio")
engpy.helpers.equation_variables_table("compressible.rayleigh_velocity_ratio")
engpy.helpers.equation_target_count("compressible.rayleigh_velocity_ratio")
```

### Excel
```excel
=ENG_COMPRESSIBLE_RAYLEIGH_VELOCITY_RATIO_M("...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_RAYLEIGH_VELOCITY_RATIO_M("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.rayleigh_velocity_ratio")
=ENG_EQUATION_TARGETS_TEXT("compressible.rayleigh_velocity_ratio")
=ENG_EQUATION_VARIABLES_TABLE("compressible.rayleigh_velocity_ratio")
=ENG_EQUATION_TARGET_COUNT("compressible.rayleigh_velocity_ratio")
```

**Excel arguments**
- `v_vstar`: Velocity ratio to star state
- `gamma`: Specific heat ratio

