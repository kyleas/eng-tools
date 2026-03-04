# Log-Mean Temperature Difference

**Path ID:** `heat_transfer.log_mean_temperature_difference`

\[
\Delta T_{lm} = \frac{\Delta T_1 - \Delta T_2}{\ln(\Delta T_1 / \Delta T_2)}
\]

- Unicode: `Δ T_{lm} = (Δ T_1 - Δ T_2) / ln(Δ T_1 / Δ T_2)`
- ASCII: `delta_T_lm = (delta_T_1 - delta_T_2) / ln(delta_T_1 / delta_T_2)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>delta_T_lm</code></td><td>Log-mean temperature difference</td><td>\(\Delta T_{lm}\)</td><td><code>temperature</code></td><td><code>K</code></td></tr>
<tr><td><code>delta_T_1</code></td><td>End temperature difference 1</td><td>\(\Delta T_1\)</td><td><code>temperature</code></td><td><code>K</code></td></tr>
<tr><td><code>delta_T_2</code></td><td>End temperature difference 2</td><td>\(\Delta T_2\)</td><td><code>temperature</code></td><td><code>K</code></td></tr>
</tbody></table>

## Assumptions

- End-point temperature differences remain positive and unequal.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(heat_transfer::log_mean_temperature_difference::equation())
    .target_delta_t_lm()
    .given_delta_t_1(40.0)
    .given_delta_t_2(20.0)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(heat_transfer::log_mean_temperature_difference::equation())
    .target_delta_t_lm()
    .given_delta_t_1("40 K")
    .given_delta_t_2("20 K")
    .value()?;
```

### convenience_delta_t_lm

```rust
let value = equations::heat_transfer::log_mean_temperature_difference::solve_delta_t_lm(
    "40 K",
    "20 K",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::heat_transfer::log_mean_temperature_difference::equation()).for_target("delta_T_lm").value()?;
```

### Python
```python
engpy.equations.heat_transfer.solve_delta_t_lm(delta_t_1="...", delta_t_2="...")
```

### Excel
```excel
=ENG_HEAT_TRANSFER_LOG_MEAN_TEMPERATURE_DIFFERENCE_DELTA_T_LM("...","...")
```

**Excel arguments**
- `delta_t_1`: End temperature difference 1
- `delta_t_2`: End temperature difference 2

