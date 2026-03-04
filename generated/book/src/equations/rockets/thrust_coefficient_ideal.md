# Ideal Thrust Coefficient

**Path ID:** `rockets.thrust_coefficient_ideal`

$$
C_f = \sqrt{\frac{2\gamma^2}{\gamma-1}\left(\frac{2}{\gamma+1}\right)^{(\gamma+1)/(\gamma-1)}\left(1-\left(\frac{p_e}{p_c}\right)^{(\gamma-1)/\gamma}\right)} + \left(\frac{p_e}{p_c}-\frac{p_a}{p_c}\right)\frac{A_e}{A_t}
$$

- Unicode: `C_f = √((2 · γ² / (γ - 1)) · pow(2 / (γ + 1), (γ + 1) / (γ - 1)) · (1 - pow(p_e_p_c, (γ - 1) / γ))) + (p_e_p_c - p_a_p_c) · A_e_A_t`
- ASCII: `C_f = sqrt((2 * gamma^2 / (gamma - 1)) * pow(2 / (gamma + 1), (gamma + 1) / (gamma - 1)) * (1 - pow(p_e_p_c, (gamma - 1) / gamma))) + (p_e_p_c - p_a_p_c) * A_e_A_t`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>C_f</code></td><td>Thrust coefficient</td><td>\(C_f\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>gamma</code></td><td>Specific heat ratio</td><td>\(\gamma\)</td><td><code>dimensionless</code></td><td><code>1</code></td></tr>
<tr><td><code>p_e_p_c</code></td><td>Exit-to-chamber pressure ratio</td><td>\(p_e_p_c\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>p_a_p_c</code></td><td>Ambient-to-chamber pressure ratio</td><td>\(p_a_p_c\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>A_e_A_t</code></td><td>Area expansion ratio</td><td>\(A_e_A_t\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
</tbody></table>

## Assumptions

- Ideal isentropic nozzle expansion.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(rockets::thrust_coefficient_ideal::equation())
    .target_c_f()
    .given_gamma(1.22)
    .given_p_e_p_c(0.02)
    .given_p_a_p_c(0.01)
    .given_a_e_a_t(8.0)
    .value()?;
```

### convenience_c_f

```rust
let value = equations::rockets::thrust_coefficient_ideal::solve_c_f(
    1.22,
    0.02,
    0.01,
    8.0,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::rockets::thrust_coefficient_ideal::equation()).for_target("C_f").value()?;
```

### Python
```python
engpy.equations.rockets.thrust_coefficient_ideal.solve_c_f(gamma="...", p_e_p_c="...", p_a_p_c="...", a_e_a_t="...")
# helper layer
engpy.helpers.format_value(engpy.equations.rockets.thrust_coefficient_ideal.solve_c_f(gamma="...", p_e_p_c="...", p_a_p_c="...", a_e_a_t="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("rockets.thrust_coefficient_ideal")
engpy.helpers.equation_targets_text("rockets.thrust_coefficient_ideal")
engpy.helpers.equation_variables_table("rockets.thrust_coefficient_ideal")
engpy.helpers.equation_target_count("rockets.thrust_coefficient_ideal")
```

### Excel
```excel
=ENG_ROCKETS_THRUST_COEFFICIENT_IDEAL_C_F("...","...","...","...")
=ENG_FORMAT(ENG_ROCKETS_THRUST_COEFFICIENT_IDEAL_C_F("...","...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("rockets.thrust_coefficient_ideal")
=ENG_EQUATION_TARGETS_TEXT("rockets.thrust_coefficient_ideal")
=ENG_EQUATION_VARIABLES_TABLE("rockets.thrust_coefficient_ideal")
=ENG_EQUATION_TARGET_COUNT("rockets.thrust_coefficient_ideal")
```

**Excel arguments**
- `gamma`: Specific heat ratio
- `p_e_p_c`: Exit-to-chamber pressure ratio
- `p_a_p_c`: Ambient-to-chamber pressure ratio
- `a_e_a_t`: Area expansion ratio

