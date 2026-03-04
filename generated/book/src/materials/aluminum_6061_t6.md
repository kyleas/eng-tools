# Aluminum 6061-T6

- Key: `aluminum_6061_t6`
- Aliases: al6061_t6, 6061_t6
- Source: Representative handbook values for interpolation demonstrations.
- Properties: elastic_modulus, thermal_conductivity, yield_strength

Heat-treated aluminum alloy with dense temperature-property series.

## Bindings

### Python
```python
engpy.materials.mat_prop("stainless_304", "elastic_modulus", "350 K")
```

### Excel
```excel
=ENG_MAT_PROP("stainless_304","elastic_modulus","350 K")
```

**Excel arguments**
- `material`: material key or alias
- `property_key`: material property key (for example `elastic_modulus`)
- `temperature`: evaluation temperature (recommended with explicit units)
