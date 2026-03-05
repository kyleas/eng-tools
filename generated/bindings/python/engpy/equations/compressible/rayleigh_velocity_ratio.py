from engpy._runtime import invoke

def solve_m(v_vstar, gamma):
    """Solve Rayleigh Velocity Ratio for M

Args:
  v_vstar: Velocity ratio to star state
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"v_vstar": v_vstar, "gamma": gamma, "path_id": "compressible.rayleigh_velocity_ratio", "target": "M"})

def solve_v_vstar(m, gamma):
    """Solve Rayleigh Velocity Ratio for v_vstar

Args:
  m: Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"M": m, "gamma": gamma, "path_id": "compressible.rayleigh_velocity_ratio", "target": "v_vstar"})

__all__ = [
    "solve_m",
    "solve_v_vstar",
]
