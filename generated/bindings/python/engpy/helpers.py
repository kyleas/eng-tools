from ._runtime import invoke

def device_modes(key):
    """Read supported modes for a device

Args:
  key: Device key
Returns:
  list
"""
    return invoke("meta.get", {"entity": "device", "field": "supported_modes", "key": key})

def fluid_properties(key):
    """Read supported properties for a fluid

Args:
  key: Fluid key/alias
Returns:
  list
"""
    return invoke("meta.get", {"entity": "fluid", "field": "supported_properties", "key": key})

def format_value(value, in_unit, out_unit):
    """Convert a numeric value from input units to output units (with dimensional checks)

Args:
  value: Input value in `in_unit`
  in_unit: Input unit expression (for example Pa, m, psia, kg/(m*s))
  out_unit: Requested output unit expression
Returns:
  f64
"""
    return invoke("format.value", {"value": value, "in_unit": in_unit, "out_unit": out_unit})

def material_properties(key):
    """Read available properties for a material

Args:
  key: Material key/alias
Returns:
  list
"""
    return invoke("meta.get", {"entity": "material", "field": "properties", "key": key})

def meta_get(entity, key, field):
    """General metadata helper for bindings

Args:
  entity: equation | device | fluid | material | constant
  key: Entity id/key
  field: Metadata field to read
Returns:
  scalar|list|dict
"""
    return invoke("meta.get", {"entity": entity, "key": key, "field": field})

