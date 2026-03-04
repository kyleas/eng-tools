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
engpy.helpers.material_properties("stainless_304")
engpy.helpers.material_properties_text("stainless_304")
engpy.helpers.material_properties_table("stainless_304")
engpy.helpers.material_property_count("stainless_304")
```

### Excel
```excel
=ENG_MAT_PROP("stainless_304","elastic_modulus","350 K")
=ENG_MATERIAL_PROPERTIES("stainless_304")
=ENG_MATERIAL_PROPERTIES_TEXT("stainless_304")
=ENG_MATERIAL_PROPERTIES_TABLE("stainless_304")
=ENG_MATERIAL_PROPERTY_COUNT("stainless_304")
```

**Excel arguments**
- `material`: material key or alias
- `property_key`: material property key (for example `elastic_modulus`)
- `temperature`: evaluation temperature (recommended with explicit units)
