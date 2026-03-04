# Beam Bending Stress

**Path ID:** `structures.beam_bending_stress`

$$
\sigma_b = \frac{M c}{I}
$$

- Unicode: `σ_b = M · c / I`
- ASCII: `sigma_b = M * c / I`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>sigma_b</code></td><td>Bending stress</td><td>\(\sigma_b\)</td><td><code>stress</code></td><td><code>Pa</code></td></tr>
<tr><td><code>M</code></td><td>Bending moment</td><td>\(M\)</td><td><code>moment</code></td><td><code>N*m</code></td></tr>
<tr><td><code>c</code></td><td>Distance to outer fiber</td><td>\(c\)</td><td><code>length</code></td><td><code>m</code></td></tr>
<tr><td><code>I</code></td><td>Area moment of inertia</td><td>\(I\)</td><td><code>area_moment_of_inertia</code></td><td><code>m4</code></td></tr>
</tbody></table>

## Assumptions

- Linear elastic beam bending with small strains.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(structures::beam_bending_stress::equation())
    .target_sigma_b()
    .given_m(5000.0)
    .given_c(0.05)
    .given_i(5e-6)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(structures::beam_bending_stress::equation())
    .target_sigma_b()
    .given_m("5000 N*m")
    .given_c("0.05 m")
    .given_i("5e-6 m4")
    .value()?;
```

### convenience_sigma_b

```rust
let value = equations::structures::beam_bending_stress::solve_sigma_b(
    "5000 N*m",
    "0.05 m",
    "5e-6 m4",
)?;
```

### convenience_m

```rust
let value = equations::structures::beam_bending_stress::solve_m(
    "50 MPa",
    "0.05 m",
    "5e-6 m4",
)?;
```

### convenience_c

```rust
let value = equations::structures::beam_bending_stress::solve_c(
    "50 MPa",
    "5000 N*m",
    "5e-6 m4",
)?;
```

### convenience_i

```rust
let value = equations::structures::beam_bending_stress::solve_i(
    "50 MPa",
    "5000 N*m",
    "0.05 m",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::structures::beam_bending_stress::equation()).for_target("I").value()?;
```

### Python
```python
engpy.equations.structures.beam_bending_stress.solve_i(sigma_b="...", m="...", c="...")
# helper layer
engpy.helpers.format_value(engpy.equations.structures.beam_bending_stress.solve_i(sigma_b="...", m="...", c="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("structures.beam_bending_stress")
engpy.helpers.equation_targets_text("structures.beam_bending_stress")
engpy.helpers.equation_variables_table("structures.beam_bending_stress")
engpy.helpers.equation_target_count("structures.beam_bending_stress")
```

### Excel
```excel
=ENG_STRUCTURES_BEAM_BENDING_STRESS_I("...","...","...")
=ENG_FORMAT(ENG_STRUCTURES_BEAM_BENDING_STRESS_I("...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("structures.beam_bending_stress")
=ENG_EQUATION_TARGETS_TEXT("structures.beam_bending_stress")
=ENG_EQUATION_VARIABLES_TABLE("structures.beam_bending_stress")
=ENG_EQUATION_TARGET_COUNT("structures.beam_bending_stress")
```

**Excel arguments**
- `sigma_b`: Bending stress
- `m`: Bending moment
- `c`: Distance to outer fiber

