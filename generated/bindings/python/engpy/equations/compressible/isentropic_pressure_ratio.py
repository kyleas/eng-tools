from engpy._runtime import invoke

def solve_m(p_p0, gamma):
    """Solve Isentropic Pressure Ratio for M

Args:
  p_p0: Static-to-stagnation pressure ratio
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"p_p0": p_p0, "gamma": gamma, "path_id": "compressible.isentropic_pressure_ratio", "target": "M"})

def solve_p_p0(m, gamma):
    """Solve Isentropic Pressure Ratio for p_p0

Args:
  m: Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "gamma": gamma, "path_id": "compressible.isentropic_pressure_ratio", "target": "p_p0"})

__all__ = [
    "solve_m",
    "solve_p_p0",
]
