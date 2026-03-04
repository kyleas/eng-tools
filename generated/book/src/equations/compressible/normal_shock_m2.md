# Normal Shock Downstream Mach Number

**Path ID:** `compressible.normal_shock_m2`

$$
M_2 = \sqrt{\frac{1 + \frac{\gamma-1}{2}M_1^2}{\gamma M_1^2 - \frac{\gamma-1}{2}}}
$$

- Unicode: `M2 = sqrt((1 + ((gamma-1)/2) * M1^2) / (gamma*M1^2 - ((gamma-1)/2)))`
- ASCII: `M2 = sqrt((1 + ((gamma-1)/2) * M1^2) / (gamma*M1^2 - ((gamma-1)/2)))`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>M2</code></td><td>Downstream Mach number</td><td>\(M_2\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
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
    .solve(compressible::normal_shock_m2::equation())
    .target_m2()
    .given_m1(2.0)
    .given_gamma(1.4)
    .value()?;
```

### convenience_m2

```rust
let value = equations::compressible::normal_shock_m2::solve_m2(
    2.0,
    1.4,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::normal_shock_m2::equation()).for_target("M1").value()?;
```

### Python
```python
engpy.equations.compressible.normal_shock_m2.solve_m1(m2="...", gamma="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.normal_shock_m2.solve_m1(m2="...", gamma="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.normal_shock_m2")
engpy.helpers.equation_targets_text("compressible.normal_shock_m2")
engpy.helpers.equation_variables_table("compressible.normal_shock_m2")
engpy.helpers.equation_target_count("compressible.normal_shock_m2")
```

### Excel
```excel
=ENG_COMPRESSIBLE_NORMAL_SHOCK_M2_M1("...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_NORMAL_SHOCK_M2_M1("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.normal_shock_m2")
=ENG_EQUATION_TARGETS_TEXT("compressible.normal_shock_m2")
=ENG_EQUATION_VARIABLES_TABLE("compressible.normal_shock_m2")
=ENG_EQUATION_TARGET_COUNT("compressible.normal_shock_m2")
```

**Excel arguments**
- `m2`: Downstream Mach number
- `gamma`: Specific heat ratio

