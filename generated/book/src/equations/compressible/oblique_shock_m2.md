# Oblique Shock Downstream Mach

**Path ID:** `compressible.oblique_shock_m2`

$$
M_2 = \frac{M_{n2}}{\sin(\beta-\theta)}
$$

- Unicode: `m2 = mn2 / sin(beta - theta)`
- ASCII: `m2 = mn2 / sin(beta - theta)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>m2</code></td><td>Downstream Mach number</td><td>\(M_2\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
<tr><td><code>mn2</code></td><td>Downstream normal Mach</td><td>\(M_{n2}\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
<tr><td><code>beta</code></td><td>Shock angle</td><td>\(\beta\)</td><td><code>angle</code></td><td><code>rad</code></td></tr>
<tr><td><code>theta</code></td><td>Flow deflection angle</td><td>\(\theta\)</td><td><code>angle</code></td><td><code>rad</code></td></tr>
</tbody></table>

## Assumptions

- Attached oblique shock geometry with beta > theta.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(compressible::oblique_shock_m2::equation())
    .target_m2()
    .given_mn2(0.7933844244)
    .given_beta(0.6981317008)
    .given_theta(0.1854047491)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(compressible::oblique_shock_m2::equation())
    .target_m2()
    .given_mn2(0.7933844244)
    .given_beta(0.6981317008)
    .given_theta(0.1854047491)
    .value()?;
```

### convenience_m2

```rust
let value = equations::compressible::oblique_shock_m2::solve_m2(
    0.7933844244,
    0.6981317008,
    0.1854047491,
)?;
```

### convenience_mn2

```rust
let value = equations::compressible::oblique_shock_m2::solve_mn2(
    1.617318834,
    0.6981317008,
    0.1854047491,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::oblique_shock_m2::equation()).for_target("beta").value()?;
```

### Python
```python
engpy.equations.compressible.oblique_shock_m2.solve_beta(m2="...", mn2="...", theta="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.oblique_shock_m2.solve_beta(m2="...", mn2="...", theta="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.oblique_shock_m2")
engpy.helpers.equation_targets_text("compressible.oblique_shock_m2")
engpy.helpers.equation_variables_table("compressible.oblique_shock_m2")
engpy.helpers.equation_target_count("compressible.oblique_shock_m2")
```

### Excel
```excel
=ENG_COMPRESSIBLE_OBLIQUE_SHOCK_M2_BETA("...","...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_OBLIQUE_SHOCK_M2_BETA("...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.oblique_shock_m2")
=ENG_EQUATION_TARGETS_TEXT("compressible.oblique_shock_m2")
=ENG_EQUATION_VARIABLES_TABLE("compressible.oblique_shock_m2")
=ENG_EQUATION_TARGET_COUNT("compressible.oblique_shock_m2")
```

**Excel arguments**
- `m2`: Downstream Mach number
- `mn2`: Downstream normal Mach
- `theta`: Flow deflection angle

