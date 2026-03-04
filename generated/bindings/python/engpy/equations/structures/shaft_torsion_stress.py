from engpy._runtime import invoke

def solve_j(tau, t, r):
    """Solve Circular Shaft Torsion Stress for J

Args:
  tau: Shear stress
  t: Torque
  r: Radius
Returns:
  f64
"""
    return invoke("equation.solve", {"tau": tau, "T": t, "r": r, "path_id": "structures.shaft_torsion_stress", "target": "J"})

def solve_t(tau, r, j):
    """Solve Circular Shaft Torsion Stress for T

Args:
  tau: Shear stress
  r: Radius
  j: Polar moment of inertia
Returns:
  f64
"""
    return invoke("equation.solve", {"tau": tau, "r": r, "J": j, "path_id": "structures.shaft_torsion_stress", "target": "T"})

def solve_r(tau, t, j):
    """Solve Circular Shaft Torsion Stress for r

Args:
  tau: Shear stress
  t: Torque
  j: Polar moment of inertia
Returns:
  f64
"""
    return invoke("equation.solve", {"tau": tau, "T": t, "J": j, "path_id": "structures.shaft_torsion_stress", "target": "r"})

def solve_tau(t, r, j):
    """Solve Circular Shaft Torsion Stress for tau

Args:
  t: Torque
  r: Radius
  j: Polar moment of inertia
Returns:
  f64
"""
    return invoke("equation.solve", {"T": t, "r": r, "J": j, "path_id": "structures.shaft_torsion_stress", "target": "tau"})

__all__ = [
    "solve_j",
    "solve_t",
    "solve_r",
    "solve_tau",
]
