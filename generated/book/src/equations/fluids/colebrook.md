# Colebrook-White Friction Factor

**Path:** `fluids.colebrook`  
**Category:** `fluids`

## Equation

$$
\frac{1}{\sqrt{f}} + 2\log_{10}\left(\frac{\varepsilon_D}{3.7} + \frac{2.51}{Re\sqrt{f}}\right) = 0
$$

- Unicode: `1/sqrt(f) + 2 log10(eps_D/3.7 + 2.51/(Re sqrt(f))) = 0`
- ASCII: `1/sqrt(f) + 2*log10((eps_D/3.7) + (2.51/(Re*sqrt(f)))) = 0`

## Assumptions

- Fully developed turbulent internal flow.
- Incompressible flow behavior.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>f</code></td><td>Darcy friction factor</td><td><span class="math inline">\(f\)</span></td><td><code>friction_factor</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>eps_D</code></td><td>Relative roughness</td><td><span class="math inline">\(\epsilon_D\)</span></td><td><code>ratio</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>Re</code></td><td>Reynolds number</td><td><span class="math inline">\(Re\)</span></td><td><code>dimensionless</code></td><td><code>1</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `f`: numerical

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(fluids::colebrook::equation())
    .target_f()
    .given_eps_d(4.214317085279759e-4)
    .given_re(1e5)
    .value()?;
```

## References

- Colebrook, C. F. (1939) Turbulent flow in pipes with rough walls.

