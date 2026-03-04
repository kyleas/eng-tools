from engpy._runtime import invoke

def solve_m(mu):
    """Solve Mach Angle for M

Args:
  mu: Mach angle
Returns:
  f64
"""
    return invoke("equation.solve", {"mu": mu, "path_id": "compressible.mach_angle", "target": "M"})

def solve_mu(m):
    """Solve Mach Angle for mu

Args:
  m: Mach number
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "path_id": "compressible.mach_angle", "target": "mu"})

__all__ = [
    "solve_m",
    "solve_mu",
]
