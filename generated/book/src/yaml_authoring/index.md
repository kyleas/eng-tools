# YAML Authoring

Equation files: `crates/equations/registry/<category>/...yaml`
Family files: `crates/equations/registry/families/*.yaml`

Default workflow: use **minimal/typical** form. Use verbose overrides only when they add real value.

## File Shape Guidance

1. **Minimal**: smallest valid equation with baseline test.
2. **Typical (recommended)**: minimal + assumptions/references + explicit forms where useful.
3. **Verbose/override**: only when default derivation/rendering behavior needs explicit overrides.

## Required vs Optional (Equation)

| Field | Required | Notes |
| --- | --- | --- |
| `key` | yes | Stable equation key. |
| `taxonomy.category` | yes | Top-level grouping. |
| `name` | yes | Human-readable name. |
| `variables` | yes | Variable metadata. |
| `residual.expression` | yes | Source-of-truth relation. |
| `tests.baseline` | yes | Trust/regression baseline. |
| `source` | optional (recommended) | Primary citation for the equation. |
| `display` | optional (recommended) | Latex/unicode/ascii authoring control. |
| `solve.explicit_forms` | optional | Add explicit target forms when available. |
| `assumptions`/`references` | optional (recommended) | Concise trust metadata. |
| `solve.unsupported_targets` | optional (rare) | Only for truly unsafe/unsupported numerical paths. |

## Minimal Equation Example

```yaml
key: hoop_stress
taxonomy: { category: structures }
name: Thin-Wall Hoop Stress
source: "Roark's Formulas for Stress and Strain"
display: { latex: "\\sigma_h = \\frac{P r}{t}" }
variables:
  sigma_h: { dimension: stress, default_unit: Pa }
  P: { dimension: pressure, default_unit: Pa }
  r: { dimension: length, default_unit: m }
  t: { dimension: length, default_unit: m }
residual: { expression: "sigma_h - P*r/t" }
tests:
  baseline: { sigma_h: "62.5 MPa", P: "2.5 MPa", r: "0.2 m", t: "8 mm" }
```

## Typical (Recommended) Equation Example

```yaml
key: reynolds_number
taxonomy:
  category: fluids
name: Reynolds Number
source:
  source: Fox, McDonald, and Pritchard - Introduction to Fluid Mechanics
  note: https://www.wiley.com/en-us/Introduction+to+Fluid+Mechanics%2C+9th+Edition-p-9781119721025
display:
  latex: "Re = \\frac{\\rho V D}{\\mu}"
variables:
  Re: { name: Reynolds number, dimension: dimensionless, default_unit: "1" }
  rho: { name: Fluid density, dimension: density, default_unit: kg/m3, resolver: { source: fluid, kind: fluid_property, property: density } }
  V: { name: Mean velocity, dimension: velocity, default_unit: m/s }
  D: { name: Pipe diameter, dimension: length, default_unit: m }
  mu: { name: Dynamic viscosity, dimension: dynamic_viscosity, default_unit: Pa*s, resolver: { source: fluid, kind: fluid_property, property: dynamic_viscosity } }
residual:
  expression: "Re - rho*V*D/mu"
assumptions:
  - Single-phase continuum flow.
tests:
  baseline:
    Re: 300000
    rho: "998 kg/m3"
    V: "3 m/s"
    D: "0.1 m"
    mu: "1e-3 Pa*s"
```

## Verbose / Override Example

```yaml
key: darcy_weisbach_pressure_drop
taxonomy:
  category: fluids
  subcategory: internal_flow
name: Darcy-Weisbach Pressure Drop
source:
  source: Darcy-Weisbach relation (standard fluid mechanics texts)
display:
  latex: "\\Delta p = f \\frac{L}{D} \\frac{\\rho V^2}{2}"
  unicode: "delta_p = f (L/D) (rho V^2 / 2)"
  ascii: "delta_p = f*(L/D)*(rho*V^2/2)"
variables:
  delta_p: { name: Pressure drop, symbol: "\\Delta p", dimension: pressure, default_unit: Pa }
  f: { name: Darcy friction factor, symbol: f, dimension: friction_factor, default_unit: "1" }
  L: { name: Pipe length, symbol: L, dimension: length, default_unit: m }
  D: { name: Pipe diameter, symbol: D, dimension: length, default_unit: m }
  rho: { name: Fluid density, symbol: "\\rho", dimension: density, default_unit: kg/m^3 }
  V: { name: Mean velocity, symbol: V, dimension: velocity, default_unit: m/s }
residual:
  expression: "delta_p - f*(L/D)*(rho*V^2/2)"
solve:
  explicit_forms:
    delta_p: "f*(L/D)*(rho*V^2/2)"
    f: "delta_p/((L/D)*(rho*V^2/2))"
assumptions:
  - Steady, incompressible internal flow form.
references:
  - source: Standard fluid mechanics texts
tests:
  baseline:
    delta_p: "9000 Pa"
    f: 0.02
    L: "10 m"
    D: "0.1 m"
    rho: "1000 kg/m^3"
    V: "3 m/s"
```

## Family YAML Example

```yaml
key: ideal_gas
name: Ideal Gas Law
description: Common forms of the ideal-gas law under one canonical family.
canonical_equation: thermo.ideal_gas.mass_volume
canonical_law: "P * V = m * R * T"
variants:
  - key: mass_volume
    equation_id: thermo.ideal_gas.mass_volume
    display_latex: "P V = m R T"
  - key: density
    equation_id: thermo.ideal_gas.density
    display_latex: "P = \\rho R T"
```

## Optional vs Required Summary

- Required: `key`, `taxonomy.category`, `name`, `variables`, `residual.expression`, `tests.baseline`.
- Optional but recommended: `source`, `display`, `assumptions`, `references`, explicit solve forms.
- Rare optional: `unsupported_targets` (only when numerical solving is genuinely unsafe/misleading).
- Family files require `variants`; shared assumptions/references are optional but recommended.
