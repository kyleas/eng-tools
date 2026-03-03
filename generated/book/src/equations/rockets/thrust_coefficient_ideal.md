# Ideal Thrust Coefficient

**Path:** `rockets.thrust_coefficient_ideal`  
**Category:** `rockets`

## Equation

$$
C_f = \sqrt{\frac{2\gamma^2}{\gamma-1}\left(\frac{2}{\gamma+1}\right)^{(\gamma+1)/(\gamma-1)}\left(1-\left(\frac{p_e}{p_c}\right)^{(\gamma-1)/\gamma}\right)} + \left(\frac{p_e}{p_c}-\frac{p_a}{p_c}\right)\frac{A_e}{A_t}
$$

- Unicode: `C_f = √((2 · γ² / (γ - 1)) · pow(2 / (γ + 1), (γ + 1) / (γ - 1)) · (1 - pow(p_e_p_c, (γ - 1) / γ))) + (p_e_p_c - p_a_p_c) · A_e_A_t`
- ASCII: `C_f = sqrt((2 * gamma^2 / (gamma - 1)) * pow(2 / (gamma + 1), (gamma + 1) / (gamma - 1)) * (1 - pow(p_e_p_c, (gamma - 1) / gamma))) + (p_e_p_c - p_a_p_c) * A_e_A_t`

## Assumptions

- Ideal isentropic nozzle expansion.

## Variables

<table>
  <thead>
    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>
  </thead>
  <tbody>
    <tr><td><code>C_f</code></td><td>Thrust coefficient</td><td><span class="math inline">\(C_{f}\)</span></td><td><code>ratio</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>gamma</code></td><td>Specific heat ratio</td><td><span class="math inline">\(\gamma\)</span></td><td><code>dimensionless</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>p_e_p_c</code></td><td>Exit-to-chamber pressure ratio</td><td><span class="math inline">\(p_{e_p_c}\)</span></td><td><code>ratio</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>p_a_p_c</code></td><td>Ambient-to-chamber pressure ratio</td><td><span class="math inline">\(p_{a_p_c}\)</span></td><td><code>ratio</code></td><td><code>1</code></td><td><code>-</code></td></tr>
    <tr><td><code>A_e_A_t</code></td><td>Area expansion ratio</td><td><span class="math inline">\(A_{e_A_t}\)</span></td><td><code>ratio</code></td><td><code>1</code></td><td><code>-</code></td></tr>
  </tbody>
</table>

## Solve Targets

- `C_f`: explicit

## Examples

### Typed Builder (SI Numeric)

```rust
let value = eq
    .solve(rockets::thrust_coefficient_ideal::equation())
    .target_c_f()
    .given_gamma(1.22)
    .given_p_e_p_c(0.02)
    .given_p_a_p_c(0.01)
    .given_a_e_a_t(8.0)
    .value()?;
```

### Available Convenience Functions

Direct solve helpers are available for these targets.

<table>
  <thead>
    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>
  </thead>
  <tbody>
    <tr><td><code>C_f</code></td><td><code>solve_c_f(gamma, p_e_p_c, p_a_p_c, A_e_A_t)</code></td><td><code>gamma</code>, <code>p_e_p_c</code>, <code>p_a_p_c</code>, <code>A_e_A_t</code></td></tr>
  </tbody>
</table>

### Solve `C_f`

**Function signature**

```rust
equations::rockets::thrust_coefficient_ideal::solve_c_f(gamma, p_e_p_c, p_a_p_c, A_e_A_t) -> Result<f64, _>
```

**Example**

```rust
let value = equations::rockets::thrust_coefficient_ideal::solve_c_f(
    1.22,
    0.02,
    0.01,
    8.0,
)?;
```

