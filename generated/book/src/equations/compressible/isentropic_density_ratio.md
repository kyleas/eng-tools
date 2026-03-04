# Isentropic Density Ratio

**Path ID:** `compressible.isentropic_density_ratio`

\[
\frac{\rho}{\rho_0} = \left(1+\frac{\gamma-1}{2}M^2\right)^{-1/(\gamma-1)}
\]

- Unicode: `\frac{ρ}{ρ_0} = pow(1 + ((γ - 1) / 2) · M², - 1 / (γ - 1))`
- ASCII: `rho_rho0 = pow(1 + ((gamma - 1) / 2) * M^2, - 1 / (gamma - 1))`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>rho_rho0</code></td><td>Static-to-stagnation density ratio</td><td>\(\frac{\rho}{\rho_0}\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>M</code></td><td>Mach number</td><td>\(M\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
<tr><td><code>gamma</code></td><td>Specific heat ratio</td><td>\(\gamma\)</td><td><code>dimensionless</code></td><td><code>1</code></td></tr>
</tbody></table>

## Assumptions

- Isentropic perfect-gas flow with constant gamma.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(compressible::isentropic_density_ratio::equation())
    .target_rho_rho0()
    .given_m(2.0)
    .given_gamma(1.4)
    .value()?;
```

### convenience_rho_rho0

```rust
let value = equations::compressible::isentropic_density_ratio::solve_rho_rho0(
    2.0,
    1.4,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::isentropic_density_ratio::equation()).for_target("M").value()?;
```

### Python
```python
engpy.equations.compressible.solve_m(rho_rho0="...", gamma="...")
```

### Excel
```excel
=ENG_COMPRESSIBLE_ISENTROPIC_DENSITY_RATIO_M("...","...")
```

**Excel arguments**
- `rho_rho0`: Static-to-stagnation density ratio
- `gamma`: Specific heat ratio

