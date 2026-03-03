# Equation YAML Standard

## 1. Purpose
This standard defines how engineering equations are authored in YAML for the `equations` crate.

It exists to ensure:
- consistency across many equations and domains
- trustworthy behavior under validation and solving
- low authoring burden via strong defaults
- strong testability and clear runtime/docs generation

## 2. Core principles
- Residual/source-of-truth first: `residual` is authoritative.
- Strong defaults: keep common files short and explicit only where needed.
- Symbolic readability: preserve symbolic math for human-facing output.
- Constants are first-class: reference constants symbolically; auto-resolve at solve time.
- Tests are mandatory: every equation must include registry tests.
- Prefer minimal YAML unless verbosity adds real value.

## 3. Recommended file shapes
### 3.1 Minimal recommended form
Use for common explicit equations with one baseline test.

```yaml
key: hoop_stress
taxonomy:
  category: structures
name: Thin-Wall Hoop Stress
display:
  latex: "\\sigma_h = \\frac{P r}{t}"
variables:
  sigma_h: { name: Hoop stress, dimension: stress }
  P: { name: Internal pressure, dimension: pressure }
  r: { name: Mean radius, dimension: length }
  t: { name: Wall thickness, dimension: length, constraints: { nonzero: true } }
residual: "sigma_h - (P * r / t)"
solve:
  explicit_forms:
    sigma_h: P * r / t
    P: sigma_h * t / r
    r: sigma_h * t / P
    t: P * r / sigma_h
assumptions:
  - Thin-wall approximation is valid.
tests:
  - full_state: { P: "2.5 MPa", r: "0.2 m", t: "8 mm", sigma_h: "62.5 MPa" }
```

### 3.2 Standard form
Use when diagram/references/aliases or explicit test options are useful.

```yaml
key: hoop_stress
taxonomy:
  category: structures
aliases: [thin_wall_hoop_stress]
name: Thin-Wall Hoop Stress
display:
  latex: "\\sigma_h = \\frac{P r}{t}"
variables:
  sigma_h: { name: Hoop stress, symbol: "\\sigma_h", dimension: stress }
  P: { name: Internal pressure, dimension: pressure }
  r: { name: Mean radius, dimension: length }
  t: { name: Wall thickness, dimension: length, constraints: { nonzero: true } }
residual: "sigma_h - (P * r / t)"
solve:
  default_target: sigma_h
  explicit_forms:
    sigma_h: P * r / t
    P: sigma_h * t / r
    r: sigma_h * t / P
    t: P * r / sigma_h
assumptions:
  - Thin-wall approximation is valid (t << r).
references:
  - source: Roark's Formulas for Stress and Strain
diagram:
  template: thin_wall_cylinder
tests:
  relation_tolerance: { abs: 1e-7, rel: 1e-8 }
  - id: baseline_metric
    full_state: { P: "2.5 MPa", r: "0.2 m", t: "8 mm", sigma_h: "62.5 MPa" }
```

### 3.3 Verbose/override form
Use only when defaults are insufficient (special numerical bounds, branch-heavy logic, tight tolerances, custom display overrides).

## 4. Field-by-field rules
### 4.1 `key`
- Required.
- Short, stable, taxonomy-independent identifier.
- Lower_snake_case recommended.
- Unique across registry.

### 4.2 `taxonomy`
- Required.
- `category` required.
- `subcategories` optional, default `[]`.
- Keep taxonomy shallow unless deeper structure is clearly needed.

### 4.3 `name`
- Required.
- Human-facing equation title.

### 4.4 `display`
- `display.latex` required and primary authored form.
- `unicode`, `ascii`, `description` optional overrides.
- Missing fields are derived during normalization.

### 4.5 `variables`
- Required map.
- Per variable:
  - required: `name`, `dimension`
  - optional: `symbol`, `default_unit`, `constraints`, `description`, `aliases`
- `symbol` defaults to variable key if omitted.
- `default_unit` defaults by dimension if omitted.
- Dimension defaults may imply constraints (for example positive for pressure/length/stress).

### 4.6 `residual`
- Required.
- Canonical source-of-truth equation form: expression expected to evaluate to zero.

### 4.7 `solve`
- Optional but recommended for explicit forms.
- Common fields:
  - `default_target` optional
  - `explicit_forms` optional map
  - `numerical` optional section

### 4.8 `assumptions`
- Optional but strongly recommended.
- Keep concise and physically meaningful.

### 4.9 `references`
- Optional.
- Prefer authoritative source + short note.

