# Normal Shock Density Ratio

**Path ID:** `compressible.normal_shock_density_ratio`

$$
\frac{\rho_2}{\rho_1} = \frac{(\gamma+1)M_1^2}{(\gamma-1)M_1^2+2}
$$

- Unicode: `rho2_rho1 = ((gamma+1)*M1^2) / (((gamma-1)*M1^2) + 2)`
- ASCII: `rho2_rho1 = ((gamma+1)*M1^2) / (((gamma-1)*M1^2) + 2)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>rho2_rho1</code></td><td>Density ratio</td><td>\(\frac{\rho_2}{\rho_1}\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>M1</code></td><td>Upstream Mach number</td><td>\(M_1\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
<tr><td><code>gamma</code></td><td>Specific heat ratio</td><td>\(\gamma\)</td><td><code>dimensionless</code></td><td><code>1</code></td></tr>
</tbody></table>

## Assumptions

- One-dimensional, steady, adiabatic normal shock.
- Calorically perfect gas with constant gamma.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(compressible::normal_shock_density_ratio::equation())
    .target_rho2_rho1()
    .given_m1(2.0)
    .given_gamma(1.4)
    .value()?;
```

### convenience_rho2_rho1

```rust
let value = equations::compressible::normal_shock_density_ratio::solve_rho2_rho1(
    2.0,
    1.4,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::normal_shock_density_ratio::equation()).for_target("M1").value()?;
```

### Python
```python
engpy.equations.compressible.normal_shock_density_ratio.solve_m1(rho2_rho1="...", gamma="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.normal_shock_density_ratio.solve_m1(rho2_rho1="...", gamma="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.normal_shock_density_ratio")
engpy.helpers.equation_targets_text("compressible.normal_shock_density_ratio")
engpy.helpers.equation_variables_table("compressible.normal_shock_density_ratio")
engpy.helpers.equation_target_count("compressible.normal_shock_density_ratio")
```

### Excel
```excel
=ENG_COMPRESSIBLE_NORMAL_SHOCK_DENSITY_RATIO_M1("...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_NORMAL_SHOCK_DENSITY_RATIO_M1("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.normal_shock_density_ratio")
=ENG_EQUATION_TARGETS_TEXT("compressible.normal_shock_density_ratio")
=ENG_EQUATION_VARIABLES_TABLE("compressible.normal_shock_density_ratio")
=ENG_EQUATION_TARGET_COUNT("compressible.normal_shock_density_ratio")
```

**Excel arguments**
- `rho2_rho1`: Density ratio
- `gamma`: Specific heat ratio

