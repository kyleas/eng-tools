# Fanno Friction Length Parameter

**Path ID:** `compressible.fanno_friction_parameter`

$$
\frac{4 f L^*}{D} = \frac{1-M^2}{\gamma M^2} + \frac{\gamma+1}{2\gamma}\ln\!\left(\frac{\frac{\gamma+1}{2}M^2}{1+\frac{\gamma-1}{2}M^2}\right)
$$

- Unicode: `4fL*/D = (1-M^2)/(gamma*M^2) + ((gamma+1)/(2*gamma))*ln((((gamma+1)/2)*M^2)/(1+((gamma-1)/2)*M^2))`
- ASCII: `four_flstar_d = (1-M^2)/(gamma*M^2) + ((gamma+1)/(2*gamma))*ln((((gamma+1)/2)*M^2)/(1+((gamma-1)/2)*M^2))`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>four_flstar_d</code></td><td>Fanno friction length parameter</td><td>\(\frac{4 f L^*}{D}\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>M</code></td><td>Mach number</td><td>\(M\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
<tr><td><code>gamma</code></td><td>Specific heat ratio</td><td>\(\gamma\)</td><td><code>dimensionless</code></td><td><code>1</code></td></tr>
</tbody></table>

## Assumptions

- One-dimensional, steady, adiabatic, constant-area duct flow with wall friction.
- Calorically perfect gas with constant gamma.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(compressible::fanno_friction_parameter::equation())
    .target_four_flstar_d()
    .branch_subsonic()
    .given_m(0.5)
    .given_gamma(1.4)
    .value()?;
```

### convenience_four_flstar_d

```rust
let value = equations::compressible::fanno_friction_parameter::solve_four_flstar_d(
    0.5,
    1.4,
)?;
```

### typed_builder_branch

```rust
let value = eq
    .solve(compressible::fanno_friction_parameter::equation())
    .target_four_flstar_d()
    .branch_supersonic()
    .given_m(2.0)
    .given_gamma(1.4)
    .value()?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::fanno_friction_parameter::equation()).for_target("M").value()?;
```

### Python
```python
engpy.equations.compressible.fanno_friction_parameter.solve_m(four_flstar_d="...", gamma="...")
# helper layer
engpy.helpers.format_value(engpy.equations.compressible.fanno_friction_parameter.solve_m(four_flstar_d="...", gamma="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("compressible.fanno_friction_parameter")
engpy.helpers.equation_targets_text("compressible.fanno_friction_parameter")
engpy.helpers.equation_variables_table("compressible.fanno_friction_parameter")
engpy.helpers.equation_target_count("compressible.fanno_friction_parameter")
```

### Excel
```excel
=ENG_COMPRESSIBLE_FANNO_FRICTION_PARAMETER_M("...","...")
=ENG_FORMAT(ENG_COMPRESSIBLE_FANNO_FRICTION_PARAMETER_M("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("compressible.fanno_friction_parameter")
=ENG_EQUATION_TARGETS_TEXT("compressible.fanno_friction_parameter")
=ENG_EQUATION_VARIABLES_TABLE("compressible.fanno_friction_parameter")
=ENG_EQUATION_TARGET_COUNT("compressible.fanno_friction_parameter")
```

**Excel arguments**
- `four_flstar_d`: Fanno friction length parameter
- `gamma`: Specific heat ratio


**Branch behavior**
- Default solver behavior uses preferred branch (`supersonic`) when one is marked.
- Supported branches: `subsonic`, `supersonic`

### Python (explicit branch)
```python
engpy.equations.compressible.fanno_friction_parameter.solve_m(four_flstar_d="...", gamma="...", branch="supersonic")
```

### Excel (explicit branch)
```excel
=ENG_COMPRESSIBLE_FANNO_FRICTION_PARAMETER_M("...","...","supersonic")
=ENG_EQUATION_BRANCHES_TEXT("compressible.fanno_friction_parameter")
=ENG_EQUATION_BRANCHES_TABLE("compressible.fanno_friction_parameter")
```
