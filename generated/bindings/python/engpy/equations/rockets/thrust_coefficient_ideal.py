from engpy._runtime import invoke

def solve_c_f(gamma, p_e_p_c, p_a_p_c, a_e_a_t):
    """Solve Ideal Thrust Coefficient for C_f

Args:
  gamma: Specific heat ratio
  p_e_p_c: Exit-to-chamber pressure ratio
  p_a_p_c: Ambient-to-chamber pressure ratio
  a_e_a_t: Area expansion ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"gamma": gamma, "p_e_p_c": p_e_p_c, "p_a_p_c": p_a_p_c, "A_e_A_t": a_e_a_t, "path_id": "rockets.thrust_coefficient_ideal", "target": "C_f"})

__all__ = [
    "solve_c_f",
]
