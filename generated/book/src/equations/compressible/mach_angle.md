# Mach Angle

**Path ID:** `compressible.mach_angle`

$$
\mu = \arcsin\left(\frac{1}{M}\right)
$$

- Unicode: `mu = asin(1/M)`
- ASCII: `mu = asin(1/M)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>mu</code></td><td>Mach angle</td><td>\(\mu\)</td><td><code>angle</code></td><td><code>rad</code></td></tr>
<tr><td><code>M</code></td><td>Mach number</td><td>\(M\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
</tbody></table>

## Assumptions

- Calorically perfect gas, isentropic context.
- Mach angle is defined for M >= 1.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(compressible::mach_angle::equation())
    .target_mu()
    .given_m(1.1)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(compressible::mach_angle::equation())
    .target_mu()
    .given_m(1.1)
    .value()?;
```

### convenience_mu

```rust
let value = equations::compressible::mach_angle::solve_mu(
    1.1,
)?;
```

### convenience_m

```rust
let value = equations::compressible::mach_angle::solve_m(
    1.1410966606,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::mach_angle::equation()).for_target("M").value()?;
```

### Python
```python
engpy.equations.compressible.mach_angle.solve_m(mu="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.mach_angle.solve_m(mu="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.mach_angle")
engpy.helpers.equation_targets_text("compressible.mach_angle")
engpy.helpers.equation_variables_table("compressible.mach_angle")
engpy.helpers.equation_target_count("compressible.mach_angle")
```

### Excel
```excel
=ENG_COMPRESSIBLE_MACH_ANGLE_M("...")
=ENG_FORMAT(ENG_COMPRESSIBLE_MACH_ANGLE_M("..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.mach_angle")
=ENG_EQUATION_TARGETS_TEXT("compressible.mach_angle")
=ENG_EQUATION_VARIABLES_TABLE("compressible.mach_angle")
=ENG_EQUATION_TARGET_COUNT("compressible.mach_angle")
```

**Excel arguments**
- `mu`: Mach angle

