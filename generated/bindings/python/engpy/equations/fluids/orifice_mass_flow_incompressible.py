from engpy._runtime import invoke

def solve_a(m_dot, c_d, rho, delta_p):
    """Solve Incompressible Orifice Mass Flow for A

Args:
  m_dot: Mass flow rate
  c_d: Discharge coefficient
  rho: Fluid density
  delta_p: Pressure drop
Returns:
  f64
"""
    return invoke("equation.solve", {"m_dot": m_dot, "C_d": c_d, "rho": rho, "delta_p": delta_p, "path_id": "fluids.orifice_mass_flow_incompressible", "target": "A"})

def solve_c_d(m_dot, a, rho, delta_p):
    """Solve Incompressible Orifice Mass Flow for C_d

Args:
  m_dot: Mass flow rate
  a: Orifice area
  rho: Fluid density
  delta_p: Pressure drop
Returns:
  f64
"""
    return invoke("equation.solve", {"m_dot": m_dot, "A": a, "rho": rho, "delta_p": delta_p, "path_id": "fluids.orifice_mass_flow_incompressible", "target": "C_d"})

def solve_delta_p(m_dot, c_d, a, rho):
    """Solve Incompressible Orifice Mass Flow for delta_p

Args:
  m_dot: Mass flow rate
  c_d: Discharge coefficient
  a: Orifice area
  rho: Fluid density
Returns:
  f64
"""
    return invoke("equation.solve", {"m_dot": m_dot, "C_d": c_d, "A": a, "rho": rho, "path_id": "fluids.orifice_mass_flow_incompressible", "target": "delta_p"})

def solve_m_dot(c_d, a, rho, delta_p):
    """Solve Incompressible Orifice Mass Flow for m_dot

Args:
  c_d: Discharge coefficient
  a: Orifice area
  rho: Fluid density
  delta_p: Pressure drop
Returns:
  f64
"""
    return invoke("equation.solve", {"C_d": c_d, "A": a, "rho": rho, "delta_p": delta_p, "path_id": "fluids.orifice_mass_flow_incompressible", "target": "m_dot"})

def solve_rho(m_dot, c_d, a, delta_p):
    """Solve Incompressible Orifice Mass Flow for rho

Args:
  m_dot: Mass flow rate
  c_d: Discharge coefficient
  a: Orifice area
  delta_p: Pressure drop
Returns:
  f64
"""
    return invoke("equation.solve", {"m_dot": m_dot, "C_d": c_d, "A": a, "delta_p": delta_p, "path_id": "fluids.orifice_mass_flow_incompressible", "target": "rho"})

__all__ = [
    "solve_a",
    "solve_c_d",
    "solve_delta_p",
    "solve_m_dot",
    "solve_rho",
]
