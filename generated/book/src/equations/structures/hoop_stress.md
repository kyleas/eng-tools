# Thin-Wall Hoop Stress

**Path:** `structures.hoop_stress`  
**Category:** `structures`

## Equation

$$
\sigma_h = \frac{P r}{t}
$$

- Unicode: `σ_h = P · r / t`
- ASCII: `sigma_h = P * r / t`

## Assumptions

- Thin-wall approximation is valid (t << r).
- Material behavior remains in the elastic membrane-stress regime.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>sigma_h</code></td><td>Hoop stress</td><td><span class="math inline">\(\sigma_h\)</span></td><td><code>stress</code></td><td><code>Pa</code></td><td><code>-</code></td></tr>
    <tr><td><code>P</code></td><td>Internal pressure</td><td><span class="math inline">\(P\)</span></td><td><code>pressure</code></td><td><code>Pa</code></td><td><code>-</code></td></tr>
    <tr><td><code>r</code></td><td>Mean radius</td><td><span class="math inline">\(r\)</span></td><td><code>length</code></td><td><code>m</code></td><td><code>-</code></td></tr>
    <tr><td><code>t</code></td><td>Wall thickness</td><td><span class="math inline">\(t\)</span></td><td><code>length</code></td><td><code>m</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `P`: explicit, numerical
- `r`: explicit, numerical
- `sigma_h`: explicit, numerical
- `t`: explicit, numerical

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p(2.5e6)
    .given_r(0.2)
    .given_t(0.008)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p("2.5 MPa")
    .given_r("0.2 m")
    .given_t("8 mm")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>sigma_h</code></td><td><code>solve_sigma_h(P, r, t)</code></td><td><code>P</code>, <code>r</code>, <code>t</code></td></tr>
    <tr><td><code>P</code></td><td><code>solve_p(sigma_h, r, t)</code></td><td><code>sigma_h</code>, <code>r</code>, <code>t</code></td></tr>
    <tr><td><code>r</code></td><td><code>solve_r(sigma_h, P, t)</code></td><td><code>sigma_h</code>, <code>P</code>, <code>t</code></td></tr>
    <tr><td><code>t</code></td><td><code>solve_t(sigma_h, P, r)</code></td><td><code>sigma_h</code>, <code>P</code>, <code>r</code></td></tr>
  </tbody>
</table>

### Solve `sigma_h`

**Function signature**

```rust
equations::structures::hoop_stress::solve_sigma_h(P, r, t) -> Result<f64, _>
```

**Example**

```rust
let value = equations::structures::hoop_stress::solve_sigma_h(
    "2.5 MPa",
    "0.2 m",
    "8 mm",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

## Source

- [Roark's Formulas for Stress and Strain](https://www.mheducation.com/highered/product/roark-s-formulas-stress-strain-young-budynas/M9780071742475.html)

## References

- Roark's Formulas for Stress and Strain — Thin-walled cylindrical pressure vessel relations.

## Aliases

`thin_wall_hoop_stress`
