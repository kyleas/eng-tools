# YAML Authoring

Equation YAML files live under `crates/equations/registry/<category>/`. Family YAML files live under `crates/equations/registry/families/`.

## Minimal Equation Shape

```yaml
key: hoop_stress
taxonomy:
  category: structures
name: Thin-Wall Hoop Stress
display:
  latex: "\\sigma_h = \\frac{P r}{t}"
variables:
  sigma_h: { dimension: stress, default_unit: Pa }
  P: { dimension: pressure, default_unit: Pa }
  r: { dimension: length, default_unit: m }
  t: { dimension: length, default_unit: m }
residual:
  expression: "sigma_h - P*r/t"
tests:
  baseline:
    sigma_h: "62.5 MPa"
    P: "2.5 MPa"
    r: "0.2 m"
    t: "8 mm"
```

## Family YAML Shape

```yaml
key: ideal_gas
name: Ideal Gas Law
canonical_equation: thermo.ideal_gas.mass_volume
variants:
  - key: mass_volume
    equation_id: thermo.ideal_gas.mass_volume
  - key: density
    equation_id: thermo.ideal_gas.density
```

Authoring workflow:
1. add/update equation YAML
2. add baseline tests in YAML
3. run validation/tests
4. regenerate docs/catalog/html exports
