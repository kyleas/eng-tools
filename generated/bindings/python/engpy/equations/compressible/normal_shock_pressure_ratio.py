from engpy._runtime import invoke

def solve_m1(p2_p1, gamma):
    """Solve Normal Shock Static Pressure Ratio for M1

Args:
  p2_p1: Static pressure ratio
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"p2_p1": p2_p1, "gamma": gamma, "path_id": "compressible.normal_shock_pressure_ratio", "target": "M1"})

def solve_p2_p1(m1, gamma):
    """Solve Normal Shock Static Pressure Ratio for p2_p1

Args:
  m1: Upstream Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"M1": m1, "gamma": gamma, "path_id": "compressible.normal_shock_pressure_ratio", "target": "p2_p1"})

__all__ = [
    "solve_m1",
    "solve_p2_p1",
]
