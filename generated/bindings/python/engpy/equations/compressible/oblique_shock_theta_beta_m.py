from engpy._runtime import invoke

def solve_beta(theta, m1, gamma, branch=None):
    """Solve Oblique Shock Theta-Beta-M Relation for beta

Args:
  theta: Flow deflection angle
  m1: Upstream Mach number
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: weak, strong
Returns:
  f64
"""
    return invoke("equation.solve", {"theta": theta, "m1": m1, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.oblique_shock_theta_beta_m", "target": "beta"})

def solve_theta(beta, m1, gamma, branch=None):
    """Solve Oblique Shock Theta-Beta-M Relation for theta

Args:
  beta: Shock angle
  m1: Upstream Mach number
  gamma: Specific heat ratio
  branch: Optional branch selection. Supported: weak, strong
Returns:
  f64
"""
    return invoke("equation.solve", {"beta": beta, "m1": m1, "gamma": gamma, **({"branch": branch} if branch is not None else {}), "path_id": "compressible.oblique_shock_theta_beta_m", "target": "theta"})

__all__ = [
    "solve_beta",
    "solve_theta",
]
