# Conduction Thermal Resistance

**Path:** `heat_transfer.thermal_resistance_conduction`  
**Category:** `heat_transfer`

## Equation

$$
R_{th} = \frac{L}{k A}
$$

- Unicode: `R_th = L / (k · A)`
- ASCII: `R_th = L / (k * A)`

## Assumptions

- One-dimensional steady conduction with constant properties.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>R_th</code></td><td>Thermal resistance</td><td><span class="math inline">\(R_{th}\)</span></td><td><code>thermal_resistance</code></td><td><code>K/W</code></td><td><code>-</code></td></tr>
    <tr><td><code>L</code></td><td>Wall thickness</td><td><span class="math inline">\(L\)</span></td><td><code>length</code></td><td><code>m</code></td><td><code>-</code></td></tr>
    <tr><td><code>k</code></td><td>Thermal conductivity</td><td><span class="math inline">\(k\)</span></td><td><code>thermal_conductivity</code></td><td><code>W/(m*K)</code></td><td><code>-</code></td></tr>
    <tr><td><code>A</code></td><td>Area</td><td><span class="math inline">\(A\)</span></td><td><code>area</code></td><td><code>m2</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `A`: explicit
- `L`: explicit
- `R_th`: explicit
- `k`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(heat_transfer::thermal_resistance_conduction::equation())
    .target_r_th()
    .given_l(0.1)
    .given_k(10.0)
    .given_a(0.2)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(heat_transfer::thermal_resistance_conduction::equation())
    .target_r_th()
    .given_l("0.1 m")
    .given_k("10 W/(m*K)")
    .given_a("0.2 m2")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>R_th</code></td><td><code>solve_r_th(L, k, A)</code></td><td><code>L</code>, <code>k</code>, <code>A</code></td></tr>
    <tr><td><code>L</code></td><td><code>solve_l(R_th, k, A)</code></td><td><code>R_th</code>, <code>k</code>, <code>A</code></td></tr>
    <tr><td><code>k</code></td><td><code>solve_k(R_th, L, A)</code></td><td><code>R_th</code>, <code>L</code>, <code>A</code></td></tr>
    <tr><td><code>A</code></td><td><code>solve_a(R_th, L, k)</code></td><td><code>R_th</code>, <code>L</code>, <code>k</code></td></tr>
  </tbody>
</table>

### Solve `R_th`

**Function signature**

```rust
equations::heat_transfer::thermal_resistance_conduction::solve_r_th(L, k, A) -> Result<f64, _>
```

**Example**

```rust
let value = equations::heat_transfer::thermal_resistance_conduction::solve_r_th(
    "0.1 m",
    "10 W/(m*K)",
    "0.2 m2",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

## Source

- Incropera et al., Fundamentals of Heat and Mass Transfer

