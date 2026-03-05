from engpy._runtime import invoke

def solve_m(rho_rhostar, gamma):
    """Solve Rayleigh Density Ratio for M

Args:
  rho_rhostar: Density ratio to star state
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"rho_rhostar": rho_rhostar, "gamma": gamma, "path_id": "compressible.rayleigh_density_ratio", "target": "M"})

def solve_rho_rhostar(m, gamma):
    """Solve Rayleigh Density Ratio for rho_rhostar

Args:
  m: Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "gamma": gamma, "path_id": "compressible.rayleigh_density_ratio", "target": "rho_rhostar"})

__all__ = [
    "solve_m",
    "solve_rho_rhostar",
]
