# Euler Buckling Critical Load

**Path:** `structures.euler_buckling_load`  
**Category:** `structures`

## Equation

$$
P_{cr} = \frac{\pi^2 E I}{(K L)^2}
$$

- Unicode: `P_cr = (π² · E · I) / ((K · L)²)`
- ASCII: `P_cr = (pi^2 * E * I) / ((K * L)^2)`

## Assumptions

- Ideal slender column with elastic buckling.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>P_cr</code></td><td>Critical buckling load</td><td><span class="math inline">\(P_{cr}\)</span></td><td><code>force</code></td><td><code>N</code></td><td><code>-</code></td></tr>
    <tr><td><code>E</code></td><td>Elastic modulus</td><td><span class="math inline">\(E\)</span></td><td><code>pressure</code></td><td><code>Pa</code></td><td><code>material_property:elastic_modulus</code> from <code>material</code></td></tr>
    <tr><td><code>I</code></td><td>Area moment of inertia</td><td><span class="math inline">\(I\)</span></td><td><code>area_moment_of_inertia</code></td><td><code>m4</code></td><td><code>-</code></td></tr>
    <tr><td><code>K</code></td><td>Effective length factor</td><td><span class="math inline">\(K\)</span></td><td><code>ratio</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>L</code></td><td>Unbraced length</td><td><span class="math inline">\(L\)</span></td><td><code>length</code></td><td><code>m</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Resolvable from Contexts

- `E` from context `material` via `material_property`:`elastic_modulus`

## Solve Targets

- `E`: explicit
- `I`: explicit
- `K`: explicit
- `L`: explicit
- `P_cr`: explicit

## Constants Used

<ul>
  <li><a href="../../constants/pi.md"><code>pi</code></a>: Archimedes Constant - <span class="math inline">\(\pi\)</span></li>
</ul>

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(structures::euler_buckling_load::equation())
    .target_p_cr()
    .given_i(8e-6)
    .given_k(1.0)
    .given_l(2.0)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(structures::euler_buckling_load::equation())
    .target_p_cr()
    .given_i("8e-6 m4")
    .given_k(1.0)
    .given_l("2 m")
    .value()?;
```

### Typed Builder (Context-Assisted)

```rust
let value = eq
    .solve_with_context(structures::euler_buckling_load::equation())
    .material(eng_materials::stainless_304().temperature("350 K")?)
    .target_p_cr()
    .given_i("8e-6 m4")
    .given_k(1.0)
    .given_l("2 m")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>P_cr</code></td><td><code>solve_p_cr(I, K, L)</code></td><td><code>I</code>, <code>K</code>, <code>L</code></td></tr>
    <tr><td><code>E</code></td><td><code>solve_e(P_cr, I, K, L)</code></td><td><code>P_cr</code>, <code>I</code>, <code>K</code>, <code>L</code></td></tr>
    <tr><td><code>I</code></td><td><code>solve_i(P_cr, K, L)</code></td><td><code>P_cr</code>, <code>K</code>, <code>L</code></td></tr>
    <tr><td><code>K</code></td><td><code>solve_k(P_cr, I, L)</code></td><td><code>P_cr</code>, <code>I</code>, <code>L</code></td></tr>
    <tr><td><code>L</code></td><td><code>solve_l(P_cr, I, K)</code></td><td><code>P_cr</code>, <code>I</code>, <code>K</code></td></tr>
  </tbody>
</table>

### Solve `P_cr`

**Function signature**

```rust
equations::structures::euler_buckling_load::solve_p_cr(I, K, L) -> Result<f64, _>
```

**Example**

```rust
let value = equations::structures::euler_buckling_load::solve_p_cr(
    "8e-6 m4",
    1.0,
    "2 m",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

## Source

- Roark's Formulas for Stress and Strain

