from engpy._runtime import invoke

def solve_c_f(i_sp, c_star):
    """Solve Ideal Specific Impulse for C_f

Args:
  i_sp: Specific impulse
  c_star: Characteristic velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"I_sp": i_sp, "c_star": c_star, "path_id": "rockets.specific_impulse_ideal", "target": "C_f"})

def solve_i_sp(c_f, c_star):
    """Solve Ideal Specific Impulse for I_sp

Args:
  c_f: Thrust coefficient
  c_star: Characteristic velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"C_f": c_f, "c_star": c_star, "path_id": "rockets.specific_impulse_ideal", "target": "I_sp"})

def solve_c_star(i_sp, c_f):
    """Solve Ideal Specific Impulse for c_star

Args:
  i_sp: Specific impulse
  c_f: Thrust coefficient
Returns:
  f64
"""
    return invoke("equation.solve", {"I_sp": i_sp, "C_f": c_f, "path_id": "rockets.specific_impulse_ideal", "target": "c_star"})

__all__ = [
    "solve_c_f",
    "solve_i_sp",
    "solve_c_star",
]
