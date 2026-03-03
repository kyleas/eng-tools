# Circular Pipe Flow Area

**Path:** `fluids.circular_pipe_area`  
**Category:** `fluids`

## Equation

$$
A = \frac{\pi D^2}{4}
$$

- Unicode: `A = (π / 4) · D²`
- ASCII: `A = (pi / 4) * D^2`

## Assumptions

- Cross-section is circular and unobstructed.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>A</code></td><td>Area</td><td><span class="math inline">\(A\)</span></td><td><code>area</code></td><td><code>m2</code></td><td><code>-</code></td></tr>
    <tr><td><code>D</code></td><td>Diameter</td><td><span class="math inline">\(D\)</span></td><td><code>length</code></td><td><code>m</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `A`: explicit, numerical
- `D`: explicit

## Constants Used

<ul>
  <li><a href="../../constants/pi.md"><code>pi</code></a>: Archimedes Constant - <span class="math inline">\(\pi\)</span></li>
</ul>

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(fluids::circular_pipe_area::equation())
    .target_a()
    .given_d(0.1)
    .value()?;
```

### Typed Builder (Units-Aware)

```rust
let value = eq
    .solve(fluids::circular_pipe_area::equation())
    .target_a()
    .given_d("0.1 m")
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>A</code></td><td><code>solve_a(D)</code></td><td><code>D</code></td></tr>
    <tr><td><code>D</code></td><td><code>solve_d(A)</code></td><td><code>A</code></td></tr>
  </tbody>
</table>

### Solve `A`

**Function signature**

```rust
equations::fluids::circular_pipe_area::solve_a(D) -> Result<f64, _>
```

**Example**

```rust
let value = equations::fluids::circular_pipe_area::solve_a(
    "0.1 m",
)?;
```

### Notes

- Returns SI by default; use `.value_in("<unit>")` for display units.

