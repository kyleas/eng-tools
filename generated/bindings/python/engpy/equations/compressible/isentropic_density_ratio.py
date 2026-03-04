from engpy._runtime import invoke

def solve_m(rho_rho0, gamma):
    """Solve Isentropic Density Ratio for M

Args:
  rho_rho0: Static-to-stagnation density ratio
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"rho_rho0": rho_rho0, "gamma": gamma, "path_id": "compressible.isentropic_density_ratio", "target": "M"})

def solve_rho_rho0(m, gamma):
    """Solve Isentropic Density Ratio for rho_rho0

Args:
  m: Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "gamma": gamma, "path_id": "compressible.isentropic_density_ratio", "target": "rho_rho0"})

__all__ = [
    "solve_m",
    "solve_rho_rho0",
]
