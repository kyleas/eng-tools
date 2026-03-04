from engpy._runtime import invoke

def solve_a(r_th, h):
    """Solve Convection Thermal Resistance for A

Args:
  r_th: Thermal resistance
  h: Convective heat transfer coefficient
Returns:
  f64
"""
    return invoke("equation.solve", {"R_th": r_th, "h": h, "path_id": "heat_transfer.thermal_resistance_convection", "target": "A"})

def solve_r_th(h, a):
    """Solve Convection Thermal Resistance for R_th

Args:
  h: Convective heat transfer coefficient
  a: Surface area
Returns:
  f64
"""
    return invoke("equation.solve", {"h": h, "A": a, "path_id": "heat_transfer.thermal_resistance_convection", "target": "R_th"})

def solve_h(r_th, a):
    """Solve Convection Thermal Resistance for h

Args:
  r_th: Thermal resistance
  a: Surface area
Returns:
  f64
"""
    return invoke("equation.solve", {"R_th": r_th, "A": a, "path_id": "heat_transfer.thermal_resistance_convection", "target": "h"})

__all__ = [
    "solve_a",
    "solve_r_th",
    "solve_h",
]
