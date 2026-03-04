from engpy._runtime import invoke

def solve_a(r_th, l, k):
    """Solve Conduction Thermal Resistance for A

Args:
  r_th: Thermal resistance
  l: Wall thickness
  k: Thermal conductivity
Returns:
  f64
"""
    return invoke("equation.solve", {"R_th": r_th, "L": l, "k": k, "path_id": "heat_transfer.thermal_resistance_conduction", "target": "A"})

def solve_l(r_th, k, a):
    """Solve Conduction Thermal Resistance for L

Args:
  r_th: Thermal resistance
  k: Thermal conductivity
  a: Area
Returns:
  f64
"""
    return invoke("equation.solve", {"R_th": r_th, "k": k, "A": a, "path_id": "heat_transfer.thermal_resistance_conduction", "target": "L"})

def solve_r_th(l, k, a):
    """Solve Conduction Thermal Resistance for R_th

Args:
  l: Wall thickness
  k: Thermal conductivity
  a: Area
Returns:
  f64
"""
    return invoke("equation.solve", {"L": l, "k": k, "A": a, "path_id": "heat_transfer.thermal_resistance_conduction", "target": "R_th"})

def solve_k(r_th, l, a):
    """Solve Conduction Thermal Resistance for k

Args:
  r_th: Thermal resistance
  l: Wall thickness
  a: Area
Returns:
  f64
"""
    return invoke("equation.solve", {"R_th": r_th, "L": l, "A": a, "path_id": "heat_transfer.thermal_resistance_conduction", "target": "k"})

__all__ = [
    "solve_a",
    "solve_l",
    "solve_r_th",
    "solve_k",
]
