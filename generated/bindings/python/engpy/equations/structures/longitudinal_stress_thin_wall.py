from engpy._runtime import invoke

def solve_p(sigma_l, r, t):
    """Solve Thin-Wall Longitudinal Stress for P

Args:
  sigma_l: Longitudinal stress
  r: Mean radius
  t: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"sigma_l": sigma_l, "r": r, "t": t, "path_id": "structures.longitudinal_stress_thin_wall", "target": "P"})

def solve_r(sigma_l, p, t):
    """Solve Thin-Wall Longitudinal Stress for r

Args:
  sigma_l: Longitudinal stress
  p: Internal pressure
  t: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"sigma_l": sigma_l, "P": p, "t": t, "path_id": "structures.longitudinal_stress_thin_wall", "target": "r"})

def solve_sigma_l(p, r, t):
    """Solve Thin-Wall Longitudinal Stress for sigma_l

Args:
  p: Internal pressure
  r: Mean radius
  t: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"P": p, "r": r, "t": t, "path_id": "structures.longitudinal_stress_thin_wall", "target": "sigma_l"})

def solve_t(sigma_l, p, r):
    """Solve Thin-Wall Longitudinal Stress for t

Args:
  sigma_l: Longitudinal stress
  p: Internal pressure
  r: Mean radius
Returns:
  f64
"""
    return invoke("equation.solve", {"sigma_l": sigma_l, "P": p, "r": r, "path_id": "structures.longitudinal_stress_thin_wall", "target": "t"})

__all__ = [
    "solve_p",
    "solve_r",
    "solve_sigma_l",
    "solve_t",
]
