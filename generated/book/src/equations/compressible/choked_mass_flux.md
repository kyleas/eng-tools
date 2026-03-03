# Choked Mass Flux

**Path:** `compressible.choked_mass_flux`  
**Category:** `compressible`

## Equation

$$
G^* = \frac{p_0}{\sqrt{T_0}} \sqrt{\frac{\gamma}{R}} \left(\frac{2}{\gamma+1}\right)^{(\gamma+1)/(2(\gamma-1))}
$$

- Unicode: `G_star = (p0 / √(T0)) · √(γ / R) · pow(2 / (γ + 1), (γ + 1) / (2 · (γ - 1)))`
- ASCII: `G_star = (p0 / sqrt(T0)) * sqrt(gamma / R) * pow(2 / (gamma + 1), (gamma + 1) / (2 * (gamma - 1)))`

## Assumptions

- One-dimensional isentropic converging nozzle at critical condition (M=1 at throat).
- Perfect gas with constant gamma and R.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>G_star</code></td><td>Choked mass flux</td><td><span class="math inline">\(G^*\)</span></td><td><code>mass_flux</code></td><td><code>kg/(m2*s)</code></td><td><code>-</code></td></tr>
    <tr><td><code>p0</code></td><td>Stagnation pressure</td><td><span class="math inline">\(p_{0}\)</span></td><td><code>pressure</code></td><td><code>Pa</code></td><td><code>-</code></td></tr>
    <tr><td><code>T0</code></td><td>Stagnation temperature</td><td><span class="math inline">\(T_{0}\)</span></td><td><code>temperature</code></td><td><code>K</code></td><td><code>-</code></td></tr>
    <tr><td><code>gamma</code></td><td>Specific heat ratio</td><td><span class="math inline">\(\gamma\)</span></td><td><code>dimensionless</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>R</code></td><td>Gas constant</td><td><span class="math inline">\(R\)</span></td><td><code>gas_constant</code></td><td><code>J/(kg*K)</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `G_star`: explicit, numerical

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(compressible::choked_mass_flux::equation())
    .target_g_star()
    .given_p0(5e5)
    .given_t0(300.0)
    .given_gamma(1.4)
    .given_r(287.0)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(compressible::choked_mass_flux::equation())
    .target_g_star()
    .given_p0("500000 Pa")
    .given_t0("300 K")
    .given_gamma(1.4)
    .given_r("287 J/(kg*K)")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>G_star</code></td><td><code>solve_g_star(p0, T0, gamma, R)</code></td><td><code>p0</code>, <code>T0</code>, <code>gamma</code>, <code>R</code></td></tr>
  </tbody>
</table>

### Solve `G_star`

**Function signature**

```rust
equations::compressible::choked_mass_flux::solve_g_star(p0, T0, gamma, R) -> Result<f64, _>
```

**Example**

```rust
let value = equations::compressible::choked_mass_flux::solve_g_star(
    "500000 Pa",
    "300 K",
    1.4,
    "287 J/(kg*K)",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

