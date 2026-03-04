from engpy._runtime import invoke

def solve_m(t_t0, gamma):
    """Solve Isentropic Temperature Ratio for M

Args:
  t_t0: Static-to-stagnation temperature ratio
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"T_T0": t_t0, "gamma": gamma, "path_id": "compressible.isentropic_temperature_ratio", "target": "M"})

def solve_t_t0(m, gamma):
    """Solve Isentropic Temperature Ratio for T_T0

Args:
  m: Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "gamma": gamma, "path_id": "compressible.isentropic_temperature_ratio", "target": "T_T0"})

__all__ = [
    "solve_m",
    "solve_t_t0",
]
