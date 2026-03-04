from engpy._runtime import invoke

def solve_f(m_dot, c_eff):
    """Solve Thrust From Mass Flow and Effective Exhaust Velocity for F

Args:
  m_dot: Mass flow rate
  c_eff: Effective exhaust velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"m_dot": m_dot, "c_eff": c_eff, "path_id": "rockets.thrust_from_mass_flow", "target": "F"})

def solve_c_eff(f, m_dot):
    """Solve Thrust From Mass Flow and Effective Exhaust Velocity for c_eff

Args:
  f: Thrust
  m_dot: Mass flow rate
Returns:
  f64
"""
    return invoke("equation.solve", {"F": f, "m_dot": m_dot, "path_id": "rockets.thrust_from_mass_flow", "target": "c_eff"})

def solve_m_dot(f, c_eff):
    """Solve Thrust From Mass Flow and Effective Exhaust Velocity for m_dot

Args:
  f: Thrust
  c_eff: Effective exhaust velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"F": f, "c_eff": c_eff, "path_id": "rockets.thrust_from_mass_flow", "target": "m_dot"})

__all__ = [
    "solve_f",
    "solve_c_eff",
    "solve_m_dot",
]
