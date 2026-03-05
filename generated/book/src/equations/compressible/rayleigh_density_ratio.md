# Rayleigh Density Ratio

**Path ID:** `compressible.rayleigh_density_ratio`

$$
\frac{\rho}{\rho^*} = \frac{1+\gamma M^2}{(\gamma+1)M^2}
$$

- Unicode: `rho/rho* = (1 + gamma*M^2) / ((gamma+1)*M^2)`
- ASCII: `rho_rhostar = (1 + gamma*M^2) / ((gamma+1)*M^2)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>rho_rhostar</code></td><td>Density ratio to star state</td><td>\(\frac{\rho}{\rho^*}\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
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
    .solve(compressible::rayleigh_density_ratio::equation())
    .target_rho_rhostar()
    .given_m(0.5)
    .given_gamma(1.4)
    .value()?;
```

### convenience_rho_rhostar

```rust
let value = equations::compressible::rayleigh_density_ratio::solve_rho_rhostar(
    0.5,
    1.4,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::rayleigh_density_ratio::equation()).for_target("M").value()?;
```

### Python
```python
engpy.equations.compressible.rayleigh_density_ratio.solve_m(rho_rhostar="...", gamma="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.rayleigh_density_ratio.solve_m(rho_rhostar="...", gamma="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.rayleigh_density_ratio")
engpy.helpers.equation_targets_text("compressible.rayleigh_density_ratio")
engpy.helpers.equation_variables_table("compressible.rayleigh_density_ratio")
engpy.helpers.equation_target_count("compressible.rayleigh_density_ratio")
```

### Excel
```excel
=ENG_COMPRESSIBLE_RAYLEIGH_DENSITY_RATIO_M("...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_RAYLEIGH_DENSITY_RATIO_M("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.rayleigh_density_ratio")
=ENG_EQUATION_TARGETS_TEXT("compressible.rayleigh_density_ratio")
=ENG_EQUATION_VARIABLES_TABLE("compressible.rayleigh_density_ratio")
=ENG_EQUATION_TARGET_COUNT("compressible.rayleigh_density_ratio")
```

**Excel arguments**
- `rho_rhostar`: Density ratio to star state
- `gamma`: Specific heat ratio

