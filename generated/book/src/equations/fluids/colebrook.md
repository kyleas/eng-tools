# Colebrook-White Friction Factor

**Path ID:** `fluids.colebrook`

$$
\frac{1}{\sqrt{f}} + 2\log_{10}\left(\frac{\varepsilon_D}{3.7} + \frac{2.51}{Re\sqrt{f}}\right) = 0
$$

- Unicode: `1/sqrt(f) + 2 log10(eps_D/3.7 + 2.51/(Re sqrt(f))) = 0`
- ASCII: `1/sqrt(f) + 2*log10((eps_D/3.7) + (2.51/(Re*sqrt(f)))) = 0`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>f</code></td><td>Darcy friction factor</td><td>\(f\)</td><td><code>friction_factor</code></td><td><code>1</code></td></tr>
<tr><td><code>eps_D</code></td><td>Relative roughness</td><td>\(\epsilon_D\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>Re</code></td><td>Reynolds number</td><td>\(Re\)</td><td><code>dimensionless</code></td><td><code>1</code></td></tr>
</tbody></table>

## Assumptions

- Fully developed turbulent internal flow.
- Incompressible flow behavior.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(fluids::colebrook::equation())
    .target_f()
    .given_eps_d(4.214317085279759e-4)
    .given_re(1e5)
    .value()?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::fluids::colebrook::equation()).for_target("f").value()?;
```

### Python
```python
engpy.equations.fluids.colebrook.solve_f(eps_d="...", re="...")
# helper layer
engpy.helpers.format_value(engpy.equations.fluids.colebrook.solve_f(eps_d="...", re="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("fluids.colebrook")
engpy.helpers.equation_targets_text("fluids.colebrook")
engpy.helpers.equation_variables_table("fluids.colebrook")
engpy.helpers.equation_target_count("fluids.colebrook")
```

### Excel
```excel
=ENG_FLUIDS_COLEBROOK_F("...","...")
=ENG_FORMAT(ENG_FLUIDS_COLEBROOK_F("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("fluids.colebrook")
=ENG_EQUATION_TARGETS_TEXT("fluids.colebrook")
=ENG_EQUATION_VARIABLES_TABLE("fluids.colebrook")
=ENG_EQUATION_TARGET_COUNT("fluids.colebrook")
```

**Excel arguments**
- `eps_d`: Relative roughness
- `re`: Reynolds number

