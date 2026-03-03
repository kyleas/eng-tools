# Log-Mean Temperature Difference

**Path:** `heat_transfer.log_mean_temperature_difference`  
**Category:** `heat_transfer`

## Equation

$$
\Delta T_{lm} = \frac{\Delta T_1 - \Delta T_2}{\ln(\Delta T_1 / \Delta T_2)}
$$

- Unicode: `Δ T_{lm} = (Δ T_1 - Δ T_2) / ln(Δ T_1 / Δ T_2)`
- ASCII: `delta_T_lm = (delta_T_1 - delta_T_2) / ln(delta_T_1 / delta_T_2)`

## Assumptions

- End-point temperature differences remain positive and unequal.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>delta_T_lm</code></td><td>Log-mean temperature difference</td><td><span class="math inline">\(\Delta T_{lm}\)</span></td><td><code>temperature</code></td><td><code>K</code></td><td><code>-</code></td></tr>
    <tr><td><code>delta_T_1</code></td><td>End temperature difference 1</td><td><span class="math inline">\(\Delta T_1\)</span></td><td><code>temperature</code></td><td><code>K</code></td><td><code>-</code></td></tr>
    <tr><td><code>delta_T_2</code></td><td>End temperature difference 2</td><td><span class="math inline">\(\Delta T_2\)</span></td><td><code>temperature</code></td><td><code>K</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `delta_T_lm`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(heat_transfer::log_mean_temperature_difference::equation())
    .target_delta_t_lm()
    .given_delta_t_1(40.0)
    .given_delta_t_2(20.0)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(heat_transfer::log_mean_temperature_difference::equation())
    .target_delta_t_lm()
    .given_delta_t_1("40 K")
    .given_delta_t_2("20 K")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>delta_T_lm</code></td><td><code>solve_delta_t_lm(delta_T_1, delta_T_2)</code></td><td><code>delta_T_1</code>, <code>delta_T_2</code></td></tr>
  </tbody>
</table>

### Solve `delta_T_lm`

**Function signature**

```rust
equations::heat_transfer::log_mean_temperature_difference::solve_delta_t_lm(delta_T_1, delta_T_2) -> Result<f64, _>
```

**Example**

```rust
let value = equations::heat_transfer::log_mean_temperature_difference::solve_delta_t_lm(
    "40 K",
    "20 K",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

