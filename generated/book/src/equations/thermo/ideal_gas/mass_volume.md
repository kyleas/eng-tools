# Ideal Gas Law (Mass-Volume Form)

**Path:** `thermo.ideal_gas.mass_volume`  
**Category:** `thermo`

## Equation

$$
P V = m R T
$$

- Unicode: `P = m · R · T / V`
- ASCII: `P = m * R * T / V`

## Assumptions

- Thermally and calorically ideal gas behavior.
- Gas is in equilibrium at a single representative state.

## Family

- Family: [`Ideal Gas Law`](../../families/ideal_gas.md)
- Variant: `Mass-Volume Form` (`mass_volume`)
- Use when: Use when total mass and control-volume size are primary knowns.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>P</code></td><td>Absolute pressure</td><td><span class="math inline">\(P\)</span></td><td><code>pressure</code></td><td><code>Pa</code></td><td><code>-</code></td></tr>
    <tr><td><code>V</code></td><td>Control-volume</td><td><span class="math inline">\(V\)</span></td><td><code>volume</code></td><td><code>m3</code></td><td><code>-</code></td></tr>
    <tr><td><code>m</code></td><td>Gas mass</td><td><span class="math inline">\(m\)</span></td><td><code>mass</code></td><td><code>kg</code></td><td><code>-</code></td></tr>
    <tr><td><code>R</code></td><td>Specific gas constant</td><td><span class="math inline">\(R\)</span></td><td><code>gas_constant</code></td><td><code>J/(kg*K)</code></td><td><code>-</code></td></tr>
    <tr><td><code>T</code></td><td>Absolute temperature</td><td><span class="math inline">\(T\)</span></td><td><code>temperature</code></td><td><code>K</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `P`: explicit, numerical
- `R`: explicit, numerical
- `T`: explicit, numerical
- `V`: explicit, numerical
- `m`: explicit, numerical

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(thermo::mass_volume::equation())
    .target_p()
    .given_v(0.1)
    .given_m(0.1176829268)
    .given_r(287.0)
    .given_t(300.0)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(thermo::mass_volume::equation())
    .target_p()
    .given_v("0.1 m3")
    .given_m("0.1176829268292683 kg")
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
    <tr><td><code>P</code></td><td><code>solve_p(V, m, R, T)</code></td><td><code>V</code>, <code>m</code>, <code>R</code>, <code>T</code></td></tr>
    <tr><td><code>V</code></td><td><code>solve_v(P, m, R, T)</code></td><td><code>P</code>, <code>m</code>, <code>R</code>, <code>T</code></td></tr>
    <tr><td><code>m</code></td><td><code>solve_m(P, V, R, T)</code></td><td><code>P</code>, <code>V</code>, <code>R</code>, <code>T</code></td></tr>
    <tr><td><code>R</code></td><td><code>solve_r(P, V, m, T)</code></td><td><code>P</code>, <code>V</code>, <code>m</code>, <code>T</code></td></tr>
    <tr><td><code>T</code></td><td><code>solve_t(P, V, m, R)</code></td><td><code>P</code>, <code>V</code>, <code>m</code>, <code>R</code></td></tr>
  </tbody>
</table>

### Solve `P`

**Function signature**

```rust
equations::thermo::mass_volume::solve_p(V, m, R, T) -> Result<f64, _>
```

**Example**

```rust
let value = equations::thermo::mass_volume::solve_p(
    "0.1 m3",
    "0.1176829268292683 kg",
    "287 J/(kg*K)",
    "300 K",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

