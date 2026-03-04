from ._runtime import invoke

def device_modes(key):
    """Read supported modes for a device

Args:
  key: Device key
Returns:
  list
"""
    return invoke("meta.get", {"entity": "device", "field": "supported_modes", "key": key})

def device_mode_count(key):
    """Read device mode count

Args:
  key: Device key
Returns:
  u64
"""
    return invoke("device.mode.count", {"key": key})

def device_modes_text(key):
    """Read device modes as delimited text

Args:
  key: Device key
Returns:
  str
"""
    return invoke("device.modes.text", {"key": key})

def equation_branches_table(path_id):
    """Read equation branch table rows [branch, is_preferred]

Args:
  path_id: Equation path id
Returns:
  list[list]
"""
    return invoke("equation.branches.table", {"path_id": path_id})

def equation_branches_text(path_id):
    """Read equation branch names as delimited text

Args:
  path_id: Equation path id
Returns:
  str
"""
    return invoke("equation.branches.text", {"path_id": path_id})

def equation_target_count(path_id):
    """Read equation target count

Args:
  path_id: Equation path id
Returns:
  u64
"""
    return invoke("equation.target.count", {"path_id": path_id})

def equation_targets_table(path_id):
    """Read equation targets table rows [target, is_default]

Args:
  path_id: Equation path id
Returns:
  list[list]
"""
    return invoke("equation.targets.table", {"path_id": path_id})

def equation_targets_text(path_id):
    """Read equation targets as delimited text

Args:
  path_id: Equation path id
Returns:
  str
"""
    return invoke("equation.targets.text", {"path_id": path_id})

def equation_variable_count(path_id):
    """Read equation variable count

Args:
  path_id: Equation path id
Returns:
  u64
"""
    return invoke("equation.variable.count", {"path_id": path_id})

def equation_variables_table(path_id):
    """Read equation variable table rows [variable, default_unit]

Args:
  path_id: Equation path id
Returns:
  list[list]
"""
    return invoke("equation.variables.table", {"path_id": path_id})

def equation_variables_text(path_id):
    """Read equation variables as delimited text

Args:
  path_id: Equation path id
Returns:
  str
"""
    return invoke("equation.variables.text", {"path_id": path_id})

def fluid_properties(key):
    """Read supported properties for a fluid

Args:
  key: Fluid key/alias
Returns:
  list
"""
    return invoke("meta.get", {"entity": "fluid", "field": "supported_properties", "key": key})

def fluid_properties_table(key):
    """Read fluid property table rows [property, default_unit]

Args:
  key: Fluid key/alias
Returns:
  list[list]
"""
    return invoke("fluid.properties.table", {"key": key})

def fluid_properties_text(key):
    """Read fluid properties as delimited text

Args:
  key: Fluid key/alias
Returns:
  str
"""
    return invoke("fluid.properties.text", {"key": key})

def fluid_property_count(key):
    """Read fluid property count

Args:
  key: Fluid key/alias
Returns:
  u64
"""
    return invoke("fluid.property.count", {"key": key})

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

def material_properties_table(key):
    """Read material property table rows [property, unit]

Args:
  key: Material key/alias
Returns:
  list[list]
"""
    return invoke("material.properties.table", {"key": key})

def material_properties_text(key):
    """Read material properties as delimited text

Args:
  key: Material key/alias
Returns:
  str
"""
    return invoke("material.properties.text", {"key": key})

def material_property_count(key):
    """Read material property count

Args:
  key: Material key/alias
Returns:
  u64
"""
    return invoke("material.property.count", {"key": key})

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

