from ._runtime import invoke

def mat_prop(material, property, temperature):
    """Binding-friendly material property lookup

Args:
  material: Material key/name
  property: Property key
  temperature: Temperature input
Returns:
  f64
"""
    return invoke("material.prop", {"material": material, "property": property, "temperature": temperature})

