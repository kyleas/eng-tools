# Thin-Wall Longitudinal Stress

**Path ID:** `structures.longitudinal_stress_thin_wall`

\[
\sigma_l = \frac{P r}{2 t}
\]

- Unicode: `σ_l = P · r / (2 · t)`
- ASCII: `sigma_l = P * r / (2 * t)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>sigma_l</code></td><td>Longitudinal stress</td><td>\(\sigma_l\)</td><td><code>stress</code></td><td><code>Pa</code></td></tr>
<tr><td><code>P</code></td><td>Internal pressure</td><td>\(P\)</td><td><code>pressure</code></td><td><code>Pa</code></td></tr>
<tr><td><code>r</code></td><td>Mean radius</td><td>\(r\)</td><td><code>length</code></td><td><code>m</code></td></tr>
<tr><td><code>t</code></td><td>Wall thickness</td><td>\(t\)</td><td><code>length</code></td><td><code>m</code></td></tr>
</tbody></table>

## Assumptions

- Thin-wall cylindrical vessel behavior.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(structures::longitudinal_stress_thin_wall::equation())
    .target_sigma_l()
    .given_p(2.5e6)
    .given_r(0.2)
    .given_t(0.008)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(structures::longitudinal_stress_thin_wall::equation())
    .target_sigma_l()
    .given_p("2.5 MPa")
    .given_r("0.2 m")
    .given_t("8 mm")
    .value()?;
```

### convenience_sigma_l

```rust
let value = equations::structures::longitudinal_stress_thin_wall::solve_sigma_l(
    "2.5 MPa",
    "0.2 m",
    "8 mm",
)?;
```

### convenience_p

```rust
let value = equations::structures::longitudinal_stress_thin_wall::solve_p(
    "31.25 MPa",
    "0.2 m",
    "8 mm",
)?;
```

### convenience_r

```rust
let value = equations::structures::longitudinal_stress_thin_wall::solve_r(
    "31.25 MPa",
    "2.5 MPa",
    "8 mm",
)?;
```

### convenience_t

```rust
let value = equations::structures::longitudinal_stress_thin_wall::solve_t(
    "31.25 MPa",
    "2.5 MPa",
    "0.2 m",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::structures::longitudinal_stress_thin_wall::equation()).for_target("P").value()?;
```

### Python
```python
engpy.equations.structures.solve_p(sigma_l="...", r="...", t="...")
# helper layer
engpy.helpers.format_value(engpy.equations.structures.solve_p(sigma_l="...", r="...", t="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("structures.longitudinal_stress_thin_wall")
```

### Excel
```excel
=ENG_STRUCTURES_LONGITUDINAL_STRESS_THIN_WALL_P("...","...","...")
=ENG_FORMAT(ENG_STRUCTURES_LONGITUDINAL_STRESS_THIN_WALL_P("...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("structures.longitudinal_stress_thin_wall")
```

**Excel arguments**
- `sigma_l`: Longitudinal stress
- `r`: Mean radius
- `t`: Wall thickness

