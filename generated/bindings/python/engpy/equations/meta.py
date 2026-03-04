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

def equation_description(path_id):
    """Read equation description

Args:
  path_id: Equation path id
Returns:
  str
"""
    return invoke("equation.description", {"path_id": path_id})

def equation_family(path_id):
    """Read parent family/variant metadata for an equation

Args:
  path_id: Equation path id
Returns:
  dict|null
"""
    return invoke("equation.family", {"path_id": path_id})

def equation_latex(path_id):
    """Read LaTeX display form for an equation

Args:
  path_id: Equation path id
Returns:
  str
"""
    return invoke("equation.latex", {"path_id": path_id})

def equation_meta(path_id):
    """Read equation metadata (display forms, variables, dimensions, units, targets)

Args:
  path_id: Equation path id (for example `fluids.reynolds_number`)
Returns:
  dict
"""
    return invoke("equation.meta", {"path_id": path_id})

def equation_name(path_id):
    """Read equation name

Args:
  path_id: Equation path id
Returns:
  str
"""
    return invoke("equation.name", {"path_id": path_id})

def equation_targets(path_id):
    """Read solve targets for an equation

Args:
  path_id: Equation path id
Returns:
  list
"""
    return invoke("equation.targets", {"path_id": path_id})

def equation_unicode(path_id):
    """Read Unicode display form for an equation

Args:
  path_id: Equation path id
Returns:
  str
"""
    return invoke("equation.unicode", {"path_id": path_id})

def equation_variables(path_id):
    """Read variable metadata for an equation

Args:
  path_id: Equation path id
Returns:
  list
"""
    return invoke("equation.variables", {"path_id": path_id})

