from engpy._runtime import invoke

def solve_a(sigma, f):
    """Solve Axial Normal Stress for A

Args:
  sigma: Axial stress
  f: Axial force
Returns:
  f64
"""
    return invoke("equation.solve", {"sigma": sigma, "F": f, "path_id": "structures.axial_stress", "target": "A"})

def solve_f(sigma, a):
    """Solve Axial Normal Stress for F

Args:
  sigma: Axial stress
  a: Cross-sectional area
Returns:
  f64
"""
    return invoke("equation.solve", {"sigma": sigma, "A": a, "path_id": "structures.axial_stress", "target": "F"})

def solve_sigma(f, a):
    """Solve Axial Normal Stress for sigma

Args:
  f: Axial force
  a: Cross-sectional area
Returns:
  f64
"""
    return invoke("equation.solve", {"F": f, "A": a, "path_id": "structures.axial_stress", "target": "sigma"})

__all__ = [
    "solve_a",
    "solve_f",
    "solve_sigma",
]
