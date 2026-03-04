from engpy._runtime import invoke

def solve_m1(m2, gamma):
    """Solve Normal Shock Downstream Mach Number for M1

Args:
  m2: Downstream Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"M2": m2, "gamma": gamma, "path_id": "compressible.normal_shock_m2", "target": "M1"})

def solve_m2(m1, gamma):
    """Solve Normal Shock Downstream Mach Number for M2

Args:
  m1: Upstream Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"M1": m1, "gamma": gamma, "path_id": "compressible.normal_shock_m2", "target": "M2"})

__all__ = [
    "solve_m1",
    "solve_m2",
]
