# Ideal Gas Law (Density Form)

**Path:** `thermo.ideal_gas.density`  
**Category:** `thermo`

## Equation

$$
P = \rho R T
$$

- Unicode: `P = ρ · R · T`
- ASCII: `P = rho * R * T`

## Assumptions

- Thermally and calorically ideal gas behavior.
- Single-phase gas state.

## Family

- Family: [`Ideal Gas Law`](../../families/ideal_gas.md)
- Variant: `Density Form` (`density`)
- Use when: Use when density-based flow/property calculations are primary.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>P</code></td><td>Absolute pressure</td><td><span class="math inline">\(P\)</span></td><td><code>pressure</code></td><td><code>Pa</code></td><td><code>-</code></td></tr>
    <tr><td><code>rho</code></td><td>Density</td><td><span class="math inline">\(\rho\)</span></td><td><code>density</code></td><td><code>kg/m3</code></td><td><code>-</code></td></tr>
    <tr><td><code>R</code></td><td>Specific gas constant</td><td><span class="math inline">\(R\)</span></td><td><code>gas_constant</code></td><td><code>J/(kg*K)</code></td><td><code>-</code></td></tr>
    <tr><td><code>T</code></td><td>Absolute temperature</td><td><span class="math inline">\(T\)</span></td><td><code>temperature</code></td><td><code>K</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `P`: explicit, numerical
- `R`: explicit, numerical
- `T`: explicit, numerical
- `rho`: explicit, numerical

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(thermo::density::equation())
    .target_p()
    .given_rho(1.17683)
    .given_r(287.0)
    .given_t(300.0)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(thermo::density::equation())
    .target_p()
    .given_rho("1.17683 kg/m3")
    .given_r("287 J/(kg*K)")
    .given_t("300 K")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>P</code></td><td><code>solve_p(rho, R, T)</code></td><td><code>rho</code>, <code>R</code>, <code>T</code></td></tr>
    <tr><td><code>rho</code></td><td><code>solve_rho(P, R, T)</code></td><td><code>P</code>, <code>R</code>, <code>T</code></td></tr>
    <tr><td><code>R</code></td><td><code>solve_r(P, rho, T)</code></td><td><code>P</code>, <code>rho</code>, <code>T</code></td></tr>
    <tr><td><code>T</code></td><td><code>solve_t(P, rho, R)</code></td><td><code>P</code>, <code>rho</code>, <code>R</code></td></tr>
  </tbody>
</table>

### Solve `P`

**Function signature**

```rust
equations::thermo::density::solve_p(rho, R, T) -> Result<f64, _>
```

**Example**

```rust
let value = equations::thermo::density::solve_p(
    "1.17683 kg/m3",
    "287 J/(kg*K)",
    "300 K",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

## Source

- Moran et al., Fundamentals of Engineering Thermodynamics

