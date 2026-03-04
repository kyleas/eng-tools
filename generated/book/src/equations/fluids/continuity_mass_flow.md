# Continuity Mass Flow

**Path ID:** `fluids.continuity_mass_flow`

$$
\dot{m} = \rho A V
$$

- Unicode: `\dot{m} = ρ · A · V`
- ASCII: `m_dot = rho * A * V`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>m_dot</code></td><td>Mass flow rate</td><td>\(\dot{m}\)</td><td><code>mass_flow_rate</code></td><td><code>kg/s</code></td></tr>
<tr><td><code>rho</code></td><td>Fluid density</td><td>\(\rho\)</td><td><code>density</code></td><td><code>kg/m3</code></td></tr>
<tr><td><code>A</code></td><td>Flow area</td><td>\(A\)</td><td><code>area</code></td><td><code>m2</code></td></tr>
<tr><td><code>V</code></td><td>Mean velocity</td><td>\(V\)</td><td><code>velocity</code></td><td><code>m/s</code></td></tr>
</tbody></table>

## Assumptions

- One-dimensional mean properties represent cross-section averages.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(fluids::continuity_mass_flow::equation())
    .target_m_dot()
    .given_rho(998.0)
    .given_a(0.1)
    .given_v(3.0)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(fluids::continuity_mass_flow::equation())
    .target_m_dot()
    .given_rho("998 kg/m3")
    .given_a("0.1 m2")
    .given_v("3 m/s")
    .value()?;
```

### convenience_m_dot

```rust
let value = equations::fluids::continuity_mass_flow::solve_m_dot(
    "998 kg/m3",
    "0.1 m2",
    "3 m/s",
)?;
```

### convenience_rho

```rust
let value = equations::fluids::continuity_mass_flow::solve_rho(
    "299.4 kg/s",
    "0.1 m2",
    "3 m/s",
)?;
```

### convenience_a

```rust
let value = equations::fluids::continuity_mass_flow::solve_a(
    "299.4 kg/s",
    "998 kg/m3",
    "3 m/s",
)?;
```

### convenience_v

```rust
let value = equations::fluids::continuity_mass_flow::solve_v(
    "299.4 kg/s",
    "998 kg/m3",
    "0.1 m2",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::fluids::continuity_mass_flow::equation()).for_target("A").value()?;
```

### Python
```python
engpy.equations.fluids.continuity_mass_flow.solve_a(m_dot="...", rho="...", v="...")
# helper layer
engpy.helpers.format_value(engpy.equations.fluids.continuity_mass_flow.solve_a(m_dot="...", rho="...", v="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("fluids.continuity_mass_flow")
engpy.helpers.equation_targets_text("fluids.continuity_mass_flow")
engpy.helpers.equation_variables_table("fluids.continuity_mass_flow")
engpy.helpers.equation_target_count("fluids.continuity_mass_flow")
```

### Excel
```excel
=ENG_FLUIDS_CONTINUITY_MASS_FLOW_A("...","...","...")
=ENG_FORMAT(ENG_FLUIDS_CONTINUITY_MASS_FLOW_A("...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("fluids.continuity_mass_flow")
=ENG_EQUATION_TARGETS_TEXT("fluids.continuity_mass_flow")
=ENG_EQUATION_VARIABLES_TABLE("fluids.continuity_mass_flow")
=ENG_EQUATION_TARGET_COUNT("fluids.continuity_mass_flow")
```

**Excel arguments**
- `m_dot`: Mass flow rate
- `rho`: Fluid density
- `v`: Mean velocity

