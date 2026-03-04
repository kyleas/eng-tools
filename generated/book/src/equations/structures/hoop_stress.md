# Thin-Wall Hoop Stress

**Path ID:** `structures.hoop_stress`

\[
\sigma_h = \frac{P r}{t}
\]

- Unicode: `σ_h = P · r / t`
- ASCII: `sigma_h = P * r / t`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>sigma_h</code></td><td>Hoop stress</td><td>\(\sigma_h\)</td><td><code>stress</code></td><td><code>Pa</code></td></tr>
<tr><td><code>P</code></td><td>Internal pressure</td><td>\(P\)</td><td><code>pressure</code></td><td><code>Pa</code></td></tr>
<tr><td><code>r</code></td><td>Mean radius</td><td>\(r\)</td><td><code>length</code></td><td><code>m</code></td></tr>
<tr><td><code>t</code></td><td>Wall thickness</td><td>\(t\)</td><td><code>length</code></td><td><code>m</code></td></tr>
</tbody></table>

## Assumptions

- Thin-wall approximation is valid (t << r).
- Material behavior remains in the elastic membrane-stress regime.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p(2.5e6)
    .given_r(0.2)
    .given_t(0.008)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p("2.5 MPa")
    .given_r("0.2 m")
    .given_t("8 mm")
    .value()?;
```

### convenience_sigma_h

```rust
let value = equations::structures::hoop_stress::solve_sigma_h(
    "2.5 MPa",
    "0.2 m",
    "8 mm",
)?;
```

### convenience_p

```rust
let value = equations::structures::hoop_stress::solve_p(
    "62.5 MPa",
    "0.2 m",
    "8 mm",
)?;
```

### convenience_r

```rust
let value = equations::structures::hoop_stress::solve_r(
    "62.5 MPa",
    "2.5 MPa",
    "8 mm",
)?;
```

### convenience_t

```rust
let value = equations::structures::hoop_stress::solve_t(
    "62.5 MPa",
    "2.5 MPa",
    "0.2 m",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::structures::hoop_stress::equation()).for_target("P").value()?;
```

### Python
```python
engpy.equations.structures.solve_p(sigma_h="...", r="...", t="...")
# helper layer
engpy.helpers.format_value(engpy.equations.structures.solve_p(sigma_h="...", r="...", t="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("structures.hoop_stress")
```

### Excel
```excel
=ENG_STRUCTURES_HOOP_STRESS_P("...","...","...")
=ENG_FORMAT(ENG_STRUCTURES_HOOP_STRESS_P("...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("structures.hoop_stress")
```

**Excel arguments**
- `sigma_h`: Hoop stress
- `r`: Mean radius
- `t`: Wall thickness

