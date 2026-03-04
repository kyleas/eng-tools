from engpy._runtime import invoke

def solve_m1(t2_t1, gamma):
    """Solve Normal Shock Temperature Ratio for M1

Args:
  t2_t1: Static temperature ratio
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"T2_T1": t2_t1, "gamma": gamma, "path_id": "compressible.normal_shock_temperature_ratio", "target": "M1"})

def solve_t2_t1(m1, gamma):
    """Solve Normal Shock Temperature Ratio for T2_T1

Args:
  m1: Upstream Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"M1": m1, "gamma": gamma, "path_id": "compressible.normal_shock_temperature_ratio", "target": "T2_T1"})

__all__ = [
    "solve_m1",
    "solve_t2_t1",
]
