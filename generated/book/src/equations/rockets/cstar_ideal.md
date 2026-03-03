# Ideal Characteristic Velocity

**Path:** `rockets.cstar_ideal`  
**Category:** `rockets`

## Equation

$$
c^* = \sqrt{\frac{R T_c}{\gamma}} \left(\frac{\gamma+1}{2}\right)^{(\gamma+1)/(2(\gamma-1))}
$$

- Unicode: `c_star = √(R · T_c / γ) · pow((γ + 1) / 2, (γ + 1) / (2 · (γ - 1)))`
- ASCII: `c_star = sqrt(R * T_c / gamma) * pow((gamma + 1) / 2, (gamma + 1) / (2 * (gamma - 1)))`

## Assumptions

- Ideal nozzle flow with calorically perfect combustion products.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>c_star</code></td><td>Characteristic velocity</td><td><span class="math inline">\(c^*\)</span></td><td><code>velocity</code></td><td><code>m/s</code></td><td><code>-</code></td></tr>
    <tr><td><code>R</code></td><td>Gas constant</td><td><span class="math inline">\(R\)</span></td><td><code>gas_constant</code></td><td><code>J/(kg*K)</code></td><td><code>-</code></td></tr>
    <tr><td><code>T_c</code></td><td>Chamber temperature</td><td><span class="math inline">\(T_{c}\)</span></td><td><code>temperature</code></td><td><code>K</code></td><td><code>-</code></td></tr>
    <tr><td><code>gamma</code></td><td>Specific heat ratio</td><td><span class="math inline">\(\gamma\)</span></td><td><code>dimensionless</code></td><td><code>1</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `c_star`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(rockets::cstar_ideal::equation())
    .target_c_star()
    .given_r(355.0)
    .given_t_c(3500.0)
    .given_gamma(1.22)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(rockets::cstar_ideal::equation())
    .target_c_star()
    .given_r("355 J/(kg*K)")
    .given_t_c("3500 K")
    .given_gamma(1.22)
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>c_star</code></td><td><code>solve_c_star(R, T_c, gamma)</code></td><td><code>R</code>, <code>T_c</code>, <code>gamma</code></td></tr>
  </tbody>
</table>

### Solve `c_star`

**Function signature**

```rust
equations::rockets::cstar_ideal::solve_c_star(R, T_c, gamma) -> Result<f64, _>
```

**Example**

```rust
let value = equations::rockets::cstar_ideal::solve_c_star(
    "355 J/(kg*K)",
    "3500 K",
    1.22,
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

