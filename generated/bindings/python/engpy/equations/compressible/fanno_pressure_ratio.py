from engpy._runtime import invoke

def solve_m(p_pstar, gamma, branch=None):
    """Solve Fanno Pressure Ratio for M

Args:
  p_pstar: Pressure ratio to star state
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"p_pstar": p_pstar, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.fanno_pressure_ratio", "target": "M"})

def solve_p_pstar(m, gamma, branch=None):
    """Solve Fanno Pressure Ratio for p_pstar

Args:
  m: Mach number
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.fanno_pressure_ratio", "target": "p_pstar"})

__all__ = [
    "solve_m",
    "solve_p_pstar",
]
