# Axial Normal Stress

**Path ID:** `structures.axial_stress`

\[
\sigma = \frac{F}{A}
\]

- Unicode: `σ = F / A`
- ASCII: `sigma = F / A`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>sigma</code></td><td>Axial stress</td><td>\(\sigma\)</td><td><code>stress</code></td><td><code>Pa</code></td></tr>
<tr><td><code>F</code></td><td>Axial force</td><td>\(F\)</td><td><code>force</code></td><td><code>N</code></td></tr>
<tr><td><code>A</code></td><td>Cross-sectional area</td><td>\(A\)</td><td><code>area</code></td><td><code>m2</code></td></tr>
</tbody></table>

## Assumptions

- Uniform axial loading and stress distribution.

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(structures::axial_stress::equation())
    .target_sigma()
    .given_f(10000.0)
    .given_a(1.9999999999999998e-4)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(structures::axial_stress::equation())
    .target_sigma()
    .given_f("10000 N")
    .given_a("200 mm2")
    .value()?;
```

### convenience_sigma

```rust
let value = equations::structures::axial_stress::solve_sigma(
    "10000 N",
    "200 mm2",
)?;
```

### convenience_f

```rust
let value = equations::structures::axial_stress::solve_f(
    "50 MPa",
    "200 mm2",
)?;
```

### convenience_a

```rust
let value = equations::structures::axial_stress::solve_a(
    "50 MPa",
    "10000 N",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::structures::axial_stress::equation()).for_target("A").value()?;
```

### Python
```python
engpy.equations.structures.solve_a(sigma="...", f="...")
```

### Excel
```excel
=ENG_STRUCTURES_AXIAL_STRESS_A("...","...")
```

**Excel arguments**
- `sigma`: Axial stress
- `f`: Axial force

