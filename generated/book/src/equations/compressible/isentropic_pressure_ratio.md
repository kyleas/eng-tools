# Isentropic Pressure Ratio

**Path:** `compressible.isentropic_pressure_ratio`  
**Category:** `compressible`

## Equation

$$
\frac{p}{p_0} = \left(1+\frac{\gamma-1}{2}M^2\right)^{-\gamma/(\gamma-1)}
$$

- Unicode: `p_p0 = pow(1 + ((γ - 1) / 2) · M², - γ / (γ - 1))`
- ASCII: `p_p0 = pow(1 + ((gamma - 1) / 2) * M^2, - gamma / (gamma - 1))`

## Assumptions

- One-dimensional isentropic flow of a calorically perfect gas.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>p_p0</code></td><td>Static-to-stagnation pressure ratio</td><td><span class="math inline">\(\frac{p}{p_{0}}\)</span></td><td><code>ratio</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>M</code></td><td>Mach number</td><td><span class="math inline">\(M\)</span></td><td><code>mach</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>gamma</code></td><td>Specific heat ratio</td><td><span class="math inline">\(\gamma\)</span></td><td><code>dimensionless</code></td><td><code>1</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `M`: numerical
- `p_p0`: explicit, numerical

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(compressible::isentropic_pressure_ratio::equation())
    .target_p_p0()
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
    <tr><td><code>p_p0</code></td><td><code>solve_p_p0(M, gamma)</code></td><td><code>M</code>, <code>gamma</code></td></tr>
  </tbody>
</table>

### Solve `p_p0`

**Function signature**

```rust
equations::compressible::isentropic_pressure_ratio::solve_p_p0(M, gamma) -> Result<f64, _>
```

**Example**

```rust
let value = equations::compressible::isentropic_pressure_ratio::solve_p_p0(
    2.0,
    1.4,
)?;
```

## Source

- Anderson, Modern Compressible Flow: With Historical Perspective

