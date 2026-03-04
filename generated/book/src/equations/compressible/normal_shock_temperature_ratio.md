# Normal Shock Temperature Ratio

**Path ID:** `compressible.normal_shock_temperature_ratio`

$$
\frac{T_2}{T_1} = \frac{p_2/p_1}{\rho_2/\rho_1} = \left(1+\frac{2\gamma}{\gamma+1}(M_1^2-1)\right)\frac{(\gamma-1)M_1^2+2}{(\gamma+1)M_1^2}
$$

- Unicode: `T2_T1 = (1 + (2*gamma/(gamma+1))*(M1^2-1)) * (((gamma-1)*M1^2 + 2)/((gamma+1)*M1^2))`
- ASCII: `T2_T1 = (1 + (2*gamma/(gamma+1))*(M1^2-1)) * (((gamma-1)*M1^2 + 2)/((gamma+1)*M1^2))`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>T2_T1</code></td><td>Static temperature ratio</td><td>\(\frac{T_2}{T_1}\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
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
    .solve(compressible::normal_shock_temperature_ratio::equation())
    .target_t2_t1()
    .given_m1(2.0)
    .given_gamma(1.4)
    .value()?;
```

### convenience_t2_t1

```rust
let value = equations::compressible::normal_shock_temperature_ratio::solve_t2_t1(
    2.0,
    1.4,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::normal_shock_temperature_ratio::equation()).for_target("M1").value()?;
```

### Python
```python
engpy.equations.compressible.normal_shock_temperature_ratio.solve_m1(t2_t1="...", gamma="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.normal_shock_temperature_ratio.solve_m1(t2_t1="...", gamma="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.normal_shock_temperature_ratio")
engpy.helpers.equation_targets_text("compressible.normal_shock_temperature_ratio")
engpy.helpers.equation_variables_table("compressible.normal_shock_temperature_ratio")
engpy.helpers.equation_target_count("compressible.normal_shock_temperature_ratio")
```

### Excel
```excel
=ENG_COMPRESSIBLE_NORMAL_SHOCK_TEMPERATURE_RATIO_M1("...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_NORMAL_SHOCK_TEMPERATURE_RATIO_M1("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.normal_shock_temperature_ratio")
=ENG_EQUATION_TARGETS_TEXT("compressible.normal_shock_temperature_ratio")
=ENG_EQUATION_VARIABLES_TABLE("compressible.normal_shock_temperature_ratio")
=ENG_EQUATION_TARGET_COUNT("compressible.normal_shock_temperature_ratio")
```

**Excel arguments**
- `t2_t1`: Static temperature ratio
- `gamma`: Specific heat ratio

