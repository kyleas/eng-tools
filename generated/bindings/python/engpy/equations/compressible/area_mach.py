from engpy._runtime import invoke

def solve_m(area_ratio, gamma, branch=None):
    """Solve Isentropic Area-Mach Relation for M

Args:
  area_ratio: Area ratio
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"area_ratio": area_ratio, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.area_mach", "target": "M"})

def solve_area_ratio(m, gamma, branch=None):
    """Solve Isentropic Area-Mach Relation for area_ratio

Args:
  m: Mach number
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: subsonic, supersonic
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.area_mach", "target": "area_ratio"})

__all__ = [
    "solve_m",
    "solve_area_ratio",
]
