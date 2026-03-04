from engpy._runtime import invoke

def solve_m1(rho2_rho1, gamma):
    """Solve Normal Shock Density Ratio for M1

Args:
  rho2_rho1: Density ratio
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"rho2_rho1": rho2_rho1, "gamma": gamma, "path_id": "compressible.normal_shock_density_ratio", "target": "M1"})

def solve_rho2_rho1(m1, gamma):
    """Solve Normal Shock Density Ratio for rho2_rho1

Args:
  m1: Upstream Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"M1": m1, "gamma": gamma, "path_id": "compressible.normal_shock_density_ratio", "target": "rho2_rho1"})

__all__ = [
    "solve_m1",
    "solve_rho2_rho1",
]
