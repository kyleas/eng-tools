# Prandtl-Meyer Expansion Angle

**Path ID:** `compressible.prandtl_meyer`

$$
\nu = \sqrt{\frac{\gamma+1}{\gamma-1}} \tan^{-1}\!\left(\sqrt{\frac{\gamma-1}{\gamma+1}(M^2-1)}\right) - \tan^{-1}\!\left(\sqrt{M^2-1}\right)
$$

- Unicode: `nu = sqrt((gamma+1)/(gamma-1))*atan(sqrt(((gamma-1)/(gamma+1))*(M^2-1))) - atan(sqrt(M^2-1))`
- ASCII: `nu = sqrt((gamma+1)/(gamma-1))*atan(sqrt(((gamma-1)/(gamma+1))*(M^2-1))) - atan(sqrt(M^2-1))`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>nu</code></td><td>Prandtl-Meyer angle</td><td>\(\nu\)</td><td><code>angle</code></td><td><code>rad</code></td></tr>
<tr><td><code>M</code></td><td>Mach number</td><td>\(M\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
<tr><td><code>gamma</code></td><td>Specific heat ratio</td><td>\(\gamma\)</td><td><code>dimensionless</code></td><td><code>1</code></td></tr>
</tbody></table>

## Assumptions

- Calorically perfect gas with constant gamma.
- Defined for M >= 1 in supersonic expansion flow.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(compressible::prandtl_meyer::equation())
    .target_nu()
    .given_m(1.0)
    .given_gamma(1.4)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(compressible::prandtl_meyer::equation())
    .target_nu()
    .given_m(1.0)
    .given_gamma(1.4)
    .value()?;
```

### convenience_nu

```rust
let value = equations::compressible::prandtl_meyer::solve_nu(
    1.0,
    1.4,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::prandtl_meyer::equation()).for_target("M").value()?;
```

### Python
```python
engpy.equations.compressible.prandtl_meyer.solve_m(nu="...", gamma="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.prandtl_meyer.solve_m(nu="...", gamma="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.prandtl_meyer")
engpy.helpers.equation_targets_text("compressible.prandtl_meyer")
engpy.helpers.equation_variables_table("compressible.prandtl_meyer")
engpy.helpers.equation_target_count("compressible.prandtl_meyer")
```

### Excel
```excel
=ENG_COMPRESSIBLE_PRANDTL_MEYER_M("...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_PRANDTL_MEYER_M("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.prandtl_meyer")
=ENG_EQUATION_TARGETS_TEXT("compressible.prandtl_meyer")
=ENG_EQUATION_VARIABLES_TABLE("compressible.prandtl_meyer")
=ENG_EQUATION_TARGET_COUNT("compressible.prandtl_meyer")
```

**Excel arguments**
- `nu`: Prandtl-Meyer angle
- `gamma`: Specific heat ratio

