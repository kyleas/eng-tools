# Isentropic Area-Mach Relation

**Path:** `compressible.area_mach`  
**Category:** `compressible`

## Equation

$$
\frac{A}{A^*} = \frac{1}{M}\left(\frac{2}{\gamma+1}\left(1+\frac{\gamma-1}{2}M^2\right)\right)^{\frac{\gamma+1}{2(\gamma-1)}}
$$

- Unicode: `A/A* = (1/M) * ((2/(gamma+1))*(1+((gamma-1)/2)M^2))^((gamma+1)/(2(gamma-1)))`
- ASCII: `area_ratio = (1/M) * ((2/(gamma+1))*(1+((gamma-1)/2)*M^2))^((gamma+1)/(2*(gamma-1)))`

## Assumptions

- Quasi-one-dimensional isentropic flow.
- Calorically perfect gas with constant gamma.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>area_ratio</code></td><td>Area ratio</td><td><span class="math inline">\(\frac{A}{A^*}\)</span></td><td><code>ratio</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>M</code></td><td>Mach number</td><td><span class="math inline">\(M\)</span></td><td><code>mach</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>gamma</code></td><td>Specific heat ratio</td><td><span class="math inline">\(\gamma\)</span></td><td><code>dimensionless</code></td><td><code>1</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `M`: numerical
- `area_ratio`: explicit, numerical

## Branches

- `subsonic` (`1 - M`)
- `supersonic` (`M - 1`) preferred

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(compressible::area_mach::equation())
    .target_area_ratio()
    .branch_subsonic()
    .given_m(0.3)
    .given_gamma(1.4)
    .value()?;
```

### Typed Builder (Branch Example)

```rust
let value = eq
    .solve(compressible::area_mach::equation())
    .target_area_ratio()
    .branch_supersonic()
    .given_m(2.2)
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
    <tr><td><code>area_ratio</code></td><td><code>solve_area_ratio(M, gamma)</code></td><td><code>M</code>, <code>gamma</code></td></tr>
  </tbody>
</table>

### Solve `area_ratio`

**Function signature**

```rust
equations::compressible::area_mach::solve_area_ratio(M, gamma) -> Result<f64, _>
```

**Example**

```rust
let value = equations::compressible::area_mach::solve_area_ratio(
    0.3,
    1.4,
)?;
```

### Notes

- Branch selection may be required for inverse solves when multiple roots are possible.

## References

- Anderson, Modern Compressible Flow Isentropic flow function tables.

## Aliases

`isentropic_area_mach`
