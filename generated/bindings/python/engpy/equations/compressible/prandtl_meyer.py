from engpy._runtime import invoke

def solve_m(nu, gamma):
    """Solve Prandtl-Meyer Expansion Angle for M

Args:
  nu: Prandtl-Meyer angle
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"nu": nu, "gamma": gamma, "path_id": "compressible.prandtl_meyer", "target": "M"})

def solve_nu(m, gamma):
    """Solve Prandtl-Meyer Expansion Angle for nu

Args:
  m: Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "gamma": gamma, "path_id": "compressible.prandtl_meyer", "target": "nu"})

__all__ = [
    "solve_m",
    "solve_nu",
]
