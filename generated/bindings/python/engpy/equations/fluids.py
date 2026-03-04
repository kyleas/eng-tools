from engpy._runtime import invoke

def solve_a(d):
    """Solve Circular Pipe Flow Area for A

Args:
  d: Diameter
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.circular_pipe_area", "target": "A", "D": d})

def solve_d(a):
    """Solve Circular Pipe Flow Area for D

Args:
  a: Area
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.circular_pipe_area", "target": "D", "A": a})

def solve_f(eps_d, re):
    """Solve Colebrook-White Friction Factor for f

Args:
  eps_d: Relative roughness
  re: Reynolds number
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.colebrook", "target": "f", "eps_D": eps_d, "Re": re})

def solve_a(m_dot, rho, v):
    """Solve Continuity Mass Flow for A

Args:
  m_dot: Mass flow rate
  rho: Fluid density
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.continuity_mass_flow", "target": "A", "m_dot": m_dot, "rho": rho, "V": v})

def solve_v(m_dot, rho, a):
    """Solve Continuity Mass Flow for V

Args:
  m_dot: Mass flow rate
  rho: Fluid density
  a: Flow area
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.continuity_mass_flow", "target": "V", "m_dot": m_dot, "rho": rho, "A": a})

def solve_m_dot(rho, a, v):
    """Solve Continuity Mass Flow for m_dot

Args:
  rho: Fluid density
  a: Flow area
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.continuity_mass_flow", "target": "m_dot", "rho": rho, "A": a, "V": v})

def solve_rho(m_dot, a, v):
    """Solve Continuity Mass Flow for rho

Args:
  m_dot: Mass flow rate
  a: Flow area
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.continuity_mass_flow", "target": "rho", "m_dot": m_dot, "A": a, "V": v})

def solve_d(delta_p, f, l, rho, v):
    """Solve Darcy-Weisbach Pressure Drop for D

Args:
  delta_p: Pressure drop
  f: Darcy friction factor
  l: Pipe length
  rho: Fluid density
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.darcy_weisbach_pressure_drop", "target": "D", "delta_p": delta_p, "f": f, "L": l, "rho": rho, "V": v})

def solve_l(delta_p, f, d, rho, v):
    """Solve Darcy-Weisbach Pressure Drop for L

Args:
  delta_p: Pressure drop
  f: Darcy friction factor
  d: Pipe diameter
  rho: Fluid density
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.darcy_weisbach_pressure_drop", "target": "L", "delta_p": delta_p, "f": f, "D": d, "rho": rho, "V": v})

def solve_v(delta_p, f, l, d, rho):
    """Solve Darcy-Weisbach Pressure Drop for V

Args:
  delta_p: Pressure drop
  f: Darcy friction factor
  l: Pipe length
  d: Pipe diameter
  rho: Fluid density
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.darcy_weisbach_pressure_drop", "target": "V", "delta_p": delta_p, "f": f, "L": l, "D": d, "rho": rho})

def solve_delta_p(f, l, d, rho, v):
    """Solve Darcy-Weisbach Pressure Drop for delta_p

Args:
  f: Darcy friction factor
  l: Pipe length
  d: Pipe diameter
  rho: Fluid density
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.darcy_weisbach_pressure_drop", "target": "delta_p", "f": f, "L": l, "D": d, "rho": rho, "V": v})

def solve_f(delta_p, l, d, rho, v):
    """Solve Darcy-Weisbach Pressure Drop for f

Args:
  delta_p: Pressure drop
  l: Pipe length
  d: Pipe diameter
  rho: Fluid density
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.darcy_weisbach_pressure_drop", "target": "f", "delta_p": delta_p, "L": l, "D": d, "rho": rho, "V": v})

def solve_rho(delta_p, f, l, d, v):
    """Solve Darcy-Weisbach Pressure Drop for rho

Args:
  delta_p: Pressure drop
  f: Darcy friction factor
  l: Pipe length
  d: Pipe diameter
  v: Mean velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.darcy_weisbach_pressure_drop", "target": "rho", "delta_p": delta_p, "f": f, "L": l, "D": d, "V": v})

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
    return invoke("equation.solve", {"path_id": "fluids.orifice_mass_flow_incompressible", "target": "A", "m_dot": m_dot, "C_d": c_d, "rho": rho, "delta_p": delta_p})

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
    return invoke("equation.solve", {"path_id": "fluids.orifice_mass_flow_incompressible", "target": "C_d", "m_dot": m_dot, "A": a, "rho": rho, "delta_p": delta_p})

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
    return invoke("equation.solve", {"path_id": "fluids.orifice_mass_flow_incompressible", "target": "delta_p", "m_dot": m_dot, "C_d": c_d, "A": a, "rho": rho})

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
    return invoke("equation.solve", {"path_id": "fluids.orifice_mass_flow_incompressible", "target": "m_dot", "C_d": c_d, "A": a, "rho": rho, "delta_p": delta_p})

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
    return invoke("equation.solve", {"path_id": "fluids.orifice_mass_flow_incompressible", "target": "rho", "m_dot": m_dot, "C_d": c_d, "A": a, "delta_p": delta_p})

def solve_d(re, rho, v, mu):
    """Solve Reynolds Number for D

Args:
  re: Reynolds number
  rho: Fluid density
  v: Mean velocity
  mu: Dynamic viscosity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.reynolds_number", "target": "D", "Re": re, "rho": rho, "V": v, "mu": mu})

def solve_re(rho, v, d, mu):
    """Solve Reynolds Number for Re

Args:
  rho: Fluid density
  v: Mean velocity
  d: Hydraulic diameter
  mu: Dynamic viscosity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.reynolds_number", "target": "Re", "rho": rho, "V": v, "D": d, "mu": mu})

def solve_v(re, rho, d, mu):
    """Solve Reynolds Number for V

Args:
  re: Reynolds number
  rho: Fluid density
  d: Hydraulic diameter
  mu: Dynamic viscosity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.reynolds_number", "target": "V", "Re": re, "rho": rho, "D": d, "mu": mu})

def solve_mu(re, rho, v, d):
    """Solve Reynolds Number for mu

Args:
  re: Reynolds number
  rho: Fluid density
  v: Mean velocity
  d: Hydraulic diameter
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.reynolds_number", "target": "mu", "Re": re, "rho": rho, "V": v, "D": d})

def solve_rho(re, v, d, mu):
    """Solve Reynolds Number for rho

Args:
  re: Reynolds number
  v: Mean velocity
  d: Hydraulic diameter
  mu: Dynamic viscosity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "fluids.reynolds_number", "target": "rho", "Re": re, "V": v, "D": d, "mu": mu})

