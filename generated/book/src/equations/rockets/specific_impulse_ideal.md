# Ideal Specific Impulse

**Path ID:** `rockets.specific_impulse_ideal`

\[
I_{sp} = \frac{C_f c^*}{g_0}
\]

- Unicode: `I_sp = C_f · c_star / g₀`
- ASCII: `I_sp = C_f * c_star / g0`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>I_sp</code></td><td>Specific impulse</td><td>\(I_sp\)</td><td><code>specific_impulse</code></td><td><code>s</code></td></tr>
<tr><td><code>C_f</code></td><td>Thrust coefficient</td><td>\(C_f\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>c_star</code></td><td>Characteristic velocity</td><td>\(c_star\)</td><td><code>velocity</code></td><td><code>m/s</code></td></tr>
</tbody></table>

## Assumptions

- Vacuum-equivalent ideal performance relation.

## Constants Used

- [`g0`](../../constants/g0.md) (Standard Gravity) \(g_0\)

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(rockets::specific_impulse_ideal::equation())
    .target_i_sp()
    .given_c_f(1.7684408757)
    .given_c_star(1718.7683350153)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(rockets::specific_impulse_ideal::equation())
    .target_i_sp()
    .given_c_f(1.7684408757)
    .given_c_star("1718.7683350153386 m/s")
    .value()?;
```

### convenience_i_sp

```rust
let value = equations::rockets::specific_impulse_ideal::solve_i_sp(
    1.7684408757,
    "1718.7683350153386 m/s",
)?;
```

### convenience_c_f

```rust
let value = equations::rockets::specific_impulse_ideal::solve_c_f(
    "309.94684010132147 s",
    "1718.7683350153386 m/s",
)?;
```

### convenience_c_star

```rust
let value = equations::rockets::specific_impulse_ideal::solve_c_star(
    "309.94684010132147 s",
    1.7684408757,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::rockets::specific_impulse_ideal::equation()).for_target("C_f").value()?;
```

### Python
```python
engpy.equations.rockets.solve_c_f(i_sp="...", c_star="...")
# helper layer
engpy.helpers.format_value(engpy.equations.rockets.solve_c_f(i_sp="...", c_star="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("rockets.specific_impulse_ideal")
```

### Excel
```excel
=ENG_ROCKETS_SPECIFIC_IMPULSE_IDEAL_C_F("...","...")
=ENG_FORMAT(ENG_ROCKETS_SPECIFIC_IMPULSE_IDEAL_C_F("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("rockets.specific_impulse_ideal")
```

**Excel arguments**
- `i_sp`: Specific impulse
- `c_star`: Characteristic velocity

