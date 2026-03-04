# Ideal Gas Law

**Family key:** `ideal_gas`

Common engineering forms of the ideal-gas law under one canonical family.

**Canonical law:** `P * V = m * R * T`

**Canonical equation:** [`thermo.ideal_gas.mass_volume`](../thermo/ideal_gas/mass_volume.md)

## Shared Assumptions

- Thermally and calorically ideal gas behavior.
- Constant gas constant R in the operating range.

## Shared References

- Fundamentals of Thermodynamics (ideal-gas state relation)

## Variants

| Variant | Equation | Display | Use when | Notes |
| --- | --- | --- | --- | --- |
| `mass_volume` | [`thermo.ideal_gas.mass_volume`](../thermo/ideal_gas/mass_volume.md) | \(P V = m R T\) | Use when total mass and control-volume size are primary knowns. | - |
| `density` | [`thermo.ideal_gas.density`](../thermo/ideal_gas/density.md) | \(P = \rho R T\) | Use when density-based flow/property calculations are primary. | - |

## Bindings

### Python

```python
import engpy.equations.families.ideal_gas as family
family.mass_volume_solve_...(...)
```

### Excel

```excel
=ENG_FAMILY_<FAMILY>_<VARIANT>_<TARGET>(...)
```