### 4.10 `tests`
- Required.
- Minimal valid form is one baseline `full_state`.

## 5. Naming conventions
### 5.1 Equation keys
- `lower_snake_case`.
- No taxonomy path in `key`.

### 5.2 Category and subcategory naming
- `lower_snake_case`.
- Favor broad category + minimal subcategory depth.

### 5.3 Variable keys
- Use common engineering symbols translated into stable keys.
- Examples: `sigma_h`, `delta_p`, `c_star`, `area_ratio`.

### 5.4 Aliases
- Optional short alternate identifiers only.
- Do not use dotted taxonomy-style aliases.

### 5.5 Symbol usage
- Use explicit `symbol` where mathematical symbol differs meaningfully from key.
- Greek case (uppercase vs lowercase) must be explicit in `symbol`.

## 6. Display rules
- LaTeX is primary authored display.
- Unicode/ascii are derived unless explicitly overridden.
- Rendered output should use variable symbol metadata where present.
- Do not degrade symbolic constants into decimals in human-facing displays.

## 7. Constants rules
- Reference constants by identifier in expressions (for example `pi`, `g0`, `stefan_boltzmann`).
- Known constants are auto-resolved by default at solve time.
- Constants are not normal required user inputs.
- Advanced override path exists in Rust API:
  - `.override_constant("g0", 9.81)`
  - `.override_constants([...])`
- Rendering should preserve symbolic constant forms in LaTeX/unicode/ascii output.

## 8. Solve rules
### 8.1 Explicit forms
- Add explicit forms when they are algebraically clean and trustworthy.
- Explicit forms should be consistent with residual.

### 8.2 Numerical solving
- Used for implicit or inverse targets where explicit forms are unavailable or less robust.
- Defaults exist; override only when needed.

### 8.3 `unsupported_targets`
Rule:

> Do not mark a target as numerically unsupported merely because an explicit form exists.  
> Use `unsupported_targets` only when numerical solving is genuinely unsafe, ambiguous, unstable, misleading, or intentionally disallowed.

### 8.4 Branch behavior
- Define branches only when multiple physically meaningful roots/regions exist.
- Tests must cover branch behavior when relevant.

### 8.5 Default target rules if applicable
- `default_target` is optional.
- Use when UI/docs or example generation benefits from a clear primary solve target.

## 9. Test rules
### 9.1 Baseline full-state requirement
- At least one complete physically valid `full_state` is required.

### 9.2 Metric/imperial expectations
- Include metric baseline always.
- Add imperial baseline when practically meaningful for the equation/domain.

### 9.3 Branch test expectations
- If branches exist, include branch-aware cases and verify expected branch behavior.

### 9.4 Tolerances
- Prefer defaults first.
- Override relation/solve tolerances only when justified by numerical sensitivity.

### 9.5 What a good test case looks like
- Physically coherent values.
- Residual near zero.
- Solves round-trip cleanly for intended targets.
- Explicit vs numerical consistency checked where both are enabled.

## 10. Example/documentation generation rules
- Preferred typed builder style:
  - `eq.solve(category::equation::equation()).target_x().given_y(...).value()?`
- Add units-aware examples when dimensional inputs are meaningful.
- Convenience section should list all generated `solve_<target>(...)` helpers.
- Include branch example only when branch adds real information.
- Constants used by equation should be called out separately from required user inputs.

## 11. Common anti-patterns
- unnecessary `unsupported_targets`
- passing constants as ordinary givens by default
- showing raw key when explicit symbol exists
- over-verbose YAML without added value
- examples that do not match intended typed API style

## 12. Authoring checklist
- key chosen correctly (short, stable, unique)
- taxonomy category/subcategories set appropriately
- variable keys/names/symbols are clear and consistent
- constants referenced symbolically in expressions where appropriate
- explicit forms added where useful
- no unjustified `unsupported_targets`
- baseline tests included (and branch tests where needed)
- assumptions included
- equation renders cleanly in docs (symbols/constants preserved)

## 13. Codex instructions
- Prefer minimal form first.
- Use verbose overrides only when they add concrete value.
- Keep residual authoritative.
- Use constants symbolically in expressions.
- Do not treat constants as normal required givens in examples.
- Only use `unsupported_targets` when genuinely justified.
- Include baseline tests for every equation.
- Follow naming rules exactly.

