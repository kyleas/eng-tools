from engpy._runtime import invoke

def solve_g_star(p0, t0, gamma, r):
    """Solve Choked Mass Flux for G_star

Args:
  p0: Stagnation pressure
  t0: Stagnation temperature
  gamma: Specific heat ratio
  r: Gas constant
Returns:
  f64
"""
    return invoke("equation.solve", {"p0": p0, "T0": t0, "gamma": gamma, "R": r, "path_id": "compressible.choked_mass_flux", "target": "G_star"})

__all__ = [
    "solve_g_star",
]
