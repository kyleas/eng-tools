# Stainless Steel 304

- Key: `stainless_304`
- Aliases: ss304, aisi304
- Source: Representative handbook values for interpolation demonstrations.
- Properties: elastic_modulus, thermal_conductivity, yield_strength

Austenitic stainless steel with temperature-dependent mechanical and thermal properties.

## Bindings

### Python
```python
engpy.materials.mat_prop("stainless_304", "elastic_modulus", "350 K")
engpy.helpers.material_properties("stainless_304")
```

### Excel
```excel
=ENG_MAT_PROP("stainless_304","elastic_modulus","350 K")
=ENG_MATERIAL_PROPERTIES("stainless_304")
```

**Excel arguments**
- `material`: material key or alias
- `property_key`: material property key (for example `elastic_modulus`)
- `temperature`: evaluation temperature (recommended with explicit units)
