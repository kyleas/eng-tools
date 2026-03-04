from engpy._runtime import invoke

def solve_m(four_flstar_d, gamma, branch=None):
    """Solve Fanno Friction Length Parameter for M

Args:
  four_flstar_d: Fanno friction length parameter
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"four_flstar_d": four_flstar_d, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.fanno_friction_parameter", "target": "M"})

def solve_four_flstar_d(m, gamma, branch=None):
    """Solve Fanno Friction Length Parameter for four_flstar_d

Args:
  m: Mach number
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.fanno_friction_parameter", "target": "four_flstar_d"})

__all__ = [
    "solve_m",
    "solve_four_flstar_d",
]
