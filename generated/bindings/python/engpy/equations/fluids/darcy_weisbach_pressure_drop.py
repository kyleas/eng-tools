from engpy._runtime import invoke

def solve_d(delta_p, f, l, rho, v):
    """Solve Darcy-Weisbach Pressure Drop for D

Args:
  delta_p: Pressure drop
  f: Darcy friction factor
  l: Pipe length
  rho: Fluid density
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"delta_p": delta_p, "f": f, "L": l, "rho": rho, "V": v, "path_id": "fluids.darcy_weisbach_pressure_drop", "target": "D"})

def solve_l(delta_p, f, d, rho, v):
    """Solve Darcy-Weisbach Pressure Drop for L

Args:
  delta_p: Pressure drop
  f: Darcy friction factor
  d: Pipe diameter
  rho: Fluid density
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"delta_p": delta_p, "f": f, "D": d, "rho": rho, "V": v, "path_id": "fluids.darcy_weisbach_pressure_drop", "target": "L"})

def solve_v(delta_p, f, l, d, rho):
    """Solve Darcy-Weisbach Pressure Drop for V

Args:
  delta_p: Pressure drop
  f: Darcy friction factor
  l: Pipe length
  d: Pipe diameter
  rho: Fluid density
Returns:
  f64
"""
    return invoke("equation.solve", {"delta_p": delta_p, "f": f, "L": l, "D": d, "rho": rho, "path_id": "fluids.darcy_weisbach_pressure_drop", "target": "V"})

def solve_delta_p(f, l, d, rho, v):
    """Solve Darcy-Weisbach Pressure Drop for delta_p

Args:
  f: Darcy friction factor
  l: Pipe length
  d: Pipe diameter
  rho: Fluid density
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"f": f, "L": l, "D": d, "rho": rho, "V": v, "path_id": "fluids.darcy_weisbach_pressure_drop", "target": "delta_p"})

def solve_f(delta_p, l, d, rho, v):
    """Solve Darcy-Weisbach Pressure Drop for f

Args:
  delta_p: Pressure drop
  l: Pipe length
  d: Pipe diameter
  rho: Fluid density
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"delta_p": delta_p, "L": l, "D": d, "rho": rho, "V": v, "path_id": "fluids.darcy_weisbach_pressure_drop", "target": "f"})

def solve_rho(delta_p, f, l, d, v):
    """Solve Darcy-Weisbach Pressure Drop for rho

Args:
  delta_p: Pressure drop
  f: Darcy friction factor
  l: Pipe length
  d: Pipe diameter
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"delta_p": delta_p, "f": f, "L": l, "D": d, "V": v, "path_id": "fluids.darcy_weisbach_pressure_drop", "target": "rho"})

__all__ = [
    "solve_d",
    "solve_l",
    "solve_v",
    "solve_delta_p",
    "solve_f",
    "solve_rho",
]
