from engpy._runtime import invoke

def solve_m(rho_rhostar, gamma, branch=None):
    """Solve Fanno Density Ratio for M

Args:
  rho_rhostar: Density ratio to star state
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"rho_rhostar": rho_rhostar, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.fanno_density_ratio", "target": "M"})

def solve_rho_rhostar(m, gamma, branch=None):
    """Solve Fanno Density Ratio for rho_rhostar

Args:
  m: Mach number
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.fanno_density_ratio", "target": "rho_rhostar"})

__all__ = [
    "solve_m",
    "solve_rho_rhostar",
]
