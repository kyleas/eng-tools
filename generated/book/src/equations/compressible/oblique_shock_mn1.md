# Oblique Shock Normal Upstream Mach

**Path ID:** `compressible.oblique_shock_mn1`

$$
M_{n1} = M_1 \sin\beta
$$

- Unicode: `mn1 = m1 * sin(beta)`
- ASCII: `mn1 = m1 * sin(beta)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>mn1</code></td><td>Upstream normal Mach</td><td>\(M_{n1}\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
<tr><td><code>m1</code></td><td>Upstream Mach number</td><td>\(M_1\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
<tr><td><code>beta</code></td><td>Shock angle</td><td>\(\beta\)</td><td><code>angle</code></td><td><code>rad</code></td></tr>
</tbody></table>

## Assumptions

- Attached oblique shock geometry.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(compressible::oblique_shock_mn1::equation())
    .target_mn1()
    .given_m1(2.0)
    .given_beta(0.6981317008)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(compressible::oblique_shock_mn1::equation())
    .target_mn1()
    .given_m1(2.0)
    .given_beta(0.6981317008)
    .value()?;
```

### convenience_mn1

```rust
let value = equations::compressible::oblique_shock_mn1::solve_mn1(
    2.0,
    0.6981317008,
)?;
```

### convenience_m1

```rust
let value = equations::compressible::oblique_shock_mn1::solve_m1(
    1.2855752194,
    0.6981317008,
)?;
```

### convenience_beta

```rust
let value = equations::compressible::oblique_shock_mn1::solve_beta(
    1.2855752194,
    2.0,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::oblique_shock_mn1::equation()).for_target("beta").value()?;
```

### Python
```python
engpy.equations.compressible.oblique_shock_mn1.solve_beta(mn1="...", m1="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.oblique_shock_mn1.solve_beta(mn1="...", m1="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.oblique_shock_mn1")
engpy.helpers.equation_targets_text("compressible.oblique_shock_mn1")
engpy.helpers.equation_variables_table("compressible.oblique_shock_mn1")
engpy.helpers.equation_target_count("compressible.oblique_shock_mn1")
```

### Excel
```excel
=ENG_COMPRESSIBLE_OBLIQUE_SHOCK_MN1_BETA("...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_OBLIQUE_SHOCK_MN1_BETA("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.oblique_shock_mn1")
=ENG_EQUATION_TARGETS_TEXT("compressible.oblique_shock_mn1")
=ENG_EQUATION_VARIABLES_TABLE("compressible.oblique_shock_mn1")
=ENG_EQUATION_TARGET_COUNT("compressible.oblique_shock_mn1")
```

**Excel arguments**
- `mn1`: Upstream normal Mach
- `m1`: Upstream Mach number

