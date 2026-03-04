# Circular Pipe Flow Area

**Path ID:** `fluids.circular_pipe_area`

$$
A = \frac{\pi D^2}{4}
$$

- Unicode: `A = (π / 4) · D²`
- ASCII: `A = (pi / 4) * D^2`

## Variables

<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>
<tr><td><code>A</code></td><td>Area</td><td>\(A\)</td><td><code>area</code></td><td><code>m2</code></td></tr>
<tr><td><code>D</code></td><td>Diameter</td><td>\(D\)</td><td><code>length</code></td><td><code>m</code></td></tr>
</tbody></table>

## Assumptions

- Cross-section is circular and unobstructed.

## Constants Used

- [`pi`](../../constants/pi.md) (Archimedes Constant) \(\pi\)

## Examples

### typed_builder_si

```rust
let value = eq
    .solve(fluids::circular_pipe_area::equation())
    .target_a()
    .given_d(0.1)
    .value()?;
```

### typed_builder_units

```rust
let value = eq
    .solve(fluids::circular_pipe_area::equation())
    .target_a()
    .given_d("0.1 m")
    .value()?;
```

### convenience_a

```rust
let value = equations::fluids::circular_pipe_area::solve_a(
    "0.1 m",
)?;
```

### convenience_d

```rust
let value = equations::fluids::circular_pipe_area::solve_d(
    "0.007853981633974483 m2",
)?;
```


## Bindings

### Rust
```rust
let value = eq.solve(equations::fluids::circular_pipe_area::equation()).for_target("A").value()?;
```

### Python
```python
engpy.equations.fluids.circular_pipe_area.solve_a(d="...")
# helper layer
engpy.helpers.format_value(engpy.equations.fluids.circular_pipe_area.solve_a(d="..."), "<in_unit>", "<out_unit>")
engpy.equations.meta.equation_ascii("fluids.circular_pipe_area")
engpy.helpers.equation_targets_text("fluids.circular_pipe_area")
engpy.helpers.equation_variables_table("fluids.circular_pipe_area")
engpy.helpers.equation_target_count("fluids.circular_pipe_area")
```

### Excel
```excel
=ENG_FLUIDS_CIRCULAR_PIPE_AREA_A("...")
=ENG_FORMAT(ENG_FLUIDS_CIRCULAR_PIPE_AREA_A("..."),"<in_unit>","<out_unit>")
=ENG_EQUATION_ASCII("fluids.circular_pipe_area")
=ENG_EQUATION_TARGETS_TEXT("fluids.circular_pipe_area")
=ENG_EQUATION_VARIABLES_TABLE("fluids.circular_pipe_area")
=ENG_EQUATION_TARGET_COUNT("fluids.circular_pipe_area")
```

**Excel arguments**
- `d`: Diameter

