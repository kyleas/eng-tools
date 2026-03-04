from engpy._runtime import invoke

def solve_m(v_vstar, gamma, branch=None):
    """Solve Fanno Velocity Ratio for M

Args:
  v_vstar: Velocity ratio to star state
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"v_vstar": v_vstar, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.fanno_velocity_ratio", "target": "M"})

def solve_v_vstar(m, gamma, branch=None):
    """Solve Fanno Velocity Ratio for v_vstar

Args:
  m: Mach number
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.fanno_velocity_ratio", "target": "v_vstar"})

__all__ = [
    "solve_m",
    "solve_v_vstar",
]
