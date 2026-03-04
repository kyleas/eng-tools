from engpy._runtime import invoke

def solve_beta(mn1, m1):
    """Solve Oblique Shock Normal Upstream Mach for beta

Args:
  mn1: Upstream normal Mach
  m1: Upstream Mach number
Returns:
  f64
"""
    return invoke("equation.solve", {"mn1": mn1, "m1": m1, "path_id": "compressible.oblique_shock_mn1", "target": "beta"})

def solve_m1(mn1, beta):
    """Solve Oblique Shock Normal Upstream Mach for m1

Args:
  mn1: Upstream normal Mach
  beta: Shock angle
Returns:
  f64
"""
    return invoke("equation.solve", {"mn1": mn1, "beta": beta, "path_id": "compressible.oblique_shock_mn1", "target": "m1"})

def solve_mn1(m1, beta):
    """Solve Oblique Shock Normal Upstream Mach for mn1

Args:
  m1: Upstream Mach number
  beta: Shock angle
Returns:
  f64
"""
    return invoke("equation.solve", {"m1": m1, "beta": beta, "path_id": "compressible.oblique_shock_mn1", "target": "mn1"})

__all__ = [
    "solve_beta",
    "solve_m1",
    "solve_mn1",
]
