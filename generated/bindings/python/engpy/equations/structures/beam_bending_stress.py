from engpy._runtime import invoke

def solve_i(sigma_b, m, c):
    """Solve Beam Bending Stress for I

Args:
  sigma_b: Bending stress
  m: Bending moment
  c: Distance to outer fiber
Returns:
  f64
"""
    return invoke("equation.solve", {"sigma_b": sigma_b, "M": m, "c": c, "path_id": "structures.beam_bending_stress", "target": "I"})

def solve_m(sigma_b, c, i):
    """Solve Beam Bending Stress for M

Args:
  sigma_b: Bending stress
  c: Distance to outer fiber
  i: Area moment of inertia
Returns:
  f64
"""
    return invoke("equation.solve", {"sigma_b": sigma_b, "c": c, "I": i, "path_id": "structures.beam_bending_stress", "target": "M"})

def solve_c(sigma_b, m, i):
    """Solve Beam Bending Stress for c

Args:
  sigma_b: Bending stress
  m: Bending moment
  i: Area moment of inertia
Returns:
  f64
"""
    return invoke("equation.solve", {"sigma_b": sigma_b, "M": m, "I": i, "path_id": "structures.beam_bending_stress", "target": "c"})

def solve_sigma_b(m, c, i):
    """Solve Beam Bending Stress for sigma_b

Args:
  m: Bending moment
  c: Distance to outer fiber
  i: Area moment of inertia
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "c": c, "I": i, "path_id": "structures.beam_bending_stress", "target": "sigma_b"})

__all__ = [
    "solve_i",
    "solve_m",
    "solve_c",
    "solve_sigma_b",
]
