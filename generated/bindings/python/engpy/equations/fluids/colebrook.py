from engpy._runtime import invoke

def solve_f(eps_d, re):
    """Solve Colebrook-White Friction Factor for f

Args:
  eps_d: Relative roughness
  re: Reynolds number
Returns:
  f64
"""
    return invoke("equation.solve", {"eps_D": eps_d, "Re": re, "path_id": "fluids.colebrook", "target": "f"})

__all__ = [
    "solve_f",
]
