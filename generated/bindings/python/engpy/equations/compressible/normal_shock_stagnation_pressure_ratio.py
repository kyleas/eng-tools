from engpy._runtime import invoke

def solve_m1(p02_p01, gamma):
    """Solve Normal Shock Stagnation Pressure Ratio for M1

Args:
  p02_p01: Stagnation pressure ratio
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"p02_p01": p02_p01, "gamma": gamma, "path_id": "compressible.normal_shock_stagnation_pressure_ratio", "target": "M1"})

def solve_p02_p01(m1, gamma):
    """Solve Normal Shock Stagnation Pressure Ratio for p02_p01

Args:
  m1: Upstream Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"M1": m1, "gamma": gamma, "path_id": "compressible.normal_shock_stagnation_pressure_ratio", "target": "p02_p01"})

__all__ = [
    "solve_m1",
    "solve_p02_p01",
]
