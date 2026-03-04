# Isentropic Area-Mach Relation

**Path ID:** `compressible.area_mach`

\[
\frac{A}{A^*} = \frac{1}{M}\left(\frac{2}{\gamma+1}\left(1+\frac{\gamma-1}{2}M^2\right)\right)^{\frac{\gamma+1}{2(\gamma-1)}}
\]

- Unicode: `A/A* = (1/M) * ((2/(gamma+1))*(1+((gamma-1)/2)M^2))^((gamma+1)/(2(gamma-1)))`
- ASCII: `area_ratio = (1/M) * ((2/(gamma+1))*(1+((gamma-1)/2)*M^2))^((gamma+1)/(2*(gamma-1)))`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>area_ratio</code></td><td>Area ratio</td><td>\(\frac{A}{A^*}\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>M</code></td><td>Mach number</td><td>\(M\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
<tr><td><code>gamma</code></td><td>Specific heat ratio</td><td>\(\gamma\)</td><td><code>dimensionless</code></td><td><code>1</code></td></tr>
</tbody></table>

## Assumptions

- Quasi-one-dimensional isentropic flow.
- Calorically perfect gas with constant gamma.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(compressible::area_mach::equation())
    .target_area_ratio()
    .branch_subsonic()
    .given_m(0.3)
    .given_gamma(1.4)
    .value()?;
```

### convenience_area_ratio

```rust
let value = equations::compressible::area_mach::solve_area_ratio(
    0.3,
    1.4,
)?;
```

### typed_builder_branch

```rust
let value = eq
    .solve(compressible::area_mach::equation())
    .target_area_ratio()
    .branch_supersonic()
    .given_m(2.2)
    .given_gamma(1.4)
    .value()?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::area_mach::equation()).for_target("M").value()?;
```

### Python
```python
engpy.equations.compressible.solve_m(area_ratio="...", gamma="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.solve_m(area_ratio="...", gamma="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.area_mach")
```

### Excel
```excel
=ENG_COMPRESSIBLE_AREA_MACH_M("...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_AREA_MACH_M("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.area_mach")
```

**Excel arguments**
- `area_ratio`: Area ratio
- `gamma`: Specific heat ratio

