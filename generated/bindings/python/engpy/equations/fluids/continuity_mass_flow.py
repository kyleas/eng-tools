from engpy._runtime import invoke

def solve_a(m_dot, rho, v):
    """Solve Continuity Mass Flow for A

Args:
  m_dot: Mass flow rate
  rho: Fluid density
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"m_dot": m_dot, "rho": rho, "V": v, "path_id": "fluids.continuity_mass_flow", "target": "A"})

def solve_v(m_dot, rho, a):
    """Solve Continuity Mass Flow for V

Args:
  m_dot: Mass flow rate
  rho: Fluid density
  a: Flow area
Returns:
  f64
"""
    return invoke("equation.solve", {"m_dot": m_dot, "rho": rho, "A": a, "path_id": "fluids.continuity_mass_flow", "target": "V"})

def solve_m_dot(rho, a, v):
    """Solve Continuity Mass Flow for m_dot

Args:
  rho: Fluid density
  a: Flow area
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"rho": rho, "A": a, "V": v, "path_id": "fluids.continuity_mass_flow", "target": "m_dot"})

def solve_rho(m_dot, a, v):
    """Solve Continuity Mass Flow for rho

Args:
  m_dot: Mass flow rate
  a: Flow area
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"m_dot": m_dot, "A": a, "V": v, "path_id": "fluids.continuity_mass_flow", "target": "rho"})

__all__ = [
    "solve_a",
    "solve_v",
    "solve_m_dot",
    "solve_rho",
]
