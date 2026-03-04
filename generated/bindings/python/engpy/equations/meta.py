from engpy._runtime import invoke

def equation_ascii(path_id):
    """Read ASCII display form for an equation

Args:
  path_id: Equation path id
Returns:
  str
"""
    return invoke("equation.ascii", {"path_id": path_id})

def equation_default_unit(path_id, variable):
    """Read canonical default unit for one equation variable

Args:
  path_id: Equation path id
  variable: Variable key (case-insensitive)
Returns:
  str
"""
    return invoke("equation.default_unit", {"path_id": path_id, "variable": variable})

def equation_meta(path_id):
    """Read equation metadata (display forms, variables, dimensions, units, targets)

Args:
  path_id: Equation path id (for example `fluids.reynolds_number`)
Returns:
  dict
"""
    return invoke("equation.meta", {"path_id": path_id})

