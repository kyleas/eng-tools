# Euler Buckling Critical Load

**Path ID:** `structures.euler_buckling_load`

\[
P_{cr} = \frac{\pi^2 E I}{(K L)^2}
\]

- Unicode: `P_cr = (π² · E · I) / ((K · L)²)`
- ASCII: `P_cr = (pi^2 * E * I) / ((K * L)^2)`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>P_cr</code></td><td>Critical buckling load</td><td>\(P_cr\)</td><td><code>force</code></td><td><code>N</code></td></tr>
<tr><td><code>E</code></td><td>Elastic modulus</td><td>\(E\)</td><td><code>pressure</code></td><td><code>Pa</code></td></tr>
<tr><td><code>I</code></td><td>Area moment of inertia</td><td>\(I\)</td><td><code>area_moment_of_inertia</code></td><td><code>m4</code></td></tr>
<tr><td><code>K</code></td><td>Effective length factor</td><td>\(K\)</td><td><code>ratio</code></td><td><code>1</code></td></tr>
<tr><td><code>L</code></td><td>Unbraced length</td><td>\(L\)</td><td><code>length</code></td><td><code>m</code></td></tr>
</tbody></table>

## Assumptions

- Ideal slender column with elastic buckling.

## Constants Used

- [`pi`](../../constants/pi.md) (Archimedes Constant) \(\pi\)

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(structures::euler_buckling_load::equation())
    .target_p_cr()
    .given_i(8e-6)
    .given_k(1.0)
    .given_l(2.0)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(structures::euler_buckling_load::equation())
    .target_p_cr()
    .given_i("8e-6 m4")
    .given_k(1.0)
    .given_l("2 m")
    .value()?;
```

### typed_builder_context

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

### convenience_p_cr

```rust
let value = equations::structures::euler_buckling_load::solve_p_cr(
    "8e-6 m4",
    1.0,
    "2 m",
)?;
```

### convenience_e

```rust
let value = equations::structures::euler_buckling_load::solve_e(
    "3947841.760435743 N",
    "8e-6 m4",
    1.0,
    "2 m",
)?;
```

### convenience_i

```rust
let value = equations::structures::euler_buckling_load::solve_i(
    "3947841.760435743 N",
    1.0,
    "2 m",
)?;
```

### convenience_k

```rust
let value = equations::structures::euler_buckling_load::solve_k(
    "3947841.760435743 N",
    "8e-6 m4",
    "2 m",
)?;
```

### convenience_l

```rust
let value = equations::structures::euler_buckling_load::solve_l(
    "3947841.760435743 N",
    "8e-6 m4",
    1.0,
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::structures::euler_buckling_load::equation()).for_target("E").value()?;
```

### Python
```python
engpy.equations.structures.solve_e(p_cr="...", i="...", k="...", l="...")
# helper layer
engpy.helpers.format_value(engpy.equations.structures.solve_e(p_cr="...", i="...", k="...", l="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("structures.euler_buckling_load")
```

### Excel
```excel
=ENG_STRUCTURES_EULER_BUCKLING_LOAD_E("...","...","...","...")
=ENG_FORMAT(ENG_STRUCTURES_EULER_BUCKLING_LOAD_E("...","...","...","..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("structures.euler_buckling_load")
```

**Excel arguments**
- `p_cr`: Critical buckling load
- `i`: Area moment of inertia
- `k`: Effective length factor
- `l`: Unbraced length