## 14. Examples
### 14.1 Minimal equation example
```yaml
key: axial_stress
taxonomy: { category: structures }
name: Axial Stress
display: { latex: "\\sigma = \\frac{F}{A}" }
variables:
  sigma: { name: Stress, dimension: stress }
  F: { name: Force, dimension: force }
  A: { name: Area, dimension: area, constraints: { nonzero: true } }
residual: "sigma - (F / A)"
solve:
  explicit_forms:
    sigma: F / A
    F: sigma * A
    A: F / sigma
tests:
  - full_state: { sigma: "50 MPa", F: "10 kN", A: "200 mm2" }
```

### 14.2 Standard complete example
```yaml
key: hoop_stress
taxonomy:
  category: structures
aliases: [thin_wall_hoop_stress]
name: Thin-Wall Hoop Stress
display:
  latex: "\\sigma_h = \\frac{P r}{t}"
variables:
  sigma_h: { name: Hoop stress, symbol: "\\sigma_h", dimension: stress }
  P: { name: Internal pressure, dimension: pressure }
  r: { name: Mean radius, dimension: length }
  t: { name: Wall thickness, dimension: length, constraints: { nonzero: true } }
residual: "sigma_h - (P * r / t)"
solve:
  default_target: sigma_h
  explicit_forms:
    sigma_h: P * r / t
    P: sigma_h * t / r
    r: sigma_h * t / P
    t: P * r / sigma_h
assumptions:
  - Thin-wall approximation is valid (t << r).
references:
  - source: Roark's Formulas for Stress and Strain
diagram:
  template: thin_wall_cylinder
tests:
  - id: baseline_metric
    full_state: { P: "2.5 MPa", r: "0.2 m", t: "8 mm", sigma_h: "62.5 MPa" }
```

### 14.3 Constant-using equation example
```yaml
key: circular_pipe_area
taxonomy: { category: fluids }
name: Circular Pipe Flow Area
display: { latex: "A = \\frac{\\pi D^2}{4}" }
variables:
  A: { name: Flow area, dimension: area }
  D: { name: Pipe diameter, dimension: length, constraints: { nonzero: true } }
residual: "A - ((pi / 4) * D^2)"
solve:
  explicit_forms:
    A: (pi / 4) * D^2
    D: sqrt(4 * A / pi)
assumptions:
  - Circular cross-section.
tests:
  - full_state: { D: "0.1 m", A: "0.007853981633974483 m2" }
```

### 14.4 Legitimate `unsupported_targets` example
```yaml
key: area_mach
taxonomy: { category: compressible }
name: Isentropic Area-Mach Relation
display:
  latex: "\\frac{A}{A^*}=\\cdots"
variables:
  area_ratio: { name: Area ratio, dimension: ratio, symbol: "\\frac{A}{A^*}" }
  M: { name: Mach number, dimension: mach }
  gamma: { name: Specific heat ratio, dimension: dimensionless, symbol: "\\gamma" }
residual: "area_ratio - ((1/M) * ((2/(gamma+1))*(1+((gamma-1)/2)*M^2))^((gamma+1)/(2*(gamma-1))))"
solve:
  explicit_forms:
    area_ratio: "(1/M) * ((2/(gamma+1))*(1+((gamma-1)/2)*M^2))^((gamma+1)/(2*(gamma-1)))"
  numerical:
    unsupported_targets: [gamma]
branches:
  - { name: subsonic, condition: "1 - M" }
  - { name: supersonic, condition: "M - 1", preferred: true }
tests:
  - full_state: { area_ratio: 2.0049745454545462, M: 2.2, gamma: 1.4 }
```

Rationale: solving for `gamma` from this relation is often ill-conditioned/ambiguous in practical workflows.

### 14.5 Example where `unsupported_targets` should be omitted
```yaml
key: hoop_stress
taxonomy: { category: structures }
name: Thin-Wall Hoop Stress
display: { latex: "\\sigma_h = \\frac{P r}{t}" }
variables:
  sigma_h: { name: Hoop stress, dimension: stress }
  P: { name: Internal pressure, dimension: pressure }
  r: { name: Mean radius, dimension: length }
  t: { name: Wall thickness, dimension: length, constraints: { nonzero: true } }
residual: "sigma_h - (P * r / t)"
solve:
  explicit_forms:
    sigma_h: P * r / t
    P: sigma_h * t / r
    r: sigma_h * t / P
    t: P * r / sigma_h
tests:
  - full_state: { P: "2.5 MPa", r: "0.2 m", t: "8 mm", sigma_h: "62.5 MPa" }
```

Rationale: explicit forms exist, but that alone is not a reason to disable numerical solving.
