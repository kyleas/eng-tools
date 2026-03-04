from engpy._runtime import invoke

def solve_c_star(r, t_c, gamma):
    """Solve Ideal Characteristic Velocity for c_star

Args:
  r: Gas constant
  t_c: Chamber temperature
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"R": r, "T_c": t_c, "gamma": gamma, "path_id": "rockets.cstar_ideal", "target": "c_star"})

__all__ = [
    "solve_c_star",
]
