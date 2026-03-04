from engpy._runtime import invoke

def solve_m(area_ratio, gamma):
    """Solve Isentropic Area-Mach Relation for M

Args:
  area_ratio: Area ratio
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "compressible.area_mach", "target": "M", "area_ratio": area_ratio, "gamma": gamma})

def solve_area_ratio(m, gamma):
    """Solve Isentropic Area-Mach Relation for area_ratio

Args:
  m: Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "compressible.area_mach", "target": "area_ratio", "M": m, "gamma": gamma})

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
    return invoke("equation.solve", {"path_id": "compressible.choked_mass_flux", "target": "G_star", "p0": p0, "T0": t0, "gamma": gamma, "R": r})

def solve_m(rho_rho0, gamma):
    """Solve Isentropic Density Ratio for M

Args:
  rho_rho0: Static-to-stagnation density ratio
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "compressible.isentropic_density_ratio", "target": "M", "rho_rho0": rho_rho0, "gamma": gamma})

def solve_rho_rho0(m, gamma):
    """Solve Isentropic Density Ratio for rho_rho0

Args:
  m: Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "compressible.isentropic_density_ratio", "target": "rho_rho0", "M": m, "gamma": gamma})

def solve_m(p_p0, gamma):
    """Solve Isentropic Pressure Ratio for M

Args:
  p_p0: Static-to-stagnation pressure ratio
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "compressible.isentropic_pressure_ratio", "target": "M", "p_p0": p_p0, "gamma": gamma})

def solve_p_p0(m, gamma):
    """Solve Isentropic Pressure Ratio for p_p0

Args:
  m: Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "compressible.isentropic_pressure_ratio", "target": "p_p0", "M": m, "gamma": gamma})

def solve_m(t_t0, gamma):
    """Solve Isentropic Temperature Ratio for M

Args:
  t_t0: Static-to-stagnation temperature ratio
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "compressible.isentropic_temperature_ratio", "target": "M", "T_T0": t_t0, "gamma": gamma})

def solve_t_t0(m, gamma):
    """Solve Isentropic Temperature Ratio for T_T0

Args:
  m: Mach number
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "compressible.isentropic_temperature_ratio", "target": "T_T0", "M": m, "gamma": gamma})

