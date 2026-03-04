# Reynolds Number

**Path ID:** `fluids.reynolds_number`

\[
Re = \frac{\rho V D}{\mu}
\]

- Unicode: `Re = ρ · V · D / μ`
- ASCII: `Re = rho * V * D / mu`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>Re</code></td><td>Reynolds number</td><td>\(Re\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>rho</code></td><td>Fluid density</td><td>\(\rho\)</td><td><code>density</code></td><td><code>kg/m3</code></td></tr>
<tr><td><code>V</code></td><td>Mean velocity</td><td>\(V\)</td><td><code>velocity</code></td><td><code>m/s</code></td></tr>
<tr><td><code>D</code></td><td>Hydraulic diameter</td><td>\(D\)</td><td><code>length</code></td><td><code>m</code></td></tr>
<tr><td><code>mu</code></td><td>Dynamic viscosity</td><td>\(\mu\)</td><td><code>viscosity</code></td><td><code>Pa*s</code></td></tr>
</tbody></table>

## Assumptions

- Internal-flow characteristic length is represented by hydraulic diameter.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(fluids::reynolds_number::equation())
    .target_re()
    .given_v(2.5)
    .given_d(0.1)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(fluids::reynolds_number::equation())
    .target_re()
    .given_v("2.5 m/s")
    .given_d("0.1 m")
    .value()?;
```

### typed_builder_context

```rust
let value = eq
    .solve_with_context(fluids::reynolds_number::equation())
    .fluid(eng_fluids::water().state_tp("300 K", "1 bar")?)
    .target_re()
    .given_v("2.5 m/s")
    .given_d("0.1 m")
    .value()?;
```

### convenience_re

```rust
let value = equations::fluids::reynolds_number::solve_re(
    "2.5 m/s",
    "0.1 m",
)?;
```

### convenience_rho

```rust
let value = equations::fluids::reynolds_number::solve_rho(
    2.495e5,
    "2.5 m/s",
    "0.1 m",
)?;
```

### convenience_v

```rust
let value = equations::fluids::reynolds_number::solve_v(
    2.495e5,
    "0.1 m",
)?;
```

### convenience_d

```rust
let value = equations::fluids::reynolds_number::solve_d(
    2.495e5,
    "2.5 m/s",
)?;
```

### convenience_mu

```rust
let value = equations::fluids::reynolds_number::solve_mu(
    2.495e5,
    "2.5 m/s",
    "0.1 m",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::fluids::reynolds_number::equation()).for_target("D").value()?;
```

### Python
```python
engpy.equations.fluids.solve_d(re="...", rho="...", v="...", mu="...")
# helper layer
engpy.helpers.format_value(engpy.equations.fluids.solve_d(re="...", rho="...", v="...", mu="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("fluids.reynolds_number")
```

### Excel
```excel
=ENG_FLUIDS_REYNOLDS_NUMBER_D("...","...","...","...")
=ENG_FORMAT(ENG_FLUIDS_REYNOLDS_NUMBER_D("...","...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("fluids.reynolds_number")
```

**Excel arguments**
- `re`: Reynolds number
- `rho`: Fluid density
- `v`: Mean velocity
- `mu`: Dynamic viscosity

