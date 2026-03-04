from engpy._runtime import invoke

def solve_d(re, rho, v, mu):
    """Solve Reynolds Number for D

Args:
  re: Reynolds number
  rho: Fluid density
  v: Mean velocity
  mu: Dynamic viscosity
Returns:
  f64
"""
    return invoke("equation.solve", {"Re": re, "rho": rho, "V": v, "mu": mu, "path_id": "fluids.reynolds_number", "target": "D"})

def solve_re(rho, v, d, mu):
    """Solve Reynolds Number for Re

Args:
  rho: Fluid density
  v: Mean velocity
  d: Hydraulic diameter
  mu: Dynamic viscosity
Returns:
  f64
"""
    return invoke("equation.solve", {"rho": rho, "V": v, "D": d, "mu": mu, "path_id": "fluids.reynolds_number", "target": "Re"})

def solve_v(re, rho, d, mu):
    """Solve Reynolds Number for V

Args:
  re: Reynolds number
  rho: Fluid density
  d: Hydraulic diameter
  mu: Dynamic viscosity
Returns:
  f64
"""
    return invoke("equation.solve", {"Re": re, "rho": rho, "D": d, "mu": mu, "path_id": "fluids.reynolds_number", "target": "V"})

def solve_mu(re, rho, v, d):
    """Solve Reynolds Number for mu

Args:
  re: Reynolds number
  rho: Fluid density
  v: Mean velocity
  d: Hydraulic diameter
Returns:
  f64
"""
    return invoke("equation.solve", {"Re": re, "rho": rho, "V": v, "D": d, "path_id": "fluids.reynolds_number", "target": "mu"})

def solve_rho(re, v, d, mu):
    """Solve Reynolds Number for rho

Args:
  re: Reynolds number
  v: Mean velocity
  d: Hydraulic diameter
  mu: Dynamic viscosity
Returns:
  f64
"""
    return invoke("equation.solve", {"Re": re, "V": v, "D": d, "mu": mu, "path_id": "fluids.reynolds_number", "target": "rho"})

__all__ = [
    "solve_d",
    "solve_re",
    "solve_v",
    "solve_mu",
    "solve_rho",
]
