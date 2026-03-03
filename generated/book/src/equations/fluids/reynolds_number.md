# Reynolds Number

**Path:** `fluids.reynolds_number`  
**Category:** `fluids`

## Equation

$$
Re = \frac{\rho V D}{\mu}
$$

- Unicode: `Re = ρ · V · D / μ`
- ASCII: `Re = rho * V * D / mu`

## Assumptions

- Internal-flow characteristic length is represented by hydraulic diameter.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>Re</code></td><td>Reynolds number</td><td><span class="math inline">\(Re\)</span></td><td><code>ratio</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>rho</code></td><td>Fluid density</td><td><span class="math inline">\(\rho\)</span></td><td><code>density</code></td><td><code>kg/m3</code></td><td><code>fluid_property:density</code> from <code>fluid</code></td></tr>
    <tr><td><code>V</code></td><td>Mean velocity</td><td><span class="math inline">\(V\)</span></td><td><code>velocity</code></td><td><code>m/s</code></td><td><code>-</code></td></tr>
    <tr><td><code>D</code></td><td>Hydraulic diameter</td><td><span class="math inline">\(D\)</span></td><td><code>length</code></td><td><code>m</code></td><td><code>-</code></td></tr>
    <tr><td><code>mu</code></td><td>Dynamic viscosity</td><td><span class="math inline">\(\mu\)</span></td><td><code>viscosity</code></td><td><code>Pa*s</code></td><td><code>fluid_property:dynamic_viscosity</code> from <code>fluid</code></td></tr>
  </tbody>
</table>

## Resolvable from Contexts

- `rho` from context `fluid` via `fluid_property`:`density`
- `mu` from context `fluid` via `fluid_property`:`dynamic_viscosity`

## Solve Targets

- `D`: explicit
- `Re`: explicit
- `V`: explicit
- `mu`: explicit
- `rho`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(fluids::reynolds_number::equation())
    .target_re()
    .given_v(2.5)
    .given_d(0.1)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(fluids::reynolds_number::equation())
    .target_re()
    .given_v("2.5 m/s")
    .given_d("0.1 m")
    .value()?;
```

### Typed Builder (Context-Assisted)

```rust
let value = eq
    .solve_with_context(fluids::reynolds_number::equation())
    .fluid(eng_fluids::water().state_tp("300 K", "1 bar")?)
    .target_re()
    .given_v("2.5 m/s")
    .given_d("0.1 m")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>Re</code></td><td><code>solve_re(V, D)</code></td><td><code>V</code>, <code>D</code></td></tr>
    <tr><td><code>rho</code></td><td><code>solve_rho(Re, V, D)</code></td><td><code>Re</code>, <code>V</code>, <code>D</code></td></tr>
    <tr><td><code>V</code></td><td><code>solve_v(Re, D)</code></td><td><code>Re</code>, <code>D</code></td></tr>
    <tr><td><code>D</code></td><td><code>solve_d(Re, V)</code></td><td><code>Re</code>, <code>V</code></td></tr>
    <tr><td><code>mu</code></td><td><code>solve_mu(Re, V, D)</code></td><td><code>Re</code>, <code>V</code>, <code>D</code></td></tr>
  </tbody>
</table>

### Solve `Re`

**Function signature**

```rust
equations::fluids::reynolds_number::solve_re(V, D) -> Result<f64, _>
```

**Example**

```rust
let value = equations::fluids::reynolds_number::solve_re(
    "2.5 m/s",
    "0.1 m",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

