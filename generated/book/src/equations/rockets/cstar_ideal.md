# Ideal Characteristic Velocity

**Path ID:** `rockets.cstar_ideal`

\[
c^* = \sqrt{\frac{R T_c}{\gamma}} \left(\frac{\gamma+1}{2}\right)^{(\gamma+1)/(2(\gamma-1))}
\]

- Unicode: `c_star = √(R · T_c / γ) · pow((γ + 1) / 2, (γ + 1) / (2 · (γ - 1)))`
- ASCII: `c_star = sqrt(R * T_c / gamma) * pow((gamma + 1) / 2, (gamma + 1) / (2 * (gamma - 1)))`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>c_star</code></td><td>Characteristic velocity</td><td>\(c_star\)</td><td><code>velocity</code></td><td><code>m/s</code></td></tr>
<tr><td><code>R</code></td><td>Gas constant</td><td>\(R\)</td><td><code>gas_constant</code></td><td><code>J/(kg*K)</code></td></tr>
<tr><td><code>T_c</code></td><td>Chamber temperature</td><td>\(T_c\)</td><td><code>temperature</code></td><td><code>K</code></td></tr>
<tr><td><code>gamma</code></td><td>Specific heat ratio</td><td>\(\gamma\)</td><td><code>dimensionless</code></td><td><code>1</code></td></tr>
</tbody></table>

## Assumptions

- Ideal nozzle flow with calorically perfect combustion products.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(rockets::cstar_ideal::equation())
    .target_c_star()
    .given_r(355.0)
    .given_t_c(3500.0)
    .given_gamma(1.22)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(rockets::cstar_ideal::equation())
    .target_c_star()
    .given_r("355 J/(kg*K)")
    .given_t_c("3500 K")
    .given_gamma(1.22)
    .value()?;
```

### convenience_c_star

```rust
let value = equations::rockets::cstar_ideal::solve_c_star(
    "355 J/(kg*K)",
    "3500 K",
    1.22,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::rockets::cstar_ideal::equation()).for_target("c_star").value()?;
```

### Python
```python
engpy.equations.rockets.solve_c_star(r="...", t_c="...", gamma="...")
```

### Excel
```excel
=ENG_ROCKETS_CSTAR_IDEAL_C_STAR("...","...","...")
```

**Excel arguments**
- `r`: Gas constant
- `t_c`: Chamber temperature
- `gamma`: Specific heat ratio

