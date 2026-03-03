# Isentropic Temperature Ratio

**Path:** `compressible.isentropic_temperature_ratio`  
**Category:** `compressible`

## Equation

$$
\frac{T}{T_0} = \left(1+\frac{\gamma-1}{2}M^2\right)^{-1}
$$

- Unicode: `T_T0 = 1 / (1 + ((γ - 1) / 2) · M²)`
- ASCII: `T_T0 = 1 / (1 + ((gamma - 1) / 2) * M^2)`

## Assumptions

- Isentropic perfect-gas flow with constant gamma.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>T_T0</code></td><td>Static-to-stagnation temperature ratio</td><td><span class="math inline">\(\frac{T}{T_{0}}\)</span></td><td><code>ratio</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>M</code></td><td>Mach number</td><td><span class="math inline">\(M\)</span></td><td><code>mach</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>gamma</code></td><td>Specific heat ratio</td><td><span class="math inline">\(\gamma\)</span></td><td><code>dimensionless</code></td><td><code>1</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `M`: numerical
- `T_T0`: explicit, numerical

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(compressible::isentropic_temperature_ratio::equation())
    .target_t_t0()
    .given_m(2.0)
    .given_gamma(1.4)
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>T_T0</code></td><td><code>solve_t_t0(M, gamma)</code></td><td><code>M</code>, <code>gamma</code></td></tr>
  </tbody>
</table>

### Solve `T_T0`

**Function signature**

```rust
equations::compressible::isentropic_temperature_ratio::solve_t_t0(M, gamma) -> Result<f64, _>
```

**Example**

```rust
let value = equations::compressible::isentropic_temperature_ratio::solve_t_t0(
    2.0,
    1.4,
)?;
```

