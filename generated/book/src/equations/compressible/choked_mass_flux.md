# Choked Mass Flux

**Path ID:** `compressible.choked_mass_flux`

$$
G^* = \frac{p_0}{\sqrt{T_0}} \sqrt{\frac{\gamma}{R}} \left(\frac{2}{\gamma+1}\right)^{(\gamma+1)/(2(\gamma-1))}
$$

- Unicode: `G_star = (p0 / √(T0)) · √(γ / R) · pow(2 / (γ + 1), (γ + 1) / (2 · (γ - 1)))`
- ASCII: `G_star = (p0 / sqrt(T0)) * sqrt(gamma / R) * pow(2 / (gamma + 1), (gamma + 1) / (2 * (gamma - 1)))`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>G_star</code></td><td>Choked mass flux</td><td>\(G_star\)</td><td><code>mass_flux</code></td><td><code>kg/(m2*s)</code></td></tr>
<tr><td><code>p0</code></td><td>Stagnation pressure</td><td>\(p0\)</td><td><code>pressure</code></td><td><code>Pa</code></td></tr>
<tr><td><code>T0</code></td><td>Stagnation temperature</td><td>\(T0\)</td><td><code>temperature</code></td><td><code>K</code></td></tr>
<tr><td><code>gamma</code></td><td>Specific heat ratio</td><td>\(\gamma\)</td><td><code>dimensionless</code></td><td><code>1</code></td></tr>
<tr><td><code>R</code></td><td>Gas constant</td><td>\(R\)</td><td><code>gas_constant</code></td><td><code>J/(kg*K)</code></td></tr>
</tbody></table>

## Assumptions

- One-dimensional isentropic converging nozzle at critical condition (M=1 at throat).
- Perfect gas with constant gamma and R.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(compressible::choked_mass_flux::equation())
    .target_g_star()
    .given_p0(5e5)
    .given_t0(300.0)
    .given_gamma(1.4)
    .given_r(287.0)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(compressible::choked_mass_flux::equation())
    .target_g_star()
    .given_p0("500000 Pa")
    .given_t0("300 K")
    .given_gamma(1.4)
    .given_r("287 J/(kg*K)")
    .value()?;
```

### convenience_g_star

```rust
let value = equations::compressible::choked_mass_flux::solve_g_star(
    "500000 Pa",
    "300 K",
    1.4,
    "287 J/(kg*K)",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::choked_mass_flux::equation()).for_target("G_star").value()?;
```

### Python
```python
engpy.equations.compressible.choked_mass_flux.solve_g_star(p0="...", t0="...", gamma="...", r="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.choked_mass_flux.solve_g_star(p0="...", t0="...", gamma="...", r="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.choked_mass_flux")
engpy.helpers.equation_targets_text("compressible.choked_mass_flux")
engpy.helpers.equation_variables_table("compressible.choked_mass_flux")
engpy.helpers.equation_target_count("compressible.choked_mass_flux")
```

### Excel
```excel
=ENG_COMPRESSIBLE_CHOKED_MASS_FLUX_G_STAR("...","...","...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_CHOKED_MASS_FLUX_G_STAR("...","...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.choked_mass_flux")
=ENG_EQUATION_TARGETS_TEXT("compressible.choked_mass_flux")
=ENG_EQUATION_VARIABLES_TABLE("compressible.choked_mass_flux")
=ENG_EQUATION_TARGET_COUNT("compressible.choked_mass_flux")
```

**Excel arguments**
- `p0`: Stagnation pressure
- `t0`: Stagnation temperature
- `gamma`: Specific heat ratio
- `r`: Gas constant

