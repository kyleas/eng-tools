# Normal Shock Stagnation Pressure Ratio

**Path ID:** `compressible.normal_shock_stagnation_pressure_ratio`

$$
\frac{p_{02}}{p_{01}} = \left(\frac{(\gamma+1)M_1^2}{(\gamma-1)M_1^2+2}\right)^{\gamma/(\gamma-1)}\left(\frac{\gamma+1}{2\gamma M_1^2-(\gamma-1)}\right)^{1/(\gamma-1)}
$$

- Unicode: `p02_p01 = (((gamma+1)*M1^2)/(((gamma-1)*M1^2)+2))^(gamma/(gamma-1)) * ((gamma+1)/(2*gamma*M1^2 - (gamma-1)))^(1/(gamma-1))`
- ASCII: `p02_p01 = (((gamma+1)*M1^2)/(((gamma-1)*M1^2)+2))^(gamma/(gamma-1)) * ((gamma+1)/(2*gamma*M1^2 - (gamma-1)))^(1/(gamma-1))`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>p02_p01</code></td><td>Stagnation pressure ratio</td><td>\(\frac{p_{02}}{p_{01}}\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
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
    .solve(compressible::normal_shock_stagnation_pressure_ratio::equation())
    .target_p02_p01()
    .given_m1(2.0)
    .given_gamma(1.4)
    .value()?;
```

### convenience_p02_p01

```rust
let value = equations::compressible::normal_shock_stagnation_pressure_ratio::solve_p02_p01(
    2.0,
    1.4,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::normal_shock_stagnation_pressure_ratio::equation()).for_target("M1").value()?;
```

### Python
```python
engpy.equations.compressible.normal_shock_stagnation_pressure_ratio.solve_m1(p02_p01="...", gamma="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.normal_shock_stagnation_pressure_ratio.solve_m1(p02_p01="...", gamma="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.normal_shock_stagnation_pressure_ratio")
engpy.helpers.equation_targets_text("compressible.normal_shock_stagnation_pressure_ratio")
engpy.helpers.equation_variables_table("compressible.normal_shock_stagnation_pressure_ratio")
engpy.helpers.equation_target_count("compressible.normal_shock_stagnation_pressure_ratio")
```

### Excel
```excel
=ENG_COMPRESSIBLE_NORMAL_SHOCK_STAGNATION_PRESSURE_RATIO_M1("...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_NORMAL_SHOCK_STAGNATION_PRESSURE_RATIO_M1("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.normal_shock_stagnation_pressure_ratio")
=ENG_EQUATION_TARGETS_TEXT("compressible.normal_shock_stagnation_pressure_ratio")
=ENG_EQUATION_VARIABLES_TABLE("compressible.normal_shock_stagnation_pressure_ratio")
=ENG_EQUATION_TARGET_COUNT("compressible.normal_shock_stagnation_pressure_ratio")
```

**Excel arguments**
- `p02_p01`: Stagnation pressure ratio
- `gamma`: Specific heat ratio

