# Convection Thermal Resistance

**Path:** `heat_transfer.thermal_resistance_convection`  
**Category:** `heat_transfer`

## Equation

$$
R_{th} = \frac{1}{h A}
$$

- Unicode: `R_th = 1 / (h · A)`
- ASCII: `R_th = 1 / (h * A)`

## Assumptions

- Uniform convection coefficient over the area.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>R_th</code></td><td>Thermal resistance</td><td><span class="math inline">\(R_{th}\)</span></td><td><code>thermal_resistance</code></td><td><code>K/W</code></td><td><code>-</code></td></tr>
    <tr><td><code>h</code></td><td>Convective heat transfer coefficient</td><td><span class="math inline">\(h\)</span></td><td><code>heat_transfer_coefficient</code></td><td><code>W/(m2*K)</code></td><td><code>-</code></td></tr>
    <tr><td><code>A</code></td><td>Surface area</td><td><span class="math inline">\(A\)</span></td><td><code>area</code></td><td><code>m2</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `A`: explicit
- `R_th`: explicit
- `h`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(heat_transfer::thermal_resistance_convection::equation())
    .target_r_th()
    .given_h(35.0)
    .given_a(3.0)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(heat_transfer::thermal_resistance_convection::equation())
    .target_r_th()
    .given_h("35 W/(m2*K)")
    .given_a("3 m2")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>R_th</code></td><td><code>solve_r_th(h, A)</code></td><td><code>h</code>, <code>A</code></td></tr>
    <tr><td><code>h</code></td><td><code>solve_h(R_th, A)</code></td><td><code>R_th</code>, <code>A</code></td></tr>
    <tr><td><code>A</code></td><td><code>solve_a(R_th, h)</code></td><td><code>R_th</code>, <code>h</code></td></tr>
  </tbody>
</table>

### Solve `R_th`

**Function signature**

```rust
equations::heat_transfer::thermal_resistance_convection::solve_r_th(h, A) -> Result<f64, _>
```

**Example**

```rust
let value = equations::heat_transfer::thermal_resistance_convection::solve_r_th(
    "35 W/(m2*K)",
    "3 m2",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

