# Isentropic Temperature Ratio

**Path ID:** `compressible.isentropic_temperature_ratio`

$$
\frac{T}{T_0} = \left(1+\frac{\gamma-1}{2}M^2\right)^{-1}
$$

- Unicode: `T_T0 = 1 / (1 + ((γ - 1) / 2) · M²)`
- ASCII: `T_T0 = 1 / (1 + ((gamma - 1) / 2) * M^2)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>T_T0</code></td><td>Static-to-stagnation temperature ratio</td><td>\(T_T0\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>M</code></td><td>Mach number</td><td>\(M\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
<tr><td><code>gamma</code></td><td>Specific heat ratio</td><td>\(\gamma\)</td><td><code>dimensionless</code></td><td><code>1</code></td></tr>
</tbody></table>

## Assumptions

- Isentropic perfect-gas flow with constant gamma.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(compressible::isentropic_temperature_ratio::equation())
    .target_t_t0()
    .given_m(2.0)
    .given_gamma(1.4)
    .value()?;
```

### convenience_t_t0

```rust
let value = equations::compressible::isentropic_temperature_ratio::solve_t_t0(
    2.0,
    1.4,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::isentropic_temperature_ratio::equation()).for_target("M").value()?;
```

### Python
```python
engpy.equations.compressible.isentropic_temperature_ratio.solve_m(t_t0="...", gamma="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.isentropic_temperature_ratio.solve_m(t_t0="...", gamma="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.isentropic_temperature_ratio")
engpy.helpers.equation_targets_text("compressible.isentropic_temperature_ratio")
engpy.helpers.equation_variables_table("compressible.isentropic_temperature_ratio")
engpy.helpers.equation_target_count("compressible.isentropic_temperature_ratio")
```

### Excel
```excel
=ENG_COMPRESSIBLE_ISENTROPIC_TEMPERATURE_RATIO_M("...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_ISENTROPIC_TEMPERATURE_RATIO_M("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.isentropic_temperature_ratio")
=ENG_EQUATION_TARGETS_TEXT("compressible.isentropic_temperature_ratio")
=ENG_EQUATION_VARIABLES_TABLE("compressible.isentropic_temperature_ratio")
=ENG_EQUATION_TARGET_COUNT("compressible.isentropic_temperature_ratio")
```

**Excel arguments**
- `t_t0`: Static-to-stagnation temperature ratio
- `gamma`: Specific heat ratio

