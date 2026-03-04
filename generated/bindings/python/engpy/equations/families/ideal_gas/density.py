from engpy._runtime import invoke

def solve_p(rho, r, t):
    """Solve Ideal Gas Law variant Density Form for P

Args:
  rho: Density
  r: Specific gas constant
  t: Absolute temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"rho": rho, "R": r, "T": t, "path_id": "thermo.ideal_gas.density", "target": "P"})

def solve_r(p, rho, t):
    """Solve Ideal Gas Law variant Density Form for R

Args:
  p: Absolute pressure
  rho: Density
  t: Absolute temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"P": p, "rho": rho, "T": t, "path_id": "thermo.ideal_gas.density", "target": "R"})

def solve_t(p, rho, r):
    """Solve Ideal Gas Law variant Density Form for T

Args:
  p: Absolute pressure
  rho: Density
  r: Specific gas constant
Returns:
  f64
"""
    return invoke("equation.solve", {"P": p, "rho": rho, "R": r, "path_id": "thermo.ideal_gas.density", "target": "T"})

def solve_rho(p, r, t):
    """Solve Ideal Gas Law variant Density Form for rho

Args:
  p: Absolute pressure
  r: Specific gas constant
  t: Absolute temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"P": p, "R": r, "T": t, "path_id": "thermo.ideal_gas.density", "target": "rho"})

__all__ = [
    "solve_p",
    "solve_r",
    "solve_t",
    "solve_rho",
]
