from engpy._runtime import invoke

def solve_c_star(r, t_c, gamma):
    """Solve Ideal Characteristic Velocity for c_star

Args:
  r: Gas constant
  t_c: Chamber temperature
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "rockets.cstar_ideal", "target": "c_star", "R": r, "T_c": t_c, "gamma": gamma})

def solve_c_f(i_sp, c_star):
    """Solve Ideal Specific Impulse for C_f

Args:
  i_sp: Specific impulse
  c_star: Characteristic velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "rockets.specific_impulse_ideal", "target": "C_f", "I_sp": i_sp, "c_star": c_star})

def solve_i_sp(c_f, c_star):
    """Solve Ideal Specific Impulse for I_sp

Args:
  c_f: Thrust coefficient
  c_star: Characteristic velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "rockets.specific_impulse_ideal", "target": "I_sp", "C_f": c_f, "c_star": c_star})

def solve_c_star(i_sp, c_f):
    """Solve Ideal Specific Impulse for c_star

Args:
  i_sp: Specific impulse
  c_f: Thrust coefficient
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "rockets.specific_impulse_ideal", "target": "c_star", "I_sp": i_sp, "C_f": c_f})

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
    return invoke("equation.solve", {"path_id": "rockets.thrust_coefficient_ideal", "target": "C_f", "gamma": gamma, "p_e_p_c": p_e_p_c, "p_a_p_c": p_a_p_c, "A_e_A_t": a_e_a_t})

def solve_f(m_dot, c_eff):
    """Solve Thrust From Mass Flow and Effective Exhaust Velocity for F

Args:
  m_dot: Mass flow rate
  c_eff: Effective exhaust velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "rockets.thrust_from_mass_flow", "target": "F", "m_dot": m_dot, "c_eff": c_eff})

def solve_c_eff(f, m_dot):
    """Solve Thrust From Mass Flow and Effective Exhaust Velocity for c_eff

Args:
  f: Thrust
  m_dot: Mass flow rate
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "rockets.thrust_from_mass_flow", "target": "c_eff", "F": f, "m_dot": m_dot})

def solve_m_dot(f, c_eff):
    """Solve Thrust From Mass Flow and Effective Exhaust Velocity for m_dot

Args:
  f: Thrust
  c_eff: Effective exhaust velocity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "rockets.thrust_from_mass_flow", "target": "m_dot", "F": f, "c_eff": c_eff})

