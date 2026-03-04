from engpy._runtime import invoke

def solve_e(p_cr, i, k, l):
    """Solve Euler Buckling Critical Load for E

Args:
  p_cr: Critical buckling load
  i: Area moment of inertia
  k: Effective length factor
  l: Unbraced length
Returns:
  f64
"""
    return invoke("equation.solve", {"P_cr": p_cr, "I": i, "K": k, "L": l, "path_id": "structures.euler_buckling_load", "target": "E"})

def solve_i(p_cr, e, k, l):
    """Solve Euler Buckling Critical Load for I

Args:
  p_cr: Critical buckling load
  e: Elastic modulus
  k: Effective length factor
  l: Unbraced length
Returns:
  f64
"""
    return invoke("equation.solve", {"P_cr": p_cr, "E": e, "K": k, "L": l, "path_id": "structures.euler_buckling_load", "target": "I"})

def solve_k(p_cr, e, i, l):
    """Solve Euler Buckling Critical Load for K

Args:
  p_cr: Critical buckling load
  e: Elastic modulus
  i: Area moment of inertia
  l: Unbraced length
Returns:
  f64
"""
    return invoke("equation.solve", {"P_cr": p_cr, "E": e, "I": i, "L": l, "path_id": "structures.euler_buckling_load", "target": "K"})

def solve_l(p_cr, e, i, k):
    """Solve Euler Buckling Critical Load for L

Args:
  p_cr: Critical buckling load
  e: Elastic modulus
  i: Area moment of inertia
  k: Effective length factor
Returns:
  f64
"""
    return invoke("equation.solve", {"P_cr": p_cr, "E": e, "I": i, "K": k, "path_id": "structures.euler_buckling_load", "target": "L"})

def solve_p_cr(e, i, k, l):
    """Solve Euler Buckling Critical Load for P_cr

Args:
  e: Elastic modulus
  i: Area moment of inertia
  k: Effective length factor
  l: Unbraced length
Returns:
  f64
"""
    return invoke("equation.solve", {"E": e, "I": i, "K": k, "L": l, "path_id": "structures.euler_buckling_load", "target": "P_cr"})

__all__ = [
    "solve_e",
    "solve_i",
    "solve_k",
    "solve_l",
    "solve_p_cr",
]
