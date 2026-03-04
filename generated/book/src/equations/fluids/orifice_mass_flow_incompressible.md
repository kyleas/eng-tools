# Incompressible Orifice Mass Flow

**Path ID:** `fluids.orifice_mass_flow_incompressible`

$$
\dot{m} = C_d A \sqrt{2 \rho \Delta p}
$$

- Unicode: `m_dot = C_d · A · √(2 · ρ · Δ p)`
- ASCII: `m_dot = C_d * A * sqrt(2 * rho * delta_p)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>m_dot</code></td><td>Mass flow rate</td><td>\(m_dot\)</td><td><code>mass_flow_rate</code></td><td><code>kg/s</code></td></tr>
<tr><td><code>C_d</code></td><td>Discharge coefficient</td><td>\(C_d\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>A</code></td><td>Orifice area</td><td>\(A\)</td><td><code>area</code></td><td><code>m2</code></td></tr>
<tr><td><code>rho</code></td><td>Fluid density</td><td>\(\rho\)</td><td><code>density</code></td><td><code>kg/m3</code></td></tr>
<tr><td><code>delta_p</code></td><td>Pressure drop</td><td>\(\Delta p\)</td><td><code>pressure</code></td><td><code>Pa</code></td></tr>
</tbody></table>

## Assumptions

- Incompressible single-phase flow through a sharp-edged restriction.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(fluids::orifice_mass_flow_incompressible::equation())
    .target_m_dot()
    .given_c_d(1.0)
    .given_a(1.0)
    .given_rho(1.0)
    .given_delta_p(0.5)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(fluids::orifice_mass_flow_incompressible::equation())
    .target_m_dot()
    .given_c_d(1.0)
    .given_a("1 m2")
    .given_rho("1 kg/m3")
    .given_delta_p("0.5 Pa")
    .value()?;
```

### convenience_m_dot

```rust
let value = equations::fluids::orifice_mass_flow_incompressible::solve_m_dot(
    1.0,
    "1 m2",
    "1 kg/m3",
    "0.5 Pa",
)?;
```

### convenience_c_d

```rust
let value = equations::fluids::orifice_mass_flow_incompressible::solve_c_d(
    "1 kg/s",
    "1 m2",
    "1 kg/m3",
    "0.5 Pa",
)?;
```

### convenience_a

```rust
let value = equations::fluids::orifice_mass_flow_incompressible::solve_a(
    "1 kg/s",
    1.0,
    "1 kg/m3",
    "0.5 Pa",
)?;
```

### convenience_rho

```rust
let value = equations::fluids::orifice_mass_flow_incompressible::solve_rho(
    "1 kg/s",
    1.0,
    "1 m2",
    "0.5 Pa",
)?;
```

### convenience_delta_p

```rust
let value = equations::fluids::orifice_mass_flow_incompressible::solve_delta_p(
    "1 kg/s",
    1.0,
    "1 m2",
    "1 kg/m3",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::fluids::orifice_mass_flow_incompressible::equation()).for_target("A").value()?;
```

### Python
```python
engpy.equations.fluids.orifice_mass_flow_incompressible.solve_a(m_dot="...", c_d="...", rho="...", delta_p="...")
# helper layer
engpy.helpers.format_value(engpy.equations.fluids.orifice_mass_flow_incompressible.solve_a(m_dot="...", c_d="...", rho="...", delta_p="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("fluids.orifice_mass_flow_incompressible")
engpy.helpers.equation_targets_text("fluids.orifice_mass_flow_incompressible")
engpy.helpers.equation_variables_table("fluids.orifice_mass_flow_incompressible")
engpy.helpers.equation_target_count("fluids.orifice_mass_flow_incompressible")
```

### Excel
```excel
=ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_A("...","...","...","...")
=ENG_FORMAT(ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_A("...","...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("fluids.orifice_mass_flow_incompressible")
=ENG_EQUATION_TARGETS_TEXT("fluids.orifice_mass_flow_incompressible")
=ENG_EQUATION_VARIABLES_TABLE("fluids.orifice_mass_flow_incompressible")
=ENG_EQUATION_TARGET_COUNT("fluids.orifice_mass_flow_incompressible")
```

**Excel arguments**
- `m_dot`: Mass flow rate
- `c_d`: Discharge coefficient
- `rho`: Fluid density
- `delta_p`: Pressure drop

