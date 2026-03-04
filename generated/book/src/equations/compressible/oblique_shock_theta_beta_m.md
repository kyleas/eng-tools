# Oblique Shock Theta-Beta-M Relation

**Path ID:** `compressible.oblique_shock_theta_beta_m`

$$
\tan\theta = \frac{2\cot\beta\left(M_1^2\sin^2\beta - 1\right)}{M_1^2\left(\gamma + \cos 2\beta\right)+2}
$$

- Unicode: `tan(theta) = (2*cot(beta)*(M1^2*sin(beta)^2 - 1)) / (M1^2*(gamma + cos(2*beta)) + 2)`
- ASCII: `tan(theta) = (2*cot(beta)*(m1^2*sin(beta)^2 - 1)) / (m1^2*(gamma + cos(2*beta)) + 2)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>theta</code></td><td>Flow deflection angle</td><td>\(\theta\)</td><td><code>angle</code></td><td><code>rad</code></td></tr>
<tr><td><code>beta</code></td><td>Shock angle</td><td>\(\beta\)</td><td><code>angle</code></td><td><code>rad</code></td></tr>
<tr><td><code>m1</code></td><td>Upstream Mach number</td><td>\(M_1\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
<tr><td><code>gamma</code></td><td>Specific heat ratio</td><td>\(\gamma\)</td><td><code>dimensionless</code></td><td><code>1</code></td></tr>
</tbody></table>

## Assumptions

- Attached oblique shock, inviscid perfect-gas flow.
- Upstream Mach number is supersonic (M1 > 1).

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(compressible::oblique_shock_theta_beta_m::equation())
    .target_theta()
    .given_beta(0.6981317008)
    .given_m1(2.0)
    .given_gamma(1.4)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(compressible::oblique_shock_theta_beta_m::equation())
    .target_theta()
    .given_beta(0.6981317008)
    .given_m1(2.0)
    .given_gamma(1.4)
    .value()?;
```

### convenience_theta

```rust
let value = equations::compressible::oblique_shock_theta_beta_m::solve_theta(
    0.6981317008,
    2.0,
    1.4,
)?;
```

### typed_builder_branch

```rust
let value = eq
    .solve(compressible::oblique_shock_theta_beta_m::equation())
    .target_theta()
    .branch_strong()
    .given_beta(1.4608419868)
    .given_m1(2.0)
    .given_gamma(1.4)
    .value()?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::oblique_shock_theta_beta_m::equation()).for_target("beta").value()?;
```

### Python
```python
engpy.equations.compressible.oblique_shock_theta_beta_m.solve_beta(theta="...", m1="...", gamma="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.oblique_shock_theta_beta_m.solve_beta(theta="...", m1="...", gamma="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.oblique_shock_theta_beta_m")
engpy.helpers.equation_targets_text("compressible.oblique_shock_theta_beta_m")
engpy.helpers.equation_variables_table("compressible.oblique_shock_theta_beta_m")
engpy.helpers.equation_target_count("compressible.oblique_shock_theta_beta_m")
```

### Excel
```excel
=ENG_COMPRESSIBLE_OBLIQUE_SHOCK_THETA_BETA_M_BETA("...","...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_OBLIQUE_SHOCK_THETA_BETA_M_BETA("...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.oblique_shock_theta_beta_m")
=ENG_EQUATION_TARGETS_TEXT("compressible.oblique_shock_theta_beta_m")
=ENG_EQUATION_VARIABLES_TABLE("compressible.oblique_shock_theta_beta_m")
=ENG_EQUATION_TARGET_COUNT("compressible.oblique_shock_theta_beta_m")
```

**Excel arguments**
- `theta`: Flow deflection angle
- `m1`: Upstream Mach number
- `gamma`: Specific heat ratio


**Branch behavior**
- Default solver behavior uses preferred branch (`weak`) when one is marked.
- Supported branches: `weak`, `strong`

### Python (explicit branch)
```python
engpy.equations.compressible.oblique_shock_theta_beta_m.solve_beta(theta="...", m1="...", gamma="...", branch="weak")
```

### Excel (explicit branch)
```excel
=ENG_COMPRESSIBLE_OBLIQUE_SHOCK_THETA_BETA_M_BETA("...","...","...","weak")
=ENG_EQUATION_BRANCHES_TEXT("compressible.oblique_shock_theta_beta_m")
=ENG_EQUATION_BRANCHES_TABLE("compressible.oblique_shock_theta_beta_m")
```
