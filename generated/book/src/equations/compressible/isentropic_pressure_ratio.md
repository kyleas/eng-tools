# Isentropic Pressure Ratio

**Path ID:** `compressible.isentropic_pressure_ratio`

\[
\frac{p}{p_0} = \left(1+\frac{\gamma-1}{2}M^2\right)^{-\gamma/(\gamma-1)}
\]

- Unicode: `p_p0 = pow(1 + ((γ - 1) / 2) · M², - γ / (γ - 1))`
- ASCII: `p_p0 = pow(1 + ((gamma - 1) / 2) * M^2, - gamma / (gamma - 1))`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>p_p0</code></td><td>Static-to-stagnation pressure ratio</td><td>\(p_p0\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>M</code></td><td>Mach number</td><td>\(M\)</td><td><code>mach</code></td><td><code>1</code></td></tr>
<tr><td><code>gamma</code></td><td>Specific heat ratio</td><td>\(\gamma\)</td><td><code>dimensionless</code></td><td><code>1</code></td></tr>
</tbody></table>

## Assumptions

- One-dimensional isentropic flow of a calorically perfect gas.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(compressible::isentropic_pressure_ratio::equation())
    .target_p_p0()
    .given_m(2.0)
    .given_gamma(1.4)
    .value()?;
```

### convenience_p_p0

```rust
let value = equations::compressible::isentropic_pressure_ratio::solve_p_p0(
    2.0,
    1.4,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::compressible::isentropic_pressure_ratio::equation()).for_target("M").value()?;
```

### Python
```python
engpy.equations.compressible.solve_m(p_p0="...", gamma="...")
```

### Excel
```excel
=ENG_COMPRESSIBLE_ISENTROPIC_PRESSURE_RATIO_M("...","...")
```

**Excel arguments**
- `p_p0`: Static-to-stagnation pressure ratio
- `gamma`: Specific heat ratio

