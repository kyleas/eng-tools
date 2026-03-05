from engpy._runtime import invoke

def solve_m(t0_t0star, gamma, branch=None):
    """Solve Rayleigh Stagnation Temperature Ratio for M

Args:
  t0_t0star: Stagnation temperature ratio to star state
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"t0_t0star": t0_t0star, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.rayleigh_stagnation_temperature_ratio", "target": "M"})

def solve_t0_t0star(m, gamma, branch=None):
    """Solve Rayleigh Stagnation Temperature Ratio for t0_t0star

Args:
  m: Mach number
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.rayleigh_stagnation_temperature_ratio", "target": "t0_t0star"})

__all__ = [
    "solve_m",
    "solve_t0_t0star",
]
