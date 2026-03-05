from engpy._runtime import invoke

def solve_m(p0_p0star, gamma, branch=None):
    """Solve Rayleigh Stagnation Pressure Ratio for M

Args:
  p0_p0star: Stagnation pressure ratio to star state
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"p0_p0star": p0_p0star, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.rayleigh_stagnation_pressure_ratio", "target": "M"})

def solve_p0_p0star(m, gamma, branch=None):
    """Solve Rayleigh Stagnation Pressure Ratio for p0_p0star

Args:
  m: Mach number
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.rayleigh_stagnation_pressure_ratio", "target": "p0_p0star"})

__all__ = [
    "solve_m",
    "solve_p0_p0star",
]
