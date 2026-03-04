# Thrust From Mass Flow and Effective Exhaust Velocity

**Path ID:** `rockets.thrust_from_mass_flow`

\[
F = \dot{m} c_{eff}
\]

- Unicode: `F = m_dot · c_eff`
- ASCII: `F = m_dot * c_eff`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>F</code></td><td>Thrust</td><td>\(F\)</td><td><code>force</code></td><td><code>N</code></td></tr>
<tr><td><code>m_dot</code></td><td>Mass flow rate</td><td>\(m_dot\)</td><td><code>mass_flow_rate</code></td><td><code>kg/s</code></td></tr>
<tr><td><code>c_eff</code></td><td>Effective exhaust velocity</td><td>\(c_eff\)</td><td><code>velocity</code></td><td><code>m/s</code></td></tr>
</tbody></table>

## Assumptions

- Pressure-thrust contribution is embedded in effective exhaust velocity.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(rockets::thrust_from_mass_flow::equation())
    .target_f()
    .given_m_dot(250.0)
    .given_c_eff(2000.0)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(rockets::thrust_from_mass_flow::equation())
    .target_f()
    .given_m_dot("250 kg/s")
    .given_c_eff("2000 m/s")
    .value()?;
```

### convenience_f

```rust
let value = equations::rockets::thrust_from_mass_flow::solve_f(
    "250 kg/s",
    "2000 m/s",
)?;
```

### convenience_m_dot

```rust
let value = equations::rockets::thrust_from_mass_flow::solve_m_dot(
    "500000 N",
    "2000 m/s",
)?;
```

### convenience_c_eff

```rust
let value = equations::rockets::thrust_from_mass_flow::solve_c_eff(
    "500000 N",
    "250 kg/s",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::rockets::thrust_from_mass_flow::equation()).for_target("F").value()?;
```

### Python
```python
engpy.equations.rockets.solve_f(m_dot="...", c_eff="...")
# helper layer
engpy.helpers.format_value(engpy.equations.rockets.solve_f(m_dot="...", c_eff="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("rockets.thrust_from_mass_flow")
```

### Excel
```excel
=ENG_ROCKETS_THRUST_FROM_MASS_FLOW_F("...","...")
=ENG_FORMAT(ENG_ROCKETS_THRUST_FROM_MASS_FLOW_F("...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("rockets.thrust_from_mass_flow")
```

**Excel arguments**
- `m_dot`: Mass flow rate
- `c_eff`: Effective exhaust velocity

