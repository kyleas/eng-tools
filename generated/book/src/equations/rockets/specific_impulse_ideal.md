# Ideal Specific Impulse

**Path:** `rockets.specific_impulse_ideal`  
**Category:** `rockets`

## Equation

$$
I_{sp} = \frac{C_f c^*}{g_0}
$$

- Unicode: `I_sp = C_f · c_star / g₀`
- ASCII: `I_sp = C_f * c_star / g0`

## Assumptions

- Vacuum-equivalent ideal performance relation.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>I_sp</code></td><td>Specific impulse</td><td><span class="math inline">\(I_{sp}\)</span></td><td><code>specific_impulse</code></td><td><code>s</code></td><td><code>-</code></td></tr>
    <tr><td><code>C_f</code></td><td>Thrust coefficient</td><td><span class="math inline">\(C_{f}\)</span></td><td><code>ratio</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>c_star</code></td><td>Characteristic velocity</td><td><span class="math inline">\(c^*\)</span></td><td><code>velocity</code></td><td><code>m/s</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `C_f`: explicit
- `I_sp`: explicit
- `c_star`: explicit

## Constants Used

<ul>
  <li><a href="../../constants/g0.md"><code>g0</code></a>: Standard Gravity - <span class="math inline">\(g_{0}\)</span></li>
</ul>

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(rockets::specific_impulse_ideal::equation())
    .target_i_sp()
    .given_c_f(1.7684408757)
    .given_c_star(1718.7683350153)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(rockets::specific_impulse_ideal::equation())
    .target_i_sp()
    .given_c_f(1.7684408757)
    .given_c_star("1718.7683350153386 m/s")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>I_sp</code></td><td><code>solve_i_sp(C_f, c_star)</code></td><td><code>C_f</code>, <code>c_star</code></td></tr>
    <tr><td><code>C_f</code></td><td><code>solve_c_f(I_sp, c_star)</code></td><td><code>I_sp</code>, <code>c_star</code></td></tr>
    <tr><td><code>c_star</code></td><td><code>solve_c_star(I_sp, C_f)</code></td><td><code>I_sp</code>, <code>C_f</code></td></tr>
  </tbody>
</table>

### Solve `I_sp`

**Function signature**

```rust
equations::rockets::specific_impulse_ideal::solve_i_sp(C_f, c_star) -> Result<f64, _>
```

**Example**

```rust
let value = equations::rockets::specific_impulse_ideal::solve_i_sp(
    1.7684408757,
    "1718.7683350153386 m/s",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

