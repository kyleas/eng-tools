from engpy._runtime import invoke

def solve_m(t_tstar, gamma, branch=None):
    """Solve Fanno Temperature Ratio for M

Args:
  t_tstar: Temperature ratio to star state
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"t_tstar": t_tstar, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.fanno_temperature_ratio", "target": "M"})

def solve_t_tstar(m, gamma, branch=None):
    """Solve Fanno Temperature Ratio for t_tstar

Args:
  m: Mach number
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.fanno_temperature_ratio", "target": "t_tstar"})

__all__ = [
    "solve_m",
    "solve_t_tstar",
]
