from engpy._runtime import invoke

def solve_p(sigma_h, r, t):
    """Solve Thin-Wall Hoop Stress for P

Args:
  sigma_h: Hoop stress
  r: Mean radius
  t: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"sigma_h": sigma_h, "r": r, "t": t, "path_id": "structures.hoop_stress", "target": "P"})

def solve_r(sigma_h, p, t):
    """Solve Thin-Wall Hoop Stress for r

Args:
  sigma_h: Hoop stress
  p: Internal pressure
  t: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"sigma_h": sigma_h, "P": p, "t": t, "path_id": "structures.hoop_stress", "target": "r"})

def solve_sigma_h(p, r, t):
    """Solve Thin-Wall Hoop Stress for sigma_h

Args:
  p: Internal pressure
  r: Mean radius
  t: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"P": p, "r": r, "t": t, "path_id": "structures.hoop_stress", "target": "sigma_h"})

def solve_t(sigma_h, p, r):
    """Solve Thin-Wall Hoop Stress for t

Args:
  sigma_h: Hoop stress
  p: Internal pressure
  r: Mean radius
Returns:
  f64
"""
    return invoke("equation.solve", {"sigma_h": sigma_h, "P": p, "r": r, "path_id": "structures.hoop_stress", "target": "t"})

__all__ = [
    "solve_p",
    "solve_r",
    "solve_sigma_h",
    "solve_t",
]
