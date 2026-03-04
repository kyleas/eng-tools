from engpy._runtime import invoke

def solve_beta(m2, mn2, theta):
    """Solve Oblique Shock Downstream Mach for beta

Args:
  m2: Downstream Mach number
  mn2: Downstream normal Mach
  theta: Flow deflection angle
Returns:
  f64
"""
    return invoke("equation.solve", {"m2": m2, "mn2": mn2, "theta": theta, "path_id": "compressible.oblique_shock_m2", "target": "beta"})

def solve_m2(mn2, beta, theta):
    """Solve Oblique Shock Downstream Mach for m2

Args:
  mn2: Downstream normal Mach
  beta: Shock angle
  theta: Flow deflection angle
Returns:
  f64
"""
    return invoke("equation.solve", {"mn2": mn2, "beta": beta, "theta": theta, "path_id": "compressible.oblique_shock_m2", "target": "m2"})

def solve_mn2(m2, beta, theta):
    """Solve Oblique Shock Downstream Mach for mn2

Args:
  m2: Downstream Mach number
  beta: Shock angle
  theta: Flow deflection angle
Returns:
  f64
"""
    return invoke("equation.solve", {"m2": m2, "beta": beta, "theta": theta, "path_id": "compressible.oblique_shock_m2", "target": "mn2"})

def solve_theta(m2, mn2, beta):
    """Solve Oblique Shock Downstream Mach for theta

Args:
  m2: Downstream Mach number
  mn2: Downstream normal Mach
  beta: Shock angle
Returns:
  f64
"""
    return invoke("equation.solve", {"m2": m2, "mn2": mn2, "beta": beta, "path_id": "compressible.oblique_shock_m2", "target": "theta"})

__all__ = [
    "solve_beta",
    "solve_m2",
    "solve_mn2",
    "solve_theta",
]
